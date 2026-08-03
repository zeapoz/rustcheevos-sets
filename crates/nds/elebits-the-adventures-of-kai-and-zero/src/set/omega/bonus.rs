use rustcheevos::{
    add_address, add_hits, add_source, and_next, bit0, bit1, bitcount, chain, delta, measured,
    measured_if, or_next,
    prelude::*,
    reset_if, sub_source, trigger,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
        requirement::Condition,
    },
};

use crate::{
    mem,
    types::{
        game::Game,
        game_state::GameState,
        location::Location,
        omega::{Omega, OmegaState},
    },
};

#[rustfmt::skip]
pub fn generate_loaded_watts_achievements() -> Vec<Achievement> {
    vec![
        obtain_night(),
        loaded_watts_achievement(626339, 712173, "First Charge", "Load 2,500 Watts into the Capture Gun and obtain Takosuke", Omega::Takosuke, 2500, 3),
        loaded_watts_achievement(626340, 712174, "Powering Up", "Load 5,000 Watts into the Capture Gun and obtain Penta", Omega::Penta, 5000, 5),
        loaded_watts_achievement(626341, 712175, "High Voltage", "Load 7,500 Watts into the Capture Gun and obtain Twinbee", Omega::TwinBee, 7500, 10),
        loaded_watts_achievement(626342, 712176, "Full Power! Maximum Capacity!", "Load 10,000 Watts into the Capture Gun and obtain Moai", Omega::Moai, 10000, 10),
        obtain_dewy(),
        obtain_big_green(),
        obtain_big_red(),
    ]
}

fn obtain_night() -> Achievement {
    let requirements = ChainGroup::new(chain!(
        Omega::Night
            .obtained_state()
            .eq(OmegaState::Obtained as u32),
        mem::current_game_scene().eq(Location::SeaTemple.id()),
        sub_source!(bit0!(mem::diary_scrap_flags())),
        measured!(bitcount!(mem::diary_scrap_flags()).eq(7)),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::InBossFight.id())),
        and_next!(mem::game_state().eq(GameState::Overworld.id())),
        measured_if!(delta!(
            Omega::Night
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        )),
        Game::in_game(),
    ));

    Achievement::builder("Echoes of the Past")
        .description("Gather all diary scraps and obtain Night")
        .requirements(requirements)
        .points(3)
        .id(626338)
        .badge_id(712172)
        .build()
}

fn obtain_dewy() -> Achievement {
    let standard_omegas_len = Omega::all_standard().len();
    let all_standard_omegas_obtained: Chain = Omega::all_standard()
        .into_iter()
        .map(|o| add_hits!(o.obtained_state().ne(OmegaState::NotObtained as u32)).with_hits(1))
        .collect();

    let requirements = ChainGroup::new(chain!(
        trigger!(Omega::Dewy.obtained_state().eq(OmegaState::Obtained as u32)),
        all_standard_omegas_obtained,
        measured!(Condition::always_false().with_hits(standard_omegas_len as u32)),
        or_next!(mem::game_state().eq(GameState::FileConfiguration.id())),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::InBossFight.id())),
        and_next!(mem::game_state().eq(GameState::Overworld.id())),
        measured_if!(delta!(
            Omega::Dewy
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        )),
        reset_if!(mem::currently_selected_file().ne(delta!(mem::currently_selected_file()))),
    ));

    Achievement::builder("Complete Collection")
        .description("Obtain all standard Omegas and obtain Dewy")
        .requirements(requirements)
        .points(10)
        .id(626343)
        .badge_id(712177)
        .build()
}

fn obtain_big_green() -> Achievement {
    let evolvable_omegas_len = Omega::all_evolvable().len();
    let all_evolvable_omegas_evolved: Chain = Omega::all_evolvable()[..evolvable_omegas_len - 1]
        .into_iter()
        .map(|o| add_source!(bit1!(o.obtained_addr())))
        .collect();
    let requirements = ChainGroup::new(chain!(
        trigger!(
            Omega::BigGreen
                .obtained_state()
                .eq(OmegaState::Obtained as u32)
        ),
        all_evolvable_omegas_evolved,
        measured!(
            bit1!(Omega::all_evolvable().last().unwrap().obtained_addr())
                .eq(evolvable_omegas_len as u32)
        ),
        or_next!(mem::game_state().eq(GameState::FileConfiguration.id())),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::InBossFight.id())),
        and_next!(mem::game_state().eq(GameState::Overworld.id())),
        measured_if!(delta!(
            Omega::BigGreen
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        )),
    ));

    Achievement::builder("Omega Overdrive")
        .description("Evolve all evolvable Omegas into their adult forms and obtain Big Green")
        .requirements(requirements)
        .points(25)
        .id(626344)
        .badge_id(712178)
        .build()
}

fn obtain_big_red() -> Achievement {
    Achievement::builder("Code Red")
        .description("Enter a secret code and obtain Big Red")
        .points(1)
        .id(626345)
        .badge_id(712179)
        .requirements(chain!(
            delta!(
                Omega::BigRed
                    .obtained_state()
                    .eq(OmegaState::NotObtained as u32)
            ),
            Omega::BigRed
                .obtained_state()
                .eq(OmegaState::Obtained as u32),
            mem::game_state().eq(GameState::DownloadAdditionalOmegas as u32),
            mem::wi_fi_connecting_flag().eq(1),
        ))
        .build()
}

fn loaded_watts_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    omega: Omega,
    target: u32,
    points: u32,
) -> Achievement {
    const SAVE_FILE_DATA_STRIDE_BYTES: u32 =
        (mem::omega_vector_save_data_file_2() - mem::omega_vector_save_data_file_1()) as u32;
    let requirements = ChainGroup::new(chain!(
        add_address!(mem::currently_selected_file().mul(SAVE_FILE_DATA_STRIDE_BYTES)),
        delta!(
            omega
                .save_data_obtained_state()
                .eq(OmegaState::NotObtained as u32)
        ),
        add_address!(mem::currently_selected_file().mul(SAVE_FILE_DATA_STRIDE_BYTES)),
        trigger!(
            omega
                .save_data_obtained_state()
                .eq(OmegaState::Obtained as u32)
        ),
        trigger!(mem::game_state().eq(GameState::FileConfiguration.id())),
        mem::loaded_watts().ge(target),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(points)
        .id(id)
        .badge_id(badge_id)
        .build()
}
