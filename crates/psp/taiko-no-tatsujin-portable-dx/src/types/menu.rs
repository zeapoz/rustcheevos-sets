use rustcheevos::types::{requirement::Condition, value::TypedValueOps};

use crate::mem;

pub enum GameMode {
    Freeplay = 0x0,
    Omikoshi = 0x1,
    Medley = 0x2,
    DonChansRoom = 0x3,
    DownloadMode = 0x4,
    Multiplayer = 0x5,
    Settings = 0x6,
    MainMenu = 0x7,
}

impl GameMode {
    pub fn is(mode: Self) -> Condition {
        mem::game_mode().eq(mode as u32)
    }
}

pub enum MenuState {
    Freeplay(FreeplayState),
    Medley(MedleyState),
    Omikoshi(OmikoshiState),
    DonChansRoom(DonChansRoomState),
    General(GeneralMenuState),
}

impl MenuState {
    pub fn is(state: Self) -> Condition {
        let s = match state {
            MenuState::Freeplay(s) => s as u32,
            MenuState::Medley(s) => s as u32,
            MenuState::Omikoshi(s) => s as u32,
            MenuState::DonChansRoom(s) => s as u32,
            MenuState::General(s) => s as u32,
        };
        mem::current_menu_id().eq(s)
    }
}

pub enum FreeplayState {
    SongSelect = 0x61,
    Playing = 0x62,
    Results = 0x63,
}

pub enum MedleyState {
    SongSelect = 0x4c,
    Playing = 0x4d,
    Results = 0x4e,
}

pub enum OmikoshiState {
    SelectStartingPrefecture = 0x67,
    Cutscene = 0x68,
    Dojo = 0x6a,
    Map = 0x6b,
    Preparing = 0x6c,
    Battle = 0x6d,
    BattleResults = 0x6e,
    DojoTrainingResults = 0x70,
}

pub enum DonChansRoomState {
    Menu = 0x4f,
    ClothingMenu = 0x50,
    Mailbox = 0x51,
    Scoreboard = 0x52,
    AchievementList = 0x53,
    EverybodyData = 0x54,
    Tutorial = 0x55,
}

pub enum GeneralMenuState {
    SplashScreen = 0x44,
    TitleScreen = 0x45,
    MainMenu = 0x4b,
    ProfileSelect = 0x47,
    NameEntry = 0x48,
    Multiplayer = 0x56,
    Download = 0x5a,
    Settings = 0x5b,
    Mail = 0x65,
    Saving = 0x66,
    WarningScreen = 0x75,
}
