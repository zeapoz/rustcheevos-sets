use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;

use crate::mem;

pub struct Game;

impl Game {
    pub fn in_game() -> Condition {
        mem::in_game_flag().eq(InGameState::InGame as u32)
    }

    pub fn in_menu() -> Condition {
        mem::in_game_flag().eq(InGameState::InMenu as u32)
    }
}

pub enum InGameState {
    InMenu = 0x0,
    InGame = 0x1,
}
