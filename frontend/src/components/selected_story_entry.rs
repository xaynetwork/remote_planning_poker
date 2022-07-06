use common::{GameAction, Player, PlayerRole, SelectedStory, UserId, Vote};
use indexmap::IndexMap;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    allowed_votes::AllowedVotes, button::Button, casted_vote_entry::CastedVoteEntry,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub story: SelectedStory,
    pub user_id: UserId,
    pub players: IndexMap<UserId, Player>,
    pub on_action: Callback<GameAction>,
}

#[function_component(SelectedStoryEntry)]
pub fn selected_story_entry(props: &Props) -> Html {
    let final_estimate_handle = use_state(|| 0_i32);
    let votes = props
        .players
        .values()
        .filter(|player| player.active)
        .map(|player| {
            let key = player.user.id.to_string();
            let vote = props.story.votes.get(&player.user.id).cloned();
            let is_revealed =
                (props.story.votes_revealed || props.user_id == player.user.id) && vote.is_some();
            let player = player.clone();
            html! {
                <CastedVoteEntry {key} {player} {is_revealed} {vote} />
            }
        })
        .collect::<Html>();

    let on_vote_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |vote| on_action.emit(GameAction::VoteCasted(vote)))
    };

    let on_accept_round = {
        let on_action = props.on_action.clone();
        let estimate = Some(Vote::new(*final_estimate_handle).unwrap());
        Callback::from(move |_| on_action.emit(GameAction::ResultsApproved(estimate)))
    };

    let on_play_again = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotesCleared))
    };

    let on_reveal_cards = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotesRevealed))
    };

    let on_cancel_round = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotingClosed))
    };

    let is_admin = match props.players.get(&props.user_id) {
        Some(player) if player.role == PlayerRole::Admin => true,
        _ => false,
    };

    let can_accept = props.story.can_accept();
    let can_play_again = props.story.can_play_again();
    let can_reveal = props.story.can_reveal();
    let avrg = &props.story.votes_avrg();
    let closest = Vote::get_closest_vote(avrg).value();

    {
        let closest = closest.clone();
        let final_estimate_handle = final_estimate_handle.clone();
        use_effect_with_deps(
            move |closest| {
                final_estimate_handle.set(*closest);
                || ()
            },
            closest,
        );
    };

    let options = Vote::get_allowed_votes()
        .iter()
        .map(|vote| {
            let value = vote.value();
            html! {
                <option
                    key={value}
                    value={value.to_string()}
                    selected={ value == *final_estimate_handle }
                >
                    {value}
                </option>
            }
        })
        .collect::<Html>();

    let onchange = {
        let final_estimate_handle = final_estimate_handle.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let value: i32 = value.parse().unwrap();
            final_estimate_handle.set(value);
        })
    };

    html! {
        <div class={classes!("mt-2", "mb-4")}>
            <h4 class={classes!("font-bold", "text-2xl", "text-slate-500")}>
                {&props.story.info.title}
            </h4>

            <ul class={classes!("list-none", "my-4", "flex", "flex-wrap")}>
                { votes }
            </ul>

            <AllowedVotes {on_vote_click} />

            if is_admin {
                <>
                    if can_accept {
                        <div class="m-2 flex items-center text-slate-500">
                            <h5 class="text-sm mr-4">
                                <span class="mr-2">
                                    {"Average: "}
                                    <b>{avrg}</b>
                                </span>
                                <span class="mr-2">
                                    {"Closest: "}
                                    <b>{closest}</b>
                                </span>
                                <span class="mr-2">
                                    {"Final: "}
                                    <b>{*final_estimate_handle}</b>
                                </span>
                            </h5>
                            <select class="py-1 px-2 text-sm bg-white rounded-sm shadow-sm" {onchange}>
                                {options}
                            </select>
                        </div>
                    }
                    <div class={classes!("list-none", "mt-6", "mb-12", "flex", "flex-wrap")}>
                        <div class="m-1">
                            <Button disabled={!can_accept} onclick={on_accept_round}>{ "Accept round" }</Button>
                        </div>
                        <div class="m-1">
                            <Button disabled={!can_play_again} onclick={on_play_again}>{ "Play again" }</Button>
                        </div>
                        <div class="m-1">
                            <Button disabled={!can_reveal} onclick={on_reveal_cards}>{ "Reveal cards" }</Button>
                        </div>
                        <div class="m-1">
                            <Button onclick={on_cancel_round}>{ "Cancel round" }</Button>
                        </div>
                    </div>
                </>
            }
        </div>
    }
}
