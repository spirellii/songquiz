mod teams;

use futures::StreamExt;
use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use teams::Teams;
use types::{
    game::{Phase, Team},
    message::SpectatorUpdate,
    track::Track,
};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum Spectator {
    Uninitialized,
    Initialized {
        phase: Phase,
        teams: Vec<Team>,
        revealed: Option<Track>,
    },
}

impl Spectator {
    fn message_to_update(
        message: Result<Message, WebSocketError>,
    ) -> Result<SpectatorUpdate, bool> {
        match message {
            Err(_) => Err(false),
            Ok(Message::Text(message)) => {
                if let Ok(update) = serde_json::from_str::<SpectatorUpdate>(&message) {
                    Ok(update)
                } else {
                    Err(true)
                }
            }
            Ok(_) => Err(true),
        }
    }
}

impl Component for Spectator {
    type Message = Result<SpectatorUpdate, bool>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let socket = WebSocket::open("ws/spectator").unwrap_throw();
        ctx.link().send_stream(socket.map(Self::message_to_update));
        Self::Uninitialized
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(update) => {
                *self = Self::Initialized {
                    phase: update.phase,
                    teams: update.teams,
                    revealed: update.revealed,
                };
                true
            }
            // TODO: handle closed connection properly
            Err(_) => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self {
            Self::Uninitialized => html! { {"Ich warte auf den Server"} },
            Self::Initialized {
                phase,
                teams,
                revealed,
            } => {
                let current = match phase {
                    Phase::Selection => {
                        if let Some(song) = &revealed {
                            html! {
                                <div class="song container">
                                    <img src={song.image.clone()} class="song-image"/>
                                    <div class="song-info">
                                        <div class="song-name">{song.name.clone()}</div>
                                        <div class="song-artists">{song.artists.join(", ")}</div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    Phase::Listening { .. } => {
                        html! {}
                    }
                    Phase::Guessing { team, .. } => {
                        html! { <div class="container"> {teams[*team].name.clone()} {" haben den Buzzer gedrückt"} </div> }
                    }
                };
                html! {
                    <div class="spectator-container">
                        <Teams phase={phase.clone()} teams={teams.clone()}/>
                        { current }
                    </div>
                }
            }
        }
    }
}
