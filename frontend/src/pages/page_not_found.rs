use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::button::Button, Route};

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let history = use_history().unwrap();
    let on_go_back = Callback::once(move |_| history.push(Route::Home));

    html! {
        <section class="rounded-md h-full flex items-center justify-center">
            <div class="p-4 text-center text-slate-500">
                <h4 class="mb-2 text-8xl font-black text-pink-300">{"404"}</h4>
                <h2 class="mb-12 text-3xl font-medium">{"Game not found"}</h2>
                <Button onclick={on_go_back}>{ "Go back to home page" }</Button>
            </div>
        </section>
    }
}
