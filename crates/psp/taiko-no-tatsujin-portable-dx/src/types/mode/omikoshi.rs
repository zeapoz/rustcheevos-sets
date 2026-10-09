use std::fmt;

use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef, rich::LookupTable},
};

use crate::{mem, types::profile::Profile, util::PSP_POINTER_MASK};

pub struct OmikoshiMode;

impl OmikoshiMode {
    pub fn number_of_controlled_prefectures() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            mem::omikoshi_number_of_controlled_prefectures(),
        )
    }

    pub fn number_of_disciples_on_hand() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            mem::omikoshi_number_of_disciples_on_hand(),
        )
    }

    pub fn total_disciples() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            add_source!(mem::omikoshi_number_of_missing_disciples()),
            Profile::offset_for_current(),
            mem::omikoshi_number_of_disciples_on_hand()
        )
    }

    pub fn disciples_in_battle() -> Chain<MemoryRef> {
        chain!(
            add_address!(mem::game_mode_specific_data_0x1ffffff().bitwise_and(PSP_POINTER_MASK)),
            add_source!(bits32!(0x100)),
            add_address!(mem::game_mode_specific_data_0x1ffffff().bitwise_and(PSP_POINTER_MASK)),
            bits32!(0x104),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattleOpponent {
    Hokkaido = 0x00,
    Aomori = 0x01,
    Iwate = 0x02,
    Miyagi = 0x03,
    Akita = 0x04,
    Yamagata = 0x05,
    Fukushima = 0x06,
    Ibaraki = 0x07,
    Tochigi = 0x08,
    Gunma = 0x09,
    Saitama = 0x0a,
    Chiba = 0x0b,
    Tokyo = 0x0c,
    Kanagawa = 0x0d,
    Niigata = 0x0e,
    Toyama = 0x0f,
    Ishikawa = 0x10,
    Fukui = 0x11,
    Yamanashi = 0x12,
    Nagano = 0x13,
    Gifu = 0x14,
    Shizuoka = 0x15,
    Aichi = 0x16,
    Mie = 0x17,
    Shiga = 0x18,
    Kyoto = 0x19,
    Osaka = 0x1a,
    Hyogo = 0x1b,
    Nara = 0x1c,
    Wakayama = 0x1d,
    Tottori = 0x1e,
    Shimane = 0x1f,
    Okuyama = 0x20,
    Hiroshima = 0x21,
    Yamaguchi = 0x22,
    Tokushima = 0x23,
    Kagawa = 0x24,
    Ehime = 0x25,
    Kochi = 0x26,
    Fukuoka = 0x27,
    Saga = 0x28,
    Nagasaki = 0x29,
    Kumamoto = 0x2a,
    Oita = 0x2b,
    Miyasaki = 0x2c,
    Kagoshima = 0x2d,
    Okinawa = 0x2e,
    GutsCannon = 0x2f,
    DokonGang = 0x30,
    DokonPlusDokonGang = 0x31,
    GutsJet = 0x32,
    DevilDonChan = 0x33,
    GutsEater = 0x34,
    ArmageDon = 0x35,
}

impl BattleOpponent {
    pub const STATE_FLAG_OFFSET: usize = 0x6;

    pub const BOSS_DEFEATED_FLAG: u8 = 0x2;
    pub const UNLOCKED_FLAG: u8 = 0x4;

    pub fn defeated_stats(&self) -> Chain<MemoryRef> {
        let struct_offset = mem::omikoshi_prefecture_structs() + *self as usize * 32;
        chain!(
            Profile::offset_for_current(),
            bits8!(struct_offset + Self::STATE_FLAG_OFFSET)
        )
    }

    // TODO: Derive macro that generates this definition and a lookup table.
    pub fn lookup_table() -> LookupTable {
        LookupTable::new("OmikoshiOpponent").with_entries(vec![
            (Self::Hokkaido as u32, Self::Hokkaido.lookup_string()),
            (Self::Aomori as u32, Self::Aomori.lookup_string()),
            (Self::Iwate as u32, Self::Iwate.lookup_string()),
            (Self::Miyagi as u32, Self::Miyagi.lookup_string()),
            (Self::Akita as u32, Self::Akita.lookup_string()),
            (Self::Yamagata as u32, Self::Yamagata.lookup_string()),
            (Self::Fukushima as u32, Self::Fukushima.lookup_string()),
            (Self::Ibaraki as u32, Self::Ibaraki.lookup_string()),
            (Self::Tochigi as u32, Self::Tochigi.lookup_string()),
            (Self::Gunma as u32, Self::Gunma.lookup_string()),
            (Self::Saitama as u32, Self::Saitama.lookup_string()),
            (Self::Chiba as u32, Self::Chiba.lookup_string()),
            (Self::Tokyo as u32, Self::Tokyo.lookup_string()),
            (Self::Kanagawa as u32, Self::Kanagawa.lookup_string()),
            (Self::Niigata as u32, Self::Niigata.lookup_string()),
            (Self::Toyama as u32, Self::Toyama.lookup_string()),
            (Self::Ishikawa as u32, Self::Ishikawa.lookup_string()),
            (Self::Fukui as u32, Self::Fukui.lookup_string()),
            (Self::Yamanashi as u32, Self::Yamanashi.lookup_string()),
            (Self::Nagano as u32, Self::Nagano.lookup_string()),
            (Self::Gifu as u32, Self::Gifu.lookup_string()),
            (Self::Shizuoka as u32, Self::Shizuoka.lookup_string()),
            (Self::Aichi as u32, Self::Aichi.lookup_string()),
            (Self::Mie as u32, Self::Mie.lookup_string()),
            (Self::Shiga as u32, Self::Shiga.lookup_string()),
            (Self::Kyoto as u32, Self::Kyoto.lookup_string()),
            (Self::Osaka as u32, Self::Osaka.lookup_string()),
            (Self::Hyogo as u32, Self::Hyogo.lookup_string()),
            (Self::Nara as u32, Self::Nara.lookup_string()),
            (Self::Wakayama as u32, Self::Wakayama.lookup_string()),
            (Self::Tottori as u32, Self::Tottori.lookup_string()),
            (Self::Shimane as u32, Self::Shimane.lookup_string()),
            (Self::Okuyama as u32, Self::Okuyama.lookup_string()),
            (Self::Hiroshima as u32, Self::Hiroshima.lookup_string()),
            (Self::Yamaguchi as u32, Self::Yamaguchi.lookup_string()),
            (Self::Tokushima as u32, Self::Tokushima.lookup_string()),
            (Self::Kagawa as u32, Self::Kagawa.lookup_string()),
            (Self::Ehime as u32, Self::Ehime.lookup_string()),
            (Self::Kochi as u32, Self::Kochi.lookup_string()),
            (Self::Fukuoka as u32, Self::Fukuoka.lookup_string()),
            (Self::Saga as u32, Self::Saga.lookup_string()),
            (Self::Nagasaki as u32, Self::Nagasaki.lookup_string()),
            (Self::Kumamoto as u32, Self::Kumamoto.lookup_string()),
            (Self::Oita as u32, Self::Oita.lookup_string()),
            (Self::Miyasaki as u32, Self::Miyasaki.lookup_string()),
            (Self::Kagoshima as u32, Self::Kagoshima.lookup_string()),
            (Self::Okinawa as u32, Self::Okinawa.lookup_string()),
            (Self::GutsCannon as u32, Self::GutsCannon.lookup_string()),
            (Self::DokonGang as u32, Self::DokonGang.lookup_string()),
            (
                Self::DokonPlusDokonGang as u32,
                Self::DokonPlusDokonGang.lookup_string(),
            ),
            (Self::GutsJet as u32, Self::GutsJet.lookup_string()),
            (
                Self::DevilDonChan as u32,
                Self::DevilDonChan.lookup_string(),
            ),
            (Self::GutsEater as u32, Self::GutsEater.lookup_string()),
            (Self::ArmageDon as u32, Self::ArmageDon.lookup_string()),
        ])
    }

    pub fn lookup_string(&self) -> &'static str {
        match self {
            BattleOpponent::Hokkaido => "Hokkaido Prefecture",
            BattleOpponent::Aomori => "Aomori Prefecture",
            BattleOpponent::Iwate => "Iwate Prefecture",
            BattleOpponent::Miyagi => "Miyagi Prefecture",
            BattleOpponent::Akita => "Akita Prefecture",
            BattleOpponent::Yamagata => "Yamagata Prefecture",
            BattleOpponent::Fukushima => "Fukushima Prefecture",
            BattleOpponent::Ibaraki => "Ibaraki Prefecture",
            BattleOpponent::Tochigi => "Tochigi Prefecture",
            BattleOpponent::Gunma => "Gunma Prefecture",
            BattleOpponent::Saitama => "Saitama Prefecture",
            BattleOpponent::Chiba => "Chiba Prefecture",
            BattleOpponent::Tokyo => "Tokyo Prefecture",
            BattleOpponent::Kanagawa => "Kanagawa Prefecture",
            BattleOpponent::Niigata => "Niigata Prefecture",
            BattleOpponent::Toyama => "Toyama Prefecture",
            BattleOpponent::Ishikawa => "Ishikawa Prefecture",
            BattleOpponent::Fukui => "Fukui Prefecture",
            BattleOpponent::Yamanashi => "Yamanashi Prefecture",
            BattleOpponent::Nagano => "Nagano Prefecture",
            BattleOpponent::Gifu => "Gifu Prefecture",
            BattleOpponent::Shizuoka => "Shizuoka Prefecture",
            BattleOpponent::Aichi => "Aichi Prefecture",
            BattleOpponent::Mie => "Mie Prefecture",
            BattleOpponent::Shiga => "Shiga Prefecture",
            BattleOpponent::Kyoto => "Kyoto Prefecture",
            BattleOpponent::Osaka => "Osaka Prefecture",
            BattleOpponent::Hyogo => "Hyogo Prefecture",
            BattleOpponent::Nara => "Nara Prefecture",
            BattleOpponent::Wakayama => "Wakayama Prefecture",
            BattleOpponent::Tottori => "Tottori Prefecture",
            BattleOpponent::Shimane => "Shimane Prefecture",
            BattleOpponent::Okuyama => "Okuyama Prefecture",
            BattleOpponent::Hiroshima => "Hiroshima Prefecture",
            BattleOpponent::Yamaguchi => "Yamaguchi Prefecture",
            BattleOpponent::Tokushima => "Tokushima Prefecture",
            BattleOpponent::Kagawa => "Kagawa Prefecture",
            BattleOpponent::Ehime => "Ehime Prefecture",
            BattleOpponent::Kochi => "Kochi Prefecture",
            BattleOpponent::Fukuoka => "Fukuoka Prefecture",
            BattleOpponent::Saga => "Saga Prefecture",
            BattleOpponent::Nagasaki => "Nagasaki Prefecture",
            BattleOpponent::Kumamoto => "Kumamoto Prefecture",
            BattleOpponent::Oita => "Oita Prefecture",
            BattleOpponent::Miyasaki => "Miyasaki Prefecture",
            BattleOpponent::Kagoshima => "Kagoshima Prefecture",
            BattleOpponent::Okinawa => "Okinawa Prefecture",
            BattleOpponent::GutsCannon => "Guts Cannon",
            BattleOpponent::DokonGang => "Dokon Gang",
            BattleOpponent::DokonPlusDokonGang => "Dokon + Dokon Gang",
            BattleOpponent::GutsJet => "Guts Jet",
            BattleOpponent::DevilDonChan => "Devil Don-chan",
            BattleOpponent::GutsEater => "Guts Eater",
            BattleOpponent::ArmageDon => "Armage-Don",
        }
    }
}

impl fmt::Display for BattleOpponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lookup_string())
    }
}
