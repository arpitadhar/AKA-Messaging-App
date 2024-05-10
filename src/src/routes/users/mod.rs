use::serde::{Serialize, Deserialize};

pub mod create_user;
pub mod login;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    first_name: String, 
    last_name: String, 
    email: String, 
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String
}