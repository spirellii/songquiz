mod guessing;
mod listening;
mod selection;
mod teams;

use std::sync::Arc;

use either::Either::{self, Left, Right};
use futures::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use gloo::{
    console::error,
    net::websocket::{futures::WebSocket, Message, WebSocketError},
};
use guessing::Guessing;
use listening::Listening;
use selection::Selection;
use teams::Teams;
use types::{
    game::{Phase, Team},
    message::{AdminInteraction, AdminUpdate},
    track::Track,
};

use wasm_bindgen::UnwrapThrowExt;
use yew::{platform::spawn_local, prelude::*};

/// The internal state of the admin client
#[derive(Debug, Clone)]
pub enum Admin {
    Uninitialized {
        sink: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    },
    Initialized {
        phase: Phase,
        teams: Vec<Team>,
        song: Option<Track>,
        sink: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    },
}

impl Admin {
    fn message_to_update(
        message: Result<Message, WebSocketError>,
    ) -> Either<AdminInteraction, Result<AdminUpdate, bool>> {
        match message {
            Err(_) => Right(Err(false)),
            Ok(Message::Text(message)) => {
                if let Ok(update) = serde_json::from_str::<AdminUpdate>(&message) {
                    Right(Ok(update))
                } else {
                    Right(Err(true))
                }
            }
            Ok(_) => Right(Err(true)),
        }
    }

    fn send_interaction(&self, interaction: AdminInteraction) {
        let sink = match self {
            Self::Uninitialized { sink } => Arc::clone(sink),
            Self::Initialized { sink, .. } => Arc::clone(sink),
        };
        spawn_local(async move {
            if let Err(_) = sink
                .lock()
                .await
                .send(Message::Text(
                    serde_json::to_string(&interaction).unwrap_throw(),
                ))
                .await
            {
                error!("Failed to send message to client");
            }
        });
    }
}

impl Component for Admin {
    type Message = Either<AdminInteraction, Result<AdminUpdate, bool>>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let socket = WebSocket::open("ws/admin").unwrap_throw();
        let (sink, stream) = socket.split();
        ctx.link().send_stream(stream.map(Self::message_to_update));
        Self::Uninitialized {
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Left(interaction) => {
                self.send_interaction(interaction);
                false
            }
            Right(Ok(update)) => {
                *self = Self::Initialized {
                    phase: update.phase,
                    teams: update.teams,
                    song: update.song,
                    sink: Arc::clone(match self {
                        Self::Initialized { sink, .. } => sink,
                        Self::Uninitialized { sink } => sink,
                    }),
                };
                true
            }
            // TODO: handle closed connection properly
            Right(Err(_)) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Self::Uninitialized { .. } => html! { "Ich warte auf den Server" },
            Self::Initialized {
                phase, teams, song, ..
            } => {
                let callback = ctx
                    .link()
                    .callback(|interaction: AdminInteraction| Left(interaction));
                let current = match phase {
                    Phase::Selection => html! { <Selection callback={callback.clone()}/> },
                    Phase::Listening { .. } => {
                        html! { <Listening callback={callback.clone()} track={song.clone()}/> }
                    }
                    Phase::Guessing { .. } => html! { <Guessing callback={callback.clone()}/> },
                };
                html! {
                    <div class="admin-container">
                        <Teams callback={callback} phase={phase.clone()} teams={teams.clone()}/>
                        { current }
                    </div>
                }
            }
        }
    }
}
