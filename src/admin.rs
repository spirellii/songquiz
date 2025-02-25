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
use rspotify::{
    model::TrackId,
    prelude::{BaseClient, OAuthClient},
};
use tokio::{select, sync::RwLock};
use types::{
    game::{Phase, Team},
    message::{AdminInteraction, AdminUpdate},
};

use crate::{
    game::{Game, Update},
    names::random_name,
};

pub async fn admin_upgrade(
    State(state): State<Arc<RwLock<Game>>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    debug!(
        "Got admin connection from {:?} at {:?}, entering handler",
        user_agent, addr
    );
    ws.on_upgrade(move |socket| admin_handler(state, socket, addr))
}

async fn admin_handler(state: Arc<RwLock<Game>>, mut socket: WebSocket, addr: SocketAddr) {
    let mut receiver = {
        let game = state.read().await;
        let receiver = game.channel.subscribe();
        let _ = game.channel.send(Default::default());
        receiver
    };
    loop {
        select! {
            _ = receiver.recv() => {
                let update = {
                    let game = state.read().await;
                    AdminUpdate {
                        phase: game.phase.clone(),
                        teams: game.teams.clone(),
                        song: game.song.clone().map(|t| t.into())
                    }
                };
                if let Err(_) = socket.send(Message::text(serde_json::to_string(&update).unwrap())).await {
                    debug!("Connection closed by peer at {:?}, exiting handler", addr);
                    break;
                };
            },
            raw = socket.recv() => {
                if let Some(Ok(Message::Text(message))) = raw {
                    if let Ok(interaction) = serde_json::from_str::<AdminInteraction>(&message) {
                        debug!("Got admin interaction {:?}", interaction);
                        let mut game = state.write().await;
                        match interaction {
                            AdminInteraction::Selection { id } => if let Phase::Selection = game.phase {
                                let id = match TrackId::from_uri(&id) {
                                    Ok(id) => id,
                                    Err(e) => {
                                        debug!("Failed to construct Spotify ID from URI: {:?}", e);
                                        continue;
                                    }
                                };
                                match game.spotify.track(id.clone(), None).await {
                                    Ok(track) => {
                                        if let Err(e) = game.spotify.start_uris_playback(vec![id.into()], None, None, None).await {
                                            debug!("Failed to start playback: {:?}", e);
                                            continue;
                                        }
                                        game.song.replace(track);
                                        game.phase = Phase::Listening { active: game.teams.iter().map(|_| true).collect() };
                                        let _ = game.channel.send(Default::default());
                                    },
                                    Err(e) => {
                                        debug!("Failed to get track info: {:?}", e);
                                        continue;
                                    }
                                };
                            },
                            AdminInteraction::CreateTeam => if let Phase::Selection = game.phase {
                                let team = Team {
                                    name: random_name(),
                                    points: 0
                                };
                                info!("Team created: {:?}", &team);
                                game.teams.push(team);
                                let _ = game.channel.send(Default::default());
                            },
                            AdminInteraction::RenameTeam { team, name } => if let Phase::Selection = game.phase {
                                if team < game.teams.len() {
                                    let former = game.teams[team].clone();
                                    game.teams[team].name = name;
                                    info!("Team renamed: former={:?} current={:?}", former, game.teams[team]);
                                    let _ = game.channel.send(Default::default());
                                }
                            },
                            AdminInteraction::DeleteTeam { team } => if let Phase::Selection = game.phase {
                                if team < game.teams.len() {
                                    let removed = game.teams.remove(team);
                                    info!("Team removed: {:?}", removed);
                                    let _ = game.channel.send(Update {
                                        teams_invalidated: true,
                                        ..Default::default()
                                    });
                                }
                            },
                            AdminInteraction::StopListening => if let Phase::Listening { .. } = game.phase {
                                if let Err(e) = game.spotify.resume_playback(None, None).await {
                                    debug!("Spotify returned error {e}, likely not authorized");
                                }
                                game.phase = Phase::Selection;
                                let _ = game.channel.send(Default::default());
                            },
                            AdminInteraction::AcceptGuess => if let Phase::Guessing { team, .. } = game.phase {
                                if let Err(e) = game.spotify.resume_playback(None, None).await {
                                    debug!("Spotify returned error {e}, likely not authorized");
                                }
                                game.teams[team].points += 1;
                                game.phase = Phase::Selection;
                                let _ = game.channel.send(Default::default());
                            },
                            AdminInteraction::RejectGuess => if let Phase::Guessing { team, active } = &game.phase {
                                if let Err(e) = game.spotify.resume_playback(None, None).await {
                                    debug!("Spotify returned error {e}, likely not authorized");
                                }
                                let actives = active.iter().enumerate().map(|(n, toggle)| {
                                    if n == *team {
                                        false
                                    } else {
                                        *toggle
                                    }
                                }).collect::<Vec<bool>>();
                                if actives.iter().any(|f| *f) {
                                    game.phase = Phase::Listening { active: actives };
                                } else {
                                    game.phase = Phase::Selection;
                                }
                                let _ = game.channel.send(Default::default());
                            },
                        };

                    }
                } else {
                    debug!("Connection closed by peer at {:?}, exiting handler", addr);
                    break;
                }
            }
        }
    }
}
