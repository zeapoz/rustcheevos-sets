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
            clear_song_with_result("Song FC - Detective Conan Main Theme", Song::DetectiveConanMainTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Guts Guts!!", Song::GutsGuts, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - My Soul, Your Beats!", Song::MySoulYourBeats, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tougenkyou Alien", Song::TougenkyouAlien, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - A Cruel Angel's Thesis", Song::ACruelAngelsThesis, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank!", Song::Tank, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tank! (Ura)", Song::TankUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sousei no Aquarion", Song::SouseiNoAquarion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - READY!!", Song::Ready, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Lupin III Theme '78", Song::LupinTheThirdTheme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Touch", Song::Touch, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hare Hare Yukai", Song::HareHareYukai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - W-B-X ~W Boiled Extreme~", Song::BoiledExtreme, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Are!", Song::WeAre, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yuuki 100%", Song::Yuuki100Percent, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - CHA-LA HEAD-CHA-LA", Song::ChaLaHeadChaLa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - SWITCH ON!", Song::SwitchOn, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Motteke! Sailor Fuku", Song::MottekeSailorFuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Oshiri Kajiri Mushi", Song::OshiriKajiriMushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tomare!", Song::Tomare, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 1 Dream", Song::OneDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - By My Side", Song::ByMySide, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - We Go!", Song::WeGo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Orion o Nazoru", Song::OrionONazoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kill me no Baby!", Song::KillMeNoBaby, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pegasus Fantasy", Song::PegasusFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Life is SHOW TIME", Song::LifeIsShowTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi o Nosete", Song::KimiONosete, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // J-Pop
            clear_song_with_result("Song FC - Ponytail to Shushu", Song::PonytailToShushu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ring a Ding Dong", Song::RinADingDong, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kiseki", Song::Kiseki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Polyrhythm", Song::Polyrythm, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Everyday, Katyusha", Song::EverydyKatyusha, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LISTEN TO THE STEREO!!", Song::ListenToTheStereo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sakuranbo", Song::Sakuranbo, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Karon", Song::Karon, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Sobakasu", Song::Sobakasu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Egao ni Kanpai!", Song::EgaoNiKanpai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - LOVE Zukkyun", Song::LoveZukkyun, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Blue Bird", Song::BlueBird, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kibun Joujou", Song::KibunJoujou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Bakuchi Dancer", Song::BakuchiDancer, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Linda Linda", Song::LindaLinda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ikenai Taiyou", Song::IkenaiTaiyou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Niji", Song::Niji, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Ai wa Katsu", Song::AiWaKatsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 365 Nichi no Love Story", Song::NichiNoLoveStory, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - TRAIN-TRAIN", Song::TrainTrain, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Aruite Ikou", Song::AruiteIkou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsubomi", Song::Tsubomi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimagure Romantic", Song::KimagureRomantic, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Flying Get", Song::FlyingGet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Misenai Namida wa, Kitto Itsu Ka", Song::MisenaiNomidaWaKittoItsuKa, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shine", Song::Shine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - KISS KISS BANG BANG", Song::KissKissBangBang, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Memeshikute", Song::Memeshikute, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Variety
            clear_song_with_result("Song FC - Melt", Song::Melt, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Black Rock Shooter", Song::BlackRockShooter, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Maru Maru Mori Mori", Song::MaruMaruMoriMori, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - World is Mine", Song::WorldIsMine, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Go Go Yuureisen", Song::GoGoYuureisen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RHYTHM AND POLICE", Song::RhythmAndPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Otemoyan", Song::Otemoyan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Atarimae Taisou", Song::AtarimaeTaisou, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Matryoshka", Song::Matryoshka, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinsei Reset Button", Song::JinseiResetButton, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Game Music
            clear_song_with_result("Song FC - SMOKY THRILL", Song::SmokyThrill, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back", Song::NoWayBack, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - No Way Back (Ura)", Song::NoWayBackUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Naked Glow", Song::NakedGlow, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Do-Dai", Song::DoDai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Doom Noiz", Song::DoomNoiz, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Agent Yoru o Yuku", Song::AgentYoruOYuku, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Overmaster", Song::Overmaster, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GO MY WAY !!", Song::GoMyWay, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Hawk BGM1", Song::MetalHawkBGM1, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - RAGE v.self", Song::RageVself, SongResult::GoldenCrown, Difficulty::Muzukashii),

            // Namco Original
            clear_song_with_result("Song FC - Angel Dream", Song::AngelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Fantasy", Song::KazeNoFantasy, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Pastel Dream", Song::PastelDream, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Nagisa no Andromeda", Song::NagisanoAndromeda, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Saitama 2000", Song::Saitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Tsukikage SASURAI", Song::TsukikageSasurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hyakka Ryouran", Song::HyakkaRyouran, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Zastohl no Madousho", Song::ZastohlNoMadousho, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Meena no Oyashiki", Song::MeenaNoOyashiki, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 2000", Song::KitaSaitama2000, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kita Saitama 200", Song::KitaSaitama200, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Uchuu Samurai", Song::UchuuSamurai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Danba Danba Din Dan", Song::DanbaDanbaDinDan, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kimi no Planet", Song::KimiNoPlanet, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - STAGE 0. ac11", Song::Stage0Ac11, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kuon no Yoru", Song::KuonNoYoru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - The Carnivorous Carnival", Song::TheCarnivorousCarnival, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hatsune Miku no Shoushitsu", Song::HatsuneMikuNoShoushitsu, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kayou ~Flourishing Blossoms~", Song::KayouFlourishingBlossoms, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - GERMINATION", Song::Germiniation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Crane City", Song::CraneCity, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kurukuru Kurokkuru", Song::KurukuruKurokkuru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei", Song::FuunBachioSensei, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Fuun! Bachi o Sensei Long Version", Song::FuunBachioSenseiLongVersion, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Jinpuumaru", Song::Jinpuumaru, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai", Song::YozakuraShanikusai, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Yozakura Shanikusai (Ura)", Song::YozakuraShanikusaiUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police", Song::MetalPolice, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Metal Police (Ura)", Song::MetalPoliceUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - DIMENSIONS", Song::Dimensions, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Dokidoki Mune Kyun Omatsuri Time", Song::DokidokiMuneKyunOmatsuriTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Carnation", Song::Carnation, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Koi no Shohousen", Song::KoiNoShohousen, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Hanaoto Ura Hyoushi", Song::HanaotoUraHyoushi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Kaze no Kuni no Ryu to Kishi", Song::KazeNoKuniNoRyuToKishi, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time", Song::TaikoTime, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Taiko Time (Ura)", Song::TaikoTimeUra, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - 3piece-Jazz Party!", Song::ThreePieceJazzParty, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - junction", Song::Junction, SongResult::GoldenCrown, Difficulty::Muzukashii),
            clear_song_with_result("Song FC - Shimedore 2000+", Song::Shimedore2000Plus, SongResult::GoldenCrown, Difficulty::Muzukashii),
        ]
    }
}
