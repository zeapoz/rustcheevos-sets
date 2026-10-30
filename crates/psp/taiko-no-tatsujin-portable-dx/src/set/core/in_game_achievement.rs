use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::ResolvedChain},
};

use crate::types::{
    achievement::InGameAchievement,
    menu::{GeneralMenuState, MenuState},
    profile::Profile,
};

pub struct InGameAchievementBuilder {
    description: &'static str,
    achievement: InGameAchievement,
    extra_core: ResolvedChain,
    alt_groups: Vec<ResolvedChain>,
}

impl InGameAchievementBuilder {
    pub fn new(description: &'static str, achievement: InGameAchievement) -> Self {
        Self {
            description,
            achievement,
            extra_core: ResolvedChain::new(),
            alt_groups: Vec::new(),
        }
    }

    pub fn extra_core(mut self, extra_core: impl Into<ResolvedChain>) -> Self {
        self.extra_core = extra_core.into();
        self
    }

    pub fn alt_group(mut self, alt_group: impl Into<ResolvedChain>) -> Self {
        self.alt_groups.push(alt_group.into());
        self
    }

    pub fn build(self) -> Achievement {
        Achievement::builder("TBD - In-Game Achievement")
            .description(self.description)
            .core(self.build_core())
            .alt_groups(self.alt_groups)
            .build()
    }

    fn build_core(&self) -> ResolvedChain {
        chain!(
            delta!(self.achievement.is_locked()),
            self.achievement.is_unlocked(),
            self.extra_core.clone(),
            MenuState::is(GeneralMenuState::Mail as u32),
        )
        .into()
    }
}

pub fn in_game_achievement(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(Profile::is_loaded())
        .build()
}

pub fn in_game_achievement_with_extra(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    extra_core: impl Into<ResolvedChain>,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(Profile::is_loaded(), extra_core.into()))
        .build()
}
