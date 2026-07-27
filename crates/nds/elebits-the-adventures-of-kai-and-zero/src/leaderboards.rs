use rustcheevos::{
    bits32, chain, delta, measured,
    prelude::*,
    remember,
    types::{
        game::LeaderboardSet,
        leaderboard::{Leaderboard, LeaderboardFormat},
        value::TypedValue,
    },
};

use crate::{
    mem,
    types::{game::Game, game_state::GameState, location::Location},
    utils::{
        boss_data_null_pointer_check, boss_health_numerator, boss_timer_pointer,
        combo_text_pointer, combo_text_pointer_not_null,
    },
};

#[rustfmt::skip]
pub fn generate_leaderboards() -> LeaderboardSet {
    vec![
        combo_leaderboard(167921, Location::ElebitForest),
        combo_leaderboard(167922, Location::ElebitMine),
        combo_leaderboard(167923, Location::ResortIsland),
        combo_leaderboard(167924, Location::IceWorld),
        combo_leaderboard(167925, Location::RuinedWorld),
        combo_leaderboard(167926, Location::SeaTemple),
        combo_leaderboard(167927, Location::LibraOfCrystal),
        boss_time_attack(167928, Location::PowerOmegaBossArena, "Power Omega", "Defeat the Power Omega as fast as possible!"),
        boss_time_attack(167929, Location::EarthOmegaBossArena, "Earth Omega", "Defeat the Earth Omega as fast as possible!"),
        boss_time_attack(167930, Location::XFireOmegaBossArena, "X Fire Omega", "Defeat the X Fire Omega as fast as possible!"),
        boss_time_attack(167931, Location::XIceOmegaBossArena, "X Ice Omega", "Defeat the X Ice Omega as fast as possible!"),
        boss_time_attack(167932, Location::LeoBossArena, "Leo", "Defeat Leo as fast as possible!"),
        boss_time_attack(167933, Location::XEarthOmegaBossArena, "X Earth Omega", "Defeat the X Earth Omega as fast as possible!"),
        boss_time_attack(167934, Location::XWaterOmegaBossArena, "X Water Omega", "Defeat the X Water Omega as fast as possible!"),
        boss_time_attack(167935, Location::MobiusBossArena, "Mobius I", "Defeat the first phase of Mobius as fast as possible!"),
        boss_time_attack(167936, Location::MobiusSecondPhaseBossArena, "Mobius II", "Defeat the second phase of Mobius as fast as possible!"),
    ]
}

fn combo_leaderboard(id: u32, world: Location) -> Leaderboard {
    const COMBO_NUMBER_OFFSET: usize = 0x494;

    Leaderboard::builder(format!("Combo Chaser - {}", world.world_name()))
        .description(format!(
            "Acquire the highest combo counter in {}!",
            world.world_name()
        ))
        .value(chain!(
            combo_text_pointer(),
            measured!(bits32!(COMBO_NUMBER_OFFSET))
        ))
        .start(chain!(
            combo_text_pointer(),
            remember!(delta!(bits32!(COMBO_NUMBER_OFFSET))),
            combo_text_pointer(),
            bits32!(COMBO_NUMBER_OFFSET).gt(TypedValue::Recall),
            mem::current_game_scene().eq(world.id()),
            mem::game_state().eq(GameState::Overworld.id()),
            Game::in_game(),
            combo_text_pointer_not_null()
        ))
        .id(id)
        .build()
}

fn boss_time_attack(
    id: u32,
    boss_location: Location,
    boss_name_title: &str,
    description: &str,
) -> Leaderboard {
    Leaderboard::builder(format!("Time Attack - {}", boss_name_title))
        .description(description)
        .format(LeaderboardFormat::Frames)
        .lower_is_better(true)
        .value(chain!(boss_timer_pointer(), measured!(bits32!(0x84))))
        .start(chain!(
            boss_health_numerator(true).ne(0),
            boss_health_numerator(false).eq(0),
            mem::current_game_scene().eq(boss_location.id()),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .id(id)
        .build()
}
