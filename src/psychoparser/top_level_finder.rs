pub fn find_top_level_space(contents: &str) -> Option<usize> {
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

pub fn find_top_level_redirect(contents: &str) -> Option<usize> {
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

            '=' if depth == 0 => {
                return Some(i);
            }

            _ => {}
        }
    }

    None
}

pub fn find_top_level_dot(contents: &str) -> Option<usize> {
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