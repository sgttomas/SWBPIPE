# Native compact action ledger

1. CUA `let app = await cua.getApp("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app");`

Result: `The Mac is locked and automatic unlock could not unlock it. Ask the user to unlock the Mac manually before continuing.` No AX state or screenshot available. No UI input, New Blank, model operation, save or delete occurred.

## Resumed CUA action ledger (chronological)

All actions used `app` bound by the exact bundle path. Each numbered group ended in `getAXState()` before subsequent adaptive decisions. Element indexes below were from that latest AX state. Raw selected observations are in RAW_AX.txt; full tool-return history is the harness conversation. Screenshot writes used node:fs only to retain returned CUA bytes, never to drive UI.

2. Revalidate PID36554/start/path/hash; `cua.getApp(exactPath)` returned Invented Utility Loop Preview.
3. `click(29); pressKey('super+1')`: Loads/Table. `click(166)` File; `click(4)` New Blank. Exact new project 20260920t052640z; Model/Both, Select, closed Inspector.
4. `click(24)` Appearance → System/Comfortable; `click(128); click(100)` close appearance, arm Node.
5. `click(151)` unit popup; screenshot01; `pressKey('Escape')` first closes popup/focus151; second Escape closes Inspector/focus21.
6. `click(21)` reopen; `click(151); Escape; space; Return` same-value keyboard commit; `Return; Down` active mm/committed m screenshot02; `Return` commits mm.
7. `Down; Down; Escape` cancels preserving mm. `click(151); click(194)` same-value pointer mm; `space; Return; Escape` closes Inspector/focus21.
8. `click(21); click(151); Tab` moves to provenance157; `click(151); click(139)` outside focus to label; `click(151); click(30)` Loads stage; `click(29)` restores Model/Both, Inspector, draft mm.
9. `click(219); super+2` switches Model while open, popup closes; Escape disarms Node, leaves docked Inspector. `super+3; click(250)` expands nested support configuration.
10. `click(288)` Support family popup screenshot03; Escape closes popup/focus288 preserving Not provided; second Escape closes Inspector/focus21; `super+i` reopens once.
11. `click(335); click(188)` collapse nested and arm Node. `click(412); pressKey('x')` genuine label key; CmdZ first clears. `CmdZ; ShiftCmdZ` empty-stack undo then restores x.
12. `click(241); typeText('node:N-COMPACT-WITNESS'); click(247); typeText('0'); click(250); typeText('0'); click(253); typeText('0'); click(262); typeText('native_compact_witness')`. Label remains x, unit mm. `click(263)` Add validates operation; `click(270)` Apply, count2→3.
13. `click(138); CmdZ` neutral model count3→2; `ShiftCmdZ` count2→3.
14. `click(175); pressKey('x'); CmdZ` Filter x→empty, genuine text redo stack. `click(138); CmdZ` model3→2; `ShiftCmdZ` model2→3; `click(175); ShiftCmdZ` text x restored, model unchanged.
15. `ShiftCmdZ; CmdZ; CmdZ` redo exhaustion, undo x, undo exhaustion; count3 remains, Undo model available. `click(138); click(123)` opens native Edit. `click(3)` custom Undo Model Edit count3→2. `click(144)` native Edit; `click(4)` custom Redo Model Edit count2→3.
16. `click(154)` native View; `click(5)` already-active Both; Both remains on. `super+i` closes Inspector once.
17. `click(34); Tab; Tab; Tab` Libraries Close focus161; Return closes, focuses Libraries34; screenshot04.
18. `super+2; click(161)` Model, collapse drawer. Screenshot establishes corner. `drag([1200,765],[980,620])` shrinks native window; screenshot05. `drag([1226,765],[990,615])` second shrink no AX change; screenshot06. `super+3` restores Both/closed Inspector.
19. `click(152)` File; immutable read-only SQLite query confirms latest row exact own project and nodes empty; `click(5)` Open reloads exact row, clears session history, count2.
20. `click(23)` Appearance confirms System/Comfortable; `click(127)` closes. Full FINAL_AX and screenshot07 retained. No further UI actions.
