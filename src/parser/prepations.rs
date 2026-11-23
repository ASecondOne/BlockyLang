pub fn prep_raw_source(source: String) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = source.chars().peekable();

    let mut quote = None;
    let mut escaped = false;
    let mut in_tag = false;

    while let Some(c) = chars.next() {
        if let Some(q) = quote {
            current.push(c);

            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == q {
                quote = None;
            }

            continue;
        }

        if c == '"' || c == '\'' {
            quote = Some(c);
            current.push(c);
            continue;
        }

        match c {
            '<' => {
                flush(&mut current, &mut out);
                in_tag = true;
                current.push(c);
            }

            '>' if in_tag => {
                current.push(c);
                flush(&mut current, &mut out);
                in_tag = false;
            }

            '{' | '}' if !in_tag => {
                flush(&mut current, &mut out);
                out.push(c.to_string());
            }

            ';' if !in_tag => {
                current.push(c);
                flush(&mut current, &mut out);
            }

            '\n' | '\r' if !in_tag => {
                if !current.trim().is_empty() {
                    current.push(' ');
                }
            }

            _ => current.push(c),
        }
    }

    flush(&mut current, &mut out);
    out
}

fn flush(current: &mut String, out: &mut Vec<String>) {
    let line = current.trim();

    if !line.is_empty() {
        out.push(line.to_string());
    }

    current.clear();
}