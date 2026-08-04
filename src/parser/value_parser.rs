#[derive(Debug, Clone)]
pub enum Value {
    String(String),
}

pub fn parse_value(s: String) -> Option<Value> {
    if s.starts_with('"') && s.ends_with('"') {
        let out = s.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

        return Some(Value::String(out.to_string()));
    }

    None
}