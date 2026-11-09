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
            clear_song_with_result("Song FC - Detective Conan Main Theme", 0, Song::DetectiveConanMainTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Guts Guts!!", 0, Song::GutsGuts, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - My Soul, Your Beats!", 0, Song::MySoulYourBeats, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tougenkyou Alien", 0, Song::TougenkyouAlien, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - A Cruel Angel's Thesis", 0, Song::ACruelAngelsThesis, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank!", 0, Song::Tank, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank! (Ura)", 0, Song::TankUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sousei no Aquarion", 0, Song::SouseiNoAquarion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - READY!!", 0, Song::Ready, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Lupin III Theme '78", 0, Song::LupinTheThirdTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Touch", 0, Song::Touch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hare Hare Yukai", 0, Song::HareHareYukai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - W-B-X ~W Boiled Extreme~", 0, Song::BoiledExtreme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Are!", 0, Song::WeAre, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yuuki 100%", 0, Song::Yuuki100Percent, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - CHA-LA HEAD-CHA-LA", 0, Song::ChaLaHeadChaLa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - SWITCH ON!", 0, Song::SwitchOn, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Motteke! Sailor Fuku", 0, Song::MottekeSailorFuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Oshiri Kajiri Mushi", 0, Song::OshiriKajiriMushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tomare!", 0, Song::Tomare, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 1 Dream", 0, Song::OneDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - By My Side", 0, Song::ByMySide, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Go!", 0, Song::WeGo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Orion o Nazoru", 0, Song::OrionONazoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kill me no Baby!", 0, Song::KillMeNoBaby, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pegasus Fantasy", 0, Song::PegasusFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Life is SHOW TIME", 0, Song::LifeIsShowTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi o Nosete", 0, Song::KimiONosete, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Song FC - Ponytail to Shushu", 0, Song::PonytailToShushu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ring a Ding Dong", 0, Song::RinADingDong, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kiseki", 0, Song::Kiseki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Polyrhythm", 0, Song::Polyrythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Everyday, Katyusha", 0, Song::EverydyKatyusha, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LISTEN TO THE STEREO!!", 0, Song::ListenToTheStereo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sakuranbo", 0, Song::Sakuranbo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Karon", 0, Song::Karon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sobakasu", 0, Song::Sobakasu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Egao ni Kanpai!", 0, Song::EgaoNiKanpai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LOVE Zukkyun", 0, Song::LoveZukkyun, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Blue Bird", 0, Song::BlueBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kibun Joujou", 0, Song::KibunJoujou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Bakuchi Dancer", 0, Song::BakuchiDancer, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Linda Linda", 0, Song::LindaLinda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ikenai Taiyou", 0, Song::IkenaiTaiyou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Niji", 0, Song::Niji, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ai wa Katsu", 0, Song::AiWaKatsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 365 Nichi no Love Story", 0, Song::NichiNoLoveStory, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - TRAIN-TRAIN", 0, Song::TrainTrain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Aruite Ikou", 0, Song::AruiteIkou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsubomi", 0, Song::Tsubomi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimagure Romantic", 0, Song::KimagureRomantic, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Flying Get", 0, Song::FlyingGet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Misenai Namida wa, Kitto Itsu Ka", 0, Song::MisenaiNomidaWaKittoItsuKa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shine", 0, Song::Shine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - KISS KISS BANG BANG", 0, Song::KissKissBangBang, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Memeshikute", 0, Song::Memeshikute, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Song FC - Melt", 0, Song::Melt, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Black Rock Shooter", 0, Song::BlackRockShooter, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Maru Maru Mori Mori", 0, Song::MaruMaruMoriMori, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - World is Mine", 0, Song::WorldIsMine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Go Go Yuureisen", 0, Song::GoGoYuureisen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RHYTHM AND POLICE", 0, Song::RhythmAndPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Otemoyan", 0, Song::Otemoyan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Atarimae Taisou", 0, Song::AtarimaeTaisou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Matryoshka", 0, Song::Matryoshka, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinsei Reset Button", 0, Song::JinseiResetButton, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Song FC - SMOKY THRILL", 0, Song::SmokyThrill, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back", 0, Song::NoWayBack, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back (Ura)", 0, Song::NoWayBackUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Naked Glow", 0, Song::NakedGlow, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Do-Dai", 0, Song::DoDai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Doom Noiz", 0, Song::DoomNoiz, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Agent Yoru o Yuku", 0, Song::AgentYoruOYuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Overmaster", 0, Song::Overmaster, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GO MY WAY !!", 0, Song::GoMyWay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Hawk BGM1", 0, Song::MetalHawkBGM1, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RAGE v.self", 0, Song::RageVself, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Song FC - Angel Dream", 0, Song::AngelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Fantasy", 0, Song::KazeNoFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pastel Dream", 0, Song::PastelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Nagisa no Andromeda", 0, Song::NagisanoAndromeda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Saitama 2000", 0, Song::Saitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsukikage SASURAI", 0, Song::TsukikageSasurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hyakka Ryouran", 0, Song::HyakkaRyouran, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Zastohl no Madousho", 0, Song::ZastohlNoMadousho, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Meena no Oyashiki", 0, Song::MeenaNoOyashiki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 2000", 0, Song::KitaSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 200", 0, Song::KitaSaitama200, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Uchuu Samurai", 0, Song::UchuuSamurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Danba Danba Din Dan", 0, Song::DanbaDanbaDinDan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi no Planet", 0, Song::KimiNoPlanet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - STAGE 0. ac11", 0, Song::Stage0Ac11, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kuon no Yoru", 0, Song::KuonNoYoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - The Carnivorous Carnival", 0, Song::TheCarnivorousCarnival, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hatsune Miku no Shoushitsu", 0, Song::HatsuneMikuNoShoushitsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kayou ~Flourishing Blossoms~", 0, Song::KayouFlourishingBlossoms, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GERMINATION", 0, Song::Germiniation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Crane City", 0, Song::CraneCity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kurukuru Kurokkuru", 0, Song::KurukuruKurokkuru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei", 0, Song::FuunBachioSensei, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei Long Version", 0, Song::FuunBachioSenseiLongVersion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinpuumaru", 0, Song::Jinpuumaru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai", 0, Song::YozakuraShanikusai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai (Ura)", 0, Song::YozakuraShanikusaiUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police", 0, Song::MetalPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police (Ura)", 0, Song::MetalPoliceUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - DIMENSIONS", 0, Song::Dimensions, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Dokidoki Mune Kyun Omatsuri Time", 0, Song::DokidokiMuneKyunOmatsuriTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Carnation", 0, Song::Carnation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Koi no Shohousen", 0, Song::KoiNoShohousen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hanaoto Ura Hyoushi", 0, Song::HanaotoUraHyoushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Kuni no Ryu to Kishi", 0, Song::KazeNoKuniNoRyuToKishi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time", 0, Song::TaikoTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time (Ura)", 0, Song::TaikoTimeUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 3piece-Jazz Party!", 0, Song::ThreePieceJazzParty, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - junction", 0, Song::Junction, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shimedore 2000+", 0, Song::Shimedore2000Plus, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
