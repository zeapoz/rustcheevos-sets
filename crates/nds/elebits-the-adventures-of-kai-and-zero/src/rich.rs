use rustcheevos::prelude::*;
use rustcheevos::types::chain::Chain;
use rustcheevos::types::rich::{BuiltInMacro, LookupTable, MacroRef, RichPresence};

use crate::types::game_state::GameState;
use crate::types::omega::{Omega, OmegaState, active::ActiveOmega};
use crate::types::{game::Game, location::Location};
use crate::utils::{world_battery_bits, world_pink_elebit_bits};
use crate::{mem, types::player::Player};

pub(crate) fn generate_rich_presence() -> RichPresence {
    let mut rich = RichPresence::new();

    let boss_arena_lt = rich.register_lookup(
        LookupTable::new("BossArena").with_entries(Location::BOSS_LOOKUP_ENTRIES),
        mem::current_game_scene(),
    );

    let active_omega = rich.register_lookup(
        LookupTable::new("Omega").with_entries(Omega::LOOKUP_ENTRIES),
        measured!(ActiveOmega::id()),
    );

    let omega_form = rich.register_lookup(
        LookupTable::new("OmegaForm").with_entry((OmegaState::Evolved as u32, " (Adult)")),
        ActiveOmega::form(),
    );

    let action_lt = rich.register_lookup(
        LookupTable::new("GameAction")
            .with_entry((0x0, "adventuring"))
            .with_entry((0x1, "capturing Elebits")),
        capturing_state(),
    );

    let menu = rich.register_lookup(
        LookupTable::new("Menu").with_entries(build_menu_lookup_data()),
        mem::game_state(),
    );

    let current_health = rich.builtin_macro(BuiltInMacro::Number, Player::current_health());
    let max_health = rich.builtin_macro(BuiltInMacro::Number, Player::max_health());
    let watts = rich.builtin_macro(BuiltInMacro::Number, Player::current_watts());

    rich.add_conditional_display(
        mem::dtcm_memory_start_marker().eq(0),
        "Playing Elebits: The Adventures of Kai and Zero in an unsupported emulator | Play in BizHawk to earn achievements",
    );
    rich.add_conditional_display(
        mem::game_state().eq(GameState::InMultiplayerGame.id()),
        format!(
            "Kai and {active_omega}{omega_form} are {action_lt} in a multiplayer game • {watts}w"
        ),
    );
    rich.add_conditional_display(
        mem::game_state().eq(GameState::InBossFight.id()),
        format!("Kai and {active_omega}{omega_form} are fighting against {boss_arena_lt} • {current_health}/{max_health} 🔴"),
    );
    for &world in Location::all_worlds() {
        let num_pink_elebits = num_pink_elebits_macro(world, &mut rich);
        let num_batteries = num_batteries_macro(world, &mut rich);
        rich.add_conditional_display(
            chain!(mem::current_game_scene().eq(world.id())),
            format!("Kai and {active_omega}{omega_form} are {action_lt} in {} | {current_health}/{max_health} 🔴 • {watts}w | {num_pink_elebits}/3 🟣 • {num_batteries}/6 🔋", world.world_name()),
        );
    }
    rich.add_conditional_display(Game::in_menu(), format!("{menu}"));
    rich.add_static_display("Playing Elebits: The Adventures of Kai and Zero");

    rich
}

fn build_menu_lookup_data() -> Vec<(u32, &'static str)> {
    GameState::all_named()
        .iter()
        .map(|s| (*s as u32, s.lookup_value()))
        .collect()
}

fn num_pink_elebits_macro(world: Location, rich: &mut RichPresence) -> MacroRef {
    let bits = world_pink_elebit_bits(world);
    let chain = chain!(
        add_source!(bits[0]),
        add_source!(bits[1]),
        measured!(bits[2]),
    );

    rich.builtin_macro(BuiltInMacro::Number, chain)
}

fn num_batteries_macro(world: Location, rich: &mut RichPresence) -> MacroRef {
    let bits = world_battery_bits(world);
    let chain = chain!(
        add_source!(bits[0]),
        add_source!(bits[1]),
        add_source!(bits[2]),
        add_source!(bits[3]),
        add_source!(bits[4]),
        measured!(bits[5]),
    );

    rich.builtin_macro(BuiltInMacro::Number, chain)
}

fn capturing_state() -> Chain {
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(0x6c)),
        measured!(bits8!(0x48)),
    )
}
