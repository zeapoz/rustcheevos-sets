use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;

use crate::{
    types::{
        menu::GameMode,
        mode::omikoshi::{BattleOpponent, OmikoshiMode},
        profile::Profile,
    },
    util::{TOTAL_NUM_OMIKOSHI_CLOTHES, TOTAL_NUM_OMIKOSHI_HEADGEAR},
};

pub fn beat_boss(boss: BattleOpponent) -> Achievement {
    Achievement::builder("TBD - Beat Boss")
        .description(format!("Defeat {boss} in the Omikoshi story mode"))
        .core(chain!(
            delta!(boss.defeated_stats().eq(BattleOpponent::UNLOCKED_FLAG)),
            boss.defeated_stats().eq(BattleOpponent::BOSS_DEFEATED_FLAG),
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded(),
        ))
        .build()
}

pub fn obtain_omikoshi_headgear(description: &'static str) -> Achievement {
    Achievement::builder("TBD - Obtain Omikoshi headgear")
        .description(description)
        .core(chain!(
            delta!(Profile::num_omikoshi_headgear())
                .build()
                .eq(TOTAL_NUM_OMIKOSHI_HEADGEAR - 1)
                .resolve(),
            measured!(
                Profile::num_omikoshi_headgear()
                    .build()
                    .eq(TOTAL_NUM_OMIKOSHI_HEADGEAR)
            ),
            measured_if!(GameMode::is(GameMode::Omikoshi)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}

pub fn obtain_omikoshi_clothes(description: &'static str) -> Achievement {
    Achievement::builder("TBD - Obtain Omikoshi clothes")
        .description(description)
        .core(chain!(
            delta!(Profile::num_omikoshi_clothes())
                .build()
                .eq(TOTAL_NUM_OMIKOSHI_CLOTHES - 1)
                .resolve(),
            measured!(
                Profile::num_omikoshi_clothes()
                    .build()
                    .eq(TOTAL_NUM_OMIKOSHI_CLOTHES)
            ),
            measured_if!(GameMode::is(GameMode::Omikoshi)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}

pub fn control_dojo(description: &'static str, dojos: u32) -> Achievement {
    Achievement::builder("TBD - Control Dojo")
        .description(description)
        .core(chain!(
            delta!(OmikoshiMode::number_of_controlled_prefectures().eq(dojos - 1)),
            OmikoshiMode::number_of_controlled_prefectures().eq(dojos),
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded(),
        ))
        .build()
}
