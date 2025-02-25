use serde::{Deserialize, Serialize};

/// The Phase a game is currently in.
///
/// This represents what phase a running game currently is.
/// These are disjoint and describe fully all states.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Phase {
    /// The admin is currently selecting a song to play.
    #[default]
    Selection,
    /// The teams are listening to the currently playing song
    /// but no-one has guessed yet.
    Listening { active: Vec<bool> },
    /// One team has guessed a song and the admin is currently
    /// deliberating whether it is correct or incorrect
    Guessing { active: Vec<bool>, team: usize },
}

impl Phase {
    /// Check if the team at index is active
    ///
    /// This has different meanings depending on the game phase
    /// - In the selection phase, any team is always active
    /// - In the listening phase, only the teams who have not been disabled are active
    /// - In the guessing phase, only the team currently guessing is active
    ///
    /// # Examples
    ///
    /// ```
    /// use types::game::Phase;
    ///
    /// assert_eq!(Phase::Selection.is_active(0), true);
    /// assert_eq!(Phase::Listening{active: vec![false, false, true]}.is_active(0), false);
    /// assert_eq!(Phase::Guessing{active: vec![], team: 6}.is_active(6), true);
    /// assert_eq!(Phase::Guessing{active: vec![], team: 6}.is_active(2), false);
    /// ```
    pub fn is_active(&self, index: usize) -> bool {
        match self {
            Self::Selection => true,
            Self::Listening { active } => {
                if let Some(value) = active.get(index) {
                    *value
                } else {
                    false
                }
            }
            Self::Guessing { team, .. } => *team == index,
        }
    }
}

/// An in-game team.
///
/// This stores all data relating to a team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub name: String,
    pub points: usize,
}
