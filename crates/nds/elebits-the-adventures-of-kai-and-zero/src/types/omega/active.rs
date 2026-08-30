use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef, requirement::Arithmetic},
};

use crate::{mem, types::omega::Omega};

pub struct ActiveOmega;

impl ActiveOmega {
    /// Returns a chain that corresponds to the ID of the currently active Omega.
    pub fn id() -> Chain<MemoryRef> {
        chain!(
            add_address!(mem::active_omega_index().mul(Omega::STRUCT_SIZE_BYTES)),
            bits32!(mem::active_omega_party() + Omega::ID_OFFSET)
        )
    }

    pub fn form() -> Chain<Arithmetic> {
        chain!(
            add_address!(mem::active_omega_index().mul(Omega::STRUCT_SIZE_BYTES)),
            measured!(bits8!(mem::active_omega_party() + Omega::OBTAINED_OFFSET))
        )
    }
}
