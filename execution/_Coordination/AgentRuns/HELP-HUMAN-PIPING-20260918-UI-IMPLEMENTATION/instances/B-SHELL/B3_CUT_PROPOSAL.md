# Lane B-SHELL: the cut of slice B3, and what the code shows about B3 to B7

Lane B-SHELL's manager (WORKING_ITEMS, Type 1; Claude Fable 5.1 by its own statement), 2026-09-19, written while slice B2F is under independent review and before any B3 product edit. It is a proposal to ROOT where it departs from the lane brief, and the manager's own decision where the brief leaves the cut to the manager. Paths are relative to `{WORKING_ROOT}/apps/desktop` unless they begin with a placeholder. Read from: the handoff; specification §2.1 to §2.3, §2.7, §2.8, §5.4, §5.5 and §10.9; design system §0, §5.2 and §5.3; frame `s1_both_light`; the operations map's rows 9 to 40; `src/App.tsx`, `src/features/workspace/**`, `e2e/**`.

## 1. What today's shell is, against the frames

Today: a green title bar (wordmark, project name, a Units disclosure, Theme and Density selects, five project buttons); an in-app menu bar outside the native runtime; `WorkspaceToolbar` (the one palette surface, Select, Undo, Redo, Review, a task readout); a three-pane core (`ModelTree` with its list and grid modes | `PipeViewport` | `PropertyInspector`, each side collapsible by `toggle-tree` / `toggle-inspector`, with two splitters and a narrow-window drawer behaviour); a bottom dock of ten sections (`operations`, `loads`, `libraries`, `rule-packs`, `solve`, `results`, `report`, `project`, `exports`, `evidence`), one visible at a time, all reached through `setActiveSection`; a status bar of three or four pills whose body is the raw token, an audit-drawer button and the issues button.

The frames: one 48 px toolbar band, a 56 px stage rail, the surfaces (Table, Model or Both view of the stage's tables and the canvas, the inspector docked on the canvas's right), a 44 px agent strip, a 24 px status bar. There is no tree and no bottom dock of sections.

## 2. The cut of B3 (the manager's decision; one child, two stages, one slice return)

**Stage 1, the shell's control layer, nothing visible.** The guard that no file other than `workspaceSession.ts` imports a `*SessionState.ts` hook; the session key-set test; the `tokens.json` labels against `statusLabels.ts` agreement test (addendum 1); `WorkspaceSessionContext`; new chrome cells for the stage, the per-stage view and the open page, with the pure rules in one React-free module and tested there: first-open views, the per-stage memory, the home of each of today's ten sections, the rail's enabled states, tooltips and captions from the run's standing, and the status-chip policy of specification §5.4 as a function from the session's cells to a list of chips. The manager corrects `B2_RENDER_ORDER_FINDING.md` point 4 in the same records commit.

**Stage 2, the shell.** Toolbar band, stage rail, view switch, surfaces with the 55 / 45 split and the docked inspector (300 px in Both, 340 px in Model), the Model view's table drawer, the agent strip's 44 px region, status bar with chips from `statusLabels.ts`, issues count, selection readout, units and the About control. Today's panels are rehoused, not rebuilt (section 3). Tests that pinned display text or structure move, and each move is listed.

Why one slice and not two: the structure review and both Playwright lanes are the costly part, and stage 1 alone draws nothing to review by screenshot. Why two stages: stage 1 is the part where a defect costs most and it is reviewable on its own as a commit with no visible change.

## 3. Where today's ten sections live in the shell until their own slice rebuilds them

`setActiveSection` stays the one navigation primitive, so that every `view.section.*` command, the palette and the tests' `openWorkspaceSection` keep working; the shell derives the stage or page from the section.

| Today's section | Home in B3 | Rebuilt by |
|---|---|---|
| the model tree (list and grid) | Model stage, the table pane | B4 (Layout, Restraints, Node data) |
| `operations` (Review changes) | Model stage, a second tab of the table pane | B4's "Apply" chip and B5; the agent parts by the agent column (section 5, item 1) |
| `loads` | Loads stage, the table pane | B4 |
| `results`, `evidence` | Results stage, tabs of the table pane | B6 |
| `report` | Review stage | B7 |
| `libraries`, `rule-packs` | the rail's Libraries and Rules pages | not in this tranche's slices |
| `solve` | a page over the stage, opened by Analyze › Run settings and rule checks; Run itself is the toolbar's button | B6 (run menu) |
| `project`, `exports` | pages over the stage, opened from File; the five project buttons move to the Project page's header with their names unchanged | not in this tranche's slices |
| audit and boundaries drawer | opened by the status bar's About control | G-28's About page, not in this tranche's slices |

A page opens over the current stage's surfaces and returns to it, as the specification's Libraries and Rules do. All sections stay mounted as today.

## 4. Decisions inside B3 that the manager takes, stated so ROOT can reverse them

1. **View memory is session state.** Saving it with the project is G-17 (typed interface), outside the tranche, as the handoff says of G-32.
2. **The Run button starts and stops a run and says why it is disabled.** The run log popover and the run menu are B6.
3. **The inspector docks; the slide-over and states D and E of §10.9 are B5**, as the lane brief says. In Table view B3 has no inspector and, until B4, no row expansion; editing in Table view is what the tree's grid mode gives today.
4. **Theme, density, the stage and view commands and the project commands get a pointer home in both runtimes through the one palette** (`ToolkitPalette`, `DEC-094`) and the in-app menu bar. The native menu is `src-tauri/**` (item 3 of section 5).
5. **The status chips follow §5.4 in B3**: domain and label in the chip, the raw token in its tooltip and popover, none while the model is complete and unsolved, dropped after a model change, none for a Historical run, Human on the Review page only, two chips there with no rule pack (the owner's ruling of 2026-09-19). Each is a named semantic change with a test; about thirty-five test references to `status-pill-*` text move.
6. **Save state in the toolbar** ("· saved", "· edited", "· not saved") is shown only if it can be derived truthfully from the session's cells with a test; otherwise it is absent and listed.

## 5. Brought to ROOT: five things B3 cannot settle alone

1. **The agent strip and column are in no slice of the lane brief.** The strip's 44 px must exist in B3, because D-72's canvas of 603 × 828 at 1440 × 900 is (1440 − 56 − 44) × 45 %. The column's layout is G-28 (in the tranche); its live binding is G-19 (the host gap, outside it). Today's agent surfaces (`AgentWorkbenchPanel`, `OfflineProposalIntakePanel`, `AgentProposalPanel`) sit inside the `operations` section. Proposal: B3 draws the strip and the toolbar's Agent toggle **disabled with the reason shown**, and a slice after B5 ("B5A, the agent column") moves the three existing panels into the column. The reason's wording is the owner's or ROOT's; the manager's default is "Agent column: not in this build".
2. **The canvas lane's benchmark instrument is bound to today's shell.** `e2e/ui-foundation/**` (not this lane's) reads `.workspace-pane-tree` and `.workspace-pane-inspector` as a two-pane presentation contract with recorded geometry (`characterization-commands.ts:62` and `:96`; `full-cohort-controller.spec.ts:1025` records the tree pane at x 0, width 300 in a 1440 × 920 window), reads `.workspace-pane-inspector .panel.inspector > h2` in eight places of `benchmark-harness.ts`, and drives `toggle-tree`, `toggle-inspector`, `workspace-dock-close`, `command-bar`, `model-tree-filter-input` (nine uses), the "Save local" button and the "Appearance theme" and "Workspace density" labels. B3 moves every one of those regions. The manager's rule for the child: keep each hook alive on its honest successor (the table pane keeps `.workspace-pane-tree` and the tree; the docked inspector keeps `.workspace-pane-inspector`; `toggle-inspector` is the toolbar's Inspector toggle), never fake one, and return the list of hooks that changed meaning or geometry with the numbers. The frozen geometry values are the canvas lane's and ROOT's: B3 will change them by design (the rail takes 56 px on the left; the table pane is 737 px, not 300), and that is reported, not adjusted. ROOT may want the canvas lane told before B3 lands, since its second profile is measured on this shell.
3. **The native menu** has no items for Table / Model / Both, the stages, theme or density, and adding them is `src-tauri/**`. Until that is scoped, the native runtime reaches them by the toolbar, the rail and the palette, which satisfies the pointer rule; the menu-bar completeness of specification §5.5 does not hold in the native runtime.
4. **The window minimum of 1280 × 800** is `src-tauri/tauri.conf.json`. Today's narrow-window drawer behaviour and its e2e coverage (`e2e/workspace-layout.spec.ts`, the 1280 × 800 project of `playwright.config.ts`) stay meaningful in the browser. B3 keeps the shell usable down to 1280 and keeps a narrow fallback below it no worse than today's; the specification's narrow-case order is B5's.
5. **`uiPreferences` is a versioned local-storage record** (`leftRailPx`, `rightRailPx`, `dockPx`). B3 adds the Both view's split and the drawer height, reads old records without error, and stops using `leftRailPx`. No migration of stored values is attempted.

## 6. B4 to B7: no change proposed

The order stands. One note for B4: `ModelTree`'s grid mode already edits through queued intents into `applyModelOperation`; the operation-equivalence test of addendum 3 can take it as "today's route".

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
