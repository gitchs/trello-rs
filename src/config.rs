use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_token: String,
    #[serde(default)]
    pub default_board_id: Option<String>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let expanded = expand_env_vars(&content);
        let config: Config = serde_yaml::from_str(&expanded)?;
        Ok(config)
    }
}

pub fn default_config_path() -> String {
    let config_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    config_dir
        .join(".config")
        .join("trello-rs")
        .join("config.yaml")
        .to_string_lossy()
        .to_string()
}

fn expand_env_vars(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            let mut var_name = String::new();
            if chars.peek() == Some(&'{') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == '}' {
                        chars.next();
                        break;
                    }
                    var_name.push(c);
                    chars.next();
                }
            } else {
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        var_name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            if let Ok(val) = std::env::var(&var_name) {
                result.push_str(&val);
            } else {
                result.push('$');
                result.push_str(&var_name);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_env_vars() {
        std::env::set_var("TEST_VAR", "hello");
        assert_eq!(expand_env_vars("$TEST_VAR"), "hello");
        assert_eq!(expand_env_vars("${TEST_VAR}"), "hello");
        assert_eq!(expand_env_vars("prefix_$TEST_VAR"), "prefix_hello");
        assert_eq!(expand_env_vars("$NONEXISTENT"), "$NONEXISTENT");
        std::env::remove_var("TEST_VAR");
    }
}
