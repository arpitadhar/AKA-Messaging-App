//use diesel::pg::PgConnection;
//use diesel::prelude::*;
//use dotenvy::dotenv;
//use std::env;
//use diesel::insert_into;
//
//
//pub fn establish_connection() -> PConnection {
//    dotenv().ok();
//
//    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//    PgConnection::establish(&database_url)
//        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
//}
//
//fn insert_message(conn: &mut PgConnection, username: &str, message_text: &str) -> QueryResult<usize> {
//    use schema::messages::dsl::*;
//
//    insert_into(messages)
//        .values((user_id.eq(username), message.eq(message_text)))
//        .execute(conn)
//}
//