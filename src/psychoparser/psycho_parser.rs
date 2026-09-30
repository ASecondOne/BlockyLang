use colored::Colorize;

use crate::psychoparser::top_level_finder::{find_top_level_dot, find_top_level_redirect, find_top_level_space};

#[derive(Debug)]
pub struct PsychoBlock {
    pub block_type: String,
    pub contents: Vec<PsychoExpression>,
}

#[derive(Debug)]
pub struct PsychoCall {
    pub keyword: String,
    pub expressions: Vec<PsychoExpression>,
}

#[derive(Debug)]
pub enum PsychoExpression {
    Expression(String),
    Call(PsychoCall),
    Redirect(Box<PsychoExpression>, Box<PsychoExpression>),
    Closure(Vec<PsychoExpression>),
}

pub fn attempt_psycho_parse(file_contents: Vec<String>) -> Result<Vec<PsychoBlock>, ()> {
    let mut out = Vec::new();
    let mut had_errors = false;

    for c in file_contents {
        let (blocks, file_had_errors) = psycho_block_parse(prep(&c));
        out.extend(blocks);
        had_errors |= file_had_errors;
    }

    if had_errors { Err(()) } else { Ok(out) }
}

fn psycho_block_parse(contents: String) -> (Vec<PsychoBlock>, bool) {
    let mut out = Vec::new();
    let mut had_errors = false;
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut block_tag: Option<&str> = None;
    let mut start_tag_pos: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("<") && line.ends_with(">") && !line.starts_with("</") {
            if let Some(open_tag) = block_tag {
                parse_error(format!(
                    "Found `<{}>` before `<{open_tag}>` was closed",
                    &line[1..line.len() - 1]
                ));
                had_errors = true;
                continue;
            }

            block_tag = Some(&line[1..line.len() - 1]);
            start_tag_pos = Some(i);
            continue;
        }

        if line.starts_with("</") && line.ends_with(">") {
            let end_block_tag = &line[2..line.len() - 1];

            let Some(open_tag) = block_tag else {
                parse_error(format!("Found closing tag `</{end_block_tag}>` without an opening tag"));
                had_errors = true;
                continue;
            };

            if end_block_tag != open_tag {
                parse_error(format!(
                    "Closing tag `</{end_block_tag}>` does not match `<{open_tag}>`"
                ));
                had_errors = true;
                continue;
            }

            let content_between_tags =
                get_lines_between_tags(&lines, start_tag_pos.unwrap(), i);

            let mut psycho_lines: Vec<PsychoExpression> = Vec::new();

            for line in content_between_tags {
                match psycho_line_parse(line) {
                    Ok(Some(expression)) => psycho_lines.push(expression),
                    Ok(None) => {}
                    Err(()) => had_errors = true,
                }
            }

            out.push(PsychoBlock {
                block_type: open_tag.to_string(),
                contents: psycho_lines,
            });

            block_tag = None;
            start_tag_pos = None;
        }
    }

    if let Some(open_tag) = block_tag {
        parse_error(format!("Block `<{open_tag}>` has no closing tag"));
        had_errors = true;
    }

    (out, had_errors)
}

fn psycho_line_parse(contents: &str) -> Result<Option<PsychoExpression>, ()> {
    let contents = contents.trim();

    if contents.is_empty() {
        return Ok(None);
    }

    if let Err(message) = validate_expression_syntax(contents) {
        parse_error(format!("Invalid expression `{contents}`: {message}"));
        return Err(());
    }

    Ok(Some(parse_expression(contents)))
}

fn validate_expression_syntax(contents: &str) -> Result<(), String> {
    let mut string = false;
    let mut depth = 0usize;
    let mut closure_depth = 0usize;

    for c in contents.chars() {
        if c == '"' {
            string = !string;
            continue;
        }

        if string {
            continue;
        }

        match c {
            '(' => depth += 1,
            ')' if depth == 0 => return Err("unexpected `)`".to_string()),
            ')' => depth -= 1,

            '{' => closure_depth += 1,
            '}' if closure_depth == 0 => return Err("unexpected `}`".to_string()),
            '}' => closure_depth -= 1,
            _ => {}
        }
    }

    if string {
        return Err("unclosed string".to_string());
    }

    if depth != 0 {
        return Err("unclosed `(`".to_string());
    }

    if let Some(i) = find_top_level_redirect(contents) {
        if contents[..i].trim().is_empty() || contents[i + 1..].trim().is_empty() {
            return Err("redirect requires expressions on both sides of `=`".to_string());
        }
    }

    Ok(())
}

fn parse_error(message: String) {
    eprintln!("{}", format!("Parse error: {message}").red());
}

// Parses either:
// foo bar
// foo(bar)
// bar.foo()
fn parse_call(contents: &str) -> Option<PsychoCall> {
    let contents = contents.trim();

    // foo bar
    // Parse the outer call before dot notation inside its arguments.
    if let Some(i) = find_top_level_space(contents) {
        let keyword = contents[..i].trim();
        let expressions = contents[i..].trim();

        return Some(PsychoCall {
            keyword: keyword.to_string(),
            expressions: parse_arguments(expressions),
        });
    }

    // bar.foo()
    if let Some(i) = find_top_level_dot(contents) {
        let left = &contents[..i];
        let right = &contents[i + 1..];

        if let Some(mut call) = parse_call(right) {
            call.expressions.insert(0, parse_expression(left));
            return Some(call);
        }
    }

    // foo(bar)
    if let Some(i) = find_call_open(contents) {
        if contents.ends_with(')') {
            let keyword = contents[..i].trim();
            let inner = &contents[i + 1..contents.len() - 1];

            return Some(PsychoCall {
                keyword: keyword.to_string(),
                expressions: parse_arguments(inner),
            });
        }
    }

    None
}

fn parse_expression(contents: &str) -> PsychoExpression {
    let contents = contents.trim();

    if is_complete_closure(contents) {
        let body = &contents[1..contents.len() - 1];
        let expressions = split_closure_body(body)
            .into_iter()
            .filter(|expression| !expression.trim().is_empty())
            .map(parse_expression)
            .collect();

        return PsychoExpression::Closure(expressions);
    }

    if let Some(i) = find_top_level_redirect(contents) {
        return PsychoExpression::Redirect(
            Box::new(parse_expression(&contents[..i])),
            Box::new(parse_expression(&contents[i + 1..])),
        );
    }

    if let Some(call) = parse_call(contents) {
        return PsychoExpression::Call(call);
    }

    PsychoExpression::Expression(contents.to_string())
}

fn is_complete_closure(contents: &str) -> bool {
    if !contents.starts_with('{') || !contents.ends_with('}') {
        return false;
    }

    let mut depth = 0usize;
    let mut string = false;

    for (index, c) in contents.char_indices() {
        if c == '"' {
            string = !string;
            continue;
        }

        if string {
            continue;
        }

        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 && index != contents.len() - 1 {
                    return false;
                }
            }
            _ => {}
        }
    }

    depth == 0 && !string
}

fn split_closure_body(contents: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut parentheses = 0usize;
    let mut braces = 0usize;
    let mut string = false;

    for (index, c) in contents.char_indices() {
        if c == '"' {
            string = !string;
            continue;
        }

        if string {
            continue;
        }

        match c {
            '(' => parentheses += 1,
            ')' => parentheses = parentheses.saturating_sub(1),
            '{' => braces += 1,
            '}' => braces = braces.saturating_sub(1),
            ';' if parentheses == 0 && braces == 0 => {
                out.push(contents[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }

    out.push(contents[start..].trim());
    out
}

fn parse_arguments(contents: &str) -> Vec<PsychoExpression> {
    let contents = contents.trim();

    if contents.is_empty() {
        return Vec::new();
    }

    split_arguments(contents)
        .into_iter()
        .map(parse_expression)
        .collect()
}

fn split_arguments(contents: &str) -> Vec<&str> {
    let mut out = Vec::new();

    let mut string = false;
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in contents.char_indices() {
        if c == '"' {
            string = !string;
            continue;
        }

        if string {
            continue;
        }

        match c {
            '(' => depth += 1,

            ')' => depth -= 1,

            ',' if depth == 0 => {
                out.push(contents[start..i].trim());
                start = i + 1;
            }

            _ => {}
        }
    }

    out.push(contents[start..].trim());

    out
}

fn find_call_open(contents: &str) -> Option<usize> {
    let mut string = false;

    for (i, c) in contents.char_indices() {
        if c == '"' {
            string = !string;
            continue;
        }

        if !string && c == '(' {
            return Some(i);
        }
    }

    None
}

fn prep(src: &str) -> String {
    let mut out = String::new();

    let mut string = false;
    let mut space = false;
    let mut closure_depth = 0usize;

    for c in src.chars() {
        if c == '"' {
            if !string
                && out
                    .chars()
                    .last()
                    .is_some_and(|c| c.is_alphanumeric())
            {
                out.push(' ');
            }

            string = !string;
            out.push(c);
            continue;
        }

        if string {
            if c == '\n' {
                out.push_str("\\\n");
            } else {
                out.push(c);
            }

            continue;
        }

        if c == '\n' {
            space = false;
            if closure_depth == 0 && !out.ends_with('\n') {
                out.push('\n');
            }
            continue;
        }

        if c.is_whitespace() {
            space = true;
            continue;
        }

        if space && !out.ends_with('\n') {
            if c == '<' {
                out.push('\n');
            } else {
                out.push(' ');
            }
        }

        space = false;

        if c == ';' {
            if closure_depth == 0 {
                out.push('\n');
            } else {
                out.push(';');
            }
            continue;
        }

        out.push(c);

        match c {
            '{' => closure_depth += 1,
            '}' => closure_depth = closure_depth.saturating_sub(1),
            _ => {}
        }

        if c == '>' {
            out.push('\n');
        }
    }

    out
}

fn get_lines_between_tags<'a>(lines: &'a [&'a str], start: usize, end: usize) -> &'a [&'a str] {
    &lines[start + 1..end]
}
