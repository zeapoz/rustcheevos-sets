use rustcheevos::types::rich::{BuiltInMacro, LookupTable, MacroRef, RichPresence};
use rustcheevos::{add_source, chain, measured, prelude::*};

use crate::types::game_state::GameState;
use crate::types::omega::{Omega, OmegaState};
use crate::types::{game::Game, location::Location};
use crate::utils::{bit_at, world_battery_bits, world_pink_elebit_bits};
use crate::{mem, types::player::Player};

pub(crate) fn generate_rich_presence() -> RichPresence {
    let mut rich = RichPresence::new();

    let boss_arena_lt = rich.register_lookup(
        LookupTable::new("BossArena").with_entries(Location::BOSS_LOOKUP_ENTRIES),
        mem::current_game_scene(),
    );

    let active_omega = rich.register_lookup(
        LookupTable::new("Omega").with_entries(Omega::LOOKUP_ENTRIES),
        measured!(Omega::active_id()),
    );

    let omega_form = rich.register_lookup(
        LookupTable::new("OmegaForm").with_entry((OmegaState::Evolved as u32, " (Adult)")),
        Omega::active_form(),
    );

    let menu = rich.register_lookup(
        LookupTable::new("Menu").with_entries(build_menu_lookup_data()),
        mem::game_state(),
    );

    let current_health = rich.builtin_macro(BuiltInMacro::Number, Player::current_health());
    let max_health = rich.builtin_macro(BuiltInMacro::Number, Player::max_health());
    let watts = rich.builtin_macro(BuiltInMacro::Number, mem::current_watts());

    rich.add_conditional_display(
        mem::dtcm_memory_start_marker().eq(0),
        "Playing Elebits: The Adventures of Kai and Zero in an unsupported emulator | Play in BizHawk to earn achievements",
    );
    rich.add_conditional_display(
        GameState::in_boss_fight(),
        format!("Kai and {active_omega} are fighting against {boss_arena_lt} • {current_health}/{max_health} 🔴"),
    );
    for &world in Location::all_worlds() {
        let num_pink_elebits = num_pink_elebits_macro(world, &mut rich);
        let num_batteries = num_batteries_macro(world, &mut rich);
        rich.add_conditional_display(
            chain!(GameState::in_overworld(), mem::current_game_scene().eq(world.id())),
            format!("Kai and {active_omega}{omega_form} are adventuring in {} | {current_health}/{max_health} 🔴 • {watts}W | {num_pink_elebits}/3 🟣 • {num_batteries}/6 🔋", world.world_name()),
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
        add_source!(bit_at(bits[0].0, bits[0].1)),
        add_source!(bit_at(bits[1].0, bits[1].1)),
        measured!(bit_at(bits[2].0, bits[2].1)),
    );

    rich.builtin_macro(BuiltInMacro::Number, chain)
}

fn num_batteries_macro(world: Location, rich: &mut RichPresence) -> MacroRef {
    let bits = world_battery_bits(world);
    let chain = chain!(
        add_source!(bit_at(bits[0].0, bits[0].1)),
        add_source!(bit_at(bits[1].0, bits[1].1)),
        add_source!(bit_at(bits[2].0, bits[2].1)),
        add_source!(bit_at(bits[3].0, bits[3].1)),
        add_source!(bit_at(bits[4].0, bits[4].1)),
        measured!(bit_at(bits[5].0, bits[5].1)),
    );

    rich.builtin_macro(BuiltInMacro::Number, chain)
}
