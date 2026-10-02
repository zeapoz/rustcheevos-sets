use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Result {
    Fail = 0x0,
    SilverCrown = 0x1,
    GoldenCrown = 0x2,
}

impl Result {
    pub const LOOKUP_TABLE: [(u32, &'static str); 3] = [
        (Self::Fail as u32, ""),
        (Self::SilverCrown as u32, "🏅"),
        (Self::GoldenCrown as u32, "🥈"),
    ];
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Result::Fail => "Fail",
            Result::SilverCrown => "Silver Crown",
            Result::GoldenCrown => "Golden Crown",
        };
        write!(f, "{s}")
    }
}
