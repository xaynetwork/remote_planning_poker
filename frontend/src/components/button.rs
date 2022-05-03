use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let disabled = props.disabled;
    let onclick = &props.onclick;
    let children = props.children.clone();
    html!(
        <button
            class={classes!(
                "bg-blue-500", "hover:bg-blue-400",
                "text-white", "font-bold",
                "py-2", "px-4", "rounded",
                "border-b-4", "border-blue-700", "hover:border-blue-500",
                disabled.then(||Some("opacity-50")),
                disabled.then(||Some("cursor-not-allowed")),
            )}
            {disabled}
            {onclick}
        >
            {children}
        </button>
    )
}
