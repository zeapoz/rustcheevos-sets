use rustcheevos::types::{
    achievement::Achievement,
    set::{Set, Subset, SubsetType},
};

use crate::{
    set::common::song::clear_song_with_result,
    types::{
        difficulty::Difficulty,
        song::{ALL_SONGS, result::SongResult},
    },
};

pub struct OniFCSubset;

impl OniFCSubset {
    pub fn generate() -> Set<Subset> {
        let mut set = Set::new_subset(46366, "Oni Full Combos", SubsetType::Bonus);
        set.add_achievements(Self::generate_set());
        set
    }

    fn generate_set() -> Vec<Achievement> {
        ALL_SONGS
            .iter()
            .map(|song| {
                clear_song_with_result(
                    "TBD - Song FC",
                    song,
                    SongResult::GoldenCrown,
                    Difficulty::Oni,
                )
            })
            .collect()
    }
}
