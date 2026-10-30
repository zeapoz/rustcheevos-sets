use rustcheevos::types::achievement::Achievement;
use rustcheevos::{prelude::*, types::requirement::Condition};

use crate::types::song::result::SongResult;
use crate::{
    set::core::in_game_achievement::in_game_achievement_with,
    types::{achievement::InGameAchievement, difficulty::Difficulty, profile::Profile},
};

pub fn earn_silver_crowns(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    n: u32,
    difficulty: Difficulty,
) -> Achievement {
    in_game_achievement_with(
        description,
        in_game_achievement,
        chain!(
            Profile::num_silver_crowns_or_higher_per_difficulty(difficulty),
            Condition::always_false().with_hits(n),
            measured_if!(Profile::is_loaded())
        )
        .into(),
    )
}

pub fn earn_golden_crowns(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    n: u32,
) -> Achievement {
    let measured_num_crowns =
        Profile::num_crowns_per_difficulty(Difficulty::Oni, SongResult::GoldenCrown)
            .map_head(|h| measured!(h.lhs().eq(n)));

    in_game_achievement_with(
        description,
        in_game_achievement,
        chain!(
            Profile::num_crowns_per_difficulty(Difficulty::Kantan, SongResult::GoldenCrown),
            Profile::num_crowns_per_difficulty(Difficulty::Futsuu, SongResult::GoldenCrown),
            Profile::num_crowns_per_difficulty(Difficulty::Muzukashii, SongResult::GoldenCrown),
            measured_num_crowns,
            measured_if!(Profile::is_loaded())
        )
        .into(),
    )
}

pub fn accumulate_hits(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    hits: u32,
) -> Achievement {
    in_game_achievement_with(
        description,
        in_game_achievement,
        chain!(
            measured!(Profile::total_drum_hits().ge(hits)),
            measured_if!(Profile::is_loaded())
        )
        .into(),
    )
}
