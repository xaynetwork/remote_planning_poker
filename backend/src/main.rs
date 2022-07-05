use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, Path, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::routing::SpaRouter;
use common::{AppMessage, Game, GameAction, GameId, User, UserId};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};
use tokio::sync::broadcast;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// Our shared state
struct AppState {
    games: Mutex<HashMap<GameId, Game>>,
    channels: RwLock<HashMap<GameId, broadcast::Sender<String>>>,
    // tx: broadcas::Sender<String>,
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
    let channels = RwLock::new(HashMap::new());
    // let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        games,
        channels,
        // tx,
    });

    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(false));

    let spa = SpaRouter::new("/assets", "dist");

    // Compose the routes
    let app = Router::new()
        .merge(spa)
        .route("/api/game", post(create_game))
        .route("/api/game/:game_id", get(ws_handler))
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

async fn create_game(
    Json(user): Json<User>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let game = Game::new(user);
    let (tx, _rx) = broadcast::channel(100);

    state.channels.write().unwrap().insert(game.id, tx);
    state.games.lock().unwrap().insert(game.id, game.clone());

    (StatusCode::CREATED, Json(game.id))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
) -> Response {
    let game_id = GameId::new(id);
    ws.on_upgrade(move |socket| handle_socket(socket, state, game_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, game_id: GameId) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    let response = if let Some(game) = state.games.lock().unwrap().get(&game_id) {
        AppMessage::CurrentState(game.clone())
    } else {
        AppMessage::GameNotFound(game_id)
    };

    // send the current state to the player who joined
    let response = serde_json::to_string(&response).unwrap();
    let _ = sender.send(Message::Text(response)).await;

    let channels = state.channels.read().unwrap();
    let tx = if let Some(tx) = channels.get(&game_id) {
        tx
    } else {
        return;
    };

    let mut rx = tx.clone().subscribe();
    let mut _send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = tx.clone();
    let state = state.clone();

    let mut _recv_task = tokio::spawn(async move {
        // id of the user for PlayerLeft action
        let mut player_id: Option<UserId> = None;

        // TODO: maybe do some logging here for None, Err, other Message cases
        while let Some(Ok(Message::Text(data))) = receiver.next().await {
            if let AppMessage::GameMessage(user_id, action) = serde_json::from_str(&data).unwrap() {
                if let GameAction::PlayerJoined(user) = action.clone() {
                    player_id = Some(user.id);
                    tracing::info!("player_id set to: {:?}", user.id);
                }

                update_state_on_message(&state, user_id, game_id, action);
                // send the message to every subscriber
                tx.send(data).unwrap();
            }
        }
        //  send a message that the player disconnected to others
        if let Some(user_id) = player_id {
            update_state_on_message(&state, user_id, game_id, GameAction::PlayerLeft);
            let msg = AppMessage::GameMessage(user_id, GameAction::PlayerLeft);
            let msg = serde_json::to_string(&msg).unwrap();
            tx.send(msg).unwrap();
        } else {
            tracing::warn!("PlayerLeft message wasn't set");
        }
    });

    // If any one of the tasks exit, abort the other.
    // tokio::select! {
    //     _ = (&mut send_task) => recv_task.abort(),
    //     _ = (&mut recv_task) => send_task.abort(),
    // };
}

// update our "global" copy of state
fn update_state_on_message(state: &AppState, user_id: UserId, game_id: GameId, action: GameAction) {
    let mut games = state.games.lock().unwrap();
    if let Some(game) = games.get(&game_id) {
        let game = (*game).clone().reduce(user_id, action);
        games.insert(game.id, game);
    } else {
        tracing::warn!("trying to update game that doesn't exists");
    }
}
