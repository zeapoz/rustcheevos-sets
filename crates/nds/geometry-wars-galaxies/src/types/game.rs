use rustcheevos::{
    bits8, bits16, bits32, chain, delta, measured_if,
    prelude::*,
    types::{chain::Chain, memory::MemoryRef, requirement::Condition},
};

pub const PROFILE_STRIDE: u32 = 0xb8;
pub const RETRO_EVOLVED_ID: u32 = 0xffffffff;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    RetroEvolvedMenu = 0x00,
    GalaxySelect = 0x01,
    PlanetSelect = 0x02,
    DroneSelectOrResults = 0x03,
    ProfileSelect = 0x04,
    HighScores = 0x06,
    Credits = 0x0a,
    MainMenu = 0x0f,
    InGame = 0x10,
    InGamePauseScreen = 0x11,
    OptionsMenu = 0x13,
    AudioMenu = 0x14,
    PreferencesMenu = 0x15,
    Startup = 0x16,
}

pub struct Game;

impl Game {
    pub const MAX_MULTIPLIER: u32 = 150;

    /// Returns the current menu state.
    pub fn menu_state() -> MemoryRef {
        bits8!(0x1a54c0)
    }

    /// Returns the current in-game state.
    pub fn in_game_state() -> MemoryRef {
        bits8!(0x1a6990)
    }

    /// Returns the current profile.
    pub fn current_profile() -> MemoryRef {
        bits8!(0x1a8c94)
    }

    /// Returns the total number of geoms collected.
    pub fn total_geoms() -> MemoryRef {
        bits32!(0x1aaf24)
    }

    /// Returns the current galaxy index.
    pub fn current_galaxy_index() -> MemoryRef {
        bits32!(0x1d3db8)
    }

    /// Returns the current level index.
    pub fn current_level_index() -> MemoryRef {
        bits32!(0x183584)
    }

    /// Returns the in-game drone behaviour.
    pub fn in_game_drone_behaviour() -> MemoryRef {
        bits16!(0x3412e4)
    }

    /// Returns the number of bombs.
    pub fn in_game_bombs() -> MemoryRef {
        bits32!(0x31d040)
    }

    /// Returns the in-game score.
    pub fn in_game_score() -> MemoryRef {
        bits32!(0x31d048)
    }

    /// Returns the number of geoms collected.
    pub fn in_game_geoms() -> MemoryRef {
        bits32!(0x31d050)
    }

    /// Returns the in-game score multiplier.
    pub fn in_game_score_multiplier() -> MemoryRef {
        bits32!(0x31d07c)
    }

    /// Returns the number of lives.
    pub fn in_game_lives() -> MemoryRef {
        bits32!(0x31d05c)
    }

    /// Returns the number of active player bullets.
    pub fn active_player_bullets() -> MemoryRef {
        bits32!(0x1a6940)
    }

    /// Returns a chain that checks if the player is playing Retro Evolved.
    pub fn in_retro_evolved_cond() -> Condition {
        Self::current_galaxy_index().eq(RETRO_EVOLVED_ID)
    }

    /// Returns a chain that checks if the player is not playing Retro Evolved.
    pub fn not_in_retro_evolved_cond() -> Condition {
        Self::current_galaxy_index().ne(RETRO_EVOLVED_ID)
    }

    /// Returns a chain that checks the in-game state.
    pub fn in_game_cond() -> Chain {
        chain!(Self::in_game_state().eq(1))
    }

    /// Returns a chain that checks the in-game state with delta.
    pub fn in_game_cond_with_delta() -> Chain {
        chain!(delta!(Self::in_game_cond()), Self::in_game_cond())
    }

    /// Returns a chain that checks the in-game state with delta and measures if.
    pub fn in_game_cond_with_delta_and_measured_if() -> Chain {
        measured_if!(Self::in_game_cond_with_delta())
    }

    /// Returns a delta-mem chain that checks if the in-game score reached the given target.
    pub fn target_score_cond(target: u32) -> Chain {
        chain!(
            delta!(Game::in_game_score()).lt(target),
            Game::in_game_score().ge(target),
        )
    }
}
