use crate::{utils::runtime_error::{ErrorType, RuntimeError}, var_handler::{Value, parse_type}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OnCodeBlockParseError {
    Skip,
    HaltProgram,
}

#[derive(Clone, Debug, PartialEq)]
enum HandleUndefinedValueAs {
    HaltProgram,
    Value(Value)
}

enum Property {
    OnCodeBlockParseError(OnCodeBlockParseError),
    HandleUndefinedValueAs(HandleUndefinedValueAs),
}

impl Property {
    fn parse(line: &str) -> Result<Self, String> {
        let (name, value) = line.split_once('=').ok_or("Missing '='")?;

        let name = name.trim();
        let value = value.trim();

        match name {
            "OnCodeBlockParseError" => match value {
                "Skip" => Ok(Property::OnCodeBlockParseError(OnCodeBlockParseError::Skip)),
                "HaltProgram" => Ok(Property::OnCodeBlockParseError(
                    OnCodeBlockParseError::HaltProgram,
                )),
                _ => Err(format!("Invalid value for OnCodeBlockParseError: {value}")),
            },
            "HandleUndefinedValueAs" => match value {
                "HaltProgram" => Ok(Property::HandleUndefinedValueAs(HandleUndefinedValueAs::HaltProgram)),
                _ => match parse_type(value, false) {
                    Ok(v) => Ok(Property::HandleUndefinedValueAs(HandleUndefinedValueAs::Value(v))),
                    Err(_e) => Err(format!("Invalid value for HandleUndefinedValueAs: {value}"))
                }
            },
            _ => Err(format!("Unknown property: {name}")),
        }
    }
}

#[derive(Clone)]
pub struct ExecutionPolicy {
    on_code_block_parse_error: OnCodeBlockParseError,
    handle_undefined_value_as: HandleUndefinedValueAs
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            on_code_block_parse_error: OnCodeBlockParseError::HaltProgram,
            handle_undefined_value_as: HandleUndefinedValueAs::HaltProgram,
        }
    }
}

impl ExecutionPolicy {
    pub fn new() -> Self {
        ExecutionPolicy::default()
    }

    pub(crate) fn should_halt_on_code_block_parse_error(&self) -> bool {
        matches!(
            self.on_code_block_parse_error,
            OnCodeBlockParseError::HaltProgram
        )
    }

    pub(crate) fn handle_error(&self, error: RuntimeError) -> Result<Option<Value>, RuntimeError> {
        match error.get_error_type() {
            ErrorType::AlwaysError => Err(error),
            ErrorType::OnCodeBlockParseError => {
                if self.should_halt_on_code_block_parse_error() {
                    return Err(error);
                }

                Ok(None)
            },
            ErrorType::OnUndefinedValue => match &self.handle_undefined_value_as {
                HandleUndefinedValueAs::HaltProgram => Err(error),
                HandleUndefinedValueAs::Value(value) => Ok(Some(value.clone()))
            }
        }
    }

    pub fn change_policy(&mut self, raw: String) -> Result<(), String> {
        for (line_index, line) in raw.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let property = Property::parse(line).map_err(|error| {
                format!(
                    "invalid execution policy on line {}: {error}",
                    line_index + 1
                )
            })?;

            self.apply(property);
        }

        Ok(())
    }

    fn apply(&mut self, property: Property) {
        match property {
            Property::OnCodeBlockParseError(v) => {
                self.on_code_block_parse_error = v;
            },
            Property::HandleUndefinedValueAs(v) => {
                self.handle_undefined_value_as = v
            }
        }
    }
}

#[test]
fn test_undefined_value_policy_returns_its_value() {
    let mut policy = ExecutionPolicy::new();
    let _ = policy.change_policy("HandleUndefinedValueAs = \"fallback\"".to_string());

    let error = RuntimeError::new(
        "Cannot use an undefind value".to_string(),
        ErrorType::OnUndefinedValue
    );

    match policy.handle_error(error) {
        Ok(Some(Value::String(value))) => assert_eq!(value, "fallback"),
        _ => panic!("the policy should return its fallback value"),
    }
}
