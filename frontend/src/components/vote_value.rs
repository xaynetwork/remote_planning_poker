use common::{StoryId, VoteValue};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EntryProps {
    pub value: VoteValue,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(VoteValueButton)]
pub fn vote_value_button(props: &EntryProps) -> Html {
    let onclick = &props.onclick;
    html!(
        <button
            class={classes!(
                "m-1", "py-2", "w-12",
                "text-center", "font-light", "text-slate-500",
                "shadow-md", "rounded-md",
                "bg-slate-50", "hover:bg-green-200",
                "cursor-pointer"
            )}
            {onclick}
        >
            { props.value as u8 }
        </button>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub story_id: StoryId,
    #[prop_or_else(Callback::noop)]
    pub on_vote_click: Callback<(StoryId, VoteValue)>,
}

#[function_component(VoteValueList)]
pub fn vote_value_list(props: &ListProps) -> Html {
    let vote_values = [
        VoteValue::Zero,
        VoteValue::One,
        VoteValue::Two,
        VoteValue::Three,
        VoteValue::Five,
        VoteValue::Eight,
        VoteValue::Thirteen,
        VoteValue::TwentyOne,
        VoteValue::Fourty,
        VoteValue::OneHundred,
    ]
    .iter()
    .map(|value| {
        let value = value.clone();
        let onclick = {
            let on_vote_click = props.on_vote_click.clone();
            let story_id = props.story_id.clone();
            let value = value.clone();

            Callback::from(move |_| on_vote_click.emit((story_id, value)))
        };
        html!(
            <div class="m-1">
                <VoteValueButton {value} {onclick} />
            </div>
        )
    })
    .collect::<Html>();

    html!(
        <section
            class={classes!(
                "flex", "flex-wrap", "my-4", "p-4",
                "bg-slate-300", "shadow-inner", "rounded",
            )}
        >
            {vote_values}
        </section>
    )
}
