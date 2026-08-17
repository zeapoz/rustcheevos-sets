use rustcheevos::{
    and_next, chain, delta, measured, measured_if, or_next,
    prelude::*,
    remember, reset_if, reset_next_if, trigger,
    types::{
        achievement::{Achievement, Tag},
        chain::ChainGroup,
        requirement::Condition,
        value::TypedValue,
    },
};

use crate::{
    mem,
    types::{
        boss::{Boss, LeoBoss, PowerOmegaBoss, XFireOmegaBoss, XIceOmegaBoss},
        game::Game,
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
        tag_team_achievement(626348, "Power-Duo Tag Team", "In the fight against the X Fire Omega, break at least 1 pillar each with the Power Omega and the X Power Omega"),
        many_projectiles_achievement("Sharpshooter"),
        beat_boss_achievement("Return to Sender", "Defeat Leo with Zero's newfound power", Location::LeoBossArena, 626350, 712562),
        back_to_back_achievement("Back-to-Back", "In the fight against Leo, reflect back the same energy ball twice in a single turn"),
        beat_boss_fast_achievement(626352, "Speed Demon", "Defeat the X Earth Omega in less than 2 minutes"),
        no_dig_holes_achievement("Earth Environmentalist", "Defeat the X Water Omega while digging at most 15 holes in the ground"),
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
        .requirements(chain!(
            delta!(Boss::health_numerator()).ne(0),
            Boss::health_numerator().eq(0),
            mem::current_game_scene().eq(location.id()),
            Game::in_game(),
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
        .requirements(chain!(
            and_next!(ActiveOmega::id().eq(Omega::Ice.id())),
            and_next!(delta!(PowerOmegaBoss::attack_state()).eq(0x4)),
            trigger!(PowerOmegaBoss::attack_state().eq(0x0)),
            trigger!(PowerOmegaBoss::bounces_left().eq(0)),
            mem::current_game_scene().eq(Location::PowerOmegaBossArena.id()),
            Game::in_game(),
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
        .requirements(chain!(
            delta!(Boss::health_numerator()).ne(0),
            trigger!(Boss::health_numerator().eq(0)),
            or_next!(Player::current_health().lt(delta!(Player::current_health()))),
            reset_next_if!(mem::current_game_scene().ne(location.id())),
            mem::current_game_scene().eq(location.id()).with_hits(1),
            Game::in_game(),
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
        .requirements(chain!(
            and_next!(ActiveOmega::id().eq(Omega::Power.id())),
            and_next!(delta!(XFireOmegaBoss::attack_state().eq(6))), // Attack State
            trigger!(XFireOmegaBoss::attack_state().eq(3).with_hits(1)), // Attack State
            and_next!(ActiveOmega::id().eq(Omega::XPower.id())),
            and_next!(delta!(XFireOmegaBoss::attack_state().eq(6))), // Attack State
            trigger!(XFireOmegaBoss::attack_state().eq(3).with_hits(1)), // Attack State
            reset_if!(mem::current_game_scene().ne(Location::XFireOmegaBossArena.id())),
            mem::current_game_scene()
                .eq(Location::XFireOmegaBossArena.id())
                .with_hits(1),
            Game::in_game(),
            Boss::null_pointer_check(),
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
            remember!(delta!(XIceOmegaBoss::snake_attack_counter())),
            or_next!(XIceOmegaBoss::snake_attack_counter().gt(TypedValue::Recall)),
            reset_next_if!(mem::current_game_scene().ne(Location::XIceOmegaBossArena.id())),
            remember!(delta!(Boss::health_numerator())),
            trigger!(Boss::health_numerator().lt(TypedValue::Recall)).with_hits(NUM_HITS),
            mem::current_game_scene().eq(Location::XIceOmegaBossArena.id()),
            Game::in_game(),
            Boss::null_pointer_check(),
            // TODO: Add hits indicator?
        ))
        .points(5)
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
            remember!(delta!(LeoBoss::next_attack_pattern())),
            or_next!(LeoBoss::next_attack_pattern().gt(TypedValue::Recall)),
            reset_next_if!(mem::current_game_scene().ne(Location::LeoBossArena.id())),
            remember!(delta!(Boss::health_numerator())),
            trigger!(Boss::health_numerator().lt(TypedValue::Recall)).with_hits(NUM_HITS),
            mem::current_game_scene().eq(Location::LeoBossArena.id()),
            Game::in_game(),
            Boss::null_pointer_check(),
        ))
        .points(5)
        .tag(Tag::Missable)
        .id(626351)
        .badge_id(712185)
        .build()
}

fn beat_boss_fast_achievement(id: u32, title: &str, description: &str) -> Achievement {
    const TIME_LIMIT: u32 = 120 * 60; // 2 minutes at 60 fps.
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            delta!(Boss::health_numerator()).ne(0),
            trigger!(Boss::health_numerator().eq(0)),
            or_next!(Boss::timer().gt(TIME_LIMIT)),
            reset_next_if!(mem::current_game_scene().ne(Location::XEarthOmegaBossArena.id())),
            mem::current_game_scene()
                .eq(Location::XEarthOmegaBossArena.id())
                .with_hits(1),
            Game::in_game(),
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

    let mut requirements = ChainGroup::new(chain!(Condition::always_true(),));

    requirements.push_alt_group(chain!(
        delta!(Boss::health_numerator()).ne(0),
        trigger!(Boss::health_numerator().eq(0)),
        or_next!(ActiveOmega::id().eq(Omega::Earth.id())),
        and_next!(ActiveOmega::id().eq(Omega::XEarth.id())),
        and_next!(delta!(mem::omega_action_flag()).eq(0)),
        reset_next_if!(mem::omega_action_flag().eq(1)).with_hits(NUM_ALLOWED + 1),
        mem::current_game_scene()
            .eq(Location::XWaterOmegaBossArena.id())
            .with_hits(1),
        reset_if!(mem::current_game_scene().ne(Location::XWaterOmegaBossArena.id())),
        Game::in_game(),
        Boss::null_pointer_check(),
    ));

    requirements.push_alt_group(chain!(
        or_next!(ActiveOmega::id().eq(Omega::Earth.id())),
        and_next!(ActiveOmega::id().eq(Omega::XEarth.id())),
        and_next!(delta!(mem::omega_action_flag()).eq(0)),
        measured!(mem::omega_action_flag().eq(1)).with_hits(NUM_ALLOWED),
        measured_if!(mem::current_game_scene().eq(Location::XWaterOmegaBossArena.id())),
        Condition::always_false()
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(10)
        .tag(Tag::Missable)
        .id(626353)
        .badge_id(712187)
        .build()
}
