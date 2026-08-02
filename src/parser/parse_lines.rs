use std::mem::take;

#[derive(Debug)]
pub enum Expression {
    None
}

fn prepare_strings(lines: Vec<&str>) -> Vec<String> {
    let mut out = Vec::new();

    let mut unfinished_line = String::new();

    for line in lines {   
        for char in line.trim_start().chars() {
            if char == ';' {
                out.push(take(&mut unfinished_line));
            } else {
                unfinished_line.push(char);
            }
        }
    }

    out
}

pub fn parse_lines(lines: Vec<&str>) -> Expression {

    let lines = prepare_strings(lines);

    for line in lines {
        println!("{line}")
    }

    Expression::None
}