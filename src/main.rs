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


use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::associations::HasTable;
use dotenvy::dotenv;
use std::env;
use diesel::insert_into;
pub mod schema;
pub mod models;
use crate::models::Messages;
use crate::models::Users; 
use bcrypt::{hash, verify};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::Message as Email; 
use lettre::message::Mailbox;

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

    insert_message(connection, &payload.username, &payload.message);

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
    //println!("{&payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password:?}"); 
    let new_user = User{
        username: payload.username, 
        first_name: payload.first_name, 
        last_name: payload.last_name, 
        email: payload.email, 
        password: hashed_password, 
    }; 
    
    //append_string_to_file("users.json", serde_json::to_string_pretty(&new_user).unwrap()).expect("Unable to read file");

    (StatusCode::CREATED, Json(new_user))

}

async fn list_users()  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into users table    
    let connection = &mut establish_connection();

    let user_response = show_users(connection);

    println!("{:?}", user_response);
    
    //serialize query response to json string
    let response_json = serde_json::to_string(&user_response).expect("Error serializing");

    //status code 200 Ok
    (StatusCode::OK, Json(response_json))
}

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
pub fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str) {
    use schema::messages::dsl::*;

    insert_into(messages)
        .values((user_id.eq(username), message.eq(message_text)))
        .execute(conn)
        .expect("Error inserting messages");
}

fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str) -> Result<usize, diesel::result::Error> {
        use schema::users::dsl::*;
    
        let rows_inserted = insert_into(users)
            .values((username.eq(username_input), password.eq(password_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input)))
            .execute(conn2)?;
        println!("{rows_inserted}:?"); 
        Ok(rows_inserted)
}

//select * from messages
pub fn show_messages(conn: &mut PgConnection) -> Vec<Messages> {
    use schema::messages::dsl::*;

    let results = messages
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
    println!("HELLO1"); 
    // Retrieve the hashed password associated with the provided email
    if user_exists {
        println!("HELLO2"); 
        let hashed_password = match get_hashed_password_for_email(connection, &payload.email) {
            Some(password) => password,
            None => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Error retrieving password".to_string())),
        };

        if !verify_password(&payload.password, &hashed_password) {
            return (StatusCode::UNAUTHORIZED, Json("Authentication failed".to_string()));
        }
        
        let message_response = "Welcome!";
        let response_json = serde_json::to_string(message_response).expect("Error serializing");
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
    let email = Email::builder()
        .from("akaapp315@gmail.com".parse::<Mailbox>().unwrap())
        .to(email_input.parse::<Mailbox>().unwrap())
        .subject("Test Email")
        .body("Hello, this is a test email!".to_string())
        .unwrap();
  
    let mailer = create_mailer();
  
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }
  }
  
async fn new_password(Json(payload): Json<NewPassword>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    let hashed_password = match hash_password(&payload.password) {
        Ok(other_password) => other_password,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    };
    println!("123..."); 
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(password.eq(&hashed_password));
       // println!("{:?}", update_row); 
    println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
    match update_row.execute(connection) {
            Ok(rows_affected) => {
                // Check if any rows were affected
                if rows_affected > 0 {
                    println!("it worked");
                    return (StatusCode::OK, Json("Password updated successfully".to_string()));
                    println!("it worked");
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                    println!("LOL!");
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
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
    message: String
}

// the output to our `create_message` handler
#[derive(Serialize, Debug)]
struct Message {
    username: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String
}

#[derive(Serialize, Debug)]
struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String

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
struct NewPassword{
    email: String, 
    password: String
}
