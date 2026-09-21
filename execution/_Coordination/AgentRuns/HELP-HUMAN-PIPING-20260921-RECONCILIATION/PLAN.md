# Piping whole-corpus reconciliation — Agent 0 plan

## Context

`projects/chirality-piping/` scope text stopped in July while code kept moving (979 commits; `apps/desktop` +81K lines; v0.2 schemas; physics/runner/solver growth). The owner deliberately deferred deliverable upkeep ("reconcile later") and has now suspended piping and asked for a whole-corpus reconciliation of what transpired in code against the deliverables as they stand, to determine what must change in the deliverables and whether anything must change in code.

Method: root `workflows/reconciliation` (R0–R6) + kernel `docs/DELIVERABLE_CONCORDANCE_METHOD.md`, run as written, extended (not weakened) by the refined concept. Code/tests are evidence, not authority. R0–R3 write only inside the run folder. The owner rules direction-of-change at R4.

**Owner directions already given (to be recorded verbatim in the run record):**
- Piping suspended; pause holds until this run is done. Baseline = current `origin/main` → freeze **`00115c719`** (piping tree identical to the #834 merge).
- Scope: whole corpus (102 deliverables, 18 packages; DEL-01-01 ISSUED read-only; DEL-07-09 included).
- No reference to the July plan; write the best plan for these circumstances.
- A merged PR is an extension of owner intent; PR descriptions are first-class references for what/why (R0 candidate convention).
- Existing `## Remaining` entries are stale, never authority — audited as declared-state claims (R0 candidate convention).
- No per-deliverable bootstrap seeding (variance from kernel §6 — packet ruling item).
- Scope-change / DAG rebuild are typical follow-ons but outside this run.
- Models: `opus-5`, high reasoning, all roles. Cap: 16 agents including Agent 0 (owner may revise mid-run). Nesting confirmed by owner. Gate transcript: fresh run at R0.
- Nothing is dispatched before the activation merge.

## Topology

Agent 0 (HELP_HUMAN, this session) → one WORKING_ITEMS manager per package → TASK workers (one owning worker per deliverable; TASK never delegates). Oversized deliverables: manager dispatches read-only evidence-gathering TASKs feeding the single owner. Managers stay thin: one line per deliverable back, never read CSVs; scripts validate and merge. Fresh evidence-only verifier per package. Only Agent 0 commits. Every dispatch records mechanism, parentage, model, brief hash, return hash (D-GOV-35). Cap is checked before each sub-batch; drain rather than launch when usage is low.

## Concurrency (brief parameter, owner-revisable mid-run)

**Hard cap: 16 live agents at any instant, counting Agent 0 and every manager, worker, verifier, reviewer, and evidence-gatherer at any nesting depth.** Recorded in `RUN_BASIS.md` and every manager brief; re-read from `RUN_STATE.jsonl` before each sub-batch so an owner revision takes effect at the next sub-batch boundary.

| Phase | Agent 0 | Managers | Workers / others | Total |
|---|---|---|---|---|
| Activation review | 1 | 0 | 1 reviewer | 2 |
| R0 calibration | 1 | 0 | 8 TASKs, then 1 reviewer | ≤9 |
| R1 inventories | 1 | 0 | ≤10 TASKs | ≤11 |
| R2 waves | 1 | ≤3 | ≤12 across all managers (4 per manager budget) | ≤16 |
| R2 verifiers | 1 | — | run in slots freed by a finished package; never in addition to a full wave | ≤16 |
| R3 / R4 | 1 | 0 | ≤2 TASKs + 1 reviewer | ≤4 |

Each manager is given a fixed worker budget in its brief and may not exceed it; evidence-gatherers for an oversized deliverable come out of that manager's budget. Agent 0 keeps the slot ledger (launch/return events in `RUN_STATE.jsonl`) and launches a manager only when its full budget fits under the cap. If session usage runs low: stop launching, let live agents drain, commit, update `RESUME.md`.

## Step 1 — Open the run and draft activation (no dispatch)

Branch: `claude/chirality-piping-reconciliation-7f7e70` (already at `00115c719`).

Create:
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/` — `OWNER_DIRECTIONS.md` (verbatim + SHA-256), `WORK_GRAPH.json`, `HANDOFF_STATE.md` (per `loop/LOOP_INIT.md`).
- `execution/_Coordination/_DECISIONS/D-73_reconciliation_activation.md` — PROPOSAL packet, shaped on `D-41_concordance_activation.md`. Ruling items:
  1. Scope whole-corpus; freeze SHA `00115c719`; suspension declaration.
  2. Pinned method: `workflows/reconciliation/**` + kernel at the pin SHA + the new piping profile (below).
  3. Variance: no `## Remaining` seeding; program state lives in the register row + run folder (kernel §5).
  4. Coverage of the ruling: **R0–R4 only (recommended), R5 separately authorized after R4** vs R0–R6.
  5. Run ID and run folder: `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`.
- `projects/chirality-piping/docs/RECONCILIATION_PROFILE.md` (new piping adoption profile; replaces the July plan as the adoption record without citing it): corpus parameters, formats, divergence layers and how each enters the ledger, fences (F-PIP-1..4, DEC-043, DEC-081), evidence roots and read-only rule for each, routing (scope-change / review / change / implementation briefs).
- Register row D-73 `AWAITING_RULING`; coordination `NOTICE_2026-09-21_RECONCILIATION_ACTIVATION.md`.

Then: owner rules in-session → `D-73_RULING_2026-09-21.md` (verbatim words + hash, current piping ruling format), register flip, `DEC-110` in `SOFTWARE_DECOMP.md` §12 → fresh-context independent review of the full diff → registered checks (`harness-self-check`, `harness-pytest`), receipts validator → PR → CI → merge to `main`. Note: piping requires fresh-context review of this PR, and "nothing dispatched before the merge" would forbid it. Plan: one read-only reviewer agent over the frozen activation diff is the single pre-merge launch, recorded as such; it does no reconciliation work. If the owner objects, the owner reviews the PR instead.

## Step 2 — R0: freeze, tooling, gate transcript, calibration

1. Detached scratch worktree at `00115c719` **outside the repo** (under the session scratchpad); never `git worktree prune`; never touch other sessions' worktrees.
2. `RUN_BASIS.md`: source state, authorities (PRD v0.4/SCA-010, CONTRACT, SPEC, TYPES, SOFTWARE_DECOMP 0.12, DAG-010, register through D-73, DEC-109), census, concurrent-work check (none; App reconciliation is a separate project), pins, fences.
3. Run-folder tooling (Agent 0 writes; reviewed in the R0 PR):
   - `tools/extract_claims.py` — deterministic keys: `DEL-XX-YY:CLM-NNN`, `…/REQ-…`, `…/AC-…`; PKG-00 `DEL-00-0N:REQ|AB-…`; DEL-07-09 `DEL-07-09:VOC-<row>`/`CAP-<row>`; line range + hash per item. Output is the coverage denominator.
   - `tools/validate_ledger.py` — schema, key coverage exactly-once, controlled vocabularies, citation path existence at freeze, sentinel present, RFC-4180.
   - `tools/merge_ledgers.py`, `tools/stale_check.py` (diff freeze..main per sub-batch → `STALE_INPUT`).
   - `RUN_STATE.jsonl` (append-only), `RESUME.md`, `BRIEFS/` (idempotent, hashed).
4. Gate transcript, once, in the scratch worktree with all caches/outputs external (`CARGO_TARGET_DIR`, `PYTHONDONTWRITEBYTECODE`, `-p no:cacheprovider`, sweep `--output-dir` external): the six checks in `software-workflow.json`. Save full per-test output to `GATE_TRANSCRIPT/`. Native Tauri witnesses cited from existing records only. Coordinate timing with the App session. Post-run ignored-aware porcelain must be clean.
5. Candidate `CONVENTIONS.md` + `METHOD_ADDENDUM.md` for owner ruling (nothing carries silently):
   - owner's two conventions above;
   - governing vs CONTEXT: register rulings / DEC / accepted SCA = governing; AgentRuns direction records, approved plans, UX spec, merged-PR descriptions = CONTEXT (explains, clusters, never changes Disposition) — plans adopted by reference in a ruling flagged for owner call;
   - sealed two-pass protocol; `CauseTag` vocabulary; `AuthorityTier` (LOCAL_DESIGN / PROJECT_BASELINE / INVARIANT); `BaselineClass` + change path; verification sub-class (unit / browser-e2e / native);
   - ledger = July's 20 columns (kernel distinctions kept) + `AuthorityTier`, `BaselineClass`, `CauseTag`, `ContextRefs`, `VerificationClass`, `SourceStateSHA`;
   - prior conventions as candidates: piping July Conventions 1–8 + 13 addenda + piping dispositions; App MR-1..MR-11 only if the owner wants them considered;
   - July ledger hidden from workers; R3 cross-check only.
6. Calibration: 8 TASKs, one each — DEL-04-04, DEL-07-02, DEL-00-05, DEL-07-09, DEL-01-01, DEL-12-03, DEL-11-01, DEL-17-05 — full two-pass protocol against a pilot reverse slice; one fresh reviewer over all eight. Validate schema, citation quality, disposition consistency, false positives.
7. R0 PR (independent review) → **owner rules conventions, addendum, named repairs, scale-out.**

## Step 3 — R1: read-only inventories

Source-state-bound: `DELIVERABLE_INVENTORY.csv`, `VERIFICATION_INDEX.csv` (from gate transcript), `VALIDATION_AND_PROVENANCE_INDEX.csv`, `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md` (incl. governing/CONTEXT classing of D-64..D-72, DEC-093..109, SCA-009/010, merged-PR index since July), and the **reverse inventory** `IMPLEMENTATION_SURFACES.csv` built from code by disjoint area with **no candidate-owner column**:
`apps/desktop/src` by feature folder · `src-tauri` commands · `e2e` · 21 `core/*` areas · `schemas` · `fixtures` · `tests` · `tools` · `validation` (non-evidence) · docs outputs (user/developer guide, theory, validation manual, security) · root `.github` piping workflows. Capability-grain rows for UI (operations, commands, panels), not file-grain. ~10 read-only TASKs under Agent 0 directly.

## Step 4 — R2: package waves

Order by tractability/dependency: W1 PKG-00–03 · W2 PKG-04–06 · W3 PKG-08–12 · W4 PKG-13–17 · W5 PKG-07 (largest drift, last, benefits from earlier lessons). Within the 16-agent cap (see Concurrency): ≤3 managers live, each with a 4-agent budget, sub-batches of ≤4 deliverables.

Per worker, two sealed passes, whole-file outputs with sentinel:
1. **Forward**: every extractor key → evidence → disposition, `CauseTag`, `AuthorityTier`. Path hints only. Sealed (hash recorded) before pass 2.
2. **Reverse**: worker is shown capability rows for its areas; answers `CLAIMED_BY <key>` / `PARTIAL` / `NOT_MINE`.

After each sub-batch: `validate_ledger.py`, `stale_check.py`, Agent 0 commit, `RUN_STATE.jsonl` append. Per package: fresh evidence-only verifier over all flagged + non-aligned rows + representative aligned sample; defective ledgers rerun by a fresh worker; contested rows stay visible. Package summaries derived by script.

## Step 5 — R3: synthesis (Agent 0 + 1–2 TASKs)

Final unmapped set (capabilities with no `CLAIMED_BY`), duplicate ownership, shared surfaces, stale evidence, lifecycle/Remaining defects, July-ledger cross-check. Cluster non-aligned claims by `CauseTag` × `AuthorityTier` into ~15–20 classes + exceptions, each with its full claim population. Separate outputs: `SCOPE_CHANGE_HANDOFF/` (retire/merge/create deliverables), `CODE_FIX_BRIEF_CANDIDATES/` (not executed), engineering-authority items. `COVERAGE_AND_QA.md`, spot-check. R3/R4 PR with independent review.

## Step 6 — R4: owner gate

Decision packets per cause class (options, evidence, affected claim IDs, risk, routing, exact on-ruling mechanism). Owner rules direction-of-change. **Run stops here under the recommended ruling scope**; R5 tranche plan is prepared as a proposal for separate authorization.

## Step 7 — R5 / R6 (if and when authorized)

R5: only ruled edits, partitioned by owning deliverable; SoW schema kept valid; ISSUED DEL-01-01 only via its change path; no code, instruction, DAG, or lifecycle edits. R6: new immutable backcheck derivative — changed-claim re-extraction with multiset equality to the repair manifest, Remaining census with all 102 rows, containment audit, handoff; closure note in the D-73 register row; loop receipt (numbered to avoid the unmerged Receipt-161).

## Standing hygiene

Frozen-tree containment incl. ignored state · run artifacts must pass the claims lint and `harness-pytest` · F-PIP-2/DEC-081 fence in every brief · DEC-043 equation-source exclusion in every brief · no protected/private data quoted in ledgers · receipts validated before/after append · independent fresh-context review on activation, R0, R3/R4, R5, R6 PRs · merges under the standing Git authorization only with CI green and no blocking findings.

## Verification

- Activation: packet/ruling/register/DEC present on `main` before first dispatch (checked by SHA).
- Every sub-batch: validator pass, exactly-once key coverage, stale check, commit.
- Corpus: summaries reproduce from rows; extractor denominator = ledger rows; every reverse-inventory row has a terminal answer; R3 unmapped set reproducible by script.
- Containment: scratch worktree porcelain (ignored-aware) clean after gate run and at each phase close; repo diff outside run folder empty through R3.
- Resumability drill: a cold read of `RESUME.md` + `RUN_STATE.jsonl` identifies the next action.
