use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct NicknameInputProps {
    pub onsubmit: Callback<String>,
}

#[function_component(NicknameInput)]
pub fn nickname_input(props: &NicknameInputProps) -> Html {
    let onchange = {
        let onsubmit = props.onsubmit.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let trim_val = value.trim();

            if !trim_val.is_empty() {
                input.set_value("");
                onsubmit.emit(trim_val.to_string())
            }
        })
    };

    html! {
        <input {onchange} placeholder="What is your name?" />
    }
}
