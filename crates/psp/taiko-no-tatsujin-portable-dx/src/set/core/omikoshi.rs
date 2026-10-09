use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;

use crate::{
    types::{
        menu::GameMode,
        mode::omikoshi::{BattleOpponent, OmikoshiMode},
        profile::Profile,
        sounds::Sound,
    },
    util::{TOTAL_NUM_CLOTHES, TOTAL_NUM_DOJO, TOTAL_NUM_HEADGEAR},
};

pub fn beat_boss(boss: BattleOpponent) -> Achievement {
    Achievement::builder("TBD - Beat Boss")
        .description(format!("Defeat {boss} in the Omikoshi story mode"))
        .core(chain!(
            delta!(boss.defeated_stats().eq(BattleOpponent::UNLOCKED_FLAG)),
            boss.defeated_stats().eq(BattleOpponent::BOSS_DEFEATED_FLAG),
            // TODO: Replace with menu state maybe?
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded(),
        ))
        .build()
}

pub fn control_dojo(num_dojos: u32) -> Achievement {
    let fmt_string = if num_dojos == TOTAL_NUM_DOJO {
        "all"
    } else {
        &num_dojos.to_string()
    };

    Achievement::builder("TBD - Control Dojo")
        .description(format!(
            "Control {fmt_string} dojo in the Omikoshi story mode"
        ))
        .core(chain!(
            delta!(OmikoshiMode::number_of_controlled_prefectures().eq(num_dojos - 1)),
            measured!(OmikoshiMode::number_of_controlled_prefectures().eq(num_dojos)),
            measured_if!(GameMode::is(GameMode::Omikoshi)),
            Profile::is_loaded()
        ))
        .build()
}

pub fn gather_drum_disciples(num_disciples: u32, sound: Sound) -> Achievement {
    Achievement::builder("TBD - Gather Drum Disciples")
        .description(format!(
            "Gather {num_disciples} Drum disciples in the Omikoshi story mode and unlock the {sound} sound"
        ))
        .core(chain!(
            delta!(sound.is_locked()),
            sound.is_unlocked(),
            OmikoshiMode::number_of_disciples_on_hand().ge(num_disciples),
            GameMode::is(GameMode::Omikoshi),
            Profile::is_loaded()
        ))
        .build()
}

pub fn unlock_all_headgear() -> Achievement {
    Achievement::builder("TBD - Unlock All Headgear")
        .description("Find all headgear in the Omikoshi story mode")
        .core(chain!(
            delta!(Profile::num_headgear().eq(TOTAL_NUM_HEADGEAR - 1)),
            measured!(Profile::num_headgear().eq(TOTAL_NUM_HEADGEAR)),
            Profile::is_loaded()
        ))
        .build()
}

pub fn unlock_all_clothes() -> Achievement {
    Achievement::builder("TBD - Unlock All Clothes")
        .description("Find all clothes in the Omikoshi story mode")
        .core(chain!(
            delta!(Profile::num_clothes().eq(TOTAL_NUM_CLOTHES - 1)),
            measured!(Profile::num_clothes().eq(TOTAL_NUM_CLOTHES)),
            measured_if!(GameMode::is(GameMode::Omikoshi)),
            Profile::is_loaded()
        ))
        .build()
}
