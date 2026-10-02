use rustcheevos::{
    prelude::*,
    types::requirement::{Arithmetic, Condition},
};

use crate::mem;

pub struct Profile;

impl Profile {
    pub const STRIDE: u32 = 0xDE34;

    /// Returns the offset for the current profile.
    pub fn offset_for_current() -> Arithmetic {
        add_address!(mem::currently_selected_profile().mul(Self::STRIDE))
    }

    /// Returns a chain evaluating that profiles have been loaded.
    pub fn is_loaded() -> Condition {
        mem::save_data_loaded_flag().eq(0x1)
    }
}
