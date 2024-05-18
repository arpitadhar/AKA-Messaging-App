use std::fs;
use std::io::Write;

use axum::{ extract::State, 
    routing::{get, post},
    http::StatusCode,
    Json, Router, Extension
};
use serde::{Deserialize, Serialize};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer};

use tracing_subscriber::{filter::{self, combinator::And}, fmt::writer::MakeWriterExt};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::associations::HasTable;
use dotenvy::dotenv;
use std::env;
use diesel::insert_into;
pub mod schema;
pub mod models;
use crate::{models::{Conversations, Messages}, schema::conversations};
use crate::models::Users; 
use bcrypt::{hash, verify};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::Message as Email; 
use lettre::message::Mailbox;
//use orion::kex::*;
//use orion::aead;
use rand::Rng;

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
        .route("/messages", post(list_messages))
        //POST /create-user goes to create_user()
        .route("/create-users", post(create_user))
        //GET /users goes to list_users()
        .route("/users", get(list_users))
        //POST /login goes to login
        .route("/login", post(login))
        //POST /forgot-password goes to forgot_password()
        .route("/forgot-password", post(forgot_password))
        //POST /new-password goes to new_password()
        .route("/new-password", post(new_password))
        //POST /create-conversation goes to create_conversation()
        .route("/create-conversation", post(create_conversation))
        //POST /conversations goes to list_conversations()
        .route("/conversations", post(list_conversations))
        //.route("/verify-token", post(verify_token))
        //.route("/all_users", get(all_users))
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
    println!("connection established"); 
    insert_message(connection, &payload.username, &payload.message, &payload.conversation_id);
    println!("message inserted");
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
    let default_user = User{
        username: " ".to_string(), 
        first_name: " ".to_string(), 
        last_name: " ".to_string(), 
        email: " ".to_string(), 
        password: " ".to_string(), 
    }; 
    let connection = &mut establish_connection();
    if check_email_exists(connection, &payload.email){
        let message = "Email already exists"; 
        return (StatusCode::BAD_REQUEST, Json(default_user)); 
    }
    let hashed_password = match hash_password(&payload.password) {
        Ok(password) => password,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(default_user)),
    };

    insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &hashed_password); 
    //println!("{&payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password}"); 
    let new_user = User{
        username: payload.username, 
        first_name: payload.first_name, 
        last_name: payload.last_name, 
        email: payload.email, 
        password: hashed_password,  
    }; 
    
    append_string_to_file("users.json", serde_json::to_string_pretty(&new_user).unwrap()).expect("Unable to read file");

    (StatusCode::CREATED, Json(new_user))

}


async fn list_users()  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into users table    
    let connection = &mut establish_connection();

    let user_response = show_users(connection);

    println!("{:?}", user_response);
    
    //serialize query response to json string
    let response_json = serde_json::to_string(&user_response).expect("Error serializing");
    
    // let response_json = match query_params.fields.as_ref().map(String::as_str) {
    //     Some("username") => serde_json::to_string(&user_response.iter().map(|user| user.username.clone()).collect::<Vec<_>>()),
    //     Some("email") => serde_json::to_string(&user_response.iter().map(|user| user.email.clone()).collect::<Vec<_>>()),
    //     _ => serde_json::to_string(&user_response),
    // }.expect("Error serializing");
    //status code 200 Ok
    (StatusCode::OK, Json(response_json))
}

//https://medium.com/@thecodingblog/how-to-make-a-rest-api-in-rust-257862fdb728
//#[get("/all_users")]
// fn get_users(conn: &PgConnection) -> Vec<User>{
//     use schema::users::dsl::*;
//     users.load::<User>(conn)
//         .expect("Error loading users")
// }
// //#[get("all_users")]
// fn all_users(conn: &PgConnection) -> Json<Vec<User>>{
//     let connection = establish_connection(); 
//     let users = get_users(&connection); 
//     Json(users)
// }

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
    println!("goes into insert message"); 
    insert_into(messages)
        .values((user_id.eq(username), message.eq(message_text),conversation_id.eq(conversation)))
        .execute(conn)
        .expect("Error inserting messages");
}

fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> Result<usize, diesel::result::Error> {
        println!("pls"); 
        use schema::users::dsl::*;
        println!("1"); 
        let rows_inserted = insert_into(users)
            .values((username.eq(username_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input), password.eq(password_input)))
            .execute(conn2)?;
        println!("{rows_inserted}:?"); 
        Ok(rows_inserted)
}

// fn delete_user(conn: &mut PgConnection, email_input: &str){ 
//     use schema::users::dsl::*; 
//     diesel::delete(users.filter(email.eq(email_input))).execute(connection)?; 


// }


//https://docs.diesel.rs/master/diesel/fn.delete.html
fn delete_user(conn: &mut PgConnection, email_input: &str) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;

    let deleted_rows = diesel::delete(users.filter(email.eq(email_input)))
        .execute(conn)?;

    Ok(deleted_rows)
}

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
        .expect("Error loading users");

    return results;
}

//Route for user login
//@return: Status Code, JSON string
//Calls check_user_exists
async fn login(
    Json(payload): Json<Login>
) -> (StatusCode, Json<String>) { 
    let connection: &mut PgConnection = &mut establish_connection();
    let user_exists = check_email_exists(connection, &payload.email);
    if user_exists {
        let hashed_password = match get_hashed_password_for_email(connection, &payload.email) {
            Some(password) => password,
            None => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Error retrieving password".to_string())),
        };

        if !verify_password(&payload.password, &hashed_password) { 
            return (StatusCode::UNAUTHORIZED, Json("Authentication failed".to_string()));
        }
        
        let message_response = retrieve_username(connection, &payload.email);
        let response_json = serde_json::to_string(&message_response).expect("Error serializing");
        (StatusCode::OK, Json(response_json))
    } 
    else {
        let message_response = "Incorrect email or password.";
        let response_json = serde_json::to_string(message_response).expect("Error serializing");
        (StatusCode::UNAUTHORIZED, Json(response_json))
    }
}

fn check_user_exists(conn: &mut PgConnection, email_input: &str, password_input: &str) -> bool{
    use schema::users::dsl::*;
    
    match users.filter(email.eq(email_input).and(password.eq(password_input))).first::<Users>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

//route for forgot password
async fn forgot_password(
    Json(payload): Json<Password>
) -> (StatusCode, Json<String>){
    use schema::users::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();
    if check_email_exists(connection, &payload.email) {
        println!("{}: Email exists", payload.email);
        send_email(&payload.email);
        (StatusCode::OK, Json("Reset password email sent".to_string()))

    } else {
        println!("{}: Email not found", payload.email);
        (StatusCode::NOT_FOUND, Json("Email not found".to_string()))
    }
}


fn check_email_exists(conn: &mut PgConnection, email_input: &str) -> bool{
    use schema::users::dsl::* ;
    
    match users.filter(email.eq(email_input)).first::<Users>(conn){
        Ok(_) => true,
        Err(_) => false,
    }
    
}

fn get_hashed_password_for_email(conn: &mut PgConnection, email_input: &str) -> Option<String> {
    use schema::users::dsl::*;

    match users.filter(email.eq(email_input)).select(password).first::<String>(conn) {
        Ok(password_value) => Some(password_value),
        Err(_) => None,
    }
}

fn create_mailer() -> SmtpTransport {
    // Get the username and password from the env file
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    let creds = Credentials::new(username, password);
    SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build()
}



fn send_email(email_input: &str) {
    // Build the email
    //let connection: &mut PgConnection = &mut establish_connection();
    let auth_number = rand::thread_rng().gen_range(1..=100); 
    let reset_email = "http://localhost:5173/verify"; 
    let email_body = format!("Hello, this is from the AKA Messaging Team. Click \"{}\" and type in {}", reset_email, auth_number); 
    let email = Email::builder()
        .from("akaapp315@gmail.com".parse::<Mailbox>().unwrap())
        .to(email_input.parse::<Mailbox>().unwrap())
        .subject("RESET PASSWORD")
        .body(email_body)
        .unwrap();
  

    // new_token(email_input, &auth_number.to_string()); 
    let mailer = create_mailer();
    
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }
    //use schema::users::dsl::*; 
    // let update_row = diesel::update(users.filter(email.eq(email_input)))
    //     .set(token.eq(&auth_number));

}

// fn new_token(reset_email: &str, token_number: &str){
//     use schema::users::dsl::*; 
//     diesel::update(users.filter(email.eq(&reset_email)))
//         .set(token.eq(&token_number)); 
// }

// async fn verify_token(Json(payload): Json<VerifyToken>) -> (StatusCode, Json<String>){
//     use schema::users::dsl::*; 
//     let conn: &mut PgConnection = &mut establish_connection(); 
//     let reset_email = &payload.email; 
//     let mut old_token: Option<String> = None;

//     match users.filter(email.eq(reset_email)).select(token).first::<String>(conn){
//         Ok(token_value) => {
//             old_token = Some(token_value); 
//         }
//         Err(_) => {
//             println!("wrong token"); 
//         }
//     }
//       if payload.token == old_token.unwrap(){
//         let message_response = "You can now change your password";
//         let response_json = serde_json::to_string(message_response).expect("Error serializing");
//         (StatusCode::OK, Json(response_json))
//       }
//       else {
//         let message_response = "Incorrect code.";
//         let response_json = serde_json::to_string(message_response).expect("Error serializing");
//         (StatusCode::UNAUTHORIZED, Json(response_json))
//       }

// }


  
async fn new_password(Json(payload): Json<NewPassword>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    let hashed_password = match hash_password(&payload.password) {
        Ok(other_password) => other_password,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    };
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(password.eq(&hashed_password));
       // println!("{:?}", update_row); 
    println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
    match update_row.execute(connection) {
            Ok(rows_affected) => {
                // Check if any rows were affected
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("Password updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}



async fn create_conversation(
    Json(payload): Json<CreateConversation>
) -> (StatusCode, Json<String>) {
    
    let connection: &mut PgConnection = &mut establish_connection();
    let user1_exists = search_user(connection, &payload.username_receiver);
    let user2_exists = search_user(connection, &payload.username_sender); 
     
    let sender = &payload.username_sender.to_owned();
    let reciever = &payload.username_receiver.to_owned();
    println!("sender"); 
    //let generatedID: String = sender.clone() + reciever;
    let mut sorted_usernames = vec![sender, reciever];
    sorted_usernames.sort();

    let generated_id:String = sorted_usernames.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("");
    
    if user1_exists && user2_exists {
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
    //println!("end"); 
}

fn insert_conversation(conn: &mut PgConnection, uuid_input: &str, user1_input: &str, user2_input: &str) {
    use schema::conversations::dsl::*;

    insert_into(conversations)
        .values((id.eq(uuid_input), user1_id.eq(user1_input), user2_id.eq(user2_input)))
        .execute(conn)
        .expect("Error inserting messages");
}

fn search_user(conn: &mut PgConnection, username_input: &str) -> bool {
    use schema::users::dsl::*;

    match users.filter(username.eq(username_input)).first::<Users>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn retrieve_username(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*;
    match users.filter(email.eq(email_input)).select(username).first::<String>(conn)  {
        Ok(username1) => Some(username1), 
        Err(_) => None,
    }
}

async fn list_conversations(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let connection = &mut establish_connection();
    println!("connection established"); 
    println!("{}", payload); 
    let message_response = show_conversations(connection, &payload);
    println!("show conversations works?"); 
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(response_json))
}

fn show_conversations(conn: &mut PgConnection, username_input: &str) -> Vec<Conversations>{
    use schema::conversations::dsl::*;
    println!("ok inside function"); 
    println!("username_input"); 
    let results = conversations
        .filter(user1_id.eq(username_input).or(user2_id.eq(username_input)))
        .load::<Conversations>(conn)
        .expect("Error loading conversations");
    
    return results;
}

//hash password 
fn hash_password(password: &str) -> Result<String, StatusCode> {
    hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password)
        .unwrap_or(false)
}




//the input to our `create_message` handler
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
    password: String, 
}

#[derive(Serialize, Debug)]
struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String, 

}

#[derive(Deserialize)]
struct Login{
    email: String,
    password: String
}

#[derive(Deserialize)]
struct Password{
    email: String
}

#[derive(Deserialize)]
struct VerifyToken{
    email: String, 
    token: String
}

#[derive(Deserialize)]
struct NewPassword{
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
    //conversation_id: String,
    username_sender: String,
    username_receiver: String
}



// use std::fs;
// use std::io::Write;

// use axum::{
//     routing::{get, post},
//     http::StatusCode,
//     Json, Router,
// };
// use serde::{Deserialize, Serialize};
// use tower_http::{
//     trace::TraceLayer,
//     cors::CorsLayer};


// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;
// use diesel::insert_into;

// pub mod schema;
// pub mod models;
// use crate::models::Messages;
// use crate::models::Users; 

// #[tokio::main]

// async fn main() {

//     //initialize tracing
//     tracing_subscriber::fmt::init();

//     let app = Router::new()
//         //GET / goes to root
//         .route("/", get(root))
//         //POST /create-message` goes to create_message()
//         .route("/create-message", post(create_message))
//         //GET /message` goes to list_message()
//         .route("/messages", get(list_messages))
//         //POST /create-user goes to create_user()
//         .route("/create-users", post(create_user))
//         //GET /users goes to list_users()
//         .route("/users", get(list_users))
//         //allows frontend to communicate with backend
//         .layer(CorsLayer::permissive())
//         .layer(TraceLayer::new_for_http());

//     //run app istening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// //root endpoint, returns string
// async fn root() -> &'static str {
//     "Message parsing and delivery system!"
// }

// async fn create_message(
//     //Parse the request body into a <CreateMessage> type
//     Json(payload): Json<CreateMessage>,
// ) -> (StatusCode, Json<Message>) {

//     //establish database connection and insert values into messages table
//     let connection = &mut establish_connection();

//     insert_message(connection, &payload.username, &payload.message);

//     let new_message = Message {
//         username: payload.username,
//         message: payload.message,
//     };


//     append_string_to_file("text.json", serde_json::to_string_pretty(&new_message).unwrap()).expect("Unable to read file");

//     //status code 201 Created
//     (StatusCode::CREATED, Json(new_message))
// }

// async fn list_messages()  -> (StatusCode, Json<String>) {

//     //establish database connection and insert values into messages table    
//     let connection = &mut establish_connection();

//     let message_response = show_messages(connection);

//     println!("{:?}",message_response);
    
//     //serialize query response to json string
//     let response_json = serde_json::to_string(&message_response).expect("Error serializing");

//     //status code 200 Ok
//     (StatusCode::OK, Json(response_json))
// }

// async fn create_user(
//     //Parse the request body into a <CreateMessage> type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {

//     //establish database connection and insert values into messages table
//     let connection = &mut establish_connection();

//     insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password); 
//     //println!("{&payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password:?}"); 
//     let new_user = User{
//         username: payload.username, 
//         first_name: payload.first_name, 
//         last_name: payload.last_name, 
//         email: payload.email, 
//         password: payload.password, 
//     }; 
    
//     append_string_to_file("users.json", serde_json::to_string_pretty(&new_user).unwrap()).expect("Unable to read file");

//     (StatusCode::CREATED, Json(new_user))

// }

// async fn list_users()  -> (StatusCode, Json<String>) {

//     //establish database connection and insert values into users table    
//     let connection = &mut establish_connection();

//     let user_response = show_users(connection);

//     println!("{:?}", user_response);
    
//     //serialize query response to json string
//     let response_json = serde_json::to_string(&user_response).expect("Error serializing");

//     //status code 200 Ok
//     (StatusCode::OK, Json(response_json))
// }

// //Below are the remains of an attempt to store data within a JSON file
// //we experienced minimal success with this

// //fn append_string_to_file(path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
// //    let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
// //
// //    // You need to take care of the conversion yourself
// //    // and can either try to write all data at once
// //    file.write_all(&data.as_bytes())?;
// //
// //    file.write(b"\n ,")?;
// //    file.flush()?;
// //
// //    Ok(())
// //}

// //ABSOLUTELY VERY BAD NOT GOOD PRACTICE
// //this is should be a module from lib.rs
// //
// //connect to postgres using a url made up of 'postgres://[username]:[password]@localhost/[db_name]'
//  fn append_string_to_file(path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
//         let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
    
//         // You need to take care of the conversion yourself
//         // and can either try to write all data at once
//         file.write_all(&data.as_bytes())?;
    
//         file.write(b"\n ,")?;
//         file.flush()?;
    
//         Ok(())
//     }


// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// //insert into messages table
// pub fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str) {
//     use schema::messages::dsl::*;

//     insert_into(messages)
//         .values((user_id.eq(username), message.eq(message_text)))
//         .execute(conn)
//         .expect("Error inserting messages");
// }

// //insert into users table 
// // fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> QueryResult<usize>{
// //     use schema::users::dsl::*;

// //     insert_into(users)
// //         .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
// //         .execute(conn2)
// //         .expect("Error inseting messages");
// //     }
// //changed the return type for this previous function keeps causing errors
// fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> Result<usize, diesel::result::Error> {
//         use schema::users::dsl::*;
    
//         let rows_inserted = insert_into(users)
//             .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
//             .execute(conn2)?;
    
//         Ok(rows_inserted)
// }

// //select * from messages
// pub fn show_messages(conn: &mut PgConnection) -> Vec<Messages> {
//     use schema::messages::dsl::*;

//     let results = messages
//         .load::<Messages>(conn)
//         .expect("Error loading messages");

//     return results;
// }

// //select * from users 
// pub fn show_users(conn: &mut PgConnection) -> Vec<Users> {
//     use schema::users::dsl::*;

//     let results = users
//         .load::<Users>(conn)
//         .expect("Error loading messages");

//     return results;
// }

// // the input to our `create_message` handler
// #[derive(Deserialize)]
// struct CreateMessage {
//     username: String,
//     message: String
// }

// // the output to our `create_message` handler
// #[derive(Serialize, Debug)]
// struct Message {
//     username: String,
//     message: String,
// }

// #[derive(Deserialize, Debug)]
// struct CreateUser {
//     username: String,
//     first_name: String, 
//     last_name: String, 
//     email: String, 
//     password: String
// }

// #[derive(Serialize, Debug)]
// struct User {
//     username: String,
//     first_name: String, 
//     last_name: String, 
//     email: String, 
//     password: String

// }