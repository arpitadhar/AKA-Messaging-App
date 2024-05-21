use axum::{routing::post, Json, Router, http::StatusCode};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use crate::database::models::Messages;

pub fn routes() -> Router {
	Router::new().route("/list-messages", post(list_messages))
}

pub async fn list_messages(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let connection = &mut crate::database::establish_connection::establish_connection();

    let message_response = show_messages(connection, &payload);
    
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(response_json))
}

pub fn show_messages(conn: &mut PgConnection, conversation_input: &str) -> Vec<Messages> {
    use crate::database::schema::messages::dsl::*;

    let results = messages
        .filter(conversation_id.eq(conversation_input))
        .load::<Messages>(conn)
        .expect("Error loading messages");

    return results;
}
