use crate::components::{button::Button, vote_value::VoteValueList};
use common::{GameAction, Player, PlayerRole, Story, StoryStatus, UserId};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub story: Story,
    pub user_id: UserId,
    pub players: HashMap<UserId, Player>,
    pub on_action: Callback<GameAction>,
}

#[function_component(SelectedStory)]
pub fn selected_story(props: &Props) -> Html {
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
                html!(
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
                                    { vote.value as u8 }
                                </strong>
                            }
                        </div>
                        <span class={classes!("block", "p-2", "text-xs")}>
                            { player.user.name.clone() }
                        </span>
                    </li>
                )
            } else {
                // TODO: what to do?
                html!()
            }
        })
        .collect::<Html>();

    let on_vote_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |(story_id, vote_value)| {
            on_action.emit(GameAction::VoteCasted(story_id, vote_value))
        })
    };

    let on_accept_round = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::ResultsApproved(story_id)))
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

    html!(
        <div class={classes!("mt-2", "mb-4")}>
            <h4 class={classes!("font-bold", "text-2xl", "text-slate-500")}>
                {&props.story.info.title}
            </h4>

            <ul class={classes!("list-none", "my-4", "flex", "flex-wrap")}>
                { votes }
            </ul>

            <VoteValueList
                story_id={props.story.id}
                {on_vote_click}
            />

            if is_admin {
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
            }
        </div>
    )
}
