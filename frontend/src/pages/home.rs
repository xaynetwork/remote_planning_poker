use common::{GameId, User};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::components::button::Button;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let create_game = use_async(async move { create_game_req(&user).await });
    let onclick = {
        let create_game = create_game.clone();
        Callback::from(move |_| {
            create_game.run();
        })
    };

    html! {
        <section class="flex justify-center items-center h-full">
            <Button disabled={create_game.loading} {onclick}>
                { "Generate new game" }
            </Button>
            if let Some(game_id) = &create_game.data {
                <Redirect<Route> to={Route::PokerGame { id: game_id.to_uuid() }}/>
            }
            {
                if let Some(error) = &create_game.error {
                    let error_msg = match error {
                        Error::DeserializeError => "Couldn't deserialize game id",
                        Error::RequestError => "There was an issue with the request",
                    };
                    html! {
                        <div class="p-4 text-center">
                            <h2 class="mt-12 text-3xl font-medium">{error_msg}</h2>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </section>
    }
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
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
