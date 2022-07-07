use common::EstimatedStory;
use yew::prelude::*;

use crate::components::estimated_story_entry::EstimatedStoryEntry;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub stories: Vec<EstimatedStory>,
}

#[function_component(EstimatedStories)]
pub fn estimated_stories(props: &Props) -> Html {
    let stories = props
        .stories
        .iter()
        .map(|story| {
            html! {
                <EstimatedStoryEntry
                    key={story.id.to_string()}
                    story={story.clone()}
                />
            }
        })
        .collect::<Html>();

    if !props.stories.is_empty() {
        html! {
            <ul class="mb-12 bg-white shadow-sm rounded list-none">
                {stories}
            </ul>
        }
    } else {
        html! {}
    }
}
