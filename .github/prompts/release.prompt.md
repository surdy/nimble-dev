---
description: Prepare a new release — bump versions, generate changelog, tag, and push.
agent: agent
tools:
  - editFiles
  - runCommands
  - codebase
---

You are preparing a new Nimble release. The user will provide the new version
number (e.g. "0.3.0"). Follow these steps **in order**, pausing where indicated.

---

## Step 1 — Determine the previous release tag

Run `git tag --sort=-v:refname | head -5` to find the most recent `v*` tag.
This is the baseline for the changelog.

## Step 2 — Bump version in all three files

Update the `version` field to the new version in:

1. `package.json`
2. `src-tauri/tauri.conf.json`
3. `src-tauri/Cargo.toml`

Then run `cargo check --manifest-path src-tauri/Cargo.toml` to regenerate
`src-tauri/Cargo.lock` with the new version.

## Step 3 — Generate the changelog section

Run `git log --oneline <previous-tag>..HEAD` to get all commits since the last
release. Categorise them into **Added**, **Changed**, and **Fixed** sections
following [Keep a Changelog](https://keepachangelog.com/) format.

Insert a new `## [<version>] — <today's date>` section at the top of
`CHANGELOG.md` (below the header, above the previous release section).

## Step 4 — STOP and wait for review

Tell the user: "Changelog and version bump are ready for review. Please check
CHANGELOG.md and let me know when you're happy."

**Do not commit, tag, or push anything.** Wait for the user to confirm.

## Step 5 — After user approval

Once the user gives the go-ahead:

1. Re-read `CHANGELOG.md` in case the user made edits.
2. Run `cargo test --manifest-path src-tauri/Cargo.toml` and confirm all tests pass.
3. Stage all changes: `git add -A`
4. Commit: `git commit -m "chore: release v<version>"`
5. Tag: `git tag v<version>`
6. Push commit and tag separately (lightweight tags are skipped by `--follow-tags`):
   ```
   git push
   git push origin v<version>
   ```
7. Verify the tag exists on remote: `git ls-remote --tags origin | grep v<version>`
8. If the tag already existed on remote, delete and re-push it.

Report the final commit hash and tag.
