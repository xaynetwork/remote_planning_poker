use common::{Player, PlayerRole, User};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player: Player,
}

#[function_component(PlayerEntry)]
pub fn player_entry(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let is_current = user.id == props.player.user.id;
    html! {
        <li
            class={classes!(
                "py-2", "px-4", "border-b",
                "text-slate-500",
                is_current.then_some("font-semibold"),
                is_current.then_some("text-lg"),
            )}
        >
            {&props.player.user.name}
            if let PlayerRole::Admin = &props.player.role {
                <span>{" (moderator)"}</span>
            }
        </li>
    }
}
