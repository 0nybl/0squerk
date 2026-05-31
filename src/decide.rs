use crate::auth;
use crate::config::{self, Config};
use crate::event::Event;
use crate::trigger;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Decision {
    pub proceed: bool,
    pub reason: String,
    pub command: String,
}

pub fn decide(event: &Event, cfg: &Config) -> Decision {
    let command = config::command(cfg);
    if event.issue.pull_request.is_none() {
        return Decision { proceed: false, reason: "not a pull request".into(), command };
    }
    if !trigger::is_merge(&event.comment.body) {
        return Decision { proceed: false, reason: "not a /merge command".into(), command };
    }
    if !auth::is_write(&event.comment.author_association) {
        return Decision { proceed: false, reason: "author lacks write access".into(), command };
    }
    Decision { proceed: true, reason: String::new(), command }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::event::parse as parse_event;

    fn event(body: &str, assoc: &str, is_pr: bool) -> crate::event::Event {
        let pr = if is_pr { "\"pull_request\": {\"url\": \"x\"}," } else { "" };
        let json = format!(
            "{{ \"issue\": {{ \"number\": 1, {pr} \"x\": 0 }},
                \"comment\": {{ \"body\": \"{body}\", \"author_association\": \"{assoc}\" }} }}"
        );
        parse_event(&json).unwrap()
    }

    #[test]
    fn proceeds_for_authorized_merge_on_pr() {
        let d = decide(&event("/merge", "OWNER", true), &Config::default());
        assert!(d.proceed);
        assert_eq!(d.command, "cargo test --all");
    }

    #[test]
    fn rejects_non_pr() {
        let d = decide(&event("/merge", "OWNER", false), &Config::default());
        assert!(!d.proceed);
        assert_eq!(d.reason, "not a pull request");
    }

    #[test]
    fn rejects_non_command() {
        let d = decide(&event("hi", "OWNER", true), &Config::default());
        assert!(!d.proceed);
        assert_eq!(d.reason, "not a /merge command");
    }

    #[test]
    fn rejects_unauthorized() {
        let d = decide(&event("/merge", "NONE", true), &Config::default());
        assert!(!d.proceed);
        assert_eq!(d.reason, "author lacks write access");
    }

    #[test]
    fn uses_config_command() {
        let cfg = crate::config::parse("command = \"make ci\"\n");
        let d = decide(&event("/merge", "MEMBER", true), &cfg);
        assert!(d.proceed);
        assert_eq!(d.command, "make ci");
    }
}
