use common::{GameId, User};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let user = use_context::<User>().expect("no user ctx found");

    let onclick = Callback::from(move |_| {
        let history = history.clone();
        let user = user.clone();
        spawn_local(async move {
            let response = Request::post("/api/game")
                .json(&user)
                .unwrap()
                .send()
                .await
                .unwrap();

            if response.ok() {
                let game_id: GameId = response.json().await.unwrap();
                let id = game_id.to_uuid();
                history.push(Route::PokerGame { id })
            }
        });
    });

    html! {
        <section>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Generate new game" }</button>
        </section>
    }
}
