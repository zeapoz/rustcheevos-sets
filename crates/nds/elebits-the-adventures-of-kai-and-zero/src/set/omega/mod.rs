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
        obtain_standard_omega_achievement("Playing with Fire", "Help a troubled villager and obtain the Fire Omega", Omega::Fire, Location::ElebitForest, 3),
        obtain_standard_omega_achievement("Ice to Meet You", "Defrost a creature encased in ice and obtain the Ice Omega", Omega::Ice, Location::ElebitForest, 3),
        obtain_standard_omega_achievement("Warp Speed Ahead", "Smash open an ancient stone and obtain the Warp Omega", Omega::Warp, Location::ElebitForest, 3),
        obtain_standard_omega_achievement("Heavy Hitter", "Defeat and obtain the Power Omega", Omega::Power, Location::ElebitForest, 10),
        obtain_standard_omega_achievement("In Tune", "Play a major scale and obtain the Melody Omega", Omega::Melody, Location::ElebitMine, 3),
        obtain_standard_omega_achievement("Opposites Attract", "Help out the pickle gang and obtain the Magnet Omega", Omega::Magnet, Location::ElebitMine, 3),
        obtain_standard_omega_achievement("On Your Radar", "Jump the precarious blocks and obtain the Radar Omega", Omega::Radar, Location::ElebitMine, 3),
        obtain_standard_omega_achievement("Battle Beneath the Surface", "Defeat and obtain the Earth Omega", Omega::Earth, Location::ElebitMine, 10),
        obtain_standard_omega_achievement("Bulking Up", "Traverse the ancient labyrinth and obtain the X Power Omega", Omega::XPower, Location::ResortIsland, 3),
        obtain_standard_omega_achievement("Surf's Up", "Chase down and obtain the Surf Omega", Omega::Surf, Location::ResortIsland, 3),
        obtain_standard_omega_achievement("Gone with the Wind", "Calm down and obtain the Wind Omega", Omega::Wind, Location::ResortIsland, 3),
        obtain_standard_omega_achievement("Soak It Up", "Dry out a pond and obtain the Sponge Omega", Omega::Sponge, Location::ResortIsland, 3),
        obtain_standard_omega_achievement("Testing the Waters", "Balance water with fire and obtain the Water Omega", Omega::Water, Location::ResortIsland, 3),
        obtain_standard_omega_achievement("Picking up Steam", "Defeat and obtain the X Fire Omega", Omega::XFire, Location::ResortIsland, 10),
        obtain_standard_omega_achievement("Second Wind", "Start the turbine and obtain the X Wind Omega", Omega::XWind, Location::IceWorld, 3),
        obtain_standard_omega_achievement("Breaking the Ice", "Defeat and obtain the X Ice Omega", Omega::XIce, Location::IceWorld, 10),
        obtain_standard_omega_achievement("Too Fast to Follow", "Corner and obtain the Speed Omega", Omega::Speed, Location::RuinedWorld, 3),
        obtain_standard_omega_achievement("Tectonic Takedown", "Defeat and obtain the X Earth Omega", Omega::XEarth, Location::RuinedWorld, 10),
        obtain_standard_omega_achievement("Magnetic Personality", "Master magnetism and obtain the X Magnet Omega", Omega::XMagnet, Location::SeaTemple, 3),
        obtain_standard_omega_achievement("Smooth Sailing", "Defeat and obtain the X Water Omega", Omega::XWater, Location::SeaTemple, 10),
        obtain_standard_omega_achievement("Mirror, Mirror", "Guide the light and obtain the Mirror Omega", Omega::Mirror, Location::LibraOfCrystal, 3),
        obtain_standard_omega_achievement("Perfect Pitch", "Play a chime and obtain the X Melody Omega", Omega::XMelody, Location::LibraOfCrystal, 3),
        obtain_standard_omega_achievement("Frozen in Time", "Discover and obtain the Time Omega", Omega::Time, Location::LibraOfCrystal, 3),
        obtain_standard_omega_achievement("Blown Away", "Find and obtain the Storm Omega in a distant forest", Omega::Storm, Location::ElebitForest, 5),
        obtain_standard_omega_achievement("Digging It", "Find and obtain the Land Omega in a puzzling taiga", Omega::Land, Location::IceWorld, 5),
        obtain_standard_omega_achievement("Cleared for Takeoff", "Find and obtain the Flight Omega on a frozen lake", Omega::Flight, Location::IceWorld, 5),
        obtain_standard_omega_achievement("Cold-Blooded", "Find and obtain the Blizzard Omega on a precarious cliff", Omega::Blizzard, Location::IceWorld, 5),
        obtain_standard_omega_achievement("Muscle Memory", "Find and obtain the Strong Omega deep below", Omega::Strong, Location::ElebitMine, 5),
        obtain_standard_omega_achievement("Too Hot to Handle", "Find and obtain the Flame Omega where lava roars", Omega::Flame, Location::RuinedWorld, 5),
        obtain_standard_omega_achievement("Making Waves", "Find and obtain the Aqua Omega in a forgotten temple", Omega::Aqua, Location::SeaTemple, 5),
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
    points: u32,
) -> Achievement {
    let requirements = ChainGroup::new(chain!(
        delta!(omega.obtained_state().eq(OmegaState::NotObtained as u32)),
        omega.obtained_state().eq(OmegaState::Obtained as u32),
        mem::current_game_scene().eq(world as u32),
        Game::in_game(),
    ));

    Achievement::builder(title)
        .description(description)
        .requirements(requirements)
        .points(points)
        .tag(Tag::Progression)
        .build()
}
