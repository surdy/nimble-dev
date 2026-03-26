# Security Analysis: GitHub Repos as Command Sources

## Threat Model

Adding remote repos as command sources introduces Nimble's first **remote code execution surface**. This document maps the threats and potential mitigations.

---

## Threat 1: Malicious scripts in public repos

**Vector:** A user subscribes to a seemingly-useful public repo. The repo contains or later pushes scripts that exfiltrate data, install malware, or modify system files.

**Impact:** High. Scripts run with the user's full privileges.

**Mitigations:**
| Mitigation | Effort | Effectiveness |
|-----------|--------|---------------|
| `scripts: deny` setting per-source | Low | High — blocks the vector entirely |
| Script review prompt on first sync | Medium | Medium — users may click "Allow all" |
| Hash pinning (lock to a specific commit SHA) | Medium | High — prevents supply chain updates |
| Sandboxing scripts (e.g., macOS App Sandbox, seccomp) | Very High | Very High — but breaks many scripts |
| Code signing (require scripts to be signed) | Very High | Medium — adoption barrier |

**Recommended:** Default to `scripts: deny` for all remote sources. The user must explicitly opt in per-source.

---

## Threat 2: Dependency confusion / typosquatting

**Vector:** A user intends to subscribe to `surdy/nimble-commands` but types `surdy/nimble-command` (a malicious lookalike repo).

**Impact:** Same as Threat 1 if the malicious repo contains scripts.

**Mitigations:**
- Confirm the exact repo URL on first sync with a dialog
- Show repo description, star count, and last-updated date for verification
- Maintain a curated list of "verified" repos (community directory)

---

## Threat 3: Token scope overexposure

**Vector:** User sets `token_env: GITHUB_TOKEN` and the token has `repo`, `write:packages`, or other broad scopes. If Nimble's process is compromised (via a malicious script from another source), the attacker can read the env var and access all repos the token grants.

**Impact:** Medium-High. Credential theft leading to repo access.

**Mitigations:**
- Documentation: recommend fine-grained PATs with `contents:read` scope only
- Never pass `token_env` var to script subprocesses (keep it Nimble-internal only)
- Consider keychain integration instead of env vars (platform-specific but more secure)

---

## Threat 4: YAML-only attacks

**Vector:** Even without scripts, malicious YAML commands could:
- `open_url` with a phishing URL that looks legitimate
- `paste_text` with text containing hidden Unicode characters (homoglyph attacks)
- `copy_text` with a Bitcoin address swap (clipboard hijacking via social engineering)

**Impact:** Low-Medium. Requires user to actively invoke the command.

**Mitigations:**
- URL display in result subtext so the user can see where they're going
- Text preview for paste/copy commands before execution
- These mitigations should exist regardless of remote sources

---

## Threat 5: Auto-update supply chain attack

**Vector:** A trusted repo is compromised (maintainer account takeover) or the maintainer goes rogue. A malicious update is pushed. `auto_update: true` syncs it before the user notices.

**Impact:** High. Silent code execution on next use.

**Mitigations:**
| Mitigation | Effort | Effectiveness |
|-----------|--------|---------------|
| Default `auto_update: false` | Zero | Good — user must manually sync |
| "What changed" diff review before applying updates | High | Very good — but UX-heavy |
| Pin to a specific tag/SHA (immutable reference) | Low | High — no silent updates |
| Delay updates by N days (let community flag issues) | Medium | Medium |

**Recommended:** Default `auto_update: false`. For `auto_update: true`, default `scripts: deny`.

---

## Security Design Principles

1. **YAML-only is the default.** Remote sources should default to `scripts: deny`. YAML-only commands (`open_url`, `paste_text`, `copy_text`, `static_list`) are low-risk.

2. **Tokens never reach scripts.** The `token_env` value must be consumed only by Nimble's sync engine, never injected into script environments.

3. **Local always wins.** A local command with the same phrase always takes priority over a remote one. This prevents remote sources from shadowing user commands.

4. **Sync is explicit by default.** `auto_update` defaults to `false`. The user runs `/sync` when they want updates.

5. **The `_sources/` directory is clearly separate.** Users know at a glance which commands are remote-managed vs. locally authored.

---

## Minimal Viable Security Posture

If this feature ships, the **absolute minimum** security requirements:

- [ ] `scripts: deny` as default for all sources
- [ ] Token env vars excluded from script subprocesses  
- [ ] First-sync confirmation dialog showing repo details
- [ ] Source attribution visible in the UI for every remote command
- [ ] `_sources/` directory clearly marked as managed (README or .nimble-managed marker)
- [ ] Documentation on fine-grained PAT scoping
- [ ] `auto_update: false` as default
