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
    pub const LOCKED_FLAG: u32 = 0x0;
    pub const UNLOCKED_FLAG: u32 = 0x1;

    pub fn is_locked(self) -> Chain<Condition> {
        self.unlock_state().eq(Self::LOCKED_FLAG)
    }

    pub fn is_unlocked(self) -> Chain<Condition> {
        self.unlock_state().eq(Self::UNLOCKED_FLAG)
    }

    fn unlock_state(self) -> Chain<MemoryRef> {
        let offset = mem::in_game_achievements_unlock_flags() + self as usize;
        chain!(Profile::offset_for_current(), bits8!(offset))
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::FirstClear => "Clear a song in Freeplay mode",
            Self::Earn15SilverCrownsOnKantan => "Earn 15 silver crowns on Kantan",
            Self::Earn30SilverCrownsOnKantan => "Earn 30 silver crowns on Kantan",
            Self::Earn70SilverCrownsOnKantan => "Earn 70 silver crowns on Kantan",
            Self::Earn15SilverCrownsOnFutsuu => "Earn 15 silver crowns on Futsuu",
            Self::Earn30SilverCrownsOnFutsuu => "Earn 30 silver crowns on Futsuu",
            Self::Earn70SilverCrownsOnFutsuu => "Earn 70 silver crowns on Futsuu",
            Self::Earn15SilverCrownsOnMuzukashii => "Earn 15 silver crowns on Muzukashii",
            Self::Earn30SilverCrownsOnMuzukashii => "Earn 30 silver crowns on Muzukashii",
            Self::Earn70SilverCrownsOnMuzukashii => "Earn 70 silver crowns on Muzukashii",
            Self::Earn15SilverCrownsOnOni => "Earn 15 silver crowns on Oni",
            Self::Earn30SilverCrownsOnOni => "Earn 30 silver crowns on Oni",
            Self::Earn70SilverCrownsOnOni => "Earn 70 silver crowns on Oni",
            Self::Accumulate10000HitsOnTheTaikoCounter => {
                "Accumulate 10,000 hits on the Taiko Counter"
            }
            Self::Accumulate20000HitsOnTheTaikoCounter => {
                "Accumulate 20,000 hits on the Taiko Counter"
            }
            Self::Accumulate30000HitsOnTheTaikoCounter => {
                "Accumulate 30,000 hits on the Taiko Counter"
            }
            Self::Accumulate40000HitsOnTheTaikoCounter => {
                "Accumulate 40,000 hits on the Taiko Counter"
            }
            Self::Accumulate50000HitsOnTheTaikoCounter => {
                "Accumulate 50,000 hits on the Taiko Counter"
            }
            Self::Accumulate60000HitsOnTheTaikoCounter => {
                "Accumulate 60,000 hits on the Taiko Counter"
            }
            Self::Accumulate70000HitsOnTheTaikoCounter => {
                "Accumulate 70,000 hits on the Taiko Counter"
            }
            Self::Accumulate76500HitsOnTheTaikoCounter => {
                "Accumulate 76,500 hits on the Taiko Counter"
            }
            Self::Accumulate80000HitsOnTheTaikoCounter => {
                "Accumulate 80,000 hits on the Taiko Counter"
            }
            Self::Accumulate90000HitsOnTheTaikoCounter => {
                "Accumulate 90,000 hits on the Taiko Counter"
            }
            Self::Accumulate99999HitsOnTheTaikoCounter => {
                "Accumulate 99,999 hits on the Taiko Counter"
            }
            Self::Earn10GoldenCrowns => "Earn 10 golden crowns",
            Self::Earn25GoldenCrowns => "Earn 25 golden crowns",
            Self::Earn50GoldenCrowns => "Earn 50 golden crowns",
            Self::Earn75GoldenCrowns => "Earn 75 golden crowns",
            Self::Earn100GoldenCrowns => "Earn 100 golden crowns",
            Self::ClearASongWithAllGreatHitsAndNoMisses => {
                "Clear a song with all great hits and no misses"
            }
            Self::ClearASongWithAllGoodHitsAndNoMisses => {
                "Clear a song with all good hits and no misses"
            }
            Self::ReachAComboOf100HitsOrMore => "Reach a combo of 100",
            Self::ReachAComboOf200HitsOrMore => "Reach a combo of 200",
            Self::ReachAComboOf400HitsOrMore => "Reach a combo of 400",
            Self::ReachAComboOf600HitsOrMore => "Reach a combo of 600",
            Self::GetMoreThan100TotalHitsInASong => "Get more than 100 hits in a song",
            Self::Score500000PointsOrMoreInASong => "Score 500,000 points in a song",
            Self::Clear5DojoTrainingStages => "Clear 5 dojo training stages",
            Self::Clear10DojoTrainingStages => "Clear 10 dojo training stages",
            Self::Clear20DojoTrainingStages => "Clear 20 dojo training stages",
            Self::Clear30DojoTrainingStages => "Clear 30 dojo training stages",
            Self::Clear40DojoTrainingStages => "Clear 40 dojo training stages",
            Self::Clear50DojoTrainingStages => "Clear 50 dojo training stages",
            Self::Control10DojosInOmikoshiMode => "Control 10 dojos in Omikoshi mode",
            Self::Control20DojosInOmikoshiMode => "Control 20 dojos in Omikoshi mode",
            Self::Control30DojosInOmikoshiMode => "Control 30 dojos in Omikoshi mode",
            Self::Control40DojosInOmikoshiMode => "Control 40 dojos in Omikoshi mode",
            Self::ControlAllDojosInOmikoshiMode => "Control all 47 dojos in Omikoshi mode",
            Self::BeatOmikoshiMode => "Beat the Omikoshi mode",
            Self::Beat2OpponentsInARowInBossRushMode => "Beat 2 opponents in a row in Boss Rush",
            Self::Beat5OpponentsInARowInBossRushMode => "Beat 5 opponents in a row in Boss Rush",
            Self::ClearTheBossRushMode => "Clear the Boss Rush mode",
            Self::Gather100OrMoreDrumDisciplesInOmikoshiMode => {
                "Gather 100 or more Drum Disciples in Omikoshi mode"
            }
            Self::Gather200OrMoreDrumDisciplesInOmikoshiMode => {
                "Gather 200 or more Drum Disciples in Omikoshi mode"
            }
            Self::Gather300OrMoreDrumDisciplesInOmikoshiMode => {
                "Gather 300 or more Drum Disciples in Omikoshi mode"
            }
            Self::BeatTheOpponentByMoreThan9MetersInADojoBattle => {
                "Defeat opponent by more than 9m in a dojo battle"
            }
            Self::LoseToTheOpponentByMoreThan9MetersInADojoBattle => {
                "Lose to opponent by more than 9m in a dojo battle"
            }
            Self::Fail10TimesInFreeplay => "Fail 10 times in Freeplay mode",
            Self::Play1WirelessMultiplayerMatch => "Play 1 wireless multiplayer match",
            Self::Play5WirelessMultiplayerMatches => "Play 5 wireless multiplayer matches",
            Self::Play10WirelessMultiplayerMatches => "Play 10 wireless multiplayer matches",
            Self::Play15WirelessMultiplayerMatches => "Play 15 wireless multiplayer matches",
            Self::Play20WirelessMultiplayerMatches => "Play 20 wireless multiplayer matches",
            Self::PlayWirelessMultiplayerMatchesWith5DifferentPeople => {
                "Play multiplayer with 5 different people"
            }
            Self::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan => {
                "Golden crown on The world is all one!! (Kantan)"
            }
            Self::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu => {
                "Golden crown on The world is all one!! (Futsuu)"
            }
            Self::EarnGoldenCrownOnMonsterHunterMedleyOnKantan => {
                "Golden crown on Monster Hunter Medley (Kantan)"
            }
            Self::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu => {
                "Golden crown on Monster Hunter Medley (Futsuu)"
            }
            Self::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan => {
                "Golden crown on Hatsune Miku no Gekishou (Kantan)"
            }
            Self::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu => {
                "Golden crown on Hatsune Miku no Gekishou (Futsuu)"
            }
            Self::EarnGoldenCrownOnMachineGunKissOnKantan => {
                "Golden crown on MachineGun Kiss (Kantan)"
            }
            Self::EarnGoldenCrownOnMachineGunKissOnFutsuu => {
                "Golden crown on MachineGun Kiss (Futsuu)"
            }
            Self::ReachAComboOf500OrMoreInMedleyMode => "Reach a combo of 500 in Medley mode",
            Self::ReachAComboOf1000OrMoreInMedleyMode => "Reach a combo of 1000 in Medley mode",
            Self::ReachAComboOf1500OrMoreInMedleyMode => "Reach a combo of 1500 in Medley mode",
            Self::ReachAComboOf2000OrMoreInMedleyMode => "Reach a combo of 2000 in Medley mode",
            Self::DrumrollATotalOf500TimesOrMoreInMedleyMode => "Drumroll 500 times in Medley mode",
            Self::Score3000000PointsOrMoreInMedleyMode => "Score 3,000,000 points in Medley mode",
        }
    }

    pub const fn points(self) -> u32 {
        match self {
            Self::FirstClear => 10,
            Self::Earn15SilverCrownsOnKantan => 10,
            Self::Earn30SilverCrownsOnKantan => 10,
            Self::Earn70SilverCrownsOnKantan => 15,
            Self::Earn15SilverCrownsOnFutsuu => 10,
            Self::Earn30SilverCrownsOnFutsuu => 10,
            Self::Earn70SilverCrownsOnFutsuu => 15,
            Self::Earn15SilverCrownsOnMuzukashii => 10,
            Self::Earn30SilverCrownsOnMuzukashii => 10,
            Self::Earn70SilverCrownsOnMuzukashii => 15,
            Self::Earn15SilverCrownsOnOni => 10,
            Self::Earn30SilverCrownsOnOni => 10,
            Self::Earn70SilverCrownsOnOni => 15,
            Self::Accumulate10000HitsOnTheTaikoCounter => 10,
            Self::Accumulate20000HitsOnTheTaikoCounter => 10,
            Self::Accumulate30000HitsOnTheTaikoCounter => 10,
            Self::Accumulate40000HitsOnTheTaikoCounter => 15,
            Self::Accumulate50000HitsOnTheTaikoCounter => 15,
            Self::Accumulate60000HitsOnTheTaikoCounter => 20,
            Self::Accumulate70000HitsOnTheTaikoCounter => 20,
            Self::Accumulate76500HitsOnTheTaikoCounter => 20,
            Self::Accumulate80000HitsOnTheTaikoCounter => 20,
            Self::Accumulate90000HitsOnTheTaikoCounter => 20,
            Self::Accumulate99999HitsOnTheTaikoCounter => 25,
            Self::Earn10GoldenCrowns => 10,
            Self::Earn25GoldenCrowns => 10,
            Self::Earn50GoldenCrowns => 15,
            Self::Earn75GoldenCrowns => 15,
            Self::Earn100GoldenCrowns => 20,
            Self::ClearASongWithAllGreatHitsAndNoMisses => 15,
            Self::ClearASongWithAllGoodHitsAndNoMisses => 15,
            Self::ReachAComboOf100HitsOrMore => 10,
            Self::ReachAComboOf200HitsOrMore => 10,
            Self::ReachAComboOf400HitsOrMore => 15,
            Self::ReachAComboOf600HitsOrMore => 20,
            Self::GetMoreThan100TotalHitsInASong => 15,
            Self::Score500000PointsOrMoreInASong => 15,
            Self::Clear5DojoTrainingStages => 10,
            Self::Clear10DojoTrainingStages => 10,
            Self::Clear20DojoTrainingStages => 15,
            Self::Clear30DojoTrainingStages => 15,
            Self::Clear40DojoTrainingStages => 15,
            Self::Clear50DojoTrainingStages => 20,
            Self::Control10DojosInOmikoshiMode => 15,
            Self::Control20DojosInOmikoshiMode => 20,
            Self::Control30DojosInOmikoshiMode => 20,
            Self::Control40DojosInOmikoshiMode => 20,
            Self::ControlAllDojosInOmikoshiMode => 25,
            Self::BeatOmikoshiMode => 20,
            Self::Beat2OpponentsInARowInBossRushMode => 15,
            Self::Beat5OpponentsInARowInBossRushMode => 20,
            Self::ClearTheBossRushMode => 20,
            Self::Gather100OrMoreDrumDisciplesInOmikoshiMode => 15,
            Self::Gather200OrMoreDrumDisciplesInOmikoshiMode => 20,
            Self::Gather300OrMoreDrumDisciplesInOmikoshiMode => 20,
            Self::BeatTheOpponentByMoreThan9MetersInADojoBattle => 20,
            Self::LoseToTheOpponentByMoreThan9MetersInADojoBattle => 15,
            Self::Fail10TimesInFreeplay => 10,
            Self::Play1WirelessMultiplayerMatch => 10,
            Self::Play5WirelessMultiplayerMatches => 10,
            Self::Play10WirelessMultiplayerMatches => 15,
            Self::Play15WirelessMultiplayerMatches => 15,
            Self::Play20WirelessMultiplayerMatches => 15,
            Self::PlayWirelessMultiplayerMatchesWith5DifferentPeople => 20,
            Self::EarnAGoldenCrownOnTheWorldIsAllOneOnKantan => 15,
            Self::EarnAGoldenCrownOnTheWorldIsAllOneOnFutsuu => 15,
            Self::EarnGoldenCrownOnMonsterHunterMedleyOnKantan => 15,
            Self::EarnGoldenCrownOnMonsterHunterMedleyOnFutsuu => 15,
            Self::EarnGoldenCrownOnHatsuneMikuNoGekishouOnKantan => 15,
            Self::EarnGoldenCrownOnHatsuneMikuNoGekishouOnFutsuu => 15,
            Self::EarnGoldenCrownOnMachineGunKissOnKantan => 15,
            Self::EarnGoldenCrownOnMachineGunKissOnFutsuu => 15,
            Self::ReachAComboOf500OrMoreInMedleyMode => 15,
            Self::ReachAComboOf1000OrMoreInMedleyMode => 15,
            Self::ReachAComboOf1500OrMoreInMedleyMode => 20,
            Self::ReachAComboOf2000OrMoreInMedleyMode => 20,
            Self::DrumrollATotalOf500TimesOrMoreInMedleyMode => 15,
            Self::Score3000000PointsOrMoreInMedleyMode => 20,
        }
    }
}
