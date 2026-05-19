use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MedalStatus {
    Unlocked = 0x0,
    Bronze = 0x1,
    Silver = 0x2,
    Gold = 0x3,
    Locked = 0x4,
}

impl MedalStatus {
    /// Returns the name of this medal status, if any.
    pub fn name(self) -> &'static str {
        match self {
            MedalStatus::Unlocked => "Unlocked",
            MedalStatus::Bronze => "Bronze",
            MedalStatus::Silver => "Silver",
            MedalStatus::Gold => "Gold",
            MedalStatus::Locked => "Locked",
        }
    }

    /// Returns the offset of this medal status, if any.
    pub fn offset(self) -> usize {
        match self {
            MedalStatus::Bronze => 0x8,
            MedalStatus::Silver => 0x4,
            MedalStatus::Gold => 0x0,
            _ => panic!("Invalid medal status"),
        }
    }
}

impl fmt::Display for MedalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
