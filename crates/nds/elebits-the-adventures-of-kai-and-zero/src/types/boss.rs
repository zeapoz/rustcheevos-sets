use rustcheevos::{
    prelude::*,
    types::{
        chain::Chain,
        memory::MemoryRef,
        requirement::{Arithmetic, Condition},
    },
};

use crate::{
    mem,
    types::{game::Game, game_state::GameState, location::Location},
};

pub struct Boss;

impl Boss {
    pub const HEALTH_NUMERATOR_OFFSET: MemoryRef = bits16!(0x10);

    pub fn data_base_pointer() -> Chain<MemoryRef> {
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            bits24!(0x7c),
        )
    }

    pub fn null_pointer_check() -> Chain<Condition> {
        delta!(Boss::data_base_pointer()).ne(0)
    }

    pub fn data_pointer() -> Chain<Arithmetic> {
        chain!(
            add_address!(Boss::data_base_pointer()),
            add_address!(bits24!(0xa0)),
        )
    }

    pub fn health_numerator() -> Chain<MemoryRef> {
        chain!(Boss::data_pointer(), Self::HEALTH_NUMERATOR_OFFSET)
    }

    pub fn boss_defeated() -> Chain<Condition> {
        chain!(
            delta!(Boss::health_numerator()).ne(0),
            Boss::health_numerator().eq(0),
        )
    }

    pub fn in_boss_arena(location: Location) -> Chain<Condition> {
        chain!(
            mem::current_game_scene().eq(location.id()),
            mem::game_state().eq(GameState::InBossFight.id()),
            Game::in_game()
        )
    }

    pub fn timer() -> Chain<MemoryRef> {
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            add_address!(bits24!(0x6c)),
            bits32!(0x84)
        )
    }
}

pub struct PowerOmegaBoss;

impl PowerOmegaBoss {
    pub fn attack_state() -> Chain<MemoryRef> {
        chain!(Boss::data_pointer(), bits32!(0x2ac))
    }

    pub fn bounces_left() -> Chain<MemoryRef> {
        chain!(Boss::data_pointer(), bits16!(0x2b6))
    }
}

pub struct XFireOmegaBoss;

impl XFireOmegaBoss {
    pub fn attack_state() -> Chain<MemoryRef> {
        chain!(Boss::data_pointer(), bits16!(0x2f2))
    }
}

pub struct XIceOmegaBoss;

impl XIceOmegaBoss {
    pub const SNAKE_ATTACK_COUNTER_OFFSET: MemoryRef = bits16!(0x112);

    pub fn snake_attack_counter() -> Chain<MemoryRef> {
        chain!(
            add_address!(Boss::data_base_pointer()),
            Self::SNAKE_ATTACK_COUNTER_OFFSET
        )
    }
}

pub struct LeoBoss;

impl LeoBoss {
    pub const NEXT_ATTACK_PATTERN_OFFSET: MemoryRef = bits16!(0xf4);

    pub fn next_attack_pattern() -> Chain<MemoryRef> {
        chain!(
            add_address!(Boss::data_base_pointer()),
            Self::NEXT_ATTACK_PATTERN_OFFSET
        )
    }
}
