use rustcheevos::{
    add_source, chain, delta, measured,
    prelude::*,
    reset_if,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
        game::AchievementSet,
        requirement::Condition,
    },
};

use crate::types::{
    drone::DroneBehaviour,
    game::{Game, MenuState},
};

/// Returns a condition that resets the achievment if the player is not in a game.
fn reset_if_not_in_game() -> Condition {
    reset_if!(Game::in_game_state().ne(1))
}

/// Adds the drone achievements to the set.
#[rustfmt::skip]
pub fn add_drone_achievements(set: &mut AchievementSet) {
    set.push(drone_reach_level_achievement(600736, 681326, "The Best Defence...", DroneBehaviour::Attack));
    set.push(drone_reach_level_achievement(600737, 681327, "The Best Offence...", DroneBehaviour::Defend));
    set.push(drone_reach_level_achievement(600738, 681328, "Heart of a Hoarder", DroneBehaviour::Collect));
    set.push(drone_reach_level_achievement(600739, 681329, "Programmed for Precision", DroneBehaviour::Snipe));
    set.push(drone_reach_level_achievement(600740, 681330, "Cleared for Cleanup", DroneBehaviour::Sweep));
    set.push(drone_reach_level_achievement(600741, 681331, "Harbinger of Havoc", DroneBehaviour::Ram));
    set.push(drone_reach_level_achievement(600742, 681332, "Pocket Protector", DroneBehaviour::Turret));
    set.push(drone_reach_level_achievement(600743, 681333, "Magnetic Machine", DroneBehaviour::Bait));

    let mut drone_max_level=  ChainGroup::new(Game::menu_state().eq(MenuState::DroneSelectOrResults as u32));
    for drone in DroneBehaviour::all() {    
        let chain = chain!(
            add_source!(1),
            delta!(drone.level()).eq(DroneBehaviour::MAX_LEVEL),
            drone.level().eq(DroneBehaviour::MAX_LEVEL),
        );
        drone_max_level.push_alt_group(chain);
    }
    set.push(
        Achievement::builder("I Choose You")
            .description("Get any Drone Behaviour to level 10")
            .requirements(drone_max_level)
            .points(25)
            .id(600744)
            .badge_id(681334)
            .build(),
    );


    let core = chain!(
        DroneBehaviour::all().iter().copied().map(DroneBehaviour::is_unlocked).collect::<Chain>(),
        Game::menu_state().eq(MenuState::DroneSelectOrResults as u32)
    );
    let mut all_drones_unlocked = ChainGroup::new(core);
    all_drones_unlocked.set_alt_groups(DroneBehaviour::all().iter().copied().map(DroneBehaviour::unlocked));

    set.push(
        Achievement::builder("Drone Hoarder")
            .description("Unlock every Drone Behaviour")
            .requirements(all_drones_unlocked)
            .points(5)
            .id(600745)
            .badge_id(681335)
            .build(),
    );
}

/// Returns an achievement for reaching a level with a drone.
fn drone_reach_level_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    drone: DroneBehaviour,
) -> Achievement {
    const LEVEL_TARGET: u32 = 5;
    Achievement::builder(title)
        .description(format!(
            "Get the {drone} Drone Behaviour to level {LEVEL_TARGET}"
        ))
        .requirements(chain!(
            add_source!(1),
            delta!(drone.level()).eq(LEVEL_TARGET),
            drone.level().eq(LEVEL_TARGET),
            Game::menu_state().eq(MenuState::DroneSelectOrResults as u32)
        ))
        .points(5)
        .id(id)
        .badge_id(badge_id)
        .build()
}

/// Adds the miscellaneous achievements to the set.
pub fn add_misc_achievements(set: &mut AchievementSet) {
    set.push(
        Achievement::builder("Maximum Overdrive")
            .description("Acquire a 150x multiplier")
            .requirements(chain!(
                delta!(Game::in_game_score_multiplier()).lt(Game::MAX_MULTIPLIER),
                Game::in_game_score_multiplier().eq(Game::MAX_MULTIPLIER),
                Game::in_game_cond_with_delta(),
            ))
            .points(3)
            .id(600746)
            .badge_id(681336)
            .build(),
    );

    set.push(
        Achievement::builder("Second Chance")
            .description("Earn an extra life in one level")
            .requirements(chain!(
                add_source!(1),
                delta!(Game::in_game_lives()).eq(Game::in_game_lives()),
                Game::in_game_cond_with_delta(),
            ))
            .points(2)
            .id(600747)
            .badge_id(681337)
            .build(),
    );

    set.push(
        Achievement::builder("Triple Life")
            .description("Earn three extra lives in one level")
            .requirements(chain!(
                add_source!(1),
                measured!(delta!(Game::in_game_lives()).eq(Game::in_game_lives())).with_hits(3),
                Game::in_game_cond_with_delta_and_measured_if(),
                reset_if_not_in_game(),
            ))
            .points(3)
            .id(600748)
            .badge_id(681338)
            .build(),
    );
}
