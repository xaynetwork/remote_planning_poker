use common::{Player, Vote};

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub(crate) struct Props {
    pub(crate) vote: Option<Vote>,
    pub(crate) player: Player,
    pub(crate) is_revealed: bool,
}

#[function_component(CastedVoteEntry)]
pub(crate) fn casted_vote_entry(props: &Props) -> Html {
    let is_not_revealed = !props.is_revealed;
    html! {
        <li class={classes!("m-2", "pt-10", "text-center")}>
            <div
                class={classes!(
                    "h-28", "w-20",
                    "flex", "items-center", "justify-center",
                    "text-center", "font-light", "text-slate-500", "text-4xl",
                    "shadow-md", "rounded-md",
                    "transition-transform",
                    props.vote.is_none().then_some("translate-y-0"),
                    props.vote.is_some().then_some("-translate-y-10"),
                    is_not_revealed.then_some("bg-slate-300"),
                    props.is_revealed.then_some("bg-slate-50"),
                )}
            >
                if props.is_revealed && props.vote.is_some() {
                    <strong class="block">
                        { props.vote.unwrap().value() }
                    </strong>
                }
            </div>
            <span
                class={classes!(
                    "block", "w-20", "p-2",
                    "text-xs", "font-bold", "text-slate-500"
                )}
            >
                { &props.player.user.name }
            </span>
        </li>
    }
}
