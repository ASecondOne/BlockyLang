use crate::{
    utils::{
        execution_policy::ExecutionPolicy,
        runtime_error::{ErrorType, RuntimeError},
    },
    var_handler::{Value, VarMap},
};

#[derive(Debug, PartialEq)]
pub enum LogicExpression {
    Error(String),
    Value(Value),
    Variable(String),
    Not(Box<LogicExpression>),
    IsEqual(Box<LogicExpression>, Box<LogicExpression>),
    IsNotEqual(Box<LogicExpression>, Box<LogicExpression>),
    IsLessThan(Box<LogicExpression>, Box<LogicExpression>),
    IsLessThanOrEqual(Box<LogicExpression>, Box<LogicExpression>),
    IsGreaterThan(Box<LogicExpression>, Box<LogicExpression>),
    IsGreaterThanOrEqual(Box<LogicExpression>, Box<LogicExpression>),
    And(Box<LogicExpression>, Box<LogicExpression>),
    Or(Box<LogicExpression>, Box<LogicExpression>),
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),
    Variable(String),
    Not,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((index, character)) = chars.next() {
        match character {
            character if character.is_whitespace() => {}
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '=' => {
                consume_expected(&mut chars, '=', index, "Did you mean `==`?")?;
                tokens.push(Token::Equal);
            }
            '!' => {
                if matches!(chars.peek(), Some((_, '='))) {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '<' => {
                if matches!(chars.peek(), Some((_, '='))) {
                    chars.next();
                    tokens.push(Token::LessThanOrEqual);
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                if matches!(chars.peek(), Some((_, '='))) {
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '&' => {
                consume_expected(&mut chars, '&', index, "Did you mean `&&`?")?;
                tokens.push(Token::And);
            }
            '|' => {
                consume_expected(&mut chars, '|', index, "Did you mean `||`?")?;
                tokens.push(Token::Or);
            }
            '"' => {
                let mut value = String::new();
                let mut closed = false;

                while let Some((_, next_character)) = chars.next() {
                    match next_character {
                        '"' => {
                            closed = true;
                            break;
                        }
                        '\\' => {
                            let Some((_, escaped)) = chars.next() else {
                                return Err(format!(
                                    "Unterminated string starting at position {}",
                                    index + 1
                                ));
                            };

                            value.push(match escaped {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                '"' => '"',
                                '\\' => '\\',
                                other => {
                                    return Err(format!(
                                        "Unsupported escape `\\{other}` in string at position {}",
                                        index + 1
                                    ));
                                }
                            });
                        }
                        other => value.push(other),
                    }
                }

                if !closed {
                    return Err(format!(
                        "Unterminated string starting at position {}",
                        index + 1
                    ));
                }

                tokens.push(Token::Value(Value::String(value)));
            }
            character
                if character.is_ascii_digit()
                    || character == '.'
                    || (character == '-'
                        && matches!(chars.peek(), Some((_, next)) if next.is_ascii_digit() || *next == '.')) =>
            {
                let start = index;
                let mut end = index + character.len_utf8();

                while let Some(&(next_index, next_character)) = chars.peek() {
                    if !next_character.is_ascii_digit() && next_character != '.' {
                        break;
                    }

                    chars.next();
                    end = next_index + next_character.len_utf8();
                }

                let raw_number = &input[start..end];
                let number = raw_number
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {raw_number}"))?;

                tokens.push(Token::Value(Value::Number(number)));
            }
            character if character.is_ascii_alphabetic() || character == '_' => {
                let start = index;
                let mut end = index + character.len_utf8();

                while let Some(&(next_index, next_character)) = chars.peek() {
                    if !next_character.is_ascii_alphanumeric() && next_character != '_' {
                        break;
                    }

                    chars.next();
                    end = next_index + next_character.len_utf8();
                }

                match &input[start..end] {
                    "true" => tokens.push(Token::Value(Value::Bool(true))),
                    "false" => tokens.push(Token::Value(Value::Bool(false))),
                    name => tokens.push(Token::Variable((*name).to_string())),
                }
            }
            _ => {
                return Err(format!(
                    "Unexpected character '{character}' at position {}",
                    index + 1
                ));
            }
        }
    }

    Ok(tokens)
}

fn consume_expected(
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    expected: char,
    index: usize,
    hint: &str,
) -> Result<(), String> {
    if matches!(chars.peek(), Some((_, character)) if *character == expected) {
        chars.next();
        return Ok(());
    }

    Err(format!(
        "Unexpected operator at position {}. {hint}",
        index + 1
    ))
}

struct Parser<'a> {
    tokens: Vec<Token>,
    position: usize,
    vars: &'a VarMap,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, vars: &'a VarMap) -> Self {
        Self {
            tokens,
            position: 0,
            vars,
        }
    }

    fn parse(mut self) -> Result<LogicExpression, String> {
        if self.tokens.is_empty() {
            return Err("Expected a LogicExpression".to_string());
        }

        let expression = self.parse_or()?;

        if let Some(token) = self.current() {
            return Err(format!("Unexpected token: {token:?}"));
        }

        Ok(expression)
    }

    fn parse_or(&mut self) -> Result<LogicExpression, String> {
        let mut expression = self.parse_and()?;

        while matches!(self.current(), Some(Token::Or)) {
            self.advance();
            let right = self.parse_and()?;
            expression = LogicExpression::Or(Box::new(expression), Box::new(right));
        }

        Ok(expression)
    }

    fn parse_and(&mut self) -> Result<LogicExpression, String> {
        let mut expression = self.parse_equality()?;

        while matches!(self.current(), Some(Token::And)) {
            self.advance();
            let right = self.parse_equality()?;
            expression = LogicExpression::And(Box::new(expression), Box::new(right));
        }

        Ok(expression)
    }

    fn parse_equality(&mut self) -> Result<LogicExpression, String> {
        let mut expression = self.parse_comparison()?;

        loop {
            expression = match self.current() {
                Some(Token::Equal) => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    LogicExpression::IsEqual(Box::new(expression), Box::new(right))
                }
                Some(Token::NotEqual) => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    LogicExpression::IsNotEqual(Box::new(expression), Box::new(right))
                }
                _ => break,
            };
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<LogicExpression, String> {
        let mut expression = self.parse_unary()?;

        loop {
            expression = match self.current() {
                Some(Token::LessThan) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    LogicExpression::IsLessThan(Box::new(expression), Box::new(right))
                }
                Some(Token::LessThanOrEqual) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    LogicExpression::IsLessThanOrEqual(Box::new(expression), Box::new(right))
                }
                Some(Token::GreaterThan) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    LogicExpression::IsGreaterThan(Box::new(expression), Box::new(right))
                }
                Some(Token::GreaterThanOrEqual) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    LogicExpression::IsGreaterThanOrEqual(Box::new(expression), Box::new(right))
                }
                _ => break,
            };
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<LogicExpression, String> {
        if matches!(self.current(), Some(Token::Not)) {
            self.advance();
            return Ok(LogicExpression::Not(Box::new(self.parse_unary()?)));
        }

        self.parse_operand()
    }

    fn parse_operand(&mut self) -> Result<LogicExpression, String> {
        match self.current() {
            Some(Token::Value(value)) => {
                let value = value.clone();
                self.advance();
                Ok(LogicExpression::Value(value))
            }
            Some(Token::Variable(name)) => {
                let name = name.clone();

                if !self.vars.var_exists(&name) {
                    return Err(format!("Variable not found: {name}"));
                }

                self.advance();
                Ok(LogicExpression::Variable(name))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expression = self.parse_or()?;

                if !matches!(self.current(), Some(Token::RightParen)) {
                    return Err("Expected `)`".to_string());
                }

                self.advance();
                Ok(expression)
            }
            Some(token) => Err(format!("Expected an operand, found {token:?}")),
            None => Err("Expected an operand, found end of LogicExpression".to_string()),
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

pub fn attempt_logic_parse(to_parse: String, vars: &VarMap) -> LogicExpression {
    let tokens = match tokenize(&to_parse) {
        Ok(tokens) => tokens,
        Err(error) => return LogicExpression::Error(error),
    };

    match Parser::new(tokens, vars).parse() {
        Ok(expression) => expression,
        Err(error) => LogicExpression::Error(error),
    }
}

pub fn attempt_logic_run(
    expression: &LogicExpression,
    vars: &VarMap,
    policy: &ExecutionPolicy,
) -> Result<bool, RuntimeError> {
    match evaluate(expression, vars, policy)? {
        Value::Bool(value) => Ok(value),
        value => Err(type_error(format!(
            "LogicExpression must evaluate to a bool, found {}",
            value_type(&value)
        ))),
    }
}

fn evaluate(
    expression: &LogicExpression,
    vars: &VarMap,
    policy: &ExecutionPolicy,
) -> Result<Value, RuntimeError> {
    match expression {
        LogicExpression::Error(error) => Err(type_error(error.clone())),
        LogicExpression::Value(value) => Ok(value.clone()),
        LogicExpression::Variable(name) => resolve_variable(name, vars, policy),
        LogicExpression::Not(expression) => {
            let value = evaluate(expression, vars, policy)?;
            Ok(Value::Bool(!expect_bool(value, "!")?))
        }
        LogicExpression::IsEqual(left, right) => {
            let left = evaluate(left, vars, policy)?;
            let right = evaluate(right, vars, policy)?;
            Ok(Value::Bool(left == right))
        }
        LogicExpression::IsNotEqual(left, right) => {
            let left = evaluate(left, vars, policy)?;
            let right = evaluate(right, vars, policy)?;
            Ok(Value::Bool(left != right))
        }
        LogicExpression::IsLessThan(left, right) => {
            compare_ordered(left, right, vars, policy, "<", |ordering| ordering.is_lt())
        }
        LogicExpression::IsLessThanOrEqual(left, right) => {
            compare_ordered(left, right, vars, policy, "<=", |ordering| ordering.is_le())
        }
        LogicExpression::IsGreaterThan(left, right) => {
            compare_ordered(left, right, vars, policy, ">", |ordering| ordering.is_gt())
        }
        LogicExpression::IsGreaterThanOrEqual(left, right) => {
            compare_ordered(left, right, vars, policy, ">=", |ordering| ordering.is_ge())
        }
        LogicExpression::And(left, right) => {
            let left = expect_bool(evaluate(left, vars, policy)?, "&&")?;

            if !left {
                return Ok(Value::Bool(false));
            }

            Ok(Value::Bool(expect_bool(
                evaluate(right, vars, policy)?,
                "&&",
            )?))
        }
        LogicExpression::Or(left, right) => {
            let left = expect_bool(evaluate(left, vars, policy)?, "||")?;

            if left {
                return Ok(Value::Bool(true));
            }

            Ok(Value::Bool(expect_bool(
                evaluate(right, vars, policy)?,
                "||",
            )?))
        }
    }
}

fn resolve_variable(
    name: &str,
    vars: &VarMap,
    policy: &ExecutionPolicy,
) -> Result<Value, RuntimeError> {
    match vars.get_pure_value(name.to_string()) {
        Some(Value::Undefined) => {
            let error = RuntimeError::new(
                format!("Cannot use undefined variable: {name}"),
                ErrorType::OnUndefinedValue,
            );

            policy.handle_error(error)?.ok_or_else(|| {
                type_error(format!(
                    "No replacement value supplied for undefined variable: {name}"
                ))
            })
        }
        Some(value) => Ok(value),
        None => Err(type_error(format!("Variable not found: {name}"))),
    }
}

fn expect_bool(value: Value, operator: &str) -> Result<bool, RuntimeError> {
    match value {
        Value::Bool(value) => Ok(value),
        value => Err(type_error(format!(
            "Operator `{operator}` requires bool operands, found {}",
            value_type(&value)
        ))),
    }
}

fn compare_ordered(
    left: &LogicExpression,
    right: &LogicExpression,
    vars: &VarMap,
    policy: &ExecutionPolicy,
    operator: &str,
    predicate: impl FnOnce(std::cmp::Ordering) -> bool,
) -> Result<Value, RuntimeError> {
    let left = evaluate(left, vars, policy)?;
    let right = evaluate(right, vars, policy)?;

    let ordering = match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
        (Value::String(left), Value::String(right)) => Some(left.cmp(right)),
        _ => {
            return Err(type_error(format!(
                "Operator `{operator}` requires two numbers or two strings, found {} and {}",
                value_type(&left),
                value_type(&right)
            )));
        }
    };

    ordering
        .map(|ordering| Value::Bool(predicate(ordering)))
        .ok_or_else(|| type_error(format!("Cannot compare values with `{operator}`")))
}

fn value_type(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "bool",
        Value::Undefined => "undefined",
    }
}

fn type_error(message: String) -> RuntimeError {
    RuntimeError::new(message, ErrorType::AlwaysError)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str, vars: &VarMap) -> Result<bool, RuntimeError> {
        let expression = attempt_logic_parse(input.to_string(), vars);
        attempt_logic_run(&expression, vars, &ExecutionPolicy::new())
    }

    fn assert_run(input: &str, vars: &VarMap, expected: bool) {
        match run(input, vars) {
            Ok(value) => assert_eq!(value, expected, "{input}"),
            Err(_) => panic!("expected {input:?} to run successfully"),
        }
    }

    #[test]
    fn parses_comparison_without_spaces() {
        let vars = VarMap::new();

        assert_eq!(
            attempt_logic_parse("2>=1".to_string(), &vars),
            LogicExpression::IsGreaterThanOrEqual(
                Box::new(LogicExpression::Value(Value::Number(2.0))),
                Box::new(LogicExpression::Value(Value::Number(1.0))),
            )
        );
    }

    #[test]
    fn runs_every_comparison_operator() {
        let vars = VarMap::new();

        for input in [
            "2 == 2",
            "2 != 3",
            "2 < 3",
            "2 <= 2",
            "3 > 2",
            "3 >= 3",
            "\"apple\" < \"pear\"",
        ] {
            assert_run(input, &vars, true);
        }
    }

    #[test]
    fn respects_logical_precedence_and_parentheses() {
        let vars = VarMap::new();

        assert_run("true || false && false", &vars, true);
        assert_run("(true || false) && false", &vars, false);
        assert_run("!(2 >= 2) || 3 != 4", &vars, true);
    }

    #[test]
    fn reads_typed_variables() {
        let mut vars = VarMap::new();
        vars.add_new("count".to_string(), "5".to_string(), false)
            .unwrap();
        vars.add_new("enabled".to_string(), "true".to_string(), false)
            .unwrap();

        assert_run("count >= 5 && enabled", &vars, true);
    }

    #[test]
    fn undefined_variable_uses_policy_value() {
        let mut vars = VarMap::new();
        vars.add_new("missing".to_string(), "N/A".to_string(), true)
            .unwrap();
        let mut policy = ExecutionPolicy::new();
        policy
            .change_policy("HandleUndefinedValueAs = true".to_string())
            .unwrap();
        let expression = attempt_logic_parse("missing && true".to_string(), &vars);

        assert!(matches!(
            attempt_logic_run(&expression, &vars, &policy),
            Ok(true)
        ));
    }

    #[test]
    fn rejects_missing_variables_and_bad_operators() {
        let vars = VarMap::new();

        assert!(matches!(
            attempt_logic_parse("unknown == 1".to_string(), &vars),
            LogicExpression::Error(_)
        ));

        for input in ["true & false", "true | false", "1 = 1", "true &&"] {
            assert!(
                matches!(
                    attempt_logic_parse(input.to_string(), &vars),
                    LogicExpression::Error(_)
                ),
                "expected {input:?} to be rejected"
            );
        }
    }

    #[test]
    fn rejects_non_boolean_logical_operands() {
        let vars = VarMap::new();

        assert!(run("1 && true", &vars).is_err());
        assert!(run("1", &vars).is_err());
        assert!(run("true < false", &vars).is_err());
    }

    #[test]
    fn and_or_short_circuit() {
        let mut vars = VarMap::new();
        vars.add_new("missing".to_string(), "N/A".to_string(), true)
            .unwrap();

        assert_run("false && missing", &vars, false);
        assert_run("true || missing", &vars, true);
    }
}
