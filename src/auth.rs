use std::fmt;

/// A Trello API key (32 hex characters).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiKey(pub(crate) String);

/// A Trello API token (typically 64 hex characters).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiToken(pub(crate) String);

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Self {
        let key = key.into();
        Self(key)
    }
}

impl ApiToken {
    pub fn new(token: impl Into<String>) -> Self {
        let token = token.into();
        Self(token)
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
        let key = ApiKey::new("0471642aefef5fa1fa76530ce1ba4c85");
        assert_eq!(key.as_ref(), "0471642aefef5fa1fa76530ce1ba4c85");
    }

    #[test]
    fn valid_api_token() {
        let token =
            ApiToken::new("9eb76d9a9d02b8dd40c2f3e5df18556c831d4d1fadbe2c45f8310e6c93b5c548");
        assert_eq!(token.as_ref().len(), 64);
    }
}
