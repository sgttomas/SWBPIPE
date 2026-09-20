# Sealed B3A implementation TASK

TASK Type2, fresh GPT-6 Astra/low, parent retained B3 WORKING_ITEMS Astra/high under
ROOT HELP_HUMAN. Mechanism Codex delegated-harness-native; no child delegation.
Resolve REPO_ROOT in supplied wt3, WORKING_ROOT=projects/chirality-piping. Explicit
workdir on all commands. Basis clean main7e6a7f2578e5df12ea891c2d604d0cd48e869934.
Manager owns integration/index/checkpoints and native witness; child never commits,
pushes, PRs or merges. Written scope on unrestricted host, no OS-enforced isolation.

## Required sources and authority

Read rootAGENTS, agents/AGENT_TASK.md, PipingAGENTS/loop. Read sealed ROOT
B3A_MANAGER_BRIEF.md SHA25698986a724525d0e5d3fe1465b6ad2185d8d436fccd56f9f9494d24f417d195f2
and _run_records/B3A_LAUNCH_BASIS/INPUTS.json SHA256
e126b60504d1bed6c976f942393811278da1859b6a5a734be808d7340f818798 at supplied ROOT origin.
Read sealed graph snapshot and relevant B3A node there; no later live graph substitution.
Read approved continuation strategy/OWNER_APPROVAL, OWNER_MODEL_DIRECTION, owner resume,
EIGHT_UX_ITEMS item1/2, adopted UX and claims_registry section2.1/DEC102. Manager
INVENTORY_AND_PLAN.md supplies discovered code paths, not new authority. Read selected
software-defect-diagnosis skill for fail-before regression discipline. Record actual
input origins/hashes/model/parentage under own evidence before implementation.

ROOT explicitly approved this one worker's exact scope and terminal interpretation
in session. New worker Astra/low. B3B busy native menus and verified-at-save integrity
are excluded, and existing integrity behavior/tests must remain. No Runtime adoption,
model schema, engine, viewport/picking, benchmark or instruction changes.

## Product objective and precise semantics

1. One session saved-model basis against current canonical model hash, no persisted
field. Accessible Edited marker beside project name and same native window title.
Initial untouched preview uses a named loaded-source baseline; absence of Edited is
NOT evidence a file was saved. Real New/Open/Save/Create baselines require canonical
returned-model verification and correct project/session generation/request ownership.
Domain edit→dirty, successful save of current contents→clean, Undo to actual saved
canonical contents→clean, Redo→dirty. View/selection/theme/display-unit changes never
dirty engineering model. Current/Historical result gates stay unchanged.

Hash effect is asynchronous and nulled on model commit. Trace race/ownership; do not
compare stale hash or falsely clear during pending current hash. Tests must cover
out-of-order hash completion, pending edit/Undo/project replacement, failures and
supersession. A save that actually lands while a later same-project edit advances UI
must update comparison to the verified persisted snapshot without replacing current
model/history/results; Undo to that saved snapshot becomes clean. Failed write never
changes basis. Invalid response or unproved normalization must not clear marker.
Supported persisted normalization uses existing verification; do not relax it, adopt
an old model merely for chrome, or change integrity display/record semantics. If
correctness needs B3B or broader persistence semantics, report concrete dependency.

2. One display-only Solver · Not solved fallback in statusLabels.ts; never append to
the eight registered authority tokens or invent an engineering status. Unknown/
blocked recorded solver token uses fallback, preserving exact recorded token in
chip tooltip/popover and Analyze. Existing MECHANICS_SOLVED/MODEL_INCOMPLETE forms
remain. Historical lights no current chip; never-run complete model is not blanket
Not solved. ROOT's interpretation: failed job without result may use actual recorded
state failed, distinctly labelled 'Solve job state' in tooltip/Analyze. It is not a
fabricated solver token nor a current Model incomplete claim. Ordinary cancellation
without recorded failure/solver status keeps no-chip policy. Test all distinctions.

## Exact write scope (relative to apps/desktop)

- src/features/workspace/workspaceSession.ts
- src/features/workspace/modelSessionState.ts and projectSessionState.ts
- new src/features/workspace/savedModelBasis.ts and savedModelBasis.test.ts if useful
- src/features/workspace/shellLayout.ts and shellLayout.test.ts
- src/features/workspace/statusLabels.ts, statusLabels.test.ts, statusLabels.tokens.test.ts
- src/features/workspace/shell/ShellToolbar.tsx and ShellStatusBar.tsx
- src/services/nativeMenu.ts
- src-tauri/src/lib.rs ONLY native shell-state/title formatting + narrow tests
- src/App.shell.test.tsx and src/App.projectHandlers.test.tsx
- new e2e/b3a-session-status.spec.ts
- optional src/features/solve/SolvePanel.tsx ONLY required fallback/raw-source display

No other production or test path without manager/ROOT expansion first. Read-only
existing App.test.tsx normalization fixtures may guide tests but do not edit it.
No shared App.tsx/styles/context changes currently granted or expected. Marker's
plain existing presentation should avoid closing-pass visual tuning. Manager does
not concurrently write these paths. Own evidence only B3A-CODEX/_run_records/worker;
raw logs/snapshots/actual absolute origins go there. Historical B3 seals immutable.

## Execution and checks

Before product changes, explain bounded state/response ownership design and record
meaningful failing regression matrix. Prefer tests through App/session outcomes,
not only mirrored helper logic. Include actual domain edit/save/Undo/Redo/reopen;
failed save; landed save then newer edit; superseded project; supported normalization;
invalid returned identity; pending/current hash races; metadata-only no-dirty; solver
unknown/blocked/registered/Historical/never-run/failed/cancelled. Preserve original
assertions and all authority/history/result/persistence guards. No changing timeout,
floor/oracle, bypassing actions, force-click, new skips or fabricated native proof.

Unit/TypeScript and Rust title-focused tests may run now. No browser/dev/native slot
is granted: I1 owns the next5174 browser slot. Request manager reservation before
browser; use run lock, PLAYWRIGHT_WORKERS=1 and pinned Playwright Chromium explicit
executable/version. No app build/launch/CUA by child without later explicit grant.
Manager owns actual final Tauri witness and broader affected verification. No full
source/dist lanes here; ROOT owns one clean sweep at frozen integration candidate.

Send design and meaningful failure evidence early, exact changed paths/test selection,
state/solver interpretations, and any blocker. Seal return with source hashes, all
raw pass/fail logs/commands and limits. Release owned resources. No independent-review,
acceptance/usability/performance/release claim; ROOT provides fresh review/CI/merge.
