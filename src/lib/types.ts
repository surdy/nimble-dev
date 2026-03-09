// ── Command schema ─────────────────────────────────────────────────────────────
// These types mirror the Rust structs in src-tauri/src/commands.rs exactly.
// Keep them in sync when the schema changes.

export interface OpenUrlConfig {
  url: string;
}

export interface PasteTextConfig {
  text: string;
}

export type Action =
  | { type: "open_url"; config: OpenUrlConfig }
  | { type: "paste_text"; config: PasteTextConfig };

export interface Command {
  phrase: string;
  title: string;
  action: Action;
}
