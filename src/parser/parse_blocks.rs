use std::format;

#[derive(Debug)]
pub enum Block {
    Execute(String)
}

pub fn parse_blocks(lines: String) -> Result<Vec<Block>, String> {
    let mut out: Vec<Block> = Vec::new();

    let mut lines = lines.lines();

    let mut open_closure: Option<String> = None;
    let mut contents = String::new();

    while let Some(line) = lines.next() {
        if line.is_empty() { continue; }

        if let Some(ac) = open_closure {
            if get_end_tag(line, ac.as_str()) {
                open_closure = None;

                out.push(Block::Execute(std::mem::take(&mut contents)));

                continue;
            } else {
                contents.push_str(line);
                open_closure = Some(ac.to_string());
            }
        } else {
            let start_tag = get_tag(line).unwrap_or_else(
                || return "NONE"
            );

            if get_end_tag(line, start_tag) {
                open_closure = None;

                let contents = get_between_tags(line, start_tag);

                out.push(Block::Execute(contents.to_string()));

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