use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub(crate) onclick: Callback<MouseEvent>,
    pub(crate) children: Children,
}

#[function_component(Button)]
pub(crate) fn button(props: &Props) -> Html {
    let disabled = props.disabled;
    let onclick = &props.onclick;
    let children = props.children.clone();
    html! {
        <button
            class={classes!(
                "bg-blue-500", "hover:bg-blue-400",
                "text-white", "font-bold",
                "py-2", "px-4", "rounded",
                "border-b-4", "border-blue-700", "hover:border-blue-500",
                disabled.then_some("opacity-50"),
                disabled.then_some("cursor-not-allowed"),
            )}
            {disabled}
            {onclick}
        >
            {children}
        </button>
    }
}
