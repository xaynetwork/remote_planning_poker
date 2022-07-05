use common::Vote;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EntryProps {
    pub value: i32,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(VoteButton)]
pub fn vote_button(props: &EntryProps) -> Html {
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
            { props.value }
        </button>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub on_vote_click: Callback<Vote>,
}

#[function_component(VoteValueList)]
pub fn vote_value_list(props: &ListProps) -> Html {
    let allowed_votes = Vote::get_allowed_votes()
        .iter()
        .map(|vote| {
            let value = vote.value();
            let onclick = {
                let vote = vote.clone();
                let on_vote_click = props.on_vote_click.clone();
                Callback::from(move |_| on_vote_click.emit(vote))
            };
            html! {
                <div class="m-1">
                    <VoteButton {value} {onclick} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <section
            class={classes!(
                "flex", "flex-wrap", "my-4", "p-4",
                "bg-slate-300", "shadow-inner", "rounded",
            )}
        >
            {allowed_votes}
        </section>
    }
}
