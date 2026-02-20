use std::{collections::hash_map::Entry, collections::HashMap};

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        match self.users.entry(user.email.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
        }
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        // match self.users.get(email) {
        //     Some(u) => Ok(u.clone()),
        //     None=> Err(UserStoreError::UserNotFound)
        // }
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    // Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let u = self.get_user(email)?;

        if u.password == password {
            return Ok(());
        }
        Err(UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("vas@email".to_owned(), "password".to_owned(), false);
        assert_eq!(store.add_user(user.clone()), Ok(()));

        assert_eq!(store.add_user(user), Err(UserStoreError::UserAlreadyExists));

        let user = User::new("not@email".to_owned(), "password".to_owned(), false);
        assert_eq!(store.add_user(user.clone()), Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = "vas@email".to_owned();
        let not_used_email = "no@email".to_owned();
        let user = User::new(email.clone(), "password".to_owned(), false);
        assert_eq!(store.add_user(user.clone()), Ok(()));

        assert_eq!(store.get_user(&email), Ok(user.clone()));

        assert_ne!(store.get_user(&not_used_email), Ok(user));

        assert_eq!(
            store.get_user(&not_used_email),
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = "vas@email".to_owned();
        let pass = "password".to_owned();
        let wrong_pass = "youshallnotpass".to_owned();
        let not_used_email = "no@email".to_owned();
        let user = User::new(email.clone(), pass.clone(), false);
        assert_eq!(store.add_user(user.clone()), Ok(()));

        assert_eq!(store.validate_user(&email.clone(), &pass.clone()), Ok(()));

        assert_eq!(
            store.validate_user(&not_used_email.clone(), &pass.clone()),
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store.validate_user(&email.clone(), &wrong_pass.clone()),
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
