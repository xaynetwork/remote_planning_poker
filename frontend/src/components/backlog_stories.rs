use crate::components::backlog_story_entry::BacklogStoryEntry;
use common::{BacklogStory, GameAction, StoryId};
use indexmap::IndexMap;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub stories: IndexMap<StoryId, BacklogStory>,
    pub on_action: Callback<GameAction>,
}

#[function_component(BacklogStories)]
pub fn backlog_stories(props: &Props) -> Html {
    let stories = props
        .stories
        .iter()
        .enumerate()
        .map(|(idx, (story_id, story))| {
            let key = story_id.to_string();
            let story = story.clone();
            let on_action = &props.on_action;
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
