use rustcheevos::prelude::*;
use rustcheevos::types::rich::{BuiltInMacro, LookupTable, RichPresence};

use crate::mem;
use crate::types::difficulty::Difficulty;
use crate::types::mode::{FreeplayState, GameMode};
use crate::types::result::Result;
use crate::types::song::Song;
use crate::types::song::in_game_stats::InGameStats;

pub(crate) fn generate_rich_presence() -> RichPresence {
    let mut rich = RichPresence::new();

    let song_lt = LookupTable::new("SongId").with_entries(Song::LOOKUP_ENTRIES);
    let current_song = rich.register_lookup(song_lt, mem::current_song_id());

    let score = rich.builtin_macro(
        BuiltInMacro::Number,
        measured!(InGameStats::current_score()),
    );
    let combo = rich.builtin_macro(
        BuiltInMacro::Number,
        measured!(InGameStats::current_combo()),
    );

    let crown_lt = LookupTable::new("SongResult").with_entries(Result::LOOKUP_TABLE);
    let crown = rich.register_lookup(crown_lt, mem::end_result());

    let difficulty_lt = LookupTable::new("Difficulty").with_entries(Difficulty::lookup());
    let difficulty = rich.register_lookup(difficulty_lt, mem::current_song_difficulty());

    // Freeplay mode
    rich.add_conditional_display(
        chain!(
            mem::in_game_flag().eq(0x1),
            GameMode::is(GameMode::Freeplay)
        ),
        format!("Playing \"{current_song}\" ({difficulty}) • Score: {score} • Combo: {combo}"),
    );
    rich.add_conditional_display(
        chain!(
            FreeplayState::is(FreeplayState::Results),
            GameMode::is(GameMode::Freeplay)
        ),
        format!("Viewing results for \"{current_song}\" ({difficulty}) • {crown} Score: {score} • Combo: {combo}"),
    );
    rich.add_conditional_display(
        chain!(
            FreeplayState::is(FreeplayState::SongSelect),
            GameMode::is(GameMode::Freeplay)
        ),
        // TODO: Add counter for how many total crowns the player has.
        "Browsing songs in Freeplay mode",
    );
    rich.add_conditional_display(GameMode::is(GameMode::Freeplay), "In Freeplay mode");

    // Omikoshi Story mode
    rich.add_conditional_display(
        GameMode::is(GameMode::Omikoshi),
        "Playing the Omikoshi Story mode",
    );

    // Medley mode
    rich.add_conditional_display(
        chain!(
            mem::in_game_flag().eq(0x1),
            GameMode::is(GameMode::Freeplay)
        ),
        format!("Playing \"{current_song}\" ({difficulty}) in a Medley • Score: {score} • Combo: {combo}"),
    );
    rich.add_conditional_display(
        GameMode::is(GameMode::Medley),
        "Browsing songs in Medley mode",
    );

    rich.add_conditional_display(
        GameMode::is(GameMode::DonChansRoom),
        "Hanging out in Don-chan's room",
    );
    rich.add_conditional_display(GameMode::is(GameMode::DownloadMode), "Downloading...");
    rich.add_conditional_display(
        GameMode::is(GameMode::Multiplayer),
        "In the Multiplayer menu",
    );
    rich.add_conditional_display(
        GameMode::is(GameMode::Settings),
        "Configuring options in the Settings menu",
    );

    rich.add_conditional_display(GameMode::is(GameMode::MainMenu), "In the main menu");
    rich.add_static_display("Playing Taiko no Tatsujin: Portable DX");

    rich
}
