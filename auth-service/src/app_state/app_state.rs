use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::BannedTokenStore;
use crate::domain::UserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokens = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokens,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokens) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}
