use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;

use crate::types::profile::{Profile, Rank};
use crate::types::song::Song;

pub fn total_drum_hits(num: u32, song: Song) -> Achievement {
    let fmt_num = {
        let thousands = num / 1_000;
        let remainder = num % 1_000;
        format!("{thousands},{remainder:03}")
    };

    Achievement::builder("TBD - Total Drum Hits")
        .description(format!(
            "Accumulate {fmt_num} total hits on the Taiko counter and unlock \"{song}\""
        ))
        .core(chain!(
            delta!(song.is_locked()),
            song.is_unlocked(),
            Profile::total_drum_hits().ge(num),
            Profile::is_loaded(),
        ))
        .build()
}

pub fn achieve_rank(rank: Rank) -> Achievement {
    Achievement::builder("TBD - Promotion")
        .description(format!("Achieve the rank of {rank}"))
        .core(chain!(
            delta!(Profile::rank().eq(rank as u32 - 1)),
            Profile::rank().eq(rank as u32),
            Profile::is_loaded(),
        ))
        .build()
}
