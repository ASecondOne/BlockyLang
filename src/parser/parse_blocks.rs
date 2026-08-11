use crate::parser::{
    parse_lines::{Expression, parse_lines},
    variable_parser::VariableMap,
};

#[derive(Debug)]
pub enum Block {
    Execute(Vec<Expression>),
    Define(Vec<Expression>),
    None,
}

#[derive(PartialEq, Eq)]
pub enum BlockKind {
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

    fn into_block(self, contents: Vec<&str>, vars: &mut VariableMap) -> Block {
        let expressions = parse_lines(contents, vars);

        match self {
            Self::Execute => Block::Execute(expressions),
            Self::Define => Block::Define(expressions),
        }
    }
}

pub fn parse_blocks(
    lines: &String,
    vars: &mut VariableMap,
    filter_kind: BlockKind,
) -> Result<Vec<Block>, String> {
    let mut out: Vec<Block> = Vec::new();

    let mut open_closure: Option<String> = None;
    let mut skipped_closure: Option<String> = None;
    let mut block_kind: Option<BlockKind> = None;
    let mut contents = Vec::new();

    for line in lines.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(tag) = skipped_closure.as_deref() {
            if get_end_tag(line, tag) {
                skipped_closure = None;
            }

            continue;
        }

        if let Some(ac) = open_closure {
            if let Some(extra_content) = line.strip_suffix(&format!("</{ac}>")) {
                if !extra_content.is_empty() {
                    contents.push(extra_content);
                }

                let kind = block_kind.take().ok_or("Missing block type")?;
                out.push(kind.into_block(std::mem::take(&mut contents), vars));
                open_closure = None;

                continue;
            }

            if get_end_tag(line, ac.as_str()) {
                open_closure = None;

                let kind = block_kind.take().ok_or("Missing block type")?;
                out.push(kind.into_block(std::mem::take(&mut contents), vars));

                continue;
            } else {
                contents.push(line);
                open_closure = Some(ac);
            }
        } else {
            let start_tag =
                get_tag(line).ok_or_else(|| format!("Expected a block tag, found: {line}"))?;
            let kind = BlockKind::from_tag(start_tag)?;

            if kind != filter_kind {
                if !get_end_tag(line, start_tag) {
                    skipped_closure = Some(start_tag.to_string());
                }

                continue;
            }

            if let Some(extra_content) = line.strip_prefix(&format!("<{start_tag}>")) {
                if !extra_content.is_empty() {
                    if get_end_tag(extra_content, start_tag) {
                        let contents = get_between_tags(line, start_tag);
                        out.push(kind.into_block(vec![contents], vars));

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
                out.push(kind.into_block(vec![contents], vars));

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

    if let Some(tag) = skipped_closure {
        return Err(format!("Unclosed <{tag}> block"));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{Block, BlockKind, parse_blocks};
    use crate::parser::{Expression, variable_parser::VariableMap};

    const SOURCE: &str = r#"
<define>
    let a = "Hello";
</define>

<execute>
    println "Hello";
</execute>
"#;

    #[test]
    fn define_filter_skips_the_whole_execute_block() {
        let blocks = parse_blocks(
            &SOURCE.to_string(),
            &mut VariableMap::new(),
            BlockKind::Define,
        )
        .unwrap();

        assert!(matches!(
            blocks.as_slice(),
            [Block::Define(expressions)]
                if matches!(expressions.as_slice(), [Expression::VariableDefinition(_)])
        ));
    }

    #[test]
    fn execute_filter_skips_the_whole_define_block() {
        let blocks = parse_blocks(
            &SOURCE.to_string(),
            &mut VariableMap::new(),
            BlockKind::Execute,
        )
        .unwrap();

        assert!(matches!(
            blocks.as_slice(),
            [Block::Execute(expressions)]
                if matches!(expressions.as_slice(), [Expression::ExecutionExpression(_)])
        ));
    }
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
