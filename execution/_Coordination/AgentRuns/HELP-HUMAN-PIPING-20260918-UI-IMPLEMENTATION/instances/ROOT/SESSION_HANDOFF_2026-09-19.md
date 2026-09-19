# Session handoff, 2026-09-19 — SWBPIPE interface implementation

Written by ROOT (HELP_HUMAN, Agent 0; Claude Fable 5.1) at the owner's direction of 2026-09-19T05:01Z, for whoever continues this run: a new session of ROOT, or this one after its context is summarized again. It is a pointer document. If it disagrees with the owner's words, a ruling, a sealed instrument or a retained record, that other thing governs; say so to the owner and correct this file.

Path placeholders, as everywhere in this run: `{REPO_ROOT}` is `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`. No authored file carries a machine path.

## 1. Where the program stands

The owner authorized this program to implement on 2026-09-18 (`{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md`); the piping loop stays on hold; SCA-010 stands. On 2026-09-19 the owner re-sequenced it: **the control layer first, the visual redesign last** (`OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md` beside this file; amendment 1 of `{RUN}/ORCHESTRATION_PLAN.md`).

Merged to `main`, each after an independent Opus code review with no actionable finding, a passing DEC-025 sweep and both required CI checks:

| Pull request | What it did |
|---|---|
| #800 | Tranche A1: the visible rename to SWBPIPE and the D-71 rulings |
| #801 | Slice B1: design tokens into the product (`tokens.json` 1.2, `src/tokens.css`, the `--ui-*` variables mapped) |
| #802 | The A1 follow-ups: `DEC-106` to `DEC-109` (bundle identifier, the acceptance text, two wordings, the preference reset) |
| #803 | Slice B2: the session's state pulled out of `App.tsx` into `workspaceSession.ts` and six state hooks, no behaviour change |

`main` stood at `b9a07040857d2e6d3ec0f6a530697720801c7b59` before the pull request that carries this file.

Three branches are open, all pushed, none with a pull request. Nothing is running: no agent, no sweep, no dev server, no browser-test lock.

| Branch | Head | Git worktree | State |
|---|---|---|---|
| `codex/swbpipe-b-shell-20260918` | `9651fd4212ee6f0ba3c8a202ac8acbcb9d2f865a` | `swbpipe-wt3` | `main` at B2's merge plus two records commits. Addendum 2 (slice B2F) and addendum 3 (the control layer first) are sealed and unread by any manager. Next: slice B2F. |
| `codex/swbpipe-b-canvas-20260918` | `bade9f7accdc84a00d04ba8851beeef8923c0518` | `swbpipe-wt4` | Product head `3a5f7c103`, a labelled pause point, clean. Has not taken `origin/main` since `8e4c5df6e`. Done: slice C1's first part (every canvas colour from a `canvas.*` token, live repaint in both themes), the empty-layer guard, review findings F1 and F2. Not done: F3, the guidance probe T1 (tool unfinished, unreviewed), slice C1E (brief is an unsealed draft), slice C1b. Addendum 4 is sealed and unread. |
| `codex/swbpipe-design-system-v14-20260918` | `4dddd44128e052cfe518f84c0b718e6125cb2b69` | `task-management-gen-pass-518da2` | **Parked** (ROOT's decision DS5-D3, open to the owner). Design system V1.4 with correction 1 applied and checked by ROOT; correction 2 sealed and not executed; the reviewer's backcheck not run; one new finding. Reopened at the closing visual pass. When it next takes `origin/main`, it takes `main`'s versions of `HANDOFF_STATE.md`, `WORK_GRAPH.json` and `briefs/_INDEX.md`. |

A fourth worktree, `swbpipe-wt2`, holds the handoff's branch and is free after it merges; ROOT uses it, detached at a candidate, to run an independent review away from a manager's worktree. Find any worktree's path with `git worktree list`. A worktree that lacks `node_modules` needs `npm ci` and `npm run build:wasm:desktop`, both run from `{WORKING_ROOT}`.

## 2. What ROOT tidied before handing off, and what it left

Done, all records and custody, no product file touched:

- confirmed each of the three stops itself, and retained the canvas manager's and the design author's pause statuses verbatim in their lanes' records;
- parked design system V1.4 with an exact record (`lanes/DESIGN-SYSTEM-05/HANDOFF.md` on its branch) and pushed the branch;
- sealed the two lane addenda that carry the re-sequencing, and pushed both lane branches (the canvas branch had lived only on this machine);
- probed the host for a way to stop relaying children's returns (§6), and wrote the result into both addenda;
- moved ROOT's record-keeping scripts out of its session scratch folder into `{RUN}/tools/`, rewritten to carry no machine path, and two unsealed drafts into `{RUN}/drafts/`;
- brought the run's shared records up to date on one branch (`HANDOFF_STATE.md`, `WORK_GRAPH.json`, `briefs/_INDEX.md`, amendment 1 of the plan).

Left, on purpose:

- **V1.4 is not merged.** ROOT had told the owner it could land after one backcheck. The author's pause note showed the candidate states two figures that are inexact as it stands, that making them exact is a full author round and a review, and that a new reading a hair under 3:1 needs a design decision. Nothing in the control layer needs V1.4.
- **No lane branch took `origin/main`.** It is each manager's first step, with its tests re-run after.
- **No agent was resumed** except a 45-second read-only probe. Every manager, child and reviewer of 2026-09-18 and 2026-09-19 belongs to the session that launched it and cannot be resumed from another.

## 3. The sequence from here

Amendment 1 of `{RUN}/ORCHESTRATION_PLAN.md` is the statement. In brief:

1. **Shell lane:** B2F, B3, B4, B5, B6, B7. Structure and behaviour, layout to the frames, built on the tokens already in the product. No colour, contrast or fine-spacing work. (`lanes/B-SHELL/briefs/B-SHELL_addendum_3.md` on the lane's branch.)
2. **Canvas lane, in parallel:** finish the first pull request as sealed (F3, T1, C1E, C1b), then C2, C3, C4, C5. (`lanes/B-CANVAS/briefs/B-CANVAS_addendum_4.md` on the lane's branch.)
3. **No third stream** run by ROOT beside the two lanes.
4. **The closing visual pass:** the appearance questions to the owner as one package; design system V1.4 reopened and reviewed; token adoption; C6; one fidelity review over every surface; the contrast criteria checked on every touched control.
5. **D-72 qualification,** last in Tranche B. The second profile's freeze goes to the owner before any timed run.
6. **Tranche A2,** with its own plan.

Gates per slice: one fresh read-only Opus code review over the complete frozen diff, PASS with no actionable finding, with a structural checklist for a surface slice; ROOT's own look at the build; the DEC-025 sweep on the clean candidate through the browser-test lock; validators; pull request; CI on the final revision; merge commit. The Fable fidelity review by screenshot runs twice only: on B3's candidate, for structure, and at the closing pass.

## 4. First steps for the ROOT that continues

1. **Read:** `{REPO_ROOT}/AGENTS.md`; `{REPO_ROOT}/agents/AGENT_HELP_HUMAN.md`; this file; the owner's direction beside it; `{RUN}/ORCHESTRATION_PLAN.md` with amendment 1; the design program's handoff `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` (§3's ten constraints, §4's gap classes, §5's semantic changes, §6's verification, §7's open copy items); the open list in `{RUN}/HANDOFF_STATE.md`. Read a lane's brief and addenda before you launch its manager, not before.
2. **Check the state** against §1: `git fetch`; the three heads; each worktree clean; nothing listening on ports 5174, 5175, 5183 or 5184; no `swbpipe-e2e.lock` directory in the git common directory. If anything differs, stop and tell the owner what you found.
3. **Open one records branch** from `origin/main` for the run's shared records. Only one open branch edits `HANDOFF_STATE.md`, `WORK_GRAPH.json` and `briefs/_INDEX.md` at a time; lane branches never do.
4. **Launch a manager per lane** (Fable, WORKING_ITEMS Type 1, the `Agent` tool, general-purpose). Your launch message names the git worktree and the branch, says that it is a new manager with none of the earlier manager's context, points it at the lane brief (`{RUN}/briefs/B-SHELL_shell_lane.md` or `B-CANVAS_canvas_lane.md`) and at every addendum with its SHA-256, and tells it that its latest addendum gives its reading order and first steps. Its first git step is `git merge origin/main`.
5. **At each slice return:** retain it (`{RUN}/tools/retain_return.py`, `{RUN}/tools/lane_slice.py retain`); seal the review brief in the lane's `briefs/` (model your brief on `lanes/B-SHELL/briefs/B2-REVIEW_code_review.md`, which a reviewer used to good effect), index it, and run the reviewer in `swbpipe-wt2` detached at the candidate so the manager's worktree stays its own; retain the review; send findings to the manager, which routes each to the owning child; the same reviewer backchecks. On PASS: the sweep, the closeout JSON, the run record under the deliverable's `_run_records/` with a `_STATUS.md` history line, validators, push, pull request, CI, merge, and tell the other lane to merge `main` at its next clean point.
6. **Look at each build yourself** before reporting a slice done: a dev server from the lane's worktree on a port that is not 5174 or 5175, both themes. `.claude/launch.json` is git-ignored and local; write your own.

## 5. Conventions that bind

- **Briefs and returns.** Every instruction to an agent is a file, written, hashed and indexed before launch. Every return is retained verbatim with its SHA-256 and the model that actually ran. A correction goes to the owning child by message, never to a new child. Type 2 never delegates. Siblings never message each other.
- **One writer per path at a time.** Shell lane: `{DESKTOP}/src/**` except `features/viewport/**`, and it owns `App.tsx` and `styles.css`. Canvas lane: `features/viewport/**` and `features/workspace/modelIndex.ts`, with the one scoped stylesheet exception its addendum 2 grants. ROOT writes in a lane's worktree only while its manager is stopped, and only under `lanes/<LANE>/`.
- **Validators, chained with `&&` before every commit,** from `{REPO_ROOT}`: `python3 tools/validation/validate_path_anchors.py`, `python3 tools/validation/validate_claims_language.py`, `python3 tools/validation/validate_instruction_tranche_manifest.py --base origin/main --head HEAD --added-manifests-only`. The path-anchor validator reads a quoted path pattern as a machine path; `retain_return.py` declares a substitution for that case.
- **The sweep,** from `{WORKING_ROOT}`: `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh python3 tools/release/run_evidence_sweep.py --execute --require-capability host`. About seventeen minutes, five surfaces, bound to a clean commit. Touch nothing in that worktree while it runs. Not run for a change that is records or design only.
- **The browser-test lock.** Every Playwright run and every server on a test port goes through `with_e2e_lock.sh`. The Playwright configurations use ports 5174 and 5175.
- **CI.** Two required checks, "Desktop E2E (source mode)" (about 25 minutes) and "harness". Wait with `gh pr checks <n> --watch --interval 30` in the background.
- **Git.** The owner's standing authorization of 2026-09-12 in `AGENTS.md` covers commit, push, pull request and merge inside authorized work. `codex/` branches; merge commits; the commit trailer and the pull-request line the session's attribution notice gives; "No owner review is implied" in every pull-request body.
- **Never:** alter a tolerance, an oracle or a limit to obtain a pass; weaken or delete a test; fix one of the four named defects of the benchmark's first profile in passing; edit `fixture-manifest.json` or let `viewportSelectionPresentation.ts` drift from the bytes `full-cohort-controller.spec.ts` reads; start a build during a timed measurement.
- **Copy.** "Accept", never "Approve". None of certify, seal, approve, authenticate, comply, compliant or sign-off as a control. Canadian English. A status label is never shown without its token reachable and its authority domain (`DEC-102`).
- **Records.** Governed records end with the claim fence line this file ends with. The owner's words are extracted from the stored transcript, hashed and labelled "stored-transcript custody, not transport bytes"; ROOT's readings are labelled as ROOT's. A task notification or a host status prompt is never the owner's input.
- **Host habits.** Never read an agent's output file into the conversation; use `{RUN}/tools/relay.py` or `retain_return.py`. Check the time with `date -u`. In zsh, quote glob patterns.

## 6. How the Agent 0, 1 and 2 behaviours worked

The owner asked, on 2026-09-19, whether the behaviours are working as intended, whether ROOT carrying the Type 2 returns to the managers is a failure of the paradigm, and who launches the managers' children.

**What the record shows.**

- **Who launched what.** Each manager sealed and launched its own Type 2 children: the shell manager its state-extraction implementer; the canvas manager its palette implementer, its instrument inventory and its probe child. ROOT launched the two managers, every independent reviewer, and the flat Type 2 work outside the lanes (Tranche A1's three children, the follow-ups, the tokens slice, the design-system author). ROOT launching the reviews is deliberate: a review stays independent of the manager's account of the work.
- **The roles held.** No Type 2 delegated. Siblings never messaged each other. Corrections went to the owning child with its context intact. Managers brought decisions up as proposals with consequences; ROOT decided what was its to decide, labelled each decision as its own and open to the owner, and carried the rest to the owner.
- **The transport did not match the tree.** The host lists a nested child under the top-level session. A child launched in the background reports its completion to ROOT when its manager has stopped, and to the manager only when the manager is still in its turn. The lane briefs said "background", which was ROOT's wording. So ROOT became the carrier: it extracted each return verbatim to a file, hashed it, and sent the manager the path, the hash and a short digest labelled as ROOT's reading; the manager verified the hash and retained the return itself. Authority stayed where the paradigm puts it. The cost was real: every child's return also landed in ROOT's context, a lane waited on ROOT's attention, and ROOT read a child's work before its manager did, which can lean on the manager's judgment even when labelled.
- **ROOT ran a third stream itself.** The design-system amendment was ROOT acting as a manager: its own child, its own reviewer, its own corrections, in parallel with two lanes and the relays. That is Type 1 work done by Type 0, and it is where ROOT's attention and context went.
- **Width met the usage limit.** Three Fable-led streams with Opus reviewers ran at once and the owner's session limit was reached twice. Agents stopped by the limit were resumable with their context intact, and both graceful pauses worked.

**ROOT's assessment.** The paradigm did not fail; the brief's launch mode did not fit the host, and the paradigm's flexibility absorbed it at a cost ROOT should not have kept paying. What a manager is on this host matters more than the diagram: it is not a standing supervisor, it is a resumable context that holds a lane's detail so that ROOT does not have to. The canvas manager held the benchmark instrument's inventory, three proposals and a probe design; the shell manager held a three-stage audit. Without them that detail would have been ROOT's, and ROOT's context was already summarized several times in two days. That is the value of Type 1 here, and it is worth keeping.

**What changes.** Managers launch a child in the foreground when its return is their next input. ROOT probed it on 2026-09-19T05:14Z (`PROBE_NESTED_FOREGROUND_2026-09-19.md` beside this file): the nested child's reply reached its parent directly, in the same turn, and nothing came to ROOT. Both lanes' addenda carry the new clause; the relay protocol stays for background children. ROOT runs no stream of its own beside the lanes. ROOT keeps dispatching the independent reviews. A new session starts new managers from the lanes' records, which is what those records are for.

**What is not known.** Whether a foreground child that runs for a long time is cut off by the host. Whether a foreground nested child is listed under ROOT's session, which decides whether ROOT can reach it with a pause while its manager waits. A manager waiting on a foreground child reads no message until the child returns, so a pause takes effect at that return.

## 7. Open with the owner, none blocking

ROOT's decisions, each labelled as ROOT's and open to the owner:

- **DS5-D1:** the design system's disabled ink is lifted (`#8f949a` light, `#767b80` dark).
- **DS5-D2:** the contrast sweep lists the check box pairs under a washed row or menu item; the alternative, no pressed wash on such a row, not taken.
- **DS5-D3:** design system V1.4 is parked with its branch and not merged now.
- **C1-D1:** the deformed shape stays on `canvas.vector` as a named provisional binding until §6.8's deformation view is built.

For the closing visual pass, as one package: the pressed-row finding (marks and offers on a pressed zebra row read 2.9985:1 in light; ROOT leans to lightening `pressed.wash`, undecided); the text contrast target (design system §8 item 6); the casing's look (canvas proposal P1 ASK-4); the derived silhouette shades (worst 1.61:1 on `canvas.bg`, a node or rigid element in dark); the canvas roles the design does not name; §6.8's deformation view; `canvas.edgeAlt`.

Rendering and measurement: the stencil buffer and draw-over (P1 ASK-5), a renderer-construction question under the handoff's constraint 5, which slice C2 brings with numbers and options; the second profile's freeze, one package before any timed run; four defects of the benchmark's first profile that predate the lane, named in the canvas lane's records and fixed by no one.

Copy and wording: the artifact label "External-run evidence"; the handoff's §7 items as each is first seen in the running product; the Review page's two-chip reading with no rule pack loaded (slice B7); the claims lint's `BS-ACCEPT` wording, an instruction-surface edit that needs its own tranche manifest.

Gaps, returned per gap: G-11 (Stale runs) and G-17 (persisted view state) first.

Sequencing: whether Tranche A2 may overlap the closing visual pass.

## 8. Where things are

| What | Where |
|---|---|
| The design basis, read and never edited | `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md`; the design system under `{DESIGN}/instances/DESIGN-SYSTEM/`; the specification and the operations map under `{DESIGN}/instances/UX-SPEC/` (`UX_SPEC_V1.md`, `OPERATIONS_MAP.md`); the frames under `{DESIGN}/instances/MOCKS/` |
| The plan and amendment 1; the dated log; the work graph; the index of sealed briefs | `{RUN}/ORCHESTRATION_PLAN.md`; `{RUN}/HANDOFF_STATE.md`; `{RUN}/WORK_GRAPH.json`; `{RUN}/briefs/_INDEX.md` |
| The owner's words, recorded | `{RUN}/instances/ROOT/` |
| ROOT's records for a lane; the manager's own records | `{RUN}/lanes/<LANE>/` (index, graph, handoff, ROOT's sealed addenda, retained slice returns and reviews); `{RUN}/instances/<LANE>/` (its children's briefs and returns, proposals, tools, the lane log). The shell lane's are on `main`; the canvas lane's and the design system's are on their branches until a first pull request |
| Scripts | `{RUN}/tools/`: `with_e2e_lock.sh`, `relay.py`, `retain_return.py`, `lane_slice.py`, `find_acceptance_texts.py` |
| Unsealed drafts | `{RUN}/drafts/`: the token adoption brief and the fidelity review template. A draft is not an instruction |
| Sweep results | `{WORKING_ROOT}/validation/evidence/sweeps/` |
| Decision rows `DEC-099` to `DEC-109` | `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |

ROOT's session scratch folder did not move and nothing depends on it.

## 9. Things that will bite

- On `main` the console still shows two `THREE.Object3D.add` errors from an empty layer replacement. They predate every slice and are repaired on the canvas branch (`5bfdb9909`); they leave `main` with the canvas lane's first pull request.
- The first screenshot after a load can catch the white `.viewport-shell` background before the first paint. It is the stylesheet item of slice C1b and not a defect of whatever slice you are looking at.
- `e2e/ui-foundation/resource-lifecycle-source.benchmark.ts` lists only `src/App.tsx` among the executed files; after slice B2 the session code lives in `workspaceSession.ts` and six state hooks. It is the canvas lane's file, for the second profile.
- A child that touches a `*.spec.ts` runs both Playwright lanes, source and dist. The first sweep of Tranche A1 failed on a missed dist twin.
- The sweep summary holds exit codes and no test counts. Take counts from the run's own logs.
- `WORK_GRAPH.json` and the lane graphs are written with two-space indentation, `ensure_ascii` off and a trailing newline.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
