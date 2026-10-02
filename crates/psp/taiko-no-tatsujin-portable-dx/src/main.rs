const GAME_ID: u32 = 19377;
const GAME_NAME: &str = "Taiko no Tatsujin: Portable DX";

fn main() -> Result<(), CliError> {
    let game_data = GameData::new(GAME_ID, GAME_NAME);
    RustcheevosCli::parse().run(&game_data)
}
