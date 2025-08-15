use crate::domain::UserStoreError;
#[derive(Clone, Debug, PartialEq)]
pub struct Password(pub String);

impl AsRef<str> for &Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Password {
    pub(crate) fn parse(&self) -> Result<String, UserStoreError> {
        match &self.0.len() {
            8.. => Ok(self.0.clone()),
            _ => Err(UserStoreError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_parse() {
        let password = Password("".to_string());
        assert_eq!(password.parse(), Err(UserStoreError::InvalidCredentials));
    }
}
