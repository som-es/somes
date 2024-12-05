use std::{net::SocketAddr, ops::ControlFlow};

use axum::{extract::{ws::{Message, WebSocket}, ConnectInfo, WebSocketUpgrade}, response::IntoResponse};
use axum_extra::TypedHeader;

pub async fn ai_chat_ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("`{user_agent}` at {addr:?} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(&socket, msg, who).is_break() {
                return;
            }
        } else {
            log::info!("client {who} abruptly disconnected");
            return;
        }
    }
}


fn process_message(socket:&WebSocket, msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(chat_msg) => {
            // start python script
            // send result 
            ControlFlow::Continue(())
        }
        _ => ControlFlow::Break(())
    }
}