use common::Vote;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub vote: Vote,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(AllowedVoteButton)]
pub fn allowed_vote_button(props: &Props) -> Html {
    let onclick = &props.onclick;
    html! {
        <button
            class={classes!(
                "m-1", "py-2", "w-12",
                "text-center", "font-bold", "text-slate-500",
                "shadow-md", "rounded-md",
                "bg-slate-50", "hover:bg-green-200",
                "cursor-pointer"
            )}
            {onclick}
        >
            { props.vote.value() }
        </button>
    }
}
