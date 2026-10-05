# LR-LOOPINIT: independent review of PR #1092 (piping LOOP_INIT binding form; T3 handoff as init steer)

**Reviewer:** LR, a Chirality Type 2 TASK dispatched by HELP_HUMAN (ROOT) for run `PIPING-LOOP-INIT-20261005`, under `RUN/BRIEF_LR.md`. Model: Claude Opus 5.5 (`claude-opus-5-5`), as a Claude Code subagent. I wrote none of the reviewed text. I made no Git writes, used no network beyond `gh` reads, and wrote only this file, `SHA256SUMS` beside it, and logs under `WT/scratch/lr_loopinit_01/`.

**Placeholders.** `WT` = the T3 worktree root; `NUM` = `WT/numerics`; `RUN` = `NUM/projects/chirality-piping/execution/_Coordination/AgentRuns/PIPING-LOOP-INIT-20261005`; `V4RUN` = App v4's `APP-V4-GRAPH-CLOSURE-20261004` run; `P` = `projects/chirality-piping`; `T3` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3`; `RR` = `T3/ROOT_RULINGS_V1.md`; `WG` = `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`; `COORD` = `P/execution/_Coordination/_COORDINATION.md`; AUM, FB = the Agent User Manual v3 and the Field Book v1.

**Candidate.** PR https://github.com/sgttomas/chirality/pull/1092, head `605adf726d569b3f0f7166fcfe7558f09982b7a0` (draft, open, MERGEABLE), one commit on main `6479bf110a3b1f6dd9afecab83bc847237e42240`. NUM head `8225f8f2a2`.

| File at the head | sha256 (prefix) |
|---|---|
| `P/loop/LOOP_INIT.md` | `5c20a16fef96…` (matches RULINGS) |
| `RUN/LOOP_INIT_MAPPING.md` | `12c34aa82533…` |
| `RUN/LOOP_INIT_PROPOSED.md` | `072f6d36f367…` |
| `RUN/CONSISTENCY_EDITS.md` | `3021790802cc…` |
| `RUN/RULINGS.md` | `6cd178e87078…` |
| `RUN/OWNER_DECISIONS.md` | `d1f24dadeec8…` |

## Verdict

**REPAIR.** One MAJOR finding: the new LOOP_INIT's only standing constraint points stage-gate assessment at the wrong criteria. Its fix is a one-sentence edit to a path the manifest already covers. The mapping is sound, no load-bearing rule is lost, every path and workflow name checks, the validators and GEN-8 pass, and the T3 records are accurate apart from the items below.

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| MAJOR | 1 |
| MINOR | 3 |
| NOTE | 9 |

## Findings

### MAJOR-1: The stage-gate constraint sends an assessor to PRD §24's milestone criteria, but Piping's gate criteria are the ones ruled in COORD

**Evidence.**
- LOOP_INIT lines 84–87: "Piping's stage gates are the exit criteria of the release milestones in `docs/PRD.md` §24. `execution/_Coordination/_COORDINATION.md`, under "Current Target Stage", records the target stage; only the owner's approved update advances it."
- COORD "Current Target Stage (ruled record)" records the target as "PRD R5 exit criterion" and states its exit criteria itself, "expressed in amended-PRD tokens per `DEC-080`": the actor-neutral clean-checkout reproduction criterion of "amended PRD (v0.3) §24 R6", and "the public repository contains no known protected standards data (v0.3 §20.1 / D-20 lineage)". The label follows the historical v0.1 PRD ("### 22.6 Release R5: Engineering Beta" in `P/docs/_history/PRD_v0.1.md`, exit criteria "External engineers can reproduce validation examples" and "Public repository contains no known protected standards data"), through the D-21 Annex A crosswalk.
- `P/docs/PRD.md` §24's "### R5: Piping Components and Nonlinear Supports" has different exit criteria: "component data appears in reports with provenance" and "nonlinear cases converge or report actionable diagnostics".
- `P/docs/PLAN.md` names COORD ("Working Desktop Application Standard") as "**authoritative for the stage gate**". AUM §5: "Stage gates remain human-managed milestones under the project's own records."

**Consequence.** An agent that follows LOOP_INIT reads the recorded target "R5" against §24's R5 and prepares or assesses stage evidence against the wrong criteria. Advancement still needs the owner, so nothing advances wrongly, but the one standing constraint misdirects the work it exists to steer. Mapping row N13 and RULINGS ("HELP_HUMAN checked … the stage-gate source") carry the same reading.

**Fix.** Replace the bullet with a pointer to the ruled record, for example: "Piping's stage gate is the target stage and the exit criteria recorded in `execution/_Coordination/_COORDINATION.md` under "Current Target Stage". Assess against those criteria, not against a `docs/PRD.md` §24 milestone read by its label; only the owner's approved update advances the target." Note the correction in a RULINGS addendum (N13).

### MINOR-1: "It records each adopted amendment" is not true of SCA-010

**Evidence.** LOOP_INIT lines 54–57 say the decomposition "records each adopted amendment, and `execution/_ScopeChange/_LATEST.md` selects the accepted scope change." The decomposition's revision notes record SCA-001 to SCA-009 and SCA-011 (v0.4–v0.13) and D-77 (v0.14). SCA-010 (accepted by the owner on 2026-09-18, PRD-only, "accepted, not yet executed") is not recorded there; its `Impact_Assessment.md` says "revision 0.12 stands". `_ScopeChange/_LATEST.md` selects "only the accepted SCA-011 amendment" and leaves earlier snapshots as historical authority.

**Fix.** "Its revision notes record each amendment that changed it; `execution/_ScopeChange/` holds every amendment, and its `_LATEST.md` selects the latest accepted one."

### MINOR-2: The manifest's M6 record names the changed instruction files as notice routes, and its rationale omits the workflow sentence RULINGS says it records

**Evidence.**
- `m6_notice.disposition: routed` with `routed_to` = `P/loop/LOOP_INIT.md` and `init/dev-loop-init-prompt.md`. The validator's schema documents `routed_to` as "notice paths", and of the 153 manifests this is the only one whose routes are not `NOTICE_*.md` files. Piping-only precedents route a notice to Piping's own `_Coordination` (PIPING-LOOP-WORKGRAPH-20260919, PIPING-REMAINING-RETIREMENT-20260923); an own-loop change with no other affected loop has also used `none-required` (APP-V4-LOOP-ENTRY-20260928). No notice was written, so "routed" is not truthful as recorded.
- RULINGS ruling 5: "Section C, the manual citations and construct's introduction sentence, is outside this tranche. The manifest's notice rationale records it." The rationale records only the AUM citations. `workflows/construct-local-work-graph/WORKFLOW.md` line 11 ("`LOOP_INIT.md` supplies the evergreen procedure for recovering, constructing and following its graph") becomes untrue for Piping with this change, as it already is for App v4 (CONSISTENCY_EDITS C3).
- G4 passes in every mode; this is a truthfulness finding, not a validator failure.

**Fix.** Either set `disposition: none-required` with `routed_to: []`, or write a Piping notice and route it. In either case add one rationale sentence that C3 needs a `create-workflow` revision of construct's introduction.

### MINOR-3: The product-PR gate set and Git rules no longer have a live consolidated statement, although the new ruling says they live in the rulings

**Evidence.**
- The old handoff's "Rules that continue", on main, held "Gates before a main merge" (complete-diff review with same-reviewer confirmation; hosted CI and the full-SHA dispatch; the 40-manifest suite before the freeze; an exact-final-head Mac DEC-025 compared per test against a fresh main baseline; GEN-8; Pass B with confirmation and T9/both-entry for D1 call-graph changes) and the Git rules (TASKs make no Git writes; never rebase or force-push; product PRs `--merge --match-head-commit`; no auto-merge; check main is unmoved). The PR rewrites the handoff without them.
- RR "Owner direction: proportionate CI…" says "**The full gate set stays** for PRs that change source, tests, CI, tools or the portability policy". The set it refers to is enumerated only in that old handoff text. RR's other gate statements are unit-specific (U9/#1082; "I61's U8 plan ruled…", item 7, for U8 only).
- The new steer spells out the records-only PR gates but not the product-PR gates, although S-I1's PR and the T6 slice's PR come first. `BRIEFS/I73_S_I1.md` gives no PR gates. The old handoff's "S-I1 … needs no D1 gates, but does need DEC-025 and hosted CI" is now only in Git history. Project `AGENTS.md` keeps review, the DEC-025 sweep and required CI; the T3-specific parts (full-SHA dispatch, per-test comparison with a fresh main baseline, GEN-8 on product PRs, carry-over by ruling, merge method) have no live consolidated home.
- RR "Owner direction: piping LOOP_INIT in the binding form…" says the handoff "no longer restates the loop, the gates or the rules, which live in the workflows, project `AGENTS.md`, LOOP_INIT and these rulings."

**Fix.** Append a short RR ruling that restates the product-PR gate set and the Git/merge rules, with their bases (RR is append-only, so append). List it under the WG section's "T3 rulings in force", and add one steer bullet pointing to it.

### NOTEs

- **NOTE-1: The PR tree is not NUM's tree; it is NUM's plus five App v4 files from main.** `git diff 8225f8f2a2 605adf726d` shows only the five files of #1090 (`decision_view.rs`, `record_relations.rs` ×2, two APP-V4-GROUP-A records). NUM last absorbed main at `928145450a` (09:03 −06:00); #1090 merged at 09:23. The PR's delta from main is exactly the six instruction paths plus `P/execution/`, and the PR equals NUM on every path of that delta. The construction is right, but the dispatch's and brief's premise that the trees are equal is not. The WG "Position" ("Its maintained source equals main's") and steer item 1 have been stale since #1090. Steer step 4 ("whether main moved … absorb it into NUM first") catches this; "equalled main's at its last absorb" would stay true.
- **NOTE-2: Main has moved again,** to `01809013ae` (#1091). `gh api …/compare` lists 35 files, all under `projects/chirality-app-v4/`, disjoint from the PR, and the PR remains MERGEABLE. This meets the premise of T3's carry-over rule ("RV100 passes #1088 at H…": another project's directory only; nothing in `P/`, `tools/`, `.github/`, the policy or root build files). The rule also needs GEN-8 on the local combination, which I did not run, because building the combination needs a fetch.
- **NOTE-3: Ruling 3 is sound.** "If you are unsure whether a section matters, read it." keeps the reading decision with the agent, as the owner directed ("the agent decides when to visit it"). It removes the tension with Root `AGENTS.md` ("uncertainty alone does not require an extra prompt"), and "Read the section, not the chapter" bounds it. App v4's LOOP_INIT still says "ask the human", so the shared paragraph now differs between the two loops. Align App v4 at its next instruction change, or take mapping O1 (an AUM home) at the next manual revision.
- **NOTE-4: The LOOP_RECEIPTS statement is accurate.** The last entry is Receipt-162, and the 2026-09-23 notice says the ledger "remains historical and … is not appended by this method". Two residues remain. The ledger's own header still says "Append-only", and Receipt-162 says "Receipt-161 remains reserved to the unrelated unmerged continuation" (branch `codex/swbpipe-continuation-20260919`, paused 2026-09-21). If that continuation resumes, it may not append; its receipt goes to its own run.
- **NOTE-5: Some three-part-test residue remains,** in the same shape as App v4's accepted file.
  - The opening says the file "states only what is specific to Piping and stated in none of them", which is stronger than App v4's wording, but several clauses restate other layers:
    - "Record what you read … as Root `AGENTS.md` requires";
    - the MEMORY clause's first half (AUM §13);
    - "`bounded-reconciliation`, for the closeout comparisons" (construct §3);
    - "through WORKING_ITEMS" (AUM §13);
    - "Instructions … govern; the manuals explain" (the alignment-manual README).
  - The `WORKING_ROOT` sentence repeats project `AGENTS.md`.
  - "When to read further" is general by design (ruling 6).
  - Dropping "and stated in none of them" would make the self-description true. Alternatively, trim the restatements.
- **NOTE-6: Some stale statements are left out correctly, but nothing routes them for correction.**
  - Section B is correctly left out. B1–B3 were stale before this change (since 2026-09-19/23).
  - I found one more of the same kind: `P/docs/contributor_guide/index.md` row 8, "`loop/LOOP_INIT.md`, its selected current work graph" (written 2026-09-22, stale since the evergreen change).
  - Section C is correctly out of scope: manual authorship stays with the owner, and workflow revisions need `create-workflow`.
  - B is recorded only in this run's records. A Task Management intake or the next Piping instruction tranche would keep it from being lost.
- **NOTE-7: Some host details in the handoff need a check before the next session.**
  - "with no swap": T3's own `OPERATING_NOTES_2026-09-30.md` §4 says "no swap partition; macOS showed about 1 GiB of dynamic swap". `sysctl vm.swapusage` now shows 1024 MB total, about 91 MB used.
  - "Worktrees: Only those two", and the steer's check "the worktrees are `numerics` and `sweep-skewpin`": `WT/loop-init-pr` is a third registered worktree. ROOT should remove it after the merge.
  - **Host note from this review:** the permitted `test_ci_e2e_plan.py` run built two small crates. It left gitignored `target/` folders, about 79 MB, under `WT/loop-init-pr/P/core/serialization/canonical_json/` and `WT/loop-init-pr/P/core/units/`. My removal was refused by the host's permission check, so they go when the worktree is removed.
  - A few host-table imperatives duplicate the steer ("It must run during any build", "Never pruned without the owner"). They are tolerable as machine-local notes.
- **NOTE-8: Small gaps in the WG section and the steer.**
  - **Owner decisions.** The WG's "Owner decisions in force" omits the 2026-10-05 direction that all T3 scratch lives in `WT/scratch`. `T3/IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md` step 0 and `BRIEFS/U8_COMMON.md` carry it, so it is not lost.
  - **Bases of the steer's practices.** The steer says the practices' "basis is in the T3 rulings the graph lists". For "Product PRs are cut compactly from main, with maintained-source equality", the basis is in rulings the graph does not list ("U9 planned and ruled: the D1 milestone PR", decision 2, and RR:6921). For "verify every return yourself", the basis is practice recorded throughout RR. "In T3's rulings" would be exact.
  - **T3 row.** The row's historical 2026-10-03 tail still says the handoff and ROOT_CURRENT "own details and owner decisions". This is acceptable as history, now that ROOT_CURRENT is a retirement pointer.
- **NOTE-9: A4 is acceptable shorthand.** It says "the project `AGENTS.md` holds its fences". Project `AGENTS.md` binds F-PIP-1 to F-PIP-4, and their definitions are in `loop/WORKPLAN_2026-07-18b_piping_loop.md`.

## 1. The new `P/loop/LOOP_INIT.md`

**The three-part test.** I read each sentence against the owner's test in `V4RUN/OWNER_DECISIONS.md` and `RUN/OWNER_DECISIONS.md`.
- **The sentences pass:** entry steps 1–3, the Methods bindings and permissions, the record pointers, the MEMORY creation grant and the LOOP_RECEIPTS convention.
- **Residue:** NOTE-5, which is App v4's accepted pattern.
- **The stage-gate bullet** is specific and instructs, but is inaccurate (MAJOR-1).
- **Owner constraints:** no routers, no edition pin (editions come from the README), and no new record type.
- **Entry step 2** ("Read project `AGENTS.md` in full before you write anything") is justified. Piping's init prompt does not read project `AGENTS.md`, the entry map gives only the AUM headings, and the write fences apply before the first write.

**No load-bearing rule is lost.**
- **Class (a) quotes.** I re-ran the mapping's destination quotes with a script, `WT/scratch/lr_loopinit_01/quotecheck.py`, which collapses whitespace and attributes each quote to the source label before it, against the files at the head.
  - **Result:** 123 class-(a) quotes, from 71 rows, were found verbatim in their attributed sources: construct 29, AUM 28, BR 20, FB 17, project `AGENTS.md` 14 (at the head, after A1–A3), Root `AGENTS.md` 7, TM 5, SPEC 2 and the init prompt 1.
  - **Unmatched by the script:** 24 strings. All are parser artifacts: draft section names, owner quotes without a label, and pairs mis-split by nested quotes in rows 8, 9 and 29. I checked the five record-pointer quotes in those rows by hand with `grep -F`, and all were found.
- **Substance.** The rows I read in substance, not just words, carry their rules: 22, 26, 47, 53, 56, 59, 62–67, 80, 83, 89 and 92.
- **Class (b), every row** (1, 2, 7–11, 13–15, 17) lands in the draft.
- **Class (d), every row** is correctly retired:
  - row 4, by the owner's reading;
  - row 29, because `_DAG/_LATEST.md` names "approved_active_graph_authority";
  - row 36, which goes with the removed steps;
  - row 71, which is editorial.
- **The MEMORY creation grant** (ruling 1) is safe today: the five deliverables without `MEMORY.md` (DEL-04-07, DEL-07-09, DEL-07-11, DEL-07-12, DEL-16-06) are all OPEN.

**Paths and names.**
- **Paths.** Every path exists at the head:
  - `docs/PRD.md`;
  - `_Decomposition/SOFTWARE_DECOMP.md` and its `_LATEST.md` (revision 0.14);
  - `_ScopeChange/_LATEST.md`;
  - `_DAG/_LATEST.md`;
  - 106 deliverable folders: 97 `ScopeOfWork.md`, 8 PKG-00 `ArchitectureBasis.md`, DEL-07-09 bespoke; 106 `_DEPENDENCIES.md`, 98 `Dependencies.csv`, 106 `_STATUS.md`, 101 `MEMORY.md`;
  - `_DECISIONS/_REGISTER.md`, whose header says how rulings are recorded;
  - 74 `NOTICE_*.md`;
  - `software-workflow.json`, and project `AGENTS.md`'s "## Software checks";
  - `_TaskManagement/REGISTER.csv`;
  - `WorkGraphs/`;
  - `loop/LOOP_RECEIPTS.md`;
  - COORD;
  - `{REPO_ROOT}/docs/alignment-manual/README.md`.
- **Workflows.** All eight workflow names are in `workflows/index.json` `methods` as `kind: workflow`, `source: bundled`, `sourceRootId: chirality-root`.
- **Machine-local content and current state.** None. "Receipt 162" names where a closed ledger ends, and "PKG-00" names a structure.

**Item by item.**
- **The stage-gate constraint** is not accurate (MAJOR-1).
- **The LOOP_RECEIPTS statement** is accurate (NOTE-4).
- **Ruling 3** is sound (NOTE-3).
- **The record pointers** are accurate apart from MINOR-1.

## 2. Consistency edits A1–A7

**Each edit is minimal and true.**
- **The diff matches the plan.** The PR's diff against main changes exactly the "before" text of each edit into its "after" text. Nothing else changes in those files.
- **A1–A3** (project `AGENTS.md`) describe LOOP_INIT as binding, not owning. The unchanged lines 65–67 and 87–88 stay true.
- **A4** (the Root launcher §5 prose) is true (NOTE-9). The tagged block is unchanged, and `validate_instruction_entrypoints.py` passes, so it still byte-matches `P/init/dev-loop-init-prompt.md`.
- **A5** (COORD header), **A6** (the DEL-11-05 banner) and **A7** (the newest WORKPLAN's navigation) are true. A7 also removes the clause "with LOOP_INIT pointing to the selected graph", stale since 2026-09-23. The owner-adopted retirement text is unchanged.

**Nothing else in live instructions becomes untrue,** apart from what is recorded already.
- **What I searched:** `LOOP_INIT` in `P/` outside `execution/`, Root `init/`, `docs/SPEC.md`, `workflows/` and `agents/`.
- **Still true:**
  - `P/init/dev-loop-init-prompt.md` and both `taskmgmt-init-prompt.md` files;
  - the Root launcher catalog;
  - SPEC line 998 ("LOOP_INIT remains evergreen…");
  - the work-graph template.
- **Becomes untrue:** construct's introduction sentence (C3; MINOR-2).
- **Already stale:** B1–B3 and the contributor guide's row 8 (NOTE-6). `AGENTIC_DEVELOPMENT_WORKFLOW.md` line 51's table row is old text that its own banner disclaims.
- **The test that reads LOOP_INIT** (`P/tests/test_ci_e2e_plan.py`) uses only the path, and it passes.

**A6: no active concordance run freezes DEL-11-05.**
- DEL-11-05's `_STATUS.md` reads "**Current State:** IN_PROGRESS" (last updated 2026-09-22).
- `RECON_2026-09-21_WHOLE_CORPUS` last changed at `a89b5ddecf` ("Close App and Piping record reconciliation") and `e715b2055c` (the D-74 closeout), both on 2026-09-22. Piping development has continued since then.
- The other concordance runs (`DELIVERABLE_CONCORDANCE_2026-07-11_1305` and three SCOPED/TM runs) are older.
- An earlier instruction tranche (`29decb9fee`) and D-77/D-78 (`b53c0f8c4b`, `b529c22aec`) edited the same file without a MEMORY row. A6 follows that precedent.

**Sections B and C** are correctly left out (NOTE-6). However, ruling 5's claim about the rationale is inexact (MINOR-2).

## 3. The tranche manifest

- **The validator passes,** run in `WT/loop-init-pr` with the Piping venv (logs in `g4_*.log`):
  - **CI mode:** G4 PASS (153 manifests schema-valid).
  - **Diff mode,** `--base 6479bf110a… --head 605adf726d… --tranche PIPING-LOOP-INIT-20261005`: PASS. 47 changed paths, 2 on the instruction surface, covered.
  - **Governed mode,** `--added-manifests-only`: PASS.
  - **INFO only:** the five project paths are over-declaration.
- **Authorization quotes.** "Make appropriate changes to the LOOP_INIT and your own handoff steering instruction ... carry out those actions to get things set up properly." matches `RUN/OWNER_DECISIONS.md`, with the ellipsis eliding the parenthetical and "So don't start the next session, I'll do that, but otherwise". "No routers" and "No don't pin to editions" match `V4RUN/OWNER_DECISIONS.md`.
- **The notice disposition.** The rationale's facts hold. I found no other loop's live instructions or basis records that pin Piping's LOOP_INIT or the Root launcher's §5 paragraph; the other hits are historical records. The recorded disposition and routes are not truthful as recorded (MINOR-2).
- **The review field** names this file.

## 4. T3 records

**The WG section "T3 current route (numerical integrity)" matches T3's records.**
- **Nodes.** U8, SI1, T6S, B0, B1/B6, B2/B3/B4, B7, B8, SI2/F2b/F3 and G10, with their needs, assignments and states, match:
  - the old handoff (main `6479bf110a`);
  - RR "I61's U8 plan ruled…" decisions 1–14;
  - PLAN §§1–3 (`I61/u8_plan_01/PLAN.md`: §2 "The F2a-breadth roadmap", §2.2 "The end of F2a…", §3 "S-I1's readiness", §4 "Main's movement");
  - `BRIEFS/` (U8_COMMON, I68, I69_I70_I71, I72, I73, I74, RV97, RV98 and RV99 all exist).
  - "RR near line 10407" is the D-6 lock residual that RR:11946 cites.
- **Owner-held choices.** All eight match the old handoff.
- **Owner decisions.** The F2a order, the T6 pull-forward, proportionate CI, cleanup, G10 and the earlier T1/D-3/D-6/D-7 match RR and the old handoff (NOTE-8).
- **IDs.** I75 and RV101 are next; RV100 reviewed #1088.
- **Ruling headings.** All seven quoted headings exist verbatim as prefixes of RR headings, at lines 11769, 11898, 12179, 12198, 12111, 12134 and 12096. Each supports the item it is cited for. "Ruled for T3's later freezes: the full 40-manifest suite…" is at RR:11769ff.
- **Routed notes.** They match the old handoff's open items plus RV95 N-6.
- **The anchor** `#t3-current-route-numerical-integrity` resolves.

**The T3 row's status** points to the section.

**The retired `ROOT_CURRENT.md`** is a correct retirement pointer. Every item of the old file (closed, open, next, branches, IDs) is carried by the WG section.

**The ephemeral handoff** is accurate apart from NOTE-7. It no longer restates the loop. MINOR-3 is about what its rewrite removed.

**`HANDOFF_2026-10-05_PROMPT.md`.**
- **The init block is byte-identical** to `P/init/dev-loop-init-prompt.md`, except for the steer. The prefix up to `Steer (this run):` is equal, and the suffix `</init-prompt>\n` is equal; checked byte-wise in Python.
- **The steer is consistent with LOOP_INIT and project `AGENTS.md`.** It names the undertaking and its graph, which completes LOOP_INIT entry step 3. It adds ckw in proportion, as LOOP_INIT's Methods do. It contradicts neither file.
- **Its only stale statement** is "Its maintained source equals main's" (NOTE-1), which its own step 4 corrects.
- **Each listed practice has a basis in RR:**
  - the guard and one cargo job: I61 decisions 1 and 13;
  - TASKs run no DEC-025, native or solver-at-scale jobs: I61 decision 1;
  - `run_dec025.sh` and `ALL-DONE`: "RV100 passes #1088…" N-6;
  - the full suite before a freeze: "DEC-025 on F…";
  - compact product PRs: "U9 planned and ruled…" decision 2, RR:6921;
  - never merging NUM: "A follow-up records-only PR…";
  - records-only PRs: "Owner direction: proportionate CI…", A-1, E-4 and N-1;
  - fresh IDs: I61 "Dispatches prepared…";
  - owner-held choices: I61 decisions 9 and 12;
  - cleanup: "Handoff prepared…" and "Stray scratch gathered…".
  
  Some of these bases are outside the graph's list (NOTE-8). The product-PR gate set is missing (MINOR-3).

**The T3 ruling** "Owner direction: piping LOOP_INIT in the binding form; T3's handoff becomes an init steer; ROOT_CURRENT retired" is accurate apart from its claim that the gates and rules live in the rulings (MINOR-3). Its owner quotes match `RUN/OWNER_DECISIONS.md`.

**RR is append-only.** Main's copy (1,042,052 B) is a byte prefix of the PR's (1,047,134 B). The PR appends "#1088 squash-merged…" and the new ruling.

## 5. Publication and portability

- **GEN-8:** `python -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8`.
  - In `WT/loop-init-pr`, a clean Git checkout at `605adf726d`: 1 passed (`gen8.log`).
  - On a clean `git archive` of the head with `CHIRALITY_REQUIRE_LIVE_TESTS=1`: 1 passed (`gen8_archive.log`). This repeat is clean of the build output described in NOTE-7.
- **Machine paths:** none in the 1,808 added lines. The only home-directory strings are the `<user>` placeholder forms quoted inside RV100's records. Living documents use `WT/…` and `<repo>/…`.
- **Credentials and whole-host data:** none found. I scanned the added lines for credential prefixes and key material, process listings, app paths, session IDs and host names. The hits are RV100's own description of its searches, the `Co-Authored-By` trailer, the owner's name in the manifest, and the guard's PID.
- **The entry validator:** `tools/validation/validate_instruction_entrypoints.py` gives "PASS: root instruction entrypoints are canonical".
- **The test that reads LOOP_INIT:** `P/tests/test_ci_e2e_plan.py` gives 41 passed and 21 subtests (NOTE-7 for its build output).
- **Hosted CI on the head:** every check that ran passed, including `harness`, Harness pre-merge, Desktop E2E (source mode) and the App instruction bundle. The cargo, source and accessibility jobs were skipped by coverage selection, as expected for a change with no product code.

## Repair confirmation

Repairs should be confirmed by this reviewer: MAJOR-1, and preferably MINOR-1 to MINOR-3 in the same pass. On a repaired head I would re-check:
- the LOOP_INIT diff and its sha256;
- G4 in diff mode;
- the entry validator;
- GEN-8 on the exact head;
- RR's prefix property, if a ruling is appended.

---

## Addendum A: confirmation of repairs at H3 (2026-10-05)

**Reviewer:** the same LR instance, Claude Opus 5.5 (`claude-opus-5-5`). The PR worktree and NUM were both at H3, and I made no Git writes. My network use was `gh` reads plus one read-only `git ls-remote` to see the two branch heads; the latter falls outside the brief's "gh reads" and is disclosed here. I ran the two validators and GEN-8. I did not rerun `test_ci_e2e_plan.py`, which NUM passed at `1c00d217fc` (41 passed), so it built nothing new. The original review above is bytes 1–27,265 of this file (sha256 `4de888bc…`, the first line of `SHA256SUMS`).

**Candidate.** H3 = `32b78024032354d22530265dc85d4fa2b2edc14c`. Its parents are H2 = `1762cc4489` and main `a2addb20d2` (#1093). Main is `a2addb20d2` (read with `gh` after the checks). The PR is draft, open and MERGEABLE.

| | |
|---|---|
| H3 tree | `770ed63d70…`, equal to NUM `ecfba8bd0f`'s tree |
| `P/loop/LOOP_INIT.md` at H3 | sha256 `959cd15adc41…` |
| History from main to H3 | `605adf726d`, `8a1bf06e43` (merges `01809013ae`), `1762cc4489`, `32b7802403` (merges `a2addb20d2`). None of NUM's own commits is in it (`8225f8f2a2` is not an ancestor), so a `--merge` carries no NUM history into main |

### Verdict at H3

**REPAIR, for one clause.** Every repair ROOT made is correct, except that one clause of MINOR-1's wording, which I proposed, is false (MINOR-A1). Once that clause is fixed, H3 is READY. On the fixed head I will confirm:
- the LOOP_INIT diff;
- G4 in diff mode;
- the entry validator;
- GEN-8.

| Severity | Count (this addendum) |
|---|---|
| BLOCKING | 0 |
| MAJOR | 0 |
| MINOR | 1 |
| NOTE | 4 |

### The delta

- **H→H3 outside App v4 and #1093** changes these 11 paths:
  - the manifest;
  - `LOOP_INIT.md`;
  - the work graph;
  - the T3 handoff, prompt and RR;
  - this run's `RULINGS.md`;
  - `BRIEF_LR.md`, this review and `SHA256SUMS`;
  - the new notice.
  
  Nothing else changes.
- **App v4.** The 35 App v4 paths equal main's #1091 delta, and H3's `projects/chirality-app-v4/` equals main's.
- **H2→H3** is exactly #1093's 8 paths: the workflow, its manifest, its run record and five notices. All 8 are byte-equal to main `a2addb20d2`, and none is one of #1092's paths.
- **H3 against main.** The delta is the six instruction paths plus 45 paths under `P/execution/`.

### The repairs

| Item | Status | Evidence |
|---|---|---|
| MAJOR-1 | Repaired | LOOP_INIT's standing constraint now reads "Piping's stage gate is the target stage and the exit criteria recorded in `execution/_Coordination/_COORDINATION.md` under "Current Target Stage". Assess against those criteria, not against a `docs/PRD.md` §24 milestone read by its label; only the owner's approved update advances the target." This is true against COORD's ruled record and §24. RULINGS Addendum A records the erratum to N13 and to the "checked" claim. |
| MINOR-1 | Repaired, but with a false clause (MINOR-A1) | "Its revision notes record each amendment that changed it" is true: v0.4–v0.14 cover SCA-001 to SCA-009, SCA-011 and D-77, and SCA-010 changed nothing. "`_LATEST.md` selects the latest accepted one" is true. "`execution/_ScopeChange/` holds every amendment" is not. |
| MINOR-2 | Repaired | The routed notice `P/execution/_Coordination/NOTICE_2026-10-05_PIPING_LOOP_INIT_BINDING.md` exists and is accurate. G4 checks that the routed path exists. The rationale is true at H3: #1093 is **merged** (main `a2addb20d2`, 16:26:53Z), with its `OWNER_DECISIONS.md`, and main's construct sentence now reads "supplies, or points to". The AUM tranche's owner direction exists (NOTE-A2). RULINGS records the erratum to ruling 5. |
| MINOR-3 | Repaired | RR's new ruling "T3's gate set and Git rules, consolidated after the handoff was made ephemeral" sits first in the work graph's list. The steer gains a product-PR bullet. |
| NOTE-1/2 | Resolved | NUM absorbed main (`44bbbf9bdd`, then `ecfba8bd0f`), and H3's tree equals NUM's. The work graph's and steer's "maintained source equals main's" is true again. The wording in RULINGS is NOTE-A1. |
| NOTE-3, NOTE-6 | Routed | Routed to ROOT-LOOPINIT-AUM-ALIGNMENT-20261005; see NOTE-A2. |
| NOTE-4, -5, -9 | Accepted as recorded | — |
| NOTE-7 | Repaired | "with little swap (about 1 GiB, dynamic)" matches T3's operating notes and `sysctl`. The worktrees row now names the standing pair. |
| NOTE-8 | Repaired | The work graph's owner decisions include "all T3 scratch lives in `WT/scratch`". The steer's basis claim holds, now that the graph lists the consolidated ruling. |

**The consolidated ruling makes no new rule.** I checked each item against the old handoff and prompt (main `6479bf110a`) and the rulings it cites:
- **Product-PR items 1–7** match "Gates before a main merge" and "PR packaging", with "or a registered identity" from the old prompt's step 5. `source_equality.py` exists in `IMPLEMENTATION/F2A_D1/`. The cited headings ("#1082 merged at F′…", "DEC-025 on F…", "RV100 passes #1088 at H…") exist.
- **Records-only items** match "Owner direction: proportionate CI…", A-1 and N-1.
- **Git rules** match the old handoff's Git rules, and "explicit paths, never `git add -A`" its Records rule.
- **Returns and IDs** match the old prompt's step 3 and the I61 "Dispatches prepared…" text.
- **Host items** match I61 decisions 1 and 13, and "Stray scratch gathered…".
- Citation details are NOTE-A3.

**Unchanged checks at H3:**
- **RR is append-only.** Main's copy (1,042,052 B) and H's (1,047,134 B) are both byte prefixes of H3's (1,050,478 B).
- **The init block** of the prompt is still byte-identical to `P/init/dev-loop-init-prompt.md` except for the steer.

### Checks at H3

Logs are in `WT/scratch/lr_loopinit_01/A_*`.
- **G4,** run in `WT/loop-init-pr` at H3:
  - CI mode: PASS, 154 manifests.
  - Diff mode, `--base a2addb20d2… --head 32b7802403… --tranche PIPING-LOOP-INIT-20261005`: PASS. 51 changed paths, 2 on the instruction surface, covered.
  - `--added-manifests-only`: PASS.
  - No BLOCK lines in any mode.
- **The entry validator:** "PASS: root instruction entrypoints are canonical".
- **GEN-8,** `python -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8` with `CHIRALITY_REQUIRE_LIVE_TESTS=1`, in `WT/loop-init-pr` at H3: 1 passed. This agrees with ROOT's `gen8_H3.txt`.
- **Machine paths and credentials:** none in the 430 lines that H→H2 adds outside App v4 and #1093, and no home-directory path in H3's whole delta against main.
- **Hosted CI on H3:** every check that had finished passed or was skipped by coverage. `harness` was still pending when I looked.

### Findings

**MINOR-A1: "`execution/_ScopeChange/` holds every amendment" is false. The wording was mine, and I did not check it.**
- **Evidence.**
  - `P/execution/_ScopeChange/` has folders for SCA-001 to SCA-005 and SCA-007 to SCA-011, but none for SCA-006.
  - SCA-006 (the D-43 PKG-00 architecture-basis consolidation) is recorded in the decomposition's v0.9 revision note ("Revision v0.9 records `SCA-006`…") and in `_DECISIONS/D-43_pkg00_architecture_basis_consolidation.md`.
  - Within `_ScopeChange/`, only SCA-007, SCA-009 and SCA-011 files mention it.
- **Fix.** Drop the clause: "Its revision notes record each amendment that changed it, and `execution/_ScopeChange/_LATEST.md` selects the latest accepted one." Note it in RULINGS.

**NOTE-A1: RULINGS Addendum A says "the PR is re-cut from that main".** It was not re-cut: the branch merged main twice (`8a1bf06e43` and `32b7802403`) and took NUM's paths. The facts that matter hold: H3's tree equals NUM's, and no NUM-only commit is in its history. Correct the wording when RULINGS is next touched. ROOT's message named NUM's absorb as `cb9ff41d98`. The commit on NUM's branch is `44bbbf9bdd`, with the same tree; `cb9ff41d98` is an unreferenced merge object. No record names either, so nothing needs changing.

**NOTE-A2: the routing to ROOT-LOOPINIT-AUM-ALIGNMENT-20261005 points to records that are not yet in Git.**
- The tranche's `OWNER_DECISIONS.md` exists only as untracked files in NUM. It does record the owner's direction: App v4's sentence, "any other minimal consistency edits along the lines of what you did here", and the AUM updates.
- Its current drafts (`AUM_EDITS.md`, `BRIEF_AM.md`) do not yet include Section B (B1–B3) or the contributor guide's row 8, which RULINGS Addendum A routes there.
- **Fix.** Carry them into that tranche, or the routing statement becomes untrue.

**NOTE-A3: the consolidated ruling's citations, and the steer's summary of it.**
- **Item 2's citation.** The ruling cites project `AGENTS.md` for "with the same reviewer confirming each repair". Project `AGENTS.md` says "backcheck the correction". The same-reviewer part comes from the old handoff and prompt, which the ruling's closing line names as its source.
- **The steer's summary.** The new steer bullet introduces the set with "lists it:", but omits "or a registered identity" and the T9/both-entry gates. Saying "including" instead, or adding them, would make the summary exact.
- Neither is a new or changed rule.

**NOTE-A4: merge conditions.**
- **Main.** Main is `a2addb20d2`, which H3 contains, so no carry-over ruling is needed unless main moves again.
- **Hosted CI.** The hosted `harness` check on H3 was pending when I looked.
- **Host cleanup.** The two gitignored `target/` folders from my original run remain in `WT/loop-init-pr`, and this addendum added no build output. They go when that worktree is removed after the merge.

---

## Addendum B: confirmation at H4 (2026-10-05)

**Reviewer:** the same LR instance. I made no Git writes, used only `gh` for network reads, and did not run `test_ci_e2e_plan.py`. Addendum A and everything before it are bytes 1–36,845 of this file (sha256 `787b8443…`, the second line of `SHA256SUMS`).

**Candidate.** H4 = `1ca59756f7c6fd19c402424ae031193dc83fe90f`, whose single parent is H3 `32b7802403`. Its tree is `96ed3e1296…`, equal to NUM `b8d8971908`'s. Main is still `a2addb20d2`, which H4 contains. The PR is draft, open and MERGEABLE.

**The H3→H4 delta** is exactly these four paths:
- **`P/loop/LOOP_INIT.md`:** one sentence. The new text is MINOR-A1's wording exactly: "Its revision notes record each amendment that changed it, and `execution/_ScopeChange/_LATEST.md` selects the latest accepted one." It is true. SCA-006, which has no `_ScopeChange/` folder, is in the v0.9 note, and `_LATEST.md` selects SCA-011. The file's sha256 at H4 is `65cb34a71d95…`.
- **This review and `SHA256SUMS`:** byte-identical to what I wrote. The review's sha256 at H4 is `787b8443…`.
- **`RULINGS.md` Addendum B:** accurate. It records MINOR-A1 as adopted with an erratum to my Addendum A wording, clarifies NOTE-A1, and accepts NOTE-A3 and NOTE-A4.

**Checks at H4.** Logs are in `WT/scratch/lr_loopinit_01/B_*`.
- **G4 diff mode** against `a2addb20d2`, with `--tranche PIPING-LOOP-INIT-20261005` and with `--added-manifests-only`: PASS, 51 paths, 2 on the instruction surface, no BLOCK.
- **The entry validator:** PASS.
- **GEN-8:** 1 passed in `WT/loop-init-pr` at H4.
- **Machine paths:** none in H3→H4.
- **Hosted CI on H4:** every finished check passed or was skipped by coverage selection; `harness` was pending.

**Verdict at H4: READY.** There are no open BLOCKING, MAJOR or MINOR findings. Merge conditions under Root `AGENTS.md`:
1. **`harness`** must pass on H4, or on whatever final head is merged.
2. **Main must not have moved** at merge. If it has, refresh or carry over the gates by ruling.

**NOTE-B1: the AUM tranche must actually carry Section B.** RULINGS Addendum B says Section B and the contributor guide's row 8 "are carried into tranche ROOT-LOOPINIT-AUM-ALIGNMENT-20261005". That tranche's drafts in NUM, still untracked, do not include them yet, so the statement holds only once they are added. Separately, `WT/loop-init-pr` and its gitignored `target/` folders are removed after the merge.
