use leaderboards::generate_leaderboards;
use rich::generate_rich_presence;
use rustcheevos::types::game::GameData;
use rustcheevos_cli::{CliError, RustcheevosCli};
use set::generate_set;

mod leaderboards;
mod rich;
mod set;
mod types;

const GAME_ID: u32 = 20374;
const GAME_NAME: &str = "Geometry Wars: Galaxies";

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(GAME_ID, GAME_NAME);

    game_data
        .set_achievements(generate_set())
        .set_leaderboards(generate_leaderboards())
        .set_rich_presence(generate_rich_presence());

    RustcheevosCli::parse().run(&game_data)
}
