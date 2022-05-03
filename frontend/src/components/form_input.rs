use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct FormInputProps {
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_else(Callback::noop)]
    pub onkeypress: Callback<KeyboardEvent>,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
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
            <input
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
                placeholder={props.placeholder.clone()}
                onkeypress={&props.onkeypress}
            />
        </div>
    }
}
