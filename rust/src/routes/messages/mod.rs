use::serde::{Serialize, Deserialize};

pub mod create_message;
pub mod list_messages;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    user_id: String,
    message: String,
    conversation_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessage {
    user_id: String,
    message: String,
    conversation_id: String,
}