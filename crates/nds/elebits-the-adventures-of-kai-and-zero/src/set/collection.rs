use rustcheevos::{
    add_source, bit0, bit1, bit2, bit3, bit4, bitcount, chain, delta, measured, measured_if,
    prelude::*,
    types::{achievement::Achievement, chain::ChainGroup},
};

use crate::{
    mem,
    types::{game::Game, location::Location},
    utils::{
        BATTERIES_PER_WORLD, PINK_ELEBITS_PER_WORLD, bit_at, world_battery_bits,
        world_pink_elebit_bits,
    },
};

#[rustfmt::skip]
pub fn generate_collection_achievements() -> Vec<Achievement> {
    let mut result = Vec::with_capacity(7);
    result.push(pink_elebits_achievement("Hidden Pink Among the Leaves", "Find all Pink Elebits in the Elebit Forest", Location::ElebitForest));
    result.push(pink_elebits_achievement("Deep Pink Discovery", "Find all Pink Elebits in the Elebit Mine", Location::ElebitMines));
    result.push(pink_elebits_achievement("Pink Paradise Hunter", "Find all Pink Elebits in the Resort Island", Location::ResortIsland));
    result.push(pink_elebits_achievement("Chilled Pink Collector", "Find all Pink Elebits in the Ice World", Location::IceWorld));
    result.push(pink_elebits_achievement("Flaming Pink Forager", "Find all Pink Elebits in the Ruined World", Location::RuinedWorld));
    result.push(pink_elebits_achievement("Aquatic Pink Adventurer", "Find all Pink Elebits in the Sea Temple", Location::SeaTemple));
    result.push(pink_elebits_achievement("Crystal Pink Connoisseur", "Find all Pink Elebits in the Libra of Crystal", Location::LibraOfCrystal));

    result.push(battery_achievement("Wooded Watts", "Find all 6 Batteries in the Elebit Forest", Location::ElebitForest));
    result.push(battery_achievement("Lithic Watts", "Find all 6 Batteries in the Elebit Mine", Location::ElebitMines));
    result.push(battery_achievement("Igneous Watts", "Find all 6 Batteries in the Resort Island", Location::ResortIsland));
    result.push(battery_achievement("Glacial Watts", "Find all 6 Batteries in the Ice World", Location::IceWorld));
    result.push(battery_achievement("Volcanic Watts", "Find all 6 Batteries in the Ruined World", Location::RuinedWorld));
    result.push(battery_achievement("Tidal Watts", "Find all 6 Batteries in the Sea Temple", Location::SeaTemple));
    result.push(battery_achievement("Prismatic Watts", "Find all 6 Batteries in the Libra of Crystal", Location::LibraOfCrystal));

    result.push(guard_boosts_achievement("Fully Guarded", "Obtain all 13 Guard Boosts"));
    result
}

fn pink_elebits_achievement(title: &str, description: &str, world: Location) -> Achievement {
    let bits = world_pink_elebit_bits(world);
    let requirements = ChainGroup::new(chain!(
        add_source!(delta!(bit_at(bits[0].0, bits[0].1))),
        add_source!(delta!(bit_at(bits[1].0, bits[1].1))),
        delta!(bit_at(bits[2].0, bits[2].1)).eq(PINK_ELEBITS_PER_WORLD - 1),
        add_source!(bit_at(bits[0].0, bits[0].1)),
        add_source!(bit_at(bits[1].0, bits[1].1)),
        measured!(bit_at(bits[2].0, bits[2].1).eq(PINK_ELEBITS_PER_WORLD)),
        measured_if!(Game::in_game()),
        measured_if!(mem::current_game_scene().eq(world.id())),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .build()
}

fn battery_achievement(title: &str, description: &str, world: Location) -> Achievement {
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
        measured_if!(Game::in_game()),
        measured_if!(mem::current_game_scene().eq(world.id())),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
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
        measured_if!(Game::in_game()),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .build()
}
