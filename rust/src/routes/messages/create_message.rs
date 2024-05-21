use axum::{routing::post, Json, Router};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use super::CreateMessage;
use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/create-message", post(create_message))
}

pub async fn create_message(
    payload: Json<CreateMessage>
) -> Result<Json<Value>> {
    let connection = &mut crate::database::establish_connection::establish_connection();
    insert_message(connection, &payload.user_id, &payload.message, &payload.conversation_id);

    if payload.user_id == "" {
        return Err(Error::CreateMessageFail);
    }

    let body = Json(json!({
        "result" : {
            "success" : true
        }
    }));

    Ok(body)
}

fn insert_message(conn: &mut PgConnection, sender: &str, message_input: &str, conversation_id_input: &str) {
    use crate::database::schema::messages::dsl::*;
    insert_into(messages)
        .values((user_id.eq(sender), message.eq(message_input), conversation_id.eq(conversation_id_input)))
        .execute(conn)
        .expect("Error inserting into messages table");
}