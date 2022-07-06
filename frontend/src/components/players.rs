use common::{Player, PlayerRole, User};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EntryProps {
    pub player: Player,
}

#[function_component(PlayerEntry)]
pub fn player_entry(props: &EntryProps) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let is_current = user.id == props.player.user.id;
    html! {
        <li
            class={classes!(
                "py-2", "px-4", "border-b",
                "text-slate-500",
                is_current.then(||Some("font-semibold")),
                is_current.then(||Some("text-lg")),
            )}
        >
            {&props.player.user.name}
            if let PlayerRole::Admin = &props.player.role {
                <span>{" (moderator)"}</span>
            }
        </li>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub players: Vec<Player>,
}

#[function_component(PlayerList)]
pub fn player_list(props: &ListProps) -> Html {
    let players = props
        .players
        .clone()
        .iter()
        .map(|player| {
            let key = player.user.id.to_string();
            let player = player.clone();
            html! {
                <PlayerEntry {key} {player} />
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <h3 class="px-4 font-semibold text-slate-400">
                {"Connected players:"}
            </h3>
            <ul class="my-2 shadow-inner rounded-md list-none bg-slate-300 bg-opacity-50">
                {players}
            </ul>
        </div>
    }
}
