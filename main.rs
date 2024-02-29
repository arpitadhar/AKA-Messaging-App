use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer};

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
    // insert your application logic here
    let message = Message {
        username: payload.username,
        message: payload.message,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(message))
}

// Handler for /messages
async fn list_messages() -> Json<Vec<Message>> {
    let users = vec![
        Message {
            username: "Real_American_Patriot".to_string(),
            message: "Elijah".to_string(),
        },
        Message {
            username: "Name".to_string(),
            message: "John".to_string(),
        },
    ];
    Json(users)
}

// the input to our `create_message` handler
#[derive(Deserialize)]
struct CreateMessage {
    username: String,
    message: String
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct Message {
    username: String,
    message: String,
}