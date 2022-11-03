use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_route;

use crate::components::form_input::FormInput;
use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct LoginProps {
    pub(crate) onsubmit: Callback<String>,
}

#[function_component(Login)]
pub(crate) fn login(props: &LoginProps) -> Html {
    let route: Route = use_route().unwrap_or_default();
    let onkeypress = {
        let onsubmit = props.onsubmit.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                let trim_val = value.trim();

                if !trim_val.is_empty() {
                    input.set_value("");
                    onsubmit.emit(trim_val.to_string());
                }
            }
        })
    };

    html! {
        <section
            class={classes!(
                "h-96", "flex", "flex-col",
                "justify-center", "items-center",
            )}
        >
            if let Route::PokerGame { id: _ } = route {
                <h1 class={classes!("mb-20", "sm:text-3xl", "text-slate-500")}>
                    {"You are about to enter an existing game..."}
                </h1>
            }
            <div class="w-full max-w-sm">
                <FormInput
                    placeholder="What is your name?"
                    label="Please provide your name"
                    {onkeypress}
                />
            </div>
        </section>
    }
}
