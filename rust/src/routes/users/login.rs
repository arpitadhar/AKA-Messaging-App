use axum::{routing::post, Json, Router, http::StatusCode};
use diesel::prelude::*;

use crate::database::models::Users;

use super::Login;

pub fn routes() -> Router {
	Router::new().route("/login", post(login))
}

pub async fn login(
    Json(payload): Json<Login>
) -> (StatusCode, Json<String>){
    let connection = &mut crate::database::establish_connection::establish_connection();

    let user_exists = check_user_exists(connection, &payload.email, &payload.password);

    match user_exists {
        Ok(username) => {
            let message_response = format!("{}", username);   
            (StatusCode::OK, Json(message_response))
        }  
        Err(_) => {
            let message_response = "Incorrect email or password.";
            let response_json = serde_json::to_string(message_response).expect("Error serializing");
            (StatusCode::UNAUTHORIZED, Json(response_json))
        }
    }
}

fn check_user_exists(conn: &mut PgConnection, email_input: &str, password_input: &str) -> Result<String, Box<dyn std::error::Error>>{
    use crate::database::schema::users::dsl::*;

    match users.filter(email.eq(email_input).and(password.eq(password_input))).first::<Users>(conn)  {
        Ok(user) => Ok(user.username),
        Err(err) => Err(Box::new(err))
    }
}