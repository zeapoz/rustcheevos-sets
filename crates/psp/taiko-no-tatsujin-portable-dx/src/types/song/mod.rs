use std::fmt;

use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef},
};

use crate::{
    mem::file_1_song_scoreboard_kantan,
    types::{difficulty::Difficulty, profile::Profile},
};

mod id;
pub mod in_game_stats;

pub struct SongData {
    id: u32,
    title: &'static str,
    dlc: bool,
}

macro_rules! gen_song_defs {
    ($( $variant:ident => $data:expr ),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Song {
            $( $variant ),*
        }

        impl Song {
            pub const ALL: &[Song] = &[ $( Song::$variant ),* ];

            pub const LOOKUP_ENTRIES: &[(u32, &str)] = &[
                $( ($data.id, $data.title) ),*
            ];

            /// Returns the data of the song.
            fn data(self) -> &'static SongData {
                match self {
                    $( Song::$variant => &$data ),*
                }
            }

            /// Returns the title of the song.
            pub fn title(&self) -> &'static str {
                self.data().title
            }

            /// Returns the ID of the song.
            pub fn id(&self) -> u32 {
                self.data().id
            }

            /// Returns a chain resolving to the stored best result for the given difficulty.
            pub fn best_result(&self, difficulty: Difficulty) -> Chain<MemoryRef> {
                const SONG_SIZE_BYTES: u32 = 32;
                const RESULT_OFFSET: usize = 0x1;

                let song_stats_offset = file_1_song_scoreboard_kantan() + difficulty.offset();
                let song_offset = song_stats_offset + (self.id() * SONG_SIZE_BYTES) as usize;

                chain!(
                    Profile::offset_for_current(),
                    bits8!(song_offset + RESULT_OFFSET),
                )
            }
        }

        impl fmt::Display for Song {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.title())
            }
        }
    };
}

gen_song_defs! {
    // == Anime ==
    OneDay => SongData { id: 0x00, title: "One day", dlc: false },
    Lovers => SongData { id: 0x01, title: "Lovers", dlc: false },
    Lion => SongData { id: 0x02, title: "Lion", dlc: false },
    NoButs => SongData { id: 0x03, title: "No buts!", dlc: false },

    // DLC:
    DetectiveConanMainTheme => SongData { id: 0x6b, title: "Detective Conan Main Theme",dlc: true },
    GutsGuts => SongData { id: 0x6c, title: "Guts Guts!!",dlc: true },
    MySoulYourBeats => SongData { id: 0x71, title: "My Soul, Your Beats!",dlc: true },
    TougenkyouAlien => SongData { id: 0x72, title: "Tougenkyou Alien",dlc: true },
    ACruelAngelsThesis => SongData { id: 0x79, title: "A Cruel Angel's Thesis",dlc: true },
    Tank => SongData { id: 0x81, title: "Tank!",dlc: true },
    TankUra => SongData { id: 0x82, title: "Tank! (Ura)",dlc: true },
    SouseiNoAquarion => SongData { id: 0x83, title: "Sousei no Aquarion",dlc: true },
    Ready => SongData { id: 0x8a, title: "READY!!",dlc: true },
    LupinTheThirdTheme => SongData { id: 0x90, title: "Lupin III Theme '78",dlc: true },
    Touch => SongData { id: 0x96, title: "Touch",dlc: true },
    HareHareYukai => SongData { id: 0x97, title: "Hare Hare Yukai",dlc: true },
    BoiledExtreme => SongData { id: 0x9a, title: "W-B-X ~W Boiled Extreme~",dlc: true },
    WeAre => SongData { id: 0x9b, title: "We Are!",dlc: true },
    Yuuki100Percent => SongData { id: 0xa0, title: "Yuuki 100%", dlc: true },
    ChaLaHeadChaLa => SongData { id: 0xa1, title: "CHA-LA HEAD-CHA-LA",dlc: true },
    SwitchOn => SongData { id: 0xa8, title: "SWITCH ON!",dlc: true },
    MottekeSailorFuku => SongData { id: 0xab, title: "Motteke! Sailor Fuku",dlc: true },
    OshiriKajiriMushi => SongData { id: 0xad, title: "Oshiri Kajiri Mushi",dlc: true },
    Tomare => SongData { id: 0xae, title: "Tomare!",dlc: true },
    OneDream => SongData { id: 0xb4, title: "1 Dream",dlc: true },
    ByMySide => SongData { id: 0xc1, title: "By My Side",dlc: true },
    WeGo => SongData { id: 0xca, title: "We Go!",dlc: true },
    OrionONazoru => SongData { id: 0xcb, title: "Orion o Nazoru",dlc: true },
    KillMeNoBaby => SongData { id: 0xd2, title: "Kill me no Baby!",dlc: true },
    PegasusFantasy => SongData { id: 0xd4, title: "Pegasus Fantasy",dlc: true },
    LifeIsShowTime => SongData { id: 0xda, title: "Life is SHOW TIME",dlc: true },
    KimiONosete => SongData { id: 0xe3, title: "Kimi o Nosete",dlc: true },

    // == J-Pop ==
    HeavyRotation => SongData { id: 0x04, title: "Heavy Rotation", dlc: false },
    YokuAsobiYokuManabe => SongData { id: 0x05, title: "Yoku Asobi Yoku Manabe", dlc: false },
    IWishForYou => SongData { id: 0x06, title: "I Wish For You", dlc: false },
    Arigatou => SongData { id: 0x07, title: "Arigatou", dlc: false },
    Joyful => SongData { id: 0x08, title: "Joyful", dlc: false },
    NatsuMatsuri => SongData { id: 0x09, title: "Natsu Matsuri", dlc: false },
    TentaiKansou => SongData { id: 0x0a, title: "Tentai Kansoku", dlc: false },
    Kurenai => SongData { id: 0x0b, title: "Kurenai", dlc: false },
    Punishment => SongData { id: 0x0c, title: "Punishment", dlc: false },

    // DLC:
    PonytailToShushu => SongData { id: 0x65, title: "Ponytail to Shushu", dlc: true },
    RinADingDong => SongData { id: 0x6d, title: "Ring a Ding Dong", dlc: true },
    Kiseki => SongData { id: 0x6e, title: "Kiseki", dlc: true },
    Polyrythm => SongData { id: 0x74, title: "Polyrhythm", dlc: true },
    EverydyKatyusha => SongData { id: 0x75, title: "Everyday, Katyusha", dlc: true },
    ListenToTheStereo => SongData { id: 0x76, title: "LISTEN TO THE STEREO!!", dlc: true },
    Sakuranbo => SongData { id: 0x7b, title: "Sakuranbo", dlc: true },
    Karon => SongData { id: 0x7c, title: "Karon", dlc: true },
    Sobakasu => SongData { id: 0x7f, title: "Sobakasu", dlc: true },
    EgaoNiKanpai => SongData { id: 0x80, title: "Egao ni Kanpai!", dlc: true },
    LoveZukkyun => SongData { id: 0x84, title: "LOVE Zukkyun", dlc: true },
    BlueBird => SongData { id: 0x8b, title: "Blue Bird", dlc: true },
    KibunJoujou => SongData { id: 0x95, title: "Kibun Joujou", dlc: true },
    BakuchiDancer => SongData { id: 0x9c, title: "Bakuchi Dancer", dlc: true },
    LindaLinda => SongData { id: 0x9d, title: "Linda Linda", dlc: true },
    IkenaiTaiyou => SongData { id: 0xa2, title: "Ikenai Taiyou", dlc: true },
    Niji => SongData { id: 0xa3, title: "Niji", dlc: true },
    AiWaKatsu => SongData { id: 0xa6, title: "Ai wa Katsu", dlc: true },
    NichiNoLoveStory => SongData { id: 0xa7, title: "365 Nichi no Love Story", dlc: true },
    TrainTrain => SongData { id: 0xaa, title: "TRAIN-TRAIN", dlc: true },
    AruiteIkou => SongData { id: 0xac, title: "Aruite Ikou", dlc: true },
    Tsubomi => SongData { id: 0xb2, title: "Tsubomi", dlc: true },
    KimagureRomantic => SongData { id: 0xb3, title: "Kimagure Romantic", dlc: true },
    FlyingGet => SongData { id: 0xcc, title: "Flying Get", dlc: true },
    MisenaiNomidaWaKittoItsuKa => SongData { id: 0xd0, title: "Misenai Namida wa, Kitto Itsu Ka", dlc: true },
    Shine => SongData { id: 0xd1, title: "Shine", dlc: true },
    KissKissBangBang => SongData { id: 0xd9, title: "KISS KISS BANG BANG", dlc: true },
    Memeshikute => SongData { id: 0xdd, title: "Memeshikute", dlc: true },

    // == Variety ==
    Mister => SongData { id: 0x0d, title: "MISTER", dlc: false },
    JounetsuTairiku => SongData { id: 0x0e, title: "Jounetsu Tairiku", dlc: false },

    // DLC:
    Melt => SongData { id: 0x73, title: "Melt", dlc: true },
    BlackRockShooter => SongData { id: 0x89, title: "Black Rock Shooter", dlc: true },
    MaruMaruMoriMori => SongData { id: 0x8e, title: "Maru Maru Mori Mori", dlc: true },
    WorldIsMine => SongData { id: 0x8f, title: "World is Mine", dlc: true },
    GoGoYuureisen => SongData { id: 0xc2, title: "Go Go Yuureisen", dlc: true },
    RhythmAndPolice => SongData { id: 0xc3, title: "RHYTHM AND POLICE", dlc: true },
    Otemoyan => SongData { id: 0xcd, title: "Otemoyan", dlc: true },
    AtarimaeTaisou => SongData { id: 0xe4, title: "Atarimae Taisou", dlc: true },
    Matryoshka => SongData { id: 0xe5, title: "Matryoshka", dlc: true },
    JinseiResetButton => SongData { id: 0xe6, title: "Jinsei Reset Button", dlc: true },

    // == Classic ==
    PavaneForADeadPrincess => SongData { id: 0x0f, title: "Pavane for a Dead Princess ～Kimi no Kodou～", dlc: false },
    NinthSymphony => SongData { id: 0x10, title: "Ninth Symphony", dlc: false },
    ExcerptFromLightCavalryOverture => SongData { id: 0x11, title: "Excerpt from Light Cavalry Overture", dlc: false },
    EineKleineNachtmuzik => SongData { id: 0x12, title: "Eine Kleine Nachtmuzik", dlc: false },
    ViolinConcertoInEMinor => SongData { id: 0x13, title: "Violin Concerto in E Minor", dlc: false },
    RadetzkyMarch => SongData { id: 0x14, title: "Radetzky March", dlc: false },
    KareKantonoKanonan => SongData { id: 0x15, title: "Kare Kano Kanon", dlc: false },
    EtudeOp104 => SongData { id: 0x16, title: "Etude Op.10-4", dlc: false },
    EtudeOp104Ura => SongData { id: 0x17, title: "Etude Op.10-4 (Ura)", dlc: false },

    // == Game Music ==
    MonsterHunterMedley => SongData { id: 0x18, title: "Monster Hunter Medley", dlc: false },
    MachineGunKiss => SongData { id: 0x19, title: "MachineGun Kiss", dlc: false },
    TheWorldIsAllOne => SongData { id: 0x1a, title: "The world is all one!!", dlc: false },
    KirameKirari => SongData { id: 0x1b, title: "Kirame Kirari", dlc: false },
    MujihiNaOu => SongData { id: 0x1c, title: "Mujihi na Ou", dlc: false },
    Karma => SongData { id: 0x1d, title: "KARMA", dlc: false },
    Bambini => SongData { id: 0x1e, title: "Bambini", dlc: false },
    PaPaPaLove => SongData { id: 0x1f, title: "PaPaPa Love", dlc: false },
    MappyMedley => SongData { id: 0x20, title: "Mappy Medley", dlc: false },
    MagicalSoundShower => SongData { id: 0x21, title: "MAGICAL SOUND SHOWER", dlc: false },
    HatsuneMikuNoGekishou => SongData { id: 0x22, title: "Hatsune Miku no Gekishou", dlc: false },
    HatsuneMikuNoGekishouUra => SongData { id: 0x23, title: "Hatsune Miku no Gekishou (Ura)", dlc: false },

    // DLC:
    SmokyThrill => SongData { id: 0x66, title: "SMOKY THRILL", dlc: true },
    NoWayBack => SongData { id: 0x7d, title: "No Way Back", dlc: true },
    NoWayBackUra => SongData { id: 0x7e, title: "No Way Back (Ura)", dlc: true },
    NakedGlow => SongData { id: 0xa5, title: "Naked Glow", dlc: true },
    DoDai => SongData { id: 0xb8, title: "Do-Dai", dlc: true },
    DoomNoiz => SongData { id: 0xc4, title: "Doom Noiz", dlc: true },
    AgentYoruOYuku => SongData { id: 0xc5, title: "Agent Yoru o Yuku", dlc: true },
    Overmaster => SongData { id: 0xc6, title: "Overmaster", dlc: true },
    GoMyWay => SongData { id: 0xc7, title: "GO MY WAY !!", dlc: true },
    MetalHawkBGM1 => SongData { id: 0xd3, title: "Metal Hawk BGM1", dlc: true },
    RageVself => SongData { id: 0xd8, title: "RAGE v.self", dlc: true },

    // == Namco Original ==
    DokoDONMatsuRhythm => SongData { id: 0x24, title: "DokoDON MatsuRhythm", dlc: false },
    Canadea => SongData { id: 0x25, title: "Canadea", dlc: false },
    SORAVCosmicBird => SongData { id: 0x26, title: "SORA-V Cosmic Bird", dlc: false },
    WanyaWorld => SongData { id: 0x27, title: "Wanya World", dlc: false },
    AllNightdeIndenai => SongData { id: 0x28, title: "All Night de Indenai", dlc: false },
    MagicalLittleSpaceship => SongData { id: 0x29, title: "Magical Little Spaceship", dlc: false },
    ShiroNekoCaramelMugennoWataame => SongData { id: 0x2a, title: "Shiro Neko Caramel Mugen no Wata ame", dlc: false },
    WhiteRoseInsanity => SongData { id: 0x2b, title: "White Rose Insanity", dlc: false },
    WhiteRoseInsanityUra => SongData { id: 0x2c, title: "White Rose Insanity (Ura)", dlc: false },
    AoNoSenritsu => SongData { id: 0x2d, title: "Ao no Senritsu", dlc: false },
    InuHoeru => SongData { id: 0x2e, title: "Inu Hoeru", dlc: false },
    LOVEIkusa => SongData { id: 0x2f, title: "LOVE Ikusa!!", dlc: false },
    TaiyoumoYappappa => SongData { id: 0x30, title: "Taiyou mo Yappappa", dlc: false },
    TaiyoumoYappappaUra => SongData { id: 0x31, title: "Taiyou mo Yappappa (Ura)", dlc: false },
    SportsDigestion => SongData { id: 0x32, title: "Sports Digestion ", dlc: false },
    SportsDigestionUra => SongData { id: 0x33, title: "Sports Digestion (Ura)", dlc: false },
    HappyPeace => SongData { id: 0x34, title: "Happy & Peace", dlc: false },
    RaMorenaKumonai => SongData { id: 0x35, title: "Ra Morena Kumonai", dlc: false },
    DesertDeYakiniku => SongData { id: 0x36, title: "Desert de Yakiniku", dlc: false },
    RindawaKyoumoZekkouchou => SongData { id: 0x37, title: "Rinda wa Kyou mo Zekkouchou", dlc: false },
    RindawaKyoumoZekkouchouUra => SongData { id: 0x38, title: "Rinda wa Kyou mo Zekkouchou (Ura)", dlc: false },
    DensetsunoMatsuri => SongData { id: 0x39, title: "Densetsu no Matsuri", dlc: false },
    DontCut => SongData { id: 0x3a, title: "DON'T CUT", dlc: false },
    DontCutUra => SongData { id: 0x3b, title: "DON'T CUT (Ura)", dlc: false },
    DunAonghasanoFuefuki => SongData { id: 0x3c, title: "Dun Aonghasa no Fuefuki", dlc: false },
    RumbleRanbu => SongData { id: 0x3d, title: "Rumble Ranbu", dlc: false },
    RumbleRanbuUra => SongData { id: 0x3e, title: "Rumble Ranbu (Ura)", dlc: false },
    BuruChanNoOya2 => SongData { id: 0x3f, title: "Buru-chan no Oya 2", dlc: false },
    KissaRain => SongData { id: 0x40, title: "Kissa Rain", dlc: false },
    DodododoDonderful => SongData { id: 0x41, title: "Dodododo-Donderful!", dlc: false },
    AkuukanYuueiac125 => SongData { id: 0x42, title: "Akuukan Yuuei ac12.5", dlc: false },
    JigokunoTaikoJiten => SongData { id: 0x43, title: "Jigoku no Taiko Jiten", dlc: false },
    BeTheAce => SongData { id: 0x44, title: "BE THE ACE", dlc: false },
    Mulberry => SongData { id: 0x45, title: "Mulberry", dlc: false },
    SengokuSangen => SongData { id: 0x46, title: "Sengoku Sangen", dlc: false },
    HyakkiYakou => SongData { id: 0x47, title: "Hyakki Yakou", dlc: false },
    RotterTarmination => SongData { id: 0x48, title: "Rotter Tarmination", dlc: false },
    RotterTarminationUra => SongData { id: 0x49, title: "Rotter Tarmination (Ura)", dlc: false },
    BlackRoseApostle => SongData { id: 0x4a, title: "Black Rose Apostle", dlc: false },
    BlackRoseApostleUra => SongData { id: 0x4b, title: "Black Rose Apostle (Ura)", dlc: false },
    Soroban2000 => SongData { id: 0x4c, title: "Soroban 2000", dlc: false },
    XDay => SongData { id: 0x4d, title: "X-DAY 2000", dlc: false },
    MataSaitama2000 => SongData { id: 0x4e, title: "Mata Saitama 2000", dlc: false },
    CycleOfRebirth => SongData { id: 0x4f, title: "Cycle of Rebirth", dlc: false },

    // DLC:
    AngelDream => SongData { id: 0x67, title: "Angel Dream", dlc: true },
    KazeNoFantasy => SongData { id: 0x68, title: "Kaze no Fantasy", dlc: true },
    PastelDream => SongData { id: 0x69, title: "Pastel Dream", dlc: true },
    NagisanoAndromeda => SongData { id: 0x6a, title: "Nagisa no Andromeda", dlc: true },
    Saitama2000 => SongData { id: 0x6f, title: "Saitama 2000", dlc: true },
    TsukikageSasurai => SongData { id: 0x70, title: "Tsukikage SASURAI", dlc: true },
    HyakkaRyouran => SongData { id: 0x77, title: "Hyakka Ryouran", dlc: true },
    ZastohlNoMadousho => SongData { id: 0x78, title: "Zastohl no Madousho", dlc: true },
    MeenaNoOyashiki => SongData { id: 0x85, title: "Meena no Oyashiki", dlc: true },
    KitaSaitama2000 => SongData { id: 0x86, title: "Kita Saitama 2000", dlc: true },
    KitaSaitama200 => SongData { id: 0x87, title: "Kita Saitama 200", dlc: true },
    UchuuSamurai => SongData { id: 0x88, title: "Uchuu Samurai", dlc: true },
    DanbaDanbaDinDan => SongData { id: 0x8c, title: "Danba Danba Din Dan", dlc: true },
    KimiNoPlanet => SongData { id: 0x8d, title: "Kimi no Planet", dlc: true },
    Stage0Ac11 => SongData { id: 0x92, title: "STAGE 0. ac11", dlc: true },
    KuonNoYoru => SongData { id: 0x93, title: "Kuon no Yoru", dlc: true },
    TheCarnivorousCarnival => SongData { id: 0x99, title: "The Carnivorous Carnival", dlc: true },
    HatsuneMikuNoShoushitsu => SongData { id: 0x9e, title: "Hatsune Miku no Shoushitsu", dlc: true },
    KayouFlourishingBlossoms => SongData { id: 0x9f, title: "Kayou ~Flourishing Blossoms~", dlc: true },
    Germiniation => SongData { id: 0xa4, title: "GERMINATION", dlc: true },
    CraneCity => SongData { id: 0xa9, title: "Crane City", dlc: true },
    KurukuruKurokkuru => SongData { id: 0xaf, title: "Kurukuru Kurokkuru", dlc: true },
    FuunBachioSensei => SongData { id: 0xb0, title: "Fuun! Bachi o Sensei", dlc: true },
    FuunBachioSenseiLongVersion => SongData { id: 0xb1, title: "Fuun! Bachi o Sensei  Long Version", dlc: true },
    Jinpuumaru => SongData { id: 0xb5, title: "Jinpuumaru", dlc: true },
    YozakuraShanikusai => SongData { id: 0xb6, title: "Yozakura Shanikusai", dlc: true },
    YozakuraShanikusaiUra => SongData { id: 0xb7, title: "Yozakura Shanikusai (Ura)", dlc: true },
    MetalPolice => SongData { id: 0xbd, title: "Metal Police", dlc: true },
    MetalPoliceUra => SongData { id: 0xbe, title: "Metal Police (Ura)", dlc: true },
    Dimensions => SongData { id: 0xbf, title: "DIMENSIONS", dlc: true },
    DokidokiMuneKyunOmatsuriTime => SongData { id: 0xc0, title: "Dokidoki Mune Kyun Omatsuri Time ", dlc: true },
    Carnation => SongData { id: 0xc8, title: "Carnation", dlc: true },
    KoiNoShohousen => SongData { id: 0xc9, title: "Koi no Shohousen", dlc: true },
    HanaotoUraHyoushi => SongData { id: 0xce, title: "Hanaoto Ura Hyoushi", dlc: true },
    KazeNoKuniNoRyuToKishi => SongData { id: 0xcf, title: "Kaze no Kuni no Ryu to Kishi", dlc: true },
    TaikoTime => SongData { id: 0xd5, title: "Taiko Time", dlc: true },
    TaikoTimeUra => SongData { id: 0xd6, title: "Taiko Time (Ura)", dlc: true },
    ThreePieceJazzParty => SongData { id: 0xd7, title: "3piece-Jazz Party!", dlc: true },
    Junction => SongData { id: 0xdb, title: "junction", dlc: true },
    Shimedore2000Plus => SongData { id: 0xdc, title: "Shimedore 2000+", dlc: true },

    // == Wadaiko Enbu ==
    TaikoGuideChapter116 => SongData { id: 0x58, title: "Taiko Guide: Chapter 116", dlc: false },
}
