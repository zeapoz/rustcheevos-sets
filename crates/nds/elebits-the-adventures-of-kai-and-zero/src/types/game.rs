use rustcheevos::{prelude::TypedValueOps, types::chain::Chain};

use crate::mem;

pub struct Game;

impl Game {
    pub fn in_game() -> Chain {
        mem::in_game_flag().eq(InGameState::InGame as u32).into()
    }

    pub fn in_menu() -> Chain {
        mem::in_game_flag().eq(InGameState::InMenu as u32).into()
    }
}

pub enum InGameState {
    InGame = 0x0,
    InMenu = 0x1,
}
