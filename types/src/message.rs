use serde::{Deserialize, Serialize};

use crate::{
    game::{Phase, Team},
    track::Track,
};

/// An update to a spectator.
///
/// This represents the part of the state the spectator sees
/// and is sent by the server to each spectator on an update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpectatorUpdate {
    /// Which phase the game is currently in.
    pub phase: Phase,
    /// Which teams currently exist.
    pub teams: Vec<Team>,
    /// Which song is currently revealed
    pub revealed: Option<Track>,
}

/// An update to a buzzer.
///
/// This represents the part of the state the buzzer sees
/// and is sent by the server to each buzzer on an update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuzzerUpdate {
    /// Which phase the game is currently in.
    pub phase: Phase,
    /// Which teams currently exist.
    pub teams: Vec<Team>,
    /// Which team this buzzer is registered to.
    pub registered: Option<usize>,
}

/// An interaction from a buzzer.
///
/// This represents an action a buzzer can take and
/// is sent by the buzzer to the server on a user
/// interaction on the buzzer side.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuzzerInteraction {
    /// The buzzer wants to register for the team.
    Register { team: usize },
    /// The buzzer wants to buzz for its registered team.
    Buzz,
}

/// An update to an admin.
///
/// This represents the part of the state the admin sees
/// and is sent by the server to each admin on an update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdminUpdate {
    /// Which phase the game is currently in.
    pub phase: Phase,
    /// Which teams currently exist.
    pub teams: Vec<Team>,
    /// Which song is currently or was selected.
    pub song: Option<Track>,
}

/// An interaction from an admin.
///
/// This represents a possible action take by an admin
/// and is sent from an admin to the server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdminInteraction {
    /// The admin has finished the selection phase and
    /// has decided on a song with the given Spotify ID.
    Selection { id: String },
    /// The admin has stopped a listening phase
    StopListening,
    /// The admin has accepted a given guess
    AcceptGuess,
    /// The admin has reject a given guess
    RejectGuess,
    /// The admin has created a new team.
    CreateTeam,
    /// The admin has renamed an existing team
    RenameTeam { team: usize, name: String },
    /// The admin has delted an existing team
    DeleteTeam { team: usize },
}
