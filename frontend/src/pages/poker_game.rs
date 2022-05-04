use common::{
    Game, GameAction, GameId, GameMessage, Player, PlayerRole, Story, StoryId, StoryInfo,
    StoryStatus, User, UserId, VoteValue,
};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use log::debug;
use std::{collections::HashMap, ops::Deref, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{
    backlog::{BacklogStoryEntry, BacklogStoryList},
    button::Button,
    story_form::StoryForm,
    vote_value::VoteValueList,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

enum GameState {
    Loading,
    Playing(Game),
    NotFound,
}

impl Reducible for GameState {
    /// Reducer Action Type
    type Action = GameMessage;

    /// Reducer Function
    fn reduce(self: Rc<Self>, message: Self::Action) -> Rc<Self> {
        match self.deref() {
            GameState::Loading => match message.action {
                GameAction::CurrentState(game) => GameState::Playing(game),
                GameAction::GameNotFound(_) => GameState::NotFound,
                // TODO: this shouldn't happen, so figure out how to handle it
                _ => GameState::Loading,
            },
            GameState::Playing(game) => {
                let game = game.clone().reduce(message);
                GameState::Playing(game)
            }
            GameState::NotFound => GameState::NotFound,
        }
        .into()
    }
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let state = use_reducer(|| GameState::Loading);

    let ws_ref = use_mut_ref(|| {
        let ws = WebSocket::open("ws://localhost:3000/api/game").unwrap();
        let (ws_write, mut ws_read) = ws.split();

        let state = state.clone();
        spawn_local(async move {
            while let Some(Ok(Message::Text(data))) = ws_read.next().await {
                if let Ok(action) = serde_json::from_str(&data) {
                    state.dispatch(action);
                }
            }
        });

        ws_write
    });

    let crate_msg = {
        let user_id = user.id.clone();
        let game_id = GameId::new(props.id.clone());

        move |action: GameAction| GameMessage {
            user_id,
            game_id,
            action,
        }
    };

    let send_msg = {
        let ws_ref = ws_ref.clone();
        move |msg: GameMessage| {
            spawn_local(async move {
                let action = serde_json::to_string(&msg).unwrap();

                ws_ref
                    .deref()
                    .borrow_mut()
                    .send(Message::Text(action))
                    .await
                    .unwrap();
            });
        }
    };

    {
        let user = user.clone();
        let send_msg = send_msg.clone();
        use_effect_with_deps(
            move |_| {
                let msg = crate_msg(GameAction::PlayerJoined(user));
                send_msg(msg);
                || ()
            },
            (), // dependents
        );
    }

    let on_submit = {
        let send_msg = send_msg.clone();
        Callback::from(move |stories: Vec<String>| {
            let send_msg = send_msg.clone();
            let stories = stories
                .into_iter()
                .map(|title| Story::new(StoryInfo { title }))
                .collect();
            let msg = crate_msg(GameAction::StoriesAdded(stories));
            send_msg(msg);
        })
    };

    let on_select = {
        let send_msg = send_msg.clone();
        Callback::from(move |story_id: StoryId| {
            let send_msg = send_msg.clone();
            let msg = crate_msg(GameAction::VotingOpened(story_id));
            send_msg(msg);
        })
    };
    let on_update = {
        let send_msg = send_msg.clone();
        Callback::from(move |(story_id, info): (StoryId, StoryInfo)| {
            let send_msg = send_msg.clone();
            let msg = crate_msg(GameAction::StoryUpdated(story_id, info));
            send_msg(msg);
        })
    };
    let on_remove = {
        let send_msg = send_msg.clone();
        Callback::from(move |story_id: StoryId| {
            let send_msg = send_msg.clone();
            let msg = crate_msg(GameAction::StoryRemoved(story_id));
            send_msg(msg);
        })
    };
    let on_vote_click = {
        let send_msg = send_msg.clone();
        Callback::from(move |(story_id, vote_value): (StoryId, VoteValue)| {
            let send_msg = send_msg.clone();
            let msg = crate_msg(GameAction::VoteCasted(story_id, vote_value));
            send_msg(msg);
        })
    };
    let on_story_action = {
        let send_msg = send_msg.clone();
        Callback::from(move |(story_id, status): (StoryId, StoryStatus)| {
            let send_msg = send_msg.clone();
            let action = match status {
                StoryStatus::Approved => GameAction::ResultsApproved(story_id),
                StoryStatus::Revealed => GameAction::VotesRevealed(story_id),
                StoryStatus::Voting => GameAction::VotingOpened(story_id),
                StoryStatus::Init => todo!(),
            };
            let msg = crate_msg(action);
            send_msg(msg);
        })
    };

    match state.deref() {
        GameState::Loading => html! {
            <div class={classes!("p-4", "bg-yellow-200")}>
                <h2>{"Joining game..."}</h2>
            </div>
        },
        GameState::NotFound => html! {
            <div class={classes!("p-4", "bg-red-200")}>
                <h2>{"Game not found"}</h2>
            </div>
        },
        GameState::Playing(game) => {
            let estimated = game
                .stories
                .clone()
                .iter()
                .filter(|(_, story)| story.status == StoryStatus::Approved)
                .map(|(id, story)| {
                    html! {
                        <EstimatedStory
                            key={id.to_string()}
                            story={story.clone()}
                        />
                    }
                })
                .collect::<Html>();

            let selected = game
                .stories
                .clone()
                .iter()
                .filter(|(_, story)| {
                    story.status == StoryStatus::Voting || story.status == StoryStatus::Revealed
                })
                .map(|(id, story)| {
                    let user_id = user.id.clone();
                    let players = game.players.clone();
                    html! {
                        <SelectedStory
                            key={id.to_string()}
                            story={story.clone()}
                            on_vote_click={on_vote_click.clone()}
                            on_story_action={on_story_action.clone()}
                            {user_id}
                            {players}
                        />
                    }
                })
                .collect::<Html>();

            let backlog_entries = game
                .stories
                .clone()
                .iter()
                .filter(|(_, story)| story.status == StoryStatus::Init)
                .map(|(id, story)| {
                    html! {
                        <BacklogStoryEntry
                            key={id.to_string()}
                            story={story.clone()}
                            on_select={on_select.clone()}
                            on_update={on_update.clone()}
                            on_remove={on_remove.clone()}
                        />
                    }
                })
                .collect::<Html>();

            let players = game
                .players
                .clone()
                .into_iter()
                .filter(|(_, player)| player.active)
                .map(|(id, player)| {
                    html! {
                        <li key={id.to_string()}>
                            {&player.user.name}

                            if let PlayerRole::Admin = player.role {
                                <span>{" (moderator)"}</span>
                            }

                        </li>
                    }
                })
                .collect::<Html>();

            let is_admin = match game.players.get(&user.id) {
                Some(player) if player.role == PlayerRole::Admin => true,
                _ => false,
            };

            html! {
                <div class={classes!("flex", "max-w-7xl", "mx-auto")}>
                    <section class={classes!("flex-auto", "p-4")}>

                        <ul class={classes!("mb-4","bg-white", "shadow-sm", "rounded", "list-none")}>
                            { estimated }
                        </ul>

                        { selected }

                        if is_admin {
                            <>
                                <BacklogStoryList>
                                    { backlog_entries }
                                </BacklogStoryList>
                                <StoryForm {on_submit} />
                            </>
                        }

                    </section>
                    <aside class={classes!("flex-initial", "w-80", "p-4")}>

                        <h3 class="font-bold">{"Connected players:"}</h3>
                        <ul>
                            {players}
                        </ul>

                    </aside>
                </div>
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectedStoryProps {
    pub story: Story,
    pub user_id: UserId,
    pub players: HashMap<UserId, Player>,
    pub on_vote_click: Callback<(StoryId, VoteValue)>,
    pub on_story_action: Callback<(StoryId, StoryStatus)>,
}

#[function_component(SelectedStory)]
pub fn selected_story(props: &SelectedStoryProps) -> Html {
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
        let on_vote_click = props.on_vote_click.clone();
        Callback::from(move |payload| on_vote_click.emit(payload))
    };

    let on_accept_round = {
        let story_id = props.story.id.clone();
        let on_story_action = props.on_story_action.clone();
        Callback::from(move |_| on_story_action.emit((story_id, StoryStatus::Approved)))
    };

    let on_play_again = {
        let story_id = props.story.id.clone();
        let on_story_action = props.on_story_action.clone();
        Callback::from(move |_| on_story_action.emit((story_id, StoryStatus::Voting)))
    };

    let on_reveal_cards = {
        let story_id = props.story.id.clone();
        let on_story_action = props.on_story_action.clone();
        Callback::from(move |_| on_story_action.emit((story_id, StoryStatus::Revealed)))
    };

    let on_cancel_round = {
        let story_id = props.story.id.clone();
        let on_story_action = props.on_story_action.clone();
        Callback::from(move |_| on_story_action.emit((story_id, StoryStatus::Init)))
    };

    let is_admin = match props.players.get(&props.user_id) {
        Some(player) if player.role == PlayerRole::Admin => true,
        _ => false,
    };

    html!(
        <div class={classes!("mt-2", "mb-4")}>
            <h4 class={classes!("font-bold", "text-2xl", "text-slate-600")}>
                {&props.story.info.title}
            </h4>

            <ul class={classes!("list-none", "my-4", "flex", "flex-wrap")}>
                { votes.clone() }
            </ul>

            <VoteValueList
                story_id={props.story.id}
                {on_vote_click}
            />

            if is_admin {
                <div class={classes!("list-none", "my-4", "flex", "flex-wrap")}>
                    <div class="m-1">
                        <Button onclick={on_accept_round}>{ "Accept round" }</Button>
                    </div>
                    <div class="m-1">
                        <Button onclick={on_play_again}>{ "Play again" }</Button>
                    </div>
                    <div class="m-1">
                        <Button onclick={on_reveal_cards}>{ "Reveal cards" }</Button>
                    </div>
                    <div class="m-1">
                        <Button onclick={on_cancel_round}>{ "Cancel round" }</Button>
                    </div>
                </div>
            }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EstimatedStoryProps {
    pub story: Story,
}

#[function_component(EstimatedStory)]
pub fn estimated_story(props: &EstimatedStoryProps) -> Html {
    html!(
        <li class={classes!("flex")}>
            <h4 class={classes!("flex-auto", "p-4", "font-bold", "text-xs")}>
                {&props.story.info.title}
            </h4>
            <em>
                {&props.story.estimation()}
            </em>
        </li>
    )
}
