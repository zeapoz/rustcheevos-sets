use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::types::{
    mail::PromotionMail,
    menu::{GeneralMenuState, MenuState},
    profile::Profile,
};

pub fn recieved_mail(title: &'static str, points: u32, mail: PromotionMail) -> Achievement {
    Achievement::builder(title)
        .description(format!("Achieve the rank of {mail}"))
        .core(chain!(
            delta!(mail.is_unseen()),
            mail.is_seen(),
            MenuState::is(GeneralMenuState::Mail as u32),
            Profile::is_loaded(),
        ))
        .points(points)
        .build()
}
