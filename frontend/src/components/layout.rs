use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div
            class={classes!(
                "min-h-screen",
                "bg-slate-300",
                "flex", "flex-col"
            )}
        >
            <main class={classes!("p-4", "bg-slate-200", "flex-1")} style="flex-basis: 0;">
                { props.children.clone() }
            </main>
            <footer class={classes!("p-4", "text-center")}>
                <span class={classes!("text-xs", "text-slate-600")}>
                    {"Copyleft by Xayn"}
                </span>
            </footer>
        </div>
    }
}
