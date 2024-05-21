use axum::{routing::post, Json, Router, http::StatusCode};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use crate::database::models::Users;

use super::SearchUser;

pub fn routes() -> Router {
	Router::new().route("/search-user", post(list_users))
}

pub async fn list_users(Json(payload): Json<SearchUser>
) -> (StatusCode, Json<Vec<String>>){
    let connection = &mut crate::database::establish_connection::establish_connection();

    let message_response = show_users(connection, &payload.username_input);
    
    //let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(message_response))
}

fn show_users(conn: &mut PgConnection, username_input: &str) -> Vec<String>{
    use crate::database::schema::users::dsl::*;

    let results = users
        .filter(username.eq(username_input))
        .select(email)
        .load::<String>(conn)
        .expect("Error loading users");

    results
}