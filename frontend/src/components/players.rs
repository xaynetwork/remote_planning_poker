use common::Player;
use yew::prelude::*;

use crate::components::player_entry::PlayerEntry;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub players: Vec<Player>,
}

#[function_component(Players)]
pub fn players(props: &Props) -> Html {
    let players = props
        .players
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
