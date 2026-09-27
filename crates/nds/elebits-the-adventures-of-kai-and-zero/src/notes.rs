use rustcheevos::types::note::CodeNote;

use crate::types::omega::Omega;

const CHARGED_WATTS_TEMPLATE: &str = "[32-bit] {omega} - Charged Watts";

const FORM_ENUM_TEMPLATE: &str = r#"[Lower4] {omega} - Form
0x0 = Not Obtained
0x1 = Child
0x2 = Adult"#;

const ID_TEMPLATE: &str = r#"[32-bit] {omega} - Unique Omega ID ({id})
0xff if this Omega has not been obtained yet"#;

/// Generates predictable code notes for the game.
pub fn generate_code_notes() -> Vec<CodeNote> {
    let mut notes = Vec::with_capacity(Omega::ALL.len() * 3);
    for omega in Omega::ALL {
        let charged_watts = CHARGED_WATTS_TEMPLATE.replace("{omega}", &omega.to_string());
        notes.push(CodeNote::new(omega.base_addr(), charged_watts));

        let form = FORM_ENUM_TEMPLATE.replace("{omega}", &omega.to_string());
        notes.push(CodeNote::new(omega.obtained_addr(), form));

        let id = ID_TEMPLATE
            .replace("{omega}", &omega.to_string())
            .replace("{id}", &format!("{:#04x}", omega.id()));
        notes.push(CodeNote::new(omega.id_addr(), id));
    }

    notes
}
