use axum::{routing::post, Json, Router};
use diesel::{insert_into, prelude::*};
use serde_json::{json, Value};

use super::CreateUser;
use crate::{Error,Result};

pub fn routes() -> Router {
	Router::new().route("/create-user", post(create_user))
}

pub async fn create_user(
    Json(payload): Json<CreateUser>
) -> Result<Json<Value>>{
    let connection = &mut crate::database::establish_connection::establish_connection();
    insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password);

    if payload.username == "" {
        return Err(Error::CreateUserFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

fn insert_user(conn: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) {
    use crate::database::schema::users::dsl::*;
    insert_into(users)
        .values((email.eq(email_input), username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input)))
        .execute(conn)
        .expect("Error inserting into users table");
}