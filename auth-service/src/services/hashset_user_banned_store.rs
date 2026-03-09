use std::collections::{HashSet};
use crate::domain::{BannedTokenStore, User, UserStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    store: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore  {
    
    async fn add_banned_token(&mut self, token: &str) -> Result<(), UserStoreError>{
        self.store.insert(token.to_owned());
        Ok(())
        
    }
    
    async fn is_banned_token(&self, token: &str) -> Result<(), UserStoreError>{
        if !self.store.contains(token) {
            // TODO questionable
            // so add its own enum
            return Err(UserStoreError::UserNotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[tokio::test]
    async fn add_banned_token_should_work(){
        let mut store = HashsetBannedTokenStore::default();
        let token = "myfaketoken".to_owned();
        let result = store.add_banned_token(&token).await;
        assert!(result.is_ok());

        let result = store.add_banned_token(&token).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn is_banned_token_should_work(){
        let mut store = HashsetBannedTokenStore::default();
        let token = "myfaketoken".to_owned();
        let result = store.add_banned_token(&token).await;
        assert!(result.is_ok());

        let result = store.is_banned_token(&token).await;
        assert!(result.is_ok());

        let not_banned= "Iamgoodtoken".to_owned();
        let result = store.is_banned_token(&not_banned).await;
        assert!(result.is_err());
    }
}