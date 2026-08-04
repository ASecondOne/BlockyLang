use std::{format, vec};

use crate::parser::parse_lines::{Expression, parse_lines};

#[derive(Debug)]
pub enum Block {
    Execute(Vec<Expression>)
}

pub fn parse_blocks(lines: String) -> Result<Vec<Block>, String> {
    let mut out: Vec<Block> = Vec::new();

    let mut lines = lines.lines();

    let mut open_closure: Option<String> = None;
    let mut contents = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() { continue; }

        if let Some(ac) = open_closure {
            if let Some(extra_content) = line.strip_suffix(&format!("</{ac}>")) {
                if !extra_content.is_empty() {
                    open_closure = None;

                    contents.push(extra_content);

                    out.push(Block::Execute(parse_lines(std::mem::take(&mut contents))));

                    continue;
                }
            }

            if get_end_tag(line, ac.as_str()) {
                open_closure = None;

                out.push(Block::Execute(parse_lines(std::mem::take(&mut contents))));

                continue;
            } else {
                contents.push(line);
                open_closure = Some(ac.to_string());
            }
        } else {
            let start_tag = get_tag(line).unwrap_or_else(
                || return "NONE"
            );

            if let Some(extra_content) = line.strip_prefix(&format!("<{start_tag}>")) {
                if !extra_content.is_empty() {
                    if get_end_tag(extra_content, start_tag) {
                        open_closure = None;

                        let contents = get_between_tags(line, start_tag);

                        out.push(Block::Execute(parse_lines(vec![contents])));

                        continue;
                    } else {
                        contents.push(extra_content);
                        open_closure = Some(start_tag.to_string());
                        continue;
                    }
                }
            }

            if get_end_tag(line, start_tag) {
                open_closure = None;

                let contents = get_between_tags(line, start_tag);

                out.push(Block::Execute(parse_lines(vec![contents])));

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

fn get_between_tags<'a>(text: &'a str, st: &str) -> &'a str {
    let end_tag = format!("</{st}>");
    let start_tag = format!("<{st}>");

    return text.strip_prefix(&start_tag).unwrap().strip_suffix(&end_tag).unwrap();
}