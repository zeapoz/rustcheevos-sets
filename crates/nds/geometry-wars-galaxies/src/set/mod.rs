use rustcheevos::types::game::AchievementSet;

pub mod challenge;
pub mod medal;
pub mod misc;

/// Generates and returns the core achievement set.
pub fn generate_set() -> AchievementSet {
    let mut set = AchievementSet::new();

    medal::add_galaxy_medal_achievements(&mut set);
    misc::add_drone_achievements(&mut set);
    misc::add_misc_achievements(&mut set);
    challenge::add_retro_evolved_achievements(&mut set);
    challenge::add_galaxy_challenge_achievements(&mut set);

    set
}
