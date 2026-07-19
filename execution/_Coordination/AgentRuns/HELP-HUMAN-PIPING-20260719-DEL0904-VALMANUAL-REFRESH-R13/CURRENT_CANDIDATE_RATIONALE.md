# CURRENT CANDIDATE RATIONALE — CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N1
**Prepared by:** ORCHESTRATOR (Agent 1) for HELP_HUMAN
**Date:** 2026-07-19
**Lane:** D-54/`DEC-087` reasoned selection on the D-52/`DEC-085`
standing-approval overlay
**Candidate:**
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`

## 1. Sources Opened (Live Tree, HEAD 45ec0524d3b0c155392553a3b3e4190534ff0fe8)

Paths relative to `projects/chirality-piping/` unless noted.

- Parent plan and run frame: `ORCHESTRATION_PLAN.md` in this AgentRuns
  directory (plan version 1, TERMINAL_FAN_OUT_IN, serialized N1→N2→N3→N4).
- DEL-09-04 deliverable folder (under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
  `_STATUS.md` (incl. both `## Remaining` bullets and full History),
  `MEMORY.md` (R11 entry newest), `_CONTEXT.md` (Type `DOC_UPDATE`),
  `Dependencies.csv`, and
  `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R11.md`
  (pinned source commit `23eeaabc904064e2297690e391df153dea116ff0`; bundle
  `REPRO_DEL0904_20260719T202023Z_23eeaabc9040`, `INTERNALLY_VERIFIED`).
- The page: `docs/validation_manual/headless_runner_reproduction.md` (live
  stale rows at lines 43, 59, 95–99).
- R12 chain:
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`
  (adopted/executed; §5 item 7 and §7 record the DEL-09-04 follow-on),
  `../HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
  (implementation facts, check table incl. the five witness-run exit codes
  0/0/0/1/1, claims 1–30), and
  `../HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/HANDOFF_STATE.md`
  (this refresh named as the natural next selection; fresh-run-ID
  consequence).
- Runner ground truth: `core/runner/headless/src/bin/openpipestress-runner.rs`
  (RunBenchmark/RunRegression routed through `execute_suite_verb`; stub
  diagnostic confined to the `ExportResults` arm; payload-missing diagnostic
  codes) and `core/runner/headless/src/benchmark_binding.rs` (per-case
  statuses, blocked-case codes).
- Witness evidence: `validation/witness/inputs/generate_del1005_payload_binding_inputs.py`;
  the five `validation/witness/inputs/del1005_payload_binding_*_input.json`
  (payload shapes and case lists parsed); the five
  `validation/witness/generated/del1005_payload_binding_*.json` (diagnostics,
  `suite_run` counts, `whole_suite_default_applied`, claim-posture string
  parsed); the frozen
  `validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json`
  (contains only `request`, operation `run_benchmark`) and its committed
  generated witness (stub diagnostic).
- Governance: `execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md`;
  `execution/_Coordination/_DECISIONS/D-54_reasoned_discretion_standing_approval_refinement.md`;
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 rows (DEC-025, DEC-046,
  DEC-065, DEC-080, DEC-081, DEC-085, DEC-087; SCA-007/DEC-080 revision
  note); prior adopted DEL-09-04 brief
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  (§8 rerun triggers).
- Graph/loop state: `execution/_DAG/_LATEST.md` (approved `DAG-007`);
  `execution/_DAG/DAG-007/DependencyEdges.csv` DEL-09-04 rows 766–777;
  `loop/LOOP_RECEIPTS.md` tail (Receipts 57–59; cursor `Receipt-59`).
- Workflow: `software-workflow.json` (checks, path rules, always-checks);
  root `docs/SOFTWARE_WORKFLOW_PROFILE.md` (existence confirmed); root
  `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py` (existence confirmed).

## 2. Fast-Reject Limits Screen (all 10 D-52 §4.1 classes, D-54 §3.1 refined)

| # | D-52 §4.1 class | Verdict | Basis |
|---|---|---|---|
| 1 | Irreducible owner preference / two defensible outcomes remain | PASS | No irreducible personal preference is needed. Several defensible documentation shapes exist (page structure, section granularity); under D-54/`DEC-087` that is expressly no longer an automatic referral — reasoned selection applies and is recorded in §3–§5. |
| 2 | Personal/professional/fiduciary/safety/legal/regulatory accountability | PASS | The refreshed page remains draft development-verification evidence; it restates that validation occurs in the user's accepted professional tools and that acceptance and professional judgment stay with the responsible engineer. No reliance posture changes. |
| 3 | Conflict ruling not determined by the authority chain | PASS | The live tree (bound verbs, committed witnesses) versus the stale page is a truth-repair, not an authority conflict: the R12 adopted brief §7 and HANDOFF_STATE already recorded this exact follow-on and routed it here. No two authorities disagree. |
| 4 | Scope/criteria change, new normative content, exception/override | PASS — attended closely | The refresh DESCRIBES landed behavior; it defines nothing. Every expected exit code, diagnostic code, and per-case count the page will assert comes from committed witness evidence (`del1005_payload_binding_*` pairs, preserved R12 N3 evidence) plus the owner-ruled DEC-065 exit policy. Blocked-case fail-closed behavior is described strictly as regression evidence, not converted into acceptance criteria. The `_STATUS.md` edit strikes a dependency that DEL-10-05 discharged; it closes no bullet — MAINTAINER_REVIEWED case-page promotion and GUI-workflow evidence stay open in the same bullet, and the MAINTAINER_REVIEWED promotion is NOT performed by this tranche. The owner-gated tolerance bullet is byte-preserved. |
| 5 | Lifecycle/stage/release/acceptance/promotion acts | PASS | None performed or implied; DEL-09-04 stays `IN_PROGRESS`; reproduction acceptance and evidence-posture promotion are named preserved gates (brief §9–§10). |
| 6 | Third-party/procurement/spend/publication/external systems | PASS | Local-only docs+state tranche; no network, publication, push, merge, or external commitment. |
| 7 | Merge/integration authority, destructive/irreversible action | PASS | Owner merges; nothing is deleted from history — the struck dependency is recorded as landed; frozen E1 surfaces, del1005 witnesses, and completed bundles are read-only; failures fail closed. |
| 8 | Protected/private data exposure | PASS | All referenced fixtures are invented public project content per the existing witness posture; no protected standards data, private data, or secrets are touched. |
| 9 | Evidence unavailable/stale/unverifiable, claim beyond warrant | PASS | Every outcome-determining premise was verified in the live tree at N1 (§1), including parsing all five generated witnesses and both frozen stub surfaces; the brief mandates re-verification of every page assertion against committed witnesses before closeout and claims no run result of its own. |
| 10 | Domain-engine/prover/secrets/higher-order human boundaries | PASS | No `_DomainEngines/**`, prover, account/secret, or ResponsibleParty surface is involved. |

**Screen result: PASS — no D-49/D-52/D-54 limit hit; no near-miss recorded;
the brief is not `INELIGIBLE_REFERRED`.**

## 3. Four-Lens Analysis

- **Ontology.** The object is one bounded documentation-refresh brief for an
  existing deliverable's existing derivative surface. Every entity already
  exists: the page, the bound verbs, the committed witness family, the
  Remaining bullet, the approved DAG rows. The tranche creates no new project
  entity, criterion, threshold, or authority class; it re-aligns a derivative
  description with accepted upstream truth (the derivative-package rule:
  the page cites its upstream evidence and never substitutes for it).
  Proposal, adoption, execution, evidence, and acceptance remain distinct
  acts.
- **Epistemology.** Every premise is cited to a live artifact: the stub
  confinement and payload-missing codes in the bin source; the witness facts
  in the five committed generated JSONs; the exit codes in the preserved R12
  N3 evidence; the stale rows at named line numbers; the Remaining text in
  `_STATUS.md`; the check registry in `software-workflow.json`. The refreshed
  page will assert only witness-committed values, mark the pre-#287
  expectation historical rather than erasing it, and keep prior pinned
  evidence interpretable — claim status is preserved, never upgraded.
- **Praxeology.** The selected shape gives the executor a bounded, checkable,
  fail-closed path: an exact two-part page structure (frozen E1 + bound
  path), an exact write fence with a self-consistent no-`validation/**`
  determination, a mandatory witness cross-check step for every page
  assertion, byte-identity checks on the seven frozen surfaces and the
  untouched tolerance bullet, halting registered checks, containment, and
  return-to-parent on any out-of-fence discovery. Rerunnable under §8.
- **Axiology.** The tranche restores truthfulness of a governed evidence
  surface (OBJ-008 rigor; OBJ-011 professional-boundary separation), values
  evidence over plausibility (witness-anchored assertions only), preserves
  human responsibility (owner gates on acceptance, promotion, tolerance,
  merge), and preserves truthful attribution (`OwnerCaseSelection: NONE`,
  effect held). Leaving the page stale would let a documented expectation
  silently contradict the tree — the worse outcome on every lens.

All four lenses support the same bounded outcome: author and advance this
candidate brief with the §4 selected shape, effect held pending independent
refutation.

## 4. Materially Rejected Alternatives (D-54 §3.2 item 4)

1. **Append-only staleness note (leave the stale rows in place).** Rejected:
   the Fixture Set table row 3 and the Reproduction Procedure command read as
   current normative-looking expectations; a trailing note beside a
   contradicting row leaves two live answers on one page. Correct in place,
   preserve the historical expectation as an explicitly dated historical
   record.
2. **Full rewrite that drops the tp_runner_015 E1 procedure.** Rejected: the
   frozen E1 procedure is the documented basis of the completed R11
   reproduction bundle and earlier recorded deltas; deleting it would orphan
   pinned evidence. The E1 section is preserved with the dated case-3
   historical note.
3. **Document `run-benchmark` only and defer `run-regression`.** Rejected:
   both verbs landed together in PR #287 with committed witness coverage
   (including the 5/5 nonlinear full-suite witness); documenting half would
   leave the page stale about `run-regression` by construction. The blocked
   nuance (mechanics/stress whole-suite runs contain fail-closed cases) is
   handled truthfully as regression-evidence description, not by omission.
4. **Rerun the clean-checkout reproduction in the same tranche.** Rejected:
   serialized follow-on. A reproduction run is a separate governed act with
   its own brief-family rerun triggers (`CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001`
   §8), a fresh run ID, and a new immutable bundle from a post-#287 commit;
   bundling it would widen the fence into `validation/**` and change the
   profile check set. The page instead states the rerun-trigger consequence.
5. **Also updating `docs/validation_manual/index.md`.** Rejected: the index
   inventories the slice as `DRAFT_EVIDENCE`, which remains true; the parent
   plan's fence does not include it; touching it widens scope without a
   truth-repair need. Any index defect discovered at N3 returns to
   HELP_HUMAN.
6. **Closing the whole E2 Remaining first bullet.** Rejected: only the
   bindings dependency landed. MAINTAINER_REVIEWED case-page promotion and
   GUI-workflow validation evidence remain open in that bullet, and the
   MAINTAINER_REVIEWED promotion is a preserved gate not performed here.
7. **Running `piping-pytest`/`evidence-sweep` anyway ("more checks are
   safer").** Rejected: the profile's path rules select checks from changed
   paths; this tranche changes no `core/**`, `validation/**`, or `tests/**`
   path, and an evidence sweep would write a new `validation/**` artifact the
   fence forbids — running it would itself expand the changed-path set. The
   profile is encoded as it actually reads; extra unrequired state-writing
   checks are scope drift, not rigor.

## 5. Single Selected Outcome

Author `CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001` refreshing
`docs/validation_manual/headless_runner_reproduction.md` into a two-part page
— the byte-preserved-in-substance frozen tp_runner_015 E1 procedure with a
dated 2026-07-19 historical note on case 3 (post-#287 sources exit 1 with
`HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` on the frozen payload-less
input), plus a new bound-path section documenting the five committed
`del1005_payload_binding_*` cases (expected exits 0/0/0/1/1, diagnostics,
per-case counts, whole-suite default, fail-closed blocked-case semantics as
regression evidence only), a refreshed truthful Remaining E2 paragraph, and
the fresh-run-ID rerun consequence — and striking only the landed bindings
dependency from the DEL-09-04 `_STATUS.md` first Remaining bullet, with one
History entry, one MEMORY entry, one new run record, one appended receipt,
inside the exact §5 fence (no `validation/**` write, no sweep) with the §6
witness cross-check plan, effect `HELD`.

## 6. Attempted Failure Mode (Adversarial Self-Test)

**Attempt 1 — "documenting expected exits/diagnostics creates acceptance
criteria (limit 4)."** Fails: every documented expectation is a quotation of
committed witness evidence produced by the adopted, independently verified
R12 execution, mapped through the owner-ruled DEC-065 exit policy. The page
adds a description of what the committed evidence shows; nothing becomes
required of any future run beyond what the tree already encodes, and cases
without an encoded predicate are described as fail-closed regression
evidence, not judged.

**Attempt 2 — "editing the page invalidates the R11 reproduction bundle or
requires rerunning it now (limit 9/limit 5)."** Fails: the bundle is
immutable, pinned to `23eeaabc9040`, and remains truthful for that commit —
the brief says so on the page itself. The clean-repro brief's §8 rerun
triggers require a fresh run ID for any NEW reproduction, which this tranche
states as a consequence and defers as serialized follow-on; no acceptance or
promotion is touched.

**Attempt 3 — "striking the bindings clause is a lifecycle/closure act
(limit 5)."** Fails: the strike records a dependency discharged by another
deliverable's completed, receipted tranche (DEL-10-05, PR #287, Receipt-59);
the bullet stays open with its other two obligations, lifecycle stays
`IN_PROGRESS`, and closure of the bullet, the E2 row, and the deliverable all
remain future gated acts.

**Attempt 4 — "defer despite no defect."** Does not survive: the live tree
contradicts a governed evidence page; the R12 handoff routed exactly this
repair as the next selection on the R6 path; deferral has no positive basis
and leaves a stale documented expectation standing.

## 7. Attribution Form Fields

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (pending independent refutation)
RuleActivation: NOT_ACTIVATED
AgentJudgment: SELECT_AND_ADVANCE
SelectedOutcome: as stated in §5 above
JudgedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13 / N1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL
OwnerCaseSelection: NONE
RejectedAlternatives: §4 items 1–7
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: PENDING — N2 (instances/N2/RETURN.md)
EffectStatus: HELD
PreservedGates: reproduction acceptance and evidence-posture promotion; MAINTAINER_REVIEWED case-page promotion; GUI-workflow validation evidence; DEC-046 threshold/tolerance promotion; export-results binding (DEL-10-05); lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; D-45; D-06b; F-PIP-1..5
```

## 8. Preserved Gates and Boundaries

- No lifecycle, stage, release, acceptance, issuance, reproduction-
  acceptance, or evidence-posture act; DEL-09-04 remains `IN_PROGRESS`.
- No DEC-046 tolerance/threshold creation or promotion; no new normative
  content or acceptance criteria anywhere in the tranche.
- No reproduction run; completed immutable bundles are read-only and remain
  truthful for their pinned commits.
- Frozen E1 fixtures/generator/witnesses, the del1005 witness family, suite
  crates, runner sources, schemas, and tests are read-only.
- MAINTAINER_REVIEWED case-page promotion and GUI-workflow validation
  evidence stay open; the `export-results` stub stays DEL-10-05 work.
- Owner merges; adoption effect only through the standing rule after
  independent `COMMIT-SAFE`; `EffectStatus: HELD` until HELP_HUMAN
  progresses it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
