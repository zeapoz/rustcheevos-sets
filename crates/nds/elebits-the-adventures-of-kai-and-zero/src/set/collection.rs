use rustcheevos::{
    add_source, bit0, bit1, bit2, bit3, bit4, bitcount, chain, delta, measured, measured_if,
    prelude::*,
    types::{achievement::Achievement, chain::ChainGroup},
};

use crate::{
    mem,
    types::{game::Game, game_state::GameState, location::Location},
    utils::{
        BATTERIES_PER_WORLD, PINK_ELEBITS_PER_WORLD, bit_at, world_battery_bits,
        world_pink_elebit_bits,
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
    let requirements = ChainGroup::new(chain!(
        add_source!(delta!(bit_at(bits[0].0, bits[0].1))),
        add_source!(delta!(bit_at(bits[1].0, bits[1].1))),
        delta!(bit_at(bits[2].0, bits[2].1)).eq(PINK_ELEBITS_PER_WORLD - 1),
        add_source!(bit_at(bits[0].0, bits[0].1)),
        add_source!(bit_at(bits[1].0, bits[1].1)),
        measured!(bit_at(bits[2].0, bits[2].1).eq(PINK_ELEBITS_PER_WORLD)),
        measured_if!(mem::current_game_scene().eq(world.id())),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
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
    let requirements = ChainGroup::new(chain!(
        add_source!(delta!(bit_at(bits[0].0, bits[0].1))),
        add_source!(delta!(bit_at(bits[1].0, bits[1].1))),
        add_source!(delta!(bit_at(bits[2].0, bits[2].1))),
        add_source!(delta!(bit_at(bits[3].0, bits[3].1))),
        add_source!(delta!(bit_at(bits[4].0, bits[4].1))),
        delta!(bit_at(bits[5].0, bits[5].1)).eq(BATTERIES_PER_WORLD - 1),
        add_source!(bit_at(bits[0].0, bits[0].1)),
        add_source!(bit_at(bits[1].0, bits[1].1)),
        add_source!(bit_at(bits[2].0, bits[2].1)),
        add_source!(bit_at(bits[3].0, bits[3].1)),
        add_source!(bit_at(bits[4].0, bits[4].1)),
        measured!(bit_at(bits[5].0, bits[5].1).eq(BATTERIES_PER_WORLD)),
        measured_if!(mem::current_game_scene().eq(world.id())),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(10)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn guard_boosts_achievement(title: &str, description: &str) -> Achievement {
    const NUM_GUARD_BOOSTS: u32 = 13;
    let requirements = ChainGroup::new(chain!(
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
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(25)
        .id(626370)
        .badge_id(712204)
        .build()
}
