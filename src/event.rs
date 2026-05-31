use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub comment: Comment,
    pub issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub body: String,
    pub author_association: String,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub number: u64,
    pub pull_request: Option<serde_json::Value>,
}

pub fn parse(json: &str) -> Result<Event, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PR_COMMENT: &str = r#"{
      "issue": { "number": 7, "pull_request": { "url": "x" } },
      "comment": { "body": "/merge", "author_association": "OWNER" }
    }"#;

    const NON_PR: &str = r#"{
      "issue": { "number": 7 },
      "comment": { "body": "hello", "author_association": "NONE" }
    }"#;

    #[test]
    fn parses_pr_comment() {
        let e = parse(PR_COMMENT).unwrap();
        assert_eq!(e.issue.number, 7);
        assert!(e.issue.pull_request.is_some());
        assert_eq!(e.comment.body, "/merge");
        assert_eq!(e.comment.author_association, "OWNER");
    }

    #[test]
    fn detects_non_pr_when_key_absent() {
        let e = parse(NON_PR).unwrap();
        assert!(e.issue.pull_request.is_none());
    }
}
