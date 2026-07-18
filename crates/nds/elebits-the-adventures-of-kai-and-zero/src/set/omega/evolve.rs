use rustcheevos::{
    add_source, chain, delta,
    prelude::*,
    types::{
        achievement::Achievement,
        chain::{Chain, ChainGroup},
    },
};

use crate::types::{
    game::Game,
    omega::{Omega, OmegaState},
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
    let mut requirements = ChainGroup::new(chain!(Game::in_game()));
    for cond in all_evolvable_omegas_evolved {
        requirements.push_alt_group(cond);
    }

    Achievement::builder("Omega Charger")
        .description("Evolve an Omega into its adult form")
        .requirements(requirements)
        .build()
}

fn evolve_half_achivement() -> Achievement {
    let num_evolvable = Omega::all_evolvable().len();
    let all_evolvable: Chain = Omega::all_evolvable()[..num_evolvable.saturating_sub(1)]
        .into_iter()
        .map(|o| add_source!(o.obtained_state()))
        .collect();
    let alt_groups: Vec<_> = Omega::all_evolvable()
        .into_iter()
        .map(|o| delta!(o.obtained_state().eq(OmegaState::Evolved as u32)))
        .collect();

    let mut requirements = ChainGroup::new(chain!(
        all_evolvable,
        Omega::all_evolvable()
            .last()
            .unwrap()
            .obtained_state()
            .eq((num_evolvable as u32 / 2) * OmegaState::Evolved as u32)
    ));
    for cond in alt_groups {
        requirements.push_alt_group(cond);
    }

    Achievement::builder("Omega Conduit")
        .description("Evolve 12 Omegas into their master form")
        .requirements(requirements)
        .build()
}
