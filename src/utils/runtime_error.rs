use colored::Colorize;

pub struct RuntimeError {
    message: String
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError { message }
    }

    pub fn report(&self) {
        eprintln!("{} {}", "An error occurred:".red(), self.message.red())
    }
}