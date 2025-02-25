use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use log::{debug, info};
use rspotify::prelude::OAuthClient;
use tokio::{select, sync::RwLock};
use types::{
    game::Phase,
    message::{BuzzerInteraction, BuzzerUpdate},
};

use crate::game::Game;

pub async fn buzzer_upgrade(
    State(state): State<Arc<RwLock<Game>>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    debug!(
        "Got buzzer connection from {:?} at {:?}, entering handler",
        user_agent, addr
    );
    ws.on_upgrade(move |socket| buzzer_handler(state, socket, addr))
}

async fn buzzer_handler(state: Arc<RwLock<Game>>, mut socket: WebSocket, addr: SocketAddr) {
    let mut receiver = {
        let game = state.read().await;
        let receiver = game.channel.subscribe();
        let _ = game.channel.send(Default::default());
        receiver
    };
    let mut registered = None;
    loop {
        select! {
            update = receiver.recv() => {
                if let Ok(update) = update {
                    if update.teams_invalidated {
                        registered = None;
                    }
                } else {
                    debug!("Internal channel closed, exiting handler");
                    break;
                }
                let update = {
                    let game = state.read().await;
                    BuzzerUpdate {
                        phase: game.phase.clone(),
                        teams: game.teams.clone(),
                        registered
                    }
                };
                if let Err(_) = socket.send(Message::text(serde_json::to_string(&update).unwrap())).await {
                    debug!("Connection closed by peer at {:?}, exiting handler", addr);
                    break;
                };
            },
            raw = socket.recv() => {
                if let Some(Ok(Message::Text(message))) = raw {
                    if let Ok(interaction) = serde_json::from_str::<BuzzerInteraction>(&message) {
                        match interaction {
                            BuzzerInteraction::Register { team } => {
                                let game = state.read().await;
                                if team < game.teams.len() {
                                    info!("Buzzer at {:?} registered for team {:?}", addr, state.read().await.teams[team]);
                                    registered = Some(team);
                                    let _ = game.channel.send(Default::default());
                                }
                            },
                            BuzzerInteraction::Buzz => {
                                if let Some(team) = registered {
                                    let mut game = state.write().await;
                                    if let Phase::Listening { active } = &game.phase {
                                        if active[team] {
                                            debug!("Received valid buzz from team {} ({:?})", team, game.teams[team]);
                                            game.phase = Phase::Guessing { active: active.clone(), team: team };
                                            let _ = game.channel.send(Default::default());
                                            if let Err(e) = game.spotify.pause_playback(None).await {
                                                debug!("Spotify returned error {e}, likely not authorized");
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    debug!("Connection closed by peer at {:?}, exiting handler", addr);
                    break;
                }
            }
        }
    }
}
