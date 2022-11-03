use common::User;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::components::login::Login;

const STORAGE_KEY: &str = "yew.user.self";

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) children: Children,
}

#[function_component(UserProvider)]
pub(crate) fn user_provider(props: &Props) -> Html {
    let user = use_state(|| LocalStorage::get(STORAGE_KEY).ok() as Option<User>);

    {
        let user = user.clone();
        use_effect_with_deps(
            move |user| {
                if let Some(user) = &**user {
                    LocalStorage::set(STORAGE_KEY, user).expect("failed to set");
                }
                || ()
            },
            user,
        );
    };

    let onsubmit = {
        let user = user.clone();
        Callback::from(move |nickname: String| {
            let new_user = User::new(nickname);
            user.set(Some(new_user));
        })
    };

    html! {
        if let Some(user) = &*user {
            <ContextProvider<User> context={(*user).clone()}>
              { props.children.clone() }
            </ContextProvider<User>>
        } else {
            <Login {onsubmit} />
        }
    }
}
