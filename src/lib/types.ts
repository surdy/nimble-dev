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

export interface ShowListConfig {
  list: string;
}

export interface ListItem {
  title: string;
  subtext?: string;
}

export type Action =
  | { type: "open_url"; config: OpenUrlConfig }
  | { type: "paste_text"; config: PasteTextConfig }
  | { type: "copy_text"; config: CopyTextConfig }
  | { type: "show_list"; config: ShowListConfig };

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
