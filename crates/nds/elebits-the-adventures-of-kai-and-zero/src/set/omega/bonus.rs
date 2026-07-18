use rustcheevos::{
    add_address, add_source, bit0, bit1, bitcount, chain, delta, measured, measured_if, or_next,
    prelude::*,
    sub_source, trigger,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
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
        loaded_watts_achievement("First Charge", "Load 2500 Watts into the Capture Gun and obtain the Takosuke Omega", Omega::Takosuke, 2500),
        loaded_watts_achievement("Powering Up", "Load 5000 Watts into the Capture Gun and obtain the Penta Omega", Omega::Penta, 5000),
        loaded_watts_achievement("High Voltage", "Load 7500 Watts into the Capture Gun and obtain the Twinbee Omega", Omega::TwinBee, 7500),
        loaded_watts_achievement("Full Power! Maximum Capacity!", "Load 10000 Watts into the Capture Gun and obtain the Moai Omega", Omega::Moai, 10000),
        obtain_dewy(),
        obtain_big_green(),
        obtain_big_red(),
    ]
}

fn obtain_night() -> Achievement {
    let requirements = ChainGroup::new(chain!(
        delta!(
            Omega::Night
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        ),
        Omega::Night
            .obtained_state()
            .eq(OmegaState::Obtained as u32),
        mem::current_game_scene().eq(Location::SeaTemple.id()),
        sub_source!(bit0!(mem::diary_scrap_flags())),
        measured!(bitcount!(mem::diary_scrap_flags()).eq(7)),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
        Game::in_game(),
    ));

    Achievement::builder("Echoes of the Past")
        .description("Gather all diary scraps and obtain Night")
        .requirements(requirements)
        .build()
}

fn obtain_dewy() -> Achievement {
    let standard_omegas_len = Omega::all_standard().len();
    let all_standard_omegas_obtained: Chain = Omega::all_standard()[..standard_omegas_len - 1]
        .into_iter()
        .map(|o| add_source!(bit0!(o.obtained_addr())))
        .collect();
    let requirements = ChainGroup::new(chain!(
        delta!(
            Omega::Dewy
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        ),
        trigger!(Omega::Dewy.obtained_state().eq(OmegaState::Obtained as u32)),
        all_standard_omegas_obtained,
        measured!(
            bit0!(Omega::all_standard().last().unwrap().obtained_addr())
                .eq(standard_omegas_len as u32)
        ),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::FileConfiguration.id())),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
    ));

    Achievement::builder("Complete Collection")
        .description("Obtain all standard Omegas and obtain Dewy")
        .requirements(requirements)
        .build()
}

fn obtain_big_green() -> Achievement {
    let evolvable_omegas_len = Omega::all_evolvable().len();
    let all_evolvable_omegas_evolved: Chain = Omega::all_evolvable()[..evolvable_omegas_len - 1]
        .into_iter()
        .map(|o| add_source!(bit1!(o.obtained_addr())))
        .collect();
    let requirements = ChainGroup::new(chain!(
        delta!(
            Omega::BigGreen
                .obtained_state()
                .eq(OmegaState::NotObtained as u32)
        ),
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
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::FileConfiguration.id())),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
    ));

    Achievement::builder("Omega Overdrive")
        .description("Evolve all evolvable Omegas into their adult forms and obtain Big Green")
        .requirements(requirements)
        .build()
}

fn obtain_big_red() -> Achievement {
    Achievement::builder("Code Red")
        .description("Enter a secret code and obtain the Big Red Omega")
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
    title: &str,
    description: &str,
    omega: Omega,
    target: u32,
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
        .build()
}
