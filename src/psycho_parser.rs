struct PsychoBlock {
    name: String,
    contents: Vec<PsychoLine>
}

enum PsychoLine {
    Unsure(Vec<PsychoLine>),

    Keyword(String),
    Expression(String),
}

pub fn attempt_psycho_parse(contents: Vec<String>) {
    for c in contents {
        psycho_block_parse(prep(&c));
    }

}

fn prep(src: &str) -> String {
    let mut out = String::new();
    let mut string = false;
    let mut space = false;

    for c in src.chars() {
        if c == '"' {
            string = !string;
            out.push(c);
            continue;
        }

        if string {
            out.push(c);
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

fn psycho_block_parse(contents: String) {

}