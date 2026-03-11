// ── Command schema ─────────────────────────────────────────────────────────────
// These types mirror the Rust structs in src-tauri/src/commands.rs exactly.
// Keep them in sync when the schema changes.

export interface OpenUrlConfig {
  url: string;
}

export interface PasteTextConfig {
  text: string;
}

export interface CopyTextConfig {
  text: string;
}

export type ItemAction = "paste_text" | "copy_text" | "open_url";

export interface StaticListConfig {
  list: string;
  /** If absent, selecting an item only dismisses the launcher. */
  item_action?: ItemAction;
}

/** Controls when a dynamic_list script is invoked and whether a suffix arg is passed. */
export type ArgMode = "none" | "optional" | "required";

export interface DynamicListConfig {
  /** Name of the script file (without path) inside `config_dir/scripts/`. */
  script: string;
  /** Defaults to "none" if absent. */
  arg?: ArgMode;
  /** If absent, selecting an item only dismisses the launcher. */
  item_action?: ItemAction;
}

export interface ListItem {
  title: string;
  subtext?: string;
}

export type Action =
  | { type: "open_url"; config: OpenUrlConfig }
  | { type: "paste_text"; config: PasteTextConfig }
  | { type: "copy_text"; config: CopyTextConfig }
  | { type: "static_list"; config: StaticListConfig }
  | { type: "dynamic_list"; config: DynamicListConfig };

export interface Command {
  phrase: string;
  title: string;
  action: Action;
}

export interface DuplicateWarning {
  phrase: string;
  /** Config-dir-relative path of the file whose command was kept. */
  kept: string;
  /** Config-dir-relative path of the file whose command was ignored. */
  ignored: string;
}

/** Payload returned by list_commands and emitted as commands://reloaded. */
export interface CommandsPayload {
  commands: Command[];
  duplicates: DuplicateWarning[];
}
