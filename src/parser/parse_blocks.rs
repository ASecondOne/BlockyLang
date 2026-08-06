use crate::parser::parse_lines::{Expression, parse_lines};

#[derive(Debug)]
pub enum Block {
    Execute(Vec<Expression>),
    Define(Vec<Expression>),
    None,
}

enum BlockKind {
    Execute,
    Define,
}

impl BlockKind {
    fn from_tag(tag: &str) -> Result<Self, String> {
        match tag {
            "execute" => Ok(Self::Execute),
            "define" => Ok(Self::Define),
            other => Err(format!("Unknown block type: {other}")),
        }
    }

    fn into_block(self, contents: Vec<&str>) -> Block {
        let expressions = parse_lines(contents);

        match self {
            Self::Execute => Block::Execute(expressions),
            Self::Define => Block::Define(expressions),
        }
    }
}

pub fn parse_blocks(lines: String) -> Result<Vec<Block>, String> {
    let mut out: Vec<Block> = Vec::new();

    let mut open_closure: Option<String> = None;
    let mut block_kind: Option<BlockKind> = None;
    let mut contents = Vec::new();

    for line in lines.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(ac) = open_closure {
            if let Some(extra_content) = line.strip_suffix(&format!("</{ac}>")) {
                if !extra_content.is_empty() {
                    contents.push(extra_content);
                }

                let kind = block_kind.take().ok_or("Missing block type")?;
                out.push(kind.into_block(std::mem::take(&mut contents)));
                open_closure = None;

                continue;
            }

            if get_end_tag(line, ac.as_str()) {
                open_closure = None;

                let kind = block_kind.take().ok_or("Missing block type")?;
                out.push(kind.into_block(std::mem::take(&mut contents)));

                continue;
            } else {
                contents.push(line);
                open_closure = Some(ac);
            }
        } else {
            let start_tag =
                get_tag(line).ok_or_else(|| format!("Expected a block tag, found: {line}"))?;
            let kind = BlockKind::from_tag(start_tag)?;

            if let Some(extra_content) = line.strip_prefix(&format!("<{start_tag}>")) {
                if !extra_content.is_empty() {
                    if get_end_tag(extra_content, start_tag) {
                        let contents = get_between_tags(line, start_tag);
                        out.push(kind.into_block(vec![contents]));

                        continue;
                    } else {
                        contents.push(extra_content);
                        open_closure = Some(start_tag.to_string());
                        block_kind = Some(kind);
                        continue;
                    }
                }
            }

            if get_end_tag(line, start_tag) {
                let contents = get_between_tags(line, start_tag);
                out.push(kind.into_block(vec![contents]));

                continue;
            } else {
                open_closure = Some(start_tag.to_string());
                block_kind = Some(kind);
            }
        }
    }

    if let Some(tag) = open_closure {
        return Err(format!("Unclosed <{tag}> block"));
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

    return text
        .strip_prefix(&start_tag)
        .unwrap()
        .strip_suffix(&end_tag)
        .unwrap();
}