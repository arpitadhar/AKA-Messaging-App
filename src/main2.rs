
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
//use http_server_test::*;

pub mod schema;
pub mod models;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::insert_into;
//use models::{User, NewUser}; 



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
        .route("/create-users", post(create_user))
        .route("/get-users", get(list_users))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Context{
    header: String, 
}
// #[get["/signup"]]
// fn signup_page() -> Template{
//     let context = Context{
//         header: "Sign up:".to_string(),
//     }; 
//     Template::render("signup", &context)
// }

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Message parsing and delivery system!"
}

async fn create_message(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateMessage` type
    Json(payload): Json<CreateMessage>, //takes json payload as argument 
) -> (StatusCode, Json<Message>) {

    let connection = &mut establish_connection(); //create a connection 

    let info = insert_message(connection, &payload.username, &payload.message); //actual insertion 

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
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    use schema::users;

    let connection = &mut establish_connection();
    let info = insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password); 
    println!("{info:?}"); 
    let new_user = User{
        username: payload.username, 
        first_name: payload.first_name, 
        last_name: payload.last_name, 
        email: payload.email, 
        password: payload.password, 
    }; 
    
    append_string_to_file("users.json", serde_json::to_string_pretty(&new_user).unwrap()).expect("Unable to read file");

    (StatusCode::CREATED, Json(new_user))

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

async fn list_users() -> Json<String> {
    let data = fs::read_to_string("users.json").expect("Unable to read file"); 
    println!("{}", data); 
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

fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> QueryResult<usize>{
    use schema::users::dsl::*;

    insert_into(users)
        .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
        .execute(conn2)
}



// the input to our `create_message` handler
#[derive(Deserialize)]
struct CreateMessage {
    username: String,
    message: String
}


#[derive(Deserialize)]
struct CreateUser {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String
}

#[derive(Serialize)]
struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String

}
// the output to our `create_message` handler
#[derive(Serialize)]
struct Message {
    username: String,
    message: String,
}
