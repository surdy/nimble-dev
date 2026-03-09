# First Run

When you launch Contexts for the first time, you will be greeted by the onboarding screen before the launcher is usable. This only happens once.

---

## Choosing your global shortcut

The onboarding screen asks you to press the key combination you want to use to open the launcher from anywhere on your Mac.

![Onboarding screen showing "Welcome to Contexts" with a shortcut capture area](../static/favicon.png)

**How it works:**

1. Click anywhere on the onboarding window to make sure it has focus
2. Press your desired key combination — for example `Cmd+Space`, `Ctrl+Option+Space`, or `Cmd+Shift+/`
3. The combination you pressed appears in the preview box
4. If you want a different shortcut, simply press another combination — it replaces the previous one
5. Click **Confirm shortcut** to save it

**Requirements for a valid shortcut:**
- Must include at least one modifier key: `Cmd` (⌘), `Ctrl`, `Option` (⌥), or `Shift`
- Can use any letter, number, or key like `Space`, `Up`, `Down`, etc.
- Bare modifier keys alone (e.g. just `Shift`) are not valid

If the shortcut is already claimed by another application, an error is shown — try a different combination.

---

## After onboarding

Once confirmed:
- The launcher window hides itself
- The app continues running silently in the background (no Dock icon required)
- Your chosen shortcut is saved and automatically re-registered every time the app starts

---

## Using the launcher

Press your shortcut at any time to summon the command bar. Press it again (or press `Escape`, or click elsewhere) to dismiss it.

---

## Resetting your shortcut

To change your shortcut, open the browser DevTools console within the app window and run:

```js
localStorage.removeItem('contexts_hotkey')
```

Then relaunch the app — the onboarding screen will appear again.

> A proper "change shortcut" UI will be added in a later stage.

---

## Supported platforms

- **macOS** — primary target, fully supported
- Windows and Linux — planned for a future release
