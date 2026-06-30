use rustcheevos::{
    add_source, chain, measured,
    prelude::*,
    types::{
        chain::{Chain, ChainGroup},
        requirement::Condition,
        rich::{BuiltInMacro, LookupTable, RichPresence},
    },
};

use crate::types::drone::DroneBehaviour;
use crate::types::galaxy::Galaxy;
use crate::types::game::{Game, MenuState};
use crate::types::planet::Planet;

/// Returns a chain that returns the current level lookup key.
fn level_lookup_key() -> Chain {
    chain!(
        add_source!(Game::current_galaxy_index().mul(10)),
        measured!(Game::current_level_index())
    )
}

/// Returns a chain for when to show the watching credits display.
fn credits_cond() -> Chain {
    Game::menu_state().eq(MenuState::Credits as u32).into()
}

/// Returns a chain for when to show the preparing for battle display.
fn preparing_cond() -> Chain {
    Game::menu_state()
        .eq(MenuState::DroneSelectOrResults as u32)
        .into()
}

/// Returns a chain group for when to show the exploring the universe display.
fn exploring_cond() -> ChainGroup {
    let mut group = ChainGroup::new(Condition::always_true());
    group.push_alt_group(Game::menu_state().eq(MenuState::GalaxySelect as u32));
    group.push_alt_group(Game::menu_state().eq(MenuState::PlanetSelect as u32));
    group
}

/// Returns a chain group for when to show the in menu display.
fn menu_cond() -> ChainGroup {
    let mut group = ChainGroup::new(Condition::always_true());
    for menu in MenuState::all_named() {
        group.push_alt_group(Game::menu_state().eq(*menu as u32));
    }
    group
}

/// Generates and returns the level lookup data.
fn build_menu_lookup_data() -> Vec<(u32, &'static str)> {
    MenuState::all_named()
        .iter()
        .map(|m| (*m as u32, m.name().expect("state should have name")))
        .collect()
}

/// Generates and returns the level lookup data.
fn build_level_lookup_data() -> Vec<(u32, &'static str)> {
    Planet::all()
        .iter()
        .map(|p| (p.lookup_key(), p.name))
        .collect()
}

/// Generates and returns the galaxy lookup data.
fn build_galaxy_lookup_data() -> Vec<(u32, &'static str)> {
    Galaxy::all()
        .iter()
        .map(|g| (*g as u32, g.shorthand()))
        .collect()
}

/// Generates and returns the drone lookup data.
fn build_drone_lookup_data() -> Vec<(u32, &'static str)> {
    DroneBehaviour::all()
        .iter()
        .map(|d| (*d as u32, d.shorthand()))
        .collect()
}

/// Generates and returns the rich presence.
pub fn generate_rich_presence() -> RichPresence {
    let mut rich_presence = RichPresence::new();

    let current_state = rich_presence.register_lookup(
        LookupTable::new("State")
            .with_entry((MenuState::InGame as u32, "Battling"))
            .with_fallback("Idling"),
        Game::menu_state(),
    );

    let current_level = rich_presence.register_lookup(
        LookupTable::new("Level").with_entries(build_level_lookup_data()),
        level_lookup_key(),
    );

    let current_galaxy = rich_presence.register_lookup(
        LookupTable::new("Galaxy").with_entries(build_galaxy_lookup_data()),
        Game::current_galaxy_index(),
    );

    let current_drone = rich_presence.register_lookup(
        LookupTable::new("Drone").with_entries(build_drone_lookup_data()),
        Game::in_game_drone_behaviour(),
    );

    let menu_name = rich_presence.register_lookup(
        LookupTable::new("Menu").with_entries(build_menu_lookup_data()),
        Game::menu_state(),
    );

    let lives = rich_presence.builtin_macro(BuiltInMacro::Number, Game::in_game_lives());
    let bombs = rich_presence.builtin_macro(BuiltInMacro::Number, Game::in_game_bombs());
    let game_score = rich_presence.builtin_macro(BuiltInMacro::Number, Game::in_game_score());
    let score_multiplier =
        rich_presence.builtin_macro(BuiltInMacro::Number, Game::in_game_score_multiplier());
    let total_geoms = rich_presence.builtin_macro(BuiltInMacro::Number, Game::total_geoms());

    rich_presence.add_conditional_display(
        chain!(Game::in_game_cond(), Game::not_in_retro_evolved_cond()),
        format!(
            "{current_state} on {current_level} ({current_galaxy}) with {current_drone} 🛰️ | {lives}x♥️ {bombs}x💣 | {game_score} (x{score_multiplier})"
        ),
    );
    rich_presence.add_conditional_display(
        chain!(Game::in_game_cond(), Game::in_retro_evolved_cond()),
        format!(
            "{current_state} in Retro Evolved | {lives}x♥️ {bombs}x💣 | {game_score} (x{score_multiplier})"
        ),
    );
    rich_presence.add_conditional_display(credits_cond(), "Watching the credits 🏆");
    rich_presence.add_conditional_display(
        preparing_cond(),
        format!("Preparing for a battle on {current_level} ({current_galaxy}) 🌑"),
    );
    rich_presence.add_conditional_display(
        exploring_cond(),
        format!("Exploring the Universe 🌌 | {total_geoms} Geoms"),
    );
    rich_presence.add_conditional_display(
        menu_cond(),
        format!("Stargazing in the {menu_name} Menu ✨"),
    );
    rich_presence.add_static_display("Playing Geometry Wars: Galaxies");
    rich_presence
}
