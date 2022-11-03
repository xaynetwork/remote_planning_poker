use yew::prelude::*;
use yew_hooks::use_location;

#[derive(Properties, PartialEq)]
pub(crate) struct LayoutProps {
    pub(crate) children: Children,
}

#[function_component(Layout)]
pub(crate) fn layout(props: &LayoutProps) -> Html {
    let location = use_location();
    let is_localhost = location.hostname == "localhost";
    let assets_path = if is_localhost { "/" } else { "/assets/" };
    let src = [assets_path, "xayn-logo-beta.svg"].concat();
    html! {
        <div class="min-h-screen bg-slate-200 flex flex-col">
            <main class="p-4 flex-1" style="flex-basis: 0;">
                { props.children.clone() }
            </main>
            <footer class="p-6 flex justify-end">
                <img
                    {src}
                    alt="Xayn Logo"
                    class="max-h-8"
                />
            </footer>
        </div>
    }
}
