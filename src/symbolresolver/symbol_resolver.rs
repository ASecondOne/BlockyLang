use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use colored::Colorize;

use crate::executers::ExcuterOutput;
use crate::i_core::datastore::{var::Variable, ASSUMEND_VARIABLES};
use crate::i_core::{gather_blocktypes, gather_expression_parsers, gather_keywords};
use crate::psychoparser::psycho_parser::{PsychoBlock, PsychoCall, PsychoExpression};

//? Used for Values and Variables
pub trait Expression: Debug + Send {
    fn evaluate(&self) -> Option<SharedExpression>;
    fn display(&self) -> Option<String>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn redirect(&mut self, new: SharedExpression) -> ExcuterOutput;
}

pub type SharedExpression = Arc<Mutex<dyn Expression>>;

//? Used to Parse everything that implements Expression
#[derive(Debug)]
pub struct ExpressionParser {
    pub origin: &'static str,
    pub parse: fn(&str) -> Option<SharedExpression>,
}

#[derive(Debug)]
pub struct BlockType {
    pub name: String,
    pub symbol_blacklist: Vec<String>,
    pub symbol_whitelist: Vec<String>,
    pub execution_order: usize,
}

#[derive(Debug)]
pub struct ResolvedBlock {
    pub block_type: Arc<BlockType>,
    pub resolved_lines: Vec<ResolvedExpression>,
}

#[derive(Debug)]
pub struct Closure {
    pub contents: Vec<ResolvedExpression>,
    pub local_state: LocalState,
}

#[derive(Debug, Default)]
pub struct LocalState {
    active: bool,
    scope_id: String,
    variables: std::collections::HashMap<String, SharedExpression>,
}

static NEXT_LOCAL_SCOPE_ID: AtomicU64 = AtomicU64::new(1);

impl LocalState {
    pub fn resolve(&mut self, expression: SharedExpression) -> Result<SharedExpression, String> {
        if !self.active {
            return Ok(expression);
        }

        let variable = {
            let expression = expression
                .lock()
                .map_err(|_| "Could not lock variable for closure state".to_string())?;
            expression
                .as_any()
                .downcast_ref::<Variable>()
                .cloned()
        };

        let Some(variable) = variable else {
            return Ok(expression);
        };

        if let Some(local) = self.variables.get(&variable.root) {
            return Ok(Arc::clone(local));
        }

        let key = variable.root.clone();
        let mut variable = variable;
        variable.origin = variable.identity.clone();
        variable.identity = format!("{}/{}", self.scope_id, variable.name);
        let local: SharedExpression = Arc::new(Mutex::new(variable));
        self.variables.insert(key, Arc::clone(&local));
        Ok(local)
    }

    pub fn fork(&self) -> Result<Self, String> {
        let mut forked = Self {
            active: true,
            scope_id: format!("closure{}", NEXT_LOCAL_SCOPE_ID.fetch_add(1, Ordering::Relaxed)),
            variables: std::collections::HashMap::with_capacity(self.variables.len()),
        };

        for (origin, expression) in &self.variables {
            let variable = {
                let expression = expression
                    .lock()
                    .map_err(|_| "Could not copy enclosing closure state".to_string())?;
                expression
                    .as_any()
                    .downcast_ref::<Variable>()
                    .cloned()
            };

            if let Some(mut variable) = variable {
                variable.origin = variable.identity.clone();
                variable.identity = format!("{}/{}", forked.scope_id, variable.name);
                forked
                    .variables
                    .insert(origin.clone(), Arc::new(Mutex::new(variable)));
            }
        }

        Ok(forked)
    }
}

#[derive(Debug)]
pub enum ResolvedExpression {
    Expression(SharedExpression),
    KeywordCall(KeywordCall),
    Redirect(Box<ResolvedExpression>, Box<ResolvedExpression>),
    Closure(Closure),
}

#[derive(Debug)]
pub struct KeywordCall {
    pub keyword: Keyword,
    pub args: Vec<ResolvedExpression>,
}

#[derive(Clone, Debug)]
pub struct Keyword {
    pub origin: String,
    pub execute: fn(Vec<SharedExpression>) -> ExcuterOutput,
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) -> Result<Vec<ResolvedBlock>, ()> {
    let block_types = gather_blocktypes();

    let mut out = Vec::new();
    let mut had_errors = false;

    for psycho_block in psycho_blocks {
        let block_type = psycho_block.block_type;
        let contents = psycho_block.contents;

        let Some(resolved_block_type) = block_types
            .iter()
            .find(|b| b.name == block_type)
            .cloned()
        else {
            eprintln!(
                "{}",
                format!("Resolver error: Unknown block type `<{block_type}>`").red()
            );
            had_errors = true;
            continue;
        };

        let resolved_contents: Vec<ResolvedExpression> = contents
            .iter()
            .enumerate()
            .filter_map(|(line, content)| {
                match resolve_individual_line(content, &resolved_block_type) {
                    Ok(expression) => Some(expression),
                    Err(message) => {
                        eprintln!(
                            "{}",
                            format!(
                                "Resolver error in `<{block_type}>` expression {}: {message}",
                                line + 1
                            )
                            .red()
                        );
                        had_errors = true;
                        None
                    }
                }
            })
            .collect();

        out.push(ResolvedBlock {
            block_type: resolved_block_type,
            resolved_lines: resolved_contents,
        });
    }

    if let Err(message) = validate_rough_variable_existance(&out) {
        eprintln!("{}", format!("Resolver error: {message}").red());
        had_errors = true;
    }

    if had_errors { Err(()) } else { Ok(out) }
}

fn resolve_individual_line(input: &PsychoExpression, block_type: &BlockType) -> Result<ResolvedExpression, String> {
    match input {
        PsychoExpression::Expression(e) => {
            resolve_expressions(e)
        }

        PsychoExpression::Call(c) => {
            resolve_call(c, block_type)
        }

        PsychoExpression::Redirect(into, from) => {
            Ok(ResolvedExpression::Redirect(
                Box::new(resolve_individual_line(into, block_type)?),
                Box::new(resolve_individual_line(from, block_type)?),
            ))
        }

        PsychoExpression::Closure(contents) => Ok(ResolvedExpression::Closure(Closure {
            contents: contents
                .iter()
                .map(|expression| resolve_individual_line(expression, block_type))
                .collect::<Result<Vec<_>, _>>()?,
            local_state: LocalState::default(),
        })),
    }
}

fn resolve_call(input: &PsychoCall, block_type: &BlockType) -> Result<ResolvedExpression, String> {
    let available_keywords = gather_keywords();

    let resolved_expressions: Vec<ResolvedExpression> = input
        .expressions
        .iter()
        .map(|expression| resolve_individual_line(expression, block_type))
        .collect::<Result<Vec<_>, _>>()?;

    let matching_keyword = available_keywords
        .iter()
        .find(|keyword| keyword.origin.contains(&input.keyword));

    if matching_keyword.is_none() {
        return Err(format!("Unknown keyword `{}`", input.keyword));
    }

    let keyword = available_keywords
        .into_iter()
        .find(|k| {
            let matches = k.origin.contains(&input.keyword);

            let whitelisted = block_type.symbol_whitelist.is_empty() || block_type
                .symbol_whitelist
                .iter()
                .any(|s| k.origin.contains(s));

            let blacklisted = block_type
                .symbol_blacklist
                .iter()
                .any(|s| k.origin.contains(s));

            matches && whitelisted && !blacklisted
        })
        .ok_or_else(|| {
            format!(
                "Keyword `{}` is not allowed in `<{}>` blocks",
                input.keyword, block_type.name
            )
        })?;

    Ok(ResolvedExpression::KeywordCall(KeywordCall {
        keyword,
        args: resolved_expressions,
    }))
}

fn resolve_expressions(input: &str) -> Result<ResolvedExpression, String> {
    let expression_parsers = gather_expression_parsers();

    for parser in expression_parsers {
        if let Some(exp) = (parser.parse)(input) {
            return Ok(ResolvedExpression::Expression(exp));
        }
    }

    Err(format!("Could not parse expression `{input}`"))
}

fn validate_rough_variable_existance(input: &[ResolvedBlock]) -> Result<(), String> {
    let mut assumed_variables = ASSUMEND_VARIABLES
        .lock()
        .map_err(|_| "Could not lock assumed variables".to_string())?;

    for rb in input {
        for expression in &rb.resolved_lines {
            search_for_let(expression, &mut assumed_variables, &rb.block_type.name)?;
        }
    }

    if assumed_variables.is_empty() {
        return Ok(());
    }

    let mut missing_variables: Vec<&str> = assumed_variables
        .iter()
        .map(String::as_str)
        .collect();
    missing_variables.sort_unstable();

    Err(format!(
        "Variables do not exist: {}",
        missing_variables.join(", ")
    ))
}

fn search_for_let(
    input: &ResolvedExpression,
    assumed_variables: &mut HashSet<String>,
    origin: &str,
) -> Result<(), String> {
    match input {
        ResolvedExpression::KeywordCall(call) => {
            if call.keyword.origin == "i_core::datastore::var::let" {
                for arg in &call.args {
                    let ResolvedExpression::Expression(expression) = arg else {
                        return Err("`let` requires a variable name".to_string());
                    };

                    let mut expression = expression
                        .lock()
                        .map_err(|_| "Could not lock declared variable".to_string())?;

                    let variable = expression
                        .as_any_mut()
                        .downcast_mut::<Variable>()
                        .ok_or_else(|| "`let` requires a variable name".to_string())?;

                    assumed_variables.remove(&variable.name);
                    variable.origin = format!("{origin}/{}", variable.name);
                    variable.root = variable.origin.clone();
                    variable.identity = variable.root.clone();
                }
            }

            for arg in &call.args {
                search_for_let(arg, assumed_variables, origin)?;
            }
        }

        ResolvedExpression::Redirect(into, from) => {
            search_for_let(into, assumed_variables, origin)?;
            search_for_let(from, assumed_variables, origin)?;
        }

        ResolvedExpression::Closure(closure) => {
            for expression in &closure.contents {
                search_for_let(expression, assumed_variables, origin)?;
            }
        }

        ResolvedExpression::Expression(_) => {}
    }

    Ok(())
}
