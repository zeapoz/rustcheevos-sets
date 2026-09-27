use rustcheevos::{
    prelude::*,
    types::achievement::{Achievement, Tag},
};

use crate::{
    mem,
    set::{
        boss::generate_boss_achievements, collection::generate_collection_achievements,
        items::generate_items_achievements, omega::generate_omega_achievements,
    },
    types::game_state::GameState,
};

mod boss;
mod collection;
mod items;
mod omega;

pub fn generate_set() -> Vec<Achievement> {
    let mut set = Vec::new();
    set.extend(generate_omega_achievements());
    set.extend(generate_boss_achievements());
    set.push(win_condition());
    set.extend(generate_collection_achievements());
    set.extend(generate_items_achievements());
    set
}

fn win_condition() -> Achievement {
    Achievement::builder("Journey's End")
        .description("Defeat Mobius and reunite with your best friend")
        .core(chain!(
            delta!(mem::game_state().eq(GameState::Overworld.id())),
            mem::game_state().eq(GameState::Credits.id()),
            mem::primary_event_state().eq(0x7),
            mem::secondary_event_state().eq(0x3),
        ))
        .points(25)
        .tag(Tag::WinCondition)
        .id(626355)
        .badge_id(712189)
        .build()
}
