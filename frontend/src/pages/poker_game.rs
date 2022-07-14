use common::{AppEvent, Game, GameId, User};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use yew_hooks::UseWebSocketReadyState;
use yew_router::prelude::*;

use crate::{
    components::{
        backlog_stories::BacklogStories, connection_indicator::ConnectionIndicator,
        connection_provider::use_game_connection, estimated_stories::EstimatedStories,
        players::Players, selected_story_entry::SelectedStoryEntry, story_form::StoryForm,
    },
    Route,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: GameId,
}

enum GameState {
    Loading,
    Playing(Game),
    NotFound,
}

impl Reducible for GameState {
    type Action = AppEvent;

    /// Reducer Function
    fn reduce(self: Rc<Self>, message: Self::Action) -> Rc<Self> {
        match self.deref() {
            GameState::Loading => match message {
                AppEvent::CurrentState(game) => GameState::Playing(game),
                AppEvent::GameNotFound(_) => GameState::NotFound,
                // TODO: this shouldn't happen, so figure out how to handle it
                _ => GameState::Loading,
            },
            GameState::Playing(game) => {
                let mut game = game.clone();
                match message {
                    AppEvent::GameMessage(user_id, action) => {
                        game.update(user_id, action);
                        GameState::Playing(game)
                    }
                    _ => GameState::Playing(game),
                }
            }
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
            let players = game.to_active_players();
            let is_admin = game.is_user_admin(&user.id);
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

                            <EstimatedStories
                                stories={game.estimated_stories.clone()}
                            />

                            {
                                if let Some(story) = &game.selected_story {
                                    let key = story.id.to_string();
                                    let story = story.clone();
                                    let user_id = user.id;
                                    let players = game.players.clone();
                                    html! {
                                        <SelectedStoryEntry
                                            {key} {story} {user_id} {players}
                                            on_action={&conn.send}
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
                                    <BacklogStories
                                        stories={game.backlog_stories.clone()}
                                        on_action={&conn.send}
                                    />
                                    <StoryForm on_action={&conn.send} />
                                </>
                            }

                        </section>
                        <aside class="w-1/3 p-4">

                            <Players {players} />

                        </aside>
                    </div>
                </>
            }
        }
    }
}
