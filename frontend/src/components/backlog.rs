use common::{GameAction, Story, StoryId, StoryInfo};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::form_input::FormInput;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EntryProps {
    pub story: Story,
    pub idx: usize,
    pub on_action: Callback<GameAction>,
}

enum EntryState {
    Default,
    Editing,
    Removing,
}

#[function_component(BacklogStoryEntry)]
pub fn backlog_story_entry(props: &EntryProps) -> Html {
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

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub stories: Vec<Story>,
    pub stories_ids: Vec<StoryId>,
    pub on_action: Callback<GameAction>,
}

#[function_component(BacklogStoryList)]
pub fn backlog_story_list(props: &ListProps) -> Html {
    let stories = props
        .stories
        .clone()
        .iter()
        .map(|story| {
            let key = story.id.to_string();
            let story = story.clone();
            let on_action = props.on_action.clone();
            let idx = props
                .stories_ids
                .iter()
                .position(|id| id == &story.id)
                .unwrap();

            html! {
                <BacklogStoryEntry {key} {idx} {story} {on_action} />
            }
        })
        .collect::<Html>();

    if !props.stories.is_empty() {
        html! {
            <section class="mb-12">
                <h3 class="px-4 font-semibold text-slate-400">
                    {"Your backlog"}
                </h3>
                <ul class="my-2 bg-white shadow-md rounded list-none">
                    {stories}
                </ul>
            </section>
        }
    } else {
        html! {
            <section class="mb-12">
                <h3 class="text-center text-2xl text-slate-400">
                    {"Add some stories to your backlog"}
                </h3>
            </section>
        }
    }
}

#[function_component(SelectIcon)]
pub fn select_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
    }
}

#[function_component(CancelIcon)]
pub fn cancel_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
    }
}

#[function_component(EditIcon)]
pub fn edit_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
        </svg>
    }
}

#[function_component(RemoveIcon)]
pub fn remove_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
    }
}

#[function_component(GoUpIcon)]
pub fn go_up_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 15l7-7 7 7" />
        </svg>
    }
}

#[function_component(GoDownIcon)]
pub fn go_down_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
    }
}
