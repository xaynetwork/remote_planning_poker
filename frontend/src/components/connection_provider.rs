use common::{AppEvent, GameAction, GameId, User};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::{
    use_async, use_location, use_web_socket_with_options, UseAsyncHandle, UseWebSocketOptions,
    UseWebSocketReadyState,
};

// use dotenv_codegen::dotenv;
// const API_BASE_URL: &str = dotenv!("API_BASE_URL");

#[derive(Clone, PartialEq)]
pub struct Connection {
    pub ready_state: UseWebSocketReadyState,
    pub message: Option<String>,
    pub send: Callback<GameAction>,
}

pub fn use_game_connection(game_id: &GameId, user: &User) -> Connection {
    let location = use_location();
    let base_url = if &location.hostname == "localhost" {
        "localhost:3000"
    } else {
        &location.hostname
    };
    let protocol = &location.protocol.replace("http", "ws");
    let ws_url = [protocol, "//", base_url, "/api/game/", &game_id.to_string()].concat();
    let ws = use_web_socket_with_options(
        ws_url.clone(),
        UseWebSocketOptions {
            reconnect_limit: Some(1000),
            ..Default::default()
        },
    );

    let send_msg = {
        let ws = ws.clone();
        let user_id = user.id.clone();
        move |action: GameAction| {
            let msg = AppEvent::GameMessage(user_id, action);
            let msg = serde_json::to_string(&msg).unwrap();
            ws.send(msg);
        }
    };

    {
        let ws = ws.clone();
        let ws_state = ws.ready_state.clone();
        let send_msg = send_msg.clone();
        let user = user.clone();
        // Send `join` message when the connection opens
        use_effect_with_deps(
            move |ws_state| {
                match **ws_state {
                    UseWebSocketReadyState::Open => {
                        send_msg(GameAction::PlayerJoined(user));
                    }
                    UseWebSocketReadyState::Closed => {
                        // wait some time and reconnect
                        // ws.open();
                    }
                    UseWebSocketReadyState::Closing | UseWebSocketReadyState::Connecting => (),
                }
                || ()
            },
            ws_state, // dependents
        );
    }

    Connection {
        ready_state: (*ws.ready_state).clone(),
        message: (*ws.message).clone(),
        send: Callback::from(send_msg),
    }
}

pub fn use_crate_game_req(user: &User) -> UseAsyncHandle<GameId, Error> {
    let user = user.clone();
    use_async(async move { create_game_req(&user).await })
}

async fn create_game_req(user: &User) -> Result<GameId, Error> {
    let response = Request::post("/api/game").json(user).unwrap().send().await;

    if let Ok(data) = response {
        if let Ok(game_id) = data.json::<GameId>().await {
            Ok(game_id)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
    DeserializeError,
}
