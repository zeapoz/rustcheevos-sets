use rustcheevos::{
    prelude::*,
    types::{
        achievement::{Achievement, Tag},
        chain::Chain,
        memory::MemoryRef,
        requirement::Condition,
        value::TypedValue,
    },
};

use crate::{
    mem,
    types::{
        boss::{Boss, LeoBoss, PowerOmegaBoss, XFireOmegaBoss, XIceOmegaBoss},
        location::Location,
        omega::{Omega, active::ActiveOmega},
        player::Player,
    },
};

#[rustfmt::skip]
pub fn generate_boss_achievements() -> Vec<Achievement> {
    vec![
        disarm_achievement("A Chilly Surprise", "Disarm the Power Omega using the Ice Omega"),
        beat_boss_damageless("Unshaken", "Defeat the Earth Omega without taking a single hit", Location::EarthOmegaBossArena, 10, 626347, 713055),
        tag_team_achievement(626348, "Power-Duo Tag Team", "In the fight against the X Fire Omega, break one pillar each with the Power Omega and the X Power Omega"),
        counter_turn_achievement(
            "Sharpshooter",
            "Hit the X Ice Omega with 3 or more projectiles in a single turn",
            XIceOmegaBoss::snake_attack_counter,
            Location::XIceOmegaBossArena,
            3,
            626349,
            712183,
        ),
        beat_boss_achievement("Return to Sender", "Defeat Leo with Zero's newfound power", Location::LeoBossArena, 626350, 712562),
        counter_turn_achievement(
            "Back-to-Back",
            "In the fight against Leo, reflect back the same energy ball twice in a single turn",
            LeoBoss::next_attack_pattern,
            Location::LeoBossArena,
            2,
            626351,
            712185,
        ),
        beat_boss_fast_achievement(626352, "Speed Demon", "Defeat the X Earth Omega in less than 2 minutes"),
        no_dig_holes_achievement("Earth Environmentalist", "Defeat the X Water Omega while digging no more than 15 holes in the ground"),
        beat_boss_damageless("Sunchaser", "Defeat the first phase of Mobius without taking a single hit", Location::MobiusBossArena, 10, 627149, 713057),
        beat_boss_damageless("Multi-Ender", "Defeat the second phase of Mobius without taking a single hit", Location::MobiusSecondPhaseBossArena, 25, 626354, 713056),
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
        .core(chain!(
            Boss::boss_defeated(),
            Boss::in_boss_arena(location),
            Boss::null_pointer_check(),
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
        .core(chain!(
            and_next!(ActiveOmega::id().eq(Omega::Ice.id())),
            and_next!(delta!(PowerOmegaBoss::attack_state()).eq(0x4)),
            trigger!(PowerOmegaBoss::attack_state().eq(0x0)),
            trigger!(PowerOmegaBoss::bounces_left().eq(0)),
            Boss::in_boss_arena(Location::PowerOmegaBossArena),
            Boss::null_pointer_check(),
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
        .core(chain!(
            trigger!(Boss::boss_defeated()),
            reset_next_if!(mem::current_game_scene().ne(location.id())),
            pause_if!(Player::current_health().lt(delta!(Player::current_health()))).with_hits(1),
            Boss::in_boss_arena(location),
            Boss::null_pointer_check(),
        ))
        .points(points)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn tag_team_achievement(id: u32, title: &str, description: &str) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            break_pillar(Omega::Power),
            break_pillar(Omega::XPower),
            reset_if!(mem::current_game_scene().ne(Location::XFireOmegaBossArena.id())),
            Boss::in_boss_arena(Location::XFireOmegaBossArena),
            Boss::null_pointer_check(),
        ))
        .points(2)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(712182)
        .build()
}

fn counter_turn_achievement(
    title: &str,
    description: &str,
    counter: impl Fn() -> Chain<MemoryRef>,
    location: Location,
    num_hits: u32,
    id: u32,
    badge_id: u32,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            remember!(delta!(counter())),
            or_next!(counter().gt(TypedValue::Recall)),
            reset_next_if!(mem::current_game_scene().ne(location.id())),
            remember!(delta!(Boss::health_numerator())),
            trigger!(Boss::health_numerator().lt(TypedValue::Recall)).with_hits(num_hits),
            Boss::in_boss_arena(location),
            Boss::null_pointer_check(),
        ))
        .points(5)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(badge_id)
        .build()
}

fn beat_boss_fast_achievement(id: u32, title: &str, description: &str) -> Achievement {
    const TIME_LIMIT: u32 = 120 * 60; // 2 minutes at 60 fps.
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            trigger!(Boss::boss_defeated()),
            Boss::timer().lt(TIME_LIMIT),
            Boss::in_boss_arena(Location::XEarthOmegaBossArena),
            Boss::null_pointer_check(),
        ))
        .points(10)
        .tag(Tag::Missable)
        .id(id)
        .badge_id(712186)
        .build()
}

fn no_dig_holes_achievement(title: &str, description: &str) -> Achievement {
    const NUM_ALLOWED: u32 = 15;

    Achievement::builder(title)
        .description(description)
        .core(chain!(
            trigger!(Boss::boss_defeated()),
            pause_if!(digging_with_earth_omega()).with_hits(NUM_ALLOWED + 1),
            Boss::in_boss_arena(Location::XWaterOmegaBossArena),
            Boss::null_pointer_check(),
        ))
        .alt_group(chain!(
            measured!(digging_with_earth_omega()).with_hits(NUM_ALLOWED),
            measured_if!(mem::current_game_scene().eq(Location::XWaterOmegaBossArena.id())),
        ))
        .alt_group(reset_if!(
            mem::current_game_scene().ne(Location::XWaterOmegaBossArena.id())
        ))
        .points(10)
        .tag(Tag::Missable)
        .id(626353)
        .badge_id(712187)
        .build()
}

fn break_pillar(omega: Omega) -> Chain<Condition> {
    chain!(
        and_next!(ActiveOmega::id().eq(omega.id())),
        and_next!(delta!(XFireOmegaBoss::attack_state().eq(6))),
        trigger!(XFireOmegaBoss::attack_state().eq(3).with_hits(1)),
    )
}

fn digging_with_earth_omega() -> Chain<Condition> {
    chain!(
        or_next!(ActiveOmega::id().eq(Omega::Earth.id())),
        and_next!(ActiveOmega::id().eq(Omega::XEarth.id())),
        and_next!(delta!(mem::omega_action_flag()).eq(0)),
        mem::omega_action_flag().eq(1)
    )
}
