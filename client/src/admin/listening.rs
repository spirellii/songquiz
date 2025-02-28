use types::{message::AdminInteraction, track::Track};
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Properties {
    pub callback: Callback<AdminInteraction>,
    pub track: Option<Track>,
}

#[function_component]
pub fn Listening(props: &Properties) -> Html {
    let song = props.track.clone().unwrap_or_default();
    let onclick = {
        let callback = props.callback.clone();
        move |_| {
            callback.emit(AdminInteraction::StopListening);
        }
    };
    html! {
        <div class="song container">
            <img src={song.image.clone()} class="song-image"/>
            <div class="song-info">
                <div class="song-name-smaller">{song.name.clone()}</div>
                <div class="song-artists">{song.artists.join(", ")}</div>
            </div>
            <button class="admin-stop-song" {onclick}>{"Song stoppen"}</button>
        </div>
    }
}
