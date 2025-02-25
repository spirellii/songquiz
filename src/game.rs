use rspotify::{model::FullTrack, AuthCodeSpotify};
use tokio::sync::broadcast::Sender;
use types::game::{Phase, Team};

/// An internal update.
///
/// This is sent to every handler on an internal state change
/// in a Game.
#[derive(Debug, Clone, Default)]
pub struct Update {
    pub teams_invalidated: bool,
}

/// The server-side representation of a game.
///
/// This includes pure game state as well as communication
/// channels to update websocket handlers and Spotify API.
/// handlers.
#[derive(Debug, Clone)]
pub struct Game {
    /// Which phase the game is currently in.
    pub phase: Phase,
    /// Which teams currently exist in-game.
    pub teams: Vec<Team>,
    /// Which song is currently selected.
    pub song: Option<FullTrack>,
    /// The channel which is used to communicate a change
    /// of the game state.
    pub channel: Sender<Update>,
    /// The Spotify API connection used to play songs.
    pub spotify: AuthCodeSpotify,
}
