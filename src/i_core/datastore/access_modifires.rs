use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::{datastore::var::Variable, value::Value}, symbol_resolver::SharedExpression};

#[derive(Debug, Clone)]
pub enum AccessModifires {
    Mutabl,
    Borrow,
}

pub fn i_get_AcMods(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    if args.is_empty() {
        return ExcuterOutput::Error("`get_AcMods` requires one argument".to_string());
    }

    let arg = args.remove(0);
    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExcuterOutput::Error("Could not lock `println` argument".to_string()),
    };

    if let Some(var) = arg.as_any().downcast_ref::<Variable>() {
        return ExcuterOutput::ValidSome(
            Arc::new(
                Mutex::new(
                    Value::String(
                        { // //! Needs to be later put into an acctual type and vec
                            let mut out = String::new();

                            for a_m in &var.access_modifires {
                                out.push_str(&format!("{:#?} ", a_m));
                            }

                            out
                        }
                    )
                )
            )
        );
    }

    ExcuterOutput::ValidNone
}