use common::{Game, GameAction, GameId, GameMessage, PlayerRole, User};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use std::{ops::Deref, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

enum GameState {
    Loading,
    Playing(Game),
    NotFound,
}

impl Reducible for GameState {
    /// Reducer Action Type
    type Action = GameMessage;

    /// Reducer Function
    fn reduce(self: Rc<Self>, message: Self::Action) -> Rc<Self> {
        match self.deref() {
            GameState::Loading => match message.action {
                GameAction::CurrentState(game) => GameState::Playing(game),
                GameAction::GameNotFound(_) => GameState::NotFound,
                // TODO: this shouldn't happen, so figure out how to handle it
                _ => GameState::Loading,
            },
            GameState::Playing(game) => {
                let game = game.clone().reduce(message);
                GameState::Playing(game)
            }
            GameState::NotFound => GameState::NotFound,
        }
        .into()
    }
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let state = use_reducer(|| GameState::Loading);

    let ws_ref = use_mut_ref(|| {
        let ws = WebSocket::open("ws://localhost:3000/api/game").unwrap();
        let (ws_write, mut ws_read) = ws.split();

        let state = state.clone();
        spawn_local(async move {
            while let Some(Ok(Message::Text(data))) = ws_read.next().await {
                if let Ok(action) = serde_json::from_str(&data) {
                    state.dispatch(action);
                }
            }
        });

        ws_write
    });

    {
        let user = user.clone();
        let id = props.clone().id;
        // let ws_ref = ws_ref.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let msg = GameMessage {
                        user_id: user.id,
                        game_id: GameId::new(id),
                        action: GameAction::PlayerJoined(user),
                    };
                    let action = serde_json::to_string(&msg).unwrap();

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

    match state.deref() {
        GameState::Loading => html! {
            <div class={classes!("p-4", "bg-yellow-200")}>
                <h2>{"Joining game..."}</h2>
            </div>
        },
        GameState::NotFound => html! {
            <div class={classes!("p-4", "bg-red-200")}>
                <h2>{"Game not found"}</h2>
            </div>
        },
        GameState::Playing(game) => {
            let stories = game
                .clone()
                .stories
                .into_iter()
                .map(|(id, story)| {
                    html! {
                        <article key={id.to_string()}>
                            <h3>{format!("Story: {}", &story.info.title)}</h3>
                        </article>
                    }
                })
                .collect::<Html>();

            let players = game
                .clone()
                .players
                .into_iter()
                .filter(|(_, player)| player.active)
                .map(|(id, player)| {
                    html! {
                        <li key={id.to_string()}>
                            {&player.user.name}

                            if let PlayerRole::Admin = player.role {
                                <span>{" (admin)"}</span>
                            }

                        </li>
                    }
                })
                .collect::<Html>();

            html! {
                <div class={classes!("flex", "bg-white")}>
                    <section class={classes!("flex-1", "p-4", "bg-blue-200")}>

                        <h2>{game.id}</h2>

                        { stories }

                    </section>
                    <aside class={classes!("flex-initial", "w-80", "p-4", "bg-red-200")}>

                        <h3>{"Connected players:"}</h3>
                        <ul>
                            {players}
                        </ul>

                    </aside>
                </div>
            }
        }
    }
}
