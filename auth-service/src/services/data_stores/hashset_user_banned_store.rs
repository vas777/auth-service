use color_eyre::eyre::eyre;
use secrecy::{SecretString, ExposeSecret} ;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    store: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(&mut self, token: SecretString) -> Result<(), BannedTokenStoreError> {
        self.store.insert(token.expose_secret().to_owned());
        Ok(())
    }

    async fn is_banned_token(&self, token: &SecretString) -> Result<bool, BannedTokenStoreError> {
        if !self.store.contains(token.expose_secret()) {
            return Err(BannedTokenStoreError::UnexpectedError(eyre!(
                "failed to check if token exists"
            )));
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;

    #[tokio::test]
    async fn add_banned_token_should_work() {
        let mut store = HashsetBannedTokenStore::default();
        let token = SecretString::new("myfaketoken".to_owned().into_boxed_str());
        let result = store.add_banned_token(token.clone()).await;
        assert!(result.is_ok());

        let result = store.add_banned_token(token).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn is_banned_token_should_work() {
        let mut store = HashsetBannedTokenStore::default();
        let token = SecretString::new("myfaketoken".to_owned().into_boxed_str());
        let result = store.add_banned_token(token.clone()).await;
        assert!(result.is_ok());

        let result = store.is_banned_token(&token).await;
        assert!(result.is_ok());

        let not_banned = SecretString::new("Iamgoodtoken".to_owned().into_boxed_str()); 
        let result = store.is_banned_token(&not_banned).await;
        assert!(result.is_err());
    }
}
