# Native keyboard diagnosis and probe access — ROOT engineering decision

The actual uninstrumented f5bc01f8b native witness found two failures: CmdZ in
the focused Filter model tree input left Witnessx unchanged while model history
was preserved; first Escape from the visibly open coordinate-unit select popup
closed both the popup and Both inspector. Neutral-focus model Undo/Redo worked
once per key. These are observed failures, not acceptance evidence.

Right-click and CmdAltI did not reveal an observed Web Inspector route, so exact
window metrics and key-event timing remain unobserved by that route. The manager
retains exact actions under B3-CODEX/_run_records/native/F5_CUA_ACTION_LOG.md.

ROOT directs bounded diagnosis under software-defect-diagnosis, not a speculative
shortcut redesign. All new diagnostic/probe TASK deployments use Astra/low under
the owner's model direction. Parent manager owns their scope and UI scheduling.

## Probe method

Use an isolated disposable checkout/build from the frozen candidate. Preserve
production source unchanged. Retain base, patch, source and binary hashes.
A minimal probe-only patch may use supported Tauri Rust window APIs to record
inner/outer sizes and scale factor at startup and resize; open the built-in
devtools through Tauri's public API; and add passive DOM event observations.
Those observers do not prevent default, stop propagation, synthesize actions or
alter controls. All actual pointer/keyboard/resize actions use CUA.

Only disposable test project IDs, one UI owner, non-secret evidence. No unexpected
permission expansion or production debug surface. The probe is diagnostic, not
the final uninstrumented product witness. Final behaviour must be reconfirmed on
the repaired uninstrumented build.

A separately labelled experiment may add predefined Undo/Redo menu items to test
the missing-native-command hypothesis. Test focused text AND neutral model
shortcuts, since fixing one by intercepting the other would not be a repair.
For select Escape, record selector support and event-time :open/defaultPrevented/
target/ordering; do not swallow every select Escape or infer popup state from
focus alone. Page-close focus repair can continue independently in owning files.

## Primary reference pointers, not causal proof

Tauri documents OS/Tauri-defined actions separately from custom menu callbacks:
[Window Menu](https://v2.tauri.app/learn/window-menu/#creating-predefined-menu).
The installed candidate uses custom model Undo/Redo items and predefined
cut/copy/paste/select-all, without predefined Undo/Redo. That supports a
hypothesis; it does not establish the cause of this observed native failure.

MDN documents :open for a select whose picker is displayed:
[:open](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:open).
Actual host support and event ordering still require observation before relying
on that selector. Do not assume current documentation proves this WebKit's
behaviour.

This is ROOT's implementation/verification method decision within the approved
B3 scope. No new owner product ruling, protected limit change, performance
qualification or release follows.
