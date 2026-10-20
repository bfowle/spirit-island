//! The turn structure and the whole state at one moment.

use serde::{Deserialize, Serialize};

use crate::decks::{BlightState, FearState, InvaderDeck};
use crate::island::Island;
use crate::setup::GameSetup;
use crate::spirit::SpiritState;

/// The engine's state names, in turn order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Phase {
    PlayerSetup,
    InvadersStartingAction,
    SpiritPhase,
    FastPower,
    BlightedIsland,
    Event,
    Fear,
    /// England's extra Build slot, resolved before the Ravage.
    HighImmigration,
    Ravage,
    Build,
    Explore,
    AdvanceInvaderCards,
    SlowPower,
    TimePasses,
    GameOver,
    Other,
}

impl Phase {
    pub fn parse(s: &str) -> Phase {
        match s.split('/').next().unwrap_or(s) {
            "PlayerSetup" => Phase::PlayerSetup,
            "InvadersStartingAction" => Phase::InvadersStartingAction,
            "SpiritPhase" => Phase::SpiritPhase,
            "FastPower" => Phase::FastPower,
            "BlightedIsland" => Phase::BlightedIsland,
            "Event" => Phase::Event,
            "Fear" => Phase::Fear,
            "HighImmigration" => Phase::HighImmigration,
            "Ravage" => Phase::Ravage,
            "Build" => Phase::Build,
            "Explore" => Phase::Explore,
            "AdvanceInvaderCards" => Phase::AdvanceInvaderCards,
            "SlowPower" => Phase::SlowPower,
            "TimePasses" => Phase::TimePasses,
            "GameOver" => Phase::GameOver,
            _ => Phase::Other,
        }
    }

    pub fn is_invader_step(self) -> bool {
        matches!(self, Phase::HighImmigration | Phase::Ravage | Phase::Build | Phase::Explore)
    }

    pub fn name(self) -> &'static str {
        match self {
            Phase::PlayerSetup => "Setup",
            Phase::InvadersStartingAction => "Invaders setup",
            Phase::SpiritPhase => "Spirit",
            Phase::FastPower => "Fast",
            Phase::BlightedIsland => "Blighted Island",
            Phase::Event => "Event",
            Phase::Fear => "Fear",
            Phase::HighImmigration => "HighImmigration",
            Phase::Ravage => "Ravage",
            Phase::Build => "Build",
            Phase::Explore => "Explore",
            Phase::AdvanceInvaderCards => "Advance",
            Phase::SlowPower => "Slow",
            Phase::TimePasses => "Time Passes",
            Phase::GameOver => "Game over",
            Phase::Other => "?",
        }
    }
}

/// A moment: turn number (1-based) and phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Step {
    pub turn: u32,
    pub phase: Phase,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T{} {}", self.turn, self.phase.name())
    }
}

/// How a game ended, as the engine names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ending {
    WinInvadersDestroyed,
    WinTerrorVictory,
    LoseTooMuchBlight,
    LoseAdversary,
    LoseInvaderDeck,
    LosePresence,
    Other,
}

impl Ending {
    pub fn parse(s: &str) -> Ending {
        match s {
            "WinInvadersDestroyed" => Ending::WinInvadersDestroyed,
            "WinTerrorVictory" => Ending::WinTerrorVictory,
            "LoseTooMuchBlight" => Ending::LoseTooMuchBlight,
            "LoseAdversary" => Ending::LoseAdversary,
            "LoseInvaderDeck" | "LoseInvaderCards" => Ending::LoseInvaderDeck,
            "LosePresence" | "LoseNoPresence" => Ending::LosePresence,
            _ => Ending::Other,
        }
    }

    pub fn won(self) -> bool {
        matches!(self, Ending::WinInvadersDestroyed | Ending::WinTerrorVictory)
    }

    pub fn words(self) -> &'static str {
        match self {
            Ending::WinInvadersDestroyed => "won: Invaders destroyed",
            Ending::WinTerrorVictory => "won: Terror victory",
            Ending::LoseTooMuchBlight => "lost: too much Blight",
            Ending::LoseAdversary => "lost: adversary condition",
            Ending::LoseInvaderDeck => "lost: Invader deck ran out",
            Ending::LosePresence => "lost: no presence",
            Ending::Other => "ended",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub setup: GameSetup,
    pub turn: u32,
    pub phase: Option<Phase>,
    pub island: Island,
    pub spirits: Vec<SpiritState>,
    pub invaders: InvaderDeck,
    pub fear: FearState,
    pub blight: BlightState,
    #[serde(default)]
    pub status: Vec<String>,
    pub ending: Option<Ending>,
}

impl GameState {
    pub fn step(&self) -> Step {
        Step { turn: self.turn, phase: self.phase.unwrap_or(Phase::Other) }
    }

    pub fn spirit(&self, id: &str) -> Option<&SpiritState> {
        self.spirits.iter().find(|s| s.id == id)
    }

    pub fn spirit_mut(&mut self, id: &str) -> Option<&mut SpiritState> {
        self.spirits.iter_mut().find(|s| s.id == id)
    }
}
