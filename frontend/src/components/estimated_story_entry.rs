use common::EstimatedStory;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub story: EstimatedStory,
}

#[function_component(EstimatedStoryEntry)]
pub fn estimated_story_entry(props: &Props) -> Html {
    let estimation = &props.story.estimate.value();

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
