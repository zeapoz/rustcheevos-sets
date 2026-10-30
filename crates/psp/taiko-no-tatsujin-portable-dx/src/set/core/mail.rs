use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::types::{
    mail::Mail,
    menu::{GeneralMenuState, MenuState},
    profile::Profile,
};

pub fn recieved_mail(description: &'static str, mail: Mail) -> Achievement {
    Achievement::builder("TBD - Recieved Mail")
        .description(description)
        .core(chain!(
            delta!(mail.is_unseen()),
            mail.is_seen(),
            MenuState::is(GeneralMenuState::Mail as u32),
            Profile::is_loaded(),
        ))
        .build()
}
