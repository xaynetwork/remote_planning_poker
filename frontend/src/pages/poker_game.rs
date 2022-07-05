use common::{AppMessage, Game, PlayerRole, StoryStatus, User};
use std::{ops::Deref, rc::Rc};
use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::UseWebSocketReadyState;
use yew_router::prelude::*;

use crate::{
    components::{
        approved::ApprovedStoryList, backlog::BacklogStoryList,
        connection_indicator::ConnectionIndicator, connection_provider::use_game_connection,
        players::PlayerList, story_form::StoryForm, voting::SelectedStory,
    },
    Route,
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
    type Action = AppMessage;

    /// Reducer Function
    fn reduce(self: Rc<Self>, message: Self::Action) -> Rc<Self> {
        match self.deref() {
            GameState::Loading => match message {
                AppMessage::CurrentState(game) => GameState::Playing(game),
                AppMessage::GameNotFound(_) => GameState::NotFound,
                // TODO: this shouldn't happen, so figure out how to handle it
                _ => GameState::Loading,
            },
            GameState::Playing(game) => match message {
                AppMessage::GameMessage(user_id, action) => {
                    let game = game.clone().reduce(user_id, action);
                    GameState::Playing(game)
                }
                _ => GameState::Playing(game.clone()),
            },
            GameState::NotFound => GameState::NotFound,
        }
        .into()
    }
}

#[function_component(PokerGame)]
pub fn poker_game(props: &Props) -> Html {
    let user = use_context::<User>().expect("no user ctx found");
    let conn = use_game_connection(&props.id, &user);
    let state = use_reducer(|| GameState::Loading);

    {
        let ws = conn.clone();
        let state = state.clone();
        // Receive message by depending on `ws.message`.
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &*message {
                    let action = serde_json::from_str(&message).unwrap();
                    state.dispatch(action);
                }
                || ()
            },
            ws.message,
        );
    }

    match &*state {
        GameState::Loading => html! {
            <section class="h-full flex items-center justify-center">
                <div class="p-4 text-center text-slate-500">
                    <h2 class="mb-12 text-3xl font-medium">{"Joining a game..."}</h2>
                </div>
            </section>
        },
        GameState::NotFound => html! {
            <Redirect<Route> to={Route::NotFound}/>
        },
        GameState::Playing(game) => {
            let approved = game.stories_by_filter(|s| s.status == StoryStatus::Approved);
            let selected = game.stories_by_filter(|s| {
                s.status == StoryStatus::Voting || s.status == StoryStatus::Revealed
            });
            let backlog = game.stories_by_filter(|s| s.status == StoryStatus::Init);
            let players = game.active_players();
            let is_admin = match game.players.get(&user.id) {
                Some(player) if player.role == PlayerRole::Admin => true,
                _ => false,
            };
            let (label, bg_class) = match conn.ready_state {
                UseWebSocketReadyState::Connecting => ("Connecting", "bg-yellow-500"),
                UseWebSocketReadyState::Open => ("Connection open", "bg-green-500"),
                UseWebSocketReadyState::Closing => ("Closing connection", "bg-orange-500"),
                UseWebSocketReadyState::Closed => ("Connection closed", "bg-red-500"),
            };

            html! {
                <>
                    <ConnectionIndicator {label} {bg_class} />
                    <div class="flex max-w-7xl mx-auto">
                        <section class="w-2/3 p-4">

                            <ApprovedStoryList stories={approved} />

                            {
                                if let Some(story) = selected.first() {
                                    let key = story.id.to_string();
                                    let story = story.clone();
                                    let user_id = user.id.clone();
                                    let players = game.players.clone();
                                    html! {
                                        <SelectedStory
                                            {key} {story} {user_id} {players}
                                            on_action={conn.send.clone()}
                                        />
                                    }
                                } else {
                                    html! {
                                        <section class="mb-12">
                                            <h3 class="text-center text-2xl text-slate-400">
                                                {"Waiting for a round to start.."}
                                            </h3>
                                        </section>
                                    }
                                }
                            }

                            if is_admin {
                                <>
                                    <BacklogStoryList
                                        stories={backlog}
                                        on_action={conn.send.clone()}
                                    />
                                    <StoryForm on_action={conn.send.clone()} />
                                    <pre class="my-8">{STORIES_TO_COPY}</pre>
                                </>
                            }

                        </section>
                        <aside class="w-1/3 p-4">

                            <PlayerList {players} />

                        </aside>
                    </div>
                </>
            }
        }
    }
}

const STORIES_TO_COPY: &str = "TY-2588 Build exploration stack
TY-2731 Use semantic filtering across stacks
TY-2802 improve error handling for invalid engine state
TY-2749 handle available sources event
TY-2756 serialize exploration stack data";
