use common::{
    Game, GameAction, GameId, GameMessage, PlayerRole, Story, StoryInfo, StoryStatus, User,
};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use std::{ops::Deref, rc::Rc};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{
    approved::ApprovedStoryList, backlog::BacklogStoryList, players::PlayerList,
    story_form::StoryForm, voting::SelectedStory,
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

    let send_msg = {
        let user_id = user.id.clone();
        let game_id = GameId::new(props.id.clone());
        let ws_ref = ws_ref.clone();

        move |action: GameAction| {
            spawn_local(async move {
                let msg = GameMessage {
                    user_id,
                    game_id,
                    action,
                };
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
                send_msg(GameAction::PlayerJoined(user));
                || ()
            },
            (),
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
            send_msg(GameAction::StoriesAdded(stories));
        })
    };
    let on_action = {
        let send_msg = send_msg.clone();
        Callback::from(move |action: GameAction| {
            let send_msg = send_msg.clone();
            send_msg(action);
        })
    };

    match state.deref() {
        GameState::Loading => html! {
            <div class={classes!("p-4")}>
                <h2>{"Joining game..."}</h2>
            </div>
        },
        GameState::NotFound => html! {
            <div class={classes!("p-4", "bg-red-200")}>
                <h2>{"Game not found"}</h2>
            </div>
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
                <div class={classes!("flex", "max-w-7xl", "mx-auto")}>
                    <section class={classes!("flex-auto", "p-4")}>

                        <ApprovedStoryList stories={approved} />
                        {selected
                            .iter()
                            .map(|story| {
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
                            })
                            .collect::<Html>()
                        }

                        if is_admin {
                            <>
                                <BacklogStoryList
                                    stories={backlog}
                                    {on_action}
                                />
                                <StoryForm {on_submit} />
                                <pre class="my-8">{STORIES_TO_COPY}</pre>
                            </>
                        }

                    </section>
                    <aside class={classes!("w-80", "p-4")}>

                        <PlayerList {players} />

                    </aside>
                </div>
            }
        }
    }
}

const STORIES_TO_COPY: &str = "TY-2588 Build exploration stack
TY-2731 Use semantic filtering across stacks
TY-2802 improve error handling for invalid engine state
TY-2749 handle available sources event
TY-2756 serialize exploration stack data";
