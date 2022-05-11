use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="min-h-screen bg-slate-200 flex flex-col">
            <main class="p-4 flex-1" style="flex-basis: 0;">
                { props.children.clone() }
            </main>
            <footer class="p-6 flex justify-end">
                <img
                    src="/assets/xayn-logo-beta.svg"
                    alt="Xayn Logo"
                    class="max-h-8"
                />
            </footer>
        </div>
    }
}
