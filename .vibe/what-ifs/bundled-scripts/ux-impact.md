# UX Impact: Bundled Scripts

## What Changes from the User's Perspective

### Creating a new dynamic list command with a script

| Step | Current | With co-located scripts |
|------|---------|------------------------|
| 1 | Create `commands/my-command.yaml` | Create `commands/my-command/command.yaml` |
| 2 | Create `scripts/my-script.sh` | Create `commands/my-command/my-script.sh` |
| 3 | `chmod +x scripts/my-script.sh` | `chmod +x commands/my-command/my-script.sh` |
| 4 | Reference `script: my-script.sh` in YAML | Reference `script: ./my-script.sh` in YAML |

**Net effect**: Same number of files, but they live together. Slightly more directory structure.

### Sharing/distributing a command

| Scenario | Current | With co-located |
|----------|---------|----------------|
| Share via gist/zip | Must include YAML from `commands/` AND script from `scripts/` separately | Zip one directory — done |
| Clone from a recipe repo | Copy files to two locations | Copy one directory into `commands/` |
| "Install a community command" | Multi-step, error-prone | Drop a folder into `commands/` |

This is the **strongest argument** for co-location — command portability.

### Day-to-day editing

| Scenario | Current | With co-located |
|----------|---------|----------------|
| Edit command + script | Open two files in different directories | Open two files in the same directory |
| "Which script does this command use?" | Read YAML, find `script: X`, navigate to `scripts/X` | It's right next to the YAML |
| "Which commands use this script?" | Grep all command YAMLs for the script name | Same (no change for shared scripts) |

### Discoverability

| Aspect | Current | With co-located |
|--------|---------|----------------|
| "Where are all my scripts?" | `scripts/` directory — one flat list | Some in `scripts/`, some scattered across `commands/` subdirectories |
| "What does this command package include?" | Must check two directories | Self-evident from the subdirectory |

### File watcher behaviour

Currently Nimble watches `commands/` for `.yaml`/`.yml` files only. With co-located scripts:
- The watcher would see `.sh`, `.py`, `.js` files in `commands/` — it must ignore them (not try to parse as commands)
- When a script file changes, the watcher doesn't need to do anything — scripts are invoked fresh each time
- No functional change, but the watcher filter logic becomes slightly more nuanced

---

## Security Considerations

### Current model (simple, restrictive)
- `script:` must be a plain filename — no `/`, `\`, `..`
- Resolution: `config_dir/scripts/<name>` — one fixed directory
- Easy to audit: look at `scripts/` to see everything that can run

### Co-located model (more flexible, more surface area)
- `./` prefix signals relative resolution from the YAML file's directory
- Must still reject `../` traversal (e.g., `script: ./../../../etc/evil.sh`)
- Canonicalization needed: resolve the path, then verify it's still within `commands/`
- Harder to audit: scripts can be anywhere in the `commands/` tree

### Mitigation
- Only allow `./filename` — reject any `./path/to/file` patterns
- After resolution, verify the final path is inside `config_dir/commands/`
- Log a warning if a script in `commands/` is not executable

---

## The `scripts/` Directory

- Remains fully functional and the default resolution path
- Still the right place for shared scripts used by multiple commands
- Not deprecated — it's the simpler model that most users should default to
- Co-located scripts are a power-user opt-in for when portability matters

---

## Risk: Directory Proliferation

Every co-located command becomes a subdirectory:

```
commands/
  open-github.yaml                 ← simple commands stay as files
  search-google.yaml
  say-hello/                       ← scripted commands become directories
    command.yaml
    hello.sh
  paste-timestamp/
    command.yaml
    timestamp.sh
  find-file/
    command.yaml
    find-file.sh
```

This mixes files and directories in `commands/`, which is slightly messier than the current flat-files-only approach. Not a dealbreaker, but worth noting — the directory listing becomes less scannable.

---

## Verdict

| Factor | Assessment |
|--------|-----------|
| Simplicity | Slight decrease — two resolution strategies instead of one |
| Portability | Clear improvement — single-directory distribution |
| Security | Slight decrease — more complex path validation |
| Day-to-day editing | Slight improvement — related files are closer |
| Shared scripts | No change — `scripts/` handles this already |
| Overall | **Positive for power users who share/distribute commands; neutral-to-slightly-negative for most users** |
