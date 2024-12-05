use std::{net::SocketAddr, ops::ControlFlow};

use axum::{extract::{ws::{Message, WebSocket}, ConnectInfo, WebSocketUpgrade}, response::IntoResponse};
use axum_extra::TypedHeader;

pub async fn ai_chat_ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("user agent: {user_agent:?}");
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            log::info!("recv msg: {msg:?}");
            if process_message(&mut socket, msg).await.is_break() {
                return;
            }
        } else {
            return;
        }
    }
}


async fn process_message(socket: &mut WebSocket, msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(chat_msg) => {
            log::info!("chat msg: {chat_msg}");
            // start python script
            // send result 
            ControlFlow::Continue(())
        }
        Message::Ping(data) => {
            socket.send(Message::Pong(data)).await.unwrap();
            ControlFlow::Continue(())
        }, // send pong 
        _ => ControlFlow::Break(())
    }
}