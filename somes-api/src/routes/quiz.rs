mod add_quiz;
use std::{
    cell::Cell,
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
use rand::Rng;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use serde::{Deserialize, Serialize};
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
    Question((QuizQuestion, Instant)),
    Removed,
    Scoreboard,
    End,
}

static USER_MAP: LazyLock<Arc<RwLock<HashMap<(String, u64), f64>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

static SCORE_BOARD: LazyLock<Arc<RwLock<Vec<((String, u64), f64)>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
// static INFORM_USERS: LazyLock<Arc<RwLock<Vec<Box<dyn Fn() -> BoxFuture<'static, ()>>>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static QUESTION: LazyLock<Arc<RwLock<State>>> =
    LazyLock::new(|| Arc::new(RwLock::new(State::Ready)));

static ANSWERS_TO_QUESTION: LazyLock<Arc<RwLock<usize>>> =
    LazyLock::new(|| Arc::new(RwLock::new(0)));

// static QUESTION_TX_RX: LazyLock<(Sender<QuizQuestion>, Receiver<QuizQuestion>)> = LazyLock::new(|| tokio::sync::broadcast::channel(2048));

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConnectedUser {
    name: String,
    id: u64,
    token: String,
    is_admin: bool,
    answer_locked_in: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ScoreInfo {
    pub score: f64,
    pub place: i32,
    pub correct_answer: i32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Scorer {
    pub name: String,
    pub id: u64,
    pub score: f64,
}

async fn handle_socket(mut socket: WebSocket, pg: PgPool) {
    let user = Arc::new(RwLock::new(None));

    let (tx_to_send, mut rx_to_send) = tokio::sync::mpsc::channel(16);

    let (mut sender, mut receiver) = socket.split();

    let tx_to_send_in_recv = tx_to_send.clone();
    let recv_user = user.clone();
    let mut recv_task = tokio::spawn(async move {
        let mut db_questions;
        let mut questions = None;
        // let mut questions = questions.iter();
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_message(
                    tx_to_send_in_recv.clone(),
                    msg,
                    recv_user.clone(),
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
                && recv_user
                    .read()
                    .await
                    .as_ref()
                    .map(|x: &ConnectedUser| x.is_admin)
                    .unwrap_or_default()
            {
                db_questions = sqlx::query_as!(QuizQuestion, "select question, answer1, answer2, answer3, answer4, correct_answer from quiz_questions where quiz_id = 4").fetch_all(&pg).await.unwrap_or_default();
                log::info!("{db_questions:?}");
                questions = Some(db_questions.iter());
            }
        }
    });

    let question_user = user.clone();
    let mut question_send_task = tokio::spawn(async move {
        let mut last_state = State::Ready;
        let mut last_correct_answer = 0;
        loop {
            let question = QUESTION.read().await;
            if &*question == &State::End {
                // send close message ?

                // add other mechanism to close !
                // break;
            }

            if *question != last_state {
                last_state = (*question).clone();
                let question = match last_state.clone() {
                    State::Question((q, _)) => {
                        last_correct_answer = q.correct_answer;
                        Some(QuizQuestionNoCorrection {
                            question: q.question,
                            answer1: q.answer1,
                            answer2: q.answer2,
                            answer3: q.answer3,
                            answer4: q.answer4,
                        })
                    }
                    _ => None,
                };
                tx_to_send
                    .clone()
                    .send(Message::Text(
                        serde_json::to_string(&question).unwrap_or_default(),
                    ))
                    .await
                    .unwrap();

                if last_state == State::Scoreboard {
                    question_user
                        .write()
                        .await
                        .as_mut()
                        .map(|user| user.answer_locked_in = false);
                    if let Some(user) = &*question_user.clone().read().await {
                        let scoreboard = SCORE_BOARD.read().await;
                        let idx = scoreboard
                            .iter()
                            .position(|((_, id), _)| id == &user.id)
                            .map(|idx| idx as i32)
                            .unwrap_or(-1);

                        let score = if idx >= 0 {
                            let ((_name, _id), score) = &scoreboard[idx as usize];
                            *score
                        } else {
                            0.
                        };
                        tx_to_send
                            .send(Message::Text(
                                serde_json::to_string(&ScoreInfo {
                                    score,
                                    place: (idx + 1),
                                    correct_answer: last_correct_answer,
                                })
                                .unwrap(),
                            ))
                            .await
                            .unwrap();
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        }
        // while QUESTION_TX_RX.1.recv()
    });

    // let mut score_send_task = tokio::spawn(async move {

    //     tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    // });

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
            // question_send_task.abort();
        }
        rv_a = (&mut send_task) => {
            // send_task.abort();
        },
        rv_b = (&mut recv_task) => {
            // recv_task.abort();
        }
    }

    if let Some(user) = &*user.read().await {
        USER_MAP.write().await.remove(&(user.name.clone(), user.id));
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InfoCounts {
    user_count: usize,
    answer_count: usize,
}

async fn process_message(
    sender: tokio::sync::mpsc::Sender<Message>,
    msg: Message,
    user: Arc<RwLock<Option<ConnectedUser>>>,
    iter: Option<&mut std::slice::Iter<'_, QuizQuestion>>,
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

                    let mut rng = rand::rngs::OsRng::default();
                    let id = rng.gen();

                    let new_user = ConnectedUser {
                        name,
                        id,
                        token: "token".to_string(),
                        is_admin: true,
                        answer_locked_in: false,
                    };
                    log::info!("new user: {:?}", new_user);
                    sender
                        .send(Message::Text(format!("{};{}", new_user.name, new_user.id)))
                        .await
                        .unwrap();

                    USER_MAP
                        .write()
                        .await
                        .insert((new_user.name.clone(), new_user.id), 0.);
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
                            id: token_data.claims.id as u64,
                            token: token.to_string(),
                            is_admin: token_data.claims.is_admin,
                            answer_locked_in: false,
                        };
                        *user.write().await = Some(new_user);
                    }
                }

                b'a' => {
                    let nth_answer = chat_msg.chars().nth(1).unwrap().to_digit(10).unwrap();

                    if let State::Question((question, available_since)) = &*QUESTION.read().await {
                        let mut user = user.write().await;
                        if let Some(user) = user.as_mut() {
                            if user.answer_locked_in {
                                return ControlFlow::Continue(());
                            }
                            user.answer_locked_in = true;

                            *ANSWERS_TO_QUESTION.write().await += 1;
                            USER_MAP
                                .write()
                                .await
                                .get_mut(&(user.name.clone(), user.id))
                                .map(|score| {
                                    *score += ((question.correct_answer == nth_answer as i32) as u32
                                        as f64
                                        * 1100.
                                        * (available_since.elapsed().as_secs_f64() * (-1. / 23.))
                                            .exp())
                                    .round()
                                });
                        }
                    }
                }

                // request scoreboard
                b's' => {
                    if user.read().await.as_ref().unwrap().is_admin {
                        // sort scoreboard (afterwards send score)
                        let mut scoreboard = SCORE_BOARD.write().await;
                        *scoreboard = USER_MAP
                            .read()
                            .await
                            .iter()
                            .map(|(a, b)| (a.clone(), *b))
                            .collect::<Vec<_>>();
                        scoreboard.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });

                        let scoreboard = scoreboard
                            .iter()
                            .map(|((name, id), score)| Scorer {
                                name: name.clone(),
                                id: *id,
                                score: *score,
                            })
                            .collect::<Vec<_>>();

                        sender
                            .send(Message::Text(
                                serde_json::to_string(&scoreboard).unwrap_or_default(),
                            ))
                            .await
                            .unwrap();

                        *QUESTION.write().await = State::Scoreboard;
                    }
                }

                // remove question
                b'r' => {
                    if user
                        .read()
                        .await
                        .as_ref()
                        .map(|x| x.is_admin)
                        .unwrap_or_default()
                    {
                        *QUESTION.write().await = State::Removed;
                    }
                }

                b'u' => {
                    if user
                        .read()
                        .await
                        .as_ref()
                        .map(|x| x.is_admin)
                        .unwrap_or_default()
                    {
                        sender
                            .send(Message::Text(
                                serde_json::to_string(&InfoCounts {
                                    user_count: USER_MAP.read().await.len(),
                                    answer_count: *ANSWERS_TO_QUESTION.read().await,
                                })
                                .unwrap_or_default(),
                            ))
                            .await
                            .unwrap();
                    }
                }

                // next question
                b'n' => {
                    if user
                        .read()
                        .await
                        .as_ref()
                        .map(|x| x.is_admin)
                        .unwrap_or_default()
                    {
                        let next_state = match iter.unwrap().next().cloned() {
                            Some(question) => State::Question((question, Instant::now())),
                            None => State::End,
                        };
                        *QUESTION.write().await = next_state;
                        *ANSWERS_TO_QUESTION.write().await = 0;
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
            if let Some(user) = &*user.read().await {
                USER_MAP.write().await.remove(&(user.name.clone(), user.id));
            }
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
