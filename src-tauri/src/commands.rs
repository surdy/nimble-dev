use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Seeded into commands.yaml on first launch if no file exists.
const DEFAULT_COMMANDS_YAML: &str = r#"# Contexts Launcher — commands.yaml
#
# Define your personal commands here. Each entry requires:
#   phrase  – the words you type in the launcher to match this command
#   title   – the human-readable label shown as the result title
#   action  – what happens when you execute the command
#
# Action types:
#   open_url
#     config:
#       url: <URL>          Use {param} anywhere in the URL to substitute text
#                           the user types after the command phrase.
#
#   paste_text
#     config:
#       text: <string>      Pasted into the previously focused application.

- phrase: open google
  title: Open Google
  action:
    type: open_url
    config:
      url: https://www.google.com

- phrase: search google
  title: Search Google for…
  action:
    type: open_url
    config:
      url: https://www.google.com/search?q={param}

- phrase: open github
  title: Open GitHub
  action:
    type: open_url
    config:
      url: https://github.com

- phrase: paste email
  title: Paste email address
  action:
    type: paste_text
    config:
      text: hello@example.com

- phrase: paste greeting
  title: Paste polite greeting
  action:
    type: paste_text
    config:
      text: "Hi,\n\nThank you for reaching out.\n\nBest regards"
"#;

// ── Schema ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenUrlConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasteTextConfig {
    pub text: String,
}

/// The action executed when a command is selected.
/// Serialised as `{ type: "open_url"|"paste_text", config: { … } }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub enum Action {
    OpenUrl(OpenUrlConfig),
    PasteText(PasteTextConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub phrase: String,
    pub title: String,
    pub action: Action,
}

// ── Loader ─────────────────────────────────────────────────────────────────────

/// Ensure `config_dir` exists, seed `commands.yaml` if absent, then parse and
/// return the command list.
pub fn load_from_dir(config_dir: &Path) -> Result<Vec<Command>, String> {
    fs::create_dir_all(config_dir)
        .map_err(|e| format!("Could not create config directory: {e}"))?;

    let commands_path = config_dir.join("commands.yaml");

    if !commands_path.exists() {
        fs::write(&commands_path, DEFAULT_COMMANDS_YAML)
            .map_err(|e| format!("Could not write default commands.yaml: {e}"))?;
    }

    let yaml = fs::read_to_string(&commands_path)
        .map_err(|e| format!("Could not read commands.yaml: {e}"))?;

    serde_yaml::from_str::<Vec<Command>>(&yaml)
        .map_err(|e| format!("Could not parse commands.yaml: {e}"))
}
