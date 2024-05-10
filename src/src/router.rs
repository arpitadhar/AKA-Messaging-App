use axum::{routing::get, Router};
use serde_json::Value;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower::ServiceBuilder;
use tracing::info;

use crate::routes::{self, root::root};

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Recieved message: {:?}", data);
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
        .layer(
            ServiceBuilder::new()
                //.layer(TraceLayer::new_for_http())
                .layer(layer),
        )
        .layer(CorsLayer::permissive());

    routes_all
}