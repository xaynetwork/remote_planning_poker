use common::Vote;
use yew::prelude::*;

use crate::components::allowed_vote_button::AllowedVoteButton;

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct Props {
    pub(crate) on_vote_click: Callback<Vote>,
}

#[function_component(AllowedVotes)]
pub(crate) fn allowed_votes(props: &Props) -> Html {
    let allowed_votes = Vote::get_allowed_votes()
        .iter()
        .map(|vote| {
            let onclick = {
                let vote = *vote;
                let on_vote_click = props.on_vote_click.clone();
                Callback::from(move |_| on_vote_click.emit(vote))
            };
            html! {
                <div key={vote.value()} class="m-1">
                    <AllowedVoteButton vote={*vote} {onclick} />
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
