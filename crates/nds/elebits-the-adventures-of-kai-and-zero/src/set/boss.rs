use rustcheevos::{
    add_address, and_next, bits16, bits24, bits32, chain, delta, measured, measured_if, or_next,
    pause_if,
    prelude::*,
    remember, reset_if, reset_next_if, trigger,
    types::{
        achievement::{Achievement, Tag},
        chain::{Chain, ChainGroup, PendingChain},
        memory::MemoryRef,
        requirement::Condition,
        value::TypedValue,
    },
};

use crate::{
    mem,
    types::{game::Game, location::Location, omega::Omega, player::Player},
    utils::{
        boss_data_base_pointer, boss_data_null_pointer_check, boss_data_pointer,
        boss_health_numerator,
    },
};

#[rustfmt::skip]
pub fn generate_boss_achievements() -> Vec<Achievement> {
    vec![
        disarm_achievement("A Chilly Surprise", "Disarm the Power Omega using the Ice Omega"),
        beat_boss_damageless("Unshaken", "Defeat the Earth Omega without taking a single hit", Location::EarthOmegaBossArena, 10, 626347, 712181),
        tag_team_achievement(626348, "Power-Duo Tag Team", "In the fight against the X Fire Omega, break at least 1 pillar each with the Power Omega and the X Power Omega"),
        many_projectiles_achievement("Sharpshooter"),
        beat_boss_achievement("Return to Sender", "Defeat Leo with Zero's newfound power", Location::LeoBossArena, 626350, 712184),
        back_to_back_achievement("Back-to-Back", "In the fight against Leo, reflect back the same energy ball twice in a single turn"),
        beat_boss_fast_achievement(626352, "Speed Demon", "Defeat the X Earth Omega in less than 2 minutes"),
        no_dig_holes_achievement("Earth Environmentalist", "Defeat the X Water Omega while digging at most 15 holes in the ground"),
        beat_boss_damageless("Multi-Ender",
            "Defeat the second phase of Mobius without taking a single hit",
            Location::MobiusSecondPhaseBossArena,
            25,
            626354,
            712188
),
    ]
}

fn beat_boss_achievement(
    title: &str,
    description: &str,
    location: Location,
    id: u32,
    badge_id: u32,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            boss_health_numerator(true).ne(0),
            boss_health_numerator(false).eq(0),
            mem::current_game_scene().eq(location.id()),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(10)
        .tag(Tag::Progression)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn disarm_achievement(title: &str, description: &str) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            and_next!(Omega::active_id().eq(Omega::Ice.id())),
            boss_data_pointer(),
            and_next!(delta!(bits32!(0x2ac)).eq(0x4)), // Attack State
            boss_data_pointer(),
            trigger!(bits32!(0x2ac).eq(0x0)), // Attack State
            boss_data_pointer(),
            trigger!(bits16!(0x02b6).eq(0)), // Bounces Left
            mem::current_game_scene().eq(Location::PowerOmegaBossArena.id()),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(2)
        .tag(Tag::Missable)
        .id(626346)
        .badge_id(712180)
        .build()
}

fn beat_boss_damageless(
    title: &str,
    description: &str,
    location: Location,
    points: u32,
    id: u32,
    badge_id: u32,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            boss_health_numerator(true).ne(0),
            trigger!(boss_health_numerator(false).eq(0)),
            or_next!(Player::current_health().lt(delta!(Player::current_health()))),
            reset_next_if!(mem::current_game_scene().ne(location.id())),
            mem::current_game_scene().eq(location.id()).with_hits(1),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(points)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn tag_team_achievement(id: u32, title: &str, description: &str) -> Achievement {
    const ATTACK_STATE_OFFSET: usize = 0x2f2;
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            and_next!(Omega::active_id().eq(Omega::Power.id())),
            boss_data_pointer(),
            and_next!(delta!(bits16!(ATTACK_STATE_OFFSET).eq(6))), // Attack State
            boss_data_pointer(),
            trigger!(bits16!(ATTACK_STATE_OFFSET).eq(3).with_hits(1)), // Attack State
            and_next!(Omega::active_id().eq(Omega::XPower.id())),
            boss_data_pointer(),
            and_next!(delta!(bits16!(ATTACK_STATE_OFFSET).eq(6))), // Attack State
            boss_data_pointer(),
            trigger!(bits16!(ATTACK_STATE_OFFSET).eq(3).with_hits(1)), // Attack State
            reset_if!(mem::current_game_scene().ne(Location::XFireOmegaBossArena.id())),
            mem::current_game_scene()
                .eq(Location::XFireOmegaBossArena.id())
                .with_hits(1),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(2)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(712182)
        .build()
}

fn many_projectiles_achievement(title: &str) -> Achievement {
    const NUM_HITS: u32 = 3;
    Achievement::builder(title)
        .description(format!(
            "Hit the X Ice Omega with {NUM_HITS} or more projectiles in a single turn"
        ))
        .requirements(chain!(
            boss_data_base_pointer(),
            remember!(delta!(bits16!(0x112))), // Snake Attack Counter
            boss_data_base_pointer(),
            or_next!(bits16!(0x112).gt(TypedValue::Recall)),
            reset_next_if!(mem::current_game_scene().ne(Location::XIceOmegaBossArena.id())),
            boss_data_pointer(),
            remember!(delta!(bits16!(0x10))), // Boss Health Numerator
            boss_data_pointer(),
            trigger!(bits16!(0x10).lt(TypedValue::Recall)).with_hits(NUM_HITS),
            mem::current_game_scene().eq(Location::XIceOmegaBossArena.id()),
            Game::in_game(),
            boss_data_null_pointer_check(),
            // TODO: Add hits indicator?
        ))
        .points(10)
        .tag(Tag::Missable)
        .id(626349)
        .badge_id(712183)
        .build()
}

fn back_to_back_achievement(title: &str, description: &str) -> Achievement {
    const NUM_HITS: u32 = 2;
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            boss_data_base_pointer(),
            remember!(delta!(bits16!(0xf4))), // Next Attack Pattern
            boss_data_base_pointer(),
            or_next!(bits16!(0xf4).gt(TypedValue::Recall)),
            reset_next_if!(mem::current_game_scene().ne(Location::LeoBossArena.id())),
            boss_data_pointer(),
            remember!(delta!(bits16!(0x10))), // Boss Health Numerator
            boss_data_pointer(),
            trigger!(bits16!(0x10).lt(TypedValue::Recall)).with_hits(NUM_HITS),
            mem::current_game_scene().eq(Location::LeoBossArena.id()),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(10)
        .tag(Tag::Missable)
        .id(626351)
        .badge_id(712185)
        .build()
}

pub fn boss_timer_pointer() -> Chain {
    const PLAYER_DATA_OFFET: usize = 0x6c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(PLAYER_DATA_OFFET)),
    )
}

fn beat_boss_fast_achievement(id: u32, title: &str, description: &str) -> Achievement {
    const TIME_LIMIT: u32 = 120 * 60; // 2 minutes at 60 fps.
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            boss_health_numerator(true).ne(0),
            trigger!(boss_health_numerator(false).eq(0)),
            boss_timer_pointer(),
            or_next!(bits32!(0x84).gt(TIME_LIMIT)), // Boss Fight Timer
            reset_next_if!(mem::current_game_scene().ne(Location::XEarthOmegaBossArena.id())),
            mem::current_game_scene()
                .eq(Location::XEarthOmegaBossArena.id())
                .with_hits(1),
            Game::in_game(),
            boss_data_null_pointer_check(),
        ))
        .points(10)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(712186)
        .build()
}

fn no_dig_holes_achievement(title: &str, description: &str) -> Achievement {
    const NUM_ALLOWED: u32 = 15;

    let mut requirements = ChainGroup::new(chain!(
        boss_health_numerator(true).ne(0),
        trigger!(boss_health_numerator(false).eq(0)),
        or_next!(Omega::active_id().eq(Omega::Earth.id())),
        and_next!(Omega::active_id().eq(Omega::XEarth.id())),
        and_next!(delta!(mem::omega_action_flag()).eq(0)),
        pause_if!(mem::omega_action_flag().eq(1)).with_hits(NUM_ALLOWED + 1),
        mem::current_game_scene().eq(Location::XWaterOmegaBossArena.id()),
        Game::in_game(),
        boss_data_null_pointer_check(),
    ));

    requirements.push_alt_group(chain!(
        or_next!(Omega::active_id().eq(Omega::Earth.id())),
        and_next!(Omega::active_id().eq(Omega::XEarth.id())),
        and_next!(delta!(mem::omega_action_flag()).eq(0)),
        measured!(mem::omega_action_flag().eq(1)).with_hits(NUM_ALLOWED),
        measured_if!(mem::current_game_scene().eq(Location::XWaterOmegaBossArena.id())),
        reset_if!(mem::current_game_scene().ne(Location::XWaterOmegaBossArena.id())),
    ));

    requirements.push_alt_group(Condition::always_true());

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(10)
        .tag(Tag::Missable)
        .id(626353)
        .badge_id(712187)
        .build()
}
