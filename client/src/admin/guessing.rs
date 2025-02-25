use types::message::AdminInteraction;
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Properties {
    pub callback: Callback<AdminInteraction>,
}

#[function_component]
pub fn Guessing(props: &Properties) -> Html {
    let callback = props.callback.clone();
    let accept = {
        let callback = callback.clone();
        move |_| {
            callback.emit(AdminInteraction::AcceptGuess);
        }
    };
    let reject = {
        let callback = callback.clone();
        move |_| {
            callback.emit(AdminInteraction::RejectGuess);
        }
    };
    html! {
        <>
            <button onclick={accept}>{"Akzeptieren"}</button>
            <button onclick={reject}>{"Ablehnen"}</button>
        </>
    }
}
