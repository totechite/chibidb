\use std::string::ToString;

pub fn is_match_command<S: ToString>(s: S) -> bool {
    match s.to_string().as_str() {
        "SELECT" => true,
        "FROM" => true,
        "WHERE" => true,
        "AND" => true,
        "UPDATE" => true,
        "DELETE" => true,
        "INSERT" => true,
        _ => false,
    }
}

pub fn noise_scanner(c: &char) -> bool {
    (&' ' == c) || (&'\n' == c) || (&',' == c)
}