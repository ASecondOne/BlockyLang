use std::sync::{Arc, Mutex, atomic::AtomicU64};

use crate::{
    i_core::datastore::{VARIABLES, var::Variable},
    symbolresolver::symbol_resolver::SharedExpression,
};

#[derive(Debug, Default)]
pub struct LocalState {
    active: bool,
    scope_id: String,
    variables: std::collections::HashMap<String, SharedExpression>,
    parent_variables: std::collections::HashMap<String, SharedExpression>,
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
            expression.as_any().downcast_ref::<Variable>().cloned()
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
            scope_id: format!(
                "closure{}",
                NEXT_LOCAL_SCOPE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ),
            variables: std::collections::HashMap::with_capacity(self.variables.len()),
            parent_variables: std::collections::HashMap::with_capacity(self.variables.len()),
        };

        for (origin, expression) in &self.variables {
            let variable = {
                let expression = expression
                    .lock()
                    .map_err(|_| "Could not copy enclosing closure state".to_string())?;
                expression.as_any().downcast_ref::<Variable>().cloned()
            };

            if let Some(mut variable) = variable {
                forked
                    .parent_variables
                    .insert(variable.identity.clone(), Arc::clone(expression));
                variable.origin = variable.identity.clone();
                variable.identity = format!("{}/{}", forked.scope_id, variable.name);
                forked
                    .variables
                    .insert(origin.clone(), Arc::new(Mutex::new(variable)));
            }
        }

        Ok(forked)
    }

    pub fn transfer(&self, local: SharedExpression) -> Result<(), String> {
        if !self.active {
            return Err("`transfer` must be called inside a closure".to_string());
        }

        let (origin, value, access_modifires) = {
            let local = local
                .lock()
                .map_err(|_| "Could not lock local variable for transfer".to_string())?;
            let variable = local
                .as_any()
                .downcast_ref::<Variable>()
                .ok_or_else(|| "`transfer` requires a variable".to_string())?;
            (
                variable.origin.clone(),
                variable.value.clone(),
                variable.access_modifires.clone(),
            )
        };

        let target = if let Some(parent) = self.parent_variables.get(&origin) {
            Arc::clone(parent)
        } else {
            let variables = VARIABLES
                .lock()
                .map_err(|_| "Could not find the original variable".to_string())?;

            variables
                .iter()
                .find_map(|candidate_expression| {
                    candidate_expression.lock().ok().and_then(|candidate| {
                        candidate
                            .as_any()
                            .downcast_ref::<Variable>()
                            .filter(|variable| variable.identity == origin)
                            .map(|_| Arc::clone(candidate_expression))
                    })
                })
                .ok_or_else(|| format!("Could not find transfer origin `{origin}`"))?
        };

        let mut target = target
            .lock()
            .map_err(|_| format!("Could not lock transfer origin `{origin}`"))?;
        let target = target
            .as_any_mut()
            .downcast_mut::<Variable>()
            .ok_or_else(|| format!("Transfer origin `{origin}` is not a variable"))?;

        target.value = value;
        target.access_modifires = access_modifires;
        Ok(())
    }
}
