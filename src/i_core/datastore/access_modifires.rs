use std::collections::HashSet;

use crate::{executers::ExcuterOutput, i_core::{datastore::helpers::{get_variable, string_expression, take_first_argument, update_variable}, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::SharedExpression}};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum AccessModifires {
    Mutabl,
    OneTimeMutabl,
}

pub fn i_get_acmods(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`get_AcMods` requires one argument".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(var)) => {
            let mut out = String::new();
            for access_modifier in &var.access_modifires {
                out.push_str(&format!("{access_modifier:#?} "));
            }
            ExcuterOutput::ValidSome(string_expression(out))
        }
        Ok(None) => ExcuterOutput::ValidNone,
        Err(message) => ExcuterOutput::Error(message),
    }
}

pub fn i_set_acmods(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`get_AcMods` requires one argument".to_string());
    };

    let mut to_set: HashSet<AccessModifires> = HashSet::new();

    for arg in args {
        let value = match arg.lock() {
            Ok(guard) => guard.as_any().downcast_ref::<Value>().cloned(),
            Err(_) => return ExcuterOutput::Error("Could not lock access modifier".to_string()),
        };

        match value {
            Some(Value::String(s)) => {
                if s.is_none() {
                    return ExcuterOutput::Error("Value has to be set for this operation".to_string());
                }
                if s.unwrap().to_uppercase().trim() == "MUTABL" {
                    to_set.insert(AccessModifires::Mutabl);
                }
            }
            _ => return ExcuterOutput::Error("`set_AcMods` requires string modifiers".to_string()),
        }
    }

    match update_variable(&arg, |var| var.access_modifires = to_set) {
        Ok(true) => ExcuterOutput::ValidNone,
        Ok(false) => ExcuterOutput::ValidNone,
        Err(message) => ExcuterOutput::Error(message),
    }
}

pub fn i_mut(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    if args.len() != 1 {
        return ExcuterOutput::Error("`mut` requires one variable".to_string());
    }

    let Some(variable) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`mut` requires a variable".to_string());
    };

    match update_variable(&variable, |variable| {
        variable.access_modifires.insert(AccessModifires::Mutabl);
    }) {
        Ok(true) => ExcuterOutput::ValidNone,
        Ok(false) => ExcuterOutput::Error("`mut` requires a variable".to_string()),
        Err(message) => ExcuterOutput::Error(message),
    }
}
