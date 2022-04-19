use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct NicknameInputProps {
    pub onsubmit: Callback<String>,
}

#[function_component(NicknameInput)]
pub fn nickname_input(props: &NicknameInputProps) -> Html {
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
    }
}
