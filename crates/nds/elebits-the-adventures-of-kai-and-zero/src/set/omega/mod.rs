use rustcheevos::{
    chain, delta,
    prelude::*,
    types::{
        achievement::{Achievement, Tag},
        chain::ChainGroup,
    },
};

use crate::{
    mem,
    set::omega::{bonus::generate_loaded_watts_achievements, evolve::generate_evolve_achievements},
    types::{
        game::Game,
        location::Location,
        omega::{Omega, OmegaState},
    },
};

mod bonus;
mod evolve;

#[rustfmt::skip]
pub fn generate_omega_achievements() -> Vec<Achievement> {
    let mut result = vec![
        obtain_standard_omega_achievement("Playing with Fire", "Help a troubled villager and obtain the Fire Omega", Omega::Fire, Location::ElebitForest),
        obtain_standard_omega_achievement("Ice to Meet You", "Defrost a creature encased and obtain the Ice Omega", Omega::Ice, Location::ElebitForest),
        obtain_standard_omega_achievement("Warp Speed Ahead", "Discover an ancient stone and obtain the Warp Omega", Omega::Warp, Location::ElebitForest),
        obtain_standard_omega_achievement("In Tune", "Play a scale and obtain the Melody Omega", Omega::Melody, Location::ElebitMines),
        obtain_standard_omega_achievement("Opposites Attract", "Encounter a mysterious kid and obtain the Magnet Omega", Omega::Magnet, Location::ElebitMines),
        obtain_standard_omega_achievement("On Your Radar", "Jump the precarious blocks and obtain the Radar Omega", Omega::Radar, Location::ElebitMines),
        obtain_standard_omega_achievement("Bulking Up", "Traverse the ancient labyrinth and obtain the X Power Omega", Omega::XPower, Location::ResortIsland),
        obtain_standard_omega_achievement("Surf's Up", "Chase and obtain the Surf Omega", Omega::Surf, Location::ResortIsland),
        obtain_standard_omega_achievement("Gone with the Wind", "Calm down and obtain the Wind Omega", Omega::Wind, Location::ResortIsland),
        obtain_standard_omega_achievement("Soak It Up", "Dry out the pond and obtain the Sponge Omega", Omega::Sponge, Location::ResortIsland),
        obtain_standard_omega_achievement("Testing the Waters", "Balance water with fire and obtain the Water Omega", Omega::Water, Location::ResortIsland),
        obtain_standard_omega_achievement("Second Wind", "Start the turbine and obtain the X Wind Omega", Omega::XWind, Location::IceWorld),
        obtain_standard_omega_achievement("Too Fast to Follow", "Corner and obtain the Speed Omega", Omega::Speed, Location::RuinedWorld),
        obtain_standard_omega_achievement("Magnetic Personality", "Master magnetism and obtain the X Magnet Omega", Omega::XMagnet, Location::SeaTemple),
        obtain_standard_omega_achievement("Mirror, Mirror", "Guide the light and obtain the Mirror Omega", Omega::Mirror, Location::LibraOfCrystal),
        obtain_standard_omega_achievement("Perfect Pitch", "Play a chime and obtain the X Melody Omega", Omega::XMelody, Location::LibraOfCrystal),
        obtain_standard_omega_achievement("Frozen in Time", "Discover and obtain the Time Omega", Omega::Time, Location::LibraOfCrystal),
        obtain_standard_omega_achievement("Blown Away", "Find and obtain the Storm Omega", Omega::Storm, Location::ElebitForest),
        obtain_standard_omega_achievement("Digging It", "Find and obtain the Land Omega", Omega::Land, Location::IceWorld),
        obtain_standard_omega_achievement("Cleared for Takeoff", "Find and obtain the Flight Omega", Omega::Flight, Location::IceWorld),
        obtain_standard_omega_achievement("Cold-Blooded", "Find and obtain the Blizzard Omega", Omega::Blizzard, Location::IceWorld),
        obtain_standard_omega_achievement("Muscle Memory", "Find and obtain the Strong Omega", Omega::Strong, Location::ElebitMines),
        obtain_standard_omega_achievement("Too Hot to Handle", "Find and obtain the Flame Omega", Omega::Flame, Location::RuinedWorld),
        obtain_standard_omega_achievement("Making Waves", "Find and obtain the Aqua Omega", Omega::Aqua, Location::SeaTemple),
    ];

    result.extend(generate_evolve_achievements());
    result.extend(generate_loaded_watts_achievements());
    result
}

pub fn obtain_standard_omega_achievement(
    title: &str,
    description: &str,
    omega: Omega,
    world: Location,
) -> Achievement {
    let requirements = ChainGroup::new(chain!(
        delta!(omega.obtained_state().eq(OmegaState::NotObtained as u32)),
        omega.obtained_state().eq(OmegaState::Obtained as u32),
        mem::current_game_scene().eq(world as u32), // FIXME: Maybe not needed? But probably not hurtful.
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .tag(Tag::Progression)
        .build()
}
