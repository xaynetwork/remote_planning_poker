use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConnectionIndicatorProps {
    pub label: String,
    pub bg_class: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ConnectionIndicator)]
pub fn connection_indicator(props: &ConnectionIndicatorProps) -> Html {
    let onclick = &props.onclick;
    html! {
        <div
            class={classes!(
                "fixed", "top-0", "right-0",
                "py-1", "px-2", "shadow",
                "text-xs", "font-bold", "uppercase", "text-white",
                "cursor-pointer",
                &props.bg_class
            )}
            {onclick}
        >
            {&props.label}
        </div>
    }
}
