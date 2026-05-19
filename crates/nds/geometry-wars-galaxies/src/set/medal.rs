use rustcheevos::{
    chain, delta,
    prelude::*,
    types::{
        achievement::{Achievement, Tag},
        chain::{Chain, ChainGroup},
        game::AchievementSet,
    },
};

use crate::types::{galaxy::Galaxy, game::Game, planet::Planet, status::MedalStatus};

/// Adds achievements for progression to the given set.
#[rustfmt::skip]
pub fn add_galaxy_medal_achievements(set: &mut AchievementSet) {
    set.push(new_galaxy_medal_achievement(600707, 681297, "Alpha Amateur", Galaxy::Alpha, MedalStatus::Bronze, 3, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600708, 681298, "Beta Beginner", Galaxy::Beta, MedalStatus::Bronze, 4, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600709, 681299, "Gamma Greenhorn", Galaxy::Gamma, MedalStatus::Bronze, 5, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600710, 681300, "Delta Dabbler", Galaxy::Delta, MedalStatus::Bronze, 5, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600711, 681301, "Epsilon Entrant", Galaxy::Epsilon, MedalStatus::Bronze, 5, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600712, 681302, "Zeta Hopeful", Galaxy::Zeta, MedalStatus::Bronze, 10, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600713, 681303, "Eta Initiate", Galaxy::Eta, MedalStatus::Bronze, 10, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600714, 681304, "Theta Trainee", Galaxy::Theta, MedalStatus::Bronze, 10, Some(Tag::Progression)));
    set.push(new_galaxy_medal_achievement(600715, 681305, "Kappa Cadet", Galaxy::Kappa, MedalStatus::Bronze, 10, Some(Tag::Progression)));

    set.push(new_galaxy_medal_achievement(600716, 681306, "Alpha Armor", Galaxy::Alpha, MedalStatus::Silver, 5, None));
    set.push(new_galaxy_medal_achievement(600717, 681307, "Beta Bulwark", Galaxy::Beta, MedalStatus::Silver, 5, None));
    set.push(new_galaxy_medal_achievement(600718, 681308, "Gamma Guardian", Galaxy::Gamma, MedalStatus::Silver, 5, None));
    set.push(new_galaxy_medal_achievement(600719, 681309, "Delta Defender", Galaxy::Delta, MedalStatus::Silver, 5, None));
    set.push(new_galaxy_medal_achievement(600720, 681310, "Epsilon Sentinel", Galaxy::Epsilon, MedalStatus::Silver, 5, None));
    set.push(new_galaxy_medal_achievement(600721, 681311, "Zeta Vigilante", Galaxy::Zeta, MedalStatus::Silver, 10, None));
    set.push(new_galaxy_medal_achievement(600722, 681312, "Eta Escort", Galaxy::Eta, MedalStatus::Silver, 10, None));
    set.push(new_galaxy_medal_achievement(600723, 681313, "Theta Trustee", Galaxy::Theta, MedalStatus::Silver, 10, None));
    set.push(new_galaxy_medal_achievement(600724, 681314, "Kappa Custodian", Galaxy::Kappa, MedalStatus::Silver, 10, None));

    set.push(new_galaxy_medal_achievement(600725, 681315, "Alpha Ace", Galaxy::Alpha, MedalStatus::Gold, 10, None));
    set.push(new_galaxy_medal_achievement(600726, 681316, "Beta Buff", Galaxy::Beta, MedalStatus::Gold, 10, None));
    set.push(new_galaxy_medal_achievement(600727, 681317, "Gamma Guru", Galaxy::Gamma, MedalStatus::Gold, 10, None));
    set.push(new_galaxy_medal_achievement(600728, 681318, "Delta Dab Hand", Galaxy::Delta, MedalStatus::Gold, 10, None));
    set.push(new_galaxy_medal_achievement(600729, 681319, "Epsilon Enthusiast", Galaxy::Epsilon, MedalStatus::Gold, 25, None));
    set.push(new_galaxy_medal_achievement(600730, 681320, "Zeta Virtuoso", Galaxy::Zeta, MedalStatus::Gold, 25, None));
    set.push(new_galaxy_medal_achievement(600731, 681321, "Eta Expert", Galaxy::Eta, MedalStatus::Gold, 25, None));
    set.push(new_galaxy_medal_achievement(600732, 681322, "Theta Maven", Galaxy::Theta, MedalStatus::Gold, 50, None));
    set.push(new_galaxy_medal_achievement(600733, 681323, "Kappa Connoisseur", Galaxy::Kappa, MedalStatus::Gold, 50, None));

    set.push(
        Achievement::builder("Galactic Explorer")
            .description("Unlock every galaxy excluding the Lambda galaxy")
            .requirements(Galaxy::unlocked_all_cond())
            .points(5)
            .id(600734)
            .badge_id(681324)
            .tag(Tag::Progression)
            .build(),
    );

    set.push(
        Achievement::builder("Solar System Sentinel")
            .description("Earn a Bronze medal or higher on every planet in every galaxy excluding the Lambda galaxy")
            .requirements(all_planets_medal_group(Planet::all(), MedalStatus::Bronze))
            .points(25)
            .id(600735)
            .badge_id(681325)
            .tag(Tag::WinCondition)
            .build(),
    );
}

/// Creates a chain group that requires all planets to have the given status.
fn all_planets_medal_group(planets: &[Planet], status: MedalStatus) -> ChainGroup {
    let planets_are_at_least: Chain = planets
        .iter()
        .map(|p| p.status_is_at_least(status))
        .collect();
    let core = chain!(planets_are_at_least, Game::in_game_cond_with_delta());

    let alt_groups: Vec<Chain> = planets
        .iter()
        .map(|p| delta!(p.status()).lt(status as u32))
        .collect();

    let mut group = ChainGroup::new(core);
    group.set_alt_groups(alt_groups);
    group
}

/// Creates a new galaxy medal achievement.
fn new_galaxy_medal_achievement(
    id: u32,
    badge_id: u32,
    title: &str,
    galaxy: Galaxy,
    status: MedalStatus,
    points: u32,
    tag: Option<Tag>,
) -> Achievement {
    let mut builder = Achievement::builder(title)
        .description(format!(
            "Earn a {status} medal or higher on every planet of the {galaxy} galaxy"
        ))
        .requirements(all_planets_medal_group(galaxy.planets(), status))
        .points(points)
        .badge_id(badge_id)
        .id(id);

    if let Some(tag) = tag {
        builder = builder.tag(tag);
    }

    builder.build()
}
