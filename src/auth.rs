use std::fmt;

/// A Trello API key (32 hex characters).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiKey(pub(crate) String);

/// A Trello API token (typically 64 hex characters).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiToken(pub(crate) String);

/// Error returned when an API key or token fails validation.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid API key: {0}")]
    InvalidKey(String),
    #[error("invalid API token: {0}")]
    InvalidToken(String),
}

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Result<Self, AuthError> {
        let key = key.into();
        Ok(Self(key))
    }
}

impl ApiToken {
    pub fn new(token: impl Into<String>) -> Result<Self, AuthError> {
        let token = token.into();
        Ok(Self(token))
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for ApiToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ApiKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ApiToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_api_key() {
        let key = ApiKey::new("0471642aefef5fa1fa76530ce1ba4c85").unwrap();
        assert_eq!(key.as_ref(), "0471642aefef5fa1fa76530ce1ba4c85");
    }

    #[test]
    fn invalid_api_key_short() {
        assert!(ApiKey::new("short").is_err());
    }

    #[test]
    fn invalid_api_key_non_hex() {
        assert!(ApiKey::new("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxz").is_err());
    }

    #[test]
    fn valid_api_token() {
        let token =
            ApiToken::new("9eb76d9a9d02b8dd40c2f3e5df18556c831d4d1fadbe2c45f8310e6c93b5c548")
                .unwrap();
        assert_eq!(token.as_ref().len(), 64);
    }

    #[test]
    fn invalid_api_token_short() {
        assert!(ApiToken::new("short").is_err());
    }
}
