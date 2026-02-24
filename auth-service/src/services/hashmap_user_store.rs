use std::{collections::hash_map::Entry, collections::HashMap};

use crate::domain::{Email, Password, User, UserStore, UserStoreError};
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        // let e  = Email::parse().map_err(||UserStoreError::UnexpectedError);
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
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
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
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let u = self.get_user(email).await?;

        if u.password.eq(password) {
            return Ok(());
        }
        Err(UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Password;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        let user = User::new(
            Email::parse("vas@email".to_owned()).unwrap(),
            Password::parse("password".to_owned()).unwrap(),
            false,
        );
        assert_eq!(store.add_user(user.clone()).await, Ok(()));

        assert_eq!(
            store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );

        let user = User::new(
            Email::parse("not@email".to_owned()).unwrap(),
            Password::parse("password".to_owned()).unwrap(),
            false,
        );
        assert_eq!(store.add_user(user.clone()).await, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("vas@email".to_owned()).unwrap();
        let not_used_email = Email::parse("no@email".to_owned()).unwrap();
        let user = User::new(email.clone(), Password::parse("password".to_owned()).unwrap(), false);
        assert_eq!(store.add_user(user.clone()).await, Ok(()));

        assert_eq!(store.get_user(&email).await, Ok(user.clone()));

        assert_ne!(store.get_user(&not_used_email).await, Ok(user));

        assert_eq!(
            store.get_user(&not_used_email).await,
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("vas@email".to_owned()).unwrap();
        let pass = Password::parse("password".to_owned()).unwrap();
        let wrong_pass = Password::parse("youshallnotpass".to_owned()).unwrap();
        let not_used_email = Email::parse("no@email".to_owned()).unwrap();
        let user = User::new(email.clone(), pass.clone(), false);
        assert_eq!(store.add_user(user.clone()).await, Ok(()));

        assert_eq!(
            store.validate_user(&email.clone(), &pass.clone()).await,
            Ok(())
        );

        assert_eq!(
            store
                .validate_user(&not_used_email.clone(), &pass.clone())
                .await,
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store
                .validate_user(&email.clone(), &wrong_pass.clone())
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
