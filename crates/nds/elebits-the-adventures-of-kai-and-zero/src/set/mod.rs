use rustcheevos::{
    chain, delta,
    prelude::*,
    types::{
        achievement::{Achievement, Tag},
        game::AchievementSet,
    },
};

use crate::{
    mem,
    set::{
        boss::generate_boss_achievements, collection::generate_collection_achievements,
        items::generate_items_achievements, omega::generate_omega_achievements,
    },
    types::{game_state::GameState, location::Location},
};

mod boss;
mod collection;
mod items;
mod omega;

pub fn generate_set() -> AchievementSet {
    let mut set = AchievementSet::new();
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
        .requirements(chain!(
            delta!(mem::current_game_scene().eq(Location::IceWorld as u32)),
            // TODO: Consider constraining further with the Sub-Scene.
            delta!(GameState::in_overworld()),
            mem::game_state().eq(GameState::Credits as u32)
        ))
        .tag(Tag::WinCondition)
        .build()
}
