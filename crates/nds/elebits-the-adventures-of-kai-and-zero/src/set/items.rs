use rustcheevos::{
    add_address, bits24, bits32, chain, delta,
    prelude::*,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
    },
};

use crate::{mem, types::game::Game, utils::bit_at};

pub fn generate_items_achievements() -> Vec<Achievement> {
    vec![
        item_pickup_achievement("Pathfinder", "Pick up a Trace Laser", 3),
        item_pickup_achievement("Target Acquired", "Pick up a Wide Lock Laser", 2),
        item_pickup_achievement("On Fire", "Pick up a Fever Laser x2", 0),
        item_pickup_achievement("Ablaze", "Pick up a Fever Laser x3", 1),
        all_power_ups_achievement(),
        trace_laser_challenge_achievement(),
        wide_lock_challenge_achievement(),
        fever_laser_challenge_achievement(),
    ]
}

fn item_pickup_achievement(title: &str, description: &str, bit: u32) -> Achievement {
    let requirements = ChainGroup::new(chain!(
        delta!(bit_at(mem::item_collected_flags(), bit).eq(0)),
        bit_at(mem::item_collected_flags(), bit).eq(1),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .build()
}

fn all_power_ups_achievement() -> Achievement {
    let mut requirements = ChainGroup::new(chain!(
        mem::fever_laser_x2_timer().ne(0),
        mem::fever_laser_x3_timer().ne(0),
        mem::wide_lock_laser_timer().ne(0),
        mem::trace_laser_timer().ne(0),
        Game::in_game(),
    ));
    requirements.set_alt_groups(vec![
        chain!(delta!(mem::fever_laser_x2_timer().eq(0))),
        chain!(delta!(mem::fever_laser_x3_timer().eq(0))),
        chain!(delta!(mem::wide_lock_laser_timer().eq(0))),
        chain!(delta!(mem::trace_laser_timer().eq(0))),
    ]);

    Achievement::builder("Power-Up Package")
        .description("Have all power-ups active at once")
        .requirements(requirements)
        .build()
}

fn combo_text_pointer() -> Chain {
    const HUD_POINTER_OFFSET: usize = 0x90;
    const COMBO_TEXT_OFFSET: usize = 0x18;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(HUD_POINTER_OFFSET)),
        add_address!(bits24!(COMBO_TEXT_OFFSET)),
    )
}

fn trace_laser_challenge_achievement() -> Achievement {
    const TARGET: u32 = 25;
    const COMBO_NUMBER_OFFSET: usize = 0x494;

    Achievement::builder("Chasing the Combo")
        .description(format!(
            "With the Trace Laser active, capture {TARGET} Elebits in a single combo"
        ))
        .requirements(chain!(
            combo_text_pointer(),
            delta!(bits32!(COMBO_NUMBER_OFFSET).ne(TARGET)),
            combo_text_pointer(),
            bits32!(COMBO_NUMBER_OFFSET).eq(TARGET),
            mem::trace_laser_timer().ne(0),
            Game::in_game(),
        ))
        .build()
}

fn wide_lock_challenge_achievement() -> Achievement {
    const TARGET: u32 = 25;
    const COMBO_NUMBER_OFFSET: usize = 0x494;

    Achievement::builder("Sweeping Blow")
        .description(format!(
            "With the wide lock active, earn a combo of more than x{TARGET}"
        ))
        .requirements(chain!(
            combo_text_pointer(),
            delta!(bits32!(COMBO_NUMBER_OFFSET).ne(TARGET)),
            combo_text_pointer(),
            bits32!(COMBO_NUMBER_OFFSET).eq(TARGET),
            mem::wide_lock_laser_timer().ne(0),
            Game::in_game(),
        ))
        .build()
}

fn fever_laser_challenge_achievement() -> Achievement {
    const TARGET: u32 = 2500;
    const COMBO_WATTS: usize = 0x474;

    let mut requirements = ChainGroup::new(chain!(
        combo_text_pointer(),
        delta!(bits32!(COMBO_WATTS).ne(TARGET)),
        combo_text_pointer(),
        bits32!(COMBO_WATTS).eq(TARGET),
        Game::in_game(),
    ));

    requirements.push_alt_group(chain!(mem::fever_laser_x2_timer().ne(0)));
    requirements.push_alt_group(chain!(mem::fever_laser_x3_timer().ne(0)));

    Achievement::builder("Watt Powerhouse")
        .description(format!(
            "With any fever laser active, earn {TARGET} or more Watts in a single combo"
        ))
        .requirements(requirements)
        .build()
}
