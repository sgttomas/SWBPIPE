# Sealed brief — INV1-INSTRUMENT: read-only inventory of what the benchmark instrument pins

Sealed by B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager of Tranche B) on 2026-09-19 before launch. Role of the reader: TASK (Type 2), **read-only**. Model requested: Claude Sonnet. Mechanism: Claude Code `Agent` tool, `Explore` type, background. **You work alone and never delegate: launch no agent.** Your parent is B-CANVAS.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the git worktree your launch message names; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`; `{INSTRUMENT}` is `{DESKTOP}/e2e/ui-foundation`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Write no absolute machine path in your return. **You write no file, edit no file, run no build, no test, no server and no git command that changes state.** Another agent is editing `{DESKTOP}/src/features/viewport/**` in the same worktree while you read; read that folder only through `git show HEAD:<path>` so that you describe the committed state.

## Why this inventory is wanted

The instrument under `{INSTRUMENT}` was frozen for an earlier interface. The owner's ruling D-72 (`{WORKING_ROOT}/execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` and its addendum beside it; read both whole, they are short) asks for a **second frozen boundary profile** for the redesigned interface: window 1440 × 900; two canvas configurations, 603 × 828 (Both view) and 1000 × 828 (Model view); label populations Off and Budget with caps of 138 and 230; one run per fixture size per canvas configuration, repeated twice only when a gated quantity is above 80 % of its limit; and the redesign changes how a selection is drawn (a halo in a new colour, in place of a recolour and a diamond marker). The first profile, its visual tokens, its oracle script, its manifest and its recorded results are never edited. Your parent has to propose how a second profile sits **beside** the first without touching it, and needs to know exactly what the instrument's code pins today.

## Limits of the lane (carried in full; most bind implementers, all bind what you may suggest)

- **One mutation route.** Every engineering action is a typed operation through `applyModelOperation` / `applyOperationBatch` into the Rust applier. Tables and canvas are projections; neither holds state the model does not.
- **Result integrity.** The Current and Historical designation, the exact solve-input basis, the generation gates, reviewed application, undo and redo, and persistence compatibility are preserved. A Historical record never gains a current-model overlay, a chip or a readiness cue through a presentation change.
- **Picking repair preserved.** PR #794's shared closest-point computation and its tests stay. Never alter a tolerance, an oracle expectation, a benchmark limit or a frozen characterization value to make something pass.
- **Rendering foundation retained**: persistent renderer, instancing and chunking, invalidation scheduler, resource ownership ledger, typed model index, selection and picking.
- **Every control maps to an operation or a classed gap**; a control is never faked.
- **Semantic changes are named** as changes, never delivered as restyling.
- **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation, schemas, `core/**`, `src-tauri/**`; the identity layer (document kinds, schema ids, crate names) is not touched.
- **Copy.** The product is SWBPIPE; no other vendor's product is named; Canadian spelling.
- **Accessibility.** WCAG 2.2 AA for touched controls; PDU-045 and PDU-046 remain holds; no usability acceptance is claimed.
- **Browser tests share the host**: you run none.
- **Never weaken a test.**
- **Frozen history is never edited**: `{WORKING_ROOT}/execution/**`, `validation/evidence/**`, `plans/**`, `docs/_history/**`, `docs/_ScopeChange/**`. You edit nothing at all.
- **The benchmark stays a reusable instrument** with named setup and query actions, typed entity identities, and real pointer and keyboard stimuli. The recorded demonstration is a comparison basis, not advance acceptance.

## What to inventory

Read `{INSTRUMENT}/README.md` whole first, then the code. For every file under `{INSTRUMENT}` (TypeScript, `.mjs`, `.json`, `.md`; skip the bulk data under `fixtures/` and `samples/` except to say what kinds of file are there and which hashes bind them), find every **pinned first-profile value**, which means a literal or a hard-coded check of any of these:

1. window or viewport size (1440, 920, `[1440,920]`), device pixel ratio, canvas size, ROI or clip sizes derived from them;
2. label state, label cap or budget (80, `selected_label_cap_candidate`, `labels.budget`, `renderedCount`), and the phase label policy (labels off for point picks, on for orbit);
3. shell layout selectors and pane or panel expectations (`.workspace-pane-tree`, `.workspace-pane-inspector`, `.panel.model-tree`, `.panel.inspector`, `.app-shell`, `data-theme`, `data-density`), and every product control the instrument drives or reads by test id, role or name (list each test id once with the files that use it), including `candidate-control-binding-v1.json`;
4. the selection's visual cue: colours (`0xf08c22`, `[240,140,34]`, `[163,68,0]`, rim colours, scene colours `[223,229,232]`, `[12,17,20]`), CSS variables read back (`--ui-viewport-selection-geometry`, `--ui-canvas`), cue geometry (11 px, 0.34, 0.5, erosion, 48 px ROI, 96 px ROI), tolerances (48 per channel), minimum counts (1, 4), the contrast ratio 3, and the schema names of the plans and witnesses;
5. hashes bound in code or prose: the fixture manifest's, the point-hit and box policies', `VISUAL_TOKENS_V4.json`'s, the cue source's and geometry source's, product revisions, authority hashes, control-binding and observer-binding hashes; say for each what file or record it binds and whether the bound bytes live in this repository (give the path) or outside it;
6. the run plan and cohort shape: five runs by two fixtures, `[1,2,3,4,5]`, 243 segments, repetitions, continuation slots, reference-profile requirements (host model, memory, refresh rate, display name), collection modes;
7. the numeric limits (2000, 100, 200, 200, 16.7, 33.3 ms) and where each is defined and where it is consumed;
8. the environment variables the instrument reads (`UI_FOUNDATION_*`, `D70_*` and any other), each with the file that reads it and what it selects;
9. the Playwright configuration files: for each, what it runs, its viewport, its web server and port, and which other config it extends.

For each pinned value give: file path relative to `{DESKTOP}`, line number, the enclosing function or constant name, whether that function or constant is **exported**, and which entry points reach it (which `*.benchmark.ts` or `*.spec.ts` or script). Say which functions are **profile-agnostic** (usable unchanged by a second profile by import) and which hard-code a first-profile value inside their body (so that a second profile could not reuse them without an edit).

Also answer, briefly and with file and line:

- A. How is `fixture-manifest.json` produced and verified (`generate-fixtures.mjs`, `verify-fixtures.mjs`)? Would adding a key to it change a hash that anything pins? What in the repository's ordinary test lanes (`npm run test:e2e`, `npm run test:e2e:dist`, `npm run test:desktop`, the evidence sweep `{WORKING_ROOT}/tools/release/run_evidence_sweep.py`) runs or checks instrument files, and which instrument files do those lanes execute?
- B. Which instrument files are themselves hash-bound by another instrument file or by a recorded result (so that editing them would break a binding)? Look for lists such as `dependencies`, `verifierDriverFiles`, `candidateExecutedFiles`, method manifests.
- C. Where does the product publish what the instrument reads (`__openPipeStressUiDiagnosticsV1`; `{DESKTOP}/src/features/workspace/uiDiagnostics.ts`; the `labels`, `canvas`, `camera`, `geometry`, `selection` fields)? Read these through `git show HEAD:<path>`. List the fields of the published snapshot that the instrument validates, with the instrument line that validates each.
- D. What does the instrument do with `--ui-viewport-selection-geometry` and `--ui-canvas` after reading them back, step by step, from the camera preflight to the timed point run?

## What to return

Your final message is the inventory; your parent retains it verbatim. Use tables. Keep it complete but compact: one row per pinned value or per group of identical values in one function; at most about 300 rows. Begin with who you are (model id), that you verified this brief's hash before starting (your launch message gives it), that you wrote nothing and ran nothing that changes state, and what you read whole and in part. End with anything you could not determine and why. Do not propose a design; report what is there. Attribute nothing to the owner beyond the records you read.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
