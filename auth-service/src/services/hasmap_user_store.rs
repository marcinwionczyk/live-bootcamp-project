use std::collections::HashMap;
use crate::domain::{User, UserStore, UserStoreError};



// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl Default for HashmapUserStore {
    fn default() -> Self {
        Self { users: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore{
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }        
        self.users.insert((&user.email).to_string(), user);
        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)

    }
    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        self.get_user(email).await.map(|user| if user.password == password {
            Ok(())
        } else {
           Err(UserStoreError::InvalidCredentials)
        })?
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut hasmap_user_store = HashmapUserStore::default();
        let user = User::new("some@email.com", "MassWord", false);
        hasmap_user_store.add_user(user.clone()).await.expect("Cannot add user");
        assert_eq!(hasmap_user_store.users.get(&user.email), Some(&user))
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut hasmap_user_store = HashmapUserStore::default();
        let valid_user = User::new("some@email.com", "HelloWorld", false);
        let invalid_user = User::new("other@email.com", "HellWorld", false);
        hasmap_user_store.add_user(valid_user.clone()).await.expect("Cannot add user");
        assert_eq!(hasmap_user_store.get_user(&valid_user.email).await, Ok(&valid_user));
        assert_eq!(hasmap_user_store.get_user(&invalid_user.email).await, Err(UserStoreError::UserNotFound))
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut hasmap_user_store = HashmapUserStore::default();
        let valid_user = User::new("some@email.com", "HelloWorld", false);
        hasmap_user_store.add_user(valid_user.clone()).await.expect("Cannot add user");
        assert_eq!(hasmap_user_store.validate_user(&valid_user.email, "HelloWorld").await, Ok(()));
        assert_eq!(hasmap_user_store.validate_user(&valid_user.email, "HellWorld").await, Err(UserStoreError::InvalidCredentials));
    }
}