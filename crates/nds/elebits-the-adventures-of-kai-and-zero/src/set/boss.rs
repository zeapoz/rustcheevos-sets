use rustcheevos::{
    add_address, and_next, bits16, bits24, bits32, chain, delta, or_next, pause_if,
    prelude::*,
    remember, reset_if, reset_next_if, trigger,
    types::{
        achievement::{Achievement, Tag},
        chain::{Chain, PendingChain},
        memory::MemoryRef,
        value::TypedValue,
    },
};

use crate::{
    mem,
    types::{location::Location, omega::Omega, player::Player},
};

#[rustfmt::skip]
pub fn generate_boss_achievements() -> Vec<Achievement> {
    vec![
        beat_boss_achievement("Heavy Hitter", "Defeat the Power Omega", Location::PowerOmegaBossArena),
        disarm_achievement("A Chilly Surprise", "Disarm the Power Omega using the Ice Omega"),
        beat_boss_achievement("Battle Beneath the Surface", "Defeat the Earth Omega", Location::EarthOmegaBossArena),
        beat_boss_damageless("Unshaken", "Defeat the Earth Omega without letting it damage your cart", Location::EarthOmegaBossArena),
        beat_boss_achievement("Picking up Steam", "Defeat the X Fire Omega", Location::XFireOmegaBossArena),
        tag_team_achievement("Power-Duo Tag Team", "Defeat the X Fire Omega while breaking at least 1 pillar each with the Power Omega and the X Power Omega"),
        beat_boss_achievement("Breaking the Ice", "Defeat the X Ice Omega", Location::XIceOmegaBossArena),
        many_projectiles_achievement("Sharpshooter"),
        beat_boss_achievement("Return to Sender", "Defeat ??? with Zero's new found power", Location::LeoBossArena),
        back_to_back_achievement("Back-to-Back", "Reflect an energy ball back to ??? Twice in a single turn"),
        beat_boss_achievement("Tectonic Takedown", "Defeat the X Earth Omega", Location::XEarthOmegaBossArena),
        beat_boss_fast_achievement("Speed Demon", "Defeat the X Earth Omega in less than XX:XX"),
        beat_boss_achievement("Smooth Sailing", "Defeat the X Water Omega", Location::XWaterOmegaBossArena),
        no_dig_holes_achievement("Earth Environmentalist", "Defeat the X Water Omega while digging at most X holes in the ground"),
        beat_mobius_with_limited_health(),
    ]
}

fn boss_data_base_pointer() -> Chain {
    const BOSS_DATA_POINTER_OFFSET_1: usize = 0x7c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(BOSS_DATA_POINTER_OFFSET_1)),
    )
}

fn boss_data_pointer() -> Chain {
    const BOSS_DATA_POINTER_OFFSET_2: usize = 0xa0;
    chain!(
        boss_data_base_pointer(),
        add_address!(bits24!(BOSS_DATA_POINTER_OFFSET_2)),
    )
}

fn boss_health_numerator(with_delta: bool) -> PendingChain<MemoryRef> {
    const BOSS_HEALTH_NUMERATOR_OFFST: usize = 0x10;
    chain!(
        boss_data_pointer(),
        if with_delta {
            // In Rustcheevos, we should _probably_ not iterate over all conditions and apply a flag if we can.
            delta!(bits16!(BOSS_HEALTH_NUMERATOR_OFFST))
        } else {
            bits16!(BOSS_HEALTH_NUMERATOR_OFFST)
        },
    )
}

fn boss_data_null_pointer_check() -> Chain {
    const BOSS_DATA_OFFSET: usize = 0x7c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        bits24!(BOSS_DATA_OFFSET).ne(0),
    )
}

fn beat_boss_achievement(title: &str, description: &str, location: Location) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            boss_health_numerator(true).ne(0),
            boss_health_numerator(false).eq(0),
            mem::current_game_scene().eq(location.id()),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Progression)
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
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn beat_boss_damageless(title: &str, description: &str, location: Location) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            trigger!(boss_health_numerator(true).ne(0)),
            trigger!(boss_health_numerator(false).eq(0)),
            mem::current_game_scene().eq(location.id()),
            reset_next_if!(mem::current_game_scene().ne(location.id())),
            pause_if!(Player::current_health().lt(delta!(Player::current_health()))).with_hits(1),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn tag_team_achievement(title: &str, description: &str) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            trigger!(boss_health_numerator(true).ne(0)),
            trigger!(boss_health_numerator(false).eq(0)),
            and_next!(Omega::active_id().eq(Omega::Earth.id())),
            boss_data_pointer(),
            and_next!(delta!(bits16!(0x2f0).eq(5))), // Attack State
            boss_data_pointer(),
            trigger!(bits16!(0x2f0).eq(3).with_hits(1)), // Attack State
            and_next!(Omega::active_id().eq(Omega::XEarth.id())),
            boss_data_pointer(),
            and_next!(delta!(bits16!(0x2f0).eq(5))), // Attack State
            boss_data_pointer(),
            trigger!(bits16!(0x2f0).eq(3).with_hits(1)), // Attack State
            mem::current_game_scene()
                .eq(Location::XFireOmegaBossArena.id())
                .with_hits(1),
            reset_if!(mem::current_game_scene().ne(Location::XFireOmegaBossArena.id())),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn many_projectiles_achievement(title: &str) -> Achievement {
    const NUM_HITS: u32 = 4;
    Achievement::builder(title)
        .description(format!(
            "Hit the X Ice Omega with {NUM_HITS} or more projectiles in a single turn"
        ))
        .requirements(chain!(
            boss_data_base_pointer(),
            remember!(delta!(bits16!(0x112))), // Snake Attack Counter
            boss_data_base_pointer(),
            or_next!(bits16!(0x112).gt(TypedValue::Recall)),
            reset_if!(mem::current_game_scene().ne(Location::XIceOmegaBossArena.id())),
            boss_data_pointer(),
            remember!(delta!(bits16!(0x10))), // Boss Health Numerator
            boss_data_pointer(),
            trigger!(bits16!(0x10).lt(TypedValue::Recall)).with_hits(NUM_HITS),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
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
            reset_if!(mem::current_game_scene().ne(Location::LeoBossArena.id())),
            boss_data_pointer(),
            remember!(delta!(bits16!(0x10))), // Boss Health Numerator
            boss_data_pointer(),
            trigger!(bits16!(0x10).lt(TypedValue::Recall)).with_hits(NUM_HITS),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn timer_pointer() -> Chain {
    const PLAYER_DATA_OFFET: usize = 0x6c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(PLAYER_DATA_OFFET)),
    )
}

fn beat_boss_fast_achievement(title: &str, description: &str) -> Achievement {
    const TIME_LIMIT: u32 = 60 * 60; // 1 minute at 60 fps.
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            timer_pointer(),
            or_next!(bits32!(0x84).gt(TIME_LIMIT)), // Boss Fight Timer
            reset_next_if!(mem::current_game_scene().ne(Location::XEarthOmegaBossArena.id())),
            mem::current_game_scene()
                .eq(Location::XEarthOmegaBossArena.id())
                .with_hits(1),
            trigger!(boss_health_numerator(true).ne(0)),
            trigger!(boss_health_numerator(false).eq(0)),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn no_dig_holes_achievement(title: &str, description: &str) -> Achievement {
    const NUM_ALLOWED: u32 = 10;
    Achievement::builder(title)
        .description(description)
        .requirements(chain!(
            trigger!(boss_health_numerator(true).ne(0)),
            trigger!(boss_health_numerator(false).eq(0)),
            mem::current_game_scene().eq(Location::XWaterOmegaBossArena.id()),
            reset_next_if!(mem::current_game_scene().ne(Location::XWaterOmegaBossArena.id())),
            or_next!(Omega::active_id().eq(Omega::Earth.id())),
            and_next!(Omega::active_id().eq(Omega::XEarth.id())),
            and_next!(delta!(mem::omega_action_flag()).eq(0)),
            pause_if!(mem::omega_action_flag().eq(1)).with_hits(NUM_ALLOWED),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}

fn beat_mobius_with_limited_health() -> Achievement {
    const ALLOWED_HITS: u32 = 3;
    Achievement::builder("Multi-Ender")
        .description(format!(
            "Defeat the second phase of Mobius without taking more than {ALLOWED_HITS} hits"
        ))
        .requirements(chain!(
            reset_next_if!(Player::current_health().lt(delta!(Player::current_health())))
                .with_hits(ALLOWED_HITS),
            mem::current_game_scene()
                .eq(Location::MobiusSecondPhaseBossArena.id())
                .with_hits(1),
            trigger!(boss_health_numerator(true).ne(0)),
            trigger!(boss_health_numerator(false).eq(0)),
            reset_if!(mem::current_game_scene().ne(Location::MobiusSecondPhaseBossArena.id())),
            boss_data_null_pointer_check(),
        ))
        .tag(Tag::Missable)
        .build()
}
