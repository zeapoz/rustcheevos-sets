use std::fmt;

use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;
use rustcheevos::types::{chain::Chain, memory::MemoryRef};

use crate::mem;
use crate::types::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromotionMail {
    Apprentice = 0x4e,
    Novice = 0x4f,
    HalfAMan = 0x50,
    Qualified = 0x51,
    Capable = 0x52,
    GoGetter = 0x53,
    Professional = 0x54,
    Expert = 0x55,
    IronMan = 0x56,
    Master = 0x57,
    GrandMaster = 0x58,
}

impl PromotionMail {
    pub const UNSEEN_FLAG: u32 = 0x0;
    pub const SEEN_FLAG: u32 = 0x1;

    pub fn is_unseen(self) -> Chain<Condition> {
        self.seen_state().eq(Self::UNSEEN_FLAG)
    }

    pub fn is_seen(self) -> Chain<Condition> {
        self.seen_state().eq(Self::SEEN_FLAG)
    }

    fn seen_state(self) -> Chain<MemoryRef> {
        let offset = mem::recieved_mail_flags() + self as usize;
        chain!(Profile::offset_for_current(), bits8!(offset))
    }
}

impl fmt::Display for PromotionMail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PromotionMail::Apprentice => "Apprentice",
            PromotionMail::Novice => "Novice",
            PromotionMail::HalfAMan => "Half-a-man",
            PromotionMail::Qualified => "Qualified",
            PromotionMail::Capable => "Capable",
            PromotionMail::GoGetter => "GoGetter",
            PromotionMail::Professional => "Professional",
            PromotionMail::Expert => "Expert",
            PromotionMail::IronMan => "IronMan",
            PromotionMail::Master => "Master",
            PromotionMail::GrandMaster => "GrandMaster",
        };
        write!(f, "{s}")
    }
}
