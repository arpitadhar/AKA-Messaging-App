use axum::{routing::post, Json, Router, http::StatusCode};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use crate::database::models::Conversations;

pub fn routes() -> Router {
	Router::new().route("/list-conversations", post(list_conversations))
}

pub async fn list_conversations(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let connection = &mut crate::database::establish_connection::establish_connection();

    let message_response = show_conversations(connection, &payload);
    
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(response_json))
}

fn show_conversations(conn: &mut PgConnection, username_input: &str) -> Vec<Conversations>{
    use crate::database::schema::conversations::dsl::*;

    let results = conversations
        .filter(user1_id.eq(username_input).or(user2_id.eq(username_input)))
        .load::<Conversations>(conn)
        .expect("Error loading conversations");

    return results;
}
