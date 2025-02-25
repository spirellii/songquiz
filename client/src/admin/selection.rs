use gloo::console::debug;
use types::message::AdminInteraction;
use url::Url;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Properties {
    pub callback: Callback<AdminInteraction>,
}

fn spotify_link_to_uri(link: String) -> Option<String> {
    let url = Url::parse(&link).ok()?;
    if url.domain()? != "open.spotify.com" {
        return None;
    }
    if url.scheme() != "https" {
        return None;
    }
    let mut segs = url.path_segments()?;
    if segs.next() != Some("track") {
        return None;
    }
    let id = segs.next()?;
    if segs.next() != None {
        return None;
    }
    Some(format!("spotify:track:{}", id))
}

#[function_component]
pub fn Selection(props: &Properties) -> Html {
    let onclick = {
        let callback = props.callback.clone();
        move |_| {
            let document = gloo::utils::document();
            let input = document
                .get_element_by_id("selection-input")
                .unwrap_throw()
                .dyn_into::<HtmlInputElement>()
                .unwrap_throw();
            if let Some(uri) = spotify_link_to_uri(input.value()) {
                callback.emit(AdminInteraction::Selection { id: uri })
            } else {
                debug!("Spotify link could not be parsed: {}", input.value());
            }
        }
    };
    html! {
        <div class={classes!("selection", "container")}>
            <button id="selection-button" {onclick}>{"play_arrow"}</button>
            <input id="selection-input"/>
        </div>
    }
}
