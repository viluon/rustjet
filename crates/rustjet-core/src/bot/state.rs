use serde::{Deserialize, Serialize};

/// Conversation states for the login wizard
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    AwaitingAccountCode,
    AwaitingPassword {
        account_code: String,
    },
}
