use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTextConfig {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowListConfig {
    /// Name of the list file (without extension) inside `config_dir/lists/`.
    pub list: String,
}

/// The action executed when a command is selected.
/// Serialised as `{ type: "open_url"|"paste_text"|"copy_text"|"show_list", config: { … } }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub enum Action {
    OpenUrl(OpenUrlConfig),
    PasteText(PasteTextConfig),
    CopyText(CopyTextConfig),
    ShowList(ShowListConfig),
}

/// A single item in a named list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub title: String,
    pub subtext: Option<String>,
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

/// A warning produced when two YAML files define the same command phrase.
/// The older file (by mtime) wins; the newer file is ignored.
#[derive(Debug, Clone, Serialize)]
pub struct DuplicateWarning {
    /// The conflicting phrase (lowercased).
    pub phrase: String,
    /// Config-dir-relative path of the file whose command was kept.
    pub kept: String,
    /// Config-dir-relative path of the file whose command was ignored.
    pub ignored: String,
}

/// The result of loading commands from the config directory.
#[derive(Debug, Clone, Serialize)]
pub struct LoadResult {
    pub commands: Vec<Command>,
    pub duplicates: Vec<DuplicateWarning>,
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
    (
        "examples/copy-email.yaml",
        r#"phrase: copy email
title: Copy email address
action:
  type: copy_text
  config:
    text: hello@example.com
"#,
    ),
    (
        "examples/show-team-emails.yaml",
        r#"phrase: team emails
title: Team email addresses
action:
  type: show_list
  config:
    list: team-emails
"#,
    ),
];

// ── List loader ────────────────────────────────────────────────────────────────

/// Load a named list from `config_dir/lists/<list_name>.yaml`.
///
/// `list_name` must be a plain filename (no path separators or `..` components).
/// Returns `Err` if the name is unsafe, the file is missing, or parsing fails.
pub fn load_list(config_dir: &Path, list_name: &str) -> Result<Vec<ListItem>, String> {
    // Security: reject names that could escape the lists/ directory.
    if list_name.contains('/') || list_name.contains('\\') || list_name.contains("..") {
        return Err(format!("Invalid list name: {list_name:?}"));
    }

    let path = config_dir.join("lists").join(format!("{list_name}.yaml"));
    let yaml = fs::read_to_string(&path)
        .map_err(|e| format!("Could not read list {:?}: {e}", path.display()))?;
    serde_yaml::from_str::<Vec<ListItem>>(&yaml)
        .map_err(|e| format!("Could not parse list {:?}: {e}", path.display()))
}

// ── Command loader ─────────────────────────────────────────────────────────────

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
/// Files are processed oldest-first (by mtime) so that the original command
/// always wins when duplicates are present. Files that fail to parse are
/// skipped (with an eprintln warning) so one malformed file does not prevent
/// others from loading.
pub fn load_from_dir(config_dir: &Path) -> Result<LoadResult, String> {
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

    // Sort files oldest-first by mtime; use path as a stable tiebreaker.
    let mut yaml_files = collect_yaml_files(config_dir);
    yaml_files.sort_by(|a, b| {
        let mtime_a = fs::metadata(a).and_then(|m| m.modified()).ok();
        let mtime_b = fs::metadata(b).and_then(|m| m.modified()).ok();
        mtime_a.cmp(&mtime_b).then_with(|| a.cmp(b))
    });

    let mut commands = Vec::new();
    let mut duplicates = Vec::new();
    // Maps lowercase phrase → relative path of the file that claimed it.
    let mut seen: HashMap<String, String> = HashMap::new();

    for path in yaml_files {
        // Use a config-dir-relative path for human-readable warnings.
        let display = path
            .strip_prefix(config_dir)
            .unwrap_or(&path)
            .display()
            .to_string();

        match fs::read_to_string(&path) {
            Err(e) => eprintln!("[ctx] could not read {}: {e}", path.display()),
            Ok(yaml) => match serde_yaml::from_str::<Command>(&yaml) {
                Err(e) => eprintln!("[ctx] could not parse {}: {e}", path.display()),
                Ok(cmd) if !cmd.enabled => {} // disabled — silently skip
                Ok(cmd) => {
                    let key = cmd.phrase.to_lowercase();
                    if let Some(winner) = seen.get(&key) {
                        eprintln!(
                            "[ctx] duplicate phrase {:?} in {display}, kept {winner}",
                            cmd.phrase
                        );
                        duplicates.push(DuplicateWarning {
                            phrase: cmd.phrase.clone(),
                            kept: winner.clone(),
                            ignored: display,
                        });
                    } else {
                        seen.insert(key, display);
                        commands.push(cmd);
                    }
                }
            },
        }
    }

    Ok(LoadResult { commands, duplicates })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_yaml(dir: &TempDir, relative: &str, content: &str) {
        let path = dir.path().join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    // ── YAML parsing ──────────────────────────────────────────────────────────

    #[test]
    fn parses_open_url_command() {
        let dir = TempDir::new().unwrap();
        write_yaml(
            &dir,
            "open-google.yaml",
            "phrase: open google\ntitle: Open Google\naction:\n  type: open_url\n  config:\n    url: https://www.google.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].phrase, "open google");
        assert!(matches!(result.commands[0].action, Action::OpenUrl(_)));
    }

    #[test]
    fn parses_paste_text_command() {
        let dir = TempDir::new().unwrap();
        write_yaml(
            &dir,
            "paste.yaml",
            "phrase: paste email\ntitle: Paste email\naction:\n  type: paste_text\n  config:\n    text: hello@example.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert_eq!(result.commands.len(), 1);
        assert!(matches!(result.commands[0].action, Action::PasteText(_)));
    }

    #[test]
    fn parses_copy_text_command() {
        let dir = TempDir::new().unwrap();
        write_yaml(
            &dir,
            "copy.yaml",
            "phrase: copy email\ntitle: Copy email\naction:\n  type: copy_text\n  config:\n    text: hello@example.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert_eq!(result.commands.len(), 1);
        assert!(matches!(result.commands[0].action, Action::CopyText(_)));
    }

    // ── Deduplication ─────────────────────────────────────────────────────────

    #[test]
    fn duplicate_phrase_emits_warning_and_keeps_first() {
        let dir = TempDir::new().unwrap();
        // Write two files with identical phrases; the oldest-mtime file wins.
        // Easiest way to guarantee mtime ordering is to sleep briefly, but that
        // is fragile in CI. Instead we rely on alphabetical sort as a tiebreaker
        // by naming them a.yaml (kept) and b.yaml (ignored).
        write_yaml(
            &dir,
            "a.yaml",
            "phrase: open google\ntitle: First\naction:\n  type: open_url\n  config:\n    url: https://www.google.com\n",
        );
        write_yaml(
            &dir,
            "b.yaml",
            "phrase: open google\ntitle: Second\naction:\n  type: open_url\n  config:\n    url: https://duckduckgo.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert_eq!(result.commands.len(), 1, "only one command should survive");
        assert_eq!(result.duplicates.len(), 1, "one duplicate warning expected");
        assert_eq!(result.duplicates[0].phrase, "open google");
    }

    // ── Disabled commands ─────────────────────────────────────────────────────

    #[test]
    fn disabled_command_is_skipped() {
        let dir = TempDir::new().unwrap();
        write_yaml(
            &dir,
            "disabled.yaml",
            "phrase: hidden cmd\ntitle: Hidden\nenabled: false\naction:\n  type: open_url\n  config:\n    url: https://example.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert!(result.commands.is_empty(), "disabled command must be filtered out");
    }

    // ── Malformed YAML ────────────────────────────────────────────────────────

    #[test]
    fn malformed_yaml_is_skipped_without_panic() {
        let dir = TempDir::new().unwrap();
        write_yaml(&dir, "bad.yaml", "this: is: not: valid: yaml: ::::\n");
        // A second, valid file should still load fine
        write_yaml(
            &dir,
            "good.yaml",
            "phrase: open google\ntitle: Open Google\naction:\n  type: open_url\n  config:\n    url: https://www.google.com\n",
        );
        let result = load_from_dir(dir.path()).unwrap();
        assert_eq!(result.commands.len(), 1, "only the valid command should load");
    }

    // ── load_list ─────────────────────────────────────────────────────────────

    fn write_list(dir: &TempDir, name: &str, content: &str) {
        let path = dir.path().join("lists").join(format!("{name}.yaml"));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    #[test]
    fn load_list_returns_items() {
        let dir = TempDir::new().unwrap();
        write_list(
            &dir,
            "emails",
            "- title: Alice\n  subtext: alice@example.com\n- title: Bob\n  subtext: bob@example.com\n",
        );
        let items = load_list(dir.path(), "emails").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Alice");
        assert_eq!(items[0].subtext.as_deref(), Some("alice@example.com"));
        assert_eq!(items[1].title, "Bob");
    }

    #[test]
    fn load_list_item_without_subtext() {
        let dir = TempDir::new().unwrap();
        write_list(&dir, "names", "- title: Alice\n- title: Bob\n");
        let items = load_list(dir.path(), "names").unwrap();
        assert_eq!(items.len(), 2);
        assert!(items[0].subtext.is_none());
    }

    #[test]
    fn load_list_missing_file_returns_err() {
        let dir = TempDir::new().unwrap();
        fs::create_dir_all(dir.path().join("lists")).unwrap();
        assert!(load_list(dir.path(), "nonexistent").is_err());
    }

    #[test]
    fn load_list_rejects_path_traversal_dotdot() {
        let dir = TempDir::new().unwrap();
        assert!(load_list(dir.path(), "../secret").is_err());
    }

    #[test]
    fn load_list_rejects_path_with_slash() {
        let dir = TempDir::new().unwrap();
        assert!(load_list(dir.path(), "sub/file").is_err());
    }
}
