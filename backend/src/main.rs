use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use common::{Game, GameAction, GameId, GameMessage, Player, User};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Our shared state
struct AppState {
    games: Mutex<HashMap<GameId, Game>>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let games = Mutex::new(HashMap::new());
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { games, tx });

    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(false));

    // Compose the routes
    let app = Router::new()
        .route("/api/index", get(get_root))
        .route("/api/create-game", post(create_game))
        .route("/api/game", get(ws_handler).post(create_game))
        .layer(tracing_layer)
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

async fn get_root() -> impl IntoResponse {
    Json("hello from axum")
}

async fn create_game(
    Json(user): Json<User>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let game = Game::new(user);
    let game_id = game.id;
    tracing::debug!("game{:#?}", &game);

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
                    let msg: GameMessage = serde_json::from_str(&data).unwrap();
                    println!("client sent msg: {:#?}", msg);

                    let mut games = state.games.lock().unwrap();

                    if let Some(game) = games.get(&msg.game_id) {
                        println!("OLD game: {:#?}", &game);

                        let msg = msg.clone();
                        let game = (*game).clone();
                        let game = game.reduce(msg);

                        println!("NEW game: {:#?}", &game);

                        games.insert(game.id, game);

                        // if let GameAction::PlayerJoined(_) = msg.action {
                        //     //
                        // }
                    }

                    // let mut counter = state.counter.lock().unwrap();
                    // println!("OLD counter: {:?}", counter.count);
                    // let new_counter = counter.reduce(action);
                    // *counter = new_counter;
                    // println!("NEW counter: {:?}", counter.count);

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
