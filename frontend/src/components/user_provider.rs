use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;
use yew::prelude::*;

use crate::components::nickname_input::NicknameInput;

const STORAGE_KEY: &str = "yew.user.self";

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
struct User {
    id: Uuid,
    nickname: String,
}

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
            let id = Uuid::new_v4();
            let new_user = User { id, nickname };
            user.set(Some(new_user));
        })
    };

    html! {
        if let Some(user) = user.deref() {
            <ContextProvider<User> context={(*user).clone()}>
              { props.children.clone() }
            </ContextProvider<User>>
        } else {
            <section style="padding: 32px">
                <NicknameInput {onsubmit} />
            </section>
        }
    }
}
