use rustcheevos::prelude::*;
use rustcheevos::types::rich::{BuiltInMacro, LookupTable, MacroRef, RichPresence};

use crate::mem;
use crate::types::difficulty::Difficulty;
use crate::types::menu::{
    DonChansRoomState, FreeplayState, GeneralMenuState, MedleyState, MenuState, OmikoshiState,
};
use crate::types::mode::omikoshi::BattleOpponent;
use crate::types::mode::{MedleyMode, omikoshi::OmikoshiMode};
use crate::types::profile::Profile;
use crate::types::song::Song;
use crate::types::song::in_game_stats::InGameStats;
use crate::types::song::result::SongResult;
use crate::util::{TOTAL_NUM_CLOTHES, TOTAL_NUM_HEADGEAR, TOTAL_NUM_IN_GAME_ACHIEVEMENTS};

const MENU_STATE_LOOKUP: [(u32, &str); 23] = [
    (
        FreeplayState::SongSelect as u32,
        "Browsing songs in Freeplay mode",
    ),
    (
        MedleyState::SongSelect as u32,
        "Browsing songs in Medley mode",
    ),
    (
        OmikoshiState::SelectStartingPrefecture as u32,
        "Selecting home prefecture",
    ),
    (OmikoshiState::Cutscene as u32, "Watching a cutscene"),
    (OmikoshiState::Dojo as u32, "Hanging around the Dojo"),
    (OmikoshiState::Map as u32, "Planning a new conquest"),
    (OmikoshiState::Preparing as u32, "Preparing for battle"),
    (
        OmikoshiState::BattleResults as u32,
        "Viewing the results of the battle",
    ),
    (
        DonChansRoomState::Menu as u32,
        "Hanging out in Don-chan's room",
    ),
    (
        DonChansRoomState::Scoreboard as u32,
        "Looking at the scoreboard",
    ),
    // TODO: Would be nice if we could have the count of mails.
    (
        DonChansRoomState::Mailbox as u32,
        "Going through the Mailbox",
    ),
    (
        DonChansRoomState::AchievementList as u32,
        "Browsing the in-game achievements",
    ),
    (
        DonChansRoomState::EverybodyData as u32,
        "Looking at Everybody's Data",
    ),
    (
        GeneralMenuState::SplashScreen as u32,
        "Viewing the splash screens",
    ),
    (GeneralMenuState::TitleScreen as u32, "On the title screen"),
    (
        GeneralMenuState::WarningScreen as u32,
        "On the warning screen",
    ),
    (
        GeneralMenuState::ProfileSelect as u32,
        "Selecting a profile",
    ),
    (GeneralMenuState::Mail as u32, "Reading an e-mail"),
    (GeneralMenuState::NameEntry as u32, "Entering a name"),
    (GeneralMenuState::Download as u32, "In the Download menu"),
    (
        GeneralMenuState::Multiplayer as u32,
        "In the Multiplayer menu",
    ),
    (
        GeneralMenuState::Settings as u32,
        "Configuring options in the Settings menu",
    ),
    (GeneralMenuState::Saving as u32, "Saving..."),
];

struct InGameHandles {
    score: MacroRef,
    combo: MacroRef,
    total_hits: MacroRef,
    quota_gauge: MacroRef,
}

impl InGameHandles {
    fn register(rp: &mut RichPresence) -> Self {
        let score = rp.builtin_macro(
            BuiltInMacro::Number,
            measured!(InGameStats::current_score()),
        );

        let combo = rp.builtin_macro(
            BuiltInMacro::Number,
            measured!(InGameStats::current_combo()),
        );
        let total_hits =
            rp.builtin_macro(BuiltInMacro::Number, measured!(InGameStats::total_hits()));
        let quota_gauge = rp.builtin_macro(
            BuiltInMacro::Number,
            measured!(InGameStats::quoata_guage().div(100)),
        );
        Self {
            score,
            combo,
            total_hits,
            quota_gauge,
        }
    }
}

struct ResultHandles {
    score: MacroRef,
    great_hits: MacroRef,
    quota_gauge: MacroRef,
    combo: MacroRef,
    crown: MacroRef,
}

impl ResultHandles {
    fn register(rp: &mut RichPresence) -> Self {
        let score = rp.builtin_macro(BuiltInMacro::Number, mem::results_score());
        let great_hits = rp.builtin_macro(BuiltInMacro::Number, mem::results_great_hits());
        let quota_gauge = rp.builtin_macro(
            BuiltInMacro::Number,
            measured!(mem::results_quota_gauge().div(100)),
        );
        let combo = rp.builtin_macro(BuiltInMacro::Number, mem::results_highest_combo());
        let crown_lt = LookupTable::new("SongResult").with_entries(SongResult::LOOKUP_TABLE);
        let crown = rp.register_lookup(crown_lt, mem::end_result());
        Self {
            score,
            great_hits,
            quota_gauge,
            combo,
            crown,
        }
    }
}

struct RpHandles {
    in_game: InGameHandles,
    result: ResultHandles,
    current_song: MacroRef,
    difficulty: MacroRef,
    golden_crowns: MacroRef,
    silver_crowns: MacroRef,
    menu_state: MacroRef,
}

impl RpHandles {
    fn register(rp: &mut RichPresence) -> Self {
        let in_game = InGameHandles::register(rp);
        let result = ResultHandles::register(rp);

        let song_lt = LookupTable::new("SongId").with_entries(Song::LOOKUP_ENTRIES);
        let current_song = rp.register_lookup(song_lt, mem::current_song_id());

        let difficulty_lt = LookupTable::new("Difficulty").with_entries(Difficulty::lookup());
        let difficulty = rp.register_lookup(difficulty_lt, mem::current_song_difficulty());

        let golden_crowns = rp.builtin_macro(BuiltInMacro::Number, Profile::num_golden_crowns());
        let silver_crowns = rp.builtin_macro(BuiltInMacro::Number, Profile::num_silver_crowns());

        let menu_state_lt = LookupTable::new("MenuState").with_entries(MENU_STATE_LOOKUP);
        let menu_state = rp.register_lookup(menu_state_lt, mem::current_menu_id());

        Self {
            in_game,
            result,
            current_song,
            difficulty,
            golden_crowns,
            silver_crowns,
            menu_state,
        }
    }
}

pub(crate) fn generate_rich_presence() -> RichPresence {
    let mut rp = RichPresence::new();

    let rp_handles = RpHandles::register(&mut rp);

    let taiko_counter = rp.builtin_macro(
        BuiltInMacro::Number,
        chain!(
            Profile::offset_for_current(),
            measured!(mem::file_1_taiko_counter())
        ),
    );
    rp.add_conditional_display(
        MenuState::is(GeneralMenuState::MainMenu as u32),
        format!("In the main menu • 🥁x{taiko_counter}"),
    );

    rp.add_conditional_display(
        chain!(
            or_next!(MenuState::is(DonChansRoomState::Scoreboard as u32)),
            MenuState::is(FreeplayState::SongSelect as u32)
        ),
        format!(
            "{} • 🥇x{} • 🥈x{}",
            rp_handles.menu_state, rp_handles.golden_crowns, rp_handles.silver_crowns
        ),
    );

    register_freeplay_displays(&mut rp, &rp_handles);
    register_omikoshi_displays(&mut rp, &rp_handles);
    register_medley_displays(&mut rp, &rp_handles);
    register_don_chans_room_displays(&mut rp, &rp_handles);

    rp.add_conditional_display(
        chain!(
            or_next!(MenuState::is(FreeplayState::Playing as u32)),
            MenuState::is(MedleyState::Playing as u32),
            mem::in_game_flag().eq(0x0),
        ),
        "Loading...",
    );

    rp.add_conditional_display(
        mem::in_game_flag().eq(0x0),
        format!("{}", rp_handles.menu_state),
    );

    rp.add_static_display("Playing Taiko no Tatsujin: Portable DX");
    rp
}

fn register_freeplay_displays(
    rp: &mut RichPresence,
    RpHandles {
        in_game,
        result,
        current_song,
        difficulty,
        ..
    }: &RpHandles,
) {
    rp.add_conditional_display(
        chain!(
            MenuState::is(FreeplayState::Playing as u32),
            mem::in_game_flag().eq(0x1),
            InGameStats::pointer_not_null(),
        ),
        format!(
            "Playing \"{current_song}\" ({difficulty}) • Score: {} • Combo: {} • Soul gauge: {}%",
            in_game.score, in_game.combo, in_game.quota_gauge
        ),
    );
    rp.add_conditional_display(
        MenuState::is(FreeplayState::Results as u32),
        format!(
            "Viewing results for \"{current_song}\" ({difficulty}) • {}Score: {} • Highest Combo: {} • Soul gauge: {}%",
            result.crown, result.score, result.combo, result.quota_gauge
        ),
    );
}

fn register_medley_displays(
    rp: &mut RichPresence,
    RpHandles {
        in_game,
        result,
        current_song,
        difficulty,
        ..
    }: &RpHandles,
) {
    let current_song_nr =
        rp.builtin_macro(BuiltInMacro::Number, measured!(MedleyMode::current_song()));
    let medley_num_songs =
        rp.builtin_macro(BuiltInMacro::Number, measured!(MedleyMode::num_songs()));

    rp.add_conditional_display(
        chain!(
            MenuState::is(MedleyState::Playing as u32),
            mem::in_game_flag().eq(0x1),
            InGameStats::pointer_not_null(),
        ),
        format!(
            "Playing \"{current_song}\" ({difficulty}) in a medley [{current_song_nr}/{medley_num_songs}] • Score: {} • Combo: {} Soul gauge: {}%",
            in_game.score, in_game.combo, in_game.quota_gauge
        ),
    );
    rp.add_conditional_display(
        MenuState::is(MedleyState::Results as u32),
        format!(
            "Viewing medley results • Score: {} • Highest Combo: {} • Soul gauge: {}%",
            result.score, result.combo, result.quota_gauge
        ),
    );
}

fn register_omikoshi_displays(
    rp: &mut RichPresence,
    RpHandles {
        in_game,
        result,
        menu_state,
        ..
    }: &RpHandles,
) {
    let prefecture_or_boss =
        rp.register_lookup(BattleOpponent::lookup_table(), mem::omikoshi_opponent_id());

    let conquered_dojos = rp.builtin_macro(
        BuiltInMacro::Number,
        measured!(OmikoshiMode::number_of_controlled_prefectures()),
    );

    let disciples = rp.builtin_macro(
        BuiltInMacro::Number,
        measured!(OmikoshiMode::number_of_disciples_on_hand()),
    );

    let total_disciples = rp.builtin_macro(
        BuiltInMacro::Number,
        measured!(OmikoshiMode::total_disciples()),
    );

    let disciples_in_battle = rp.builtin_macro(
        BuiltInMacro::Number,
        measured!(OmikoshiMode::disciples_in_battle()),
    );

    rp.add_conditional_display(
        MenuState::is(OmikoshiState::DojoTrainingResults as u32),
        format!(
            "Viewing the results for a Dojo Training session • Combo: {} • Great hits: {} • Soul gauge: {}%",
            result.combo, result.great_hits, result.quota_gauge
        ),
    );
    rp.add_conditional_display(
        MenuState::is(OmikoshiState::Battle as u32),
        format!(
            "Battling against {prefecture_or_boss} • Disciples: {disciples_in_battle} • Combo: {} • Total hits: {}",
            in_game.combo,
            in_game.total_hits,
        ),
    );
    rp.add_conditional_display(
        chain!(
            or_next!(MenuState::is(OmikoshiState::Cutscene as u32)),
            or_next!(MenuState::is(OmikoshiState::Dojo as u32)),
            or_next!(MenuState::is(OmikoshiState::Map as u32)),
            or_next!(MenuState::is(OmikoshiState::Preparing as u32)),
            MenuState::is(OmikoshiState::BattleResults as u32),
        ),
        format!("{menu_state} • Disciples: {disciples}/{total_disciples} • {conquered_dojos} Dojo controlled"),
    );
}

fn register_don_chans_room_displays(
    rp: &mut RichPresence,
    RpHandles { menu_state, .. }: &RpHandles,
) {
    rp.add_conditional_display(
        MenuState::is(DonChansRoomState::Tutorial as u32),
        "Learning how to play the Taiko with Bachi-sensei",
    );

    let headgear = rp.builtin_macro(BuiltInMacro::Number, measured!(Profile::num_headgear()));
    let clothing = rp.builtin_macro(BuiltInMacro::Number, measured!(Profile::num_clothes()));
    rp.add_conditional_display(
        MenuState::is(DonChansRoomState::ClothingMenu as u32),
        format!("Searching for a new fit • 🧢 {headgear}/{TOTAL_NUM_HEADGEAR}  • 👚 {clothing}/{TOTAL_NUM_CLOTHES}"),
    );

    let challenges = rp.builtin_macro(
        BuiltInMacro::Number,
        measured!(Profile::num_challenges_completed()),
    );
    rp.add_conditional_display(
        chain!(
            or_next!(MenuState::is(DonChansRoomState::Menu as u32)),
            MenuState::is(DonChansRoomState::AchievementList as u32)
        ),
        format!("{menu_state} • 🏆 {challenges}/{TOTAL_NUM_IN_GAME_ACHIEVEMENTS}"),
    );
}
