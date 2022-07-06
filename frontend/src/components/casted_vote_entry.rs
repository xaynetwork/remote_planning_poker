use common::{Player, Vote};

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub vote: Vote,
    pub player: Player,
    pub is_revealed: bool,
}

#[function_component(CastedVoteEntry)]
pub fn casted_vote_entry(props: &Props) -> Html {
    let is_not_revealed = !props.is_revealed;
    html! {
        <li
            class={classes!("m-2", "text-center")}
        >
            <div
                class={classes!(
                    "h-28", "w-20",
                    "flex", "items-center", "justify-center",
                    "text-center", "font-light", "text-slate-500", "text-4xl",
                    "shadow-md", "rounded-md",
                    is_not_revealed.then(||Some("bg-slate-300")),
                    props.is_revealed.then(||Some("bg-slate-50")),
                )}
            >
                if props.is_revealed {
                    <strong class={classes!("block")}>
                        { props.vote.value() }
                    </strong>
                }
            </div>
            <span
                class={classes!(
                    "block", "w-20", "p-2",
                    "text-xs", "text-slate-500"
                )}
            >
                { &props.player.user.name }
            </span>
        </li>
    }
}
