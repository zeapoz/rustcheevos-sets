use rustcheevos::types::{
    achievement::Achievement,
    set::{Set, Subset, SubsetType},
};

use crate::{
    set::{common::leaderboard::generate_leaderboards, common::song::clear_song_with_result},
    types::{difficulty::Difficulty, song::ALL_DLC_SONGS, song::result::SongResult},
};

pub struct DLCSubset;

impl DLCSubset {
    pub fn generate() -> Set<Subset> {
        let mut set = Set::new_subset(46365, "DLC", SubsetType::Bonus);
        set.add_achievements(Self::generate_set())
            .add_leaderboards(generate_leaderboards(&ALL_DLC_SONGS));
        set
    }

    fn generate_set() -> Vec<Achievement> {
        ALL_DLC_SONGS
            .iter()
            .map(|song| {
                clear_song_with_result(
                    "DLC Song FC",
                    song,
                    SongResult::GoldenCrown,
                    Difficulty::Muzukashii,
                )
            })
            .collect()
    }
}
