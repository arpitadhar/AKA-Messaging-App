use axum::{routing::post, Json, Router, http::StatusCode};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use super::CreateConversation;
use crate::Result;
use crate::database::models::Users;

pub fn routes() -> Router {
	Router::new().route("/create-conversation", post(create_conversation))
}

pub async fn create_conversation(
    Json(payload): Json<CreateConversation>
) -> (StatusCode, Json<String>) {
    let connection = &mut crate::database::establish_connection::establish_connection();

    //let user_exists = search_user(connection, &payload.user2_id);

    let sender = &payload.user_sender.to_owned();
    let receiver = &payload.user_receiver.to_owned();

    //let generatedID: String = sender.clone() + reciever;
    let mut sorted_emails = vec![sender, receiver];
    sorted_emails.sort();

    let generated_id:String = sorted_emails.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("");

    insert_conversation(connection, &generated_id, sender, receiver); 

    let message_response = "You've started a conversation with: ";

    let response_json = serde_json::to_string(message_response).expect("Error serializing");
    (StatusCode::CREATED, Json(response_json))
}

fn insert_conversation(conn: &mut PgConnection, id_input: &str, user1_input: &str, user2_input:&str) {
    use crate::database::schema::conversations::dsl::*;

    insert_into(conversations)
        .values((id.eq(id_input), user1_id.eq(user1_input), user2_id.eq(user2_input)))
        .execute(conn)
        .expect("Error inserting into users table");
}
