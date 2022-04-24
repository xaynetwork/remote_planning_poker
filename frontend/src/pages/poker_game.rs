use common::{Game, GameAction, GameId, GameMessage, User};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use std::{ops::Deref, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

struct GameState {
    game: Option<Game>,
    is_loading: bool,
}

impl Default for GameState {
    fn default() -> Self {
        let game = None;
        let is_loading = true;
        Self { game, is_loading }
    }
}

impl Reducible for GameState {
    /// Reducer Action Type
    type Action = GameMessage;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action {
            GameAction::CurrentState(game) => Self {
                game: Some(game),
                is_loading: false,
            }
            .into(),
            _ => {
                if let Some(game) = &self.game {
                    Self {
                        game: Some((*game).clone().reduce(action)),
                        is_loading: false,
                    }
                    .into()
                } else {
                    self
                }
            }
        }
    }
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let state = use_reducer(GameState::default);

    let ws_ref = use_mut_ref(|| {
        let ws = WebSocket::open("ws://localhost:3000/api/game").unwrap();
        log::info!("websocket opened");
        let (ws_write, mut ws_read) = ws.split();

        let state = state.clone();
        spawn_local(async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("[yew] text from websocket: {}", data);

                        if let Ok(action) = serde_json::from_str(&data) {
                            state.dispatch(action);
                        }
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log::debug!("[yew] bytes from websocket: {}", val);
                        }
                    }
                    Err(e) => {
                        log::error!("[yew] ws error: {:?}", e)
                    }
                }
            }
            log::debug!("[yew] WebSocket Closed");
        });

        ws_write
    });

    {
        let user = user.clone();
        let game_id = props.clone().id;
        let ws_ref = ws_ref.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let game_id = GameId(game_id);
                    let user_id = user.id;
                    let action = GameAction::PlayerJoined(user);
                    let msg = GameMessage {
                        action,
                        game_id,
                        user_id,
                    };
                    let action = serde_json::to_string(&msg).unwrap();

                    log::info!("joining: {:?}", action);
                    ws_ref
                        .deref()
                        .borrow_mut()
                        .send(Message::Text(action))
                        .await
                        .unwrap();
                });
                || ()
            },
            (), // dependents
        );
    }

    // let add_one = {
    //     let ws_ref = ws_ref.clone();
    //     Callback::from(move |_| {
    //         let ws_ref = ws_ref.clone();

    //         spawn_local(async move {
    //             let action = CounterAction::AddOne;
    //             let action = serde_json::to_string(&action).unwrap();
    //             ws_ref
    //                 .deref()
    //                 .borrow_mut()
    //                 .send(Message::Text(action))
    //                 .await
    //                 .unwrap();
    //         });
    //     })
    // };

    html! {
        <>
            <header class={classes!("mb-12")}>
                <nav class={classes!("py-4")}>
                    <Link<Route> to={Route::Home}>{ "Go back home" }</Link<Route>>
                </nav>
                <h1 class={classes!("text-3xl")}>
                    {format!("Welcome ")}
                    <strong class={classes!("text-4xl")}>
                        { user.name }
                    </strong>
                </h1>
            </header>
            {
                if state.is_loading {
                    html!{
                        <div class={classes!("p-4", "bg-yellow-200")}>
                            <h2>{"is loading..."}</h2>
                        </div>
                    }
                } else if let Some(game) = &(*state).game {

                    let players = (*game).clone().players.clone().into_iter().map(|(id, player)| {
                        html! {
                            <li key={id.0.to_string()}>
                                {player.user.name}
                            </li>
                        }
                    }).collect::<Html>();

                    html!{
                        <div class={classes!("flex", "bg-white")}>

                            <section class={classes!("flex-1", "p-4", "bg-blue-200")}>
                                <h2>{game.id.0}</h2>
                            </section>

                            <aside class={classes!("flex-initial", "w-80", "p-4", "bg-red-200")}>
                                <h3>{"Players"}</h3>

                                <ul>
                                    {players}
                                </ul>
                            </aside>

                        </div>
                    }
                } else {
                    html!{
                        <div class={classes!("p-4", "bg-yellow-200")}>
                            <h2>{"no game fetched"}</h2>
                        </div>
                    }
                }
            }
        </>
    }
}
