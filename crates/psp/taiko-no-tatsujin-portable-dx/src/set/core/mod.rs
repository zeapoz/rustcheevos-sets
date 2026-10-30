use crate::{
    set::{
        common::{leaderboard::generate_leaderboards, song::clear_song_with_result},
        core::{
            general::{accumulate_hits, earn_golden_crowns, earn_silver_crowns},
            omikoshi::{
                control_dojo, gather_drum_disciples, obtain_omikoshi_clothes,
                obtain_omikoshi_headgear,
            },
            song::{clear_song_with_all_good, clear_song_with_all_great},
        },
    },
    types::song::{ALL_STANDARD_SONGS, result::SongResult},
    util::TOTAL_NUM_DOJO,
};
use rustcheevos::types::{
    achievement::Achievement,
    set::{Base, Set},
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

mod general;
mod in_game_achievement;
mod mail;
mod omikoshi;
mod song;

pub struct CoreSet;

impl CoreSet {
    pub fn generate() -> Set<Base> {
        let mut set = Set::new_base();
        set.add_achievements(Self::generate_set())
            .add_leaderboards(generate_leaderboards(&ALL_STANDARD_SONGS));
        set
    }

    #[rustfmt::skip]
    fn generate_set() -> Vec<Achievement> {
        let mut core = vec![
            // Omikoshi
            beat_boss(BattleOpponent::GutsCannon),
            beat_boss(BattleOpponent::DokonGang),
            beat_boss(BattleOpponent::DokonPlusDokonGang),
            beat_boss(BattleOpponent::GutsJet),
            beat_boss(BattleOpponent::DevilDonChan),
            beat_boss(BattleOpponent::GutsEater),
            beat_boss(BattleOpponent::ArmageDon),

            // Omikoshi clothes and headgear.
            obtain_omikoshi_headgear("Find all headgear in the Omikoshi story mode"),
            obtain_omikoshi_clothes("Find all clothes in the Omikoshi story mode"),

            // In-game achievements
            in_game_achievement("Clear a song in Freeplay mode", InGameAchievement::FirstClear),
            earn_silver_crowns("Earn 15 silver crowns on Kantan difficulty", InGameAchievement::Earn15SilverCrownsOnKantan, 15, Difficulty::Kantan),
            earn_silver_crowns("Earn 30 silver crowns on Kantan difficulty", InGameAchievement::Earn30SilverCrownsOnKantan, 30, Difficulty::Kantan),
            earn_silver_crowns("Earn 70 silver crowns on Kantan difficulty", InGameAchievement::Earn70SilverCrownsOnKantan, 70, Difficulty::Kantan),
            earn_silver_crowns("Earn 15 silver crowns on Futsuu difficulty", InGameAchievement::Earn15SilverCrownsOnFutsuu, 15, Difficulty::Futsuu),
            earn_silver_crowns("Earn 30 silver crowns on Futsuu difficulty", InGameAchievement::Earn30SilverCrownsOnFutsuu, 30, Difficulty::Futsuu),
            earn_silver_crowns("Earn 70 silver crowns on Futsuu difficulty", InGameAchievement::Earn70SilverCrownsOnFutsuu, 30, Difficulty::Futsuu),
            earn_silver_crowns("Earn 15 silver crowns on Muzukashii difficulty", InGameAchievement::Earn15SilverCrownsOnMuzukashii, 15, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 30 silver crowns on Muzukashii difficulty", InGameAchievement::Earn30SilverCrownsOnMuzukashii, 30, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 70 silver crowns on Muzukashii difficulty", InGameAchievement::Earn70SilverCrownsOnMuzukashii, 70, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 15 silver crowns on Oni difficulty", InGameAchievement::Earn15SilverCrownsOnOni, 15, Difficulty::Oni),
            earn_silver_crowns("Earn 30 silver crowns on Oni difficulty", InGameAchievement::Earn30SilverCrownsOnOni, 30, Difficulty::Oni),
            earn_silver_crowns("Earn 70 silver crowns on Oni difficulty", InGameAchievement::Earn70SilverCrownsOnOni, 70, Difficulty::Oni),
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
            earn_golden_crowns("Earn 10 golden crowns", InGameAchievement::Earn10GoldenCrowns, 10),
            earn_golden_crowns("Earn 25 golden crowns", InGameAchievement::Earn25GoldenCrowns, 25),
            earn_golden_crowns("Earn 50 golden crowns", InGameAchievement::Earn50GoldenCrowns, 50),
            earn_golden_crowns("Earn 75 golden crowns", InGameAchievement::Earn75GoldenCrowns, 75),
            earn_golden_crowns("Earn 100 golden crowns", InGameAchievement::Earn100GoldenCrowns, 100),
            clear_song_with_all_great("Clear a song with all great hits and no misses in Freeplay mode"),
            clear_song_with_all_good("Clear a song with all good hits and no misses in Freeplay mode"),
            in_game_achievement("Reach a combo of 100 hits or more", InGameAchievement::ReachAComboOf100HitsOrMore),
            in_game_achievement("Reach a combo of 200 hits or more", InGameAchievement::ReachAComboOf200HitsOrMore),
            in_game_achievement("Reach a combo of 400 hits or more", InGameAchievement::ReachAComboOf400HitsOrMore),
            in_game_achievement("Reach a combo of 600 hits or more", InGameAchievement::ReachAComboOf600HitsOrMore),
            in_game_achievement("Get more than 100 total hits in a song in Freeplay mode", InGameAchievement::GetMoreThan100TotalHitsInASong),
            in_game_achievement("Score 500,000 points or more in a song", InGameAchievement::Score500000PointsOrMoreInASong),
            in_game_achievement("Clear 5 dojo training stages", InGameAchievement::Clear5DojoTrainingStages),
            in_game_achievement("Clear 10 dojo training stages", InGameAchievement::Clear10DojoTrainingStages),
            in_game_achievement("Clear 20 dojo training stages", InGameAchievement::Clear20DojoTrainingStages),
            in_game_achievement("Clear 30 dojo training stages", InGameAchievement::Clear30DojoTrainingStages),
            in_game_achievement("Clear 40 dojo training stages", InGameAchievement::Clear40DojoTrainingStages),
            in_game_achievement("Clear 50 dojo training stages", InGameAchievement::Clear50DojoTrainingStages),
            control_dojo("Control 10 dojo in Omikoshi mode", 10),
            control_dojo("Control 20 dojo in Omikoshi mode", 20),
            control_dojo("Control 30 dojo in Omikoshi mode", 30),
            control_dojo("Control 40 dojo in Omikoshi mode", 40),
            control_dojo("Control all 47 dojo in Omikoshi mode", TOTAL_NUM_DOJO),
            in_game_achievement("Beat the Omikoshi mode", InGameAchievement::BeatOmikoshiMode),
            in_game_achievement("Beat 2 opponents in a row in Boss Rush mode", InGameAchievement::Beat2OpponentsInARowInBossRushMode),
            in_game_achievement("Beat 5 opponents in a row in Boss Rush mode", InGameAchievement::Beat5OpponentsInARowInBossRushMode),
            in_game_achievement("Clear the Boss Rush mode", InGameAchievement::ClearTheBossRushMode),
            gather_drum_disciples("Gather 100 or more Drum Disciples in Omikoshi mode", 100),
            gather_drum_disciples("Gather 200 or more Drum Disciples in Omikoshi mode", 200),
            gather_drum_disciples("Gather 300 or more Drum Disciples in Omikoshi mode", 300),
            in_game_achievement("Beat the opponent by more than 9 meters in an Omikoshi battle", InGameAchievement::BeatTheOpponentByMoreThan9MetersInADojoBattle),
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
            clear_song_with_result("TBD - Song FC", song, SongResult::GoldenCrown, Difficulty::Muzukashii)
        );
        core.extend(all_songs_fc);
        core
    }
}
