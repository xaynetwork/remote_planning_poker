use common::{Game, GameId, GameMessage, Player, User};
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
    game: Game,
}

impl Reducible for GameState {
    /// Reducer Action Type
    type Action = GameMessage;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let game = self.game.clone().reduce(action);
        Self { game }.into()
    }
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let counter = use_reducer(|| {
        let game = Game::new(user.clone());
        GameState { game }
    });

    let ws_ref = use_mut_ref(|| {
        let ws = WebSocket::open("ws://localhost:3000/game").unwrap();
        log::info!("websocket opened");
        let (ws_write, mut ws_read) = ws.split();

        let counter = counter.clone();
        spawn_local(async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("[yew] text from websocket: {}", data);

                        if let Ok(action) = serde_json::from_str(&data) {
                            counter.dispatch(action);
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
        <section>
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

            // <div class={classes!("flex", "shadow-md", "bg-white")}>
            //     <button class={classes!("py-2", "px-4", "bg-red-200")} onclick={subtract_one}>{ "-1" }</button>
            //     <div class={classes!("p-2", "flex-1", "text-2xl", "text-center")}>{ counter.state.count }</div>
            //     <button class={classes!("py-2", "px-4","bg-green-200")} onclick={add_one}>{ "+1" }</button>
            // </div>
        </section>
    }
}
