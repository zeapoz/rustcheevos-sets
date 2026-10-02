use rustcheevos::types::game::GameData;
use rustcheevos_cli::{CliError, RustcheevosCli};

use crate::{rich::generate_rich_presence, set::core::generate_set};

mod mem;
mod rich;
mod set;
mod types;

const GAME_ID: u32 = 19377;
const GAME_NAME: &str = "Taiko no Tatsujin: Portable DX";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    game_data
        .add_achievements(generate_set())
        .set_rich_presence(generate_rich_presence());

    RustcheevosCli::parse().run(&game_data)
}
