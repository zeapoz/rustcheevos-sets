use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::requirement::Condition;

use crate::mem;
use crate::types::song::result::SongResult;
use crate::{
    set::core::in_game_achievement::InGameAchievementBuilder,
    types::{achievement::InGameAchievement, difficulty::Difficulty, profile::Profile},
};

pub fn earn_silver_crowns(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    n: u32,
    difficulty: Difficulty,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(
            Profile::num_silver_crowns_or_higher_per_difficulty(difficulty),
            Condition::always_false().with_hits(n),
            measured_if!(Profile::is_loaded()),
            reset_if!(
                mem::currently_selected_profile().ne(delta!(mem::currently_selected_profile()))
            ),
        ))
        .build()
}

pub fn earn_golden_crowns(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    n: u32,
) -> Achievement {
    let measured_num_crowns =
        Profile::num_crowns_per_difficulty(Difficulty::Oni, SongResult::GoldenCrown)
            .map_head(|h| measured!(h.lhs().eq(n)));

    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(
            Profile::num_crowns_per_difficulty(Difficulty::Kantan, SongResult::GoldenCrown),
            Profile::num_crowns_per_difficulty(Difficulty::Futsuu, SongResult::GoldenCrown),
            Profile::num_crowns_per_difficulty(Difficulty::Muzukashii, SongResult::GoldenCrown),
            measured_num_crowns,
            measured_if!(Profile::is_loaded())
        ))
        .build()
}

pub fn accumulate_hits(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    hits: u32,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(
            measured!(Profile::total_drum_hits().ge(hits)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}
