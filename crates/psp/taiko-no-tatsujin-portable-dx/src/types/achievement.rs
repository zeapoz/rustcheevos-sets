use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;
use rustcheevos::types::{chain::Chain, memory::MemoryRef};

use crate::mem;
use crate::types::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameAchievement {
    FirstClear = 0x00,
    Earn15SilverCrownsOnKantan = 0x01,
    Earn30SilverCrownsOnKantan = 0x02,
    Earn70SilverCrownsOnKantan = 0x03,
    Earn15SilverCrownsOnFutsuu = 0x04,
    Earn30SilverCrownsOnFutsuu = 0x05,
    Earn70SilverCrownsOnFutsuu = 0x06,
    Earn15SilverCrownsOnMuzukashii = 0x07,
    Earn30SilverCrownsOnMuzukashii = 0x08,
    Earn70SilverCrownsOnMuzukashii = 0x09,
    Earn15SilverCrownsOnOni = 0x0a,
    Earn30SilverCrownsOnOni = 0x0b,
    Earn70SilverCrownsOnOni = 0x0c,
    Accumulate10000HitsOnTheTaikoCounter = 0x0d,
    Accumulate20000HitsOnTheTaikoCounter = 0x0e,
    Accumulate30000HitsOnTheTaikoCounter = 0x0f,
    Accumulate40000HitsOnTheTaikoCounter = 0x10,
    Accumulate50000HitsOnTheTaikoCounter = 0x11,
    Accumulate60000HitsOnTheTaikoCounter = 0x12,
    Accumulate70000HitsOnTheTaikoCounter = 0x13,
    Accumulate76500HitsOnTheTaikoCounter = 0x14,
    Accumulate80000HitsOnTheTaikoCounter = 0x15,
    Accumulate90000HitsOnTheTaikoCounter = 0x16,
    Accumulate99999HitsOnTheTaikoCounter = 0x17,
    Earn10GoldenCrowns = 0x18,
    Earn25GoldenCrowns = 0x19,
    Earn50GoldenCrowns = 0x1a,
    Earn75GoldenCrowns = 0x1b,
    Earn100GoldenCrowns = 0x1c,
    ClearASongWithAllGreatHitsAndNoMisses = 0x1d,
    ClearASongWithAllGoodHitsAndNoMisses = 0x1e,
    ReachAComboOf100HitsOrMore = 0x1f,
    ReachAComboOf200HitsOrMore = 0x20,
    ReachAComboOf400HitsOrMore = 0x21,
    ReachAComboOf600HitsOrMore = 0x22,
    GetMoreThan100TotalHitsInASong = 0x23,
    Score500000PointsOrMoreInASong = 0x24,
    Clear5DojoTrainingStages = 0x25,
    Clear10DojoTrainingStages = 0x26,
    Clear20DojoTrainingStages = 0x27,
    Clear30DojoTrainingStages = 0x28,
    Clear40DojoTrainingStages = 0x29,
    Clear50DojoTrainingStages = 0x2a,
    Control10DojosInOmikoshiMode = 0x2b,
    Control20DojosInOmikoshiMode = 0x2c,
    Control30DojosInOmikoshiMode = 0x2d,
    Control40DojosInOmikoshiMode = 0x2e,
    ControlAllDojosInOmikoshiMode = 0x2f,
    BeatOmikoshiMode = 0x30,
    Beat2OpponentsInARowInBossRushMode = 0x31,
    Beat5OpponentsInARowInBossRushMode = 0x32,
    ClearTheBossRushMode = 0x33,
    Gather100OrMoreDrumDisciplesInOmikoshiMode = 0x34,
    Gather200OrMoreDrumDisciplesInOmikoshiMode = 0x35,
    Gather300OrMoreDrumDisciplesInOmikoshiMode = 0x36,
    BeatTheOpponentByMoreThan9MetersInADojoBattle = 0x37,
    LoseToTheOpponentByMoreThan9MetersInADojoBattle = 0x38,
    Fail10TimesInFreeplay = 0x39,
    Play1WirelessMultiplayerMatch = 0x3a,
    Play5WirelessMultiplayerMatches = 0x3b,
    Play10WirelessMultiplayerMatches = 0x3c,
    Play15WirelessMultiplayerMatches = 0x3d,
    Play20WirelessMultiplayerMatches = 0x3e,
    PlayWirelessMultiplayerMatchesWith5DifferentPeople = 0x3f,
    EarnAGoldenCrownOnTheWorldIsAllOneOnKantan = 0x40,
    EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu = 0x41,
    EarnGoldenCrownOnMonsterHunterMedleyOnKantan = 0x42,
    EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu = 0x43,
    EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan = 0x44,
    EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu = 0x45,
    EarnGoldenCrownOnMachineGunKissOnKantan = 0x46,
    EarnGoldenCrownOnMachineGunKissOnFutsuu = 0x47,
    ReachAComboOf500OrMoreInMedleyMode = 0x48,
    ReachAComboOf1000OrMoreInMedleyMode = 0x49,
    ReachAComboOf1500OrMoreInMedleyMode = 0x4a,
    ReachAComboOf2000OrMoreInMedleyMode = 0x4b,
    DrumrollATotalOf500TimesOrMoreInMedleyMode = 0x4c,
    Score3000000PointsOrMoreInMedleyMode = 0x4d,
}

impl InGameAchievement {
    pub fn is_locked(self) -> Chain<Condition> {
        self.unlock_state().eq(0x0)
    }

    pub fn is_unlocked(self) -> Chain<Condition> {
        self.unlock_state().eq(0x1)
    }

    fn unlock_state(self) -> Chain<MemoryRef> {
        let offset = mem::in_game_achievements_unlock_flags() + self as usize;
        chain!(Profile::offset_for_current(), bits8!(offset))
    }
}
