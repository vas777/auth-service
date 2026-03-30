// TODO: why this pub is necessary?
// necessary so app_state can see it .
// why it is not able to without ? siblings ?
mod data_stores;
mod mock_email_client;
mod postmark_email_client;

pub use data_stores::*;
pub use mock_email_client::*;
pub use postmark_email_client::*;
