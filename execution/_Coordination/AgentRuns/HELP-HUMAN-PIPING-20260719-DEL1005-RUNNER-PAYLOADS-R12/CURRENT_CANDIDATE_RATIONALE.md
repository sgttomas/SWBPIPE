# CURRENT CANDIDATE RATIONALE — CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001

**Run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`, node N1
**Prepared by:** ORCHESTRATOR (Agent 1) for HELP_HUMAN
**Date:** 2026-07-19
**Lane:** D-54/`DEC-087` reasoned selection on the D-52/`DEC-085`
standing-approval overlay
**Candidate:**
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`

## 1. Sources Opened (Live Tree, HEAD 96563e8e09b89908e13e6b2f1f1139aca3283855)

Paths relative to `projects/chirality-piping/` unless noted.

- DEL-10-05 deliverable folder: `_STATUS.md` (incl. `## Remaining`),
  `MEMORY.md`, `_CONTEXT.md`, `ScopeOfWork.md`, `Dependencies.csv`,
  `_run_records/WORKING_ITEMS_RUN_2026-07-05_TP-RUNNER-015.md`,
  `_run_records/WORKING_ITEMS_RUN_2026-07-04_TP-RUNNER-014.md`
  (under `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/`).
- Runner surfaces: `core/runner/headless/src/bin/openpipestress-runner.rs`,
  `core/runner/headless/src/lib.rs`, `core/runner/headless/Cargo.toml`,
  `schemas/headless_runner.schema.yaml`,
  `tests/test_headless_runner_contract.py`.
- Suites: `validation/benchmarks/mechanics/` (README, `Cargo.toml`,
  `src/lib.rs` incl. `fixture_inventory()` / `fixture_inventory_ids()`),
  `validation/benchmarks/stress/` (README, `Cargo.toml`, `src/lib.rs` incl.
  `fixture_inventory()`), `validation/benchmarks/nonlinear/` (README,
  `Cargo.toml`, `src/lib.rs` incl. `fixture_inventory()` and assembled
  inventories/observations; DEC-046 `*.dec046.json` policy records listed).
- E1 surfaces: `docs/validation_manual/headless_runner_reproduction.md`,
  `validation/witness/inputs/generate_tp_runner_015_inputs.py`, the three
  `validation/witness/inputs/tp_runner_015_final_cli_*_input.json` fixtures.
- Governance: `execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md`;
  `execution/_Coordination/_DECISIONS/D-54_reasoned_discretion_standing_approval_refinement.md`;
  `execution/_Coordination/_DECISIONS/_REGISTER.md` rows D-33, D-49, D-52,
  D-54; `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 rows DEC-025,
  DEC-046, DEC-064, DEC-065, DEC-080, DEC-081, DEC-087;
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  (structural model).
- Graph/loop state: `execution/_DAG/_LATEST.md`;
  `execution/_DAG/DAG-007/DependencyEdges.csv` (DEL-10-05 rows) and
  `DeliverableNodes.csv` row DEL-10-05; `loop/LOOP_RECEIPTS.md`
  (Receipts 56–58); `loop/` workplan listing (active
  `WORKPLAN_2026-07-18b_piping_loop.md`).
- DEL-09-04 `_STATUS.md` (`## Remaining` E2 residual naming the runner
  payload stubs) under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`.
- Workflow: `software-workflow.json` (checks, path rules, always-checks);
  root `docs/SOFTWARE_WORKFLOW_PROFILE.md` (existence confirmed); root
  `tools/software_workflow/run_registered_checks.py` (`--help` surface) and
  `tools/software_workflow/validate_change_scope.py`; root
  `tools/validation/validate_claims_language.py`,
  `validate_path_anchors.py`, `validate_piping_loop_receipts.py`
  (existence confirmed).
- Parent run: `ORCHESTRATION_PLAN.md` and `WORK_GRAPH.json` in this AgentRuns
  directory.

## 2. Fast-Reject Limits Screen (all 10 D-52 §4.1 classes, D-54 §3.1 refined)

| # | D-52 §4.1 class | Verdict | Basis |
|---|---|---|---|
| 1 | Irreducible owner preference / two defensible outcomes remain | PASS | No irreducible personal preference is needed. Several defensible implementation shapes exist (payload naming, case addressing, module layout); under D-54/`DEC-087` that is expressly no longer an automatic referral — reasoned selection applies and is recorded in §4–§5. |
| 2 | Personal/professional/fiduciary/safety/legal/regulatory accountability | PASS | The tranche produces regression evidence in the suites' existing claim posture only; release thresholds, professional reliance, and code-compliance remain `TBD` owner-gated and are named preserved gates. |
| 3 | Conflict ruling not determined by the authority chain | PASS | DEC-065 determines verbs, I/O posture, process policy, and exit codes; the suites determine recorded values and claim posture; the Remaining item determines scope. No conflicting authorities require a ruling. The known E1 case-3 staleness consequence is routed to HELP_HUMAN as follow-on, not resolved here. |
| 4 | Scope/criteria change, new normative content, exception/override | PASS | The binding implements an existing DEL-10-05 Remaining item anticipated by TP-RUNNER-015 ("downstream payload binding must be supplied in a later bounded tranche"). Acceptance predicates derive from DEC-065's exit policy, the suites' recorded comparison bases, and the Remaining text; no new tolerance, threshold, or acceptance criterion is created. Payload field names are implementation shape, not normative content. |
| 5 | Lifecycle/stage/release/acceptance/promotion acts | PASS | None performed or implied; DEL-10-05 stays `IN_PROGRESS`; explicitly excluded in the brief (§9). |
| 6 | Third-party/procurement/spend/publication/external systems | PASS | Local-only; no network, publication, push, merge, or external commitment; DEC-065 policy forbids network/daemon/telemetry. |
| 7 | Merge/integration authority, destructive/irreversible action | PASS | Owner merges; no history rewriting; frozen fixtures/witnesses/bundles are read-only; failures fail closed. |
| 8 | Protected/private data exposure | PASS | All fixtures are invented public project content per the suite READMEs and the witness generator's stated posture; no protected standards data is touched. |
| 9 | Evidence unavailable/stale/unverifiable, claim beyond warrant | PASS | Every outcome-determining premise was verified in the live tree (§1); the brief claims no run result and holds effect pending independent refutation. |
| 10 | Domain-engine/prover/secrets/higher-order human boundaries | PASS | No `_DomainEngines/**`, prover, account/secret, or ResponsibleParty surface is involved. |

**Screen result: PASS — no D-49/D-52/D-54 limit hit; no near-miss recorded;
the brief is not `INELIGIBLE_REFERRED`.**

## 3. Four-Lens Analysis

- **Ontology.** The proposed object is one bounded implementation brief for an
  existing deliverable's existing Remaining item. The entities involved are
  stable and already exist: the DEC-065-settled CLI verbs (`run-benchmark`,
  `run-regression` are stable verbs today, stubbed), the three suite crates
  with stable `fixture_id`-keyed inventories, and the DEL-09-04 E2 residual
  that names these stubs. The brief creates no new project entity, deliverable,
  scope row, or authority class; it converts an existing stub into an
  existing-verb behavior. Proposal, adoption, execution, evidence, and
  acceptance remain distinct acts.
- **Epistemology.** Every premise is cited to a live artifact: the stub arm
  and diagnostic in `openpipestress-runner.rs`; the Remaining texts in both
  deliverables' `_STATUS.md`; DEC-065's exit and process policy in
  `SOFTWARE_DECOMP.md` §12; the suites' regression-evidence claim posture in
  their READMEs; the check registry in `software-workflow.json`. The brief
  claims no execution result, requires per-case observed-vs-recorded evidence,
  and keeps every claim inside the regression-evidence posture — pass/fail
  reporting reuses recorded comparison bases and fails closed rather than
  inventing a tolerance where a basis cannot be reused.
- **Praxeology.** The selected design gives the later executor a bounded,
  executable, fail-closed path: verb-named payloads mirroring the existing
  `solve` wrapper, suite + `fixture_id` addressing against the crates' own
  inventories, one binding module in the existing crate, an exact enumerated
  write fence, halting per-check validation, and explicit return-to-parent on
  any out-of-fence repair need. Rerunnable; no silent partial skips; frozen E1
  surfaces preserved byte-for-byte so prior reproduction evidence stays
  interpretable.
- **Axiology.** The tranche advances the adopted project purpose (OBJ-008
  verification/regression automation; the R6-path prerequisite chain) with
  evidence over plausibility, preserves human responsibility (owner gates on
  thresholds, lifecycle, acceptance, merge), preserves prohibition integrity
  (DEC-065 no-network/no-daemon/no-telemetry; F-PIP-1..5; DEC-081 claims
  fence), and preserves truthful attribution (agent judgment under owner
  standing approval; `OwnerCaseSelection: NONE`; effect held). No new value
  hierarchy is created.

All four lenses support the same bounded outcome: author and advance this
candidate brief with the §4 selected design, effect held pending independent
refutation.

## 4. Materially Rejected Alternatives (D-54 §3.2 item 4)

1. **Suite-only addressing (no per-case list).** Rejected: the Remaining item
   and the DEL-09-04 E2 residual are about *per-case* reproduction through the
   runner; whole-suite-only binding would leave the per-case need served by
   suite tests again.
2. **Global fixture-ID-only addressing (no suite selector).** Rejected: no
   accepted cross-suite uniqueness registry exists; suite-scoped `fixture_id`
   addressing reuses each crate's own inventory as the identity authority
   without creating a new registry (limit-4 avoidance).
3. **Both verbs accept all three suites interchangeably.** Rejected: it
   collapses the accepted suite ontology (DEL-09-01/DEL-09-02 benchmark
   suites; DEL-09-03 nonlinear regression suite) and makes the two
   DEC-065-settled verbs redundant aliases; the selected verb-to-suite mapping
   mirrors the deliverable structure already in the tree.
4. **New generic payload envelope names (e.g., `downstream_payload`).**
   Rejected: the settled TP-RUNNER-015 CLI wrapper precedent is verb-named
   (`solve.preview_model`); following it is the coherent, least-novel shape.
5. **A new standalone binding crate.** Rejected: adds packaging surface with
   no accepted basis; the existing lib/bin split in
   `core/runner/headless` already hosts bounded modules and unit tests, and
   the bin can only reach code through that crate anyway.
6. **Implementing comparison logic with a new numeric tolerance constant for
   awkward cases.** Rejected outright: it would create normative content and
   collide with the DEC-046 owner gate; the selected design fails such cases
   closed with structured diagnostics instead.
7. **Also updating `docs/validation_manual/headless_runner_reproduction.md`
   case 3 and DEL-09-04 Remaining wording in the same tranche.** Rejected as
   default scope: those are DEL-09-04-owned surfaces; bundling them widens the
   write fence across deliverables. The consequence is recorded as follow-on
   work returned to HELP_HUMAN; DEL-09-04 edits are allowed only under the
   brief's §5 item 7 explicit-release condition.

## 5. Single Selected Outcome

Author `CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001` binding `run-benchmark`
(suites `mechanics`, `stress`) and `run-regression` (suite `nonlinear`) via
verb-named optional payload objects `{suite, cases[]}` addressed by suite-local
`fixture_id`, executed through one new bounded binding module in
`core/runner/headless` with path dependencies on the three suite crates,
reporting per-case observed-vs-recorded deltas and match status in the existing
regression-evidence claim posture, DEC-065 exit policy (0 all-match / 1
blocking diagnostics / 2 usage), payload-missing diagnostics per the existing
naming pattern, and the stub diagnostic retained for `export-results` only —
inside the exact write fence and validation plan of the brief, with effect
`HELD`.

## 6. Attempted Failure Mode (Adversarial Self-Test)

**Attempt 1 — "per-case pass/fail is a new acceptance criterion (limit 4)."**
Fails: the pass/fail predicate is the suites' own recorded comparison basis,
already encoded and exercised by their tests; the runner output restates that
existing regression-evidence comparison per case and adds the DEC-065 exit
mapping that the owner already ruled (`0` completed/no blocking diagnostics,
`1` blocking). Nothing about release readiness, thresholds, or acceptance is
decided; cases without a reusable recorded basis fail closed rather than
receiving an invented criterion.

**Attempt 2 — "extending the CLI input wrapper reopens the settled DEC-065 /
CLM-004 surface."** Fails: DEC-065 settled binary name, verbs, I/O posture,
process policy, and exit policy — none change. TP-RUNNER-015 expressly left
`run-benchmark`/`run-regression` as stable verbs "until downstream payload
bindings exist," and DEL-10-05 `ScopeOfWork.md` CLM-004 scopes the settled
wrapper to the TP-RUNNER-015 surface. Supplying the anticipated payload
binding is the recorded next tranche, not a reopening.

**Attempt 3 — "defer despite no defect."** Does not survive: the loop's
adopted direction advances the widest lawful selectable tranche; the DEL-09-04
E2 residual is blocked on exactly this DEL-10-05 item; no risk or evidence
record in the live tree supports deferral. Deferral has no positive basis.

## 7. Attribution Form Fields

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (pending independent refutation)
RuleActivation: NOT_ACTIVATED
AgentJudgment: SELECT_AND_ADVANCE
SelectedOutcome: as stated in §5 above
JudgedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12 / N1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL
OwnerCaseSelection: NONE
RejectedAlternatives: §4 items 1–7
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: PENDING — N2 (instances/N2/RETURN.md)
EffectStatus: HELD
PreservedGates: export-results binding; DEC-046 threshold/tolerance promotion; lifecycle/stage/issuance/release/acceptance; reproduction acceptance and evidence-posture promotion; prover activation/correlation; publication/external action; merge authority; D-45; D-06b; F-PIP-1..5
```

## 8. Preserved Gates and Boundaries

- No lifecycle, stage, release, acceptance, issuance, or reproduction-
  acceptance act; DEL-10-05 remains `IN_PROGRESS`.
- No DEC-046 tolerance/threshold creation or promotion; no new normative
  content or acceptance criteria anywhere in the tranche.
- `export-results` stays stubbed with its existing diagnostic; DEL-08-01
  coupling untouched.
- Frozen E1 fixtures, generator, committed generated witnesses, completed
  reproduction bundles, suite recorded values/policy records/READMEs, and (by
  default) all DEL-09-04 surfaces are read-only.
- DEC-065 local process policy preserved: no network, daemon, telemetry,
  hidden filesystem mutation, or direct SQL access.
- Owner merges; adoption effect only through the standing rule after
  independent `COMMIT-SAFE`; `EffectStatus: HELD` until HELP_HUMAN progresses
  it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
