use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::form_textarea::FormTextarea;

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
        <div class={classes!("py-4")}>
            <FormTextarea
                rows={stories_count + 1}
                label="Add your stories (one per line)"
                value={raw_form.to_string()}
                {oninput}
            />
            <div class="h-4" />
            <button
                class={classes!(
                    "bg-blue-500", "hover:bg-blue-400",
                    "text-white", "font-bold",
                    "py-2", "px-4", "rounded",
                    "border-b-4", "border-blue-700", "hover:border-blue-500",
                    disabled.then(||Some("opacity-50")),
                    disabled.then(||Some("cursor-not-allowed")),
                )}
                {disabled}
                {onclick}
            >
                {"Add stories"}
            </button>
        </div>
    }
}
