use serde::Deserialize;

pub const DEFAULT_COMMAND: &str = "cargo test --all";

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub command: Option<String>,
}

pub fn parse(toml_str: &str) -> Config {
    toml::from_str(toml_str).unwrap_or_default()
}

pub fn command(cfg: &Config) -> String {
    cfg.command
        .clone()
        .unwrap_or_else(|| DEFAULT_COMMAND.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_command_override() {
        let cfg = parse("command = \"make ci\"\n");
        assert_eq!(command(&cfg), "make ci");
    }

    #[test]
    fn empty_config_uses_default() {
        let cfg = parse("");
        assert_eq!(command(&cfg), DEFAULT_COMMAND);
        assert_eq!(DEFAULT_COMMAND, "cargo test --all");
    }

    #[test]
    fn malformed_config_uses_default() {
        let cfg = parse("this is not = valid = toml ===");
        assert_eq!(command(&cfg), DEFAULT_COMMAND);
    }
}
