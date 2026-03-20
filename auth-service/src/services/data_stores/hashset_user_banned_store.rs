use crate::domain::{BannedTokenStore, BannedTokenStoreError};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    store: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.store.insert(token);
        Ok(())
    }

    async fn is_banned_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        if !self.store.contains(token) {
            return Err(BannedTokenStoreError::TokenDoesNotExists);
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_banned_token_should_work() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "myfaketoken".to_owned();
        let result = store.add_banned_token(token.clone()).await;
        assert!(result.is_ok());

        let result = store.add_banned_token(token).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn is_banned_token_should_work() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "myfaketoken".to_owned();
        let result = store.add_banned_token(token.clone()).await;
        assert!(result.is_ok());

        let result = store.is_banned_token(&token).await;
        assert!(result.is_ok());

        let not_banned = "Iamgoodtoken".to_owned();
        let result = store.is_banned_token(&not_banned).await;
        assert!(result.is_err());
    }
}
