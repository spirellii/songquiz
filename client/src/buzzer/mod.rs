use std::sync::Arc;

use either::Either::{self, Left, Right};
use futures::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use gloo::{
    console::error,
    net::websocket::{futures::WebSocket, Message, WebSocketError},
};
use types::{
    game::{Phase, Team},
    message::{BuzzerInteraction, BuzzerUpdate},
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::Element;
use yew::{platform::spawn_local, prelude::*};

#[derive(Debug, Clone)]
pub enum Buzzer {
    Uninitialized {
        sink: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    },
    Initialized {
        phase: Phase,
        teams: Vec<Team>,
        registered: Option<usize>,
        sink: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    },
}

impl Buzzer {
    fn message_to_update(
        message: Result<Message, WebSocketError>,
    ) -> Either<BuzzerInteraction, Result<BuzzerUpdate, bool>> {
        match message {
            Err(_) => Right(Err(false)),
            Ok(Message::Text(message)) => {
                if let Ok(update) = serde_json::from_str::<BuzzerUpdate>(&message) {
                    Right(Ok(update))
                } else {
                    Right(Err(true))
                }
            }
            Ok(_) => Right(Err(true)),
        }
    }

    fn send_interaction(&self, interaction: BuzzerInteraction) {
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

impl Component for Buzzer {
    type Message = Either<BuzzerInteraction, Result<BuzzerUpdate, bool>>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let socket = WebSocket::open("ws/buzzer").unwrap_throw();
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
                match self {
                    Self::Uninitialized { sink } => {
                        *self = Self::Initialized {
                            phase: update.phase,
                            teams: update.teams,
                            registered: update.registered,
                            sink: Arc::clone(sink),
                        };
                    }
                    Self::Initialized { sink, .. } => {
                        *self = Self::Initialized {
                            phase: update.phase,
                            teams: update.teams,
                            registered: update.registered,
                            sink: Arc::clone(sink),
                        }
                    }
                };
                true
            }
            // TODO: handle closed connection properly
            Right(Err(_)) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx
            .link()
            .callback(|interaction: BuzzerInteraction| Left(interaction));
        match self {
            Self::Uninitialized { .. } => html! { {"Ich warte auf den Server"} },
            Self::Initialized {
                phase,
                teams,
                registered,
                ..
            } => match registered {
                Some(team) => {
                    let onclick = {
                        let callback = callback.clone();
                        move |_| {
                            callback.emit(BuzzerInteraction::Buzz);
                        }
                    };
                    let inactive = if phase.is_active(*team) {
                        None
                    } else {
                        Some("buzzer-inactive")
                    };
                    html! {
                        <div class={classes!("container", "buzzer-container")}>
                            <div class="buzzer-team-name">{teams[*team].name.clone()}</div>
                            <button class={classes!("buzzer", inactive)} {onclick}></button>
                        </div>
                    }
                }
                None => {
                    let onclick = {
                        let callback = callback.clone();
                        move |event: MouseEvent| {
                            let target = event.target().unwrap_throw();
                            let element = target.dyn_into::<Element>().unwrap_throw();
                            let team = element
                                .get_attribute("data-team")
                                .unwrap_throw()
                                .parse::<usize>()
                                .ok()
                                .unwrap_throw();
                            callback.emit(BuzzerInteraction::Register { team: team });
                        }
                    };
                    let teams = teams.iter().enumerate().map(|(n, team)| {
                        html! { <li class={classes!("container", "margin-bottom")}><button class="buzzer-team-name" onclick={onclick.clone()} data-team={n.to_string()}>{team.name.clone()}</button></li> }
                    }).collect::<Html>();
                    html! {
                        <ul>
                            { teams }
                        </ul>
                    }
                }
            },
        }
    }
}
