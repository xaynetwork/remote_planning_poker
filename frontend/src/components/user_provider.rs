use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

use crate::components::nickname_input::NicknameInput;

const STORAGE_KEY: &str = "yew.user.self";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
    let maybe_user = use_state(|| LocalStorage::get(STORAGE_KEY).ok() as Option<User>);

    use_effect_with_deps(
        move |maybe_user| {
            if maybe_user.clone().is_some() {
                let user = (*maybe_user).as_ref().unwrap();
                LocalStorage::set(STORAGE_KEY, user).expect("failed to set");
            };
            || ()
        },
        maybe_user.clone(),
    );

    let onsubmit = {
        let maybe_user = maybe_user.clone();
        Callback::from(move |nickname: String| {
            let id = Uuid::new_v4();
            let user = User { id, nickname };
            maybe_user.set(Some(user));
        })
    };

    html! {
        if maybe_user.is_none() {
            <section style="padding: 32px">
                <NicknameInput {onsubmit} />
            </section>
        } else {
            <ContextProvider<User> context={(*maybe_user).clone().unwrap()}>
              { props.children.clone() }
            </ContextProvider<User>>
        }
    }
}
