use rustcheevos::types::game::GameData;
use rustcheevos_cli::{CliError, RustcheevosCli};

use crate::{rich::generate_rich_presence, set::generate_set};

mod mem;
mod rich;
mod set;
mod types;
mod utils;

const GAME_ID: u32 = 14780;
const GAME_NAME: &str = "Elebits: The Adventures of Kai and Zero";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    game_data
        .set_achievements(generate_set())
        .set_rich_presence(generate_rich_presence());

    RustcheevosCli::parse().run(&game_data)
}
