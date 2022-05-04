use common::Story;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EntryProps {
    pub story: Story,
}

#[function_component(ApprovedStory)]
pub fn approved_story(props: &EntryProps) -> Html {
    html!(
        <li class="flex items-center mx-2">
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
                {&props.story.estimation()}
            </strong>
        </li>
    )
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

    html!(
        <ul class="mb-12 bg-white shadow-sm rounded list-none">
            {stories}
        </ul>
    )
}
