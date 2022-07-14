use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct FormTextareaProps {
    #[prop_or(1)]
    pub rows: usize,
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputEvent>,
}

#[function_component(FormTextarea)]
pub fn form_textarea(props: &FormTextareaProps) -> Html {
    html! {
        <div class={classes!("w-full")}>
            <label
                class={classes!(
                    "block",
                    "px-3", "py-2",
                    "sm:text-sm", "text-slate-500",
                )}
            >
                {props.label.clone()}
            </label>
            <textarea
                rows={props.rows.clone().to_string()}
                class={classes!(
                    "block",
                    "w-full",
                    "px-3", "py-2",
                    "sm:text-sm",
                    "text-slate-500",
                    "rounded-md",
                    "shadow-sm", "shadow-slate-300",
                    "outline-none",
                    "focus:shadow-md",
                )}
                value={props.value.clone()}
                oninput={&props.oninput}
            />
        </div>
    }
}
