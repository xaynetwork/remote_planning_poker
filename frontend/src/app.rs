use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{layout::Layout, user_provider::UserProvider},
    pages::{home::Home, page_not_found::PageNotFound, poker_game::PokerGame},
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game/:id")]
    PokerGame { id: Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::PokerGame { id } => {
            html! { <PokerGame id={*id} /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <UserProvider>
                    <Switch<Route> render={Switch::render(switch)} />
                </UserProvider>
            </Layout>
        </BrowserRouter>
    }
}
