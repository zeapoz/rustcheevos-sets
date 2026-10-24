use crate::{
    set::core::songs::clear_song_with_result,
    types::{result::Result, song::ALL_STANDARD_SONGS},
};
use rustcheevos::types::achievement::Achievement;

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
        in_game_achievement("Accumulate 10,000 hits on the Taiko Counter", InGameAchievement::Accumulate10000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 20,000 hits on the Taiko Counter", InGameAchievement::Accumulate20000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 30,000 hits on the Taiko Counter", InGameAchievement::Accumulate30000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 40,000 hits on the Taiko Counter", InGameAchievement::Accumulate40000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 50,000 hits on the Taiko Counter", InGameAchievement::Accumulate50000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 60,000 hits on the Taiko Counter", InGameAchievement::Accumulate60000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 70,000 hits on the Taiko Counter", InGameAchievement::Accumulate70000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 76,500 hits on the Taiko Counter", InGameAchievement::Accumulate76500HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 80,000 hits on the Taiko Counter", InGameAchievement::Accumulate80000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 90,000 hits on the Taiko Counter", InGameAchievement::Accumulate90000HitsOnTheTaikoCounter),
        in_game_achievement("Accumulate 99,999 hits on the Taiko Counter", InGameAchievement::Accumulate99999HitsOnTheTaikoCounter),
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
        in_game_achievement("Control 10 dojos in Omikoshi mode", InGameAchievement::Control10DojosInOmikoshiMode),
        in_game_achievement("Control 20 dojos in Omikoshi mode", InGameAchievement::Control20DojosInOmikoshiMode),
        in_game_achievement("Control 30 dojos in Omikoshi mode", InGameAchievement::Control30DojosInOmikoshiMode),
        in_game_achievement("Control 40 dojos in Omikoshi mode", InGameAchievement::Control40DojosInOmikoshiMode),
        in_game_achievement("Control all dojos in Omikoshi mode", InGameAchievement::ControlAllDojosInOmikoshiMode),
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

// BACKUP
// // clear_song_with_result("Test Achievement", Song::OneDay, Result::SilverCrown, Difficulty::Muzukashii),
// // clear_song_with_result("Test Achievement 2", Song::OneDay, Result::GoldenCrown, Difficulty::Muzukashii),
//
// // Omikoshi
// control_dojo(10),
// control_dojo(20),
// control_dojo(30),
// control_dojo(40),
// control_dojo(TOTAL_NUM_DOJO),
// gather_drum_disciples(100, Sound::ChibiDon),
// gather_drum_disciples(200, Sound::GangsterDon),
// gather_drum_disciples(300, Sound::YokozunaDon),
// unlock_all_headgear(),
// unlock_all_clothes(),
//
// // General
// total_drum_hits(10_000, Song::EtudeOp104Ura),
// total_drum_hits(20_000, Song::BlackRoseApostleUra),
// total_drum_hits(30_000, Song::HatsuneMikuNoGekishouUra),
// total_drum_hits(40_000, Song::RotterTarminationUra),
// total_drum_hits(50_000, Song::WhiteRoseInsanityUra),
// total_drum_hits(60_000, Song::TaiyouMoYappappaUra),
// total_drum_hits(70_000, Song::SportsDigestionUra),
// total_drum_hits(76_500, Song::RindaWaKyoumoZekkouchouUra),
// total_drum_hits(80_000, Song::DontCutUra),
// total_drum_hits(90_000, Song::RumbleRanbuUra),
// total_drum_hits(99_999, Song::MataSaitama2000),
// achieve_rank(Rank::Apprentice),
// achieve_rank(Rank::Novice),
// achieve_rank(Rank::HalfAMan),
// achieve_rank(Rank::Qualified),
// achieve_rank(Rank::Capable),
// achieve_rank(Rank::GoGetter),
// achieve_rank(Rank::Professional),
// achieve_rank(Rank::Expert),
// achieve_rank(Rank::IronMan),
// achieve_rank(Rank::Master),
// achieve_rank(Rank::GrandMaster),
