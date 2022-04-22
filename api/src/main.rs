use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use common::{Game, GameId, Player, User};
// use common::{}
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashMap,
    // collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use uuid::Uuid;

// Our shared state
struct AppState {
    games: Mutex<HashMap<GameId, Game>>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let games = Mutex::new(HashMap::new());
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { games, tx });

    // Compose the routes
    let app = Router::new()
        .route("/", get(get_root))
        .route("/game", get(ws_handler).post(create_game))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
        )
        .layer(Extension(app_state));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_game(
    Json(user): Json<User>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let game = Game::new(user);
    let game_id = game.id;
    state.games.lock().unwrap().insert(game.id, game);
    (StatusCode::CREATED, Json(game_id))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    let _send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("will send msg: {}", msg);
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();

    // let count = state.counter.lock().unwrap().count;
    // let action = CounterAction::CurrentState(count);
    // let action = serde_json::to_string(&action).unwrap();
    // tx.send(action).unwrap();

    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(data) => {
                    println!("client sent str: {:?}", data);

                    // if let Ok(action) = serde_json::from_str(&data) {
                    //     let mut counter = state.counter.lock().unwrap();
                    //     println!("OLD counter: {:?}", counter.count);
                    //     let new_counter = counter.reduce(action);
                    //     *counter = new_counter;
                    //     println!("NEW counter: {:?}", counter.count);
                    // }

                    tx.send(data).unwrap();
                }
                Message::Close(_) => {
                    println!("CLOSE client disconnected");
                    return;
                }
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
            }
            // } else if let Err(err) = msg {
            //     println!("ERR client disconnected: {:?}", err);
            //     return;
        };
    }

    println!("EXIT client disconnected");
}

async fn get_root() -> impl IntoResponse {
    Json("hello from axum!".to_string())
}

// type Db = Arc<RwLock<HashMap<Uuid, Game>>>;
