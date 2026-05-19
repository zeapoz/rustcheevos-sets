use rustcheevos::types::note::CodeNote;

use crate::types::planet::Planet;

const HIGH_SCORE_TEMPLATE: &str = "[32-bit] {planet} - High Score (Divided by 25)";

const STATUSES_TEMPLATE: &str = r#"[6-byte Array][Upper4] {planet} - Statuses
0x0 = Unlocked
0x1 = Bronze
0x2 = Silver
0x3 = Gold
0x4 = Locked

Offsets:
| 0x0 = [Upper4] Profile 1 Status
| 0x2 = [Upper4] Profile 2 Status
| 0x4 = [Upper4] Profile 3 Status"#;

/// Generates predictable code notes for the game.
pub fn generate_code_notes() -> Vec<CodeNote> {
    let mut notes = Vec::with_capacity(Planet::all().len() * 2);
    for planet in Planet::all() {
        let high_score = HIGH_SCORE_TEMPLATE.replace("{planet}", planet.name);
        notes.push(CodeNote::new(planet.file_data_addr(), high_score));

        let statuses = STATUSES_TEMPLATE.replace("{planet}", planet.name);
        notes.push(CodeNote::new(planet.status_addr(), statuses));
    }

    notes
}
