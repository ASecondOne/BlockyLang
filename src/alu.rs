use crate::var_handler::VarMap;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Error(String),
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((index, character)) = chars.next() {
        match character {
            character if character.is_whitespace() => {}
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            character if character.is_ascii_digit() || character == '.' => {
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

                tokens.push(Token::Number(number));
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

                tokens.push(Token::Variable(input[start..end].to_string()));
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

    fn parse(mut self) -> Result<Expression, String> {
        if self.tokens.is_empty() {
            return Err("Expected an expression".to_string());
        }

        let expression = self.parse_expression()?;

        if let Some(token) = self.current() {
            return Err(format!("Unexpected token: {token:?}"));
        }

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_term()?;

        loop {
            match self.current() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    expression = Expression::Add(Box::new(expression), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    expression = Expression::Subtract(Box::new(expression), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_operand()?;

        loop {
            match self.current() {
                Some(Token::Multiply) => {
                    self.advance();
                    let right = self.parse_operand()?;
                    expression = Expression::Multiply(Box::new(expression), Box::new(right));
                }
                Some(Token::Divide) => {
                    self.advance();
                    let right = self.parse_operand()?;
                    expression = Expression::Divide(Box::new(expression), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_operand(&mut self) -> Result<Expression, String> {
        match self.current() {
            Some(Token::Number(number)) => {
                let number = *number;
                self.advance();
                Ok(Expression::Number(number))
            }
            Some(Token::Variable(name)) => {
                let name = name.clone();

                if !self.vars.var_exists(&name) {
                    return Err(format!("Variable not found: {name}"));
                }

                self.advance();
                Ok(Expression::Variable(name))
            }
            Some(token) => Err(format!("Expected an operand, found {token:?}")),
            None => Err("Expected an operand, found end of expression".to_string()),
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

pub fn attempt_calculator_parse(to_parse: String, vars: &VarMap) -> Expression {
    let tokens = match tokenize(&to_parse) {
        Ok(tokens) => tokens,
        Err(error) => return Expression::Error(error),
    };

    match Parser::new(tokens, vars).parse() {
        Ok(expression) => expression,
        Err(error) => Expression::Error(error),
    }
}

pub fn attempt_calculator_run(exp: &Expression, vars: &VarMap) -> Result<f64, String> {
    match exp {
        Expression::Error(error) => Err(error.clone()),

        Expression::Number(number) => Ok(*number),

        Expression::Variable(var) => {
            if let Some(var) = vars.get_var(var.clone()) {
                match var.parse::<f64>() {
                    Ok(num) => return Ok(num),
                    Err(_) => return Err("Error while parsing".to_string()),
                }
            }

            Err("failed getting variable".to_string())
        }

        Expression::Add(left, right) => {
            Ok(attempt_calculator_run(left, vars)? + attempt_calculator_run(right, vars)?)
        }

        Expression::Subtract(left, right) => {
            Ok(attempt_calculator_run(left, vars)? - attempt_calculator_run(right, vars)?)
        }

        Expression::Multiply(left, right) => {
            Ok(attempt_calculator_run(left, vars)? * attempt_calculator_run(right, vars)?)
        }

        Expression::Divide(left, right) => {
            let left = attempt_calculator_run(left, vars)?;
            let right = attempt_calculator_run(right, vars)?;

            if right == 0.0 {
                return Err("Cannot divide by zero".to_string());
            }

            Ok(left / right)
        }
    }
}

#[test]
fn test_simple_division() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("8 / 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Number(8.0)),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_multiple_multiplications() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 * 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Multiply(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Number(4.0)),
        )
    );
}

#[test]
fn test_multiple_divisions() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("16 / 4 / 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(16.0)),
                Box::new(Expression::Number(4.0)),
            )),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_multiplication_then_division() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Number(4.0)),
        )
    );
}

#[test]
fn test_division_then_multiplication() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("8 / 4 * 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Multiply(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_division_precedence() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1 + 8 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
        )
    );
}

#[test]
fn test_multiple_high_precedence_expressions() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 + 8 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
        )
    );
}

#[test]
fn test_long_multiplication_and_division_chain() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 / 4 * 5 / 6".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Multiply(
                Box::new(Expression::Divide(
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                    Box::new(Expression::Number(4.0)),
                )),
                Box::new(Expression::Number(5.0)),
            )),
            Box::new(Expression::Number(6.0)),
        )
    );
}

#[test]
fn test_all_operators_and_precedence() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1 + 2 * 3 - 8 / 4 + 5 * 6 / 3".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Subtract(
                Box::new(Expression::Add(
                    Box::new(Expression::Number(1.0)),
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                )),
                Box::new(Expression::Divide(
                    Box::new(Expression::Number(8.0)),
                    Box::new(Expression::Number(4.0)),
                )),
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Multiply(
                    Box::new(Expression::Number(5.0)),
                    Box::new(Expression::Number(6.0)),
                )),
                Box::new(Expression::Number(3.0)),
            )),
        )
    );
}

#[test]
fn test_expression_without_spaces() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1+2*3/4-5".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Divide(
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                    Box::new(Expression::Number(4.0)),
                )),
            )),
            Box::new(Expression::Number(5.0)),
        )
    );
}

#[test]
fn test_variable_with_digit_on_either_side() {
    let mut vars = VarMap::new();
    let _ = vars.add_new("foo2".to_string(), "7".to_string());

    assert_eq!(
        attempt_calculator_parse("foo2 + 1".to_string(), &vars),
        Expression::Add(
            Box::new(Expression::Variable("foo2".to_string())),
            Box::new(Expression::Number(1.0)),
        )
    );

    assert_eq!(
        attempt_calculator_parse("1 + foo2".to_string(), &vars),
        Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Variable("foo2".to_string())),
        )
    );
}

#[test]
fn test_undefined_variable_with_digit_is_an_error() {
    let vars = VarMap::new();

    assert_eq!(
        attempt_calculator_parse("foo8 + 1".to_string(), &vars),
        Expression::Error("Variable not found: foo8".to_string())
    );
}

#[test]
fn test_missing_and_double_operands_are_errors() {
    let vars = VarMap::new();

    for input in ["2 +", "+ 2", "2 + * 3", "2 3"] {
        assert!(
            matches!(
                attempt_calculator_parse(input.to_string(), &vars),
                Expression::Error(_)
            ),
            "expected {input:?} to be rejected"
        );
    }
}

#[test]
fn test_unknown_characters_are_errors() {
    let vars = VarMap::new();

    assert_eq!(
        attempt_calculator_parse("2 $ + 3".to_string(), &vars),
        Expression::Error("Unexpected character '$' at position 3".to_string())
    );
}
