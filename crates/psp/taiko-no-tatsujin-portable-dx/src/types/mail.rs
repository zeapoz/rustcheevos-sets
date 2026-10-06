use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;
use rustcheevos::types::{chain::Chain, memory::MemoryRef};

use crate::mem;
use crate::types::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mail {
    ApprenticePromotion = 0x4e,
    NovicePromotion = 0x4f,
    DisciplePromotion = 0x50,
    HalfAManPromotion = 0x51,
    QualifiedPromotion = 0x52,
    CapablePromotion = 0x53,
    GoGetterPromotion = 0x54,
    ProfessionalPromotion = 0x55,
    ExpertPromotion = 0x56,
    IronManPromotion = 0x57,
    MasterPromotion = 0x58,
    GrandMasterPromotion = 0x59,
}

impl Mail {
    pub fn is_unseen(self) -> Chain<Condition> {
        self.seen_state().eq(0x0)
    }

    pub fn is_seen(self) -> Chain<Condition> {
        self.seen_state().eq(0x1)
    }

    fn seen_state(self) -> Chain<MemoryRef> {
        let offset = mem::recieved_mail_flags() + self as usize;
        chain!(Profile::offset_for_current(), bits8!(offset))
    }
}
