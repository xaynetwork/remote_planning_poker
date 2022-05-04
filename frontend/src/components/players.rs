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
    html!(
        <li
            class={classes!(
                "bg-slate-200", "text-slate-500",
                is_current.then(||Some("font-semibold")),
            )}
        >
            {&props.player.user.name}
            if let PlayerRole::Admin = &props.player.role {
                <span>{" (moderator)"}</span>
            }
        </li>
    )
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

    html!(
        <div>
            <h3 class="font-bold">
                {"Connected players:"}
            </h3>
            <ul class="py-4">
                {players}
            </ul>
        </div>
    )
}
