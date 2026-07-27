use rustcheevos::{
    add_source, bit1, chain, delta, measured, measured_if, or_next,
    prelude::*,
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
    let all_evolvable_omegas_evolved: Vec<_> = Omega::all_evolvable()
        .into_iter()
        .map(|o| {
            chain!(
                delta!(o.obtained_state().eq(OmegaState::Obtained as u32)),
                o.obtained_state().eq(OmegaState::Evolved as u32),
            )
        })
        .collect();
    let mut requirements = ChainGroup::new(chain!(
        mem::game_state().eq(GameState::Overworld.id()),
        Game::in_game()
    ));
    for cond in all_evolvable_omegas_evolved {
        requirements.push_alt_group(cond);
    }

    Achievement::builder("Omega Charger")
        .description("Evolve an Omega into its adult form")
        .requirements(requirements)
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

    let mut requirements = ChainGroup::new(chain!(
        all_evolvable,
        measured!(
            bit1!(Omega::all_evolvable().last().unwrap().obtained_addr())
                .eq((num_evolvable / 2) as u32)
        ),
        or_next!(mem::game_state().eq(GameState::TransitioningWorldCutscene.id())),
        measured_if!(mem::game_state().eq(GameState::Overworld.id())),
        Game::in_game()
    ));
    for cond in alt_groups {
        requirements.push_alt_group(cond);
    }

    Achievement::builder("Omega Conduit")
        .description("Evolve 12 Omegas into their adult forms")
        .requirements(requirements)
        .points(5)
        .id(626337)
        .badge_id(712171)
        .build()
}
