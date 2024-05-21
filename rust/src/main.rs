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
use crate::models::Flagged;
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
        //POST /user-info goes to retrieve_user_information()
        .route("/user-info", post(retrieve_user_information))
        .route("/update-username", post(update_username))
        .route("/update-first", post(update_first))
        .route("/update-last", post(update_last))
        .route("/update-img", post(update_img))
        .route("/create-flag", post(create_flagged_user))
        .route("/flagged", get(list_flagged))
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

//creates message by taking in user_id, message, and conversation_id
async fn create_message(
    //Parse the request body into a <CreateMessage> type
    Json(payload): Json<CreateMessage>,
) -> (StatusCode, Json<Message>) {

    //establish database connection and insert values into messages table
    let connection = &mut establish_connection();
    println!("connection established"); 
    insert_message(connection, &payload.user_id, &payload.message, &payload.conversation_id);
    println!("message inserted");
    let new_message = Message {
        user_id: payload.user_id,
        message: payload.message,
        conversation_id: payload.conversation_id,
    };

    append_string_to_file("text.json", serde_json::to_string_pretty(&new_message).unwrap()).expect("Unable to read file");

    //status code 201 Created
    (StatusCode::CREATED, Json(new_message))
}

//lists the messages in the datasbase
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

//POST, creates user by taking in 6 inputs and checking if the email/username already exists. it then 
//hashes the password and inserts the user into the database
async fn create_user(
    //Parse the request body into a <CreateMessage> type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<String>) {
    //establish database connection and insert values into messages table
    let default_user = User{
        username: " ".to_string(), 
        first_name: " ".to_string(), 
        last_name: " ".to_string(), 
        email: " ".to_string(), 
        password: " ".to_string(), 
        is_admin: false, 
        img_url: " ".to_string(),
    }; 
    let connection = &mut establish_connection();
    if check_email_exists(connection, &payload.email){
        let message = "Email already exists"; 
        return (StatusCode::BAD_REQUEST, Json(message.to_string())); 
    }
    if check_username_exists(connection, &payload.username){
        let message = "Username already exists"; 
        return (StatusCode::BAD_REQUEST, Json(message.to_string())); 
    }

    let hashed_password = match hash_password(&payload.password) {
        Ok(password) => password,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("failed to hash password".to_string())),
    };

    insert_user(connection, &payload.username, &payload.first_name, &payload.last_name, &payload.email, &hashed_password, payload.is_admin); 
    //println!("{&payload.username, &payload.first_name, &payload.last_name, &payload.email, &payload.password}"); 
    let new_user = User{
        username: payload.username, 
        first_name: payload.first_name, 
        last_name: payload.last_name, 
        email: payload.email, 
        password: hashed_password,  
        is_admin: payload.is_admin, 
        img_url: "".to_string(),
    }; 
    
    append_string_to_file("users.json", serde_json::to_string_pretty(&new_user).unwrap()).expect("Unable to read file");

    (StatusCode::CREATED, Json("new user created".to_string()))

}

//get route that lists all the users in the users table in the database 
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

//https://medium.com/@thecodingblog/how-to-make-a-rest-api-in-rust-257862fdb728

//connect to postgres using a url made up of 'postgres://[username]:[password]@localhost/[db_name]'
 fn append_string_to_file(path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
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
//inserts the user into the users tables
fn insert_user(conn2: &mut PgConnection, username_input: &str, first_name_input: &str, last_name_input: &str, email_input: &str, password_input: &str, is_admin_input: bool ) -> Result<usize, diesel::result::Error> {
        println!("pls"); 
        use schema::users::dsl::*;
        println!("1"); 
        let rows_inserted = insert_into(users)
            .values((username.eq(username_input), first_name.eq(first_name_input), last_name.eq(last_name_input), email.eq(email_input), password.eq(password_input), is_admin.eq(is_admin_input)))
            .execute(conn2)?;
        println!("{rows_inserted}:?"); 
        Ok(rows_inserted)
    
}

//creates a flagged user to insert into the database, takes email and reason 
async fn create_flagged_user(
    Json(payload): Json<CreateFlagged>,
) -> (StatusCode, Json<FlaggedUser>) {

    let connection = &mut establish_connection();
    println!("connection established"); 
    insert_flagged_user(connection, &payload.email, &payload.reason);
    println!("message inserted");
    let new_flag = FlaggedUser {
        email: payload.email,
        reason: payload.reason,
    };

    append_string_to_file("text.json", serde_json::to_string_pretty(&new_flag).unwrap()).expect("Unable to read file");

    //status code 201 Created
    (StatusCode::CREATED, Json(new_flag))
}

//inserts flagged user into the flagged table, takes in email_input and reason_input, it then sends an 
//email to the user saying that they've been flagged
fn insert_flagged_user(conn2: &mut PgConnection, email_input: &str, reason_input: &str) -> Result<usize, diesel::result::Error> {
    println!("pls"); 
    use schema::flagged::dsl::*;
    println!("1"); 
    let user_exists = check_email_exists(conn2, email_input); 
    let user_not_flagged = not_flagged(conn2, email_input); 
    if user_exists && user_not_flagged{
        let rows_inserted = insert_into(flagged)
        .values((email.eq(email_input), reason.eq(reason_input)))
        .execute(conn2)?;
        println!("{rows_inserted}:?"); 
        send_email_flagged(email_input, reason_input); 
        Ok(rows_inserted)
    }
    else{
        println!("Email exists"); 
        Ok(0)
    }
}

//get route that lists all the flagged users 
async fn list_flagged()  -> (StatusCode, Json<String>) {

    //establish database connection and insert values into users table    
    let connection = &mut establish_connection();

    let user_response = show_flagged(connection);

    println!("{:?}", user_response);
    let response_json = serde_json::to_string(&user_response).expect("Error serializing");
    (StatusCode::OK, Json(response_json))
}

//show flagged users from flagged table
pub fn show_flagged(conn: &mut PgConnection) -> Vec<Flagged> {
    use schema::flagged::dsl::*;
    let results = flagged
        .load::<Flagged>(conn)
        .expect("Error loading flagged sers");

    return results;
}


//https://docs.diesel.rs/master/diesel/fn.delete.html
fn delete_user(conn: &mut PgConnection, email_input: &str) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;
    
    let deleted_rows = diesel::delete(users.filter(email.eq(email_input)))
        .execute(conn)?;

    Ok(deleted_rows)
}

// async fn delete(
//     Json(payload): Json<Password>
// ) -> (StatusCode, Json<String>) {
//     let connection: &mut PgConnection = &mut establish_connection();

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
        .expect("Error loading users");

    return results;
}

//route for user login
//check if email exists first 
//check if password matches with hashed password in database using verify_password 
//returns a status code and json response 
async fn login(
    Json(payload): Json<Login>
) -> (StatusCode, Json<String>) { 
    let connection: &mut PgConnection = &mut establish_connection();
    let user_exists = check_email_exists(connection, &payload.email);
    let default_user = User{
        username: " ".to_string(), 
        first_name: " ".to_string(), 
        last_name: " ".to_string(), 
        email: " ".to_string(), 
        password: " ".to_string(), 
        is_admin: false, 
        img_url: " ".to_string(),
    }; 
    if user_exists {
        let hashed_password = match get_hashed_password_for_email(connection, &payload.email) {
            Some(password) => password,
            None => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Error retrieving password".to_string())),
        };

        if !verify_password(&payload.password, &hashed_password) { 
            return (StatusCode::UNAUTHORIZED, Json("Authentication failed".to_string()));
        }
        
        let username_response = retrieve_username(connection, &payload.email);
        let user_firstname = retrieve_firstname(connection, &payload.email);
        let user_lastname = retrieve_lastname(connection, &payload.email);
        let user_password = get_hashed_password_for_email(connection, &payload.email); 
        let user_admin = retrieve_isadmin(connection, &payload.email); 
        let message_response = username_response; 
        let response_json = serde_json::to_string(&message_response).expect("Error serializing");
        println!("{response_json}:?"); 
        (StatusCode::OK, Json(response_json))
    } 
    else {
        let message_response = "Incorrect email or password.";
        let response_json = serde_json::to_string(message_response).expect("Error serializing");
        (StatusCode::UNAUTHORIZED, Json(response_json))
    }
}
//this route is called when on the user profile page
//this takes the username from the localStorage and then uses all the helper functions 
//to retrieve the other information about the user
//this then sends a user object to the front end 
async fn retrieve_user_information(Json(payload): Json<String>
) -> (StatusCode, Json<User>){
    let connection: &mut PgConnection = &mut establish_connection();
    let default_user = User{
        username: " ".to_string(), 
        first_name: " ".to_string(), 
        last_name: " ".to_string(), 
        email: " ".to_string(), 
        password: " ".to_string(), 
        is_admin: false, 
        img_url: " ".to_string(),
    }; 
    println!("1?");
    let email_response = retrieve_email(connection, &payload).expect("reason").to_string();
    println!("email"); 
    let user_exists = check_email_exists(connection, &email_response);
    println!("exists"); 
       // let username_response = retrieve_username(connection, &payload).expect("reason").to_string();
    let user_firstname = retrieve_firstname(connection, &email_response).expect("reason").to_string();
    println!("first"); 
    let user_lastname = retrieve_lastname(connection, &email_response).expect("reason").to_string();
    let user_password = retrieve_password(connection, &email_response).expect("reason").to_string(); 
    let user_admin = retrieve_isadmin(connection, &email_response); 
    let user_img_url = retrieve_img(connection, &email_response).expect("reason").to_string();
    println!("{}", user_admin); 
    let default_user1 = User{
        username: payload, 
        first_name: user_firstname, 
        last_name: user_lastname, 
        email: email_response, 
        password: user_password, 
        is_admin: user_admin, 
        img_url: user_img_url,
    }; 
    if user_exists{
        //let response_json = serde_json::to_string(default_user1).expect("Error serializing");
        println!("?"); 
        (StatusCode::OK, Json(default_user1))
    }
    else{
        (StatusCode::UNAUTHORIZED, Json(default_user))
    }
}

//checks if an user already exists using email, returns a boolean 
fn check_user_exists(conn: &mut PgConnection, email_input: &str, password_input: &str) -> bool{
    use schema::users::dsl::*;
    
    match users.filter(email.eq(email_input).and(password.eq(password_input))).first::<Users>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

//checks if an user is not already flagged before adding them to the flagged table
fn not_flagged(conn: &mut PgConnection, email_input: &str) -> bool{
    use schema::flagged::dsl::*; 
    match flagged.filter(email.eq(email_input)).first::<Flagged>(conn){
        Ok(_) => false,
        Err(_) => true,
    }
}

//route for forgot password
//creates an authentication number using rand crate 
//uses check_email_exists function to make sure the email the user inputted is correct
//calls send email function if successful and sends email 
async fn forgot_password(
    Json(payload): Json<Password>
) -> (StatusCode, Json<String>){
    use schema::users::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();
    let auth_number = rand::thread_rng().gen_range(1..=100).to_string(); 
    if check_email_exists(connection, &payload.email) {
        println!("{}: Email exists", payload.email);
        send_email(&payload.email, &auth_number);
        (StatusCode::OK, Json(auth_number.to_string()))

    } else {
        println!("{}: Email not found", payload.email);
        (StatusCode::NOT_FOUND, Json("Email not found".to_string()))
    }
}

//helper function that checks if an email already exists in the database 
fn check_email_exists(conn: &mut PgConnection, email_input: &str) -> bool{
    use schema::users::dsl::* ;
    
    match users.filter(email.eq(email_input)).first::<Users>(conn){
        Ok(_) => true,
        Err(_) => false,
    }
    
}

//this function checks if an username exists in the users table, takes in username input
fn check_username_exists(conn: &mut PgConnection, username_input: &str) -> bool{
    use schema::users::dsl::* ;
    
    match users.filter(username.eq(username_input)).first::<Users>(conn){
        Ok(_) => true,
        Err(_) => false,
    }
    
}

//returns the hashed password from an email 

fn get_hashed_password_for_email(conn: &mut PgConnection, email_input: &str) -> Option<String> {
    use schema::users::dsl::*;

    match users.filter(email.eq(email_input)).select(password).first::<String>(conn) {
        Ok(password_value) => Some(password_value),
        Err(_) => None,
    }
}

//this function creates the mailer for the send_email function 
//takes in credentials from the .env file 

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

//sends email using the create_mailer() function
//first you take the auth_number and you build a reset email using that 
//you format the body of email and include the auth number 

fn send_email(email_input: &str, auth_number: &str) {
    let reset_email = format!("http://localhost:5173/verify/{}", auth_number); 
    let email_body = format!("Hello, this is from the AKA Messaging Team. Your code is {}", auth_number); 
    let email = Email::builder()
        .from("akaapp315@gmail.com".parse::<Mailbox>().unwrap())
        .to(email_input.parse::<Mailbox>().unwrap())
        .subject("RESET PASSWORD")
        .body(email_body)
        .unwrap();
    let mailer = create_mailer();
    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }

}

fn send_email_flagged(email_input: &str, reason_input: &str) {
    let reset_email = "http://localhost:5173/verify"; 
    let email_body = format!("Hello, this is from the AKA Messaging Team. You have been flagged for {}", reason_input); 
    let email = Email::builder()
        .from("akaapp315@gmail.com".parse::<Mailbox>().unwrap())
        .to(email_input.parse::<Mailbox>().unwrap())
        .subject("FLAGGED AKA")
        .body(email_body)
        .unwrap();
  
    let mailer = create_mailer();
    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }

}



//route for resetting password after you forgot it or when you want to change it
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
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("Password updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}
//updates the first name 
async fn update_first(Json(payload): Json<InputEmail>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    println!("1"); 
    println!("{}", &payload.input); 
    let new_first = &payload.input; 
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(first_name.eq(&payload.input));
       // println!("{:?}", update_row); 
    println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
    match update_row.execute(connection) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("First name updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}
//update last name route, takes email payload and new last name, returns a status code and a string 
async fn update_last(Json(payload): Json<InputEmail>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    let new_last = &payload.input; 
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(last_name.eq(&payload.input));
       // println!("{:?}", update_row); 
    println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
    match update_row.execute(connection) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("Last name updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}

//update profile img route, takes email payload and new img url, returns a status code and a string 
async fn update_img(Json(payload): Json<NewImage>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    //let new_img = &payload.new_img; 
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(img_url.eq(&payload.new_img));

    println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
    match update_row.execute(connection) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("Profile image updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}

//this is the update username function. it takes a payload with the email, old username, and new username. it successfully 
//updates the username in the users table but when the other two functions were added for called for changing 
//the username in the messages and conversations database, it started breaking 
//it might be because of the match function (which identifies patterns and sort of acts like a for loop)

async fn update_username(Json(payload): Json<UpdateUser>) -> (StatusCode, Json<String>){
    use schema::users::dsl::*; 
    let connection: &mut PgConnection = &mut establish_connection();
    let new_first = &payload.new_username; 
    //let old_username = retrieve_username(connection, &payload.email); 
    // if check_has_messages(connection, &payload.username_old){
    //     let update_row2 = update_username_messages(connection, &payload.username_old, &payload.new_username);
    // }
    let update_row = diesel::update(users.filter(email.eq(&payload.email)))
        .set(username.eq(&payload.new_username));
       // println!("{:?}", update_row); 
         println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&update_row).to_string());

        //.get_result(connection);
         match update_row.execute(connection) {
            Ok(rows_affected) => {
                // Check if any rows were affected
                if rows_affected > 0 {
                    return (StatusCode::OK, Json("Username updated successfully".to_string()));
                } else {
                    return (StatusCode::NOT_FOUND, Json("User not found".to_string()));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
       }
  
    
}

//this function updates the username in the messages table. it takes the old username and the old username and it 
//updates the username by using the update function from diesel, return true if successful 

fn update_username_messages(conn: &mut PgConnection, old_user: &str, new_user: &str) -> bool{
    use schema::messages::dsl::*;
    let update_row2 = diesel::update(messages.filter(user_id.eq(old_user)))
        .set(user_id.eq(new_user));
        match update_row2.execute(conn) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    println!("updated!"); 
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => return false,
    }

}


//this updates the username in the conversation. takes the old username and new username, uses the update function
//from diesel and returns true if successful 

fn update_username_conversation(conn: &mut PgConnection, old_user: &str, new_user: &str) -> bool{
    use schema::conversations::dsl::*;
    let is_user_1 = is_user1(conn, old_user); 
    if is_user_1{
        let update_row2 = diesel::update(conversations.filter(user1_id.eq(old_user)))
        .set(user1_id.eq(new_user));
        match update_row2.execute(conn) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    println!("updated!"); 
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    else{
        let update_row2 = diesel::update(conversations.filter(user2_id.eq(old_user)))
        .set(user2_id.eq(new_user));
        match update_row2.execute(conn) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    println!("updated!"); 
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => return false,
    }

}
}
//helper functions to check which user the user is 
fn is_user1(conn: &mut PgConnection, user: &str) -> bool{
    use schema::conversations::dsl::*;
    match conversations.filter(user1_id.eq(user)).first::<Conversations>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}
fn is_user2(conn: &mut PgConnection, user: &str) -> bool{
    use schema::conversations::dsl::*;
    match conversations.filter(user2_id.eq(user)).first::<Conversations>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

//checks if the user has any messages, helper function for updating username

fn check_has_messages(conn: &mut PgConnection, user: &str) -> bool{
    use schema::messages::dsl::*;
    match messages.filter(user_id.eq(user)).first::<Messages>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}

//create_conversation api, takes in a JSON payload with user1 and user2's id and it checks if those two users exist. then a random conversation
//id is generated by using the rand crate in rust 
//if both users exist, a new Conversation object is created and is inserted into the database using insert_conversation. 
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

    let generated_id: String = (0..10).map(|_| char::from(rand::thread_rng().gen_range(32..127))).collect();
    
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
//inserts conversations into diesel database using the uuid input, the user1 and user2 inputs
fn insert_conversation(conn: &mut PgConnection, uuid_input: &str, user1_input: &str, user2_input: &str) {
    use schema::conversations::dsl::*;

    insert_into(conversations)
        .values((id.eq(uuid_input), user1_id.eq(user1_input), user2_id.eq(user2_input)))
        .execute(conn)
        .expect("Error inserting messages");
}
//helper function that is used to search for users and check if they exist
fn search_user(conn: &mut PgConnection, username_input: &str) -> bool {
    use schema::users::dsl::*;

    match users.filter(username.eq(username_input)).first::<Users>(conn)  {
        Ok(_) => true,
        Err(_) => false,
    }
}
//helper function that is used to retrieve username from database using an email input 
fn retrieve_username(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*;
    match users.filter(email.eq(email_input)).select(username).first::<String>(conn)  {
        Ok(username1) => Some(username1), 
        Err(_) => None,
    }
}

//helper function that is used to retrieve email from database using username 
fn retrieve_email(conn: &mut PgConnection, user_input: &str) -> Option<String>{
    use schema::users::dsl::*;
    match users.filter(username.eq(user_input)).select(email).first::<String>(conn)  {
        Ok(username1) => Some(username1), 
        Err(_) => None,
    }
}

//checks if the user is admin or not 
fn retrieve_isadmin(conn: &mut PgConnection, email_input: &str) -> bool{
    use schema::users::dsl::*; 
    match users.filter(email.eq(email_input)).select(is_admin.nullable()).first::<Option<bool>>(conn)  {
        Ok(Some(isadmin_1)) => Some(isadmin_1).is_some(), 
        Ok(None) => false, 
        Err(_) => false,
    }
}

//retrives the first name of the user 
fn retrieve_firstname(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*; 
    match users.filter(email.eq(email_input)).select(first_name).first::<String>(conn)  {
        Ok(firstname_1) => Some(firstname_1), 
        Err(_) => None,
    }
}

//retrieves the last name of the user 
fn retrieve_lastname(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*; 
    match users.filter(email.eq(email_input)).select(last_name).first::<String>(conn)  {
        Ok(lastname_1) => Some(lastname_1), 
        Err(_) => None,
    }
}

//retrieves (hashed) password from database using email  
fn retrieve_password(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*; 
    match users.filter(email.eq(email_input)).select(password).first::<String>(conn)  {
        Ok(password_1) => Some(password_1), 
        Err(_) => None,
    }
}

//retrieves image url from database using email  
fn retrieve_img(conn: &mut PgConnection, email_input: &str) -> Option<String>{
    use schema::users::dsl::*; 
    match users.filter(email.eq(email_input)).select(img_url).first::<String>(conn)  {
        Ok(img_1) => Some(img_1), 
        Err(_) => None,
    }
}

//lists all the conversations based on the username provided 

async fn list_conversations(Json(payload): Json<String>) -> (StatusCode, Json<String>) {
    let connection = &mut establish_connection();
    println!("connection established"); 
    println!("{}", payload); 
    let message_response = show_conversations(connection, &payload);
    println!("show conversations works?"); 
    let response_json = serde_json::to_string(&message_response).expect("Error serializing");

    (StatusCode::OK, Json(response_json))
}

//creates a vector of conversations 
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

//matches password with hashed password 
fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password)
        .unwrap_or(false)
}




//the input to our `create_message` handler
#[derive(Deserialize)]
struct CreateMessage {
    user_id: String,
    message: String,
    conversation_id: String
}

// the output to our `create_message` handler
#[derive(Serialize, Debug)]
struct Message {
    user_id: String,
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
    is_admin: bool, 
}

#[derive(Serialize, Debug)]
struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String, 
    is_admin: bool, 
    img_url: String,
}
#[derive(Deserialize, Debug)]
struct CreateFlagged {
    email: String, 
    reason: String, 
}
#[derive(Serialize, Debug)]
struct FlaggedUser {
    email: String,
    reason: String, 
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

#[derive(Deserialize)]
struct InputEmail{
    email: String, 
    input: String, 
}

#[derive(Deserialize)]
struct NewImage{
    email: String, 
    new_img: String, 
}

#[derive(Deserialize)]
struct UpdateUser{
    email: String, 
    username_old: String, 
    new_username: String, 
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




#[cfg(test)]
mod tests{
    use crate::schema::users::img_url;

    use super::*; 
    #[test]
    fn test_insert_users(){
        let connection = &mut establish_connection();
        let result = insert_user(connection, "user12", "first1", "last1", "email21@gmail.com", "$2b$12$MlheoaYoWveoSqRnq7FhB.tqQaPHguXRiCGuw18IMWOGyhV7yM1ba", false); 
        let result = insert_user(connection, "username122", "first2", "last2", "email22@gmail.com", "$2b$12$MlheoaYoWveoSqRnq7FhB.tqQaPHguXRiCGuw18IMWOGyhV7yM1ba", false); 
        let expected_result = Ok(1); 
        assert_eq!(result, expected_result); 
    }
    //after new users are inserted check to see if default profile images were placed in the user table 
    #[test]
    fn test_profile_image(){
        let connection = &mut establish_connection();
        let result = retrieve_img(connection, "email21@gmail.com"); 
        let expected_result = Some("https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU".to_string()); 
        assert_eq!(result, expected_result); 
    }
    #[test]
    fn test_insert_conversation(){
        let connection = &mut establish_connection();
        let result = insert_conversation(connection, "khkd8sfh", "user12", "username122"); 
        let expected_result = (); 
        assert_eq!(result, expected_result); 
    }
    #[test]
    fn test_insert_message(){
        let connection = &mut establish_connection();
        let result = insert_message(connection, "user12", "hiiiiii","khkd8sfh"); 
        let expected_result = (); 
        assert_eq!(result, expected_result);
    }
    #[test]
    fn test_insert_flagged(){
        let connection = &mut establish_connection();
        let result = insert_flagged_user(connection, "email21@gmail.com", "test_case"); 
        let expected_result = Ok(1); 
        assert_eq!(result, expected_result); 
    }
    //updating username in messages table 
     #[test]
    fn test_update_username_messages(){
        let connection = &mut establish_connection();
        let result = update_username_messages(connection, "user12", "hello11"); 
        let expected_result = true; 
        assert_eq!(result, expected_result); 
    }
    //updating username in conversations table 
    #[test]
    fn test_update_username_conversations(){
        let connection = &mut establish_connection();
        let result = update_username_conversation(connection, "user12", "hello11"); 
        let expected_result = true; 
        assert_eq!(result, expected_result); 
    }
    #[test]
    fn test_delete_user(){
        let connection = &mut establish_connection();
        let result = delete_user(connection, "email21@gmail.com"); 
        let expected_result = Ok(1); 
        assert_eq!(result, expected_result); 
    }
    // macro_rules! aw {
    //     ($e:expr) => {
    //         tokio_test::block_on($e)
    //     };
    // }
   /* 
    #[actix_rt::test]
    async fn test_create_users(){
        let newUser = CreateUser{
            username: "usernmame".to_string(), 
            first_name: "first".to_string(), 
            last_name: "last".to_string(), 
            email: "email@gmail.com".to_string(), 
            password: "passwordpassword".to_string(),
            is_admin: false,
        }; 
        let new_User = User{
            username: "usernmame".to_string(), 
            first_name: "first".to_string(), 
            last_name: "last".to_string(), 
            email: "email@gmail.com".to_string(), 
            password: "passwordpassword".to_string(),
            is_admin: false,
        }; 
        let expected_user_json = serde_json::to_string(&new_User).expect("Failed to serialize user");
        let expected_result = (StatusCode::CREATED, Json(expected_user_json));
        let payload: Json<CreateUser> = serde_json::from_str(&newUser).expect("Failed");
        let result = create_user(payload).await;
        assert_eq!(result, expected_result); 
    }
    
}
   */ 
}