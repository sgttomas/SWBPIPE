# Actual CUA actions

All actions targeted explicit disposable app path from launch.json. UI clearance granted by B3 manager before launch. No synthetic input, saved model changes, or private field entry.

1. getApp(explicit probe SWBPIPE.app path): attached to PID24955; initial invented demo and docked devtools.
2. click(3 Close devtools), pressKey(super+n), getAXState: devtools closed; project unchanged (shortcut did not create project).
3. click(651 File), getAXState: native File menu.
4. click(4 New Blank Project), getAXState: blank project project:blank-local-20260920t042901z. Both view; Undo/Redo disabled.
5. click(100 Node), getAXState: Node draft inspector expanded, m coordinate unit. No draft values entered/applied.
6. click(150 New node coordinate unit), getAXState: native menu selected m; options mm/in/ft. Read runtime log while open: begin gen1 but no :open DOM observation/snapshot binding.
7. pressKey(Escape), getAXState: popup closed; Inspector remains expanded, focused unit m. Raw log only Escape keyup.
8. pressKey(space), getAXState: native popup selected m. Read runtime log while open: DOM Space keyup :open=true; native gen2 epoch1 tracking1; bind valid=true.
9. pressKey(Return), getAXState: same value m committed; popup closed, Inspector expanded. Native gen2 end key36 then will/did action, no DOM input/change/Return.
10. pressKey(Escape), getAXState: no AX change; Inspector still expanded, focused unit m. Native snapshot gen2 epoch2 tracking0 action1; current=true correlated=false; STOP logged. Required behavior FAIL.

Stopped matrix at failed correlation gate. No further input or architecture changes. Verified PID/path/start/binary, SIGTERM PID24955, ps confirms absent. UI released.

Full raw AX outputs reside in tool transcript. runtime.log is canonical captured non-secret native/DOM evidence. No separate screenshot artifact is claimed.
