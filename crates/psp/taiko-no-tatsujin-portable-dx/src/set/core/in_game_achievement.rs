use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::types::{
    achievement::InGameAchievement,
    menu::{GeneralMenuState, MenuState},
    profile::Profile,
};

pub fn in_game_achievement(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    Achievement::builder("TBD - In-Game Achievement")
        .description(description)
        .core(chain!(
            delta!(in_game_achievement.is_locked()),
            in_game_achievement.is_unlocked(),
            MenuState::is(MenuState::General(GeneralMenuState::Mail)),
            Profile::is_loaded(),
        ))
        .build()
}
