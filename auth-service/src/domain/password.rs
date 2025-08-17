use std::fmt::Display;
use crate::domain::AuthAPIError;
#[derive(Clone, Debug, PartialEq)]
pub struct Password(pub String);

impl AsRef<str> for &Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Password {
    pub(crate) fn parse(&self) -> Result<String, AuthAPIError> {
        match &self.0.len() {
            8.. => Ok(self.0.clone()),
            _ => Err(AuthAPIError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_parse() {
        let password = Password("".to_string());
        assert_eq!(password.parse(), Err(AuthAPIError::InvalidCredentials));
    }
}
