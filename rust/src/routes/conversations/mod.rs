use::serde::{Serialize, Deserialize};

pub mod create_conversation;
pub mod list_conversations;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConversation {
    user_sender: String,
    user_receiver: String,
}