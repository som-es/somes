mod add_quiz;
use std::{
    collections::HashMap,
    future::Future,
    ops::ControlFlow,
    sync::{Arc, LazyLock},
};

pub use add_quiz::*;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use fake::{faker::name::de_de::Name, locales::DE_DE, Fake};
use futures::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Validation};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use tokio::sync::RwLock;

use crate::{
    jwt::{Claims, KEYS},
    AuthError, RedisConnection,
};

pub async fn join_quiz_room(
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

static USER_MAP: LazyLock<Arc<RwLock<HashMap<ConnectedUser, u32>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));
// static INFORM_USERS: LazyLock<Arc<RwLock<Vec<Box<dyn Fn() -> BoxFuture<'static, ()>>>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static QUESTION: LazyLock<Arc<RwLock<Option<QuizQuestion>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));
// static QUESTION_TX_RX: LazyLock<(Sender<QuizQuestion>, Receiver<QuizQuestion>)> = LazyLock::new(|| tokio::sync::broadcast::channel(2048));

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectedUser {
    name: String,
    token: String,
    is_admin: bool,
}

async fn handle_socket(mut socket: WebSocket) {
    let user = Arc::new(RwLock::new(None));

    let (tx_to_send, mut rx_to_send) = tokio::sync::mpsc::channel(16);

    let (mut sender, mut receiver) = socket.split();

    let tx_to_send_in_recv = tx_to_send.clone();
    let mut recv_task = tokio::spawn(async move {
        let questions = vec![
            QuizQuestion {
                question: "1".to_string(),
                answer1: "ANSER".to_string(),
                answer2: "ANSER".to_string(),
                answer3: "ANSER".to_string(),
                answer4: "ANSER".to_string(),
            },
            QuizQuestion {
                question: "2".to_string(),
                answer1: "ANSER".to_string(),
                answer2: "ANSER".to_string(),
                answer3: "ANSER".to_string(),
                answer4: "ANSER".to_string(),
            },
            QuizQuestion {
                question: "3".to_string(),
                answer1: "ANSER".to_string(),
                answer2: "ANSER".to_string(),
                answer3: "ANSER".to_string(),
                answer4: "ANSER".to_string(),
            },
        ];
        let mut questions = questions.iter();
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_message(
                    tx_to_send_in_recv.clone(),
                    msg,
                    user.clone(),
                    &mut questions,
                )
                .await
                .is_break()
                {
                    return;
                }
            } else {
                return;
            }
        }
    });

    let mut question_send_task = tokio::spawn(async move {
        let mut last_question = None;
        loop {
            let question = QUESTION.read().await;
            if last_question.is_some() && question.is_none() {
                // send close message ?
                break;
            }

            if *question != last_question {
                last_question = (*question).clone();
                if let Some(question) = question.as_ref() {
                    tx_to_send
                        .clone()
                        .send(Message::Text(
                            serde_json::to_string(question).unwrap_or_default(),
                        ))
                        .await
                        .unwrap();
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        // while QUESTION_TX_RX.1.recv()
    });

    let mut send_task = tokio::spawn(async move {
        while let Some(recv_msg) = rx_to_send.recv().await {
            if let Message::Close(_) = recv_msg {
                break;
            }
            sender.send(recv_msg).await.unwrap();
        }
    });

    tokio::select! {
        rv_c = (&mut question_send_task) => {
            question_send_task.abort();
        }
        rv_a = (&mut send_task) => {
            send_task.abort();
        },
        rv_b = (&mut recv_task) => {
            recv_task.abort();
        }
    }
}

async fn process_message(
    sender: tokio::sync::mpsc::Sender<Message>,
    msg: Message,
    user: Arc<RwLock<Option<ConnectedUser>>>,
    iter: &mut std::slice::Iter<'_, QuizQuestion>,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(chat_msg) => {
            log::info!("chat msg: {chat_msg}");
            match chat_msg.as_bytes()[0] {
                b'b' => {
                    if user.read().await.is_some() {
                        return ControlFlow::Continue(());
                    }
                    let name: String = Name().fake();

                    let new_user = ConnectedUser {
                        name,
                        token: "token".to_string(),
                        is_admin: true,
                    };
                    log::info!("new user: {:?}", new_user);
                    sender
                        .send(Message::Text(format!(
                            "{};{}",
                            new_user.name, new_user.token
                        )))
                        .await
                        .unwrap();

                    *user.write().await = Some(new_user)
                }

                b'h' => {
                    let (_, token) = chat_msg.split_at(1);
                    let token_data =
                        decode::<Claims>(token, &KEYS.decoding, &Validation::default())
                            .map_err(|_| AuthError::InvalidToken);

                    if let Ok(token_data) = token_data {
                        let new_user = ConnectedUser {
                            name: "RANDOM_NAME".to_string(),
                            token: token.to_string(),
                            is_admin: token_data.claims.is_admin,
                        };
                        *user.write().await = Some(new_user);
                    }
                }

                b'a' => {
                    let nth_answer = chat_msg.chars().nth(1).unwrap().to_digit(10).unwrap();
                }

                // request scoreboard
                b'r' => if user.read().await.as_ref().unwrap().is_admin {},

                // next question
                b'n' => {
                    if user.read().await.as_ref().unwrap().is_admin {
                        *QUESTION.write().await = iter.next().cloned();
                        log::info!("question: {:?}", QUESTION.read().await)
                    }
                }

                _ => {}
            }

            // socket.send(Message::Text(chat_msg)).await.unwrap();
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
