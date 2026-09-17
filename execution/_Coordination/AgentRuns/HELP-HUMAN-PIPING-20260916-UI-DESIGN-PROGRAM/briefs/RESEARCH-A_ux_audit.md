You are a senior product designer performing a rigorous heuristic evaluation of the current desktop UI of OpenPipeStress, a piping stress analysis application (Tauri + React + Vite + Three.js). The owner's verdict is that the UI is "useful for the agents operating it" but "far, far away from what good UI should look like" for serious engineering work. Your job is to ground that verdict in specific, evidenced findings — and to identify what is genuinely good and must be preserved.

READ-ONLY. Do not modify any file in the repository. Write your output ONLY to:
{SCRATCHPAD}/research/A_ux_audit.md
(create the directory if needed).

Repository root (git worktree, run everything from here, do not cd elsewhere):
{REPO_ROOT}
Project root: projects/chirality-piping
App source: projects/chirality-piping/apps/desktop/src

Key files to read (all paths relative to the app source dir):
- App.tsx (3709 lines: shell composition, WORKSPACE_SECTIONS at ~line 373, MenuBar ~2660, StatusBar ~2913, AuditBoundaryDrawer ~2986, IssuesHome ~3047, AgentWorkbenchPanel ~2807). Read the JSX render sections carefully.
- styles.css (3438 lines: read the header comment, :root, layout grid, .workflow-ribbon, .modeling-workspace, .workspace-dock, .viewport-*, .command-bar, .status-bar, .panel)
- features/viewport/PipeViewport.tsx (3097 lines: viewport chrome, command bar, node/pipe creation forms)
- features/model-tree/ModelTree.tsx and PropertyInspector.tsx
- features/toolkit/ToolkitPalette.tsx, capabilityCatalog.ts, BatchReviewPanel.tsx
- features/operations/OperationApplyPanel.tsx, OperationLedgerPanel.tsx
- features/results/ResultsPanel.tsx, features/solve/SolvePanel.tsx, features/load-cases/LoadCaseManagerPanel.tsx (skim), features/missing-data/MissingDataBlockingPanel.tsx, features/diagnostics/DiagnosticsPanel.tsx
- Skim the remaining features/*/ panels just enough to characterize what each puts on screen.

Screenshots of the ACTUAL app (use the Read tool to view them; these are real renders, not mockups):
- projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/FINAL_NATIVE_1024x768.png (native Tauri, blank model, Results section open)
- projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W7/ — before-default.png, before-toolkit.png, after-default.png, after-discovery.png, after-routing.png, after-selected-pipe.png, after-support.png, after-loads.png, after-batch-review.png, after-field-error.png, after-results.png, after-batch-undone.png, after-empty.png
- projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/assets/*.png (older shell states)

Context you should know:
- The June 2026 owner verdict on an earlier shell was: "a wall of evidence/telemetry strings, three competing navigation systems, the model a postage stamp." A redesign plan followed (projects/chirality-piping/plans/PLAN_2026-06-18_workspace_redesign_c5_7.md — read §1 and §2). The September 2026 product-design report (projects/chirality-piping/plans/REPORT_2026-09-05_piping_modeling_and_analysis_product_design.md — read §1-§3) recorded "substantially the same complaint" again. Assess to what degree the current source still exhibits these defects, and what new ones exist.
- The product is code-neutral and open-source: it must never claim code compliance, professional approval, certification; it must show missing data explicitly and never insert engineering defaults; units and provenance are first-class. These are legitimate constraints, not UI defects. Distinguish "governance content that must exist somewhere" from "governance content dominating the primary work surface."
- Target users: piping stress engineers, piping designers/layout engineers, owner-operator reviewers (see docs/PRD.md §9).

Evaluate, with evidence (file:line ranges and/or screenshot filename), each of:
1. Information architecture: how many navigation/command systems exist simultaneously (menu bar, ribbon, command bar, toolkit palette, section switcher, status bar openers, drawers, dock sections)? Do they overlap? Is there one obvious "spine"?
2. Spatial hierarchy: what fraction of the window does the 3D canvas actually get at 1024x768 and 1280x800 (derive from CSS grid/clamp values and the screenshots)? What competes with it?
3. Density and typography: font sizes, control heights, panel padding; does the layout read as a professional CAD/analysis tool or as a web form?
4. Language: inventory the kinds of machine strings surfaced on primary surfaces (hashes, seams, generation counters, testids-as-labels, enum tokens, "invented", boundary flags). Give 10-15 verbatim examples with where they appear.
5. Workflow: trace, from the source, the click path for (a) create two nodes and a pipe, (b) add a support, (c) run a solve, (d) read a displacement result. Count steps and surfaces crossed. Where does the queue/review/apply gate help and where does it merely add friction?
6. Feedback and errors: how are blocking conditions, missing data, and validation errors presented? Are they actionable and located (do they select the entity)?
7. Visual language: color system, iconography (lucide), status color use, whether "green" could be misread as approval.
8. Consistency: compare 5-6 panels for heading styles, button placement, table styles, disclosure patterns.
9. The 3D viewport itself: what is rendered (fixed radius pipes, labels, glyphs, grid, triad, view cube), what a stress engineer would expect to see and not see.
10. What is genuinely good: list capabilities and interaction decisions worth preserving (atomic batch apply, undo checkpoints, shared selection identity, unit preservation, explicit Current/Historical, the searchable palette, etc.).

Output format (markdown):
- Executive summary (10 lines max)
- Findings table: ID, severity (Critical/Major/Minor), area, finding (one sentence), evidence, "what good looks like" (one sentence)
- Sections 1-10 above, each concise with evidence
- A "preserve" list
- A short "what the owner is probably reacting to" section: your best inference of the 5 things that make this UI feel unserious to a professional, ranked.
Do not propose an implementation plan. Do not write to the repository. Be concrete and merciless but fair.