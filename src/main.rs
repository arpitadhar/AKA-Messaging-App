//use std::fs;
//use std::io::Write;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer};

pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::insert_into;


#[tokio::main]

async fn main() {

    //initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        //GET / goes to root
        .route("/", get(root))
        //POST /create-message` goes to create_message()
        .route("/create-message", post(create_message))
        //GET /message` goes to list_message()
        .route("/messages", get(list_messages))
        //allows frontend to communicate with backend
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    //run app istening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//root endpoint, returns string
async fn root() -> &'static str {
    "Message parsing and delivery system!"
}

async fn create_message(
    //Parse the request body into a <CreateMessage> type
    Json(payload): Json<CreateMessage>,
) -> (StatusCode, Json<Message>) {

    //establish database connection and insert values into messages table
    let connection = &mut establish_connection();

    let info = insert_message(connection, &payload.username, &payload.message);

    let new_message = Message {
        username: payload.username,
        message: payload.message,
    };


    //append_string_to_file("text.json", serde_json::to_string_pretty(&new_message).unwrap()).expect("Unable to read file");

    //status code 201 Created
    (StatusCode::CREATED, Json(new_message))
}

async fn list_messages()  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into messages table    
    let connection = &mut establish_connection();

    let message_response = show_messages(connection);

    println!("{:?}",message_response);
    
    //serialize query response to json string
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    //status code 200 Ok
    (StatusCode::OK, Json(response_json))
}

//Below are the remains of an attempt to store data within a JSON file
//we experienced minimal success with this

//fn append_string_to_file(path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
//    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
//
//    // You need to take care of the conversion yourself
//    // and can either try to write all data at once
//    file.write_all(&data.as_bytes())?;
//
//    file.write(b"\n ,")?;
//    file.flush()?;
//
//    Ok(())
//}

//ABSOLUTELY VERY BAD NOT GOOD PRACTICE
//this is should be a module from lib.rs
//
//connect to postgres using a url made up of 'postgres://[username]:[password]@localhost/[db_name]'
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//insert into messages table
pub fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str) {
    use schema::messages::dsl::*;

    insert_into(messages)
        .values((user_id.eq(username), message.eq(message_text)))
        .execute(conn)
        .expect("Error inserting messages");
}

//select * from messages
pub fn show_messages(conn: &mut PgConnection) -> Vec<Messages> {
    use schema::messages::dsl::*;

    let results = messages
        .load::<Messages>(conn)
        .expect("Error loading messages");

    return results;
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


//ABSOLUTELY VERY BAD NOT GOOD PRACTICE
//this is a last resort, again I should use 'mod models'
//but module not found?
//need to figure out how modules work
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Messages {
    pub id: i32,
    pub user_id: String,
    pub message: String,
}