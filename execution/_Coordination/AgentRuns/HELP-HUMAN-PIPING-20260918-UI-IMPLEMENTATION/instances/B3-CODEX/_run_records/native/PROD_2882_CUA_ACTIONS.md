# Exact production 2882 CUA action summary

The actual indices below are the fresh AX indices at each step; a fresh AX
observation was taken whenever an action changed the tree.

```javascript
let prodApp = await cua.getApp("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app");
await prodApp.click(166);                 // File
await prodApp.click(4);                   // New Blank Project
await prodApp.click(99);                  // Node tool
await prodApp.click(137);                 // Node Label
await prodApp.pressKey('a');
await prodApp.pressKey('b');
await prodApp.pressKey('c');
await prodApp.pressKey('super+z');        // first no-op
await prodApp.click(184);                 // Edit menu
await prodApp.click(5);                   // predefined Undo pointer
await prodApp.pressKey('super+shift+z');
await prodApp.pressKey('super+z');
await prodApp.pressKey('super+z');        // undo exhaustion
await prodApp.pressKey('super+shift+z');
await prodApp.pressKey('super+shift+z');  // abc -> Abc layer
await prodApp.pressKey('super+shift+z');  // redo exhaustion
await prodApp.pressKey('super+z');        // Abc -> abc
await prodApp.pressKey('super+z');        // abc -> empty
```

The node form was then completed through fresh settable-field observations,
Add, review and Apply. Neutral model and pointer paths:

```javascript
await prodApp.click(12);                  // Select neutral focus
await prodApp.pressKey('super+z');
await prodApp.pressKey('super+shift+z');
await prodApp.click(10);                  // toolbar Undo
await prodApp.click(11);                  // toolbar Redo
await prodApp.click(49);                  // Filter model tree
await prodApp.typeText('wit');
await prodApp.pressKey('super+z');        // first no-op
await prodApp.pressKey('super+z');        // clears wit
await prodApp.pressKey('super+shift+z');  // restores wit
await prodApp.click(12);                  // neutral focus
await prodApp.pressKey('super+z');        // model only
await prodApp.pressKey('super+shift+z');  // model only
```

Same-value menu and page-close focus:

```javascript
await prodApp.click(126);                 // View menu
await prodApp.click(5);                   // already-active Both
await prodApp.click(34);                  // Libraries
await prodApp.pressKey('Tab');            // Rules
await prodApp.pressKey('Tab');            // Issues
await prodApp.pressKey('Tab');            // visible Close (omitted by WebKit AX)
await prodApp.pressKey('Return');         // close page
```

The full post-close AX state reported Libraries focused.

Open/title process:

```javascript
let openApp = await cua.getApp("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app");
await openApp.click(166);                  // File from invented preview
await openApp.click(5);                    // Open Local Project
```

Immediately before that Open, a read-only SQLite query established
`project:blank-local-20260920t044213z` as the latest row. The post-open AX tree
identified that row and the Blank title. Appearance was expanded by pointer;
AX reported System and Comfortable, then the disclosure was closed.
