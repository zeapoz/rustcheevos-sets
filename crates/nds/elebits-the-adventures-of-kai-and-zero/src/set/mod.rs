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
    types::{game_state::GameState, location::Location},
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
    set.push(warning_achievement());
    set
}

fn win_condition() -> Achievement {
    Achievement::builder("Journey's End")
        .description("Defeat Mobius and reunite with your best friend")
        .core(chain!(
            delta!(mem::current_game_scene().eq(Location::IceWorld.id())),
            // TODO: Consider constraining further with the Sub-Scene.
            delta!(mem::game_state().eq(GameState::Overworld.id())),
            mem::game_state().eq(GameState::Credits.id())
        ))
        .points(25)
        .tag(Tag::WinCondition)
        .id(626355)
        .badge_id(712189)
        .build()
}

fn warning_achievement() -> Achievement {
    Achievement::builder("WARNING: BizHawk Only!")
        .description(
            "This set currently only works on BizHawk due to unexposed memory in other emulators",
        )
        .core(chain!(
            delta!(mem::game_state().eq(GameState::Startup.id())),
            mem::game_state().eq(GameState::MenuCutscene.id()),
        ))
        .id(626491)
        .badge_id(712446)
        .build()
}
