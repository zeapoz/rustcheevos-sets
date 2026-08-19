use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::{
    mem,
    types::{game::Game, game_state::GameState, location::Location},
    utils::{
        BATTERIES_PER_WORLD, PINK_ELEBITS_PER_WORLD, world_battery_bits, world_pink_elebit_bits,
    },
};

#[rustfmt::skip]
pub fn generate_collection_achievements() -> Vec<Achievement> {
    let mut result = Vec::with_capacity(7);
    result.push(pink_elebits_achievement(626356, 712190, "Leafy Pink Locator", "Find all 3 Pink Elebits in the Elebit Forest", Location::ElebitForest));
    result.push(pink_elebits_achievement(626357, 712191, "Deep Pink Discovery", "Find all 3 Pink Elebits in the Elebit Mine", Location::ElebitMine));
    result.push(pink_elebits_achievement(626358, 712192, "Pink Paradise Hunter", "Find all 3 Pink Elebits in the Resort Island", Location::ResortIsland));
    result.push(pink_elebits_achievement(626359, 712193, "Chilled Pink Collector", "Find all 3 Pink Elebits in the Ice World", Location::IceWorld));
    result.push(pink_elebits_achievement(626360, 712194, "Flaming Pink Forager", "Find all 3 Pink Elebits in the Ruined World", Location::RuinedWorld));
    result.push(pink_elebits_achievement(626361, 712195, "Aquatic Pink Adventurer", "Find all 3 Pink Elebits in the Sea Temple", Location::SeaTemple));
    result.push(pink_elebits_achievement(626362, 712196, "Crystal Pink Connoisseur", "Find all 3 Pink Elebits in the Libra of Crystal", Location::LibraOfCrystal));

    result.push(battery_achievement(626363, 712197, "Wooded Watts", "Find all 6 Batteries in the Elebit Forest", Location::ElebitForest));
    result.push(battery_achievement(626364, 712198, "Lithic Watts", "Find all 6 Batteries in the Elebit Mine", Location::ElebitMine));
    result.push(battery_achievement(626365, 712199, "Tropical Watts", "Find all 6 Batteries in the Resort Island", Location::ResortIsland));
    result.push(battery_achievement(626366, 712200, "Glacial Watts", "Find all 6 Batteries in the Ice World", Location::IceWorld));
    result.push(battery_achievement(626367, 712201, "Volcanic Watts", "Find all 6 Batteries in the Ruined World", Location::RuinedWorld));
    result.push(battery_achievement(626368, 712202, "Tidal Watts", "Find all 6 Batteries in the Sea Temple", Location::SeaTemple));
    result.push(battery_achievement(626369, 712203, "Prismatic Watts", "Find all 6 Batteries in the Libra of Crystal", Location::LibraOfCrystal));

    result.push(guard_boosts_achievement("Fully Guarded", "Obtain all 13 Guard Boosts"));
    result
}

fn pink_elebits_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    world: Location,
) -> Achievement {
    let bits = world_pink_elebit_bits(world);

    Achievement::builder(title)
        .description(description)
        .core(chain!(
            add_source!(delta!(bits[0])),
            add_source!(delta!(bits[1])),
            delta!(bits[2]).eq(PINK_ELEBITS_PER_WORLD - 1),
            add_source!(bits[0]),
            add_source!(bits[1]),
            measured!(bits[2].eq(PINK_ELEBITS_PER_WORLD)),
            and_next!(mem::current_game_scene().eq(world.id())),
            or_next!(mem::game_state().eq(GameState::InBossFight.id())),
            measured_if!(mem::game_state().eq(GameState::Overworld.id())),
            Game::in_game(),
        ))
        .points(10)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn battery_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    world: Location,
) -> Achievement {
    let bits = world_battery_bits(world);

    Achievement::builder(title)
        .description(description)
        .core(chain!(
            add_source!(delta!(bits[0])),
            add_source!(delta!(bits[1])),
            add_source!(delta!(bits[2])),
            add_source!(delta!(bits[3])),
            add_source!(delta!(bits[4])),
            delta!(bits[5]).eq(BATTERIES_PER_WORLD - 1),
            add_source!(bits[0]),
            add_source!(bits[1]),
            add_source!(bits[2]),
            add_source!(bits[3]),
            add_source!(bits[4]),
            measured!(bits[5].eq(BATTERIES_PER_WORLD)),
            and_next!(mem::current_game_scene().eq(world.id())),
            or_next!(mem::game_state().eq(GameState::InBossFight.id())),
            measured_if!(mem::game_state().eq(GameState::Overworld.id())),
            Game::in_game(),
        ))
        .points(10)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn guard_boosts_achievement(title: &str, description: &str) -> Achievement {
    const NUM_GUARD_BOOSTS: u32 = 13;

    Achievement::builder(title)
        .description(description)
        .core(chain!(
            add_source!(delta!(bitcount!(mem::guard_booster_flags()))),
            add_source!(delta!(bit0!(mem::guard_booster_flags() + 1))),
            add_source!(delta!(bit1!(mem::guard_booster_flags() + 1))),
            add_source!(delta!(bit2!(mem::guard_booster_flags() + 1))),
            add_source!(delta!(bit3!(mem::guard_booster_flags() + 1))),
            delta!(bit4!(mem::guard_booster_flags() + 1).eq(NUM_GUARD_BOOSTS - 1)),
            add_source!(bitcount!(mem::guard_booster_flags())),
            add_source!(bit0!(mem::guard_booster_flags() + 1)),
            add_source!(bit1!(mem::guard_booster_flags() + 1)),
            add_source!(bit2!(mem::guard_booster_flags() + 1)),
            add_source!(bit3!(mem::guard_booster_flags() + 1)),
            measured!(bit4!(mem::guard_booster_flags() + 1).eq(NUM_GUARD_BOOSTS)),
            or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
            or_next!(mem::game_state().eq(GameState::InBossFight.id())),
            measured_if!(mem::game_state().eq(GameState::Overworld.id())),
            Game::in_game(),
        ))
        .points(25)
        .id(626370)
        .badge_id(712204)
        .build()
}
