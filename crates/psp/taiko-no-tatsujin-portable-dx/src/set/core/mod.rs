use crate::{
    set::{
        common::{leaderboard::generate_leaderboards, song::clear_song_with_result},
        core::{
            general::{accumulate_hits, earn_golden_crowns, earn_silver_crowns},
            omikoshi::{clear_omikoshi, obtain_omikoshi_clothes, obtain_omikoshi_headgear},
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
            beat_boss("Gang Buster", "Defeat the Dokon Gang in Omikoshi Story Mode", 10, BattleOpponent::DokonGang),
            beat_boss("Topple the Don", "Defeat the Dokon Gang and Dokon in Omikoshi Story Mode", 10, BattleOpponent::DokonPlusDokonGang),
            beat_boss("Crash Landing", "Defeat Guts Jet in Omikoshi Story Mode", 10, BattleOpponent::GutsJet),
            beat_boss("Devilish Defeat", "Defeat Devil Don-chan in Omikoshi Story Mode", 10, BattleOpponent::DevilDonChan),
            beat_boss("Starving the King", "Defeat Guts Eater in Omikoshi Story Mode", 10, BattleOpponent::GutsEater),
            beat_boss("Armageddon Averted", "Defeat Armage-Don in Omikoshi Story Mode", 10, BattleOpponent::ArmageDon),
            clear_omikoshi("Encore!", "Clear Omikoshi Story Mode and unlock \"Cycle of Rebirth\"", 10, InGameAchievement::BeatOmikoshiMode),

            in_game_achievement("Kannushi", "Control 10 prefectures in Omikoshi Story Mode", 5, InGameAchievement::Control10DojosInOmikoshiMode),
            in_game_achievement("Hatamoto", "Control 20 prefectures in Omikoshi Story Mode", 5, InGameAchievement::Control20DojosInOmikoshiMode),
            in_game_achievement("Daimyo", "Control 30 prefectures in Omikoshi Story Mode", 5, InGameAchievement::Control30DojosInOmikoshiMode),
            in_game_achievement("Shogun", "Control 40 prefectures in Omikoshi Story Mode", 5, InGameAchievement::Control40DojosInOmikoshiMode),
            in_game_achievement("Tenka Fubu", "Control all 47 prefectures in Omikoshi Story Mode", 10, InGameAchievement::ControlAllDojosInOmikoshiMode),

            in_game_achievement("Drum Major", "Gather 100 or more Dojo members in Omikoshi Story Mode and unlock the Chibi Don", 5, InGameAchievement::Gather100OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("Bandmaster", "Gather 200 or more Dojo members in Omikoshi Story Mode and unlock the Gangster Don", 5, InGameAchievement::Gather200OrMoreDrumDisciplesInOmikoshiMode),
            in_game_achievement("Kapellmeister", "Gather 300 Dojo members in Omikoshi Story Mode and unlock the Yokozuna Don", 10, InGameAchievement::Gather300OrMoreDrumDisciplesInOmikoshiMode),

            obtain_omikoshi_headgear("Headgear Hunter", "Collect all available headgear in Omikoshi Story Mode", 10),
            obtain_omikoshi_clothes("Fearless Fashionista", "Collect all available clothing in Omikoshi Story Mode", 10),

            in_game_achievement("Novice Buzzer", "Clear 5 Dojo challenges on a single difficulty and unlock the Cameraman sound", 3, InGameAchievement::Clear5DojoTrainingStages),
            in_game_achievement("Apprentice of the Dojo", "Clear 10 Dojo challenges on a single difficulty and unlock the Wooden Fish sound", 5, InGameAchievement::Clear10DojoTrainingStages),
            in_game_achievement("Adept of the Dojo", "Clear 20 Dojo challenges on a single difficulty and unlock the Bell sound", 5, InGameAchievement::Clear20DojoTrainingStages),
            in_game_achievement("Veteran of the Dojo", "Clear 30 Dojo challenges on a single difficulty and unlock the Sneeze sound", 10, InGameAchievement::Clear30DojoTrainingStages),
            in_game_achievement("Expert of the Dojo", "Clear 40 Dojo challenges on a single difficulty and unlock the Clapping sound", 10, InGameAchievement::Clear40DojoTrainingStages),
            in_game_achievement("Master of the Dojo", "Clear 50 Dojo challenges on a single difficulty and unlock the Rap sound", 25, InGameAchievement::Clear50DojoTrainingStages),

            in_game_achievement("Overwhelming Victory", "Beat the opponent by more than 9 meters in an Omikoshi battle", 5, InGameAchievement::BeatTheOpponentByMoreThan9MetersInADojoBattle),

            // In-game achievements
            in_game_achievement("First Beat", "Clear your first song", 3, InGameAchievement::FirstClear),
            earn_silver_crowns("Bachi Basics", 5, InGameAchievement::Earn15SilverCrownsOnKantan, 3, Difficulty::Kantan, None),
            earn_silver_crowns("Beat Baron", 5, InGameAchievement::Earn30SilverCrownsOnKantan, 5, Difficulty::Kantan, Some("the Reversed modifier")),
            earn_silver_crowns("Kantan Complete", 10, InGameAchievement::Earn70SilverCrownsOnKantan, 10, Difficulty::Kantan, None),
            earn_silver_crowns("Rhythm Warm-Up", 5, InGameAchievement::Earn15SilverCrownsOnFutsuu, 4, Difficulty::Futsuu, None),
            earn_silver_crowns("Beat Brawler", 5, InGameAchievement::Earn30SilverCrownsOnFutsuu, 5, Difficulty::Futsuu, Some("the Invisible modifier")),
            earn_silver_crowns("Futsuu Finished", 10, InGameAchievement::Earn70SilverCrownsOnFutsuu, 10, Difficulty::Futsuu, None),
            earn_silver_crowns("Dealing with the Devil", 5, InGameAchievement::Earn15SilverCrownsOnMuzukashii, 5, Difficulty::Muzukashii, Some("Oni Difficulty")),
            earn_silver_crowns("Beat Breaker", 10, InGameAchievement::Earn30SilverCrownsOnMuzukashii, 10, Difficulty::Muzukashii, Some("the Random modifier")),
            earn_silver_crowns("Muzukashii Master", 25, InGameAchievement::Earn70SilverCrownsOnMuzukashii, 25, Difficulty::Muzukashii, None),
            earn_silver_crowns("Oni Overlord", 10, InGameAchievement::Earn15SilverCrownsOnOni, 10, Difficulty::Oni, Some("the Triple Speed modifier")),
            earn_silver_crowns("Beat Behemoth", 25, InGameAchievement::Earn30SilverCrownsOnOni, 25, Difficulty::Oni, Some("the Quadruple Speed Modifier")),
            earn_silver_crowns("Oni Obliterator", 50, InGameAchievement::Earn70SilverCrownsOnOni, 50, Difficulty::Oni, None),
            earn_golden_crowns("Crown Chaser", 5, InGameAchievement::Earn10GoldenCrowns, 5, "the Sudden Death modifier"),
            earn_golden_crowns("Full Crescendo", 10, InGameAchievement::Earn25GoldenCrowns, 10, "the Tsuzumi sound"),
            earn_golden_crowns("Donder Kong", 10, InGameAchievement::Earn50GoldenCrowns, 10, "the Conga sound"),
            earn_golden_crowns("8-Bit Ace", 25, InGameAchievement::Earn75GoldenCrowns, 25, "the 8-bit Taiko sound"),
            earn_golden_crowns("Deluxe Donder", 50, InGameAchievement::Earn100GoldenCrowns, 50, "the Deluxe Taiko Sound"),

            donderful_combo("Donderful Combo!", "Clear a song with only Great hits and no misses and unlock the Gun sound", 10, InGameAchievement::ClearASongWithAllGreatHitsAndNoMisses),
            false_donderful_combo("A Bitter Taste", "Clear a song with only OK hits and no misses and unlock the Shuriken sound", 10, InGameAchievement::ClearASongWithAllGoodHitsAndNoMisses),

            in_game_achievement("Combo Maniac", "Achieve a combo of 100 or more hits in Freeplay Mode", 2, InGameAchievement::ReachAComboOf100HitsOrMore),
            in_game_achievement("Combo Fever", "Achieve a combo of 200 or more hits in Freeplay Mode", 3, InGameAchievement::ReachAComboOf200HitsOrMore),
            in_game_achievement("Combo Craze", "Achieve a combo of 400 or more hits in Freeplay Mode", 4, InGameAchievement::ReachAComboOf400HitsOrMore),
            in_game_achievement("Combo Catcher", "Achieve a combo of 600 or more hits in Freeplay Mode", 5, InGameAchievement::ReachAComboOf600HitsOrMore),
            in_game_achievement("Novice Buzzer", "Get 100 or more drumroll hits in a song and unlock the Quiz sound", 3, InGameAchievement::GetMoreThan100TotalHitsInASong),
            in_game_achievement("A Deadly Beat", "Score 500,000 or more points in a song in Freeplay Mode and unlock the Knife sound", 3, InGameAchievement::Score500000PointsOrMoreInASong),
            in_game_achievement("Fiasco Fanfare", "Fail 10 times in Freeplay Mode and unlock the Fart sound", 1, InGameAchievement::Fail10TimesInFreeplay),

            earn_gold_on("Palico Pal", 3, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan, Song::TheWorldIsAllOne, Difficulty::Kantan, Some("the Idolm@ster Ribbon headgear")),
            earn_gold_on("765PRO Prospect", 3, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnKantan, Song::MonsterHunterMedley, Difficulty::Kantan, Some("the Felyne Helm headgear")),
            earn_gold_on("Kabukicho Kid", 3, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnKantan, Song::MachineGunKiss, Difficulty::Kantan, Some("the Signpost headgear")),
            earn_gold_on("Hologram Hero", 3, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan, Song::HatsuneMikuNoGekishou, Difficulty::Kantan, Some("the Miku Hair headgear")),
            earn_gold_on("765PRO Pop Star", 3, InGameAchievement::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu, Song::TheWorldIsAllOne, Difficulty::Futsuu, Some("the Idolm@ster Dress clothes")),
            earn_gold_on("Palico Prime", 3, InGameAchievement::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu, Song::MonsterHunterMedley, Difficulty::Futsuu, Some("the Felyne Body clothes")),
            earn_gold_on("Kamurocho Kingpin", 3, InGameAchievement::EarnGoldenCrownOnMachineGunKissOnFutsuu, Song::MachineGunKiss, Difficulty::Futsuu, Some("the Kamuro Gate clothes")),
            earn_gold_on("Vocaloid Virtuoso", 3, InGameAchievement::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu, Song::HatsuneMikuNoGekishou, Difficulty::Futsuu, Some("the Miku Dress clothes")),

            in_game_achievement("Combo Spark", "Achieve a combo of 500 or more hits in Medley Mode", 2, InGameAchievement::ReachAComboOf500OrMoreInMedleyMode),
            in_game_achievement("Combotastic", "Achieve a combo of 1000 or more hits in Medley Mode", 3, InGameAchievement::ReachAComboOf1000OrMoreInMedleyMode),
            in_game_achievement("Combo Creator", "Achieve a combo of 1500 or more hits in Medley Mode", 5, InGameAchievement::ReachAComboOf1500OrMoreInMedleyMode),
            in_game_achievement("Combo Master", "Achieve a combo of 2000 or more hits in Medley Mode", 10, InGameAchievement::ReachAComboOf2000OrMoreInMedleyMode),
            in_game_achievement("Kung Fu Buzzer", "Get 500 or more drumroll hits in Medley Mode and unlock the Kung Fu sound", 3, InGameAchievement::DrumrollATotalOf500TimesOrMoreInMedleyMode),
            in_game_achievement("Explosive Finish", "Score 3,000,000 or more points in Medley Mode and unlock the Dynamite sound", 5, InGameAchievement::Score3000000PointsOrMoreInMedleyMode),

            in_game_achievement("Devil Mask On", "Beat 2 consecutive opponents in Boss Rush Mode and obtain the Devil Face headgear", 3, InGameAchievement::Beat2OpponentsInARowInBossRushMode),
            in_game_achievement("Devil Suit Up", "Beat 5 consecutive opponents in Boss Rush Mode and obtain the Devil Body clothes", 5, InGameAchievement::Beat5OpponentsInARowInBossRushMode),
            in_game_achievement("Boss Banisher", "Clear Boss Rush Mode and unlock unlimited clone usage in Omikoshi Story Mode", 10, InGameAchievement::ClearTheBossRushMode),

            accumulate_hits("Ichiman Instrumentalist", 5, InGameAchievement::Accumulate10000HitsOnTheTaikoCounter, 10_000, "10,000", Song::EtudeOp104Ura),
            accumulate_hits("Niman Notator", 5, InGameAchievement::Accumulate20000HitsOnTheTaikoCounter, 20_000, "20,000", Song::BlackRoseApostleUra),
            accumulate_hits("Sanman Soloist", 5, InGameAchievement::Accumulate30000HitsOnTheTaikoCounter, 30_000, "30,000", Song::HatsuneMikuNoGekishouUra),
            accumulate_hits("Yonman Jammer", 5, InGameAchievement::Accumulate40000HitsOnTheTaikoCounter, 40_000, "40,000", Song::RotterTarminationUra),
            accumulate_hits("Goman Groover", 10, InGameAchievement::Accumulate50000HitsOnTheTaikoCounter, 50_000, "50,000", Song::WhiteRoseInsanityUra),
            accumulate_hits("Rokuman Rhapsodist", 10, InGameAchievement::Accumulate60000HitsOnTheTaikoCounter, 60_000, "60,000", Song::TaiyouMoYappappaUra),
            accumulate_hits("Nanan Notesmith", 10, InGameAchievement::Accumulate70000HitsOnTheTaikoCounter, 70_000, "70,000", Song::SportsDigestDonUra),
            accumulate_hits("Shichiman Syncopator", 5, InGameAchievement::Accumulate76500HitsOnTheTaikoCounter, 76_500, "76,500", Song::RindaWaKyoumoZekkouchouUra),
            accumulate_hits("Hachiman Hitmaker", 10, InGameAchievement::Accumulate80000HitsOnTheTaikoCounter, 80_000, "80,000", Song::DontCutUra),
            accumulate_hits("Kyuuman Kodo", 10, InGameAchievement::Accumulate90000HitsOnTheTaikoCounter, 90_000, "90,000", Song::RumbleRanbuUra),
            accumulate_hits("Man Tan", 25, InGameAchievement::Accumulate99999HitsOnTheTaikoCounter, 99_999, "99,999", Song::MataSaitama2000),

            // Mail.
            recieved_mail("Jyuuikkyu", 2, PromotionMail::Apprentice),
            recieved_mail("Jyuukyu", 3, PromotionMail::Novice),
            recieved_mail("Kyuukyu", 5, PromotionMail::HalfAMan),
            recieved_mail("Hakkyu", 5, PromotionMail::Qualified),
            recieved_mail("Nankyu", 5, PromotionMail::Capable),
            recieved_mail("Rokkyu", 10, PromotionMail::GoGetter),
            recieved_mail("Gokyu", 10, PromotionMail::Professional),
            recieved_mail("Yonkyu", 10, PromotionMail::Expert),
            recieved_mail("Sankyu", 10, PromotionMail::IronMan),
            recieved_mail("Nikyu", 25, PromotionMail::Master),
            recieved_mail("Ikkyu", 50, PromotionMail::GrandMaster),

            // Modifiers
            clear_song_with_modifier_active("Fast Lane", "Clear any song with the Double Speed modifier or higher active", 3, SpeedModifier::is_or_higher(SpeedModifier::Double)),
            clear_song_with_modifier_active("Rush Hour", "Clear any song with the Triple Speed modifier or higher active", 5, SpeedModifier::is_or_higher(SpeedModifier::Triple)),
            clear_song_with_modifier_active("Hyper Speed", "Clear any song with the Quadruple Speed modifier active", 10, SpeedModifier::is(SpeedModifier::Quadruple)),
            clear_song_with_special_modifier("Playing for Perfection", "Clear any song with the Sudden Death modifier active", 5, SpecialModifier::SuddenDeath),
            clear_song_with_special_modifier("Ghost Player", "Clear any song with the Invisible modifier active", 25, SpecialModifier::Invisible),
            clear_song_with_modifier_active("Whimsy Waltz", "Clear any song with the Whimsical modifier active", 5, NoteModifier::is(NoteModifier::Whimsical)),
            clear_song_with_modifier_active("Caprice Cadence", "Clear any song with the Random modifier active", 5, NoteModifier::is(NoteModifier::Random)),
            clear_song_with_modifier_active("Gyaku Grove", "Clear any song with the Reversed modifier active", 5, NoteModifier::is(NoteModifier::Reveresed)),
        ];
        core.extend(Self::generate_song_full_combos());
        core
    }

    #[rustfmt::skip]
    fn generate_song_full_combos() -> Vec<Achievement> {
        vec![
            // Anime
            clear_song_with_result("A Sky After Rain", 5, Song::OneDay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Two Hearts, One Horizon", 5, Song::Lovers, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Born to Survive", 5, Song::Lion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Heads Means Go!", 5, Song::NoButs, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Twenty-Four-Hour Heartbeat", 5, Song::HeavyRotation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Play Hard, Study Harder", 5, Song::YokuAsobiYokuManabe, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("I Wish! I Wish!", 5, Song::IWishForYou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Thousand Ways to Say Thank You", 5, Song::Arigatou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("I Wanna Play, Definitely!", 5, Song::Joyful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Summer Sparks, Silent Hearts", 5, Song::NatsuMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("At 2 AM", 5, Song::TentaiKansou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Red Requiem", 10, Song::Kurenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Spiral of Zero", 25, Song::Punishment, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Hey Hey Hey, Mister!", 5, Song::Mister, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jounetsu, Unleashed", 10, Song::JounetsuTairiku, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Classic
            clear_song_with_result("Your Heartbeat in Dreams", 5, Song::PavaneForADeadPrincess, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Ode to the Drum", 5, Song::NinthSymphony, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Leichte Kavallerie, Heavy Taiko", 5, Song::ExcerptFromLightCavalryOverture, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Night's Allegro", 5, Song::EineKleineNachtmuzik, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Violin Virtuoso Vengeance", 5, Song::ViolinConcertoInEMinor, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Radetzky Rampage", 5, Song::RadetzkyMarch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dashing Through D Major", 10, Song::KareKantonoKanonan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Torrential Fury", 25, Song::EtudeOp104, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sixteenth-Note Surge", 25, Song::EtudeOp104Ura, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Elder Dragon Hunter", 10, Song::MonsterHunterMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Million Kiss Mayhem", 5, Song::MachineGunKiss, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Unity in Every Beat", 5, Song::TheWorldIsAllOne, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Super Rule!", 10, Song::KirameKirari, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Gauntlet of God", 5, Song::MujihiNaOu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Fists of Fate", 25, Song::Karma, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pittan Hearts Parade", 10, Song::Bambini, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pecs, Pursuit and Passion", 10, Song::PaPaPaLove, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Mappy's Musical Mischief", 10, Song::MappyMedley, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("City Pop Cruise", 25, Song::MagicalSoundShower, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Voice Beyond Zero", 25, Song::HatsuneMikuNoGekishou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Delivering the DIVA", 25, Song::HatsuneMikuNoGekishouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Wasshoi Whirlwind", 5, Song::DokoDONMatsuRhythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Strings of Forbidden Love", 10, Song::Canadea, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Cosmic Bird Takes Flight", 5, Song::SORAVCosmicBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Pitch Perfect Paws and Purrs", 10, Song::WanyaWorld, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Funky Monkey", 5, Song::AllNightdeIndenai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Symphony of the Small Spaceship", 5, Song::MagicalLittleSpaceship, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Cotton Candy Companions", 5, Song::ShiroNekoCaramelMugennoWataame, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("White Rose Rebellion", 10, Song::WhiteRoseInsanity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rage of the Rose", 10, Song::WhiteRoseInsanityUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Azure Aria", 25, Song::AoNoSenritsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Heart of a Howling Hound", 5, Song::InuHoeru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Clashing Confessions", 10, Song::LOVEIkusa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sunshine Celebration", 5, Song::TaiyouMoYappappa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Sun Strides On", 10, Song::TaiyouMoYappappaUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jazz Above the Clouds", 5, Song::SportsDigestDon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Skyward Syncopation", 10, Song::SportsDigestDonUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Happiness in Harmony", 5, Song::HappyPeace, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Prince's Grand Roll", 10, Song::RaMorenaKumonai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Sizzling in the Sahara", 10, Song::DesertDeYakiniku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Gusto Noch", 10, Song::RindaWaKyoumoZekkouchou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Linda the Robot Warrior", 10, Song::RindaWaKyoumoZekkouchouUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("The Midnight Matsuri", 10, Song::DensetsunoMatsuri, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("DON AND KAT", 25, Song::DontCut, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("DON'T CUT, DON'T QUIT", 25, Song::DontCutUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Whistle on the Cliffs", 10, Song::DunAonghasanoFuefuki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rumble and Resound", 10, Song::RumbleRanbu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Rumble Riot", 25, Song::RumbleRanbuUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Buru-chan's Snack Time", 10, Song::BuruChanNoOya2, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("A Cup of Downpour", 5, Song::KissaRain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dodon-derful Delirium", 5, Song::DodododoDonderful, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Subspace Drifter", 25, Song::AkuukanYuueiac125, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Hellfire Codex", 10, Song::JigokunoTaikoJiten, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Destiny Takes Flight", 5, Song::BeTheAce, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Dreams Beneath the Boughs", 10, Song::Mulberry, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Three-stringed Threat", 10, Song::SengokuSangen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Night Parade of a Hundred Haunts", 10, Song::HyakkiYakou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Termination Sequence", 25, Song::RotterTarmination, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Nanja Korya?!", 25, Song::RotterTarminationUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Apostle of the Black Thorn", 25, Song::BlackRoseApostle, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Requiem for a Black Rose", 25, Song::BlackRoseApostleUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Carry the One", 25, Song::Soroban2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Seven Days to Zero", 25, Song::XDay2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Jelly Fry Jubilee", 25, Song::MataSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Where Hope Takes Wing", 10, Song::CycleOfRebirth, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
