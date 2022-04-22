use common::User;
use gloo_storage::{LocalStorage, Storage};
use std::ops::Deref;
use yew::prelude::*;

use crate::components::login::Login;

const STORAGE_KEY: &str = "yew.user.self";

#[derive(Properties, PartialEq)]
pub struct UserProviderProps {
    pub children: Children,
}

#[function_component(UserProvider)]
pub fn user_provider(props: &UserProviderProps) -> Html {
    let user = use_state(|| LocalStorage::get(STORAGE_KEY).ok() as Option<User>);

    use_effect_with_deps(
        move |user| {
            if let Some(user) = user.deref() {
                LocalStorage::set(STORAGE_KEY, user).expect("failed to set");
            }
            || ()
        },
        user.clone(),
    );

    let onsubmit = {
        let user = user.clone();
        Callback::from(move |nickname: String| {
            let new_user = User::new(nickname);
            user.set(Some(new_user));
        })
    };

    html! {
        if let Some(user) = user.deref() {
            <ContextProvider<User> context={(*user).clone()}>
              { props.children.clone() }
            </ContextProvider<User>>
        } else {
            <Login {onsubmit} />
        }
    }
}
