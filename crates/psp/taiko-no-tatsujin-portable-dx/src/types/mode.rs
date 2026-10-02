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

pub enum FreeplayState {
    SongSelect = 0x61,
    Playing = 0x62,
    Results = 0x63,
}

impl FreeplayState {
    pub fn is(state: Self) -> Condition {
        mem::current_menu_id().eq(state as u32)
    }
}
