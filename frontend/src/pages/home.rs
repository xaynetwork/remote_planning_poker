use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| {
        let id = Uuid::new_v4();
        history.push(Route::PokerGame { id })
    });

    html! {
        <section>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Generate new game" }</button>
        </section>
    }
}
