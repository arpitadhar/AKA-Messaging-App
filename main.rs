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


//Tokio
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

    let message = Message {
        username: payload.username,
        message: payload.message,
    };


    append_string_to_file("text.json", serde_json::to_string_pretty(&message).unwrap()).expect("Unable to read file");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(message))
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

    file.write(b"\n")?;
    file.flush()?;

    Ok(())
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
