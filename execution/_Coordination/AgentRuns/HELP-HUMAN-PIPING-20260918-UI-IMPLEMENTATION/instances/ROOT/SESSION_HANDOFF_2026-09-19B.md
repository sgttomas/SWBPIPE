# Session handoff, 2026-09-19 (second) — SWBPIPE interface implementation

Written by ROOT (HELP_HUMAN, Agent 0; the session began as Claude Opus 5 and the host changed its model to Claude Fable 5.1 during the day) at the owner's direction of 2026-09-19 ("We are nearing the next weekly session limit and will have to hand off further work to another agent in a fresh session. Finish B3 and C2 and then gracefully pause work."). It continues `SESSION_HANDOFF_2026-09-19.md` beside it, which still holds for everything this file does not restate: its sections 5 (conventions that bind), 6 (how the Agent 0, 1 and 2 behaviours work), 8 (where things are) and 9 (things that will bite). If this file disagrees with the owner's words, a ruling, a sealed instrument or a retained record, that other thing governs; say so to the owner and correct this file. Path placeholders as there.

## 1. Where the program stands

Merged to `main` in this session, each after an independent Opus code review with no actionable finding, a passing DEC-025 sweep, and both required CI checks on the final revision; each sweep ran on the revision the closeout records commit sits on:

| Pull request | What it did |
|---|---|
| #805 | Shell lane, slices B2F and B2G: project-handler repairs. A failed, empty or superseded open, create or save no longer removes a recorded hash mismatch from a project that stays open; a landed write clears the open-time record even when a model edit drops its response. Two corrections were found by review and made test first. |
| #806 | Canvas lane, slice C1: every canvas colour from a `canvas.*` token, the edge line, themed canvas furniture, the empty-layer guard. The two `THREE.Object3D.add` console errors have left `main`. |
| #807 | Canvas lane, slice C2: selection and hover as halos over the element's own colour (G-31), by a depth mask with no renderer-construction change. Merged: head `06088fcca82f2968a1cdf036d9c111108db81fb7`, merge commit `b313ce821d334aa741292248fba748c5d531d972`. |

**B3 (the visible shell) is not finished and not merged.** The owner asked for it to be finished; the shell manager judged, and ROOT agrees, that finishing needs one more full-size implementer child, both full Playwright lanes and probably a correction round, which is more than the budget left. It paused cleanly instead.

Branches, all pushed; nothing is running: no agent, no sweep, no dev server, no browser-test lock.

| Branch | Head | Git worktree | State |
|---|---|---|---|
| `codex/swbpipe-b-shell-20260918` | `a9be4aaefa432e1eed56e87482391e90958cb087` | `swbpipe-wt3` | B3 in progress, a records commit on the last product checkpoint `46db29e92`; contains `origin/main` at `d20eb1294`. The shell is built and the unit suite was green at the merge (85 files, 1,354 tests); five frozen-floor items are red and ROOT's rulings on them are sealed and not yet implemented. See section 3. |
| `codex/swbpipe-b-canvas-20260918` | `f6c0bab8e8893237f83f57656b98039d68efb2b4` | `swbpipe-wt4` | Paused clean, confirmed by ROOT. Contains `origin/main` at `d20eb1294` and slice C2 (merged as PR #807; merge `main` first). Slice I1 is committed as a labelled pause point (`240a246f3`), not a candidate: the manager checked it in part (fixtures, samples and the manifest byte-identical; the generator's new `--check` mode reproduces the frozen bytes and writes nothing; picking 69) and has not read the whole diff or run the full lanes on the committed head. The lane log's PAUSE STATUS section is written for a new manager. See section 4. |
| `codex/swbpipe-records-20260919` | the branch that carries this file | ROOT's session worktree | The run's shared records for this session. Merged by the pull request that carries this file. |
| `codex/swbpipe-design-system-v14-20260918` | `4dddd44128e052cfe518f84c0b718e6125cb2b69` | `task-management-gen-pass-518da2` | Parked, unchanged. The owner adopted DS5-D3 on 2026-09-19: it is reopened at the closing visual pass. |

`swbpipe-wt2` is ROOT's review and pull-request worktree and is free. ROOT took each pull request from a branch of its own cut at the reviewed head (`codex/swbpipe-b2f-20260919`, `codex/swbpipe-c-pr1-20260919`, `codex/swbpipe-c2-20260919`), so a lane's worktree stays free while ROOT sweeps and merges; keep doing that.

## 2. What the owner decided in this session

Three records beside this file, each with the owner's words verbatim and ROOT's reading labelled as ROOT's:

- `OWNER_DIRECTION_2026-09-19_SECTION7_DECISIONS.md`: ROOT's four standing decisions adopted (DS5-D1, DS5-D2, DS5-D3, C1-D1); the closing pass's appearance recommendations adopted; the wording items adopted; Tranche A2 follows the closing visual pass; the repair of the benchmark first profile's four defects authorized as its own reviewed work in the canvas lane; the stencil question ROOT's to decide under a stated rule (decided: the depth mask, no construction change). Standing guidance: independent good taste, the owner's two exemplar products, and consistency of ontology, epistemology, praxeology and axiology.
- `OWNER_DIRECTION_2026-09-19_EIGHT_UX_ITEMS.md`: an unsaved-edits mark; a "Solver · Not solved" fallback chip; the native menu disabled during a busy project request (with its `src-tauri/**` part); a landed save re-derives the integrity record as verified at save; the collapsed drawer strip and D-72's canvas box (item 5, since changed by ROOT: see section 3, item 4); the routing block's home is the inspector; G-11 and G-17 authorized (G-11 after B6 with a result-integrity review); two toolbar departures accepted for now.
- The owner's word to pause, quoted at the top of this file.

## 3. Shell lane: how to finish B3

Read, on the lane branch: `{RUN}/instances/B-SHELL/LANE_LOG.md` (its last section is written for a manager who starts fresh), the sealed child brief `{RUN}/instances/B-SHELL/briefs/B3-FLOORS.md` (SHA-256 `fe295e2b575e651e77b6ec4c75d6ef1455af319119337b4ac96d6008536c0290`), the lane's sealed addendum 4 (`{RUN}/lanes/B-SHELL/briefs/B-SHELL_addendum_4.md`, SHA-256 `08b51c66b50994c92d98658b2d96b1eedd3c66599c791207ea1c7d97801c3088`), and the four retained B3 returns under `{RUN}/instances/B-SHELL/returns/`. ROOT's rulings of this session, all carried in `B3-FLOORS.md`, rest on one principle: **the canvas pane changes size only when the engineer changes the view, the split or the window; never because an inspector docks or a tool arms.**

1. `expectWorkspaceGeometry` in `e2e/ui-foundation-workflows.ts` was loosened inside the candidate by a child; the manager refused it. Restore the original assertions exactly (the drawn canvas, width and height each over 35 % of the window, area larger than the inspector's and the tree's) and make the product meet them.
2. In Both view the docked inspector always takes its width from the table pane (down to 320 px, then the 220 px canvas minimum): canvas pane 603 at 1440 and 531 at 1280, inspector open or closed. A departure from specification §10.9; ROOT's decision, told to the owner, open to the owner.
3. The routing block moves into the inspector now (the owner's ruling, pulled forward from B5): ROOT granted the shell lane a scoped exception in `src/features/viewport/PipeViewport.tsx` for one optional prop that gives a DOM container into which the existing authoring panel is rendered with `createPortal`, unchanged; nothing else under `features/viewport/**`. Arming a node, pipe or component tool opens the inspector to it. The interim armed-tool rule in the tree retires before it reaches `main`.
4. The collapsed drawer strip does not overlay the viewport: the canvas pane keeps D-72's box (603 × 828, 1000 × 828; D-72's "canvas" is the pane) and takes bottom padding equal to the strip; the furniture rules in `styles.css` return to `main`'s bytes. This replaces the mechanism of item 5 of `OWNER_DIRECTION_2026-09-19_EIGHT_UX_ITEMS.md`, which the owner adopted as "the strip overlays the canvas's foot, with the scale bar and the triad lifted clear": ROOT changed it when the frozen floors were settled, because the overlay still covered the viewport's measurement readout and because reading D-72's box as the pane keeps the pane rectangle stable. ROOT's decision, told to the owner in the session on 2026-09-19 and not answered; it is listed in section 6.
5. The split's hit box sits wholly on the table side. Below 1280 px the table drawer is in flow with a visible splitter, bounded 180 to 600 px.
6. The frozen Box16 endpoints test is settled and passing: its setup reproduces the characterized 794 × 559 drawn canvas (window 1688 × 787, stored split 50 %) and asserts that box; endpoints untouched. The DEC-105 precedent is in the test's own header.
7. Assertions about elements the design removed retire with the element, named one by one; two contrast witnesses are listed for the closing pass.

**Open with the owner and blocking B3's merge:** one assertion of the frozen helper cannot hold by the approved design. In Both view with the inspector closed the table pane (737) is larger than the canvas (603) by the 55/45 split itself, so "canvas area larger than the tree's" fails there. ROOT's recommendation, put to the owner on 2026-09-19 and not yet answered: the comparison stays wherever the tree is a side region (Model view's drawer, the narrow fallback) and in Both view is replaced by the 35 % width floor plus the canvas-larger-than-inspector floor. Until the owner answers, that assertion stays original and failing. Ask again first.

After B3's candidate: the Opus code review with a structural checklist, the one Fable design-fidelity review by screenshot for structure only (`drafts/FIDELITY-REVIEW.TEMPLATE.md`), ROOT's look, sweep, pull request, CI, merge. Then, in order: B3A (the unsaved-edits mark; the fallback chip), B3B (the native menu; verified at save), B4, B5 (with the routing block's remainder and the slide-over), B6, then G-11, B7. Two requests from the canvas lane for the shell lane's files wait for B3's freeze: `.viewport-select-target.active { border-color: var(--canvas-selection); }`, and a `hover: { ref, changeSequence }` field in `uiDiagnostics.ts`.

## 4. Canvas lane: what is next

Order: I1 (the first profile's four defects, P4 accepted; defect 1 by the lane's recommendation: internally consistent and honest about the historical revision it binds, not rebound to today's source), then C3, C4, C5 (with C1E's side rings), then the second profile's preparation. Read `{RUN}/instances/B-CANVAS/LANE_LOG.md` and `{RUN}/instances/B-CANVAS/proposals/P4_FIRST_PROFILE_REPAIR.md`.

For the owner, as one package when the lane has prepared it: **the second profile's freeze, with the casing (P1 ASK-4).** Drawn alone, the halo fails the design's pair rule in most samples of dense fixtures (42 of 200 pass at 1,000 pipes in light; 1 of 200 at 10,000); the product passes today only because the first profile's diamond satisfies the rule alone. The package also carries what the shell found: the instrument needs both panes visible at every boundary while D-72's 603 px canvas is the inspector-closed size; it pins the window at 1440 × 920 against D-72's 900; the in-app menu row takes about 33 px in the browser runtime; pane rectangles must not change within a run (ROOT's principle above now guarantees that for inspector and tool changes); `resource-accounting.ts` must open the Project page or use File › Save; the runtime a timed run uses. Review's residual risks worth carrying into C3 and C5 are in `{RUN}/lanes/B-CANVAS/reviews/`. Tell the new canvas manager that two carried items of the lane log's PAUSE STATUS step 5 are overtaken by ROOT's B3 rulings: `--shell-canvas-foot-inset` is removed by B3, and the routing block moves into the inspector in B3, not with B5. The in-app menu row measures 33 px (the later measurement); two earlier records, among them P4's section 4, say 32.

## 5. First steps for the ROOT that continues

As the earlier handoff's section 4, with these changes:

1. Check section 1's table against the repository; if anything differs, stop and tell the owner.
2. Ask the owner the one blocking question of section 3 before launching the shell manager, and launch the manager anyway: every other ruling can be built while the answer is pending.
3. Open one records branch from `origin/main`.
4. Launch a new manager per lane from the lane's records. A launch brief cannot be written into a lane's worktree by ROOT's file tool on this host: seal it in `{RUN}/briefs/` on the records branch, push, and give the manager the `git show` command and the hash (as `briefs/B-SHELL_launch_2026-09-19.md`). ROOT writes lane records through the shell and the scripts in `{RUN}/tools/`, only while that lane's manager is stopped.
5. Host lessons of this session, in addition to the earlier handoff's section 9: a child **resumed by message** is a background child, so its return goes to ROOT and is relayed by file and hash; a fresh foreground child returns to its manager. The browser-test lock has no queue: a waiter can be passed over, so tell the lanes when a sweep is about to start. The Browser pane can be hidden, and a hidden page gets no animation frame, so the canvas stays blank: take ROOT's look with headless Chrome through the lock (the script shape is in `lanes/B-CANVAS/CLOSEOUT_CHECKS_PR1.json`'s note). A three-way merge can silently resurrect a block that `main` both added and removed (it did, in `handleCreateBlankProject`): after merging `main` into a lane that holds the same slice, compare the touched functions with `main`'s byte for byte. The owner's session limit stops agents mid-turn; they resume with context intact, and their uncommitted work stays in the worktree.

## 6. Open with the owner

Blocking B3's merge: the one Both-view assertion (section 3).

Not blocking: ROOT's change to item 5 of the eight items (the strip no longer overlays the viewport; the canvas pane takes padding instead and D-72's box is read as the pane); the second profile's freeze with the casing; the deformation view, as a small drawn proposal at the closing pass; ROOT's decision that the docked inspector takes its width from the table (a departure from specification §10.9); whether "verified at save" needs a persisted field (then it is a typed-interface gap and returns to the owner); the remaining items of the earlier handoff's section 7 that the two direction records did not dispose (the handoff's §7 copy items as each is first seen in the running product; Q-23 to Q-33 of the frames).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
