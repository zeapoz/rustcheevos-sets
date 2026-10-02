use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::ResolvedChain},
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
    let all_evolvable: ResolvedChain = Omega::all_evolvable()
        .into_iter()
        .map(|o| add_source!(o.obtained_state()).div(OmegaState::Evolved as u32))
        .collect();
    let all_evolvable_delta: ResolvedChain = all_evolvable.iter().map(|r| r.delta()).collect();

    Achievement::builder("Omega Charger")
        .description("Evolve an Omega into its adult form")
        .core(chain!(
            all_evolvable_delta,
            0.eq(0),
            all_evolvable,
            0.eq(1),
            mem::game_state().eq(GameState::Overworld.id()),
            Game::in_game()
        ))
        .points(5)
        .id(626336)
        .badge_id(712170)
        .build()
}

fn evolve_half_achivement() -> Achievement {
    let num_evolvable = Omega::all_evolvable().len();
    let half_num_evolvable = (num_evolvable / 2) as u32;

    let all_evolvable: ResolvedChain = Omega::all_evolvable()
        .into_iter()
        .map(|o| add_source!(o.obtained_state()).div(OmegaState::Evolved as u32))
        .collect();
    let all_evolvable_delta: ResolvedChain = all_evolvable.iter().map(|r| r.delta()).collect();

    let core = chain!(
        all_evolvable_delta,
        0.eq(half_num_evolvable - 1),
        all_evolvable,
        measured!(0.eq(half_num_evolvable)),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
        Game::in_game(),
    );

    Achievement::builder("Omega Conduit")
        .description("Evolve 12 Omegas into their adult forms")
        .core(core)
        .points(10)
        .id(626337)
        .badge_id(712171)
        .build()
}
