use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;
use rustcheevos::types::{chain::Chain, memory::MemoryRef};

use crate::mem;
use crate::types::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mail {
    ApprenticePromotion = 0x4e,
    NovicePromotion = 0x4f,
    HalfAManPromotion = 0x50,
    QualifiedPromotion = 0x51,
    CapablePromotion = 0x52,
    GoGetterPromotion = 0x53,
    ProfessionalPromotion = 0x54,
    ExpertPromotion = 0x55,
    IronManPromotion = 0x56,
    MasterPromotion = 0x57,
    GrandMasterPromotion = 0x58,
}

impl Mail {
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
