use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::ResolvedChain},
};

use crate::types::{
    achievement::InGameAchievement,
    menu::{GeneralMenuState, MenuState},
    profile::Profile,
};

pub fn in_game_achievement(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    in_game_achievement_with(
        description,
        in_game_achievement,
        Profile::is_loaded().into(),
    )
}

pub fn in_game_achievement_with(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    extra_chain: ResolvedChain,
) -> Achievement {
    Achievement::builder("TBD - In-Game Achievement")
        .description(description)
        .core(chain!(
            delta!(in_game_achievement.is_locked()),
            in_game_achievement.is_unlocked(),
            MenuState::is(MenuState::General(GeneralMenuState::Mail)),
            extra_chain,
        ))
        .build()
}
