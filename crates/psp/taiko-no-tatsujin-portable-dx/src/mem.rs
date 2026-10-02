use rustcheevos::types::memory::MemoryRef;
use rustcheevos::{bits8, bits16, bits32};

/// [32-bit] Currently Selected Profile
pub const fn currently_selected_profile() -> MemoryRef {
    bits32!(0xb48f0c)
}

/// [32-bit][Pointer] In-Game Structs (& 0x1ffffff)
/// Only non-zero while playing a song
/// +0xc = [32-bit][Pointer]
/// ++0x4 = [32-bit][Pointer]
/// +++0x4 = [32-bit][Pointer]
/// ++++0x4 = [32-bit] Current Combo
/// ++++0x8 = [32-bit] Highest Combo
/// ++++0xc = [32-bit] Great Hits
/// ++++0x10 = [32-bit] Good Hits
/// ++++0x14 = [32-bit] Misses
/// ++++0x18 = [32-bit] Total Hits
/// ++++0x1c = [32-bit] Current Score
/// ++++0x20 = [32-bit] Quota Gauge
pub const fn in_game_structs_0x1ffffff() -> MemoryRef {
    bits32!(0xb4b2b0)
}

/// [32-bit] In Game Flag
/// 0x0 = In Menu
/// 0x1 = In Game
pub const fn in_game_flag() -> MemoryRef {
    bits32!(0xc13ac4)
}

/// [8-bit] Save Data Loaded Flag
/// 0x0 = Not Loaded
/// 0x1 = Loaded
pub const fn save_data_loaded_flag() -> MemoryRef {
    bits8!(0xcb266c)
}

/// [32-bit] Results - Score
pub const fn results_score() -> MemoryRef {
    bits32!(0xfca010)
}

/// [16-bit] Results - % Hit
pub const fn results_hit() -> MemoryRef {
    bits16!(0xfca014)
}

/// [16-bit] Results - Great Hits
pub const fn results_great_hits() -> MemoryRef {
    bits16!(0xfca016)
}

/// [16-bit] Results - Good Hits
pub const fn results_good_hits() -> MemoryRef {
    bits16!(0xfca018)
}

/// [16-bit] Results - Missed Hits
pub const fn results_missed_hits() -> MemoryRef {
    bits16!(0xfca01a)
}

/// [32-bit] End Result
/// 0x0 = Fail
/// 0x1 = Silver Crown
/// 0x2 = Gold Crown
pub const fn end_result() -> MemoryRef {
    bits32!(0xfca024)
}

/// [32-bit] Current Song ID
/// == Anime ==
/// 0x00 = One day - One Piece
/// 0x01 = Lovers (ラヴァーズ) - Naruto Shippuden
/// 0x02 = Lion (ライオン) - Macross F
/// 0x03 = No buts! - Toaru Majutsu no Index
///
/// DLC:
/// 0x6b = Detective Conan Main Theme (名探偵コナン メイン･テーマ)
/// 0x6c = Guts Guts!! (ガツガツ！！) - Toriko
/// 0x71 = My Soul, Your Beats! - Angel Beats
/// 0x72 = Tougenkyou Alien (桃源郷エイリアン) - Gintama
/// 0x79 = A Cruel Angel's Thesis (残酷な天使のテーゼ) - Neon Genesis Evangelion
/// 0x81 = Tank! - Cowboy Bebop
/// 0x82 = Tank! - Cowboy Bebop (Ura)
/// 0x83 = Sousei no Aquarion (創聖のアクエリオン)
/// 0x8a = READY!! - THE iDOLM@STER
/// 0x90 = Lupin III Theme '78 (ルパン三世のテーマ'78)
/// 0x96 = Touch (タッチ)
/// 0x97 = Hare Hare Yukai (ハレ晴レユカイ)
/// 0x9a = W-B-X ~W Boiled Extreme~ - Kamen Rider W
/// 0x9b = We Are! (ウィーアー!) - One Piece
/// 0xa0 = Yuuki 100% (勇気100％) - Nintama Rantaro
/// 0xa1 = CHA-LA HEAD-CHA-LA - Dragon Ball Z
/// 0xa8 = SWITCH ON! - Kamen Rider Fourze
/// 0xab = Motteke! Sailor Fuku (もってけ！セーラーふく) - Lucky Star
/// 0xad = Oshiri Kajiri Mushi (おしりかじり虫)
/// 0xae = Tomare! (止マレ！) - Melancholy of Haruhi Suzumiya
/// 0xb4 = 1 Dream (１ ドリーム) - Danbooru Senki
/// 0xc1 = By My Side (バイマイサイド) - Naruto Shippuden
/// 0xca = We Go! (ウィーゴー！) - One Piece
/// 0xcb = Orion o Nazoru (オリオンをなぞる) - Tiger & Bunny
/// 0xd2 = Kill me no Baby! (キルミーのベイベー！)
/// 0xd4 = Pegasus Fantasy (ペガサス幻想) - Saint Seiya
/// 0xda = Life is SHOW TIME - Kamen Rider Wizard
/// 0xe3 = Kimi o Nosete (君をのせて) - Laputa: Castle in the Sky
///
/// == J-Pop ==
/// 0x04 = Heavy Rotation (ヘビーローテーション) - AKB48
/// 0x05 = Yoku Asobi Yoku Manabe (よく遊びよく学べ) - NYC
/// 0x06 = I Wish For You - EXILE
/// 0x07 = Arigatou (ありがとう) - Ikimono Gakari
/// 0x08 = Joyful (じょいふる) - Ikimono Gakari
/// 0x09 = Natsu Matsuri (夏祭り) - Whiteberry
/// 0x0a = Tentai Kansoku (天体観測) - Bump of Chicken
/// 0x0b = Kurenai (紅) - X-Japan
/// 0x0c = Punishment - 9mm Parabellum Bullet
///
/// DLC:
/// 0x65 = Ponytail to Shushu (ポニーテールとシュシュ) - AKB48
/// 0x6d = Ring a Ding Dong - Kaela Kimura
/// 0x6e = Kiseki (キセキ) - GReeeeN
/// 0x74 = Polyrhythm (ポリリズム) - Perfume
/// 0x75 = Everyday, Katyusha (EVERYDAY, カチューシャ) - AKB48
/// 0x76 = LISTEN TO THE STEREO!! - Going Under Ground
/// 0x7b = Sakuranbo (さくらんぼ) - Ai Ootsuka
/// 0x7c = Karon (カロン) - Negoto
/// 0x7f = Sobakasu (そばかす) - Judy and Mary
/// 0x80 = Egao ni Kanpai! (笑顔にカンパイ！) - Hiromi Go
/// 0x84 = LOVE Zukkyun (LOVEずっきゅん) - Sotasei Riron
/// 0x8b = Blue Bird (ブルーバード) - Ikimono Gakari
/// 0x95 = Kibun Joujou (気分上々↑↑) - mihimaru GT
/// 0x9c = Bakuchi Dancer (バクチ・ダンサー) - DOES
/// 0x9d = Linda Linda (リンダリンダ) - The Blue Hearts
/// 0xa2 = Ikenai Taiyou (イケナイ太陽) - Orange Range
/// 0xa3 = Niji (虹) - Aqua Timez
/// 0xa6 = Ai wa Katsu (愛は勝つ) - KAN
/// 0xa7 = 365 Nichi no Love Story (365日のラブストーリー。) - Sonar Pocket
/// 0xaa = TRAIN-TRAIN - The Blue Hearts
/// 0xac = Aruite Ikou (歩いていこう) - Ikimono Gakari
/// 0xb2 = Tsubomi (蕾) - Kobukuro
/// 0xb3 = Kimagure Romantic (気まぐれロマンティック) - Ikimono Gakari
/// 0xcc = Flying Get (フライングゲット) - AKB48
/// 0xd0 = Misenai Namida wa, Kitto Itsu Ka (ミセナイナミダハ、きっといつか) - GReeeeN
/// 0xd1 = Shine - Ieiri Leo
/// 0xd9 = KISS KISS BANG BANG - Ikimono Gakari
/// 0xdd = Memeshikute (女々しくて) - Golden Bomber
///
/// == Variety ==
/// 0x0d = MISTER - KARA
/// 0x0e = Jounetsu Tairiku (情熱大陸) - Hakase-tarou
///
/// DLC:
/// 0x73 = Melt (メルト) - feat. Hatsune Miku
/// 0x89 = Black Rock Shooter (ブラック★ロックシューター) - feat. Hatsune Miku
/// 0x8e = Maru Maru Mori Mori (マル・マル・モリ・モリ) - Marumo no Okite theme song
/// 0x8f = World is Mine (ワールドイズマイン) - feat. Hatsune Miku
/// 0xc2 = Go Go Yuureisen (ゴーゴー幽霊船)
/// 0xc3 = RHYTHM AND POLICE - Odoro Dai Sousa Sen (踊る大捜査線)
/// 0xcd = Otemoyan (おてもやん) - Omodaka feat. Paradise Yamamoto
/// 0xe4 = Atarimae Taisou (あたりまえ体操)
/// 0xe5 = Matryoshka (マトリョシカ) - Hachi feat. Hatsune Miku and Gumi
/// 0xe6 = Jinsei Reset Button (人生リセットボタン) - kemu feat. Gumi
///
/// == Classic ==
/// 0x0f = Pavane for a Dead Princess ～Kimi no Kodou～ (亡き王女のためのパヴァーヌ ～きみのこどう～) - Ravel
/// 0x10 = Ninth Symphony (第九交響曲) - Beethoven
/// 0x11 = Excerpt from Light Cavalry Overture (「軽騎兵」序曲から) - Franz von Suppe
/// 0x12 = Eine Kleine Nachtmuzik (アイネクライネナハトムジーク) - Mozart
/// 0x13 = Violin Concerto in E Minor (バイオリン協奏曲ホ短調) - Mendelssohn
/// 0x14 = Radetzky March (ラデツキー行進曲) - Strauss
/// 0x15 = Kare Kano Kanon (カレ・カノ・カノン)
/// 0x16 = Etude Op.10-4 (練習曲Op.10-4) - Chopin
/// 0x17 = Etude Op.10-4 (練習曲Op.10-4) - Chopin (Ura)
///
/// == Game Music ==
/// 0x18 = Monster Hunter Medley (モンスターハンターメドレー)
/// 0x19 = MachineGun Kiss - Yakuza OF THE END
/// 0x1a = The world is all one!! - THE iDOLM@STER 2
/// 0x1b = Kirame Kirari (キラメキラリ) - THE iDOLM@STER
/// 0x1c = Mujihi na Ou (無慈悲な王) - GOD EATER BURST
/// 0x1d = KARMA (Tatsujin Mix) - Tekken 6- Bloodline Rebellion
/// 0x1e = Bambini (バンビーニ) - Kotoba no Puzzle: Mojipittan
/// 0x1f = PaPaPa Love
/// 0x20 = Mappy Medley (マッピーメドレー)
/// 0x21 = MAGICAL SOUND SHOWER - OutRun
/// 0x22 = Hatsune Miku no Gekishou (初音ミクの激唱) - Hatsune Miku Project DIVA - 2nd
/// 0x23 = Hatsune Miku no Gekishou (初音ミクの激唱) - Hatsune Miku Project DIVA - 2nd (Ura)
///
/// DLC:
/// 0x66 = SMOKY THRILL - THE iDOLM@STER 2
/// 0x7d = No Way Back - God Eater
/// 0x7e = No Way Back - God Eater (Ura)
/// 0xa5 = Naked Glow - R4: Ridge Racer Type 4
/// 0xb8 = Do-Dai - THE iDOLM@STER
/// 0xc4 = Doom Noiz - Galaga Legions
/// 0xc5 = Agent Yoru o Yuku (エージェント夜を往く) - THE iDOLM@STER
/// 0xc6 = Overmaster (オーバーマスター) - THE iDOLM@STER 2
/// 0xc7 = GO MY WAY !! - THE iDOLM@STER 2
/// 0xd3 = Metal Hawk BGM1 (メタルホーク ＢＧＭ１)
/// 0xd8 = RAGE v.self - Rage Racer
///
/// == Namco Original ==
/// 0x24 = DokoDON MatsuRhythm (ドコＤＯＮ☆まつリズム)
/// 0x25 = Canadea (カナデア)
/// 0x26 = SORA-V Cosmic Bird (SORA-V コズミックバード)
/// 0x27 = Wanya World (わんにゃーワールド)
/// 0x28 = All Night de Indenai (オールナイト de インデナイ)
/// 0x29 = Magical Little Spaceship (マジカル・リトル・スペースシップ)
/// 0x2a = Shiro Neko Caramel Mugen no Wata ame (白猫きゃらめる夢幻のわたあめ)
/// 0x2b = White Rose Insanity
/// 0x2c = White Rose Insanity (Ura)
/// 0x2d = Ao no Senritsu (蒼の旋律)
/// 0x2e = Inu Hoeru (犬吠える)
/// 0x2f = LOVE Ikusa!! (LOVE戦!!)
/// 0x30 = Taiyou mo Yappappa (太陽もヤッパッパー)
/// 0x31 = Taiyou mo Yappappa (太陽もヤッパッパー) (Ura)
/// 0x32 = Sports Digestdon  (スポーツダイジェスドン) - ~Fill In The Sky~
/// 0x33 = Sports Digestdon  (スポーツダイジェスドン) - ~Fill In The Sky~ (Ura)
/// 0x34 = Happy & Peace
/// 0x35 = Ra Morena Kumonai (ラ・モレーナ・クモナイ)
/// 0x36 = Desert de Yakiniku (Sahara-hen) (デザートｄｅ焼肉（サハラ編）)
/// 0x37 = Rinda wa Kyou mo Zekkouchou (リンダは今日も絶好調)
/// 0x38 = Rinda wa Kyou mo Zekkouchou (リンダは今日も絶好調) (Ura)
/// 0x39 = Densetsu no Matsuri (伝説の祭り)
/// 0x3a = DON'T CUT
/// 0x3b = DON'T CUT (Ura)
/// 0x3c = Dun Aonghasa no Fuefuki (ドン・エンガスの笛吹き)
/// 0x3d = Rumble Ranbu (らんぶる乱舞)
/// 0x3e = Rumble Ranbu (らんぶる乱舞) (Ura)
/// 0x3f = Buru-chan no Oya 2 (ブルちゃんのおや2)
/// 0x40 = Kissa Rain (喫茶レイン)
/// 0x41 = Dodododo-Donderful! (ドドドドドンだフル！)
/// 0x42 = Akuukan Yuuei ac12.5 (亜空間遊泳ac12.5)
/// 0x43 = Jigoku no Taiko Jiten (地獄の太鼓事典)
/// 0x44 = BE THE ACE
/// 0x45 = Mulberry
/// 0x46 = Sengoku Sangen (戦国三弦)
/// 0x47 = Hyakki Yakou (百鬼夜行)
/// 0x48 = Rotter Tarmination
/// 0x49 = Rotter Tarmination (Ura)
/// 0x4a = Black Rose Apostle
/// 0x4b = Black Rose Apostle (Ura)
/// 0x4c = Soroban 2000 (十露盤2000)
/// 0x4d = X-DAY 2000
/// 0x4e = Mata Saitama 2000 (またさいたま2000)
/// 0x4f = Cycle of Rebirth (サイクル・オブ・リバース)
///
/// DLC:
/// 0x67 = Angel Dream (エンジェル　ドリーム)
/// 0x68 = Kaze no Fantasy (風のファンタジー)
/// 0x69 = Pastel Dream (パステル　ドリーム)
/// 0x6a = Nagisa no Andromeda (渚のアンドロメダ)
/// 0x6f = Saitama 2000 (さいたま2000)
/// 0x70 = Tsukikage SASURAI (月影SASURAI)
/// 0x77 = Hyakka Ryouran (百花繚乱)
/// 0x78 = Zastohl no Madousho (ザストゥールの魔導書)
/// 0x85 = Meena no Oyashiki (ミーナのおやしき)
/// 0x86 = Kita Saitama 2000 (きたさいたま2000)
/// 0x87 = Kita Saitama 200 (きたさいたま200)
/// 0x88 = Uchuu Samurai (宇宙SAMURAI)
/// 0x8c = Danba Danba Din Dan (ダンバ・ダンバ・ディン・ダン) - YMCK
/// 0x8d = Kimi no Planet (君のプラネット)
/// 0x92 = STAGE 0. ac11
/// 0x93 = Kuon no Yoru (久遠の夜)
/// 0x99 = The Carnivorous Carnival
/// 0x9e = Hatsune Miku no Shoushitsu -Gekijouban- (初音ミクの消失-劇場版-)
/// 0x9f = Kayou ~Flourishing Blossoms~ (花漾 ~Flourishing Blossoms~)
/// 0xa4 = GERMINATION
/// 0xa9 = Crane City (クレーンシティ)
/// 0xaf = Kurukuru Kurokkuru (クルクルクロックル)
/// 0xb0 = Fuun! Bachi o Sensei (風雲！バチお先生)
/// 0xb1 = Fuun! Bachi o Sensei  Long Version (風雲！バチお先生 ロング　バーション)
/// 0xb5 = Jinpuumaru (迅風丸)
/// 0xb6 = Yozakura Shanikusai (夜桜謝肉祭)
/// 0xb7 = Yozakura Shanikusai (夜桜謝肉祭) (Ura)
/// 0xbd = Metal Police (めたるぽりす)
/// 0xbe = Metal Police (めたるぽりす) (Ura)
/// 0xbf = DIMENSIONS
/// 0xc0 = Dokidoki Mune Kyun Omatsuri Time  (ドキドキ胸きゅん おまつりタイム)
/// 0xc8 = Carnation (和蘭撫子)
/// 0xc9 = Koi no Shohousen (恋の処方箋)
/// 0xce = Hanaoto Ura Hyoushi (花オト裏拍子)
/// 0xcf = Kaze no Kuni no Ryu to Kishi (風の国の龍と騎士)
/// 0xd5 = Taiko Time (タイコタイム)
/// 0xd6 = Taiko Time (タイコタイム) (Ura)
/// 0xd7 = 3piece-Jazz Party!
/// 0xdb = junction
/// 0xdc = Shimedore 2000+ (〆ドレー２０００＋)
///
/// == Wadaiko Enbu ==
/// 0x58 = Taiko Guide: Chapter 116 (太鼓指南　百十六の巻)
pub const fn current_song_id() -> MemoryRef {
    bits32!(0xfca080)
}

/// [32-bit] Current Song Difficulty
/// 0x0 = Kantan (Easy)
/// 0x1 = Futsuu (Normal)
/// 0x2 = Muzukashii (Hard)
/// 0x3 = Oni (Demon)
pub const fn current_song_difficulty() -> MemoryRef {
    bits32!(0xfca084)
}

/// [30x1 bytes] File 1 - Headgear Unlock Flags
/// 0x0 = Locked
/// 0x1 = Unlocked
///
/// |0x00 = Girl Hair (たてまきがみ)
/// |0x01 = Mohawk Haircut (モヒカンヘッド)
/// |0x02 = Afro Hair (アフロヘッド)
/// |0x03 = Regent Hairstyle (男前リーゼント)
/// |0x04 = Topknot (ちょんまげ)
/// |0x05 = Princess (おひめさま)
/// |0x06 = U-chan (うーちゃん)
/// |0x07 = Kappa Head (かっぱのおさら)
/// |0x08 = Empty Can (空き缶)
/// |0x09 = Big Eye (大きな目)
/// |0x0a = Drill (ドリル)
/// |0x0b = Ghost Mask (おばけのおめん)
/// |0x0c = DJ Headphones (DJヘッドホン)
/// |0x0d = Sumo Head (どすごい頭)
/// |0x0e = Helmet (ヘルメット)
/// |0x0f = Samurai Helmet (武者かぶと)
/// |0x10 = White Veil (白いベール)
/// |0x11 = Ninja Tuft (忍者のふさふさ)
/// |0x12 = Skull (どくろ)
/// |0x13 = King's Crown (王者のかんむり)
/// |0x14 = Angel's Halo (天使のわっか)
/// |0x15 = Cloth Hood (ほっかむり)
/// |0x16 = Mappy's Hat (マッピーぼうし)
/// |0x17 = Red Cow Head (赤べこヘッド)
/// |0x18 = Fox Mask (きつねのお面)
/// |0x19 = Devil Face (デビルフェイス)
/// |0x1a = Idolm@ster Ribbon (アイマスリボン)
/// |0x1b = Felyne Helm (アイルーヘルム)
/// |0x1c = Miku Hair (ミクミクヘアー)
/// |0x1d = Signpost (ひょうしき)
pub const fn file_1_headgear_unlock_flags() -> usize {
    0x100e1d3
}

/// [29x1 bytes] File 1 - Clothes Unlock Flags
/// 0x0 = Locked
/// 0x1 = Unlocked
///
/// |0x00 = Thousand-hand Drum (千手かんどん)
/// |0x01 = Apple (つがるりんご)
/// |0x02 = Pencil (えんぴつ)
/// |0x03 = Chibi Don (ちびどん)
/// |0x04 = Katamari Damacy (かたまり魂)
/// |0x05 = Castle (しろ)
/// |0x06 = Don Coaster (どんコースター)
/// |0x07 = Cake (ケーキ)
/// |0x08 = Fried Prawn (エビフーライ)
/// |0x09 = Dice (サイコロ)
/// |0x0a = Mt. Fuji Body (ふじさんボディ)
/// |0x0b = Kotatsu Table (こたつ)
/// |0x0c = Zipper (ジッパー)
/// |0x0d = Sumo Wrestler's Loincloth (まわし)
/// |0x0e = Space Suit (宇宙服)
/// |0x0f = Samurai Armor (武者よろい)
/// |0x10 = White Dress (白いドレス)
/// |0x11 = Ninja Clothes (忍者のふく )
/// |0x12 = Rib Bones (ろっこつ)
/// |0x13 = King's Cloak (王者のマント)
/// |0x14 = Angel's Wings (天使のはなね)
/// |0x15 = Furoshiki (ふろしき)
/// |0x16 = Mappy's Clothes (マッピーのふく)
/// |0x17 = Red Cow's Body (赤べこボディ)
/// |0x18 = Devil Body (デビルボディ)
/// |0x19 = Idolm@ster Dress (アイマスドレス)
/// |0x1a = Felyne Body (アイルーボディ)
/// |0x1b = Miku Dress (ミクミクドレス)
/// |0x1c = Kamuro Gate (かむろゲート)
pub const fn file_1_clothes_unlock_flags() -> usize {
    0x100e237
}

/// [8-bit][Bitflags] Omikoshi - Flags
/// Bit0 = Always True
/// Bit1 = Battled Today Flag
/// Bit3 = Boss Rush Mode Unlocked
/// Bit7 = Unknown
pub const fn omikoshi_flags() -> MemoryRef {
    bits8!(0x100e29c)
}

/// [8-bit] Omikoshi - Starting Prefecture
/// 0x00 = Hokkaido
/// 0x01 = Aomori
/// 0x02 = Iwate
/// 0x03 = Miyagi
/// 0x04 = Akita
/// 0x05 = Yamagata
/// 0x06 = Fukushima
/// 0x07 = Ibaraki
/// 0x08 = Tochigi
/// 0x09 = Gunma
/// 0x0a = Saitama
/// 0x0b = Chiba
/// 0x0c = Tokyo
/// 0x0d = Kanagawa
/// 0x0e = Niigata
/// 0x0f = Toyama
/// 0x10 = Ishikawa
/// 0x11 = Fukui
/// 0x12 = Yamanashi
/// 0x13 = Nagano
/// 0x14 = Gifu
/// 0x15 = Shizuoka
/// 0x16 = Aichi
/// 0x17 = Mie
/// 0x18 = Shiga
/// 0x19 = Kyoto
/// 0x1a = Osaka
/// 0x1b = Hyogo
/// 0x1c = Nara
/// 0x1d = Wakayama
/// 0x1e = Tottori
/// 0x1f = Shimane
/// 0x20 = Okuyama
/// 0x21 = Hiroshima
/// 0x22 = Yamaguchi
/// 0x23 = Tokushima
/// 0x24 = Kagawa
/// 0x25 = Ehime
/// 0x26 = Kochi
/// 0x27 = Fukuoka
/// 0x28 = Saga
/// 0x29 = Nagasaki
/// 0x2a = Kumamoto
/// 0x2b = Oita
/// 0x2c = Miyasaki
/// 0x2d = Kagoshima
/// 0x2e = Okinawa
pub const fn omikoshi_starting_prefecture() -> MemoryRef {
    bits8!(0x100e2b0)
}

/// [16-bit] Omikoshi - Days Passed
pub const fn omikoshi_days_passed() -> MemoryRef {
    bits16!(0x100e2b2)
}

/// [16-bit] Omikoshi - Number of Disciples on Hand
pub const fn omikoshi_number_of_disciples_on_hand() -> MemoryRef {
    bits16!(0x100e2b8)
}

/// [16-bit] Omikoshi - Number of Missing Disciples
pub const fn omikoshi_number_of_missing_disciples() -> MemoryRef {
    bits16!(0x100e2ba)
}

/// [47x32 bytes] Omikoshi - Prefecture Structs
/// Prefectures are offset based on their numerical IDs
/// - See 0x100e2b0 for the full prefecture list
///
/// |0x0 = [16-bit] Number of Disciples
/// |0x6 = [8-bit][Lower4] State Flags
/// 0x0 = Starting Prefecture
/// 0x1 = Conquered
/// 0x4 = Unlocked
/// 0x6 = Locked
pub const fn omikoshi_prefecture_structs() -> usize {
    0x100e2ec
}

/// [8x32 bytes] Omikoshi - Boss Structs
/// |0x0 = [16-bit] Power
/// |0x6 = [8-bit][Lower4] State Flags
/// 0x2 = Defeated
/// 0x4 = Unlocked
/// 0x6 = Locked
pub const fn omikoshi_boss_structs() -> usize {
    0x100e8cc
}

/// [8-bit] Omikoshi - Number of Controlled Prefectures
/// Bosses trigger based on specific values:
/// 0x06 = Guts Cannon (ガッツ・のキャノン)
/// 0x0e = Dokon Gang (ドコンギャング)
/// 0x18 = Dokon + Dokon Gang (ドコン + ドコンギャング)
/// 0x20 = Guts Jet (ガッツ・ジェット)
/// 0x28 = Devil Don-chan (デビルどんちゃん)
/// 0x2f = Guts Eater (ガッツ・イーテル) / Armage-Don (アルマゲどん)
pub const fn omikoshi_number_of_controlled_prefectures() -> MemoryRef {
    bits8!(0x100e9cc)
}

/// [8-bit] Omikoshi - Screen Tile
/// Controls which screen tile is shown when interacting with the Japan map
/// 0x2f = Guts Cannon
/// 0x30 = Dokon Gang
/// 0x31 = Dokon + Dokon Gang
/// 0x32 = Guts Jet
/// 0x33 = Devil Don-chan
/// 0x34 = Guts Eater
/// 0x35 = Armage-Don
/// 0x36 = Boss Rush
/// 0x37 = Japan / Boss Rush
pub const fn omikoshi_screen_tile() -> MemoryRef {
    bits8!(0x100e9ce)
}

/// [8-bit] Omikoshi - Last Cursor Position
/// - See 0x100e2b0 for prefecture definitions
pub const fn omikoshi_last_cursor_position() -> MemoryRef {
    bits8!(0x100ec83)
}

/// [50x4 bytes] Omikoshi - Wadaiko Enbu Data (Amakuchi)
/// |0x00 = [8-bit] Status
/// .0x1 = New
/// .0x2 = Attempted
/// .0x3 = Cleared
/// |0x04 = [16-bit] Times Cleared
pub const fn omikoshi_wadaiko_enbu_data_amakuchi() -> usize {
    0x100ec9c
}

/// [50x4 bytes] Omikoshi - Wadaiko Enbu Data (Chuukara)
/// - See 0x0100ec9c for definition
pub const fn omikoshi_wadaiko_enbu_data_chuukara() -> usize {
    0x100ed64
}

/// [50x4 bytes] Omikoshi - Wadaiko Enbu Data (Karakuchi)
/// - See 0x0100ec9c for definition
pub const fn omikoshi_wadaiko_enbu_data_karakuchi() -> usize {
    0x100ee2c
}

/// [8-bit] File 1 - Selected Taiko Sound
/// 0x00 = Taiko (太鼓)
/// 0x01 = Deluxe Taiko (豪華な太鼓)
/// 0x02 = Drum (ドラム)
/// 0x03 = Synthesizer Drum (シンセドラム)
/// 0x04 = Tambourine (タンバリン)
/// 0x05 = Wooden Fish (もくぎょ)
/// 0x06 = Shuriken (手裏剣)
/// 0x07 = Fart (おなら)
/// 0x08 = Don-chan (どんちゃん)
/// 0x09 = Tsuzumi (つづみ)
/// 0x0a = Fry Pan (フライパン)
/// 0x0b = Bell (すず)
/// 0x0c = Quiz (クイズ)
/// 0x0d = Rap (ラップ)
/// 0x0e = Clapping (手拍子)
/// 0x0f = Dog and Cat (いぬねこ)
/// 0x10 = 8-bit Taiko (８ビット太鼓)
/// 0x11 = Knife (刀)
/// 0x12 = Chibi Don (ちびどん)
/// 0x13 = Cameraman (カメラマン)
/// 0x14 = Gangster Don (ツッパリどん)
/// 0x15 = Yokozuna Don (よこづなどん)
/// 0x16 = Chabu-dai (ちゃぶ台)
/// 0x17 = Gun (銃)
/// 0x18 = Dynamite (ダイナマイト)
/// 0x19 = Conga (コンガ)
/// 0x1a = Yahoo (ヤッホー)
/// 0x1b = Fizzy Drink (炭酸飲料)
/// 0x1c = Sneeze (くしゃみ)
/// 0x1d = Kung Fu (拳法)
/// 0x1e = Random (ランダム)
pub const fn file_1_selected_taiko_sound() -> MemoryRef {
    bits8!(0x100eef4)
}

/// [8-bit] File 1 - Game Modifiers - Speed
/// 0x0 = Normal (普通)
/// 0x1 = Double Speed (ばいぞく)
/// 0x2 = Triple Speed (さんばい)
/// 0x3 = Quadruple Speed (よんばい)
pub const fn file_1_game_modifiers_speed() -> MemoryRef {
    bits8!(0x100eef5)
}

/// [8-bit] File 1 - Game Modifiers - Special
/// 0x0 = Normal (普通)
/// 0x1 = Auto (オート)
/// 0x2 = Sudden Death (かんぺき)
/// 0x3 = Invisible (ドロン)
pub const fn file_1_game_modifiers_special() -> MemoryRef {
    bits8!(0x100eef6)
}

/// [8-bit] File 1 - Game Modifiers - Note Symbol
/// 0x0 = Normal
/// 0x1 = Whimsical
/// 0x2 = Random
/// 0x3 = Reversed
pub const fn file_1_game_modifiers_note_symbol() -> MemoryRef {
    bits8!(0x100eef7)
}

/// [80x1 bytes] File 1 - Song Unlock Flags
/// Songs are offset based on their numerical IDs
/// - See 0xfca080 for the full song list
///
/// 0x0 = Locked
/// 0x1 = Unlocked
///
/// Unlockable Songs:
/// |0x17 = Etude Op.10-4 (練習曲Op.10-4) - Chopin (Ura)
/// |0x23 = Hatsune Miku no Gekishou (初音ミクの激唱) - Hatsune Miku Project DIVA - 2nd (Ura)
/// |0x2C = White Rose Insanity (Ura)
/// |0x31 = Taiyou mo Yappappa (太陽もヤッパッパー) (Ura)
/// |0x33 = Sports Digestdon (スポーツダイジェスドン) - ~Fill In The Sky~ (Ura)
/// |0x38 = Rinda wa Kyou mo Zekkouchou (リンダは今日も絶好調) (Ura)
/// |0x3B = DON'T CUT (Ura)
/// |0x3E = Rumble Ranbu (らんぶる乱舞) (Ura)
/// |0x49 = Rotter Tarmination (Ura)
/// |0x4B = Black Rose Apostle (Ura)
/// |0x4E = Mata Saitama 2000 (またさいたま2000)
/// |0x4F = Cycle of Rebirth (サイクル・オブ・リバース)
pub const fn file_1_song_unlock_flags() -> usize {
    0x100eef8
}

/// [8-bit][Bitflags] File 1 - Taiko Sound Unlock Flags
/// Bit0 = Taiko (太鼓)
/// Bit1 = Deluxe Taiko (豪華な太鼓)
/// Bit2 = Drum (ドラム)
/// Bit3 = Synthesizer Drum (シンセドラム)
/// Bit4 = Tambourine (タンバリン)
/// Bit5 = Wooden Fish (もくぎょ)
/// Bit6 = Shuriken (手裏剣)
/// Bit7 = Fart (おなら)
pub const fn file_1_taiko_sound_unlock_flags_1() -> MemoryRef {
    bits8!(0x100ef48)
}

/// [8-bit][Bitflags] File 1 - Taiko Sound Unlock Flags
/// Bit0 = Don-chan (どんちゃん)
/// Bit1 = Tsuzumi (つづみ)
/// Bit2 = Fry Pan (フライパン)
/// Bit3 = Bell (すず)
/// Bit4 = Quiz (クイズ)
/// Bit5 = Rap (ラップ)
/// Bit6 = Clapping (手拍子)
/// Bit7 = Dog and Cat (いぬねこ)
pub const fn file_1_taiko_sound_unlock_flags_2() -> MemoryRef {
    bits8!(0x100ef49)
}

/// [8-bit][Bitflags] File 1 - Taiko Sound Unlock Flags
/// Bit0 = 8-bit Taiko (８ビット太鼓)
/// Bit1 = Knife (刀)
/// Bit2 = Chibi Don (ちびどん)
/// Bit3 = Cameraman (カメラマン)
/// Bit4 = Gangster Don (ツッパリどん)
/// Bit5 = Yokozuna Don (よこづなどん)
/// Bit6 = Chabu-dai (ちゃぶ台)
/// Bit7 = Gun (銃)
pub const fn file_1_taiko_sound_unlock_flags_3() -> MemoryRef {
    bits8!(0x100ef4a)
}

/// [8-bit][Bitflags] File 1 - Taiko Sound Unlock Flags
/// Bit0 = Dynamite (ダイナマイト)
/// Bit1 = Conga (コンガ)
/// Bit2 = Yahoo (ヤッホー)
/// Bit3 = Fizzy Drink (炭酸飲料)
/// Bit4 = Sneeze (くしゃみ)
/// Bit5 = Kung Fu (拳法)
/// Bit6 = Random (ランダム)
/// Bit7 = Unused
pub const fn file_1_taiko_sound_unlock_flags_4() -> MemoryRef {
    bits8!(0x100ef4b)
}

/// [8-bit][Bitflags] File 1 - Speed Modifier Unlock Flags
/// Bit0 = Normal (普通)
/// Bit1 = Double Speed (ばいぞく)
/// Bit2 = Triple Speed (さんばい)
/// Bit3 = Quadruple Speed (よんばい)
pub const fn file_1_speed_modifier_unlock_flags() -> MemoryRef {
    bits8!(0x100ef4c)
}

/// [8-bit][Bitflags] File 1 - Special Modifier Unlock Flags
/// Bit0 = Normal (普通)
/// Bit1 = Auto (オート)
/// Bit2 = Sudden Death (かんぺき)
/// Bit3 = Invisible (ドロン)
pub const fn file_1_special_modifier_unlock_flags() -> MemoryRef {
    bits8!(0x100ef4d)
}

/// [8-bit][Bitflags] File 1 - Note Symbol Modifier Unlock Flags
/// Bit0 = Normal (普通)
/// Bit1 = Whimsical (きまぐれ)
/// Bit2 = Random (でたらめ)
/// Bit3 = Reverse (あべこべ)
pub const fn file_1_note_symbol_modifier_unlock_flags() -> MemoryRef {
    bits8!(0x100ef4e)
}

/// [8-bit][Bitflags] File 1 - Difficulty Unlock Flags
/// Bit3 = Oni
pub const fn file_1_difficulty_unlock_flags() -> MemoryRef {
    bits8!(0x100ef4f)
}

/// [16-bit] File 1 - Highest Combo
pub const fn file_1_highest_combo() -> MemoryRef {
    bits16!(0x100ef58)
}

/// [16-bit] File 1 - Highest Drumroll Hits
pub const fn file_1_highest_drumroll_hits() -> MemoryRef {
    bits16!(0x100ef5a)
}

/// [32-bit] File 1 - Highest Score
pub const fn file_1_highest_score() -> MemoryRef {
    bits32!(0x100ef5c)
}

/// [32-bit] File 1 - Taiko Counter
pub const fn file_1_taiko_counter() -> MemoryRef {
    bits32!(0x100ef60)
}

/// [16-bit] File 1 - Highest Combo (Medley Mode)
pub const fn file_1_highest_combo_medley_mode() -> MemoryRef {
    bits16!(0x100ef68)
}

/// [16-bit] File 1 - Highest Drumroll Hits (Medley Mode)
pub const fn file_1_highest_drumroll_hits_medley_mode() -> MemoryRef {
    bits16!(0x100ef6a)
}

/// [32-bit] File 1 - Highest Score (Medley Mode)
pub const fn file_1_highest_score_medley_mode() -> MemoryRef {
    bits32!(0x100ef6c)
}

/// [230x32 bytes] File 1 - Song Scoreboard (Kantan)
/// Songs are offset based on their numerical IDs
/// - See 0xfca080 for the full song list
///
/// |0x01 = [8-bit] Crown
/// .0x0 = None
/// .0x1 = Silver
/// .0x2 = Gold
/// |0x02 = [16-bit] Highest Combo
/// |0x04 = [16-bit] Drumroll Hits
/// |0x06 = [16-bit] Percentage Hit
/// |0x08 = [16-bit] Great Hits
/// |0x0a = [16-bit] Good Hits
/// |0x0c = [16-bit] Misses
/// |0x10 = [32-bit] High Score
/// |0x14 = [32-bit] Times Played
pub const fn file_1_song_scoreboard_kantan() -> usize {
    0x100ef70
}

/// [230x32 bytes] File 1 - Song Scoreboard (Futsuu)
/// - See 0x0100ef70 for definition
pub const fn file_1_song_scoreboard_futsuu() -> usize {
    0x10114f0
}

/// [230x32 bytes] File 1 - Song Scoreboard (Muzukashii)
/// - See 0x0100ef70 for definition
pub const fn file_1_song_scoreboard_muzukashii() -> usize {
    0x1013a70
}

/// [230x32 bytes] File 1 - Song Scoreboard (Oni)
/// - See 0x0100ef70 for definition
pub const fn file_1_song_scoreboard_oni() -> usize {
    0x1015ff0
}

/// [78x1 bytes] In-Game Achievements Unlock Flags
/// 0x0 = Locked
/// 0x1 = Unlocked
///
/// |0x00 = First Clear
/// |0x01 = Earn 15 Silver Crowns on Kantan
/// |0x02 = Earn 30 Silver Crowns on Kantan
/// |0x03 = Earn 70 Silver Crowns on Kantan
/// |0x04 = Earn 15 Silver Crowns on Futsuu
/// |0x05 = Earn 30 Silver Crowns on Futsuu
/// |0x06 = Earn 70 Silver Crowns on Futsuu
/// |0x07 = Earn 15 Silver Crowns on Muzukashii
/// |0x08 = Earn 30 Silver Crowns on Muzukashii
/// |0x09 = Earn 70 Silver Crowns on Muzukashii
/// |0x0a = Earn 15 Silver Crowns on Oni
/// |0x0b = Earn 30 Silver Crowns on Oni
/// |0x0c = Earn 70 Silver Crowns on Oni
/// |0x0d = Accumulate 10,000 hits on the Taiko Counter
/// |0x0e = Accumulate 20,000 hits on the Taiko Counter
/// |0x0f = Accumulate 30,000 hits on the Taiko Counter
/// |0x10 = Accumulate 40,000 hits on the Taiko Counter
/// |0x11 = Accumulate 50,000 hits on the Taiko Counter
/// |0x12 = Accumulate 60,000 hits on the Taiko Counter
/// |0x13 = Accumulate 70,000 hits on the Taiko Counter
/// |0x14 = Accumulate 76,500 hits on the Taiko Counter
/// |0x15 = Accumulate 80,000 hits on the Taiko Counter
/// |0x16 = Accumulate 90,000 hits on the Taiko Counter
/// |0x17 = Accumulate 99,999 hits on the Taiko Counter
/// |0x18 = Earn 10 Golden Crowns
/// |0x19 = Earn 25 Golden Crowns
/// |0x1a = Earn 50 Golden Crowns
/// |0x1b = Earn 75 Golden Crowns
/// |0x1c = Earn 100 Golden Crowns
/// |0x1d = Clear a song with all Great hits and no misses
/// |0x1e = Clear a song with all Good hits and no misses
/// |0x1f = Reach a combo of 100 hits or more
/// |0x20 = Reach a combo of 200 hits or more
/// |0x21 = Reach a combo of 400 hits or more
/// |0x22 = Reach a combo of 600 hits or more
/// |0x23 = Get more than 100 total hits in a song
/// |0x24 = Score 500,000 points or more in a song
/// |0x25 = Clear 5 dojo training stages
/// |0x26 = Clear 10 dojo training stages
/// |0x27 = Clear 20 dojo training stages
/// |0x28 = Clear 30 dojo training stages
/// |0x29 = Clear 40 dojo training stages
/// |0x2a = Clear 50 dojo training stages
/// |0x2b = Control 10 dojos in Omikoshi mode
/// |0x2c = Control 20 dojos in Omikoshi mode
/// |0x2d = Control 30 dojos in Omikoshi mode
/// |0x2e = Control 40 dojos in Omikoshi mode
/// |0x2f = Control all dojos in Omikoshi mode
/// |0x30 = Beat Omikoshi mode
/// |0x31 = Beat 2 opponents in a row in Boss Rush mode
/// |0x32 = Beat 5 opponents in a row in Boss Rush mode
/// |0x33 = Clear the Boss Rush mode
/// |0x34 = Gather 100 or more Drum disciples in Omikoshi mode
/// |0x35 = Gather 200 or more Drum disciples in Omikoshi mode
/// |0x36 = Gather 300 or more Drum disciples in Omikoshi mode
/// |0x37 = Beat the opponent by more than 9 meters in a dojo battle
/// |0x38 = Lose to the opponent by more than 9 meters in a dojo battle
/// |0x39 = Fail 10 times in Freeplay
/// |0x3a = Play 1 wireless multiplayer match
/// |0x3b = Play 5 wireless multiplayer match
/// |0x3c = Play 10 wireless multiplayer match
/// |0x3d = Play 15 wireless multiplayer match
/// |0x3e = Play 20 wireless multiplayer match
/// |0x3f = Play wireless multiplayer matches with 5 different people
/// |0x40 = Earn a Gold Crown on "The world is all one!!" on Kantan
/// |0x41 = Earn a Gold Crown on "The world is all one!!" on Futsuu
/// |0x42 = Earn a Gold Crown on "Monster Hunter Medley" on Kantan
/// |0x43 = Earn a Gold Crown on "Monster Hunter Medley" on Futsuu
/// |0x44 = Earn a Gold Crown on "Hatsune Miku no Gekishou" on Kantan
/// |0x45 = Earn a Gold Crown on "Hatsune Miku no Gekishou" on Futsuu
/// |0x46 = Earn a Gold Crown on "MachineGun Kiss" on Kantan
/// |0x47 = Earn a Gold Crown on "MachineGun Kiss" on Futsuu
/// |0x48 = Reach a combo of 500 or more in Medley mode
/// |0x49 = Reach a combo of 1000 or more in Medley mode
/// |0x4a = Reach a combo of 1500 or more in Medley mode
/// |0x4b = Reach a combo of 2000 or more in Medley mode
/// |0x4c = Drumroll a total of 500 times or more in Medley mode
/// |0x4d = Score 3,000,000 points or more in Medley mode
pub const fn in_game_achievements_unlock_flags() -> usize {
    0x101bd00
}

/// [97x1 bytes] Recieved Mail Flags
/// Likely non-exhaustive
///
/// 0x0 = Unseen
/// 0x1 = Seen
///
/// Achievements:
/// |0x00 = First Clear
/// |0x01 = Earn 15 Silver Crowns on Kantan
/// |0x02 = Earn 30 Silver Crowns on Kantan
/// |0x03 = Earn 70 Silver Crowns on Kantan
/// |0x04 = Earn 15 Silver Crowns on Futsuu
/// |0x05 = Earn 30 Silver Crowns on Futsuu
/// |0x06 = Earn 70 Silver Crowns on Futsuu
/// |0x07 = Earn 15 Silver Crowns on Muzukashii
/// |0x08 = Earn 30 Silver Crowns on Muzukashii
/// |0x09 = Earn 70 Silver Crowns on Muzukashii
/// |0x0a = Earn 15 Silver Crowns on Oni
/// |0x0b = Earn 30 Silver Crowns on Oni
/// |0x0c = Earn 70 Silver Crowns on Oni
/// |0x0d = Accumulate 10,000 hits on the Taiko Counter
/// |0x0e = Accumulate 20,000 hits on the Taiko Counter
/// |0x0f = Accumulate 30,000 hits on the Taiko Counter
/// |0x10 = Accumulate 40,000 hits on the Taiko Counter
/// |0x11 = Accumulate 50,000 hits on the Taiko Counter
/// |0x12 = Accumulate 60,000 hits on the Taiko Counter
/// |0x13 = Accumulate 70,000 hits on the Taiko Counter
/// |0x14 = Accumulate 76,500 hits on the Taiko Counter
/// |0x15 = Accumulate 80,000 hits on the Taiko Counter
/// |0x16 = Accumulate 90,000 hits on the Taiko Counter
/// |0x17 = Accumulate 99,999 hits on the Taiko Counter
/// |0x18 = Earn 10 Golden Crowns
/// |0x19 = Earn 25 Golden Crowns
/// |0x1a = Earn 50 Golden Crowns
/// |0x1b = Earn 75 Golden Crowns
/// |0x1c = Earn 100 Golden Crowns
/// |0x1d = Clear a song with all Great hits and no misses
/// |0x1e = Clear a song with all Good hits and no misses
/// |0x1f = Reach a combo of 100 hits or more
/// |0x20 = Reach a combo of 200 hits or more
/// |0x21 = Reach a combo of 400 hits or more
/// |0x22 = Reach a combo of 600 hits or more
/// |0x23 = Get more than 100 total hits in a song
/// |0x24 = Score 500,000 points or more in a song
/// |0x25 = Clear 5 dojo training stages
/// |0x26 = Clear 10 dojo training stages
/// |0x27 = Clear 20 dojo training stages
/// |0x28 = Clear 30 dojo training stages
/// |0x29 = Clear 40 dojo training stages
/// |0x2a = Clear 50 dojo training stages
/// |0x2b = Control 10 dojos in Omikoshi mode
/// |0x2c = Control 20 dojos in Omikoshi mode
/// |0x2d = Control 30 dojos in Omikoshi mode
/// |0x2e = Control 40 dojos in Omikoshi mode
/// |0x2f = Control all dojos in Omikoshi mode
/// |0x30 = Beat Omikoshi mode
/// |0x31 = Beat 2 opponents in a row in Boss Rush mode
/// |0x32 = Beat 5 opponents in a row in Boss Rush mode
/// |0x33 = Clear the Boss Rush mode
/// |0x34 = Gather 100 or more Drum disciples in Omikoshi mode
/// |0x35 = Gather 200 or more Drum disciples in Omikoshi mode
/// |0x36 = Gather 300 or more Drum disciples in Omikoshi mode
/// |0x37 = Beat the opponent by more than 9 meters in a dojo battle
/// |0x38 = Lose to the opponent by more than 9 meters in a dojo battle
/// |0x39 = Fail 10 times in Freeplay
/// |0x3a = Play 1 wireless multiplayer match
/// |0x3b = Play 5 wireless multiplayer match
/// |0x3c = Play 10 wireless multiplayer match
/// |0x3d = Play 15 wireless multiplayer match
/// |0x3e = Play 20 wireless multiplayer match
/// |0x3f = Play wireless multiplayer matches with 5 different people
/// |0x40 = Earn a Gold Crown on "The world is all one!!" on Kantan
/// |0x41 = Earn a Gold Crown on "The world is all one!!" on Futsuu
/// |0x42 = Earn a Gold Crown on "Monster Hunter Medley" on Kantan
/// |0x43 = Earn a Gold Crown on "Monster Hunter Medley" on Futsuu
/// |0x44 = Earn a Gold Crown on "Hatsune Miku no Gekishou" on Kantan
/// |0x45 = Earn a Gold Crown on "Hatsune Miku no Gekishou" on Futsuu
/// |0x46 = Earn a Gold Crown on "MachineGun Kiss" on Kantan
/// |0x47 = Earn a Gold Crown on "MachineGun Kiss" on Futsuu
/// |0x48 = Reach a combo of 500 or more in Medley mode
/// |0x49 = Reach a combo of 1000 or more in Medley mode
/// |0x4a = Reach a combo of 1500 or more in Medley mode
/// |0x4b = Reach a combo of 2000 or more in Medley mode
/// |0x4c = Drumroll a total of 500 times or more in Medley mode
/// |0x4d = Score 3,000,000 points or more in Medley mode
/// |0x4e = Apprentice Promotion
/// |0x4f = Novice Promotion
/// |0x50 = Disciple Promotion
/// |0x51 = Half a Man Promotion
/// |0x52 = Qualified Promotion
/// |0x53 = Capable Promotion
/// |0x54 = Go-getter Promotion
/// |0x55 = Professional Promotion
/// |0x56 = Expert Promotion
/// |0x57 = Iron Man Promotion
/// |0x58 = Master Promotion
/// |0x59 = Grand Master Promotion
/// |0x5a = Mikona
/// |0x5b = Welcome Message
/// |0x5c = Bachi-sensei - Omikoshi Battle
/// |0x5d = Bachi-sensei - Medley Mode
/// |0x5e = Bachi-sensei - Clothes
/// |0x5f = Suzu-don
pub const fn recieved_mail_flags() -> usize {
    0x101bd52
}

/// [32 Bytes] File 2 - Song Stats - One Day (Easy)
pub const fn file_2_song_stats_one_day_easy() -> usize {
    0x101cda4
}

/// [32-bit] Game Mode
/// 0x0 = Freeplay
/// 0x1 = Omikoshi Story
/// 0x2 = Medley Mode
/// 0x3 = Don-chan's Room
/// 0x4 = Download Mode
/// 0x5 = Multiplayer
/// 0x6 = Settings
/// 0x7 = Main Menu
pub const fn game_mode() -> MemoryRef {
    bits32!(0x1068d50)
}

// [32-bit] Current Menu ID
// 0x45 = Title Screen
// 0x47 = Profile Select
// 0x49 = Data Install
// 0x4b = Main Menu
// 0x4c = Medley Mode
// 0x4d = Playing Medley
// 0x4e = Medley - Results
// 0x4f = Don-chan's Room
// 0x50 = Change Clothes
// 0x51 = Mailbox
// 0x52 = Scoreboard
// 0x53 = In-game Achievement List
// 0x54 = Everybody's Data
// 0x55 = Tutorial
// 0x56 = Multiplayer Menu
// 0x5b = Settings
// 0x61 = Freeplay - Song Select
// 0x62 = Playing Song
// 0x63 = Results
// 0x65 = Mail Recieved
// 0x66 = Saving
// 0x67 = Omikoshi - Select Starting Prefecture
// 0x68 = Omikoshi - Cutscene
// 0x6a = Omikoshi - Dojo
// 0x6b = Omikoshi - Map
// 0x6d = Omikoshi - Battle
// 0x6e = Omikoshi - Battle Results
// 0x70 = Doji Training - Results
pub const fn current_menu_id() -> MemoryRef {
    bits32!(0x1068d5c)
}
