use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Messages {
    pub id: i32,
    pub user_id: String,
    pub message: String,
    pub conversation_id: String,
}
use diesel::{Queryable, Selectable};
#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub first_name: String, 
    pub last_name: String, 
    pub email: String,
    pub password: String,
    pub img_url: String,
    pub is_admin: Option<bool>,
    //pub logged_in: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversations {
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
}


#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::flagged)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Flagged{
    pub id: i32, 
    pub email: String, 
    pub reason: String, 
}