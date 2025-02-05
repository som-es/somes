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
use sqlx::PgPool;
use tokio::{sync::RwLock, time::Instant};

use crate::{
    jwt::{Claims, KEYS},
    AuthError, PgPoolConnection, RedisConnection,
};

pub async fn join_quiz_room(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    PgPoolConnection(pg): PgPoolConnection,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("user agent: {user_agent:?}");
    let res = ws.on_upgrade(move |socket| handle_socket(socket, pg));

    log::info!("returning after upgrade");

    res
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Ready,
    Question((QuizQuestionNoCorrection, Instant)),
    Removed,
    End
}

static USER_MAP: LazyLock<Arc<RwLock<HashMap<ConnectedUser, u32>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));
// static INFORM_USERS: LazyLock<Arc<RwLock<Vec<Box<dyn Fn() -> BoxFuture<'static, ()>>>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static QUESTION: LazyLock<Arc<RwLock<State>>> =
    LazyLock::new(|| Arc::new(RwLock::new(State::Ready)));
// static QUESTION_TX_RX: LazyLock<(Sender<QuizQuestion>, Receiver<QuizQuestion>)> = LazyLock::new(|| tokio::sync::broadcast::channel(2048));

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectedUser {
    name: String,
    token: String,
    is_admin: bool,
}

async fn handle_socket(mut socket: WebSocket, pg: PgPool) {
    let user = Arc::new(RwLock::new(None));

    let (tx_to_send, mut rx_to_send) = tokio::sync::mpsc::channel(16);

    let (mut sender, mut receiver) = socket.split();

    let tx_to_send_in_recv = tx_to_send.clone();
    let mut recv_task = tokio::spawn(async move {
        let mut db_questions;
        let mut questions = None;
        // let mut questions = questions.iter();
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_message(
                    tx_to_send_in_recv.clone(),
                    msg,
                    user.clone(),
                    questions.as_mut(),
                )
                .await
                .is_break()
                {
                    return;
                }
            } else {
                return;
            }
            if questions.is_none()
                && user
                    .read()
                    .await
                    .as_ref()
                    .map(|x: &ConnectedUser| x.is_admin)
                    .unwrap_or_default()
            {
                db_questions = sqlx::query_as!(QuizQuestionNoCorrection, "select question, answer1, answer2, answer3, answer4 from quiz_questions where quiz_id = 1").fetch_all(&pg).await.unwrap_or_default();
                questions = Some(db_questions.iter());
            }
        }
    });

    let mut question_send_task = tokio::spawn(async move {
        let mut last_question = State::Ready;
        loop {
            let question = QUESTION.read().await;
            if &*question == &State::End {
                // send close message ?

                // add other mechanism to close !
                // break;
            }

            if *question != last_question {
                last_question = (*question).clone();
                let question = match &*question {
                    State::Question((q, _)) => Some(q),
                    _ => {None}
                };
                tx_to_send
                    .clone()
                    .send(Message::Text(
                        serde_json::to_string(&question).unwrap_or_default(),
                    ))
                    .await
                    .unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
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
    iter: Option<&mut std::slice::Iter<'_, QuizQuestionNoCorrection>>,
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
                    // if *QUE
                }

                // request scoreboard
                b's' => if user.read().await.as_ref().unwrap().is_admin {},

                // remove question
                b'r' => if user.read().await.as_ref().unwrap().is_admin {
                    *QUESTION.write().await = State::Removed;
                },

                // next question
                b'n' => {
                    if user.read().await.as_ref().unwrap().is_admin {
                        let next_state = match iter.unwrap().next().cloned() {
                            Some(question) => {
                                State::Question((question, Instant::now()))
                            }
                            None => {
                                State::End
                            }
                        };
                        *QUESTION.write().await = next_state;
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
