use rustcheevos::{
    bits32, chain, delta,
    prelude::*,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
    },
};

use crate::{
    mem,
    types::{game::Game, game_state::GameState, location::Location},
    utils::{bit_at, combo_text_pointer, combo_text_pointer_not_null},
};

#[rustfmt::skip]
pub fn generate_items_achievements() -> Vec<Achievement> {
    vec![
        item_pickup_achievement(626371, 712205, "Pathfinder", "Pick up a Trace Laser", 3),
        item_pickup_achievement(626372, 712206, "Target Acquired", "Pick up a Wide Lock Laser", 2),
        item_pickup_achievement(626373, 712207, "On Fire", "Pick up a Fever Laser x2", 0),
        item_pickup_achievement(626374, 712208, "Hot Streak", "Pick up a Fever Laser x3", 1),
        all_power_ups_achievement(),
        combo_challenge_achievement(626376, 712210, "Trailblazer", "With the Trace Laser active, net a combo of 17 or more in the Elebit Forest", 17, &Location::ElebitForest, Some(chain!(mem::trace_laser_timer().ne(0)))),
        combo_challenge_achievement(626377, 712211, "Elebit Excavation", "With the Fever Laser x3 active, net a combo of 20 or more in the Elebit Mine", 20, &Location::ElebitMine, Some(chain!(mem::fever_laser_x3_timer().ne(0)))),
        combo_challenge_achievement(626378, 712212, "Vista Voyager", "With the Fever Laser x2 active, net a combo of 19 or more in the Resort Island", 19, &Location::ResortIsland, Some(chain!(mem::fever_laser_x2_timer().ne(0)))),
        combo_challenge_achievement(626379, 712213, "Cold Rush", "With the Fever Laser x3 active, net a combo of 25 or more in the Ice World", 25, &Location::IceWorld, Some(chain!(mem::fever_laser_x3_timer().ne(0)))),
        combo_challenge_achievement(626380, 712214, "Hot Spot", "Net a combo of 16 or more in the Ruined World", 16, &Location::RuinedWorld, None),
        combo_challenge_achievement(626381, 712215, "Reef Raider", "With the Fever Laser x2 active, net a combo of 28 or more in the Sea Temple", 28, &Location::SeaTemple, Some(chain!(mem::fever_laser_x2_timer().ne(0)))),
        combo_challenge_achievement(626382, 712216, "Crystal Catcher", "With the Fever Laser x3 active, net a combo of 22 or more in the Libra of Crystal", 22, &Location::LibraOfCrystal, Some(chain!(mem::fever_laser_x3_timer().ne(0)))),
    ]
}

fn item_pickup_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    bit: u32,
) -> Achievement {
    let requirements = ChainGroup::new(chain!(
        delta!(bit_at(mem::item_collected_flags(), bit).eq(0)),
        bit_at(mem::item_collected_flags(), bit).eq(1),
        mem::game_state().eq(GameState::Overworld.id()),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(2)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn all_power_ups_achievement() -> Achievement {
    let mut requirements = ChainGroup::new(chain!(
        mem::fever_laser_x2_timer().ne(0),
        mem::fever_laser_x3_timer().ne(0),
        mem::wide_lock_laser_timer().ne(0),
        mem::trace_laser_timer().ne(0),
        mem::game_state().eq(GameState::Overworld.id()),
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
        .points(10)
        .id(626375)
        .badge_id(712209)
        .build()
}

fn combo_challenge_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    target: u32,
    world: &Location,
    extra_cond: Option<Chain>,
) -> Achievement {
    const COMBO_NUMBER_OFFSET: usize = 0x494;

    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            combo_text_pointer(),
            delta!(bits32!(COMBO_NUMBER_OFFSET).ne(target)),
            combo_text_pointer(),
            bits32!(COMBO_NUMBER_OFFSET).ge(target),
            extra_cond.unwrap_or_default(),
            mem::current_game_scene().eq(world.id()),
            mem::game_state().eq(GameState::Overworld.id()),
            Game::in_game(),
            combo_text_pointer_not_null()
        ))
        .points(10)
        .id(id)
        .badge_id(badge_id)
        .build()
}
