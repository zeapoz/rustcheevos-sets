use leaderboards::generate_leaderboards;
use rich::generate_rich_presence;
use rustcheevos::types::{game::GameData, set::Set};
use rustcheevos_cli::{CliError, RustcheevosCli};
use set::generate_set;

use crate::notes::generate_code_notes;

mod leaderboards;
mod notes;
mod rich;
mod set;
mod types;

const GAME_ID: u32 = 20374;
const GAME_NAME: &str = "Geometry Wars: Galaxies";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    let mut base_set = Set::new_base();
    base_set
        .add_achievements(generate_set())
        .add_leaderboards(generate_leaderboards());

    game_data
        .set_base_set(base_set)
        .set_rich_presence(generate_rich_presence())
        .add_code_notes(generate_code_notes());

    RustcheevosCli::parse().run(&game_data)
}
