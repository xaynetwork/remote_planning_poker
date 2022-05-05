use common::{Story, Vote};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EntryProps {
    pub story: Story,
}

#[function_component(ApprovedStory)]
pub fn approved_story(props: &EntryProps) -> Html {
    let avrg = &props.story.votes_avrg();
    let estimation = Vote::get_closest_vote(avrg).value();
    html! {
        <li class="flex items-center px-2 border-b">
            <h4
                class={classes!(
                    "flex-auto", "p-2",
                    "font-bold", "text-xs", "text-slate-500"
                )}
            >
                {&props.story.info.title}
            </h4>
            <strong
                class={classes!(
                    "m-2", "py-1", "px-2",
                    "font-bold", "text-xs", "text-green-700",
                    "rounded", "bg-green-200"
                )}
            >
                {estimation}
            </strong>
        </li>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListProps {
    pub stories: Vec<Story>,
}

#[function_component(ApprovedStoryList)]
pub fn approved_story_list(props: &ListProps) -> Html {
    let stories = props
        .stories
        .iter()
        .map(|story| {
            html! {
                <ApprovedStory
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
