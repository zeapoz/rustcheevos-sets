/// [32-bit] Current Song ID
/// See [`crate::mem::current_song_id`]
///
/// `(Ura)` denotes the harder arrangement of a song.
#[repr(u32)]
#[allow(dead_code)]
pub enum SongId {
    // == Anime ==
    /// One day - One Piece
    OneDay = 0x00,
    /// Lovers - Naruto Shippuden
    Lovers = 0x01,
    /// Lion - Macross F
    Lion = 0x02,
    /// No buts! - Toaru Majutsu no Index
    NoButs = 0x03,

    /// Detective Conan Main Theme (DLC)
    DetectiveConanMainTheme = 0x6b,
    /// Guts Guts!! - Toriko (DLC)
    GutsGuts = 0x6c,
    /// My Soul, Your Beats! - Angel Beats (DLC)
    MySoulYourBeats = 0x71,
    /// Tougenkyou Alien - Gintama (DLC)
    TougenkyouAlien = 0x72,
    /// A Cruel Angel's Thesis - Neon Genesis Evangelion (DLC)
    ACruelAngelsThesis = 0x79,
    /// Tank! - Cowboy Bebop (DLC)
    Tank = 0x81,
    /// Tank! - Cowboy Bebop (Ura) (DLC)
    TankUra = 0x82,
    /// Sousei no Aquarion (DLC)
    SouseiNoAquarion = 0x83,
    /// READY!! - THE iDOLM@STER (DLC)
    Ready = 0x8a,
    /// Lupin III Theme '78 (DLC)
    LupinIIITheme78 = 0x90,
    /// Touch (DLC)
    Touch = 0x96,
    /// Hare Hare Yukai (DLC)
    HareHareYukai = 0x97,
    /// W-B-X ~W Boiled Extreme~ - Kamen Rider W (DLC)
    WBXWBoiledExtreme = 0x9a,
    /// We Are! - One Piece (DLC)
    WeAre = 0x9b,
    /// Yuuki 100% - Nintama Rantaro (DLC)
    Yuuki100Percent = 0xa0,
    /// CHA-LA HEAD-CHA-LA - Dragon Ball Z (DLC)
    ChalaHeadChala = 0xa1,
    /// SWITCH ON! - Kamen Rider Fourze (DLC)
    SwitchOn = 0xa8,
    /// Motteke! Sailor Fuku - Lucky Star (DLC)
    MottekeSailorFuku = 0xab,
    /// Oshiri Kajiri Mushi (DLC)
    OshiriKajiriMushi = 0xad,
    /// Tomare! - Melancholy of Haruhi Suzumiya (DLC)
    Tomare = 0xae,
    /// 1 Dream - Danbooru Senki (DLC)
    N1Dream = 0xb4,
    /// By My Side - Naruto Shippuden (DLC)
    ByMySide = 0xc1,
    /// We Go! - One Piece (DLC)
    WeGo = 0xca,
    /// Orion o Nazoru - Tiger & Bunny (DLC)
    OrionONazoru = 0xcb,
    /// Kill me no Baby! (DLC)
    KillMeNoBaby = 0xd2,
    /// Pegasus Fantasy - Saint Seiya (DLC)
    PegasusFantasy = 0xd4,
    /// Life is SHOW TIME - Kamen Rider Wizard (DLC)
    LifeIsShowTime = 0xda,
    /// Kimi o Nosete - Laputa: Castle in the Sky (DLC)
    KimiONosete = 0xe3,

    // == J-Pop ==
    /// Heavy Rotation - AKB48
    HeavyRotation = 0x04,
    /// Yoku Asobi Yoku Manabe - NYC
    YokuAsobiYokuManabe = 0x05,
    /// I Wish For You - EXILE
    IWishForYou = 0x06,
    /// Arigatou - Ikimono Gakari
    Arigatou = 0x07,
    /// Joyful - Ikimono Gakari
    Joyful = 0x08,
    /// Natsu Matsuri - Whiteberry
    NatsuMatsuri = 0x09,
    /// Tentai Kansoku - Bump of Chicken
    TentaiKansoku = 0x0a,
    /// Kurenai - X-Japan
    Kurenai = 0x0b,
    /// Punishment - 9mm Parabellum Bullet
    Punishment = 0x0c,

    /// Ponytail to Shushu - AKB48 (DLC)
    PonytailToShushu = 0x65,
    /// Ring a Ding Dong - Kaela Kimura (DLC)
    RingADingDong = 0x6d,
    /// Kiseki - GReeeeN (DLC)
    Kiseki = 0x6e,
    /// Polyrhythm - Perfume (DLC)
    Polyrhythm = 0x74,
    /// Everyday, Katyusha - AKB48 (DLC)
    EverydayKatyusha = 0x75,
    /// LISTEN TO THE STEREO!! - Going Under Ground (DLC)
    ListenToTheStereo = 0x76,
    /// Sakuranbo - Ai Ootsuka (DLC)
    Sakuranbo = 0x7b,
    /// Karon - Negoto (DLC)
    Karon = 0x7c,
    /// Sobakasu - Judy and Mary (DLC)
    Sobakasu = 0x7f,
    /// Egao ni Kanpai! - Hiromi Go (DLC)
    EgaoNiKanpai = 0x80,
    /// LOVE Zukkyun - Sotasei Riron (DLC)
    LoveZukkyun = 0x84,
    /// Blue Bird - Ikimono Gakari (DLC)
    BlueBird = 0x8b,
    /// Kibun Joujou - mihimaru GT (DLC)
    KibunJoujou = 0x95,
    /// Bakuchi Dancer - DOES (DLC)
    BakuchiDancer = 0x9c,
    /// Linda Linda - The Blue Hearts (DLC)
    LindaLinda = 0x9d,
    /// Ikenai Taiyou - Orange Range (DLC)
    IkenaiTaiyou = 0xa2,
    /// Niji - Aqua Timez (DLC)
    Niji = 0xa3,
    /// Ai wa Katsu - KAN (DLC)
    AiWaKatsu = 0xa6,
    /// 365 Nichi no Love Story - Sonar Pocket (DLC)
    N365NichiNoLoveStory = 0xa7,
    /// TRAIN-TRAIN - The Blue Hearts (DLC)
    TrainTrain = 0xaa,
    /// Aruite Ikou - Ikimono Gakari (DLC)
    AruiteIkou = 0xac,
    /// Tsubomi - Kobukuro (DLC)
    Tsubomi = 0xb2,
    /// Kimagure Romantic - Ikimono Gakari (DLC)
    KimagureRomantic = 0xb3,
    /// Flying Get - AKB48 (DLC)
    FlyingGet = 0xcc,
    /// Misenai Namida wa, Kitto Itsu Ka - GReeeeN (DLC)
    MisenaiNamidaWaKittoItsuKa = 0xd0,
    /// Shine - Ieiri Leo (DLC)
    Shine = 0xd1,
    /// KISS KISS BANG BANG - Ikimono Gakari (DLC)
    KissKissBangBang = 0xd9,
    /// Memeshikute - Golden Bomber (DLC)
    Memeshikute = 0xdd,

    // == Variety ==
    /// MISTER - KARA
    Mister = 0x0d,
    /// Jounetsu Tairiku - Hakase-tarou
    JounetsuTairiku = 0x0e,

    /// Melt (DLC)
    Melt = 0x73,
    /// Black Rock Shooter (DLC)
    BlackRockShooter = 0x89,
    /// Maru Maru Mori Mori (DLC)
    MaruMaruMoriMori = 0x8e,
    /// World is Mine (DLC)
    WorldIsMine = 0x8f,
    /// Go Go Yuureisen (DLC)
    GoGoYuureisen = 0xc2,
    /// RHYTHM AND POLICE (DLC)
    RhythmAndPolice = 0xc3,
    /// Otemoyan (DLC)
    Otemoyan = 0xcd,
    /// Atarimae Taisou (DLC)
    AtarimaeTaisou = 0xe4,
    /// Matryoshka (DLC)
    Matryoshka = 0xe5,
    /// Jinsei Reset Button (DLC)
    JinseiResetButton = 0xe6,

    // == Classic ==
    /// Pavane for a Dead Princess ~Kimi no Kodou~ - Ravel
    PavaneForADeadPrincess = 0x0f,
    /// Ninth Symphony - Beethoven
    NinthSymphony = 0x10,
    /// Excerpt from Light Cavalry Overture - Franz von Suppe
    ExcerptFromLightCavalryOverture = 0x11,
    /// Eine Kleine Nachtmuzik - Mozart
    EineKleineNachtmuzik = 0x12,
    /// Violin Concerto in E Minor - Mendelssohn
    ViolinConcertoInEMinor = 0x13,
    /// Radetzky March - Strauss
    RadetzkyMarch = 0x14,
    /// Kare Kano Kanon
    KareKanoKanon = 0x15,
    /// Etude Op.10-4 - Chopin
    EtudeOp10_4 = 0x16,
    /// Etude Op.10-4 - Chopin (Ura)
    EtudeOp10_4Ura = 0x17,

    // == Game Music ==
    /// Monster Hunter Medley
    MonsterHunterMedley = 0x18,
    /// MachineGun Kiss - Yakuza OF THE END
    MachineGunKiss = 0x19,
    /// The world is all one!! - THE iDOLM@STER 2
    TheWorldIsAllOne = 0x1a,
    /// Kirame Kirari - THE iDOLM@STER
    KirameKirari = 0x1b,
    /// Mujihi na Ou - GOD EATER BURST
    MujihiNaOu = 0x1c,
    /// KARMA (Tatsujin Mix) - Tekken 6- Bloodline Rebellion
    KarmaTatsujinMix = 0x1d,
    /// Bambini - Kotoba no Puzzle: Mojipittan
    Bambini = 0x1e,
    /// PaPaPa Love
    PaPaPaLove = 0x1f,
    /// Mappy Medley
    MappyMedley = 0x20,
    /// MAGICAL SOUND SHOWER - OutRun
    MagicalSoundShower = 0x21,
    /// Hatsune Miku no Gekishou - Hatsune Miku Project DIVA - 2nd
    HatsuneMikuNoGekishou = 0x22,
    /// Hatsune Miku no Gekishou - Hatsune Miku Project DIVA - 2nd (Ura)
    HatsuneMikuNoGekishouUra = 0x23,

    /// SMOKY THRILL - THE iDOLM@STER 2 (DLC)
    SmokyThrill = 0x66,
    /// No Way Back - God Eater (DLC)
    NoWayBack = 0x7d,
    /// No Way Back - God Eater (Ura) (DLC)
    NoWayBackUra = 0x7e,
    /// Naked Glow - R4: Ridge Racer Type 4 (DLC)
    NakedGlow = 0xa5,
    /// Do-Dai - THE iDOLM@STER (DLC)
    DoDai = 0xb8,
    /// Doom Noiz - Galaga Legions (DLC)
    DoomNoiz = 0xc4,
    /// Agent Yoru o Yuku - THE iDOLM@STER (DLC)
    AgentYoruOYuku = 0xc5,
    /// Overmaster - THE iDOLM@STER 2 (DLC)
    Overmaster = 0xc6,
    /// GO MY WAY !! - THE iDOLM@STER 2 (DLC)
    GoMyWay = 0xc7,
    /// Metal Hawk BGM1 (DLC)
    MetalHawkBgm1 = 0xd3,
    /// RAGE v.self - Rage Racer (DLC)
    RageVSelf = 0xd8,

    // == Namco Original ==
    /// DokoDON MatsuRhythm
    DokoDonMatsuRhythm = 0x24,
    /// Canadea
    Canadea = 0x25,
    /// SORA-V Cosmic Bird
    SoraVCosmicBird = 0x26,
    /// Wanya World
    WanyaWorld = 0x27,
    /// All Night de Indenai
    AllNightDeIndenai = 0x28,
    /// Magical Little Spaceship
    MagicalLittleSpaceship = 0x29,
    /// Shiro Neko Caramel Mugen no Wata ame
    ShiroNekoCaramelMugenNoWataAme = 0x2a,
    /// White Rose Insanity
    WhiteRoseInsanity = 0x2b,
    /// White Rose Insanity (Ura)
    WhiteRoseInsanityUra = 0x2c,
    /// Ao no Senritsu
    AoNoSenritsu = 0x2d,
    /// Inu Hoeru
    InuHoeru = 0x2e,
    /// LOVE Ikusa!!
    LoveIkusa = 0x2f,
    /// Taiyou mo Yappappa
    TaiyouMoYappappa = 0x30,
    /// Taiyou mo Yappappa (Ura)
    TaiyouMoYappappaUra = 0x31,
    /// Sports Digestdon - ~Fill In The Sky~
    SportsDigestdon = 0x32,
    /// Sports Digestdon - ~Fill In The Sky~ (Ura)
    SportsDigestdonUra = 0x33,
    /// Happy & Peace
    HappyAndPeace = 0x34,
    /// Ra Morena Kumonai
    RaMorenaKumonai = 0x35,
    /// Desert de Yakiniku (Sahara-hen)
    DesertDeYakinikuSaharaHen = 0x36,
    /// Rinda wa Kyou mo Zekkouchou
    RindaWaKyouMoZekkouchou = 0x37,
    /// Rinda wa Kyou mo Zekkouchou (Ura)
    RindaWaKyouMoZekkouchouUra = 0x38,
    /// Densetsu no Matsuri
    DensetsuNoMatsuri = 0x39,
    /// DON'T CUT
    DontCut = 0x3a,
    /// DON'T CUT (Ura)
    DontCutUra = 0x3b,
    /// Dun Aonghasa no Fuefuki
    DunAonghasaNoFuefuki = 0x3c,
    /// Rumble Ranbu
    RumbleRanbu = 0x3d,
    /// Rumble Ranbu (Ura)
    RumbleRanbuUra = 0x3e,
    /// Buru-chan no Oya 2
    BuruChanNoOya2 = 0x3f,
    /// Kissa Rain
    KissaRain = 0x40,
    /// Dodododo-Donderful!
    DodododoDonderful = 0x41,
    /// Akuukan Yuuei ac12.5
    AkuukanYuueiAc12_5 = 0x42,
    /// Jigoku no Taiko Jiten
    JigokuNoTaikoJiten = 0x43,
    /// BE THE ACE
    BeTheAce = 0x44,
    /// Mulberry
    Mulberry = 0x45,
    /// Sengoku Sangen
    SengokuSangen = 0x46,
    /// Hyakki Yakou
    HyakkiYakou = 0x47,
    /// Rotter Tarmination
    RotterTarmination = 0x48,
    /// Rotter Tarmination (Ura)
    RotterTarminationUra = 0x49,
    /// Black Rose Apostle
    BlackRoseApostle = 0x4a,
    /// Black Rose Apostle (Ura)
    BlackRoseApostleUra = 0x4b,
    /// Soroban 2000
    Soroban2000 = 0x4c,
    /// X-DAY 2000
    XDay2000 = 0x4d,
    /// Mata Saitama 2000
    MataSaitama2000 = 0x4e,
    /// Cycle of Rebirth
    CycleOfRebirth = 0x4f,

    /// Angel Dream (DLC)
    AngelDream = 0x67,
    /// Kaze no Fantasy (DLC)
    KazeNoFantasy = 0x68,
    /// Pastel Dream (DLC)
    PastelDream = 0x69,
    /// Nagisa no Andromeda (DLC)
    NagisaNoAndromeda = 0x6a,
    /// Saitama 2000 (DLC)
    Saitama2000 = 0x6f,
    /// Tsukikage SASURAI (DLC)
    TsukikageSasurai = 0x70,
    /// Hyakka Ryouran (DLC)
    HyakkaRyouran = 0x77,
    /// Zastohl no Madousho (DLC)
    ZastohlNoMadousho = 0x78,
    /// Meena no Oyashiki (DLC)
    MeenaNoOyashiki = 0x85,
    /// Kita Saitama 2000 (DLC)
    KitaSaitama2000 = 0x86,
    /// Kita Saitama 200 (DLC)
    KitaSaitama200 = 0x87,
    /// Uchuu Samurai (DLC)
    UchuuSamurai = 0x88,
    /// Danba Danba Din Dan (DLC)
    DanbaDanbaDinDan = 0x8c,
    /// Kimi no Planet (DLC)
    KimiNoPlanet = 0x8d,
    /// STAGE 0. ac11 (DLC)
    Stage0Ac11 = 0x92,
    /// Kuon no Yoru (DLC)
    KuonNoYoru = 0x93,
    /// The Carnivorous Carnival (DLC)
    TheCarnivorousCarnival = 0x99,
    /// Hatsune Miku no Shoushitsu -Gekijouban- (DLC)
    HatsuneMikuNoShoushitsuGekijouban = 0x9e,
    /// Kayou ~Flourishing Blossoms~ (DLC)
    KayouFlourishingBlossoms = 0x9f,
    /// GERMINATION (DLC)
    Germination = 0xa4,
    /// Crane City (DLC)
    CraneCity = 0xa9,
    /// Kurukuru Kurokkuru (DLC)
    KurukuruKurokkuru = 0xaf,
    /// Fuun! Bachi o Sensei (DLC)
    FuunBachiOSensei = 0xb0,
    /// Fuun! Bachi o Sensei Long Version (DLC)
    FuunBachiOSenseiLongVersion = 0xb1,
    /// Jinpuumaru (DLC)
    Jinpuumaru = 0xb5,
    /// Yozakura Shanikusai (DLC)
    YozakuraShanikusai = 0xb6,
    /// Yozakura Shanikusai (Ura) (DLC)
    YozakuraShanikusaiUra = 0xb7,
    /// Metal Police (DLC)
    MetalPolice = 0xbd,
    /// Metal Police (Ura) (DLC)
    MetalPoliceUra = 0xbe,
    /// DIMENSIONS (DLC)
    Dimensions = 0xbf,
    /// Dokidoki Mune Kyun Omatsuri Time (DLC)
    DokidokiMuneKyunOmatsuriTime = 0xc0,
    /// Carnation (DLC)
    Carnation = 0xc8,
    /// Koi no Shohousen (DLC)
    KoiNoShohousen = 0xc9,
    /// Hanaoto Ura Hyoushi (DLC)
    HanaotoUraHyoushi = 0xce,
    /// Kaze no Kuni no Ryu to Kishi (DLC)
    KazeNoKuniNoRyuToKishi = 0xcf,
    /// Taiko Time (DLC)
    TaikoTime = 0xd5,
    /// Taiko Time (Ura) (DLC)
    TaikoTimeUra = 0xd6,
    /// 3piece-Jazz Party! (DLC)
    N3PieceJazzParty = 0xd7,
    /// junction (DLC)
    Junction = 0xdb,
    /// Shimedore 2000+ (DLC)
    Shimedore2000Plus = 0xdc,

    // == Wadaiko Enbu ==
    /// Taiko Guide: Chapter 116
    TaikoGuideChapter116 = 0x58,
}

impl SongId {
    /// Returns the raw value stored in [`crate::mem::current_song_id`].
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}
