use common::{Game, GameAction, GameId, GameMessage, PlayerRole, StoryStatus, User, UserId};
use std::{ops::Deref, rc::Rc};
use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::{use_location, use_web_socket, UseWebSocketReadyState};
use yew_router::prelude::*;

use crate::{
    components::{
        approved::ApprovedStoryList, backlog::BacklogStoryList, players::PlayerList,
        story_form::StoryForm, voting::SelectedStory,
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
    let location = use_location();
    let user = use_context::<User>().expect("no user ctx found");
    let state = use_reducer(|| GameState::Loading);
    let url = location.origin.replace("http", "ws");
    let ws_url = format!("{}/api/game", url);
    let ws = use_web_socket(ws_url);

    let prepare_msg = {
        let user_id = user.id.clone();
        let game_id = GameId::new(props.id.clone());
        use_ref(|| create_message(&user_id, &game_id))
    };

    {
        let ws = ws.clone();
        let ws_state = ws.ready_state.clone();
        let prepare_msg = prepare_msg.clone();
        let user = user.clone();
        // Send `join` message when the connection opens
        use_effect_with_deps(
            move |ready_state| {
                if UseWebSocketReadyState::Open == **ready_state {
                    let msg = prepare_msg(GameAction::PlayerJoined(user));
                    ws.send(msg);
                }
                || ()
            },
            ws_state, // dependents
        );
    }

    {
        let ws = ws.clone();
        let state = state.clone();
        // Receive message by depending on `ws.message`.
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    if let Ok(action) = serde_json::from_str(message) {
                        state.dispatch(action);
                    }
                }
                || ()
            },
            ws.message,
        );
    }

    let on_action = {
        let ws = ws.clone();
        let prepare_msg = prepare_msg.clone();
        Callback::from(move |action: GameAction| {
            let msg = prepare_msg(action);
            ws.send(msg);
        })
    };

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

            html! {
                <div class="flex max-w-7xl mx-auto">
                    <section class="w-2/3 p-4">

                        <ApprovedStoryList stories={approved} />

                        {
                            if let Some(story) = selected.first() {
                                let key = story.id.to_string();
                                let story = story.clone();
                                let user_id = user.id.clone();
                                let players = game.players.clone();
                                let on_action = on_action.clone();
                                html! {
                                    <SelectedStory
                                        {key} {story} {user_id} {players}
                                        {on_action}
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
                                    on_action={on_action.clone()}
                                />
                                <StoryForm {on_action} />
                                <pre class="my-8">{STORIES_TO_COPY}</pre>
                            </>
                        }

                    </section>
                    <aside class="w-1/3 p-4">

                        <PlayerList {players} />

                    </aside>
                </div>
            }
        }
    }
}

fn create_message(user_id: &UserId, game_id: &GameId) -> impl Fn(GameAction) -> String {
    let user_id = user_id.to_owned();
    let game_id = game_id.to_owned();

    move |action: GameAction| {
        let msg = GameMessage {
            user_id,
            game_id,
            action,
        };
        serde_json::to_string(&msg).unwrap()
    }
}

const STORIES_TO_COPY: &str = "TY-2588 Build exploration stack
TY-2731 Use semantic filtering across stacks
TY-2802 improve error handling for invalid engine state
TY-2749 handle available sources event
TY-2756 serialize exploration stack data";
