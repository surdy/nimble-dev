use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

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
    /// Whether this command is active. Omitting the field in YAML is the same
    /// as `enabled: true`. Disabled commands are filtered out at load time and
    /// never sent to the frontend.
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub action: Action,
}

fn default_true() -> bool {
    true
}

// ── Seed files written on first launch ────────────────────────────────────────
// Each entry is (relative path inside config dir, YAML content).
// Paths may include subdirectories — they mirror the kind of structure a user
// would organise their own commands into.

static SEED_FILES: &[(&str, &str)] = &[
    (
        "examples/open-google.yaml",
        r#"phrase: open google
title: Open Google
action:
  type: open_url
  config:
    url: https://www.google.com
"#,
    ),
    (
        "examples/search-google.yaml",
        r#"phrase: search google
title: Search Google for…
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
"#,
    ),
    (
        "examples/open-github.yaml",
        r#"phrase: open github
title: Open GitHub
action:
  type: open_url
  config:
    url: https://github.com
"#,
    ),
    (
        "examples/paste-email.yaml",
        r#"phrase: paste email
title: Paste email address
action:
  type: paste_text
  config:
    text: hello@example.com
"#,
    ),
    (
        "examples/paste-greeting.yaml",
        r#"phrase: paste greeting
title: Paste polite greeting
action:
  type: paste_text
  config:
    text: |
      Hi,

      Thank you for reaching out.

      Best regards
"#,
    ),
];

// ── Loader ─────────────────────────────────────────────────────────────────────

/// Collect all `.yaml` / `.yml` file paths under `config_dir` recursively.
fn collect_yaml_files(config_dir: &Path) -> Vec<std::path::PathBuf> {
    WalkDir::new(config_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|x| x.to_str()),
                Some("yaml") | Some("yml")
            )
        })
        .map(|e| e.into_path())
        .collect()
}

/// Ensure `config_dir` exists. If no YAML files are found, seed the example
/// commands. Then walk the tree, parse every `.yaml`/`.yml` file as a single
/// `Command`, and return the collected list.
/// Files that fail to parse are skipped (with an eprintln warning) so one
/// malformed file does not prevent others from loading.
pub fn load_from_dir(config_dir: &Path) -> Result<Vec<Command>, String> {
    fs::create_dir_all(config_dir)
        .map_err(|e| format!("Could not create config directory: {e}"))?;

    // Seed examples if the directory contains no YAML files yet.
    if collect_yaml_files(config_dir).is_empty() {
        for (rel_path, content) in SEED_FILES {
            let dest = config_dir.join(rel_path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Could not create {}: {e}", parent.display()))?;
            }
            fs::write(&dest, content)
                .map_err(|e| format!("Could not write {}: {e}", dest.display()))?;
        }
    }

    let mut commands = Vec::new();
    for path in collect_yaml_files(config_dir) {
        match fs::read_to_string(&path) {
            Err(e) => eprintln!("[contexts] could not read {}: {e}", path.display()),
            Ok(yaml) => match serde_yaml::from_str::<Command>(&yaml) {
                Err(e) => eprintln!("[contexts] could not parse {}: {e}", path.display()),
                Ok(cmd) if cmd.enabled => commands.push(cmd),
                Ok(_) => {} // disabled — silently skip
            },
        }
    }

    Ok(commands)
}
