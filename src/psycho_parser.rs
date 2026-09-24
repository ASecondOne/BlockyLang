#[derive(Debug)]
pub struct PsychoBlock {
    pub block_type: String,
    pub contents: Vec<PsychoCall>,
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
}

pub fn attempt_psycho_parse(file_contents: Vec<String>) -> Vec<PsychoBlock> {
    let mut out = Vec::new();

    for c in file_contents {
        out.extend(psycho_block_parse(prep(&c)));
    }

    out
}

fn psycho_block_parse(contents: String) -> Vec<PsychoBlock> {
    let mut out = Vec::new();
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut block_tag: Option<&str> = None;
    let mut start_tag_pos: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("<") && line.ends_with(">") && !line.starts_with("</") {
            block_tag = Some(
                line.strip_prefix("<")
                    .unwrap()
                    .strip_suffix(">")
                    .unwrap(),
            );

            start_tag_pos = Some(i);
        }

        if line.starts_with("</") && line.ends_with(">") && block_tag.is_some() {
            // TODO: Make psycho parser return in case of an start-less end tag
            let end_block_tag = line
                .strip_prefix("</")
                .unwrap()
                .strip_suffix(">")
                .unwrap();

            if end_block_tag == block_tag.unwrap() {
                let content_between_tags =
                    get_lines_between_tags(&lines, start_tag_pos.unwrap(), i);

                let psycho_lines: Vec<PsychoCall> = content_between_tags
                    .iter()
                    .filter_map(|line| psycho_line_parse(line))
                    .collect();

                out.push(PsychoBlock {
                    block_type: block_tag.unwrap().to_string(),
                    contents: psycho_lines,
                });
            }
        }
    }

    out
}

fn psycho_line_parse(contents: &str) -> Option<PsychoCall> {
    let contents = contents.trim();

    if contents.is_empty() {
        return None;
    }

    parse_call(contents)
}

// * Parses either:
// * foo bar
// * foo(bar)
// * bar.foo()
fn parse_call(contents: &str) -> Option<PsychoCall> {
    let contents = contents.trim();

    // bar.foo()
    if let Some(i) = find_top_level_dot(contents) {
        let left = &contents[..i];
        let right = &contents[i + 1..];

        if let Some(mut call) = parse_call(right) {
            call.expressions.insert(0, parse_expression(left));
            return Some(call);
        }
    }

    // foo bar
    if let Some(i) = find_top_level_space(contents) {
        let keyword = contents[..i].trim();
        let expressions = contents[i..].trim();

        return Some(PsychoCall {
            keyword: keyword.to_string(),
            expressions: parse_arguments(expressions),
        });
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

    if find_top_level_dot(contents).is_some()
        || find_call_open(contents).is_some()
        || find_top_level_space(contents).is_some()
    {
        if let Some(call) = parse_call(contents) {
            return PsychoExpression::Call(call);
        }
    }

    PsychoExpression::Expression(contents.to_string())
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

fn find_top_level_space(contents: &str) -> Option<usize> {
    let mut string = false;
    let mut depth = 0;

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

            c if c.is_whitespace() && depth == 0 => {
                return Some(i);
            }

            _ => {}
        }
    }

    None
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

fn find_top_level_dot(contents: &str) -> Option<usize> {
    let mut string = false;
    let mut depth = 0;
    let mut found = None;

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

            '.' if depth == 0 => {
                found = Some(i);
            }

            _ => {}
        }
    }

    found
}

fn prep(src: &str) -> String {
    let mut out = String::new();

    let mut string = false;
    let mut space = false;

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

        if c.is_whitespace() {
            space = true;
            continue;
        }

        if space && !out.ends_with('\n') && c != '<' {
            out.push(' ');
        }

        space = false;

        if c == ';' {
            out.push('\n');
            continue;
        }

        out.push(c);

        if c == '>' {
            out.push('\n');
        }
    }

    out
}

fn get_lines_between_tags<'a>(lines: &'a [&'a str],start: usize,end: usize) -> &'a [&'a str] {
    &lines[start + 1..end]
}