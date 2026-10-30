---
doc_id: CB-2026-07-20-T7-DEL-09-04-VALMANUAL-STALE-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-20
package_id: PKG-09
deliverable_id: DEL-09-04
decision_basis: DEC-065, DEC-080, DEC-081, DEC-046 (preserved gate), DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T7 DEL-09-04 Reproduction-Manual Stale-Text Refresh (Post-R14 Waves 1–2)

**Status:** `EFFECTIVE — EXECUTION RELEASED BY W4 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)`

**Verification record (2026-07-20):** the fresh-context adversarial brief
verifier returned `COMMIT-SAFE` (`instances/W4/T7/VERIFY_BRIEF.md`;
C1–C10 confirmed; independent ten-class screen pass; fast-reject sweep no
hit). Its one Low, non-blocking defect is recorded without operative-text
change: the §4/rationale shorthand "the case-1 cell is contradicted"
overstates — the cell's literal predicates still hold post-T2 (the
verifier confirmed `validate_result` is structural-only, so
"request/result validation diagnostics are empty" remains literally
true); the operative staleness ground is the broken committed-witness
byte-match plus the un-noted new envelope warning, exactly as §2.1/§3.1
state, and §3.1's exact wording with the mandatory live-run gate controls
what reaches the page.

**Prepared by:** WORKING_ITEMS (W4, PKG-09 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W4, tranche T7

**Selected work item:** the W1-routed DEL-09-04 docs-lane follow-on
(`instances/W1/RETURN.md` §5 item 2): refresh
`docs/validation_manual/headless_runner_reproduction.md` where — and only
where — the live tree shows its assertions went stale after the R14 waves
1–2 landings, per the R12→R13 case-3 dated-note precedent
(`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`
is the structural model). Docs + bounded deliverable state only. This
tranche closes NO Remaining row (DEL-09-04 `## Remaining` stays
byte-identical), performs no reproduction run acceptance, no lifecycle
change, no threshold/tolerance content, and no code, schema, test,
fixture, or witness change.

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies
and proposes only. The adoption effect is `HELD` until independent
refutation returns `COMMIT-SAFE` and the W4 manager progresses the chain
under the R14 campaign-plan execution rules. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: make the DEL-09-04 headless-runner reproduction page truthful
again on post-R14-wave sources by (a) adding a dated historical note for
the Part 1 case-1 solve expectation that W1 T2 made stale, and (b) adding
a dated currency note for the Part 2 pinned whole-suite figures now that
the mechanics suite carries 24 fixtures — every corrected assertion
anchored to committed witness bytes, committed derivative evidence, or a
live offline run of the documented command at the execution head.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(branch `claude/piping-r14-pkg09-evidence`, post-T6 head `db9197a5d` on
post-wave-2 main base `e315fb840`):

- root and project `AGENTS.md`; active workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; the R14 campaign plan, the
  W4 dispatch transcript (`instances/W4/W4_DISPATCH_TRANSCRIPT.md`), and
  the W1/W2/W3 returns;
- the routing source: `instances/W1/RETURN.md` §5 item 2 and its chain
  evidence — T2 executor return (`instances/W1/T2/EXECUTE_RETURN.md`,
  "Recorded Follow-On" and "Pinned-Case And del1005 Results": the frozen
  solve command at the T2 head kept exit 0 / `COMPLETED` with the only
  content changes being one added non-blocking
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` diagnostic for `support:CE-120`
  (empty `restraints`), updated sign-convention disclosure text on its
  two `constant_effort_user_input_review` rows, and the consequent
  result-envelope checksum; before/after stdout SHA-256
  `738d3c074dd9…` → `b3cd85af8565…`; result-row count 830 unchanged) and
  T2 implementation verifier (`instances/W1/T2/VERIFY_IMPL.md`, defect D1:
  the committed fallback fixture
  `fixtures/product_preview/invented_mechanics_result.json` still carries
  the superseded "no global constant-effort load…" review-row text);
- the page itself: `docs/validation_manual/headless_runner_reproduction.md`
  at the live head — Part 1 Fixture Set case-1 row ("Expected evidence:
  `runner_result.job.state` is `COMPLETED`; request/result validation
  diagnostics are empty; `runner_result.result_refs` is non-empty"), the
  existing dated 2026-07-19 case-3 note (the precedent form), the Part 1
  procedure paragraph, the Part 2 "Per-Case Reporting and Fail-Closed
  Semantics" section with the R12-head-pinned figures ("mechanics 11/21
  `executed_and_matched` + 10 blocked; stress 12/15 + 3 blocked;
  nonlinear 5/5"), the Rerun Consequence section, and the Remaining E2
  Work paragraph;
- DEL-09-04 deliverable folder (under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
  `_STATUS.md` (`IN_PROGRESS`; two Remaining bullets, BOTH preserved
  byte-identical by this tranche — the E2-residuals bullet's open
  sub-items are owner-gated per W3 row 7, and the tolerance bullet is
  owner-gated per W3 row 8), `MEMORY.md` (R13 refresh entry newest),
  and the R13 run record
  `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_VALMANUAL_REFRESH_R13.md`;
- solver/runner ground truth (live source, read-only):
  `core/product_physics/src/lib.rs` (T2's
  `classify_constant_effort_consumption` and envelope-level
  `append_constant_effort_consumption_diagnostics` emitting the
  non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning for
  non-consuming shapes, including the empty-`restraints` shape);
  `core/product_physics/src/validation.rs` (the
  `CONSTANT_EFFORT_USER_DATA_REVIEWED` info-text, untouched by T2, whose
  clause "no global constant-effort solve behavior … is claimed" remains
  literally true of the validation acceptance itself);
  `core/runner/headless/src/bin/openpipestress-runner.rs` (DEC-065 exit
  policy unchanged);
- frozen E1 surfaces (read-only, byte-frozen): the three
  `validation/witness/inputs/tp_runner_015_final_cli_*_input.json`
  fixtures (the solve input's `supports[6]` is `support:CE-120`, family
  `constant_effort_support`, `restraints: []`, `hanger.constant_load`
  375 N — the non-consuming shape), their generator, and the three
  committed `validation/witness/generated/tp_runner_015_final_cli_*.json`
  witnesses (the committed solve witness remains truthful for its pinned
  pre-T2 commits);
- frozen del1005 surfaces (read-only, byte-frozen): the five input/five
  generated `del1005_payload_binding_*` pairs and their generator —
  verified byte-identical at the W1 and W2 heads by both waves' chains;
  the nonlinear whole-suite witness (5/5) therefore remains truthful at
  head, and the stress suite is unchanged (readiness assertion 15);
- committed derivative evidence for the currency note: the T6 bundle
  `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
  (whole-suite mechanics at head base `e315fb840`: 24 cases, 11
  `executed_and_matched` + 13 `blocked`, exit 1, recorded as regression
  evidence; committed at `db9197a5d`) and the mechanics suite readiness
  assertion `assert_eq!(fixtures.len(), 24)`;
- wave provenance for the dated note: W1 T2 implementation commit
  `faee4faed` (constant-effort assembled-solve consumption), on main at
  post-wave-1 merge `581a15b1c` (PR #292, Receipt-61); waves verified
  T3 left the pinned solve output unchanged (W1-C6) and W2 T4 touched
  no witness-facing solve surface for this input (no `equivalent_static`
  block in any witness input);
- negative verification (nothing else stale, checked at preparation):
  the manual nowhere mentions wind schema required-set text; the Bound
  Fixture Set rows and Reproduction Procedure (bound path) remain
  anchored to byte-identical committed witnesses; the Review Checks
  commands remain valid; the Rerun Consequence and Remaining E2 Work
  paragraphs remain truthful (their open items match W3 rows 7–8); the
  case-2 and case-3 rows remain truthful (case 3 already carries its
  dated note);
- governance: `DEC-065`, `DEC-080`, `DEC-081`, `DEC-046` (untouched
  gate), D-52/`DEC-085`, D-54/`DEC-087`, the R13 brief as structural
  model, and the R13 rerun-trigger row "the DEL-09-04 … page itself
  through another lane" — this tranche IS such a lane, so its run
  record notes the R13 trigger consequence truthfully (documentation
  currency, no reproduction rerun performed);
- `software-workflow.json` and the root tools
  `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts (verified staleness inventory)

Exactly two page surfaces are stale or currency-lapsed at the live head;
three routed observations require no page edit:

1. **STALE — Part 1 case-1 row and procedure text.** The case-1
   "Expected evidence" cell asserts "request/result validation
   diagnostics are empty". On sources at or after `faee4faed` (merged at
   `581a15b1c`), the same frozen command still exits 0 with
   `job.state = COMPLETED`, empty runner/request/result validation
   diagnostics, and non-empty `result_refs`, but the solve result
   envelope now additionally carries one non-blocking warning diagnostic
   `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` for `support:CE-120` (the
   frozen input's empty-`restraints` constant-effort support) and
   updated sign-convention disclosure text on its two
   `constant_effort_user_input_review` rows — so regenerated output no
   longer matches the committed witness byte-for-byte, and a reader
   comparing against `tp_runner_015_final_cli_solve.json` needs the
   dated historical framing that case 3 already has.
2. **CURRENCY — Part 2 pinned whole-suite figures.** The R12-head-pinned
   figures (mechanics 11/21 + 10 blocked) remain truthful for
   `60841413a`, but the mechanics suite now carries 24 fixtures (R14 W1
   T2/T3 + W2 T4 additions) and the committed T6 bundle records the head
   whole-suite result (24 cases: 11 matched + 13 blocked, exit 1). A
   dated currency note prevents the pinned figures from being read as
   current.
3. **NO PAGE EDIT — fallback fixture text.** The committed
   `fixtures/product_preview/invented_mechanics_result.json` still
   carries the superseded "no global constant-effort load…" review-row
   text (W1 T2 VERIFY_IMPL D1). The manual page nowhere references this
   fixture; it is a pinned earlier-generation committed output outside
   the docs lane. Disposition: recorded in the run record/MEMORY as a
   remaining routed observation for a future code/fixture-lane
   selection; no manual assertion is stale on its account.
4. **NO PAGE EDIT — `validation.rs` info-text.** The
   `CONSTANT_EFFORT_USER_DATA_REVIEWED` clause remains literally true of
   the validation acceptance itself (T2 executor's own finding); it is a
   code surface outside the docs lane and is not quoted by the manual.
   Disposition: recorded observation only.
5. **NO PAGE EDIT — wind schema required-set text.** Not mentioned
   anywhere on the page (verified); the W2 wind schema change has no
   manual surface here.

DEL-09-04 remains `IN_PROGRESS`; both Remaining bullets are owner-gated
(W3 rows 7–8) and stay byte-identical. Receipt cursor is `Receipt-62`;
no receipt is appended by this wave (HELP_HUMAN fan-in act).

## 3. Objective and Acceptance Predicates

Refresh the page and DEL-09-04 state so that all of the following hold on
the execution head:

1. **Case-1 dated note, precedent form.** Part 1 gains a dated 2026-07-20
   note (mirroring the existing case-3 note's form) stating: case 1's
   committed-witness byte expectation is historical for sources at or
   after R14-W1 T2 (implementation commit `faee4faed`, on main at
   `581a15b1c`, PR #292, Receipt-61); on such sources the same frozen
   solve command still exits 0 with `job.state = COMPLETED`, empty
   request/result validation diagnostics, and non-empty `result_refs`,
   and the solve result envelope additionally carries one non-blocking
   warning diagnostic `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` for
   `support:CE-120` (whose frozen `restraints` list is empty — a
   deliberately non-consuming shape) plus updated sign-convention
   disclosure text on its two `constant_effort_user_input_review` rows;
   therefore regenerated output no longer byte-matches the committed
   `tp_runner_015_final_cli_solve.json` witness, which — like the prior
   reproduction bundles — remains truthful for its pinned pre-T2
   commits and is not edited. The case-1 "Expected evidence" cell is
   corrected in place to state the post-T2 expectation (exit 0;
   `COMPLETED`; request/result validation diagnostics empty; non-empty
   `result_refs`; one non-blocking constant-effort non-consumption
   warning in the result envelope on post-T2 sources, per the dated
   note). The non-consumption warning is described as review evidence
   about a non-consuming user-data shape — not a defect, not a solve
   error, and not new acceptance criteria.
2. **Head-anchored verification of the case-1 claims.** Before closeout
   the executor verifies the note's head claims by a live offline run of
   the documented frozen case-1 command at the execution head (exit
   code, `job.state`, the single added warning diagnostic code and its
   `support:CE-120` subject, non-empty `result_refs`), plus source
   anchors (the frozen input's `restraints: []`; the envelope-level
   warning emission in `core/product_physics/src/lib.rs`). Outputs are
   ephemeral; no durable write outside §5. If the local build
   prerequisite is missing, the tranche is BLOCKED truthfully (the live
   run is the mandatory anchor for head-behavior claims), not
   downgraded.
3. **Part 2 dated currency note.** The "Per-Case Reporting and
   Fail-Closed Semantics" section gains a dated 2026-07-20 note: the
   pinned R12-head figures remain truthful for `60841413a`; at the R14
   head (`e315fb840` base) the mechanics suite carries 24 fixtures
   (readiness assertion `fixtures.len() == 24`; the three R14 additions
   named) and the committed derivative evidence bundle
   `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
   records the head whole-suite `run-benchmark` result: 24 cases, 11
   `executed_and_matched` + 13 `blocked`, exit 1 — regression evidence
   under the recorded claim posture, not a defect record, no release
   judgment, thresholds/tolerances/CI-gate policy still `TBD`
   owner-gated. The stress (12/15 + 3) and nonlinear (5/5) pinned
   figures are noted as unchanged by the R14 waves (suites untouched;
   del1005 witnesses byte-identical at head).
4. **No other page content changes in meaning.** The frozen E1 fixture
   table rows for cases 2–3, the existing case-3 dated note, the bound
   fixture set, both reproduction procedures, the Rerun Consequence
   section, the Review Checks section, and the Remaining E2 Work
   paragraph are either byte-identical or receive only the two dated
   notes and the corrected case-1 cell above. Frontmatter identity
   (`OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION`,
   `draft_evidence`) unchanged.
5. **Frozen surfaces byte-identical.** The seven tp_runner_015 surfaces
   (three inputs, generator, three generated witnesses) and the eleven
   del1005 surfaces (five inputs, generator, five generated witnesses)
   are untouched (`git status --porcelain` empty over them), as are all
   reproduction bundles, sweeps, and the T6 bundle.
6. **Claims calibration.** All new text stays inside the DEC-081
   regression/verification-evidence posture; no new tolerance,
   threshold, acceptance criterion, or normative content anywhere; the
   page continues to state that validation acceptance and professional
   judgment remain with the responsible engineer.
7. **Bounded state update.** DEL-09-04 `_STATUS.md`: `## Remaining`
   byte-identical (both bullets); exactly one new History entry and
   updated `Last Updated`; `Current State: IN_PROGRESS` unchanged.
   `MEMORY.md`: exactly one new entry (newest-first per the file's
   convention), recording the two dated notes, the three no-edit
   dispositions (§2 items 3–5), and the R13 rerun-consequence
   restatement (any future clean-checkout reproduction runs from a
   post-refresh source commit under a fresh run ID and new immutable
   bundle). Exactly one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md`.
8. **Non-acts.** No reproduction run bundle is created; no witness,
   fixture, code, schema, test, tool, or `validation/**` write; no
   lifecycle, stage, promotion, acceptance, or receipt act; no
   DEL-10-05 write; and the §6 validation plan passes.

A successful run closes no Remaining bullet and promotes nothing.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several documentation shapes were defensible, the selection below
was made under D-54/`DEC-087`; the four-lens analysis and materially
rejected alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape: **two dated notes plus one corrected cell, R13 precedent
form** — correct the stale case-1 cell in place, carry the historical
framing in a dated note exactly parallel to the existing case-3 note, add
one dated currency note to Part 2 anchored to the committed T6 bundle,
and leave every other surface byte-stable. Rejected: full-page
restructure (nothing structural went stale); silent cell edit without a
dated note (breaks the pinned-witness truthfulness convention); witness
regeneration (forbidden frozen surfaces, DEL-10-05 lane); fixing the
fallback fixture or `validation.rs` text here (outside the docs lane);
NO-OP (refuted — the case-1 cell is contradicted on post-T2 sources).

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch `claude/piping-r14-pkg09-evidence` (post-T6
  head) in the integration checkout; record the base commit before any
  durable write; verify the tree is clean apart from this run's lawful
  pre-existing state (the W4 instance directory and this brief).
- Re-verify the §1/§2 facts (the case-1 cell text, the frozen input's
  `restraints: []`, the warning emission site, the T6 bundle's recorded
  tallies, the suite count assertion). Stop on material drift; do not
  silently reinterpret scope.

### 4.2 Verify head behavior, then edit

- Run the §3.2 live offline verification FIRST; if any observed value
  contradicts the §3.1 note content, STOP and return the discrepancy —
  do not write a note the head does not witness.
- Apply the §3.1/§3.3 edits to
  `docs/validation_manual/headless_runner_reproduction.md`.

### 4.3 Update deliverable state and close out

On success only: apply §3.7, then run §6 in order. On failure or block:
leave deliverable state unchanged (or record the truthful partial
state), write truthful evidence and `EXECUTE_RETURN.md` under
`instances/W4/T7/`, and return to the W4 manager. The executor does not
commit; the W4 manager commits after independent implementation
verification.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited
to (paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. `docs/validation_manual/headless_runner_reproduction.md`;
3. DEL-09-04 only (under
   `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
   `_STATUS.md` (History + `Last Updated` only; Remaining byte-identical),
   `MEMORY.md` (one new entry), one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md`;
4. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/**`;
5. no receipt append and no evidence-sweep artifact in this tranche
   (wave-level closeout acts per the W4 dispatch).

Ephemeral writes: task-local Cargo target dirs and scratch captures
outside durable project paths.

No other project file is writable. In particular: no other `docs/**`
file (including `docs/validation_manual/index.md` and the case pages);
no `validation/**` path (witnesses, bundles, sweeps, the T6 bundle,
benchmark crates); no `core/**`, `schemas/**`, `tests/**`, `tools/**`,
or `fixtures/**` (the §2.3 fallback-fixture and §2.4 info-text
observations are recorded, not fixed); no other deliverable or package
folder, register, DAG, decomposition, decision packet, PRD/PLAN,
workplan, or `loop/LOOP_RECEIPTS.md`; no root governance,
`_DomainEngines/**`, app-dev, PEC, or external path. No push, pull,
fetch, PR, or merge.

## 6. Evidence and Validation Plan

In sequence from `WORKING_ROOT` unless noted; every failure stops
subsequent state-changing closeout; all cargo offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no check may install, fetch, or
update anything:

- the §3.2 live offline case-1 verification (mandatory; a missing build
  prerequisite is a truthful BLOCKED result);
- page-assertion cross-check: every value asserted by the two new notes
  and the corrected cell is confirmed against the live run, the frozen
  input bytes, the committed T6 bundle
  (`SUITE_RUN_MECHANICS.json` tallies and `MANIFEST.json` identity), the
  suite count assertion, or the preserved W1/W2 chain evidence — no
  asserted value may originate in this tranche;
- frozen-surface guard: `git status --porcelain` over
  `validation/witness/**`, `validation/evidence/**`, `fixtures/**`,
  `validation/benchmarks/**`, `validation/hand_calcs/**` is empty;
- byte-identity of the DEL-09-04 `## Remaining` section (both bullets)
  against the pre-edit text;
- `python3 tools/validation/validate_claims_language.py --repo-root .`
  and `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`; JSON parse of any new/changed
  `.json` (expected: only the containment JSON);
- changed-path containment:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`,
  persisting JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json`.

The branch-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) run once at W4 wave closeout per
the controlling dispatch.

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate failure, §6 check failure, live-run
  discrepancy against the drafted note content, frozen-surface drift, or
  unexpected changed path stops closeout; record truthful evidence under
  `instances/W4/T7/` and return to the W4 manager.
- No scope drift: no reproduction bundle, no witness regeneration, no
  MAINTAINER_REVIEWED promotion, no GUI-workflow evidence, no DEC-046
  content, no lifecycle/stage/release/acceptance act, no fallback-fixture
  or `validation.rs` edit, no DEL-10-05 or code work.
- A repair need outside the §5 fence is reported and returned for a new
  lawful selection, not fixed here.
- Dirty checkout beyond lawful R14 state: stop and return the condition.

## 8. Rerun Triggers

A rerun (new execution record, same governed brief unless superseded) is
required when any of these changes after the implementation base commit:
the runner bin/binding sources or DEC-065 exit policy; the
constant-effort consumption/warning surfaces in `core/product_physics`;
the frozen E1 or del1005 surfaces; the T6 bundle (which is immutable —
any successor bundle triggers a currency re-check instead); the
DEL-09-04 Remaining scope, lifecycle state, or the page through another
lane; applicable DAG-007 rows or the DAG pointer;
`software-workflow.json`; or a prior failed/blocked result after its
condition resolves. Consequence carried forward (already stated on the
page and restated in MEMORY): any subsequent clean-checkout reproduction
executes from a post-refresh source commit under a fresh run ID and new
immutable bundle; completed bundles are never edited. A material
governance change returns the brief itself to HELP_HUMAN before any
rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- any reproduction run bundle, reproduction acceptance, or
  evidence-posture promotion;
- MAINTAINER_REVIEWED case-page promotion, GUI-workflow validation
  evidence, or closure of any DEL-09-04 Remaining bullet (both bullets
  owner-gated per W3 rows 7–8, byte-preserved);
- promotion of release thresholds, final tolerance policy, CI gate
  policy, or any DEC-046 record; new tolerance constants, acceptance
  criteria, or normative content of any kind;
- the `export-results` binding or any DEL-10-05, code, schema, test,
  fixture, or tool work (including the §2.3/§2.4 observed surfaces);
- edits to frozen witnesses, generators, reproduction bundles, sweeps,
  the T6 bundle, or suite crates;
- lifecycle transition, stage/milestone advancement, issuance, receipt
  append, release, packaging, publication, push, PR creation/merge,
  hosted CI, network use, or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W4 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T7
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: two dated notes plus one corrected case-1 cell on the reproduction page (R13 precedent form), head-anchored by a mandatory live offline case-1 run and the committed T6 bundle, with three no-edit dispositions recorded and both Remaining bullets byte-preserved, per §3–§4 within the §5 fence
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T7
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (full restructure; silent cell edit; witness regeneration; out-of-lane fixture/code fixes; NO-OP)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W4/T7/VERIFY_BRIEF.md` (C1–C10 confirmed; ten-class screen pass; one Low shorthand defect recorded in the status block, operative §2.1/§3.1 text controlling)
EffectStatus: EFFECTIVE — EXECUTION RELEASED BY W4 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)
PreservedGates: reproduction acceptance and evidence-posture promotion; MAINTAINER_REVIEWED case-page promotion; GUI-workflow validation evidence; DEC-046 threshold/tolerance promotion; export-results binding (DEL-10-05); PDU-037/PDU-013/PDU-060 holds; lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the
agent classifies, selects among defensible documentation shapes under
D-54, and proposes. The W4 manager progresses `EffectStatus` only after
the independent refutation returns `COMMIT-SAFE`, under the R14 campaign
plan's execution rules and the durably landed D-52/D-54 records. No
execution is released by this document in its current state.
