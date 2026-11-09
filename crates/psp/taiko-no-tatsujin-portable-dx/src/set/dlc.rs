use rustcheevos::types::{
    achievement::Achievement,
    set::{Set, Subset, SubsetType},
};

use crate::{
    set::common::{leaderboard::generate_leaderboards, song::clear_song_with_result},
    types::{
        difficulty::Difficulty,
        song::{ALL_DLC_SONGS, Song, result::SongResult},
    },
};

pub struct DLCSubset;

impl DLCSubset {
    pub fn generate() -> Set<Subset> {
        let mut set = Set::new_subset(46365, "DLC", SubsetType::Bonus);
        set.add_achievements(Self::generate_set())
            .add_leaderboards(generate_leaderboards(&ALL_DLC_SONGS));
        set
    }

    #[rustfmt::skip]
    fn generate_set() -> Vec<Achievement> {
        vec![
            // Anime
            clear_song_with_result("Song FC - Detective Conan Main Theme", 5, Song::DetectiveConanMainTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Guts Guts!!", 5, Song::GutsGuts, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - My Soul, Your Beats!", 5, Song::MySoulYourBeats, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tougenkyou Alien", 5, Song::TougenkyouAlien, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - A Cruel Angel's Thesis", 5, Song::ACruelAngelsThesis, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank!", 10, Song::Tank, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank! (Ura)", 25, Song::TankUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sousei no Aquarion", 5, Song::SouseiNoAquarion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - READY!!", 5, Song::Ready, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Lupin III Theme '78", 5, Song::LupinTheThirdTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Touch", 5, Song::Touch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hare Hare Yukai", 10, Song::HareHareYukai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - W-B-X ~W Boiled Extreme~", 5, Song::BoiledExtreme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Are!", 5, Song::WeAre, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yuuki 100%", 5, Song::Yuuki100Percent, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - CHA-LA HEAD-CHA-LA", 5, Song::ChaLaHeadChaLa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - SWITCH ON!", 5, Song::SwitchOn, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Motteke! Sailor Fuku", 10, Song::MottekeSailorFuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Oshiri Kajiri Mushi", 5, Song::OshiriKajiriMushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tomare!", 10, Song::Tomare, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 1 Dream", 5, Song::OneDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - By My Side", 5, Song::ByMySide, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Go!", 5, Song::WeGo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Orion o Nazoru", 10, Song::OrionONazoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kill me no Baby!", 10, Song::KillMeNoBaby, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pegasus Fantasy", 5, Song::PegasusFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Life is SHOW TIME", 5, Song::LifeIsShowTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi o Nosete", 5, Song::KimiONosete, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Song FC - Ponytail to Shushu", 5, Song::PonytailToShushu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ring a Ding Dong", 5, Song::RinADingDong, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kiseki", 5, Song::Kiseki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Polyrhythm", 10, Song::Polyrythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Everyday, Katyusha", 5, Song::EverydyKatyusha, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LISTEN TO THE STEREO!!", 5, Song::ListenToTheStereo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sakuranbo", 10, Song::Sakuranbo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Karon", 5, Song::Karon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sobakasu", 5, Song::Sobakasu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Egao ni Kanpai!", 5, Song::EgaoNiKanpai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LOVE Zukkyun", 10, Song::LoveZukkyun, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Blue Bird", 5, Song::BlueBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kibun Joujou", 5, Song::KibunJoujou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Bakuchi Dancer", 10, Song::BakuchiDancer, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Linda Linda", 5, Song::LindaLinda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ikenai Taiyou", 5, Song::IkenaiTaiyou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Niji", 5, Song::Niji, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ai wa Katsu", 5, Song::AiWaKatsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 365 Nichi no Love Story", 5, Song::NichiNoLoveStory, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - TRAIN-TRAIN", 5, Song::TrainTrain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Aruite Ikou", 5, Song::AruiteIkou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsubomi", 5, Song::Tsubomi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimagure Romantic", 5, Song::KimagureRomantic, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Flying Get", 5, Song::FlyingGet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Misenai Namida wa, Kitto Itsu Ka", 5, Song::MisenaiNomidaWaKittoItsuKa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shine", 5, Song::Shine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - KISS KISS BANG BANG", 5, Song::KissKissBangBang, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Memeshikute", 10, Song::Memeshikute, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Song FC - Melt", 10, Song::Melt, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Black Rock Shooter", 10, Song::BlackRockShooter, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Maru Maru Mori Mori", 5, Song::MaruMaruMoriMori, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - World is Mine", 10, Song::WorldIsMine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Go Go Yuureisen", 5, Song::GoGoYuureisen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RHYTHM AND POLICE", 5, Song::RhythmAndPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Otemoyan", 5, Song::Otemoyan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Atarimae Taisou", 5, Song::AtarimaeTaisou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Matryoshka", 5, Song::Matryoshka, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinsei Reset Button", 25, Song::JinseiResetButton, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Song FC - SMOKY THRILL", 10, Song::SmokyThrill, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back", 10, Song::NoWayBack, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back (Ura)", 25, Song::NoWayBackUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Naked Glow", 10, Song::NakedGlow, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Do-Dai", 5, Song::DoDai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Doom Noiz", 5, Song::DoomNoiz, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Agent Yoru o Yuku", 25, Song::AgentYoruOYuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Overmaster", 10, Song::Overmaster, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GO MY WAY !!", 5, Song::GoMyWay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Hawk BGM1", 10, Song::MetalHawkBGM1, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RAGE v.self", 25, Song::RageVself, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Song FC - Angel Dream", 25, Song::AngelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Fantasy", 5, Song::KazeNoFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pastel Dream", 10, Song::PastelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Nagisa no Andromeda", 10, Song::NagisanoAndromeda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Saitama 2000", 10, Song::Saitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsukikage SASURAI", 10, Song::TsukikageSasurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hyakka Ryouran", 25, Song::HyakkaRyouran, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Zastohl no Madousho", 10, Song::ZastohlNoMadousho, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Meena no Oyashiki", 25, Song::MeenaNoOyashiki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 2000", 25, Song::KitaSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 200", 10, Song::KitaSaitama200, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Uchuu Samurai", 25, Song::UchuuSamurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Danba Danba Din Dan", 25, Song::DanbaDanbaDinDan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi no Planet", 25, Song::KimiNoPlanet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - STAGE 0. ac11", 10, Song::Stage0Ac11, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kuon no Yoru", 5, Song::KuonNoYoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - The Carnivorous Carnival", 25, Song::TheCarnivorousCarnival, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hatsune Miku no Shoushitsu", 10, Song::HatsuneMikuNoShoushitsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kayou ~Flourishing Blossoms~", 25, Song::KayouFlourishingBlossoms, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GERMINATION", 25, Song::Germiniation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Crane City", 25, Song::CraneCity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kurukuru Kurokkuru", 25, Song::KurukuruKurokkuru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei", 10, Song::FuunBachioSensei, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei Long Version", 10, Song::FuunBachioSenseiLongVersion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinpuumaru", 10, Song::Jinpuumaru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai", 10, Song::YozakuraShanikusai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai (Ura)", 10, Song::YozakuraShanikusaiUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police", 10, Song::MetalPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police (Ura)", 10, Song::MetalPoliceUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - DIMENSIONS", 10, Song::Dimensions, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Dokidoki Mune Kyun Omatsuri Time", 10, Song::DokidokiMuneKyunOmatsuriTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Carnation", 25, Song::Carnation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Koi no Shohousen", 25, Song::KoiNoShohousen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hanaoto Ura Hyoushi", 10, Song::HanaotoUraHyoushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Kuni no Ryu to Kishi", 10, Song::KazeNoKuniNoRyuToKishi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time", 5, Song::TaikoTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time (Ura)", 10, Song::TaikoTimeUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 3piece-Jazz Party!", 10, Song::ThreePieceJazzParty, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - junction", 25, Song::Junction, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shimedore 2000+", 25, Song::Shimedore2000Plus, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
