use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::requirement::Condition;

use crate::mem;
use crate::types::song::Song;
use crate::types::song::result::SongResult;
use crate::{
    set::core::in_game_achievement::InGameAchievementBuilder,
    types::{achievement::InGameAchievement, difficulty::Difficulty, profile::Profile},
};

pub fn earn_silver_crowns(
    title: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
    target: u32,
    difficulty: Difficulty,
    unlock_str: Option<&str>,
) -> Achievement {
    let unlock_str = if let Some(unlock_str) = unlock_str {
        format!(" and unlock {unlock_str}")
    } else {
        "".to_string()
    };

    let description = format!(
        "Earn {target} {}s on {difficulty} difficulty{unlock_str}",
        SongResult::SilverCrown
    );

    InGameAchievementBuilder::new(title, &description, points, in_game_achievement)
        .extra_core(chain!(
            Profile::num_silver_crowns_or_higher_per_difficulty(difficulty),
            measured!(Condition::always_false().with_hits(target)),
            measured_if!(Profile::is_loaded()),
            reset_if!(
                mem::currently_selected_profile().ne(delta!(mem::currently_selected_profile()))
            ),
        ))
        .build()
}

pub fn earn_golden_crowns(
    title: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
    target: u32,
    unlock_str: &str,
) -> Achievement {
    let description = format!(
        "Earn {target} {}s and unlock {}",
        SongResult::GoldenCrown,
        unlock_str
    );

    let measured_num_crowns =
        Profile::num_crowns_per_difficulty(Difficulty::Oni, SongResult::GoldenCrown)
            .map_head(|h| measured!(h.lhs().eq(target)));

    InGameAchievementBuilder::new(title, &description, points, in_game_achievement)
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
    title: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
    target: u32,
    target_str: &str,
    unlock_song: Song,
) -> Achievement {
    let description =
        format!("Accumulate {target_str} hits on the Taiko Counter and unlock \"{unlock_song}\"");

    InGameAchievementBuilder::new(title, &description, points, in_game_achievement)
        .extra_core(chain!(
            measured!(Profile::total_drum_hits().ge(target)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}
