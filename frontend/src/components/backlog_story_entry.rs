use common::{BacklogStory, GameAction, StoryInfo};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{form_input::FormInput, icons::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub story: BacklogStory,
    pub idx: usize,
    pub on_action: Callback<GameAction>,
}

enum EntryState {
    Default,
    Editing,
    Removing,
}

#[function_component(BacklogStoryEntry)]
pub fn backlog_story_entry(props: &Props) -> Html {
    let state = use_state(|| EntryState::Default);

    let on_select = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::VotingOpened(story_id)))
    };
    let on_remove = {
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::StoryRemoved(story_id)))
    };
    let on_edit_intent = {
        let state = state.clone();
        Callback::from(move |_| state.set(EntryState::Editing))
    };
    let on_remove_intent = {
        let state = state.clone();
        Callback::from(move |_| state.set(EntryState::Removing))
    };
    let on_cancel = {
        let state = state.clone();
        Callback::from(move |_| state.set(EntryState::Default))
    };
    let on_go_up = {
        let new_idx = if props.idx > 0 { props.idx - 1 } else { 0 };
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::StoryPositionChanged(story_id, new_idx)))
    };
    let on_go_down = {
        let new_idx = props.idx + 1;
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::StoryPositionChanged(story_id, new_idx)))
    };
    let onkeypress = {
        let state = state.clone();
        let story_id = props.story.id.clone();
        let on_action = props.on_action.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                let title = value.trim();

                if !title.is_empty() {
                    let title = title.to_string();
                    on_action.emit(GameAction::StoryUpdated(story_id, StoryInfo { title }));
                    state.set(EntryState::Default);
                }
            }
        })
    };

    let button_class = "m-1 p-1";
    let buttons = match *state {
        EntryState::Default => html! {
            <>
                <button
                    title="Select to start round"
                    class={classes!(
                        button_class,
                        "hover:text-green-400",
                        "flex"
                    )}
                    onclick={on_select}
                >
                    <span class="mr-2">{"Select"}</span>
                    <SelectIcon />
                </button>
                <button
                    title="Edit story"
                    class={classes!(button_class, "hover:text-blue-400")}
                    onclick={on_edit_intent}
                >
                    <EditIcon />
                </button>
                <button
                    title="Remove story"
                    class={classes!(button_class, "hover:text-red-400")}
                    onclick={on_remove_intent}
                >
                    <RemoveIcon />
                </button>
                <button
                    title="Go up"
                    class={classes!(button_class, "hover:text-blue-400")}
                    onclick={on_go_up}
                >
                    <GoUpIcon />
                </button>
                <button
                    title="Go down"
                    class={classes!(button_class, "hover:text-blue-400")}
                    onclick={on_go_down}
                >
                    <GoDownIcon />
                </button>
            </>
        },
        EntryState::Editing => html! {
            <button
                title="Cancel"
                class={classes!(button_class, "hover:text-slate-400")}
                onclick={on_cancel}
            >
                <CancelIcon />
            </button>
        },
        EntryState::Removing => html! {
            <>
                <button
                    class={classes!(
                        "m-1", "px-4", "rounded-sm", "text-white",
                        "bg-red-400", "hover:bg-red-600",
                    )}
                    onclick={on_remove}
                >
                    {"Remove"}
                </button>
                <button
                    title="Cancel"
                    class={classes!(button_class, "hover:text-slate-400")}
                    onclick={on_cancel}
                >
                    <CancelIcon />
                </button>
            </>
        },
    };

    html! {
        <li class="py-4 px-4 border-b flex items-center hover:bg-slate-100 text-slate-500">
            if let EntryState::Editing = *state {
                <FormInput
                    value={props.story.info.title.clone()}
                    {onkeypress}
                />
            } else {
                <h4 class="flex-1 text-base">
                    {props.story.info.title.clone()}
                </h4>
            }
            <div class="ml-8 flex">
                {buttons}
            </div>
        </li>
    }
}
