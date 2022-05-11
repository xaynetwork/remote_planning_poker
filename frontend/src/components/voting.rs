use crate::components::{button::Button, vote_value::VoteValueList};
use common::{GameAction, Player, PlayerRole, Story, StoryStatus, UserId, Vote};
use indexmap::IndexMap;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub story: Story,
    pub user_id: UserId,
    pub players: IndexMap<UserId, Player>,
    pub on_action: Callback<GameAction>,
}

#[function_component(SelectedStory)]
pub fn selected_story(props: &Props) -> Html {
    let final_estimate_handle = use_state(|| 0_i32);
    let votes = props
        .story
        .votes
        .clone()
        .into_iter()
        .map(|(user_id, vote)| {
            if let Some(player) = props.players.get(&user_id) {
                let status = props.story.status.clone();
                let current_user_id = props.user_id.clone();
                let is_revealed = status == StoryStatus::Revealed || user_id == current_user_id;
                let is_not_revealed = !is_revealed;
                html! {
                    <li
                        key={user_id.to_string()}
                        class={classes!("m-2", "text-center")}
                    >
                        <div
                            class={classes!(
                                "h-28", "w-20",
                                "flex", "items-center", "justify-center",
                                "text-center", "font-light", "text-slate-500",
                                "shadow-md", "rounded-md",
                                is_not_revealed.then(||Some("bg-slate-300")),
                                is_revealed.then(||Some("bg-slate-50")),
                            )}
                        >
                            if is_revealed {
                                <strong class={classes!("block")}>
                                    { vote.value() }
                                </strong>
                            }
                        </div>
                        <span class={classes!("block", "p-2", "text-xs")}>
                            { player.user.name.clone() }
                        </span>
                    </li>
                }
            } else {
                // TODO: maybe iterate over players instead?
                html! {}
            }
        })
        .collect::<Html>();

    let on_vote_click = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |vote| on_action.emit(GameAction::VoteCasted(story_id, vote)))
    };

    let on_accept_round = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        let estimate = Some(Vote::new(*final_estimate_handle).unwrap());
        Callback::from(move |_| on_action.emit(GameAction::ResultsApproved(story_id, estimate)))
    };

    let on_play_again = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotingOpened(story_id)))
    };

    let on_reveal_cards = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotesRevealed(story_id)))
    };

    let on_cancel_round = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotingClosed(story_id)))
    };

    let is_admin = match props.players.get(&props.user_id) {
        Some(player) if player.role == PlayerRole::Admin => true,
        _ => false,
    };

    let can_accept = match props.story.status {
        StoryStatus::Revealed => true,
        _ => false,
    };

    let can_play_again = match props.story.status {
        StoryStatus::Revealed => true,
        StoryStatus::Voting if !props.story.votes.is_empty() => true,
        _ => false,
    };

    let can_reveal = match props.story.status {
        StoryStatus::Voting if !props.story.votes.is_empty() => true,
        _ => false,
    };

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

            <VoteValueList {on_vote_click} />

            if is_admin {
                <>
                    if can_accept {
                        <div class="m-2 flex items-center text-slate-500">
                            <h5 class="text-sm mr-4">
                                {format!(
                                    "Average: {}, Closest: {}, Final: {:?}",
                                    avrg, closest, *final_estimate_handle
                                )}
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
