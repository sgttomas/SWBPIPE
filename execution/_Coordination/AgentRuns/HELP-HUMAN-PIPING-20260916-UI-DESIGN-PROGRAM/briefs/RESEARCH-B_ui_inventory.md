Produce an objective, quantitative inventory of the current desktop UI of OpenPipeStress. Numbers only — no design opinions. READ-ONLY: do not modify any repository file.

Repository root (git worktree, run everything from here, do not cd elsewhere):
{REPO_ROOT}
App source dir: projects/chirality-piping/apps/desktop/src

Write your output ONLY to:
{SCRATCHPAD}/research/B_ui_inventory.md
(create the directory if needed). Include a methods section stating the exact commands used for each number so they are reproducible.

Measure (use grep/awk/wc/node scripts as needed; approximate where exact is impractical but say so):
1. Structure: number of feature directories under features/; number of *Panel.tsx components; number of panel components imported into App.tsx; lines of code per feature dir (non-test .ts/.tsx), sorted descending; total non-test LOC vs test LOC.
2. Navigation surfaces in App.tsx and PipeViewport.tsx: count entries in WORKSPACE_SECTIONS; count MenuCommandId union members and menu items rendered by MenuBar; count ribbon stops; count command-bar buttons; count status-bar buttons/openers; count drawers/dialogs; count of distinct `data-testid` values across src (non-test files); count of `aria-label` attributes.
3. Interactive controls: count of `<button` occurrences across features (non-test); `<input`, `<select`, `<textarea`; `<details`/`<summary` disclosures; `<table`.
4. Machine-string exposure: for each of these substrings, count occurrences inside JSX text or template literals in non-test .tsx files (rough proxy for what is rendered): "sha256", "hash", "seam", "generation", "boundary", "evidence", "telemetry", "professional", "invented", "testid", "TBD", "UNKNOWN", "not_", "_id", "manifest". Report counts per file for the top 10 files.
5. Styles (styles.css): total rules (count of `{`), distinct hex/rgb colors, distinct font-size values, distinct border-radius values, distinct box-shadow values, distinct font-family declarations, count of !important, count of media queries, number of `min-width`/`min-height` pixel constraints on the shell (body min-width etc.), the grid-template-columns of .modeling-workspace and any clamp() on the viewport height.
6. Canvas budget: from styles.css and the app-shell grid rows, estimate the pixel height available to the 3D canvas at 1024x768 and 1280x800 when a dock section is open vs closed (show your arithmetic, cite the CSS lines). Cross-check against the screenshot projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/FINAL_NATIVE_1024x768.png by viewing it with the Read tool and estimating the canvas rectangle.
7. Icons: count distinct lucide-react icons imported across src; list them.
8. Tests: number of test files, approximate number of `it(`/`test(` cases, number of e2e specs under apps/desktop/e2e.
9. Toolkit catalogue: count entries in features/toolkit/capabilityCatalog.ts, grouped by their group field; count marked roadmap/unavailable.
10. Dependencies: list runtime dependencies from apps/desktop/package.json with versions.

Output as markdown with tables; end with a one-paragraph "notable numbers" list (no opinions, just the numbers that stand out by magnitude).