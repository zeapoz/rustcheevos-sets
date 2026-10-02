use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Difficulty {
    Kantan = 0x0,
    Futsuu = 0x1,
    Muzukashii = 0x2,
    Oni = 0x3,
}

impl Difficulty {
    pub const SAVE_DATA_STRIDE: usize = 0xe6;

    /// Returns the save data offset for this difficulty.
    pub fn offset(&self) -> usize {
        *self as usize * Self::SAVE_DATA_STRIDE
    }

    pub fn lookup() -> [(u32, String); 4] {
        [
            (Self::Kantan as u32, Self::Kantan.to_string()),
            (Self::Futsuu as u32, Self::Futsuu.to_string()),
            (Self::Muzukashii as u32, Self::Muzukashii.to_string()),
            (Self::Oni as u32, Self::Oni.to_string()),
        ]
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Difficulty::Kantan => "Kantan",
            Difficulty::Futsuu => "Futsuu",
            Difficulty::Muzukashii => "Muzukashii",
            Difficulty::Oni => "Oni",
        };
        write!(f, "{s}")
    }
}
