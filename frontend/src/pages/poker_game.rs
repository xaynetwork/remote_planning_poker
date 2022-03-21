use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    use_effect_with_deps(
        move |props| {
            // send "joined game" event to
            // if game exists add user as team member to game room
            // if game doesn't exists create a game room and make user owner of it
            log::info!("game: {}", props.id);
            || ()
        },
        props.clone(),
    );

    html! {
        <section>
            <h1>{format!("Welcome to Game: {}", props.id)}</h1>
            <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
        </section>
    }
}
