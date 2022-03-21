use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <section>
            <h1>{ "Page not found" }</h1>
            <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
        </section>
    }
}
