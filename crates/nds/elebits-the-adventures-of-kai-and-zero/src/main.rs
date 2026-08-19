use rustcheevos::types::game::GameData;
use rustcheevos_cli::{CliError, RustcheevosCli};

use crate::{
    leaderboards::generate_leaderboards, notes::generate_code_notes, rich::generate_rich_presence,
    set::generate_set,
};

mod leaderboards;
mod mem;
mod notes;
mod rich;
mod set;
mod types;
mod utils;

const GAME_ID: u32 = 14780;
const GAME_NAME: &str = "Elebits: The Adventures of Kai and Zero";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    game_data
        .add_achievements(generate_set())
        .add_leaderboards(generate_leaderboards())
        .add_code_notes(generate_code_notes())
        .set_rich_presence(generate_rich_presence());

    RustcheevosCli::parse().run(&game_data)
}
