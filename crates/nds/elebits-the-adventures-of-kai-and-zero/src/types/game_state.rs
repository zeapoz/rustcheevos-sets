use rustcheevos::{chain, prelude::*, types::chain::Chain};

use crate::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Startup = 0x00,
    MenuCutscene = 0x01,
    IntroCutscene = 0x02,
    MainMenu = 0x03,
    FileSelect = 0x04,
    FileConfiguration = 0x05,
    FileMenu = 0x06,
    Extras = 0x07,
    MultiplayerMenu = 0x08,
    MulticardPlay = 0x09,
    MultiplayerWifiMenu = 0x0a,
    DownloadAdditionalOmegas = 0x0b,
    Overworld = 0x0c,
    InBossFight = 0x0e,
    EndingCutscene = 0x10,
    TransitioningWorldCutscene = 0x11,
    Credits = 0x12,
    GameOver = 0x13,
}

impl GameState {
    pub fn lookup_value(&self) -> &'static str {
        match self {
            GameState::Startup => "Booting up the game",
            GameState::MenuCutscene | GameState::IntroCutscene => "Watching a cutscene",
            GameState::MainMenu => "In the main menu",
            GameState::FileSelect | GameState::FileConfiguration => "Selecting a file",
            GameState::FileMenu => "Browsing the menus",
            GameState::Extras => "Looking at the extras section",
            GameState::MultiplayerMenu
            | GameState::MulticardPlay
            | GameState::MultiplayerWifiMenu => "In the multiplayer menu",
            GameState::DownloadAdditionalOmegas => "Downloadinging additional Omegas",
            GameState::Overworld | GameState::InBossFight => unreachable!(),
            GameState::EndingCutscene => "Watching a special cutscene",
            GameState::TransitioningWorldCutscene => "Kai and G.G. are travelling to a new world",
            GameState::Credits => "Watching the credits and reminiscing about the journey",
            GameState::GameOver => "Kai is hurt and contemplating whether to continue",
        }
    }

    pub fn in_overworld() -> Chain {
        chain!(mem::game_state().eq(Self::Overworld as u32))
    }

    pub fn in_boss_fight() -> Chain {
        chain!(mem::game_state().eq(Self::InBossFight as u32))
    }

    pub fn all_named() -> &'static [Self] {
        &[
            Self::Startup,
            Self::MenuCutscene,
            Self::IntroCutscene,
            Self::MainMenu,
            Self::FileSelect,
            Self::FileConfiguration,
            Self::FileMenu,
            Self::Extras,
            Self::MultiplayerMenu,
            Self::MulticardPlay,
            Self::MultiplayerWifiMenu,
            Self::DownloadAdditionalOmegas,
            Self::EndingCutscene,
            Self::TransitioningWorldCutscene,
            Self::Credits,
            Self::GameOver,
        ]
    }
}
