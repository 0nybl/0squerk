pub const COMMAND: &str = "/merge";

pub fn is_merge(body: &str) -> bool {
    body.trim() == COMMAND
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_merge_matches() {
        assert!(is_merge("/merge"));
        assert!(is_merge("  /merge  "));
    }

    #[test]
    fn other_bodies_do_not_match() {
        assert!(!is_merge("/merge please"));
        assert!(!is_merge("merge"));
        assert!(!is_merge(""));
        assert!(!is_merge("please /merge"));
    }
}
