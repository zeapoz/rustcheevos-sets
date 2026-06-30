use rustcheevos::{
    chain, delta, measured,
    prelude::*,
    types::{
        chain::Chain,
        game::LeaderboardSet,
        leaderboard::{Leaderboard, LeaderboardFormat},
    },
};

use crate::types::{
    game::{Game, RETRO_EVOLVED_ID},
    planet::Planet,
};

const PLANET_LB_START_ID: u32 = 161_021;

/// Returns a chain that checks if the player just died in a certain level.
fn leaderboard_condition(level_clause: impl Into<Chain>) -> Chain {
    chain!(
        delta!(Game::in_game_lives()).eq(1),
        Game::in_game_lives().eq(0),
        Game::in_game_score().ne(0),
        level_clause,
        Game::in_game_cond_with_delta(),
    )
}

/// Returns a leaderboard for the given planet.
fn planet_leaderboard(planet: &Planet, id: u32) -> Leaderboard {
    Leaderboard::builder(format!("{planet} ({})", planet.galaxy.name()))
        .description(format!("Earn the highest score on {planet}!"))
        .start(leaderboard_condition(planet.player_in_planet()))
        .value(measured!(Game::in_game_score()))
        .format(LeaderboardFormat::Value)
        .lower_is_better(false)
        .id(id)
        .build()
}

/// Returns all leaderboards.
pub fn generate_leaderboards() -> LeaderboardSet {
    let mut leaderboards: LeaderboardSet = (PLANET_LB_START_ID..)
        .zip(Planet::all())
        .map(|(id, p)| planet_leaderboard(p, id))
        .collect();

    let retro_evolved_id = PLANET_LB_START_ID + Planet::all().len() as u32;
    let retro_evolved = Leaderboard::builder("Retro Evolved")
        .description("Earn the highest score in Retro Evolved!")
        .start(leaderboard_condition(
            Game::current_galaxy_index().eq(RETRO_EVOLVED_ID),
        ))
        .value(measured!(Game::in_game_score()))
        .format(LeaderboardFormat::Value)
        .lower_is_better(false)
        .id(retro_evolved_id)
        .build();

    leaderboards.push(retro_evolved);
    leaderboards
}
