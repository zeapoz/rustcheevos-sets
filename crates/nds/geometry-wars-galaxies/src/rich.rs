use rustcheevos::{
    add_source, chain, measured,
    prelude::*,
    types::{
        chain::{Chain, ChainGroup},
        requirement::Condition,
        rich::{FormatType, LookupTable, RichPresence},
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

/// Returns a chain group for when to show the exploring the universe display.
fn exploring_cond() -> ChainGroup {
    let mut group = ChainGroup::new(Condition::always_true());
    group.push_alt_group(Game::menu_state().eq(MenuState::GalaxySelect as u32));
    group.push_alt_group(Game::menu_state().eq(MenuState::PlanetSelect as u32));
    group
}

/// Generates and returns the level lookup data.
fn build_level_lookup_data() -> Vec<(u32, &'static str)> {
    Planet::all()
        .iter()
        .map(|p| (p.lookup_key(), p.name()))
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

    let lives = rich_presence.register_format("Lives", FormatType::Value, Game::in_game_lives());
    let bombs = rich_presence.register_format("Bombs", FormatType::Value, Game::in_game_bombs());
    let score = rich_presence.register_format("Score", FormatType::Value, Game::in_game_score());
    let score_multiplier = rich_presence.register_format(
        "ScoreMultiplier",
        FormatType::Value,
        Game::in_game_score_multiplier(),
    );
    let total_geoms =
        rich_presence.register_format("TotalGeoms", FormatType::Value, Game::total_geoms());

    rich_presence.add_conditional_display(
        chain!(Game::in_game_cond(), Game::not_in_retro_evolved_cond(),),
        format!(
            "Battling in {current_level} ({current_galaxy}) with {current_drone} 🛰️ | {lives}x♥️ {bombs}x💣 | {score} (x{score_multiplier})"
        ),
    );
    rich_presence.add_conditional_display(
        chain!(Game::in_game_cond(), Game::in_retro_evolved_cond()),
        format!(
            "Battling in Retro Evolved | {lives}x♥️ {bombs}x💣 | {score} (x{score_multiplier})"
        ),
    );
    rich_presence.add_conditional_display(
        exploring_cond(),
        format!("Exploring the Universe 🌌 | {total_geoms} Geoms"),
    );
    rich_presence.add_static_display("Stargazing in the Menu ✨");
    rich_presence
}
