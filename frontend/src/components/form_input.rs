use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct FormInputProps {
    #[prop_or_default]
    pub(crate) label: String,
    #[prop_or_default]
    pub(crate) placeholder: String,
    #[prop_or_default]
    pub(crate) value: String,
    #[prop_or_else(Callback::noop)]
    pub(crate) onkeypress: Callback<KeyboardEvent>,
}

#[function_component(FormInput)]
pub(crate) fn form_input(props: &FormInputProps) -> Html {
    html! {
        <div class={classes!("w-full")}>
            if !props.label.is_empty() {
                <label
                    class={classes!(
                        "block",
                        "px-3", "py-2",
                        "sm:text-sm", "text-slate-500",
                    )}
                >
                    {props.label.clone()}
                </label>
            }
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
