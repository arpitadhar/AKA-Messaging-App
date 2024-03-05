use std::fs;
use std::io::Write;
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use tokio::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer};
use http_server_test::*;

pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::insert_into;



#[tokio::main]

async fn main() {

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /create-message` goes to `create_message`
        .route("/create-message", post(create_message))
        .route("/messages", get(list_messages))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Message parsing and delivery system!"
}

async fn create_message(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateMessage` type
    Json(payload): Json<CreateMessage>,
) -> (StatusCode, Json<Message>) {

    let connection = &mut establish_connection();

    let info = insert_message(connection, &payload.username, &payload.message);

    println!("{info:?}");

    let new_message = Message {
        username: payload.username,
        message: payload.message,
    };


    append_string_to_file("text.json", serde_json::to_string_pretty(&new_message).unwrap()).expect("Unable to read file");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(new_message))
}

// Handler for /messages
async fn list_messages() -> Json<String> {

    let data = fs::read_to_string("text.json").expect("Unable to read file");
    println!("{}", data);

    //let users = vec![
    //    Message {
    //        username: "Real_American_Patriot".to_string(),
    //        message: "Elijah".to_string(),
    //    },
    //    Message {
    //        username: "Name".to_string(),
    //        message: "John".to_string(),
    //    },
    //]; 
    Json(data)
}


fn append_string_to_file(path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;

    // You need to take care of the conversion yourself
    // and can either try to write all data at once
    file.write_all(&data.as_bytes())?;

    file.write(b"\n ,")?;
    file.flush()?;

    Ok(())
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str) -> QueryResult<usize> {
    use schema::messages::dsl::*;

    insert_into(messages)
        .values((user_id.eq(username), message.eq(message_text)))
        .execute(conn)
}



// the input to our `create_message` handler
#[derive(Deserialize)]
struct CreateMessage {
    username: String,
    message: String
}

// the output to our `create_message` handler
#[derive(Serialize)]
struct Message {
    username: String,
    message: String,
}

