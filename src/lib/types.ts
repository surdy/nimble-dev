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

/** The built-in action applied to each value returned by a script_action script. */
export type ResultAction = "open_url" | "paste_text" | "copy_text";

export interface ScriptActionConfig {
  /** Name of the script file (without path) inside `config_dir/scripts/`. */
  script: string;
  /** Controls whether a suffix typed after the phrase is passed as an argument. Defaults to "none". */
  arg?: ArgMode;
  /** Built-in action to apply to every value the script returns. */
  result_action: ResultAction;
  /** Text prepended to each value when result_action is paste_text or copy_text. */
  prefix?: string;
  /** Text appended to each value when result_action is paste_text or copy_text. */
  suffix?: string;
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
  | { type: "dynamic_list"; config: DynamicListConfig }
  | { type: "script_action"; config: ScriptActionConfig };

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

export interface ReservedPhraseWarning {
  /** The rejected phrase as written in the YAML file. */
  phrase: string;
  /** Config-dir-relative path of the offending file. */
  file: string;
}

/** Payload returned by list_commands and emitted as commands://reloaded. */
export interface CommandsPayload {
  commands: Command[];
  duplicates: DuplicateWarning[];
  /** Commands rejected because their phrase starts with the reserved `ctx` prefix. */
  reserved: ReservedPhraseWarning[];
}
