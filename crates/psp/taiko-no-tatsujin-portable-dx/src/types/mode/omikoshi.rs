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
            add_address!(mem::game_mode_specific_data_pointer().bitwise_and(PSP_POINTER_MASK)),
            bits32!(0x100),
        )
    }
}

pub enum BattleOpponents {
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

impl BattleOpponents {
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
            BattleOpponents::Hokkaido => "Hokkaido Prefecture",
            BattleOpponents::Aomori => "Aomori Prefecture",
            BattleOpponents::Iwate => "Iwate Prefecture",
            BattleOpponents::Miyagi => "Miyagi Prefecture",
            BattleOpponents::Akita => "Akita Prefecture",
            BattleOpponents::Yamagata => "Yamagata Prefecture",
            BattleOpponents::Fukushima => "Fukushima Prefecture",
            BattleOpponents::Ibaraki => "Ibaraki Prefecture",
            BattleOpponents::Tochigi => "Tochigi Prefecture",
            BattleOpponents::Gunma => "Gunma Prefecture",
            BattleOpponents::Saitama => "Saitama Prefecture",
            BattleOpponents::Chiba => "Chiba Prefecture",
            BattleOpponents::Tokyo => "Tokyo Prefecture",
            BattleOpponents::Kanagawa => "Kanagawa Prefecture",
            BattleOpponents::Niigata => "Niigata Prefecture",
            BattleOpponents::Toyama => "Toyama Prefecture",
            BattleOpponents::Ishikawa => "Ishikawa Prefecture",
            BattleOpponents::Fukui => "Fukui Prefecture",
            BattleOpponents::Yamanashi => "Yamanashi Prefecture",
            BattleOpponents::Nagano => "Nagano Prefecture",
            BattleOpponents::Gifu => "Gifu Prefecture",
            BattleOpponents::Shizuoka => "Shizuoka Prefecture",
            BattleOpponents::Aichi => "Aichi Prefecture",
            BattleOpponents::Mie => "Mie Prefecture",
            BattleOpponents::Shiga => "Shiga Prefecture",
            BattleOpponents::Kyoto => "Kyoto Prefecture",
            BattleOpponents::Osaka => "Osaka Prefecture",
            BattleOpponents::Hyogo => "Hyogo Prefecture",
            BattleOpponents::Nara => "Nara Prefecture",
            BattleOpponents::Wakayama => "Wakayama Prefecture",
            BattleOpponents::Tottori => "Tottori Prefecture",
            BattleOpponents::Shimane => "Shimane Prefecture",
            BattleOpponents::Okuyama => "Okuyama Prefecture",
            BattleOpponents::Hiroshima => "Hiroshima Prefecture",
            BattleOpponents::Yamaguchi => "Yamaguchi Prefecture",
            BattleOpponents::Tokushima => "Tokushima Prefecture",
            BattleOpponents::Kagawa => "Kagawa Prefecture",
            BattleOpponents::Ehime => "Ehime Prefecture",
            BattleOpponents::Kochi => "Kochi Prefecture",
            BattleOpponents::Fukuoka => "Fukuoka Prefecture",
            BattleOpponents::Saga => "Saga Prefecture",
            BattleOpponents::Nagasaki => "Nagasaki Prefecture",
            BattleOpponents::Kumamoto => "Kumamoto Prefecture",
            BattleOpponents::Oita => "Oita Prefecture",
            BattleOpponents::Miyasaki => "Miyasaki Prefecture",
            BattleOpponents::Kagoshima => "Kagoshima Prefecture",
            BattleOpponents::Okinawa => "Okinawa Prefecture",
            BattleOpponents::GutsCannon => "Guts Cannon",
            BattleOpponents::DokonGang => "Dokon Gang",
            BattleOpponents::DokonPlusDokonGang => "Dokon + Dokon Gang",
            BattleOpponents::GutsJet => "Guts Jet",
            BattleOpponents::DevilDonChan => "Devil Don-chan",
            BattleOpponents::GutsEater => "Guts Eater",
            BattleOpponents::ArmageDon => "Armage-Don",
        }
    }
}
