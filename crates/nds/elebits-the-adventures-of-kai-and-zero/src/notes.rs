use rustcheevos::types::note::CodeNote;

use crate::types::omega::Omega;

const CHARGED_WATTS_TEMPLATE: &str = "[32-bit] {omega} - Charged Watts";

const FORM_ENUM_TEMPLATE: &str = r#"[8-bit] {omega} - Form
0x00 = Not Obtained
0x11 = Child
0x22 = Adult
"#;

const ID_TEMPLATE: &str = "[32-bit] {omega} - Unique Omega ID";

/// Generates predictable code notes for the game.
pub fn generate_code_notes() -> Vec<CodeNote> {
    let mut notes = Vec::with_capacity(Omega::all().len() * 3);
    for omega in Omega::all() {
        let charged_watts = CHARGED_WATTS_TEMPLATE.replace("{omega}", &omega.to_string());
        notes.push(CodeNote::new(omega.base_addr(), charged_watts));

        let form = FORM_ENUM_TEMPLATE.replace("{omega}", &omega.to_string());
        notes.push(CodeNote::new(omega.obtained_addr(), form));

        let id = ID_TEMPLATE.replace("{omega}", &omega.to_string());
        notes.push(CodeNote::new(omega.id_addr(), id));
    }

    notes
}
