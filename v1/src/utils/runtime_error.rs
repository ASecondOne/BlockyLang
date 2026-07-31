use colored::Colorize;

pub struct RuntimeError {
    message: String,
    etype: ErrorType
}

pub enum ErrorType {
    AlwaysError,
    OnCodeBlockParseError,
    OnUndefinedValue
}

impl RuntimeError {
    pub fn new(message: String, etype: ErrorType) -> Self {
        RuntimeError { message, etype}
    }

    pub(crate) fn get_error_type(&self) -> &ErrorType {
        &self.etype
    }

    pub fn report(&self) {
        eprintln!("{} {}", "An error occurred:".red(), self.message.red())
    }
}
