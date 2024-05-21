use axum::{routing::get, Router};
use serde_json::Value;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower::ServiceBuilder;
use tracing::info;

use crate::routes::{self, conversations::create_conversation::routes, root::root};

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
    user: String
}

#[derive(serde::Serialize)]
struct MessageOut {
    text: String,
    user: String,
    date: chrono::DateTime<chrono::Utc>
}

//Numerous info! calls to test whether the client side is actually connecting to the socket,
//joining a room, and sending a message
async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Received join: {:?}", room);
    });

    socket.on("message", | socket: SocketRef, Data::<MessageIn>(data)| {
        info!("Received message: {:?}", data);

        let response = MessageOut {
            text: data.text,
            user: data.user,
            date: chrono::Utc::now()
        };
        info!("Outgoing message: {:?}", response.text);
        info!("Socket to go to: {:?}", socket);
        
        socket.emit("message-rec", response);
    })
}

pub fn create_router() -> Router {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let routes_all = Router::new()
        .route("/", get(|| async { "Connection Establish" }))
        //Not actually the root, it's the /hello-world path, should change the name
        .merge(root())
        //Create user from '/create-user' path
        .merge(routes::users::create_user::routes())
        .merge(routes::messages::create_message::routes())
        .merge(routes::users::login::routes())
        .merge(routes::conversations::create_conversation::routes())
        .merge(routes::conversations::list_conversations::routes())
        .merge(routes::messages::list_messages::routes())
        .merge(routes::users::search::routes())
        .layer(
            ServiceBuilder::new()
                //.layer(TraceLayer::new_for_http())
                .layer(layer),
        )
        .layer(CorsLayer::permissive());

    routes_all
}