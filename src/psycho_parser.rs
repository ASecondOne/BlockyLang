#[derive(Debug)]
pub struct PsychoBlock {
    block_type: String,
    contents: Vec<Vec<PsychoLine>>
}

#[derive(Debug)]
pub enum PsychoLine {
    Node(PsychoType),

    Space
}

#[derive(Debug)]
pub enum PsychoType {
    Unsure(Vec<PsychoLine>),

    Keyword(String),
    Expression(String),
}

pub fn attempt_psycho_parse(file_contents: Vec<String>) -> Vec<PsychoBlock> {
    let mut out = Vec::new();

    for c in file_contents {
        let blocks = psycho_block_parse(prep(&c));

        for block in &blocks {
            println!("{:#?}", block);
        }

        out.extend(blocks);
    }

    out
}

fn psycho_block_parse(contents: String) -> Vec<PsychoBlock> {

    println!("{contents}");

    let mut out = Vec::new();

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut block_tag: Option<&str> = None;
    let mut start_tag_pos: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("<") && line.ends_with(">") && !line.starts_with("</") {
            block_tag = Some(line.strip_prefix("<").unwrap().strip_suffix(">").unwrap());
            start_tag_pos = Some(i);
        }

        if line.starts_with("</") && line.ends_with(">") && block_tag.is_some() {
            // TODO: Make psycho parser return in case of an start-less end tag

            let end_block_tag = line.strip_prefix("</").unwrap().strip_suffix(">").unwrap();

            if end_block_tag == block_tag.unwrap() {
                let content_between_tags = get_lines_between_tags(&lines, start_tag_pos.unwrap(), i);

                let psycho_lines: Vec<Vec<PsychoLine>> = content_between_tags.iter()
                    .map(|c| psycho_line_parse(&mut Vec::new(), c).unwrap()) 
                    .collect();

                out.push(PsychoBlock { block_type: block_tag.unwrap().to_string(), contents: psycho_lines });
            }
        }
    }

    out
}

fn psycho_line_parse(out: &mut Vec<PsychoLine>, contents: &str) -> Option<Vec<PsychoLine>> {
    let contents = contents.trim_start();

    if contents.is_empty() {
        return None;
    }

    if contents.starts_with(';') {
        return Some(std::mem::take(out));
    }

    let mut string = false;
    let mut end = contents.len();

    for (i, c) in contents.char_indices() {
        if c == '"' {
            string = !string;
        }

        if !string && (c.is_whitespace() || c == ';') {
            end = i;
            break;
        }
    }

    let value = &contents[..end];
    let rest = &contents[end..];

    if !value.is_empty() {
        let psycho_type = match out.last() {
            Some(PsychoLine::Space) => {
                PsychoType::Expression(value.to_string())
            }

            None => {
                PsychoType::Keyword(value.to_string())
            }

            _ => {
                PsychoType::Unsure(vec![
                    PsychoLine::Node(PsychoType::Keyword(value.to_string())),
                    PsychoLine::Node(PsychoType::Expression(value.to_string())),
                ])
            }
        };

        out.push(PsychoLine::Node(psycho_type));
    }

    if rest.starts_with(';') {
        return Some(std::mem::take(out));
    }

    if rest.chars().next().is_some_and(|c| c.is_whitespace()) {
        out.push(PsychoLine::Space);
    }

    psycho_line_parse(out, rest)
}

fn prep(src: &str) -> String {
    let mut out = String::new();
    let mut string = false;
    let mut space = false;

    for c in src.chars() {
        if c == '"' {
            if !string && out.chars().last().is_some_and(|c| c.is_alphanumeric()) {
                out.push(' ');
            }

            string = !string;
            out.push(c);
            continue;
        }

        if string {
            if c == '\n' {
                out.push_str("\\n");
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
        out.push(c);

        if c == '>' || c == ';' {
            out.push('\n');
        }
    }

    out
}

fn get_lines_between_tags<'a>(lines: &'a [&'a str], start: usize, end: usize) -> &'a [&'a str] {
    &lines[start + 1..end]
}