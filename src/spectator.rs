use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use log::debug;
use tokio::sync::RwLock;
use types::{game::Phase, message::SpectatorUpdate};

use crate::game::Game;

pub async fn spectator_upgrade(
    State(state): State<Arc<RwLock<Game>>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    debug!(
        "Got buzzer connection from {:?} at {:?}, entering handler",
        user_agent, addr
    );
    ws.on_upgrade(move |socket| spectator_handler(state, socket, addr))
}

async fn spectator_handler(state: Arc<RwLock<Game>>, mut socket: WebSocket, addr: SocketAddr) {
    let mut receiver = {
        let game = state.read().await;
        let receiver = game.channel.subscribe();
        let _ = game.channel.send(Default::default());
        receiver
    };
    while let Ok(_) = receiver.recv().await {
        let update = {
            let game = state.read().await;
            SpectatorUpdate {
                phase: game.phase.clone(),
                teams: game.teams.clone(),
                revealed: match &game.phase {
                    Phase::Selection => game.song.clone().map(|t| t.into()),
                    _ => None,
                },
            }
        };
        if let Err(_) = socket
            .send(Message::text(serde_json::to_string(&update).unwrap()))
            .await
        {
            debug!("Connection closed by peer at {:?}, exiting handler", addr);
            break;
        };
    }
    debug!("Internal channel closed, exiting handler");
}
