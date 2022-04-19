use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <main
            class={classes!(
                "p-4",
                "min-h-screen",
                "bg-slate-200",
            )}
        >
            { props.children.clone() }
        </main>
    }
}
