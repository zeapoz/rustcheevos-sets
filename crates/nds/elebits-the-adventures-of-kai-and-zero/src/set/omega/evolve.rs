use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::Chain},
};

use crate::{
    mem,
    types::{
        game::Game,
        game_state::GameState,
        omega::{Omega, OmegaState},
    },
};

#[rustfmt::skip]
pub fn generate_evolve_achievements() -> Vec<Achievement> {
    vec![
        evolve_omega_achivement(),
        evolve_half_achivement(),
    ]
}

fn evolve_omega_achivement() -> Achievement {
    let all_evolvable_omegas_evolved = Omega::all_evolvable().into_iter().map(|o| {
        chain!(
            delta!(o.obtained_state().eq(OmegaState::Obtained as u32)),
            o.obtained_state().eq(OmegaState::Evolved as u32),
        )
    });

    Achievement::builder("Omega Charger")
        .description("Evolve an Omega into its adult form")
        .core(chain!(
            mem::game_state().eq(GameState::Overworld.id()),
            Game::in_game()
        ))
        .alt_groups(all_evolvable_omegas_evolved)
        .points(5)
        .id(626336)
        .badge_id(712170)
        .build()
}

fn evolve_half_achivement() -> Achievement {
    let num_evolvable = Omega::all_evolvable().len();
    let all_evolvable: Chain = Omega::all_evolvable()[..num_evolvable - 1]
        .into_iter()
        .map(|o| add_source!(bit1!(o.obtained_addr())))
        .collect();
    let alt_groups: Vec<_> = Omega::all_evolvable()
        .into_iter()
        .map(|o| delta!(o.obtained_state().eq(OmegaState::Obtained as u32)))
        .collect();
    let core = chain!(
        all_evolvable,
        measured!(
            bit1!(Omega::all_evolvable().last().unwrap().obtained_addr())
                .eq((num_evolvable / 2) as u32)
        ),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        or_next!(mem::game_state().eq(GameState::InBossFight.id())),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
        Game::in_game(),
    );

    Achievement::builder("Omega Conduit")
        .description("Evolve 12 Omegas into their adult forms")
        .core(core)
        .alt_groups(alt_groups)
        .points(10)
        .id(626337)
        .badge_id(712171)
        .build()
}
