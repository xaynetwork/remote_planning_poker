use common::User;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::Button;
use crate::components::connection_provider::{use_crate_game_req, Error};
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let create_game = use_crate_game_req(&user);
    let onclick = {
        let create_game = create_game.clone();
        Callback::from(move |_| {
            create_game.run();
        })
    };

    html! {
        <section class="flex justify-center items-center h-full">
            {if let Some(error) = &create_game.error {
                let error_msg = match *error {
                    Error::DeserializeError => "Couldn't deserialize game id",
                    Error::RequestError => "There was an issue with the request",
                };
                html! {
                    <div class="p-4 text-center">
                        <h2 class="mt-12 text-3xl font-medium">{error_msg}</h2>
                    </div>
                }
            } else if let Some(game_id) = &create_game.data {
                html!{
                    <Redirect<Route> to={Route::PokerGame { id: *game_id }}/>
                }
            } else {
                html! {
                    <Button disabled={create_game.loading} {onclick}>
                        { "Generate new game" }
                    </Button>
                }
            }}
        </section>
    }
}
