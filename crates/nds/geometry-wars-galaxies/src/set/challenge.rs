use rustcheevos::{
    add_source, chain, delta, pause_if,
    prelude::*,
    reset_next_if, trigger,
    types::{achievement::Achievement, chain::Chain, game::AchievementSet, requirement::Condition},
};

use crate::types::{
    drone::DroneBehaviour,
    game::Game,
    planet::Planet,
    planet::{BATEIS, CLAEIS, CLAHEX, FLIHEX, ORBEIS, SURPENTE, SURTETRA, VARPENTE, VIRDUO},
    status::MedalStatus,
};

/// Returns a condition that resets the next item if the player is not in a game.
fn reset_next_if_not_in_game() -> Condition {
    reset_next_if!(Game::in_game_state().ne(1))
}

/// Returns a chain that pauses when the active player has fired bullets.
fn no_bombs_used() -> Chain {
    chain!(
        reset_next_if_not_in_game(),
        add_source!(1),
        pause_if!(Game::in_game_bombs().eq(delta!(Game::in_game_bombs()))).with_hits(1)
    )
}

/// Returns a chain that pauses when the active player has lost lives.
fn no_lives_lost() -> Chain {
    chain!(
        reset_next_if_not_in_game(),
        add_source!(1),
        pause_if!(Game::in_game_lives().eq(delta!(Game::in_game_lives()))).with_hits(1)
    )
}

/// Returns a chain that pauses when the active player has fired bullets.
fn no_bullets_fired() -> Chain {
    chain!(
        reset_next_if_not_in_game(),
        pause_if!(delta!(Game::active_player_bullets()).lt(Game::active_player_bullets()))
            .with_hits(1)
    )
}

/// Returns a chain that trigger wen the geoms collected is at least the given count.
fn min_geoms(count: u32) -> Chain {
    chain!(
        delta!(Game::in_game_geoms()).lt(count),
        trigger!(Game::in_game_geoms().ge(count)),
    )
}

/// Returns a chain that triggers when the game score is at least the given target.
fn score_threshold(planet: &Planet, status: MedalStatus) -> Chain {
    let target = planet.required_score(status);
    chain!(
        delta!(Game::in_game_score()).lt(target),
        trigger!(Game::in_game_score().ge(target)),
    )
}

/// Returns a chain that requires the player to be on the given planet.
fn planet_condition(planet: &Planet) -> Chain {
    chain!(planet.player_in_planet(), Game::in_game_cond_with_delta(),)
}

/// Returns a chain that requires the given drone behaviour to be active.
fn drone_condition(drone: DroneBehaviour) -> Condition {
    Game::in_game_drone_behaviour().eq(drone as u32)
}

/// Adds the Retro Evolved achievements to the set.
#[rustfmt::skip]
pub fn add_retro_evolved_achievements(set: &mut AchievementSet) {
    set.push(retro_evolved_score( 600749, "Retro Rookie", 100_000, "100,000", 5));
    set.push(retro_evolved_score( 600750, "Retro Rising", 250_000, "250,000", 10));
    set.push(retro_evolved_score( 600751, "Retro Rival", 500_000, "500,000", 25));
    set.push(retro_evolved_score( 600752, "Retro Radiant", 1_000_000, "1,000,000", 50));
}

/// Returns an achievement for earning a score the Retro Evolved.
fn retro_evolved_score(
    id: u32,
    title: &str,
    target: u32,
    target_str: &str,
    points: u32,
) -> Achievement {
    Achievement::builder(title)
        .description(format!(
            "In Retro Evolved, earn a score of {target_str} points or higher"
        ))
        .requirements(chain!(
            Game::target_score_cond(target),
            Game::in_retro_evolved_cond(),
            Game::in_game_cond_with_delta()
        ))
        .points(points)
        .id(id)
        .build()
}

/// Adds the challenge achievements to the set.
pub fn add_galaxy_challenge_achievements(set: &mut AchievementSet) {
    set.push(
        Achievement::builder("Going Round and Round")
            .description(
                "Acquire a 150x multiplier in Claeis using the Sweep Drone and without firing",
            )
            .requirements(chain!(
                delta!(Game::in_game_score_multiplier()).lt(Game::MAX_MULTIPLIER),
                trigger!(Game::in_game_score_multiplier().eq(Game::MAX_MULTIPLIER)),
                no_bullets_fired(),
                CLAEIS.player_in_planet(),
                Game::in_game_cond_with_delta(),
                Game::in_game_drone_behaviour().eq(DroneBehaviour::Sweep as u32),
            ))
            .points(10)
            .id(600753)
            .build(),
    );

    set.push(
        Achievement::builder("Lossless Defence")
            .description(
                "Earn a Gold medal in Bateis using the Defend Drone and without using any bombs",
            )
            .requirements(chain!(
                score_threshold(&BATEIS, MedalStatus::Gold),
                no_bombs_used(),
                planet_condition(&BATEIS),
                drone_condition(DroneBehaviour::Defend),
            ))
            .points(10)
            .id(600754)
            .build(),
    );

    set.push(
        Achievement::builder("Going with the Flow")
            .description(
                "Earn a Gold medal in Orbeis using the Bait Drone and without using any bombs",
            )
            .requirements(chain!(
                score_threshold(&ORBEIS, MedalStatus::Gold),
                no_bombs_used(),
                planet_condition(&ORBEIS),
                drone_condition(DroneBehaviour::Bait),
            ))
            .points(10)
            .id(600755)
            .build(),
    );

    set.push(
        Achievement::builder("Ram Spam")
            .description("Earn a Bronze medal or higher in Surpente using the Ram Drone without firing or using bombs")
            .requirements(chain!(
                score_threshold(&SURPENTE, MedalStatus::Bronze),
                no_bullets_fired(),
                no_bombs_used(),
                planet_condition(&SURPENTE),
                drone_condition(DroneBehaviour::Ram),
            ))
            .points(10)
            .id(600756)
            .build(),
    );

    set.push(
        Achievement::builder("Back to the Basics")
            .description(
                "Earn a Gold medal in Clahex using the Attack Drone and without losing any lives",
            )
            .requirements(chain!(
                score_threshold(&CLAHEX, MedalStatus::Gold),
                no_lives_lost(),
                planet_condition(&CLAHEX),
                drone_condition(DroneBehaviour::Attack),
            ))
            .points(25)
            .id(600757)
            .build(),
    );

    set.push(
        Achievement::builder("Silent Sniper")
            .description(
                "Earn a Gold medal in Virduo using the Snipe Drone and without using any bombs",
            )
            .requirements(chain!(
                score_threshold(&VIRDUO, MedalStatus::Gold),
                no_bombs_used(),
                planet_condition(&VIRDUO),
                drone_condition(DroneBehaviour::Snipe),
            ))
            .points(10)
            .id(600758)
            .build(),
    );

    set.push(
        Achievement::builder("Fortify and Go")
            .description(
                "Earn a Gold medal in Surtetra using the Turret Drone and without losing any lives",
            )
            .requirements(chain!(
                score_threshold(&SURTETRA, MedalStatus::Gold),
                no_lives_lost(),
                planet_condition(&SURTETRA),
                drone_condition(DroneBehaviour::Turret),
            ))
            .points(10)
            .id(600759)
            .build(),
    );

    set.push(
        Achievement::builder("Grabbing Gold and Geoms")
            .description("Earn a Gold medal in Varpente using the Collect Drone while collecting at least 10,000 Geoms")
            .requirements(chain!(
                score_threshold(&VARPENTE, MedalStatus::Gold),
                min_geoms(10_000),
                planet_condition(&VARPENTE),
                drone_condition(DroneBehaviour::Collect),
            ))
            .points(10)
            .id(600760)
            .build(),
    );

    set.push(
        Achievement::builder("The Perfect Run")
            .description(
                "Earn a Gold medal in Flihex without losing any lives and without using any bombs",
            )
            .requirements(chain!(
                score_threshold(&FLIHEX, MedalStatus::Gold),
                no_lives_lost(),
                no_bombs_used(),
                planet_condition(&FLIHEX),
            ))
            .points(50)
            .id(600761)
            .build(),
    );
}
