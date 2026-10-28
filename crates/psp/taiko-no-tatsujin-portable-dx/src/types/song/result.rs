use std::fmt;

use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;

use crate::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SongResult {
    Fail = 0x0,
    SilverCrown = 0x1,
    GoldenCrown = 0x2,
}

impl SongResult {
    pub const LOOKUP_TABLE: [(u32, &'static str); 3] = [
        (Self::Fail as u32, ""),
        (Self::SilverCrown as u32, "🥈 "),
        (Self::GoldenCrown as u32, "🏅 "),
    ];

    pub fn end_result_is(result: Self) -> Condition {
        mem::end_result().eq(result as u32)
    }
}

impl fmt::Display for SongResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SongResult::Fail => "Fail",
            SongResult::SilverCrown => "Silver Crown",
            SongResult::GoldenCrown => "Golden Crown",
        };
        write!(f, "{s}")
    }
}
