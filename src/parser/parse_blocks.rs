use std::{fmt::format, format, print};

pub enum Block {
    Execute(String)
}

pub fn parse_blocks(lines: String) -> Result<Vec<Block>, String> {
    let out: Vec<Block> = Vec::new();

    let mut lines = lines.lines().peekable();

    let mut open_closure: Option<String> = None;
    let contents: Option<String> = None;

    while let Some(line) = lines.next() {
        if line.is_empty() { continue; }

        if let Some(ac) = open_closure {
            if get_end_tag(line, ac.as_str()) {
                open_closure = None;

                continue;
            } else {
                open_closure = Some(ac.to_string());
            }
        } else {
            let start_tag = get_tag(line).unwrap_or_else(
                || return "NONE"
            );

            if get_end_tag(line, start_tag) {
                open_closure = None;

                continue;
            } else {
                open_closure = Some(start_tag.to_string());
            }
        }
    }

    Ok(out)
}

fn get_tag(text: &str) -> Option<&str> {
    let start = text.find('<')? + 1;
    let end = text[start..].find('>')? + start;

    Some(&text[start..end])
}

fn get_end_tag(text: &str, tag: &str) -> bool {
    if text.find(format!("</{tag}>").as_str()).is_some() {
        return true;
    }

    false
}