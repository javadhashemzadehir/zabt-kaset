use crate::organizer::{self, OrganizerContext};

#[tauri::command]
pub fn organizer_expand(template: String, context: OrganizerContext) -> Result<String, String> {
    organizer::expand_template(&template, &context).map_err(|e| e.to_command_error())
}
