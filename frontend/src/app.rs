use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::user_provider::UserProvider,
    pages::{home::Home, page_not_found::PageNotFound, poker_game::PokerGame},
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game/:id")]
    PokerGame { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::PokerGame { id } => {
            html! { <PokerGame id={id} /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserProvider>
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        </UserProvider>
    }
}
