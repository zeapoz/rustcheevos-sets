use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;

use crate::{
    types::{
        menu::GameMode,
        mode::omikoshi::{BattleOpponent, OmikoshiMode},
        profile::Profile,
    },
    util::{ByteArrayCounterBuilder, TOTAL_NUM_OMIKOSHI_CLOTHES, TOTAL_NUM_OMIKOSHI_HEADGEAR},
};

pub fn beat_boss(title: &str, points: u32, boss: BattleOpponent) -> Achievement {
    Achievement::builder(title)
        .description(format!("Defeat {boss} in the Omikoshi story mode"))
        .core(chain!(
            delta!(boss.defeated_stats().eq(BattleOpponent::UNLOCKED_FLAG)),
            boss.defeated_stats().eq(BattleOpponent::BOSS_DEFEATED_FLAG),
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded(),
        ))
        .points(points)
        .build()
}

fn obtain_omikoshi(
    title: &str,
    description: &str,
    points: u32,
    memory_ref: ByteArrayCounterBuilder,
    num_items: u32,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            delta!(memory_ref.build()).eq(num_items - 1).resolve(),
            measured!(memory_ref.build().eq(num_items)),
            measured_if!(GameMode::is(GameMode::Omikoshi)),
            measured_if!(Profile::is_loaded())
        ))
        .points(points)
        .build()
}

pub fn obtain_omikoshi_headgear(
    title: &str,
    description: &'static str,
    points: u32,
) -> Achievement {
    obtain_omikoshi(
        title,
        description,
        points,
        Profile::num_omikoshi_headgear(),
        TOTAL_NUM_OMIKOSHI_HEADGEAR,
    )
}

pub fn obtain_omikoshi_clothes(title: &str, description: &'static str, points: u32) -> Achievement {
    obtain_omikoshi(
        title,
        description,
        points,
        Profile::num_omikoshi_clothes(),
        TOTAL_NUM_OMIKOSHI_CLOTHES,
    )
}

pub fn control_dojo(title: &'static str, points: u32, target: u32) -> Achievement {
    Achievement::builder(title)
        .description("Control {dojo} dojo in Omikoshi mode")
        .core(chain!(
            delta!(OmikoshiMode::number_of_controlled_prefectures().eq(target - 1)),
            OmikoshiMode::number_of_controlled_prefectures().eq(target),
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded(),
        ))
        .points(points)
        .build()
}
