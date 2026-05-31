const ALLOWED: [&str; 3] = ["OWNER", "MEMBER", "COLLABORATOR"];

pub fn is_write(association: &str) -> bool {
    ALLOWED.contains(&association)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_level_associations_allowed() {
        assert!(is_write("OWNER"));
        assert!(is_write("MEMBER"));
        assert!(is_write("COLLABORATOR"));
    }

    #[test]
    fn other_associations_denied() {
        assert!(!is_write("CONTRIBUTOR"));
        assert!(!is_write("NONE"));
        assert!(!is_write("FIRST_TIME_CONTRIBUTOR"));
        assert!(!is_write(""));
    }
}
