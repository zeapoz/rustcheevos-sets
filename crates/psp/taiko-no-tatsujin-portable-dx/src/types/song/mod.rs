use std::fmt;

use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef},
};

use crate::{
    mem::file_1_song_scoreboard_kantan,
    types::{difficulty::Difficulty, profile::Profile},
};

mod id;
pub mod in_game_stats;

pub struct SongData {
    id: u32,
    title: &'static str,
    dlc: bool,
}

macro_rules! gen_song_defs {
    ($( $variant:ident => $data:expr ),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Song {
            $( $variant ),*
        }

        impl Song {
            pub const ALL: &[Song] = &[ $( Song::$variant ),* ];

            pub const LOOKUP_ENTRIES: &[(u32, &str)] = &[
                $( ($data.id, $data.title) ),*
            ];

            /// Returns the data of the song.
            fn data(self) -> &'static SongData {
                match self {
                    $( Song::$variant => &$data ),*
                }
            }

            /// Returns the title of the song.
            pub fn title(&self) -> &'static str {
                self.data().title
            }

            /// Returns the ID of the song.
            pub fn id(&self) -> u32 {
                self.data().id
            }

            /// Returns a chain resolving to the stored best result for the given difficulty.
            pub fn best_result(&self, difficulty: Difficulty) -> Chain<MemoryRef> {
                const SONG_SIZE_BYTES: u32 = 32;
                const RESULT_OFFSET: usize = 0x1;

                let song_stats_offset = file_1_song_scoreboard_kantan() + difficulty.offset();
                let song_offset = song_stats_offset + (self.id() * SONG_SIZE_BYTES) as usize;

                chain!(
                    Profile::offset_for_current(),
                    bits8!(song_offset + RESULT_OFFSET),
                )
            }
        }

        impl fmt::Display for Song {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.title())
            }
        }
    };
}

gen_song_defs! {
    OneDay => SongData { id: 0x00, title: "One day", dlc: false },
}
