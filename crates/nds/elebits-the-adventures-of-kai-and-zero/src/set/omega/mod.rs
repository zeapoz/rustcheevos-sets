use rustcheevos::{
    prelude::*,
    types::achievement::{Achievement, Tag},
};

use crate::{
    mem,
    set::omega::{bonus::generate_loaded_watts_achievements, evolve::generate_evolve_achievements},
    types::{
        game::Game,
        game_state::GameState,
        location::Location,
        omega::{Omega, OmegaState},
    },
};

mod bonus;
mod evolve;

#[rustfmt::skip]
pub fn generate_omega_achievements() -> Vec<Achievement> {
    let mut result = vec![
        obtain_standard_omega_achievement(626306, 712140, "Playing with Fire", "Help a troubled villager and obtain the Fire Omega", Omega::Fire, Location::ElebitForest, true, 3),
        obtain_standard_omega_achievement(626307, 712141, "Ice to Meet You", "Defrost a creature encased in ice and obtain the Ice Omega", Omega::Ice, Location::ElebitForest, true, 3),
        obtain_standard_omega_achievement(626308, 712142, "Warp Speed Ahead", "Smash open an ancient stone and obtain the Warp Omega", Omega::Warp, Location::ElebitForest, true, 3),
        obtain_standard_omega_achievement(626309, 712143, "Heavy Hitter", "Defeat and obtain the Power Omega", Omega::Power, Location::ElebitForest, true, 10),
        obtain_standard_omega_achievement(626310, 712144, "In Tune", "Play a major scale and obtain the Melody Omega", Omega::Melody, Location::ElebitMine, true, 3),
        obtain_standard_omega_achievement(626311, 712145, "Opposites Attract", "Help out the Pickle Gang and obtain the Magnet Omega", Omega::Magnet, Location::ElebitMine, true, 3),
        obtain_standard_omega_achievement(626312, 712146, "On Your Radar", "Jump the precarious blocks and obtain the Radar Omega", Omega::Radar, Location::ElebitMine, true, 3),
        obtain_standard_omega_achievement(626313, 712147, "Battle Beneath the Surface", "Defeat and obtain the Earth Omega", Omega::Earth, Location::ElebitMine, true, 10),
        obtain_standard_omega_achievement(626314, 712148, "Bulking Up", "Traverse the ancient labyrinth and obtain the X Power Omega", Omega::XPower, Location::ResortIsland, true, 3),
        obtain_standard_omega_achievement(626315, 712149, "Surf's Up", "Chase down and obtain the Surf Omega", Omega::Surf, Location::ResortIsland, true, 3),
        obtain_standard_omega_achievement(626316, 712150, "Gone with the Wind", "Calm down and obtain the Wind Omega", Omega::Wind, Location::ResortIsland, true, 3),
        obtain_standard_omega_achievement(626317, 712151, "Soak It Up", "Dry out a pond and obtain the Sponge Omega", Omega::Sponge, Location::ResortIsland, true, 3),
        obtain_standard_omega_achievement(626318, 712152, "Testing the Waters", "Balance water with fire and obtain the Water Omega", Omega::Water, Location::ResortIsland, true, 3),
        obtain_standard_omega_achievement(626319, 712153, "Picking up Steam", "Defeat and obtain the X Fire Omega", Omega::XFire, Location::ResortIsland, true, 10),
        obtain_standard_omega_achievement(626320, 712154, "Second Wind", "Start the turbine and obtain the X Wind Omega", Omega::XWind, Location::IceWorld, true, 3),
        obtain_standard_omega_achievement(626321, 712155, "Breaking the Ice", "Defeat and obtain the X Ice Omega", Omega::XIce, Location::IceWorld, true, 10),
        obtain_standard_omega_achievement(626322, 712156, "Too Fast to Follow", "Corner and obtain the Speed Omega", Omega::Speed, Location::RuinedWorld, true, 3),
        obtain_standard_omega_achievement(626323, 712157, "Tectonic Takedown", "Defeat and obtain the X Earth Omega", Omega::XEarth, Location::RuinedWorld, true, 10),
        obtain_standard_omega_achievement(626324, 712158, "Magnetic Personality", "Master magnetism and obtain the X Magnet Omega", Omega::XMagnet, Location::SeaTemple, true, 3),
        obtain_standard_omega_achievement(626325, 712159, "Smooth Sailing", "Defeat and obtain the X Water Omega", Omega::XWater, Location::SeaTemple, true, 10),
        obtain_standard_omega_achievement(626326, 712160, "Mirror, Mirror", "Guide the light and obtain the Mirror Omega", Omega::Mirror, Location::LibraOfCrystal, true, 3),
        obtain_standard_omega_achievement(626327, 712161, "Perfect Pitch", "Play a chime and obtain the X Melody Omega", Omega::XMelody, Location::LibraOfCrystal, true, 3),
        obtain_standard_omega_achievement(626328, 712162, "Frozen in Time", "Bare witness to a time freeze and obtain the Time Omega", Omega::Time, Location::LibraOfCrystal, true, 3),
        obtain_standard_omega_achievement(626329, 712163, "Blown Away", "Find and obtain the Storm Omega in a distant forest", Omega::Storm, Location::ElebitForest, false, 5),
        obtain_standard_omega_achievement(626330, 712164, "Digging It", "Find and obtain the Land Omega in a puzzling taiga", Omega::Land, Location::IceWorld, false, 5),
        obtain_standard_omega_achievement(626331, 712165, "Cleared for Takeoff", "Find and obtain the Flight Omega on a frozen lake", Omega::Flight, Location::IceWorld, false, 5),
        obtain_standard_omega_achievement(626332, 712166, "Cold-Blooded", "Find and obtain the Blizzard Omega on a precarious cliff", Omega::Blizzard, Location::IceWorld, false, 5),
        obtain_standard_omega_achievement(626333, 712167, "Muscle Memory", "Find and obtain the Strong Omega deep below", Omega::Strong, Location::ElebitMine, false, 5),
        obtain_standard_omega_achievement(626334, 712168, "Too Hot to Handle", "Find and obtain the Flame Omega where lava roars", Omega::Flame, Location::RuinedWorld, false, 5),
        obtain_standard_omega_achievement(626335, 712169, "Making Waves", "Find and obtain the Aqua Omega in a forgotten temple", Omega::Aqua, Location::SeaTemple, false, 5),
    ];

    result.extend(generate_evolve_achievements());
    result.extend(generate_loaded_watts_achievements());
    result
}

pub fn obtain_standard_omega_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    description: &str,
    omega: Omega,
    world: Location,
    progression: bool,
    points: u32,
) -> Achievement {
    let mut builder = Achievement::builder(title)
        .description(description)
        .core(chain!(
            delta!(omega.obtained_state().eq(OmegaState::NotObtained as u32)),
            omega.obtained_state().eq(OmegaState::Obtained as u32),
            mem::current_game_scene().eq(world as u32),
            mem::game_state().eq(GameState::Overworld.id()),
            Game::in_game(),
        ))
        .points(points)
        .id(id)
        .badge_id(badge_id);

    if progression {
        builder = builder.tag(Tag::Progression)
    }

    builder.build()
}
