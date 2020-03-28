use std::string::ToString;

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

