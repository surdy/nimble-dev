# First Run

When you launch Ctx for the first time you will see the onboarding screen. This only happens once — every subsequent launch goes straight to the background.

---

## Choosing your global shortcut

The onboarding screen asks you to press the key combination you want to use to open the launcher from anywhere on your Mac.

1. Click anywhere on the onboarding window to make sure it has focus
2. Press your desired key combination — for example `⌘Space`, `⌃⌥Space`, or `⌘⇧/`
3. The combination you pressed appears in the preview area
4. If you want a different shortcut, press another combination — it replaces the previous one
5. Click **Confirm shortcut** to save it

**Requirements for a valid shortcut:**
- Must include at least one modifier: `⌘` (Cmd), `⌃` (Ctrl), `⌥` (Option), or `⇧` (Shift)
- Plus at least one letter, number, or special key (`Space`, `Return`, arrow keys, etc.)
- Bare modifier keys on their own are not valid

If you choose a combination already claimed by another application, an error is shown — try a different one.

---

## After onboarding

Once you confirm your shortcut:

- The launcher hides itself and continues running silently in the background
- No Dock icon is shown
- Your shortcut is saved to local storage and automatically re-registered every time Ctx starts

Press your shortcut at any time to summon the command bar. Press it again — or press `Escape`, or click anywhere outside the window — to dismiss it.

---

## Changing your shortcut later

To pick a new shortcut, open the app's built-in DevTools console and run:

```js
localStorage.removeItem('ctx_hotkey')
```

Then quit and relaunch Ctx — the onboarding screen will appear again so you can choose a new combination.

> A proper "change shortcut" UI will be added in a future release.

---

## Supported platforms

- **macOS** — primary target, fully supported
- Windows and Linux — planned for a future release
