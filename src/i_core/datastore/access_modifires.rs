use crate::{executers::ExcuterOutput, i_core::datastore::helpers::{get_variable, string_expression, take_first_argument}, symbolresolver::symbol_resolver::SharedExpression};

#[derive(Debug, Clone)]
pub enum AccessModifires {

}

pub fn i_get_acmods(mut args: Vec<SharedExpression>, _local_state: &mut crate::symbolresolver::localstate::LocalState) -> ExcuterOutput {
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
