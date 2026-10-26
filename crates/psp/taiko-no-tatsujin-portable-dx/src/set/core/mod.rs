use crate::{
    set::core::{in_game_achievement::in_game_achievement_with, songs::clear_song_with_result},
    types::{
        mode::omikoshi::OmikoshiMode, profile::Profile, result::Result, song::ALL_STANDARD_SONGS,
    },
    util::{TOTAL_NUM_DOJO, TOTAL_NUM_OMIKOSHI_CLOTHES, TOTAL_NUM_OMIKOSHI_HEADGEAR},
};
use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::ResolvedChain},
};

use crate::{
    set::core::{
        in_game_achievement::in_game_achievement, mail::recieved_mail, omikoshi::beat_boss,
    },
    types::{
        achievement::InGameAchievement, difficulty::Difficulty, mail::Mail,
        mode::omikoshi::BattleOpponent,
    },
};

pub mod general;
pub mod in_game_achievement;
pub mod leaderboards;
pub mod mail;
pub mod omikoshi;
pub mod songs;

#[rustfmt::skip]
pub fn generate_set() -> Vec<Achievement> {
    let mut core = vec![
        // Omikoshi
        beat_boss(BattleOpponent::GutsCannon),
        beat_boss(BattleOpponent::DokonGang),
        beat_boss(BattleOpponent::DokonPlusDokonGang),
        beat_boss(BattleOpponent::GutsJet),
        beat_boss(BattleOpponent::DevilDonChan),
        beat_boss(BattleOpponent::GutsEater),
        beat_boss(BattleOpponent::ArmageDon),
        unlock_boss_rush_mode("Clear \"Cycle of Rebirth\" and unlock the Boss Rush mode"),

        // Omikoshi clothes and headgear.
        obtain_omikoshi_headgear("Find all headgear in the Omikoshi story mode"),
        obtain_omikoshi_clothes("Find all clothes in the Omikoshi story mode"),

        // In-game achievements
        in_game_achievement("Clear a song in Freeplay mode", InGameAchievement::FirstClear),
        in_game_achievement("Earn 15 silver crowns on Kantan difficulty", InGameAchievement::Earn15SilverCrownsOnKantan),
        in_game_achievement("Earn 30 silver crowns on Kantan difficulty", InGameAchievement::Earn30SilverCrownsOnKantan),
        in_game_achievement("Earn 70 silver crowns on Kantan difficulty", InGameAchievement::Earn70SilverCrownsOnKantan),
        in_game_achievement("Earn 15 silver crowns on Futsuu difficulty", InGameAchievement::Earn15SilverCrownsOnFutsuu),
        in_game_achievement("Earn 30 silver crowns on Futsuu difficulty", InGameAchievement::Earn30SilverCrownsOnFutsuu),
        in_game_achievement("Earn 70 silver crowns on Futsuu difficulty", InGameAchievement::Earn70SilverCrownsOnFutsuu),
        in_game_achievement("Earn 15 silver crowns on Muzukashii difficulty", InGameAchievement::Earn15SilverCrownsOnMuzukashii),
        in_game_achievement("Earn 30 silver crowns on Muzukashii difficulty", InGameAchievement::Earn30SilverCrownsOnMuzukashii),
        in_game_achievement("Earn 70 silver crowns on Muzukashii difficulty", InGameAchievement::Earn70SilverCrownsOnMuzukashii),
        in_game_achievement("Earn 15 silver crowns on Oni difficulty", InGameAchievement::Earn15SilverCrownsOnOni),
        in_game_achievement("Earn 30 silver crowns on Oni difficulty", InGameAchievement::Earn30SilverCrownsOnOni),
        in_game_achievement("Earn 70 silver crowns on Oni difficulty", InGameAchievement::Earn70SilverCrownsOnOni),
        accumulate_hits("Accumulate 10,000 hits on the Taiko Counter", InGameAchievement::Accumulate10000HitsOnTheTaikoCounter, 10_000),
        accumulate_hits("Accumulate 20,000 hits on the Taiko Counter", InGameAchievement::Accumulate20000HitsOnTheTaikoCounter, 20_000),
        accumulate_hits("Accumulate 30,000 hits on the Taiko Counter", InGameAchievement::Accumulate30000HitsOnTheTaikoCounter, 30_000),
        accumulate_hits("Accumulate 40,000 hits on the Taiko Counter", InGameAchievement::Accumulate40000HitsOnTheTaikoCounter, 40_000),
        accumulate_hits("Accumulate 50,000 hits on the Taiko Counter", InGameAchievement::Accumulate50000HitsOnTheTaikoCounter, 50_000),
        accumulate_hits("Accumulate 60,000 hits on the Taiko Counter", InGameAchievement::Accumulate60000HitsOnTheTaikoCounter, 60_000),
        accumulate_hits("Accumulate 70,000 hits on the Taiko Counter", InGameAchievement::Accumulate70000HitsOnTheTaikoCounter, 70_000),
        accumulate_hits("Accumulate 76,500 hits on the Taiko Counter", InGameAchievement::Accumulate76500HitsOnTheTaikoCounter, 76_500),
        accumulate_hits("Accumulate 80,000 hits on the Taiko Counter", InGameAchievement::Accumulate80000HitsOnTheTaikoCounter, 80_000),
        accumulate_hits("Accumulate 90,000 hits on the Taiko Counter", InGameAchievement::Accumulate90000HitsOnTheTaikoCounter, 90_000),
        accumulate_hits("Accumulate 99,999 hits on the Taiko Counter", InGameAchievement::Accumulate99999HitsOnTheTaikoCounter, 99_999),
        in_game_achievement("Earn 10 golden crowns", InGameAchievement::Earn10GoldenCrowns),
        in_game_achievement("Earn 25 golden crowns", InGameAchievement::Earn25GoldenCrowns),
        in_game_achievement("Earn 50 golden crowns", InGameAchievement::Earn50GoldenCrowns),
        in_game_achievement("Earn 75 golden crowns", InGameAchievement::Earn75GoldenCrowns),
        in_game_achievement("Earn 100 golden crowns", InGameAchievement::Earn100GoldenCrowns),
        in_game_achievement("Clear a song with all great hits and no misses in Freeplay mode", InGameAchievement::ClearASongWithAllGreatHitsAndNoMisses),
        in_game_achievement("Clear a song with all good hits and no misses in Freeplay mode", InGameAchievement::ClearASongWithAllGoodHitsAndNoMisses),
        in_game_achievement("Reach a combo of 100 hits or more", InGameAchievement::ReachAComboOf100HitsOrMore),
        in_game_achievement("Reach a combo of 200 hits or more", InGameAchievement::ReachAComboOf200HitsOrMore),
        in_game_achievement("Reach a combo of 400 hits or more", InGameAchievement::ReachAComboOf400HitsOrMore),
        in_game_achievement("Reach a combo of 600 hits or more", InGameAchievement::ReachAComboOf600HitsOrMore),
        in_game_achievement("Get more than 100 total hits in a song", InGameAchievement::GetMoreThan100TotalHitsInASong),
        in_game_achievement("Score 500,000 points or more in a song", InGameAchievement::Score500000PointsOrMoreInASong),
        in_game_achievement("Clear 5 dojo training stages", InGameAchievement::Clear5DojoTrainingStages),
        in_game_achievement("Clear 10 dojo training stages", InGameAchievement::Clear10DojoTrainingStages),
        in_game_achievement("Clear 20 dojo training stages", InGameAchievement::Clear20DojoTrainingStages),
        in_game_achievement("Clear 30 dojo training stages", InGameAchievement::Clear30DojoTrainingStages),
        in_game_achievement("Clear 40 dojo training stages", InGameAchievement::Clear40DojoTrainingStages),
        in_game_achievement("Clear 50 dojo training stages", InGameAchievement::Clear50DojoTrainingStages),
        control_dojos("Control 10 dojo in Omikoshi mode", InGameAchievement::Control10DojosInOmikoshiMode, 10),
        control_dojos("Control 20 dojo in Omikoshi mode", InGameAchievement::Control20DojosInOmikoshiMode, 20),
        control_dojos("Control 30 dojo in Omikoshi mode", InGameAchievement::Control30DojosInOmikoshiMode, 30),
        control_dojos("Control 40 dojo in Omikoshi mode", InGameAchievement::Control40DojosInOmikoshiMode, 40),
        control_dojos("Control all dojo in Omikoshi mode", InGameAchievement::ControlAllDojosInOmikoshiMode, TOTAL_NUM_DOJO),
        in_game_achievement("Beat Omikoshi mode", InGameAchievement::BeatOmikoshiMode),
        in_game_achievement("Beat 2 opponents in a row in Boss Rush mode", InGameAchievement::Beat2OpponentsInARowInBossRushMode),
        in_game_achievement("Beat 5 opponents in a row in Boss Rush mode", InGameAchievement::Beat5OpponentsInARowInBossRushMode),
        in_game_achievement("Clear the Boss Rush mode", InGameAchievement::ClearTheBossRushMode),
        in_game_achievement("Gather 100 or more Drum Disciples in Omikoshi mode", InGameAchievement::Gather100OrMoreDrumDisciplesInOmikoshiMode),
        in_game_achievement("Gather 200 or more Drum Disciples in Omikoshi mode", InGameAchievement::Gather200OrMoreDrumDisciplesInOmikoshiMode),
        in_game_achievement("Gather 300 or more Drum Disciples in Omikoshi mode", InGameAchievement::Gather300OrMoreDrumDisciplesInOmikoshiMode),
        in_game_achievement("Beat the opponent by more than 9 meters in a dojo battle", InGameAchievement::BeatTheOpponentByMoreThan9MetersInADojoBattle),
        in_game_achievement("Fail 10 times in Freeplay mode", InGameAchievement::Fail10TimesInFreeplay),
        in_game_achievement("Earn a Gold Crown on \"The world is all one!!\" on Kantan difficulty", InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan),
        in_game_achievement("Earn a Gold Crown on \"The world is all one!!\" on Futsuu difficulty", InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu),
        in_game_achievement("Earn a Gold Crown on \"Monster Hunter Medley\" on Kantan difficulty", InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnKantan),
        in_game_achievement("Earn a Gold Crown on \"Monster Hunter Medley\" on Futsuu difficulty", InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu),
        in_game_achievement("Earn a Gold Crown on \"Hatsune Miku no Gekishou\" on Kantan difficulty", InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan),
        in_game_achievement("Earn a Gold Crown on \"Hatsune Miku no Gekishou\" on Futsuu difficulty", InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu),
        in_game_achievement("Earn a Gold Crown on \"MachineGun Kiss\" on Kantan difficulty", InGameAchievement::EarnGoldenCrownOnMachineGunKissOnKantan),
        in_game_achievement("Earn a Gold Crown on \"MachineGun Kiss\" on Futsuu difficulty", InGameAchievement::EarnGoldenCrownOnMachineGunKissOnFutsuu),
        in_game_achievement("Reach a combo of 500 or more in Medley mode", InGameAchievement::ReachAComboOf500OrMoreInMedleyMode),
        in_game_achievement("Reach a combo of 1000 or more in Medley mode", InGameAchievement::ReachAComboOf1000OrMoreInMedleyMode),
        in_game_achievement("Reach a combo of 1500 or more in Medley mode", InGameAchievement::ReachAComboOf1500OrMoreInMedleyMode),
        in_game_achievement("Reach a combo of 2000 or more in Medley mode", InGameAchievement::ReachAComboOf2000OrMoreInMedleyMode),
        in_game_achievement("Drumroll a total of 500 times or more in Medley mode", InGameAchievement::DrumrollATotalOf500TimesOrMoreInMedleyMode),
        in_game_achievement("Score 3,000,000 points or more in Medley mode", InGameAchievement::Score3000000PointsOrMoreInMedleyMode),

        // Mail.
        recieved_mail("Achieve the rank of Apprentice", Mail::ApprenticePromotion),
        recieved_mail("Achieve the rank of Novice", Mail::NovicePromotion),
        recieved_mail("Achieve the rank of Disciple", Mail::DisciplePromotion),
        recieved_mail("Achieve the rank of Half a Man", Mail::HalfAManPromotion),
        recieved_mail("Achieve the rank of Qualified", Mail::QualifiedPromotion),
        recieved_mail("Achieve the rank of Capable", Mail::CapablePromotion),
        recieved_mail("Achieve the rank of Go-getter", Mail::GoGetterPromotion),
        recieved_mail("Achieve the rank of Professional", Mail::ProfessionalPromotion),
        recieved_mail("Achieve the rank of Expert", Mail::ExpertPromotion),
        recieved_mail("Achieve the rank of Iron Man", Mail::IronManPromotion),
        recieved_mail("Achieve the rank of Master", Mail::MasterPromotion),
        recieved_mail("Achieve the rank of Grand Master", Mail::GrandMasterPromotion),
    ];

    let all_songs_fc = ALL_STANDARD_SONGS.iter().map(|song|
        clear_song_with_result("TBD - Song FC", song, Result::GoldenCrown, Difficulty::Muzukashii)
    );
    core.extend(all_songs_fc);
    core
}

pub fn obtain_omikoshi_headgear(description: &'static str) -> Achievement {
    let delta_num_omikoshi_headgear: ResolvedChain = Profile::num_omikoshi_headgear()
        .eq(TOTAL_NUM_OMIKOSHI_HEADGEAR - 1)
        .resolve()
        .iter()
        .map(|r| delta!(r))
        .collect();

    Achievement::builder("TBD - Obtain Omikoshi headgear")
        .description(description)
        .core(chain!(
            delta_num_omikoshi_headgear,
            measured!(Profile::num_omikoshi_headgear().eq(TOTAL_NUM_OMIKOSHI_HEADGEAR)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}

pub fn obtain_omikoshi_clothes(description: &'static str) -> Achievement {
    let delta_num_omikoshi_clothes: ResolvedChain = Profile::num_omikoshi_clothes()
        .eq(TOTAL_NUM_OMIKOSHI_CLOTHES - 1)
        .resolve()
        .iter()
        .map(|r| delta!(r))
        .collect();

    Achievement::builder("TBD - Obtain Omikoshi clothes")
        .description(description)
        .core(chain!(
            delta_num_omikoshi_clothes,
            measured!(Profile::num_omikoshi_clothes().eq(TOTAL_NUM_OMIKOSHI_CLOTHES)),
            measured_if!(Profile::is_loaded())
        ))
        .build()
}

pub fn control_dojos(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    dojos: u32,
) -> Achievement {
    in_game_achievement_with(
        description,
        in_game_achievement,
        chain!(
            measured!(OmikoshiMode::number_of_controlled_prefectures().ge(dojos)),
            measured_if!(Profile::is_loaded())
        )
        .into(),
    )
}

pub fn accumulate_hits(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    hits: u32,
) -> Achievement {
    in_game_achievement_with(
        description,
        in_game_achievement,
        chain!(
            measured!(Profile::total_drum_hits().ge(hits)),
            measured_if!(Profile::is_loaded())
        )
        .into(),
    )
}

pub fn unlock_boss_rush_mode(description: &'static str) -> Achievement {
    Achievement::builder("TBD - Boss Rush mode unlocked")
        .description(description)
        .core(chain!(
            delta!(Profile::boss_rush_mode_unlocked().eq(0)),
            Profile::boss_rush_mode_unlocked().eq(1),
            Profile::is_loaded(),
        ))
        .build()
}
