use crate::{
    set::{
        common::{leaderboard::generate_leaderboards, song::clear_song_with_result},
        core::{
            general::{accumulate_hits, earn_golden_crowns, earn_silver_crowns},
            omikoshi::{control_dojo, obtain_omikoshi_clothes, obtain_omikoshi_headgear},
            song::{
                clear_song_with_modifier_active, clear_song_with_special_modifier, donderful_combo,
                earn_gold_on, false_donderful_combo,
            },
        },
    },
    types::{
        modifier::{NoteModifier, SpecialModifier, SpeedModifier},
        song::{ALL_STANDARD_SONGS, Song},
    },
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
    types::song::result::SongResult,
    types::{
        achievement::InGameAchievement, difficulty::Difficulty, mail::PromotionMail,
        mode::omikoshi::BattleOpponent,
    },
};

mod general;
pub(super) mod in_game_achievement;
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
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::GutsCannon),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::DokonGang),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::DokonPlusDokonGang),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::GutsJet),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::DevilDonChan),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::GutsEater),
            beat_boss("TBD - Beat Boss", 0, BattleOpponent::ArmageDon),

            // Omikoshi clothes and headgear.
            obtain_omikoshi_headgear("Obtain all Omikoshi headgear", "Find all headgear in the Omikoshi story mode", 0),
            obtain_omikoshi_clothes("Obtain all Omikoshi clothes", "Find all clothes in the Omikoshi story mode", 0),

            // In-game achievements
            in_game_achievement("TBD", "Clear a song in Freeplay mode", 0, InGameAchievement::FirstClear),
            earn_silver_crowns("Earn 15 silver crowns on Kantan difficulty", 0, InGameAchievement::Earn15SilverCrownsOnKantan, 15, Difficulty::Kantan),
            earn_silver_crowns("Earn 30 silver crowns on Kantan difficulty", 0, InGameAchievement::Earn30SilverCrownsOnKantan, 30, Difficulty::Kantan),
            earn_silver_crowns("Earn 70 silver crowns on Kantan difficulty", 0, InGameAchievement::Earn70SilverCrownsOnKantan, 70, Difficulty::Kantan),
            earn_silver_crowns("Earn 15 silver crowns on Futsuu difficulty", 0, InGameAchievement::Earn15SilverCrownsOnFutsuu, 15, Difficulty::Futsuu),
            earn_silver_crowns("Earn 30 silver crowns on Futsuu difficulty", 0, InGameAchievement::Earn30SilverCrownsOnFutsuu, 30, Difficulty::Futsuu),
            earn_silver_crowns("Earn 70 silver crowns on Futsuu difficulty", 0, InGameAchievement::Earn70SilverCrownsOnFutsuu, 30, Difficulty::Futsuu),
            earn_silver_crowns("Earn 15 silver crowns on Muzukashii difficulty", 0, InGameAchievement::Earn15SilverCrownsOnMuzukashii, 15, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 30 silver crowns on Muzukashii difficulty", 0, InGameAchievement::Earn30SilverCrownsOnMuzukashii, 30, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 70 silver crowns on Muzukashii difficulty", 0, InGameAchievement::Earn70SilverCrownsOnMuzukashii, 70, Difficulty::Muzukashii),
            earn_silver_crowns("Earn 15 silver crowns on Oni difficulty", 0, InGameAchievement::Earn15SilverCrownsOnOni, 15, Difficulty::Oni),
            earn_silver_crowns("Earn 30 silver crowns on Oni difficulty", 0, InGameAchievement::Earn30SilverCrownsOnOni, 30, Difficulty::Oni),
            earn_silver_crowns("Earn 70 silver crowns on Oni difficulty", 0, InGameAchievement::Earn70SilverCrownsOnOni, 70, Difficulty::Oni),
            accumulate_hits("Accumulate 10,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate10000HitsOnTheTaikoCounter, 10_000, "10,000"),
            accumulate_hits("Accumulate 20,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate20000HitsOnTheTaikoCounter, 20_000, "20,000"),
            accumulate_hits("Accumulate 30,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate30000HitsOnTheTaikoCounter, 30_000, "30,000"),
            accumulate_hits("Accumulate 40,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate40000HitsOnTheTaikoCounter, 40_000, "40,000"),
            accumulate_hits("Accumulate 50,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate50000HitsOnTheTaikoCounter, 50_000, "50,000"),
            accumulate_hits("Accumulate 60,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate60000HitsOnTheTaikoCounter, 60_000, "60,000"),
            accumulate_hits("Accumulate 70,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate70000HitsOnTheTaikoCounter, 70_000, "70,000"),
            accumulate_hits("Accumulate 76,500 hits on the Taiko Counter", 0, InGameAchievement::Accumulate76500HitsOnTheTaikoCounter, 76_500, "76,500"),
            accumulate_hits("Accumulate 80,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate80000HitsOnTheTaikoCounter, 80_000, "80,000"),
            accumulate_hits("Accumulate 90,000 hits on the Taiko Counter", 0, InGameAchievement::Accumulate90000HitsOnTheTaikoCounter, 90_000, "90,000"),
            accumulate_hits("Accumulate 99,999 hits on the Taiko Counter", 0, InGameAchievement::Accumulate99999HitsOnTheTaikoCounter, 99_999, "99,999"),
            earn_golden_crowns("Earn 10 golden crowns", 0, InGameAchievement::Earn10GoldenCrowns, 10),
            earn_golden_crowns("Earn 25 golden crowns", 0, InGameAchievement::Earn25GoldenCrowns, 25),
            earn_golden_crowns("Earn 50 golden crowns", 0, InGameAchievement::Earn50GoldenCrowns, 50),
            earn_golden_crowns("Earn 75 golden crowns", 0, InGameAchievement::Earn75GoldenCrowns, 75),
            earn_golden_crowns("Earn 100 golden crowns", 0, InGameAchievement::Earn100GoldenCrowns, 100),
            donderful_combo("Donderful Combo!", "Clear a song with all great hits and no misses in Freeplay mode", 0, InGameAchievement::ClearASongWithAllGreatHitsAndNoMisses),
            false_donderful_combo("False Donderful Combo", "Clear a song with all good hits and no misses in Freeplay mode", 0, InGameAchievement::ClearASongWithAllGoodHitsAndNoMisses),
            in_game_achievement("TBD - In game", "Reach a combo of 100 hits or more", 0, InGameAchievement::ReachAComboOf100HitsOrMore),
            in_game_achievement("TBD - In game", "Reach a combo of 200 hits or more", 0, InGameAchievement::ReachAComboOf200HitsOrMore),
            in_game_achievement("TBD - In game", "Reach a combo of 400 hits or more", 0, InGameAchievement::ReachAComboOf400HitsOrMore),
            in_game_achievement("TBD - In game", "Reach a combo of 600 hits or more", 0, InGameAchievement::ReachAComboOf600HitsOrMore),
            in_game_achievement("TBD - In game", "Get more than 100 total hits in a song in Freeplay mode", 0, InGameAchievement::GetMoreThan100TotalHitsInASong),
            in_game_achievement("TBD - In game", "Score 500,000 points or more in a song", 0, InGameAchievement::Score500000PointsOrMoreInASong),
            in_game_achievement("TBD - In game", "Clear 5 dojo training stages", 0, InGameAchievement::Clear5DojoTrainingStages),
            in_game_achievement("TBD - In game", "Clear 10 dojo training stages", 0, InGameAchievement::Clear10DojoTrainingStages),
            in_game_achievement("TBD - In game", "Clear 20 dojo training stages", 0, InGameAchievement::Clear20DojoTrainingStages),
            in_game_achievement("TBD - In game", "Clear 30 dojo training stages", 0, InGameAchievement::Clear30DojoTrainingStages),
            in_game_achievement("TBD - In game", "Clear 40 dojo training stages", 0, InGameAchievement::Clear40DojoTrainingStages),
            in_game_achievement("TBD - In game", "Clear 50 dojo training stages", 0, InGameAchievement::Clear50DojoTrainingStages),
            control_dojo("Control 10 dojo in Omikoshi mode", 0, 10),
            control_dojo("Control 20 dojo in Omikoshi mode", 0, 20),
            control_dojo("Control 30 dojo in Omikoshi mode", 0, 30),
            control_dojo("Control 40 dojo in Omikoshi mode", 0, 40),
            control_dojo("Control all 47 dojo in Omikoshi mode", 0, TOTAL_NUM_DOJO),
            in_game_achievement("TBD - In game", "Beat the Omikoshi mode", 0, InGameAchievement::BeatOmikoshiMode),
            in_game_achievement("TBD - In game", "Beat 2 opponents in a row in Boss Rush mode", 0, InGameAchievement::Beat2OpponentsInARowInBossRushMode),
            in_game_achievement("TBD - In game", "Beat 5 opponents in a row in Boss Rush mode", 0, InGameAchievement::Beat5OpponentsInARowInBossRushMode),
            in_game_achievement("TBD - In game", "Clear the Boss Rush mode", 0, InGameAchievement::ClearTheBossRushMode),
            in_game_achievement("TBD - In game", "Gather 100 or more Drum Disciples in Omikoshi mode", 0, InGameAchievement::Gather100OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("TBD - In game", "Gather 200 or more Drum Disciples in Omikoshi mode", 0, InGameAchievement::Gather200OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("TBD - In game", "Gather 300 or more Drum Disciples in Omikoshi mode", 0, InGameAchievement::Gather300OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("TBD - In game", "Beat the opponent by more than 9 meters in an Omikoshi battle", 0, InGameAchievement::BeatTheOpponentByMoreThan9MetersInADojoBattle),
            in_game_achievement("TBD - In game", "Fail 10 times in Freeplay mode", 0, InGameAchievement::Fail10TimesInFreeplay),
            earn_gold_on("Earn a Gold Crown on \"The world is all one!!\" on Kantan difficulty", 0, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan, Song::TheWorldIsAllOne, Difficulty::Kantan),
            earn_gold_on("Earn a Gold Crown on \"The world is all one!!\" on Futsuu difficulty", 0, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu, Song::TheWorldIsAllOne, Difficulty::Futsuu),
            earn_gold_on("Earn a Gold Crown on \"Monster Hunter Medley\" on Kantan difficulty", 0, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnKantan, Song::MonsterHunterMedley, Difficulty::Kantan),
            earn_gold_on("Earn a Gold Crown on \"Monster Hunter Medley\" on Futsuu difficulty", 0, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu, Song::MonsterHunterMedley, Difficulty::Futsuu),
            earn_gold_on("Earn a Gold Crown on \"Hatsune Miku no Gekishou\" on Kantan difficulty", 0, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan, Song::HatsuneMikuNoGekishou, Difficulty::Kantan),
            earn_gold_on("Earn a Gold Crown on \"Hatsune Miku no Gekishou\" on Futsuu difficulty", 0, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu, Song::HatsuneMikuNoGekishou, Difficulty::Futsuu),
            earn_gold_on("Earn a Gold Crown on \"MachineGun Kiss\" on Kantan difficulty", 0, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnKantan, Song::MachineGunKiss, Difficulty::Kantan),
            earn_gold_on("Earn a Gold Crown on \"MachineGun Kiss\" on Futsuu difficulty", 0, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnFutsuu, Song::MachineGunKiss, Difficulty::Futsuu),
            in_game_achievement("TBD - In game", "Reach a combo of 500 or more in Medley mode", 0, InGameAchievement::ReachAComboOf500OrMoreInMedleyMode),
            in_game_achievement("TBD - In game", "Reach a combo of 1000 or more in Medley mode", 0, InGameAchievement::ReachAComboOf1000OrMoreInMedleyMode),
            in_game_achievement("TBD - In game", "Reach a combo of 1500 or more in Medley mode", 0, InGameAchievement::ReachAComboOf1500OrMoreInMedleyMode),
            in_game_achievement("TBD - In game", "Reach a combo of 2000 or more in Medley mode", 0, InGameAchievement::ReachAComboOf2000OrMoreInMedleyMode),
            in_game_achievement("TBD - In game", "Drumroll a total of 500 times or more in Medley mode", 0, InGameAchievement::DrumrollATotalOf500TimesOrMoreInMedleyMode),
            in_game_achievement("TBD - In game", "Score 3,000,000 points or more in Medley mode", 0, InGameAchievement::Score3000000PointsOrMoreInMedleyMode),

            // Mail.
            recieved_mail("Apprentice", 0, PromotionMail::Apprentice),
            recieved_mail("Novice", 0, PromotionMail::Novice),
            recieved_mail("Half a Man", 0, PromotionMail::HalfAMan),
            recieved_mail("Qualified", 0, PromotionMail::Qualified),
            recieved_mail("Capable", 0, PromotionMail::Capable),
            recieved_mail("Go-getter", 0, PromotionMail::GoGetter),
            recieved_mail("Professional", 0, PromotionMail::Professional),
            recieved_mail("Expert", 0, PromotionMail::Expert),
            recieved_mail("Iron Man", 0, PromotionMail::IronMan),
            recieved_mail("Master", 0, PromotionMail::Master),
            recieved_mail("Grand Master", 0, PromotionMail::GrandMaster),

            // Modifiers
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Double speed modifier or higher active", 0, SpeedModifier::is_or_higher(SpeedModifier::Double)),
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Triple speed modifier or higher active", 0, SpeedModifier::is_or_higher(SpeedModifier::Triple)),
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Quadruple speed modifier active", 0, SpeedModifier::is(SpeedModifier::Triple)),
            clear_song_with_special_modifier("TBD - In game", "Clear any song with the Sudden Death modifier active", 0, SpecialModifier::SuddenDeath),
            clear_song_with_special_modifier("TBD - In game", "Clear any song with the Invisible modifier active", 0, SpecialModifier::Invisible),
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Whimsical notes modifier active", 0, NoteModifier::is(NoteModifier::Whimsical)),
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Random notes modifier active", 0, NoteModifier::is(NoteModifier::Random)),
            clear_song_with_modifier_active("TBD - In game", "Clear any song with the Reversed notes modifier active", 0, NoteModifier::is(NoteModifier::Reveresed)),
        ];
        core.extend(Self::generate_song_full_combos());
        core
    }

    #[rustfmt::skip]
    fn generate_song_full_combos() -> Vec<Achievement> {
        vec![
            // Anime
            clear_song_with_result("Song FC - One day", Song::OneDay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Lovers", Song::Lovers, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Lion", Song::Lion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No buts!", Song::NoButs, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Song FC - Heavy Rotation", Song::HeavyRotation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yoku Asobi Yoku Manabe", Song::YokuAsobiYokuManabe, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - I Wish For You", Song::IWishForYou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Arigatou", Song::Arigatou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Joyful", Song::Joyful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Natsu Matsuri", Song::NatsuMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tentai Kansoku", Song::TentaiKansou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kurenai", Song::Kurenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Punishment", Song::Punishment, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Song FC - MISTER", Song::Mister, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jounetsu Tairiku", Song::JounetsuTairiku, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Classic
            clear_song_with_result("Song FC - Pavane for a Dead Princess ～Kimi no Kodou～", Song::PavaneForADeadPrincess, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ninth Symphony", Song::NinthSymphony, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Excerpt from Light Cavalry Overture", Song::ExcerptFromLightCavalryOverture, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Eine Kleine Nachtmuzik", Song::EineKleineNachtmuzik, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Violin Concerto in E Minor", Song::ViolinConcertoInEMinor, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Radetzky March", Song::RadetzkyMarch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kare Kano Kanon", Song::KareKantonoKanonan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Etude Op.10-4", Song::EtudeOp104, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Etude Op.10-4 (Ura)", Song::EtudeOp104Ura, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Song FC - Monster Hunter Medley", Song::MonsterHunterMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - MachineGun Kiss", Song::MachineGunKiss, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - The world is all one!!", Song::TheWorldIsAllOne, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kirame Kirari", Song::KirameKirari, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Mujihi na Ou", Song::MujihiNaOu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - KARMA", Song::Karma, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Bambini", Song::Bambini, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - PaPaPa Love", Song::PaPaPaLove, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Mappy Medley", Song::MappyMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - MAGICAL SOUND SHOWER", Song::MagicalSoundShower, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hatsune Miku no Gekishou", Song::HatsuneMikuNoGekishou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hatsune Miku no Gekishou (Ura)", Song::HatsuneMikuNoGekishouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Song FC - DokoDON MatsuRhythm", Song::DokoDONMatsuRhythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Canadea", Song::Canadea, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - SORA-V Cosmic Bird", Song::SORAVCosmicBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Wanya World", Song::WanyaWorld, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - All Night de Indenai", Song::AllNightdeIndenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Magical Little Spaceship", Song::MagicalLittleSpaceship, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shiro Neko Caramel Mugen no Wata ame", Song::ShiroNekoCaramelMugennoWataame, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - White Rose Insanity", Song::WhiteRoseInsanity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - White Rose Insanity (Ura)", Song::WhiteRoseInsanityUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ao no Senritsu", Song::AoNoSenritsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Inu Hoeru", Song::InuHoeru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LOVE Ikusa!!", Song::LOVEIkusa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiyou mo Yappappa", Song::TaiyouMoYappappa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiyou mo Yappappa (Ura)", Song::TaiyouMoYappappaUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sports Digestdon", Song::SportsDigestDon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sports Digestdon (Ura)", Song::SportsDigestDonUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Happy & Peace", Song::HappyPeace, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ra Morena Kumonai", Song::RaMorenaKumonai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Desert de Yakiniku", Song::DesertDeYakiniku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rinda wa Kyou mo Zekkouchou", Song::RindaWaKyoumoZekkouchou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rinda wa Kyou mo Zekkouchou (Ura)", Song::RindaWaKyoumoZekkouchouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Densetsu no Matsuri", Song::DensetsunoMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - DON'T CUT", Song::DontCut, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - DON'T CUT (Ura)", Song::DontCutUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Dun Aonghasa no Fuefuki", Song::DunAonghasanoFuefuki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rumble Ranbu", Song::RumbleRanbu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rumble Ranbu (Ura)", Song::RumbleRanbuUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Buru-chan no Oya 2", Song::BuruChanNoOya2, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kissa Rain", Song::KissaRain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Dodododo-Donderful!", Song::DodododoDonderful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Akuukan Yuuei ac12.5", Song::AkuukanYuueiac125, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jigoku no Taiko Jiten", Song::JigokunoTaikoJiten, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - BE THE ACE", Song::BeTheAce, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Mulberry", Song::Mulberry, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sengoku Sangen", Song::SengokuSangen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hyakki Yakou", Song::HyakkiYakou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rotter Tarmination", Song::RotterTarmination, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Rotter Tarmination (Ura)", Song::RotterTarminationUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Black Rose Apostle", Song::BlackRoseApostle, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Black Rose Apostle (Ura)", Song::BlackRoseApostleUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Soroban 2000", Song::Soroban2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - X-DAY 2000", Song::XDay2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Mata Saitama 2000", Song::MataSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Cycle of Rebirth", Song::CycleOfRebirth, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
