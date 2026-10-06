use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::types::{achievement::InGameAchievement, profile::Profile};

pub fn in_game_achievement(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    Achievement::builder("TBD - In-Game Achievement")
        .description(description)
        .core(chain!(
            delta!(in_game_achievement.is_locked()),
            in_game_achievement.is_unlocked(),
            Profile::is_loaded(),
        ))
        .build()
}
