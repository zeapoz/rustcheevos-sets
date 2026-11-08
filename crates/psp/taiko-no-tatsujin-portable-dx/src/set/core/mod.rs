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
            beat_boss("Guerrilla Grove", "Defeat Guts Cannon in Omikoshi Story Mode", 10, BattleOpponent::GutsCannon),
            beat_boss("Gang Buster", "Defeat the Dokon Gang in Omikoshi Story Mode", 15, BattleOpponent::DokonGang),
            beat_boss("Topple the Don", "Defeat the Dokon Gang and Dokon in Omikoshi Story Mode", 20, BattleOpponent::DokonPlusDokonGang),
            beat_boss("Crash Landing", "Defeat Guts Jet in Omikoshi Story Mode", 25, BattleOpponent::GutsJet),
            beat_boss("Devilish Defeat", "Defeat Devil Don-chan in Omikoshi Story Mode", 25, BattleOpponent::DevilDonChan),
            beat_boss("Starving the King", "Defeat Guts Eater in Omikoshi Story Mode", 30, BattleOpponent::GutsEater),
            beat_boss("Armageddon Averted", "Defeat Armage-Don in Omikoshi Story Mode", 30, BattleOpponent::ArmageDon),

            // Omikoshi clothes and headgear.
            obtain_omikoshi_headgear("Headgear Hunter", "Collect all headgear available in Omikoshi Story Mode", 15),
            obtain_omikoshi_clothes("Fearless Fashionista", "Collect all clothing available in Omikoshi Story Mode", 15),

            // In-game achievements
            in_game_achievement("First Beat", "Clear your first song", 10, InGameAchievement::FirstClear),
            earn_silver_crowns("Bachi Basics", 15, InGameAchievement::Earn15SilverCrownsOnKantan, 15, Difficulty::Kantan),
            earn_silver_crowns("Beat Baron", 15, InGameAchievement::Earn30SilverCrownsOnKantan, 30, Difficulty::Kantan),
            earn_silver_crowns("Kantan Complete", 25, InGameAchievement::Earn70SilverCrownsOnKantan, 70, Difficulty::Kantan),
            earn_silver_crowns("Rhythm Warm-Up", 15, InGameAchievement::Earn15SilverCrownsOnFutsuu, 15, Difficulty::Futsuu),
            earn_silver_crowns("Beat Brawler", 15, InGameAchievement::Earn30SilverCrownsOnFutsuu, 30, Difficulty::Futsuu),
            earn_silver_crowns("Futsuu Finished", 25, InGameAchievement::Earn70SilverCrownsOnFutsuu, 70, Difficulty::Futsuu),
            earn_silver_crowns("Dealing with the Devil", 15, InGameAchievement::Earn15SilverCrownsOnMuzukashii, 15, Difficulty::Muzukashii),
            earn_silver_crowns("Beat Breaker", 20, InGameAchievement::Earn30SilverCrownsOnMuzukashii, 30, Difficulty::Muzukashii),
            earn_silver_crowns("Muzukashii Master", 25, InGameAchievement::Earn70SilverCrownsOnMuzukashii, 70, Difficulty::Muzukashii),
            earn_silver_crowns("Oni Overlord", 20, InGameAchievement::Earn15SilverCrownsOnOni, 15, Difficulty::Oni),
            earn_silver_crowns("Beat Behemoth", 25, InGameAchievement::Earn30SilverCrownsOnOni, 30, Difficulty::Oni),
            earn_silver_crowns("Oni Obliterator", 30, InGameAchievement::Earn70SilverCrownsOnOni, 70, Difficulty::Oni),
            accumulate_hits("Ichiman Instrumentalist", 20, InGameAchievement::Accumulate10000HitsOnTheTaikoCounter, 10_000, "10,000"),
            accumulate_hits("Niman Notator", 25, InGameAchievement::Accumulate20000HitsOnTheTaikoCounter, 20_000, "20,000"),
            accumulate_hits("Sanman Soloist", 30, InGameAchievement::Accumulate30000HitsOnTheTaikoCounter, 30_000, "30,000"),
            accumulate_hits("Yonman Jammer", 35, InGameAchievement::Accumulate40000HitsOnTheTaikoCounter, 40_000, "40,000"),
            accumulate_hits("Goman Groover", 40, InGameAchievement::Accumulate50000HitsOnTheTaikoCounter, 50_000, "50,000"),
            accumulate_hits("Rokuman Rhapsodist", 45, InGameAchievement::Accumulate60000HitsOnTheTaikoCounter, 60_000, "60,000"),
            accumulate_hits("Nanan Notesmith", 45, InGameAchievement::Accumulate70000HitsOnTheTaikoCounter, 70_000, "70,000"),
            accumulate_hits("Shichiman Syncopator", 50, InGameAchievement::Accumulate76500HitsOnTheTaikoCounter, 76_500, "76,500"),
            accumulate_hits("Hachiman Hitmaker", 50, InGameAchievement::Accumulate80000HitsOnTheTaikoCounter, 80_000, "80,000"),
            accumulate_hits("Kyuuman Kodo", 50, InGameAchievement::Accumulate90000HitsOnTheTaikoCounter, 90_000, "90,000"),
            accumulate_hits("Man Tan", 50, InGameAchievement::Accumulate99999HitsOnTheTaikoCounter, 99_999, "99,999"),
            earn_golden_crowns("Crown Chaser", 25, InGameAchievement::Earn10GoldenCrowns, 10),
            earn_golden_crowns("Full Crescendo", 30, InGameAchievement::Earn25GoldenCrowns, 25),
            earn_golden_crowns("Donder Kong", 35, InGameAchievement::Earn50GoldenCrowns, 50),
            earn_golden_crowns("8-Bit Ace", 40, InGameAchievement::Earn75GoldenCrowns, 75),
            earn_golden_crowns("Deluxe Donder", 50, InGameAchievement::Earn100GoldenCrowns, 100),
            donderful_combo("Donderful Combo!", "Clear a song with only Great hits and no misses and unlock the Gun sound", 15, InGameAchievement::ClearASongWithAllGreatHitsAndNoMisses),
            false_donderful_combo("A Bitter Taste", "Clear a song with only OK hits and no misses and unlock the Shuriken sound", 15, InGameAchievement::ClearASongWithAllGoodHitsAndNoMisses),
            in_game_achievement("Combo Maniac", "Achieve a combo of 100 or more hits in Freeplay Mode", 10, InGameAchievement::ReachAComboOf100HitsOrMore),
            in_game_achievement("Combo Fever", "Achieve a combo of 200 or more hits in Freeplay Mode", 10, InGameAchievement::ReachAComboOf200HitsOrMore),
            in_game_achievement("Combo Craze", "Achieve a combo of 400 or more hits in Freeplay Mode", 15, InGameAchievement::ReachAComboOf400HitsOrMore),
            in_game_achievement("Combo Catcher", "Achieve a combo of 600 or more hits in Freeplay Mode", 15, InGameAchievement::ReachAComboOf600HitsOrMore),
            in_game_achievement("Novice Buzzer", "Get 100 or more drumroll hits in a song and unlock the Quiz sound", 15, InGameAchievement::GetMoreThan100TotalHitsInASong),
            in_game_achievement("A Deadly Beat", "Score 500,000 or more points in a song in Freeplay Mode and unlock the Knife sound", 25, InGameAchievement::Score500000PointsOrMoreInASong),
            in_game_achievement("Novice Buzzer", "Clear 5 Dojo challenges on a single difficulty and unlock the Cameraman sound", 20, InGameAchievement::Clear5DojoTrainingStages),
            in_game_achievement("Apprentice of the Dojo", "Clear 10 Dojo challenges on a single difficulty and unlock the Wooden Fish sound", 25, InGameAchievement::Clear10DojoTrainingStages),
            in_game_achievement("Adept of the Dojo", "Clear 20 Dojo challenges on a single difficulty and unlock the Bell sound", 30, InGameAchievement::Clear20DojoTrainingStages),
            in_game_achievement("Veteran of the Dojo", "Clear 30 Dojo challenges on a single difficulty and unlock the Sneeze sound", 35, InGameAchievement::Clear30DojoTrainingStages),
            in_game_achievement("Expert of the Dojo", "Clear 40 Dojo challenges on a single difficulty and unlock the Clapping sound", 40, InGameAchievement::Clear40DojoTrainingStages),
            in_game_achievement("Master of the Dojo", "Clear 50 Dojo challenges on a single difficulty and unlock the Rap sound", 50, InGameAchievement::Clear50DojoTrainingStages),
            control_dojo("Kannushi", "Control 10 prefectures in Omikoshi Story Mode", 20, 10),
            control_dojo("Hatamoto", "Control 20 prefectures in Omikoshi Story Mode", 25, 20),
            control_dojo("Daimyo", "Control 30 prefectures in Omikoshi Story Mode", 30, 30),
            control_dojo("Shogun", "Control 40 prefectures in Omikoshi Story Mode", 35, 40),
            control_dojo("Tenka Fubu", "Control all 47 prefectures in Omikoshi Story Mode", 50, TOTAL_NUM_DOJO),
            in_game_achievement("Encore!", "Clear Omikoshi Story Mode and unlock \"Cycle of Rebirth\"", 25, InGameAchievement::BeatOmikoshiMode),
            in_game_achievement("Devil Mask On", "Beat 2 consecutive opponents in Boss Rush Mode and obtain the Devil Face headgear", 15, InGameAchievement::Beat2OpponentsInARowInBossRushMode),
            in_game_achievement("Devil Suit Up", "Beat 5 consecutive opponents in Boss Rush Mode and obtain the Devil Body clothes", 25, InGameAchievement::Beat5OpponentsInARowInBossRushMode),
            in_game_achievement("Boss Banisher", "Clear Boss Rush Mode and unlock unlimited clone usage in Omikoshi Story Mode", 35, InGameAchievement::ClearTheBossRushMode),
            in_game_achievement("Drum Major", "Gather 100 or more Dojo members in Omikoshi Story Mode and unlock the Chibi Don", 15, InGameAchievement::Gather100OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("Bandmaster", "Gather 200 or more Dojo members in Omikoshi Story Mode and unlock the Gangster Don", 20, InGameAchievement::Gather200OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("Kapellmeister", "Gather 300 Dojo members in Omikoshi Story Mode and unlock the Yokozuna Don", 25, InGameAchievement::Gather300OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("Overwhelming Victory", "Beat the opponent by more than 9 meters in an Omikoshi battle", 0, InGameAchievement::BeatTheOpponentByMoreThan9MetersInADojoBattle),
            in_game_achievement("Fiasco Fanfare", "Fail 10 times in Freeplay Mode and unlock the Fart sound", 10, InGameAchievement::Fail10TimesInFreeplay),
            earn_gold_on("Palico Pal", 25, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan, Song::TheWorldIsAllOne, Difficulty::Kantan, Some("the Idolm@ster Ribbon headgear")),
            earn_gold_on("765PRO Prospect", 25, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnKantan, Song::MonsterHunterMedley, Difficulty::Kantan, Some("the Felyne Helm headgear")),
            earn_gold_on("Kabukicho Kid", 25, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnKantan, Song::MachineGunKiss, Difficulty::Kantan, Some("the Signpost headgear")),
            earn_gold_on("Hologram Hero", 25, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan, Song::HatsuneMikuNoGekishou, Difficulty::Kantan, Some("the Miku Hair headgear")),
            earn_gold_on("765PRO Pop Star", 25, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu, Song::TheWorldIsAllOne, Difficulty::Futsuu, Some("the Idolm@ster Dress clothes")),
            earn_gold_on("Palico Prime", 25, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu, Song::MonsterHunterMedley, Difficulty::Futsuu, Some("the Felyne Body clothes")),
            earn_gold_on("Kamurocho Kingpin", 25, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnFutsuu, Song::MachineGunKiss, Difficulty::Futsuu, Some("the Kamuro Gate clothes")),
            earn_gold_on("Vocaloid Virtuoso", 25, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu, Song::HatsuneMikuNoGekishou, Difficulty::Futsuu, Some("the Miku Dress clothes")),
            in_game_achievement("Combo Spark", "Achieve a combo of 500 or more hits in Medley Mode", 15, InGameAchievement::ReachAComboOf500OrMoreInMedleyMode),
            in_game_achievement("Combotastic", "Achieve a combo of 1000 or more hits in Medley Mode", 15, InGameAchievement::ReachAComboOf1000OrMoreInMedleyMode),
            in_game_achievement("Combo Creator", "Achieve a combo of 1500 or more hits in Medley Mode", 20, InGameAchievement::ReachAComboOf1500OrMoreInMedleyMode),
            in_game_achievement("Combo Master", "Achieve a combo of 2000 or more hits in Medley Mode", 25, InGameAchievement::ReachAComboOf2000OrMoreInMedleyMode),
            in_game_achievement("Kung Fu Buzzer", "Get 500 or more drumroll hits in Medley Mode and unlock the Kung Fu sound", 15, InGameAchievement::DrumrollATotalOf500TimesOrMoreInMedleyMode),
            in_game_achievement("Explosive Finish", "Score 3,000,000 or more points in Medley Mode and unlock the Dynamite sound", 25, InGameAchievement::Score3000000PointsOrMoreInMedleyMode),

            // Mail.
            recieved_mail("Jyuuikkyu", 10, PromotionMail::Apprentice),
            recieved_mail("Jyuukyu", 10, PromotionMail::Novice),
            recieved_mail("Kyuukyu", 10, PromotionMail::HalfAMan),
            recieved_mail("Hakkyu", 10, PromotionMail::Qualified),
            recieved_mail("Nankyu", 10, PromotionMail::Capable),
            recieved_mail("Rokkyu", 10, PromotionMail::GoGetter),
            recieved_mail("Gokyu", 10, PromotionMail::Professional),
            recieved_mail("Yonkyu", 10, PromotionMail::Expert),
            recieved_mail("Sankyu", 10, PromotionMail::IronMan),
            recieved_mail("Nikyu", 10, PromotionMail::Master),
            recieved_mail("Ikkyu", 10, PromotionMail::GrandMaster),

            // Modifiers
            clear_song_with_modifier_active("Fast Lane", "Clear any song with the Double Speed modifier or higher active", 15, SpeedModifier::is_or_higher(SpeedModifier::Double)),
            clear_song_with_modifier_active("Rush Hour", "Clear any song with the Triple Speed modifier or higher active", 15, SpeedModifier::is_or_higher(SpeedModifier::Triple)),
            clear_song_with_modifier_active("Hyper Speed", "Clear any song with the Quadruple Speed modifier active", 15, SpeedModifier::is(SpeedModifier::Quadruple)),
            clear_song_with_special_modifier("Playing for Perfection", "Clear any song with the Sudden Death modifier active", 15, SpecialModifier::SuddenDeath),
            clear_song_with_special_modifier("Ghost Player", "Clear any song with the Invisible modifier active", 15, SpecialModifier::Invisible),
            clear_song_with_modifier_active("Whimsy Waltz", "Clear any song with the Whimsical modifier active", 15, NoteModifier::is(NoteModifier::Whimsical)),
            clear_song_with_modifier_active("Caprice Cadence", "Clear any song with the Random modifier active", 15, NoteModifier::is(NoteModifier::Random)),
            clear_song_with_modifier_active("Gyaku Grove", "Clear any song with the Reversed modifier active", 15, NoteModifier::is(NoteModifier::Reveresed)),
        ];
        core.extend(Self::generate_song_full_combos());
        core
    }

    #[rustfmt::skip]
    fn generate_song_full_combos() -> Vec<Achievement> {
        vec![
            // Anime
            clear_song_with_result("A Sky After Rain", Song::OneDay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Two Hearts, One Horizon", Song::Lovers, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Born to Survive", Song::Lion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Heads Means Go!", Song::NoButs, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Twenty-Four-Hour Heartbeat", Song::HeavyRotation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Play Hard, Study Harder", Song::YokuAsobiYokuManabe, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("I Wish! I Wish!", Song::IWishForYou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Thousand Ways to Say Thank You", Song::Arigatou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("I Wanna Play, Definitely!", Song::Joyful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Summer Sparks, Silent Hearts", Song::NatsuMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("At 2 AM", Song::TentaiKansou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Red Requiem", Song::Kurenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Spiral of Zero", Song::Punishment, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Hey Hey Hey, Mister!", Song::Mister, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jounetsu, Unleashed", Song::JounetsuTairiku, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Classic
            clear_song_with_result("Your Heartbeat in Dreams", Song::PavaneForADeadPrincess, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Ode to the Drum", Song::NinthSymphony, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Leichte Kavallerie, Heavy Taiko", Song::ExcerptFromLightCavalryOverture, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Night's Allegro", Song::EineKleineNachtmuzik, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Violin Virtuoso Vengeance", Song::ViolinConcertoInEMinor, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Radetzky Rampage", Song::RadetzkyMarch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dashing Through D Major", Song::KareKantonoKanonan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Torrential Fury", Song::EtudeOp104, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sixteenth-Note Surge", Song::EtudeOp104Ura, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Elder Dragon Hunter", Song::MonsterHunterMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Million Kiss Mayhem", Song::MachineGunKiss, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Unity in Every Beat", Song::TheWorldIsAllOne, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Super Rule!", Song::KirameKirari, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Gauntlet of God", Song::MujihiNaOu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Fists of Fate", Song::Karma, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pittan Hearts Parade", Song::Bambini, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pecs, Pursuit and Passion", Song::PaPaPaLove, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Mappy's Musical Mischief", Song::MappyMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("City Pop Cruise", Song::MagicalSoundShower, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Voice Beyond Zero", Song::HatsuneMikuNoGekishou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Delivering the DIVA", Song::HatsuneMikuNoGekishouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Wasshoi Whirlwind", Song::DokoDONMatsuRhythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Strings of Forbidden Love", Song::Canadea, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Cosmic Bird Takes Flight", Song::SORAVCosmicBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pitch Perfect Paws and Purrs", Song::WanyaWorld, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Funky Monkey", Song::AllNightdeIndenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Symphony of the Small Spaceship", Song::MagicalLittleSpaceship, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Cotton Candy Companions", Song::ShiroNekoCaramelMugennoWataame, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("White Rose Rebellion", Song::WhiteRoseInsanity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rage of the Rose", Song::WhiteRoseInsanityUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Azure Aria", Song::AoNoSenritsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Heart of a Howling Hound", Song::InuHoeru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Clashing Confessions", Song::LOVEIkusa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sunshine Celebration", Song::TaiyouMoYappappa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Sun Strides On", Song::TaiyouMoYappappaUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jazz Above the Clouds", Song::SportsDigestDon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Skyward Syncopation", Song::SportsDigestDonUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Happiness in Harmony", Song::HappyPeace, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Prince's Grand Roll", Song::RaMorenaKumonai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sizzling in the Sahara", Song::DesertDeYakiniku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Gusto Noch", Song::RindaWaKyoumoZekkouchou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Linda the Robot Warrior", Song::RindaWaKyoumoZekkouchouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Midnight Matsuri", Song::DensetsunoMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("DON AND KAT", Song::DontCut, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("DON'T CUT, DON'T QUIT", Song::DontCutUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Whistle on the Cliffs", Song::DunAonghasanoFuefuki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rumble and Resound", Song::RumbleRanbu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rumble Riot", Song::RumbleRanbuUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Buru-chan's Snack Time", Song::BuruChanNoOya2, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Cup of Downpour", Song::KissaRain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dodon-derful Delirium", Song::DodododoDonderful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Subspace Drifter", Song::AkuukanYuueiac125, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Hellfire Codex", Song::JigokunoTaikoJiten, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Destiny Takes Flight", Song::BeTheAce, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dreams Beneath the Boughs", Song::Mulberry, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Three-stringed Threat", Song::SengokuSangen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Night Parade of a Hunderd Haunts", Song::HyakkiYakou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Termination Sequence", Song::RotterTarmination, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Nanja Korya?!", Song::RotterTarminationUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Apostle of the Black Thorn", Song::BlackRoseApostle, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Requiem for a Black Rose", Song::BlackRoseApostleUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Carry the One", Song::Soroban2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Seven Days to Zero", Song::XDay2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jelly Fry Jubilee", Song::MataSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Where Hope Takes Wing", Song::CycleOfRebirth, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
