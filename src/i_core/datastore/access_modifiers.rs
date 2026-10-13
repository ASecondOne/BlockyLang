use std::collections::HashSet;

use crate::{executers::ExecuterOutput, i_core::{datastore::helpers::{get_variable, string_expression, take_first_argument, update_variable}, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::SharedExpression}};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum AccessModifiers {
    Mutabl,
    OneTimeMutabl,
    Transferabl,
}

pub fn i_get_acmods(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`get_AcMods` requires one argument".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(var)) => {
            let mut out = String::new();
            for access_modifier in &var.access_modifiers {
                out.push_str(&format!("{access_modifier:#?} "));
            }
            ExecuterOutput::ValidSome(string_expression(out))
        }
        Ok(None) => ExecuterOutput::ValidNone,
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_set_acmods(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`get_AcMods` requires one argument".to_string());
    };

    let mut to_set: HashSet<AccessModifiers> = HashSet::new();

    for arg in args {
        let value = match arg.lock() {
            Ok(guard) => guard.as_any().downcast_ref::<Value>().cloned(),
            Err(_) => return ExecuterOutput::Error("Could not lock access modifier".to_string()),
        };

        match value {
            Some(Value::String(s)) => {
                if s.is_none() {
                    return ExecuterOutput::Error("Value has to be set for this operation".to_string());
                }

                let s = s.unwrap();

                if s.to_uppercase().trim() == "MUTABL" {
                    to_set.insert(AccessModifiers::Mutabl);
                }
                
                if s.to_ascii_uppercase().trim() == "TRANSFERABL" {
                    to_set.insert(AccessModifiers::Transferabl);
                }
            }
            _ => return ExecuterOutput::Error("`set_AcMods` requires string modifiers".to_string()),
        }
    }

    match update_variable(&arg, |var| var.access_modifiers = to_set) {
        Ok(true) => ExecuterOutput::ValidNone,
        Ok(false) => ExecuterOutput::ValidNone,
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_mut(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    if args.len() != 1 {
        return ExecuterOutput::Error("`mut` requires one variable".to_string());
    }

    let Some(variable) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`mut` requires a variable".to_string());
    };

    match update_variable(&variable, |variable| {
        variable.access_modifiers.insert(AccessModifiers::Mutabl);
    }) {
        Ok(true) => ExecuterOutput::ValidNone,
        Ok(false) => ExecuterOutput::Error("`mut` requires a variable".to_string()),
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_trans(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    if args.len() != 1 {
        return ExecuterOutput::Error("`trans` requires one variable".to_string());
    }

    let Some(variable) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`trans` requires a variable".to_string());
    };

    match update_variable(&variable, |variable| {
        variable.access_modifiers.insert(AccessModifiers::Transferabl);
    }) {
        Ok(true) => ExecuterOutput::ValidNone,
        Ok(false) => ExecuterOutput::Error("`trans` requires a variable".to_string()),
        Err(message) => ExecuterOutput::Error(message),
    }
}
