use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_route;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct LoginProps {
    pub onsubmit: Callback<String>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
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
                    onsubmit.emit(trim_val.to_string())
                }
            }
        })
    };

    html! {
        <section
            class={classes!(
                "h-screen", "p-4",
                "flex", "justify-center", "items-center", "flex-col",
                "bg-slate-200"
            )}
        >
            if let Route::PokerGame { id: _ } = route {
                <h1
                    class={classes!(
                        "px-3", "mb-20",
                        "sm:text-3xl",
                        "text-slate-500",
                    )}
                >
                    {"You are about to enter an existing session..."}
                </h1>
            }
            <div class={classes!("w-full", "max-w-xs")}>
                <label
                    for="name"
                    class={classes!(
                        "block",
                        "px-3", "py-2",
                        "sm:text-sm", "text-slate-500",
                    )}
                >
                    {"Please provide your name"}
                </label>
                <input
                    class={classes!(
                        "block",
                        "w-full",
                        "px-3", "py-2",
                        "sm:text-sm",
                        "text-slate-500",
                        "rounded-md",
                        "shadow-sm", "shadow-slate-300",
                        "outline-none",
                        "focus:shadow-md",
                    )}
                    name="name"
                    placeholder="What is your name?"
                    {onkeypress}
                />
            </div>
        </section>
    }
}
