use std::ops::ControlFlow;

use axum::{
    extract::{
        ws::{Message, WebSocket}, WebSocketUpgrade,
    },
    response::IntoResponse,
};
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
    let res = ws.on_upgrade(move |socket| handle_socket(socket));

    log::info!("returning after upgrade");

    res
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
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
            socket.send(Message::Text(chat_msg)).await.unwrap();
            // start python script
            // send result
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> sent close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        _ => (),
    }
    ControlFlow::Continue(())
}
