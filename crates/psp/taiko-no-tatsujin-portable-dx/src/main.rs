use rustcheevos::types::{game::GameData, set::Set};
use rustcheevos_cli::{CliError, RustcheevosCli};

use crate::{
    rich::generate_rich_presence,
    set::core::{generate_set, leaderboards::generate_leaderboards},
};

mod mem;
mod rich;
mod set;
mod types;
mod util;

const GAME_ID: u32 = 19377;
const GAME_NAME: &str = "Taiko no Tatsujin: Portable DX";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    let mut base_set = Set::new_base();
    base_set
        .add_achievements(generate_set())
        .add_leaderboards(generate_leaderboards());

    game_data
        .set_base_set(base_set)
        .set_rich_presence(generate_rich_presence());

    RustcheevosCli::parse().run(&game_data)
}
