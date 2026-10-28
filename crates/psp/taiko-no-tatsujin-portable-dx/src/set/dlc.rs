use rustcheevos::types::{
    achievement::Achievement,
    set::{Set, Subset, SubsetType},
};

use crate::{
    set::{leaderboard::generate_leaderboards, song::clear_song_with_result},
    types::{difficulty::Difficulty, result::Result, song::ALL_DLC_SONGS},
};

pub fn dlc_subset() -> Set<Subset> {
    let mut dlc_subset = Set::new_subset(0, "DLC", SubsetType::Bonus);
    dlc_subset
        .add_achievements(generate_set())
        .add_leaderboards(generate_leaderboards(&ALL_DLC_SONGS));
    dlc_subset
}

fn generate_set() -> Vec<Achievement> {
    ALL_DLC_SONGS
        .iter()
        .map(|song| {
            clear_song_with_result(
                "TBD - Song FC",
                song,
                Result::GoldenCrown,
                Difficulty::Muzukashii,
            )
        })
        .collect()
}
