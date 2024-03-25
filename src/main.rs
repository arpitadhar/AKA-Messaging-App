use std::fs;
use std::io::Write;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use schema::messages::{conversation_id, message};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer};


use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use tracing_subscriber::{filter::{self, combinator::And}, fmt::writer::MakeWriterExt};
use std::env;
use diesel::insert_into;

pub mod schema;
pub mod models;
use crate::{models::{Conversations, Messages}, schema::conversations};
use crate::models::Users; 

#[tokio::main]

async fn main() {

    //initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        //GET / goes to root
        .route("/", get(root))
        //POST /create-message goes to create_message()
        .route("/create-message", post(create_message))
        //POST /messages goes to list_messages()
        .route("/messages", post(list_messages))
        //POST /create-user goes to create_user()
        .route("/create-users", post(create_user))
        //GET /users goes to list_users()
        .route("/users", get(list_users))
        //POST /login goes to login()
        .route("/login", post(login))
        //POST /create-conversation goes to create_conversation()
        .route("/create-conversation", post(create_conversation))
        //POST /conversations goes to list_conversations()
        .route("/conversations", post(list_conversations))
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

    insert_message(connection, &payload.username, &payload.message, &payload.conversation_id);
    
    let new_message = Message {
        username: payload.username,
        message: payload.message,
        conversation_id: payload.conversation_id,
    };

    append_string_to_file("text.json", serde_json::to_string_pretty(&new_message).unwrap()).expect("Unable to read file");

    //status code 201 Created
    (StatusCode::CREATED, Json(new_message))
}

async fn list_messages(Json(payload): Json<String>)  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into messages table    
    let connection = &mut establish_connection();

    let message_response = show_messages(connection,&payload);

    println!("{:?}",message_response);
    
    //serialize query response to json string
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    //status code 200 Ok
    (StatusCode::OK, Json(response_json))
}

async fn create_user(
    //Parse the request body into a <CreateMessage> type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    //establish database connection and insert values into messages table
    let connection = &mut establish_connection();

    insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password); 
    //println!("{&payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password:?}"); 
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

async fn list_users()  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into users table    
    let connection: &mut PgConnection = &mut establish_connection();

    let user_response = show_users(connection);

    println!("{:?}", user_response);
    
    //serialize query response to json string
    let response_json = serde_json::to_string(&user_response).expect("Error serializing");

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

//insert into messages table
pub fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str, conversation: &str) {
    use schema::messages::dsl::*;

    insert_into(messages)
        .values((user_id.eq(username), message.eq(message_text),conversation_id.eq(conversation)))
        .execute(conn)
        .expect("Error inserting messages");
}

//insert into users table 
pub fn insert_user(conn: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) {
    use schema::users::dsl::*;

    insert_into(users)
        .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
        .execute(conn)
        .expect("Error inserting Users");
}

//changed the return type for this previous function keeps causing errors
// fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> Result<usize, diesel::result::Error> {
//         use schema::users::dsl::*;
//         println!("hi");
//         let rows_inserted = insert_into(users)
//             .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
//             .execute(conn2)?;

//         assert_eq!(1 as usize , rows_inserted);

//         println!("bye");
//         Ok(rows_inserted)
// }

//select * from messages
pub fn show_messages(conn: &mut PgConnection, conversation_input: &str) -> Vec<Messages> {
    use schema::messages::dsl::*;

    let results = messages
        .filter(conversation_id.eq(conversation_input))
        .load::<Messages>(conn)
        .expect("Error loading messages");

    return results;
}

//select * from users 
pub fn show_users(conn: &mut PgConnection) -> Vec<Users> {
    use schema::users::dsl::*;

    let results = users
        .load::<Users>(conn)
        .expect("Error loading messages");

    return results;
}


//Route for user login
//@return: Status Code, JSON string
//Calls check_user_exists
async fn login(
    Json(payload): Json<Login>
) -> (StatusCode, Json<String>) {
    let connection: &mut PgConnection = &mut establish_connection();
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
    use schema::users::dsl::*;

    match users.filter(email.eq(email_input).and(password.eq(password_input))).first::<Users>(conn)  {
        Ok(user) => Ok(user.username),
        Err(err) => Err(Box::new(err))
    }
}

async fn create_conversation(
    Json(payload): Json<CreateConversation>
) -> (StatusCode, Json<String>) {

    let connection: &mut PgConnection = &mut establish_connection();
    let user_exists = search_user(connection, &payload.username_receiver);

    let sender = &payload.username_sender.to_owned();
    let reciever = &payload.username_receiver.to_owned();

    //let generatedID: String = sender.clone() + reciever;
    let mut sorted_usernames = vec![sender, reciever];
    sorted_usernames.sort();

    let generated_id:String = sorted_usernames.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("");
    
    if user_exists {
        insert_conversation(connection, &generated_id, sender, reciever); 
        let new_conversation = Conversation{
            conversation_id: generated_id,
            user1_id: sender.to_string(),
            user2_id: reciever.to_string(),
        }; 
    
        append_string_to_file("conversation.json", serde_json::to_string_pretty(&new_conversation).unwrap()).expect("Unable to read file");
        let message_response = "You've started a conversation with: ";
        let response_json = serde_json::to_string(message_response).expect("Error serializing");
        (StatusCode::CREATED, Json(response_json))
    } else {
        let message_response = "User not found.";
        let response_json = serde_json::to_string(message_response).expect("Error serializing");
        (StatusCode::NOT_FOUND, Json(response_json))
    }   
}

fn insert_conversation(conn: &mut PgConnection, uuid_input: &str, user1_input: &str, user2_input: &str) {
    use schema::conversations::dsl::*;

    insert_into(conversations)
        .values((id.eq(uuid_input), user1_id.eq(user1_input), user2_id.eq(user2_input)))
        .execute(conn)
        .expect("Error inseting messages");
}

fn search_user(conn: &mut PgConnection, username_input: &str) -> bool {
    use schema::users::dsl::*;

    match users.filter(username.eq(username_input)).first::<Users>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn list_conversations(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let connection = &mut establish_connection();

    let message_response = show_conversations(connection, &payload);
    
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(response_json))
}

fn show_conversations(conn: &mut PgConnection, username_input: &str) -> Vec<Conversations>{
    use schema::conversations::dsl::*;

    let results = conversations
        .filter(user1_id.eq(username_input).or(user2_id.eq(username_input)))
        .load::<Conversations>(conn)
        .expect("Error loading conversations");

    return results;
}


// the input to our `create_message` handler
#[derive(Deserialize)]
struct CreateMessage {
    username: String,
    message: String,
    conversation_id: String
}

// the output to our `create_message` handler
#[derive(Serialize, Debug)]
struct Message {
    username: String,
    message: String,
    conversation_id: String,
}

#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String

}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Conversation {
    conversation_id: String,
    user1_id: String,
    user2_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateConversation {
    username_sender: String,
    username_receiver: String
}