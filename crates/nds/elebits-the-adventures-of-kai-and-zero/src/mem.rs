use rustcheevos::types::memory::MemoryRef;
use rustcheevos::{bits8, bits16, bits32};

/// [24-bit][Pointer] Game Data Struct
/// +0x6c: [24-bit][Pointer] Read-Only Player Data
/// ++0x28: [32-bit] Capture Timer
/// ++0x2c: [32-bit] Current Health
/// ++0x30: [32-bit] Max Health
/// ++0x48: [8-bit] Capturing Flag
/// ++0x84: [32-bit] Boss Fight Timer
///
/// +0x70: [24-bit][Pointer]
/// ++0x19248: [32-bit] Stunned Elebits During Capture
/// ++0x19258: [8-bit] Number of Green Elebits On-Screen
/// ++0x19259: [8-bit] Number of Red Elebits On-Screen
/// ++0x1925A: [8-bit] Number of Blue Elebits On-Screen
/// ++0x1925B: [8-bit] Number of Purple Elebits On-Screen
/// ++0x1925E: [8-bit] Number of Yellow Elebits On-Screen
///
/// +0x7c: [24-bit][Pointer] Boss Data
/// Only non-null in boss fights
/// ++0xa0: [24-bit][Pointer]
/// +++0xc: [16-bit] Boss Health Denominator
/// +++0x10: [16-bit] Boss Health Numerator
///
/// Power Omega:
/// +++0x2ac: [32-bit] Attack State
/// Phase 1:
/// ...0x0 = Throwing
/// ...0x1 = Flying
/// Phase 2:
/// ...0x0 = Idling
/// ...0x1 = Moving
/// ...0x2 = Flying
/// ...0x3 = Chasing
/// ...0x4 = Vulnerable
/// +++0x2b6: [16-bit] Number Of Bounces Left
/// Vulnerable when set to 0
///
/// X Fire Omega:
/// +++0x2f0: [16-bit] Attack State
/// ...0x1 = Idling
/// ...0x2 = Grabbed
/// ...0x3 = Fell
/// ...0x4 = Fire Ball
/// ...0x5 = Pillar of Lava
/// ...0x6 = Vulnerable
/// ...0x7 = Ascending
/// ...0x9 = Row of Fire
/// ...0xb = Hovering
/// ...0xc = Throwing Fire
/// +++0x2fa: [16-bit] Pillar of Lava Attack Flag
///
/// X Ice Omega:
/// ++0x110: [16-bit] Next Attack Pattern
/// ++0x112: [16-bit] Snake Attack Counter
/// Number of times the snake has attacked
///
/// Leo:
/// ++0xf4: [16-bit] Next Attack Pattern
///
/// +0x80: [24-bit][Pointer] Read-Only Boss Fight Player Data
/// Only non-null in boss fights
/// ++0x2c: [32-bit] Current Health
/// ++0x30: [32-bit] Max Health
///
/// +0x90: [24-bit][Pointer] Text And Hud Elements
/// ++0x18: [24-bit][Pointer] Combo Text
/// Only works when not transitioning
/// +++0x474: [32-bit] Number of Watts Gained Through Last Combo
/// Persists until a new combo overwrites this or a screen transition occurs
/// +++0x494: [32-bit] Combo Number from Last Capture
/// Persists until a new combo overwrites this or a screen transition occurs
pub const fn game_data_struct() -> usize {
    0xe3b98
}

/// [16-bit] Touchpad X-Position
pub const fn touchpad_x_position() -> MemoryRef {
    bits16!(0xe91cc)
}

/// [16-bit] Touchpad Y-Position
pub const fn touchpad_y_position() -> MemoryRef {
    bits16!(0xe91ce)
}

/// [8-bit][Bitflags] Input Map
/// Bit0 = A
/// Bit1 = B
/// Bit2 = Select
/// Bit3 = Start
/// Bit4 = D-Pad Right
/// Bit5 = D-Pad Left
/// Bit6 = D-Pad Up
/// Bit7 = D-Pad Down
pub const fn input_map_1() -> MemoryRef {
    bits8!(0xe91ec)
}

/// [8-bit][Bitflags] Input Map
/// Bit0 = R
/// Bit1 = L
/// Bit2 = X
/// Bit3 = Y
pub const fn input_map_2() -> MemoryRef {
    bits8!(0xe91ed)
}

/// [8-bit][Bool] In-Game Flag
/// 0x0 = In-Game
/// 0x1 = In Menu
pub const fn in_game_flag() -> MemoryRef {
    bits8!(0x10bc90)
}

/// [32-bit] Current Game Scene
/// 0x11 = Power Omega Boss Arena
/// 0x12 = Earth Omega Boss Arena
/// 0x13 = X Fire Omega Boss Arena
/// 0x14 = X Ice Omega Boss Arena
/// 0x15 = X Earth Omega Boss Arena
/// 0x16 = X Water Omega Boss Arena
/// 0x17 = Mobius First Phase Boss Arena
/// 0x18 = Leo Omega Boss Arena
/// 0x19 = Mobius Second Phase Arena
/// 0x1a = Elebit Forest
/// 0x1b = Elebit Mines
/// 0x1c = Resort Island
/// 0x1d = Ice World
/// 0x1e = Ruined World
/// 0x1f = Sea Temple
/// 0x20 = Libra of Crystal
/// 0xffffffff = Uninitialized
pub const fn current_game_scene() -> MemoryRef {
    bits32!(0x1186d8)
}

/// [32-bit] Game State
/// 0x00 = Startup
/// 0x01 = Menu Cutscene
/// 0x02 = Intro Cutscene
/// 0x03 = Main Menu
/// 0x04 = File Select
/// 0x05 = File Configuration
/// 0x06 = File Menu
/// 0x07 = Extras
/// 0x08 = Multiplayer Menu
/// 0x09 = Multicard-Play
/// 0x0a = Multiplayer via Wi-Fi Menu
/// 0x0b = Download Additional Omegas
/// 0x0c = Overworld
/// 0x0e = In Boss Fight
/// 0x10 = Ending Cutscene
/// 0x11 = Transitioning World Cutscene
/// 0x12 = Credits
/// 0x13 = Game Over
pub const fn game_state() -> MemoryRef {
    bits32!(0x1186e4)
}

/// [64-bit][ASCII] Active File Name
pub const fn active_file_name() -> usize {
    0x14339c
}

/// [32-bit] Active Message Speed
/// Normal = 0x0
/// Fast = 0x1
/// Slow = 0x2
pub const fn active_message_speed() -> MemoryRef {
    bits32!(0x1442b8)
}

/// [32-bit] Active Language
/// Values other than specified default to Japanese (Kanji)
///
/// 0x0 = Japanese (Kanji)
/// 0x1 = Japanese (No Kanji)
/// 0x2-0x3 = English
/// 0x4-0x5 = French
/// 0x6 = German
/// 0x7 = Italian
/// 0x8 = Spanish
pub const fn active_language() -> MemoryRef {
    bits32!(0x1442bc)
}

/// [32-bit] Active Control Settings - Movement
/// 0x0 = Stylus & D-Pad
/// 0x1 = Stylus
/// 0x2 = D-Pad
pub const fn active_control_settings_movement() -> MemoryRef {
    bits32!(0x1442c0)
}

/// [32-bit] Active Control Settings - Switch Kai/Omega
/// 0x0 = Stylus & Right/Left Bumpers
/// 0x1 = Stylus
/// 0x2 = Right/Left Bumpers
pub const fn active_control_settings_switch_kai_omega() -> MemoryRef {
    bits32!(0x1442c4)
}

/// [8-bit] Wi-Fi Connecting Flag
/// 0x0 = Not Connecting
/// 0x1 = Connecting
pub const fn wi_fi_connecting_flag() -> MemoryRef {
    bits8!(0x244c20)
}

/// [32-bit] DTCM Memory Start Marker
/// Used to check that the emulator properly exposes the DTCM memory required for this set to function
pub const fn dtcm_memory_start_marker() -> MemoryRef {
    bits32!(0x1000000)
}

/// [32-bit] Current Watts
pub const fn current_watts() -> MemoryRef {
    bits32!(0x10001a4)
}

/// [32-bit] Max Watts
pub const fn max_watts() -> MemoryRef {
    bits32!(0x10001a8)
}

/// [32-bit] Loaded Watts
/// Set to the maximum amount of Watts that have been recorded for this file. Specific values unlock bonus Omegas when reloading the save, these are as follows:
/// 0x9c4 = Takosuke
/// 0x1388 = Penta
/// 0x1d4c = Twinbee
/// 0x2710 = Moai
pub const fn loaded_watts() -> MemoryRef {
    bits32!(0x10001ac)
}

/// [8-bit] Current Health
pub const fn current_health() -> MemoryRef {
    bits8!(0x10001b0)
}

/// [8-bit] Max Health
pub const fn max_health() -> MemoryRef {
    bits8!(0x10001b1)
}

/// [60 Bytes][Array] Active Omega Party
/// Ordered vector of 5 Omegas representing the active party layout
/// - Refer to 0x1000580 for the Omega struct layout
pub const fn active_omega_party() -> usize {
    0x10001c4
}

/// [32-bit] Player X-Position
pub const fn player_x_position() -> MemoryRef {
    bits32!(0x1000218)
}

/// [32-bit] Player Y-Position
pub const fn player_y_position() -> MemoryRef {
    bits32!(0x100021c)
}

/// [32-bit] Player Logical Z-Elevation Level
pub const fn player_logical_z_elevation_level() -> MemoryRef {
    bits32!(0x1000220)
}

/// [32-bit] Echo of Active Player X-Position
pub const fn echo_of_active_player_x_position() -> MemoryRef {
    bits32!(0x1000224)
}

/// [32-bit] Echo of Active Player Y-Position
pub const fn echo_of_active_player_y_position() -> MemoryRef {
    bits32!(0x1000228)
}

/// [32-bit] Player Facing Direction
/// 0x0 = South
/// 0x1 = Southeast
/// 0x2 = East
/// 0x3 = Northeast
/// 0x4 = North
/// 0x5 = Northwest
/// 0x6 = West
/// 0x7 = Southwest
pub const fn player_facing_direction() -> MemoryRef {
    bits32!(0x1000230)
}

/// [32-bit] Active Omega X-Position
pub const fn active_omega_x_position() -> MemoryRef {
    bits32!(0x1000234)
}

/// [32-bit] Active Omega Y-Position
pub const fn active_omega_y_position() -> MemoryRef {
    bits32!(0x1000238)
}

/// [32-bit] Echo of Active Omega X-Position
pub const fn echo_of_active_omega_x_position() -> MemoryRef {
    bits32!(0x1000240)
}

/// [32-bit] Echo of Active Omega Y-Position
pub const fn echo_of_active_omega_y_position() -> MemoryRef {
    bits32!(0x1000244)
}

/// [16-bit] Fever Laser x2 Timer
/// Decrements by 2 every other frame
///
/// 0x0 = Not Active
/// 0x1+ = Active
pub const fn fever_laser_x2_timer() -> MemoryRef {
    bits16!(0x100024c)
}

/// [16-bit] Fever Laser x3 Timer
/// Decrements by 2 every other frame
///
/// 0x0 = Not Active
/// 0x1+ = Active
pub const fn fever_laser_x3_timer() -> MemoryRef {
    bits16!(0x100024e)
}

/// [16-bit] Wide Lock Laser Timer
/// Decrements by 2 every other frame
///
/// 0x0 = Not Active
/// 0x1+ = Active
pub const fn wide_lock_laser_timer() -> MemoryRef {
    bits16!(0x1000250)
}

/// [16-bit] Trace Laser Timer
/// Decrements by 2 every other frame
///
/// 0x0 = Not Active
/// 0x1+ = Active
pub const fn trace_laser_timer() -> MemoryRef {
    bits16!(0x1000252)
}

/// [16-bit] Capture Mode Timer
/// Starts at 0x0 and increments by 1 every other frame
///
/// Stops at:
/// No modifier = 0x64
/// Big Red = c8
pub const fn capture_mode_timer() -> MemoryRef {
    bits16!(0x1000254)
}

/// [32-bit] Active Omega Facing Direction
/// 0x0 = South
/// 0x1 = Southeast
/// 0x2 = East
/// 0x3 = Northeast
/// 0x4 = North
/// 0x5 = Northwest
/// 0x6 = West
/// 0x7 = Southwest
pub const fn active_omega_facing_direction() -> MemoryRef {
    bits32!(0x1000258)
}

/// [8-bit] Active Omega Index
/// The index of the active Omega in the active party array
pub const fn active_omega_index() -> MemoryRef {
    bits8!(0x1000260)
}

/// [8-bit][Bool] Omega Leading Flag
pub const fn omega_leading_flag() -> MemoryRef {
    bits8!(0x1000267)
}

/// [8-bit][Bool] Omega Action Flag
pub const fn omega_action_flag() -> MemoryRef {
    bits8!(0x1000268)
}

/// [32-bit] Camera X-Offset
pub const fn camera_x_offset() -> MemoryRef {
    bits32!(0x1000530)
}

/// [32-bit] Camera Y-Offset
pub const fn camera_y_offset() -> MemoryRef {
    bits32!(0x1000534)
}

/// [32-bit] Loading Zone X-Position
/// The position that Kai will end up on when entering a new area
pub const fn loading_zone_x_position() -> MemoryRef {
    bits32!(0x1000544)
}

/// [32-bit] Loading Zone Y-Position
/// The position that Kai will end up on when entering a new area
pub const fn loading_zone_y_position() -> MemoryRef {
    bits32!(0x1000548)
}

/// [32-bit] Sub-Scene ID
/// 0x03 = Power Omega Boss Fight
/// 0x12 = Earth Omega Boss Fight
/// 0x37 = X Fire Omega Boss Fight
/// 0x6b = X Ice Omega Boss Fight
/// 0x6d = Leo Boss Fight
/// 0x88 = X Earth Omega Boss Fight
/// 0x97 = X Water Omega Boss FIght
pub const fn sub_scene_id() -> MemoryRef {
    bits32!(0x100055c)
}

/// [8-bit][Bool] Transitioning Flag
pub const fn transitioning_flag() -> MemoryRef {
    bits8!(0x100056c)
}

/// [8-bit][Bool] Interaction/Cutscene Flag
pub const fn interaction_cutscene_flag() -> MemoryRef {
    bits8!(0x100056d)
}

/// [8-bit][Bool] Saving Flag
pub const fn saving_flag() -> MemoryRef {
    bits8!(0x100056e)
}

/// [8-bit][Bool] In-Game Menu Flag
pub const fn in_game_menu_flag() -> MemoryRef {
    bits8!(0x100056f)
}

/// [32-bit] Primary Event State
/// Controls the global event state, values 0x5 and above unlock "Select World"
///
/// 0x1 = Elebit Forest
/// 0x2 = Elebit Mine
/// 0x3 = Resort Island
/// 0x4 = Ice World
/// 0x5 = Ruined World
/// 0x6 = Sea Temple
/// 0x7 = Libra of Crystal
pub const fn primary_event_state() -> MemoryRef {
    bits32!(0x1000570)
}

/// [32-bit] Secondary Event State
/// Controls the secondary event state, mainly within the current world
///
/// Elebit Forest:
/// 0x0 = Intro
/// 0x1 = Tutorial
/// 0x2 = Fire Omega
/// 0x3 = Ice Omega
/// 0x4 = Unused
/// 0x5 = Warp Omega
/// 0x6 = Power Omega
/// 0x7 = Elebit Mines Map
///
/// Elebit Mines:
/// 0x0 = Intro
/// 0x1 = Pickel Gang
/// 0x2 = Melody Omega
/// 0x3 = Magnet Omega
/// 0x4 = Radar Omega
/// 0x5 = Minecart Puzzles
/// 0x6 = Earth Omega Boss
/// 0x7 = Resort Island Map
///
/// Resort Island:
/// 0x0 = Intro
/// 0x1 = Boat
/// 0x2 = Wind, X Power, Surf, Sponge, and Water Omegas
/// 0x3 = Engine Room and X Fire Omega Boss
/// 0x4 = Ice World Map
///
/// Ice World:
/// 0x0 = Intro
/// 0x1 = X Wind Omega
/// 0x2 = Leo Boss
/// 0x3 = Secret Hideout
///
/// Ruined World:
/// 0x0 = Intro
/// 0x1 = Speed Omega
/// 0x2 = X Earth Boss
/// 0x3 = Elebit Forest Revisited
/// 0x4 = Resort Island Revisited
/// 0x5 = Sea Temple Map
///
/// Sea Temple:
/// 0x0 = Intro
/// 0x1 = X Magnet Omega
/// 0x2 = X Water Omega Boss
/// 0x3 = Elebit Mines Revisited
/// 0x4 = Ice World Revisited
/// 0x5 = Libra of Crystal Map
///
/// Libra of Crystal:
/// 0x0 = Intro
/// 0x1 = Mirror Omega, X Melody Omega, Time Omega
/// 0x2 = Mobius Boss
/// 0x3 = End
pub const fn secondary_event_state() -> MemoryRef {
    bits32!(0x1000574)
}

/// [32-bit] Tertiary Event State
/// Controls the cutscene/event state within world events
pub const fn tertiary_event_state() -> MemoryRef {
    bits32!(0x1000578)
}

/// [32-bit] Currently Selected File
/// 0x0 = File 1
/// 0x1 = File 2
pub const fn currently_selected_file() -> MemoryRef {
    bits32!(0x100057c)
}

/// [480 Bytes][Array] Vector of Owned Omegas
/// Each item is a 96-bit struct containing the following fields:
/// [32-bit] Charged Watts
/// [16-bit] Magic Value (0x00d2)
/// [8-bit] Form
/// .0x00 = Unowned
/// .0x11 = Child
/// .0x22 = Adult
/// [8-bit] Magic Value (0x08 if owned, 0x00 otherwise)
/// [32-bit] Omega ID
///
/// Items are ordered after their IDs.
///
/// Omega IDs:
/// 0x00 = Zero
/// 0x01 = Mobius
/// 0x02 = Ice
/// 0x03 = X Ice
/// 0x04 = Mirror
/// 0x05 = Fire
/// 0x06 = X Fire
/// 0x07 = Water
/// 0x08 = X Water
/// 0x09 = Sponge
/// 0x0a = Earth
/// 0x0b = X Earth
/// 0x0c = Magnet
/// 0x0d = X Magnet
/// 0x0e = Wind
/// 0x0f = X Wind
/// 0x10 = Time
/// 0x11 = Power
/// 0x12 = X Power
/// 0x13 = Surf
/// 0x14 = Flight
/// 0x15 = Speed
/// 0x16 = Melody
/// 0x17 = X Melody
/// 0x18 = Warp
/// 0x19 = Radar
/// 0x1a = Blizzard
/// 0x1b = Flame
/// 0x1c = Aqua
/// 0x1d = Land
/// 0x1e = Storm
/// 0x1f = Strong
/// 0x20 = Night
/// 0x21 = TwinBee
/// 0x22 = Penta
/// 0x23 = Takosuke
/// 0x24 = Dewy
/// 0x25 = Moai
/// 0x26 = Big Green
/// 0x27 = Big Red
pub const fn vector_of_owned_omegas() -> usize {
    0x1000580
}

/// [8-bit][Bitflags] Item Collected Flags
/// Bit0 = Fever Laser x2
/// Bit1 = Fever Laser x3
/// Bit2 = Wide Lock Laser
/// Bit3 = Trace Laser
/// Bit5 = Durability Recovery Item
/// Bit6 = Full Durability Recovery Item
pub const fn item_collected_flags() -> usize {
    0x1000760
}

/// [8-bit][Bitflags] Diary Scrap Flags
/// Bit1 = Elebits Village
/// Bit2 = Elebits Mine
/// Bit3 = Resort Island
/// Bit4 = Ice World
/// Bit5 = Ruined World
/// Bit6 = Sea Temple
/// Bit7 = Libra of Crystal
pub const fn diary_scrap_flags() -> MemoryRef {
    bits8!(0x1000761)
}

/// [32-bit][Bitflags] Pink Elebit Flags
/// Ordered vector with each set of three bits representing the collected state of the Pink Elebits per world
///
/// Bits 0-2 = Elebits Village
/// Bits 3-5 = Elebits Mine
/// Bits 6-8 = Resort Island
/// Bits 9-11 = Ice World
/// Bits 12-14 = Ruined World
/// Bits 15-17 = Sea Temple
/// Bits 18-20 = Libra of Crystal
/// Bits 21+ Unused
pub const fn pink_elebit_flags() -> usize {
    0x1000764
}

/// [32-bit][Bitflags] Yellow Battery Flags
/// Ordered vector with each set of three bits representing the collected state of the Yellow Batteries per world
///
/// Bits 0-3 = Elebits Village
/// Bits 4-7 = Elebits Mine
/// Bits 8-11 = Resort Island
/// Bits 12-15 = Ice World
/// Bits 16-19 = Ruined World
/// Bits 20-23 = Sea Temple
/// Bits 24-27 = Libra of Crystal
/// Bits 28+ Unused
pub const fn yellow_battery_flags() -> usize {
    0x1000768
}

/// [16-bit][Bitflags] Red Battery Flags
/// Ordered vector with each set of two bits representing the collected state of the Red Batteries per world
///
/// Bits 0-1 = Elebits Village
/// Bits 2-3 = Elebits Mine
/// Bits 4-5 = Resort Island
/// Bits 6-7 = Ice World
/// Bits 8-9 = Ruined World
/// Bits 10-11 = Sea Temple
/// Bits 12-13 = Libra of Crystal
/// Bits 14+ Unused
pub const fn red_battery_flags() -> usize {
    0x100076c
}

/// [16-bit][Bitflags] Guard Booster Flags
/// Ordered vector with each set of two bits representing the collected state of the Guard Boosters per world
///
/// Bits 0-1 = Elebits Village
/// Bits 2-3 = Elebits Mine
/// Bits 4-5 = Resort Island
/// Bits 6-7 = Ice World
/// Bits 8-9 = Ruined World
/// Bits 10-11 = Sea Temple
/// Bits 12-13 = Libra of Crystal
/// Bits 14+ Unused
pub const fn guard_booster_flags() -> usize {
    0x100076e
}

/// [32-bit] Playing Time of Active File
/// Increments by 2 every other frame
pub const fn playing_time_of_active_file() -> MemoryRef {
    bits32!(0x1000780)
}
