use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{button::Button, form_textarea::FormTextarea};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub on_submit: Callback<Vec<String>>,
}

#[function_component(StoryForm)]
pub fn story_form(props: &Props) -> Html {
    let raw_form = use_state(|| "".to_string());
    let story_titles: Vec<String> = raw_form
        .split("\n")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let stories_count = story_titles.len();
    let disabled = !(stories_count > 0);

    let oninput = {
        let raw_form = raw_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            raw_form.set(value);
        })
    };
    let onclick = {
        let raw_form = raw_form.clone();
        let on_submit = props.on_submit.clone();

        Callback::from(move |_| {
            raw_form.set("".to_string());
            on_submit.emit(story_titles.clone());
        })
    };

    html! {
        <div class={classes!("mb-8")}>
            <FormTextarea
                rows={stories_count + 2}
                label="Add your stories (one per line)"
                value={raw_form.to_string()}
                {oninput}
            />
            <div class="h-4" />
            <Button {onclick} {disabled}>
                {"Add stories"}
            </Button>
        </div>
    }
}
