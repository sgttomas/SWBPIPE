# VERIFY_BRIEF_V2 — Second-Round Fresh-Context Adversarial Verification of CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001 (v2, post-amendment)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Verifier:** fresh-context adversarial verifier, second round (governed Agent 2, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Objects reviewed (both at amendment v2):**
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CURRENT_CANDIDATE_RATIONALE.md`
- prior verifier return (context, read-only, preserved): `instances/W1/T1/VERIFY_BRIEF.md`

**Verified against:** live tree at HEAD `6152908b3246df61150dc91e3558788b05dfb643`,
branch `claude/piping-r14-pkg04-mechanics` (confirmed by `git rev-parse HEAD` /
`git branch --show-current`; working tree clean except the lawful untracked R14
AgentRuns directory and the candidate brief). All checks offline; no install,
fetch, or network use. No file modified other than this return. Per the parent
tasking, claim C7's del1005 byte-currency reproduction is relied upon from the
v1 verifier's recorded offline run — which executed at this identical HEAD and
an unchanged tracked tree — supplemented by the independent spot-checks
recorded below; every other claim was re-derived directly from live files.

---

## 1. Amendment-Cure Assessment

**(a) Witness byte-stability predicate (v1 blocking reasons 1–2; v1 D1, refuted v1 C7/C11).** CURED.
- §1 accepted basis now describes the three committed
  `tp_runner_015_final_cli_*` witnesses as "historical records, truthful for
  their pinned pre-#287 commits … NOT current at HEAD" and the five
  `del1005_payload_binding_*` witnesses as "current at HEAD" — both statements
  verified consistent with `docs/validation_manual/headless_runner_reproduction.md`
  (dated 2026-07-19 note; case-3 table row distinguishing at-or-after-#287
  behavior from the committed pre-#287 witness) and with the live tree
  (`core/runner/headless/src/bin/openpipestress-runner.rs` line 87 `suite_run`
  field in `CliOutput`; line 299 `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`
  for `RunBenchmark`, versus the committed benchmark witness's
  `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` at
  `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
  line 28).
- §3.7 now defines CLI stability as (a) del1005 stdout vs the five committed
  witnesses, (b) tp_runner_015 stdout vs pre-tranche baseline captures built
  offline at the recorded base commit, with the historical committed
  tp_runner_015 witnesses expressly "NOT the comparison target"; §4.3 orders
  the baseline capture BEFORE the first source edit; §6 repeats the same split.
- Adversarial sweep of the whole brief for any surviving
  historical-witness-match-at-HEAD requirement: none found. §7's phrase "any
  committed-witness byte drift" is best read as edit/drift of the frozen
  witness files themselves (per §4.3's do-not-touch instruction); the adverse
  reading (stdout-vs-committed-witness for tp_runner_015) is foreclosed by
  §3.7(b)'s explicit exclusion. Recorded as wording nit V2-D6, not a defect of
  substance.
- The amended predicate is satisfiable at the implementation head: the
  del1005 comparison passes today per the v1 reproduction at this HEAD, and
  the tp_runner_015 baseline comparison is self-referenced to the pre-tranche
  build, which exists by construction.

**(b) DEC-025 sweep treatment (v1 blocking reason 3; v1 D2).** CURED IN SUBSTANCE; one vestigial clause remains (V2-D1).
- §6 now truthfully records the single wave-level sweep as running "per the
  controlling HELP_HUMAN dispatch to the W1 manager", quotes that dispatch,
  names the R14 `ORCHESTRATION_PLAN.md` execution-rules sentence it refines
  ("per-tranche commits, each gated by the tranche's full check set including
  the DEC-025 evidence sweep" — verified present in the plan's Execution
  rules; the quote omits the plan's trailing "(code-touching)" parenthetical,
  V2-D3), states the refinement's bound ("no tranche commit is ever pushed
  un-swept"), and commits to surfacing the refinement in the W1 manager
  return for HELP_HUMAN fan-in visibility.
- DEC-025's own accepted semantics were verified independently:
  `docs/BUILD_AND_RELEASE.md` §5.1 defines the sweep's merge-gate role as
  "the required pre-push/fan-in evidence for every parallel agent development
  branch", with the recommended pattern commit → sweep at clean committed
  HEAD → evidence-only closeout commit → push. The brief's treatment keeps
  those pre-push gate semantics intact; the plan's per-tranche-commit gating
  sentence was stricter than DEC-025, and its refinement is by the plan's own
  author (HELP_HUMAN), surfaced back to that author before any push or
  fan-in. No owner-level (DEC-025) authority is touched.
- Residual: §5 item 9 still carries the v1 clause "the single wave-level
  DEC-025 sweep runs once at W1 closeout per the campaign plan" — the exact
  attribution the v1 verifier's D2 flagged, un-amended (the amendment record
  names §1/§2/§3.7/§4.3/§6 only). Recorded as V2-D1 below; assessed
  non-blocking because the fence item's operative content (no sweep artifact
  written in this tranche) is unambiguous, §6 of the same document carries
  the corrected and controlling attribution, and no execution divergence is
  possible under either reading.

**(c) Amendment-record truthfulness.** PASS. The brief's amendment record
states plainly that the v1 verifier "returned `BLOCK`", identifies the refuted
premise accurately (del1005 byte-identical at `6152908b3`; tp_runner_015
truthful only for pinned pre-#287 commits), names the preserved v1 artifact,
and lists the amended sections. The rationale's amendment record additionally
records the v1 class-9 screen correction and restates C7 in v2 form. Nothing
in either record softens, reinterprets, or disputes the v1 BLOCK.

## 2. Per-Claim Verdict Table (C1–C15, independently re-derived)

| Claim | Verdict | Evidence (live tree) |
|---|---|---|
| C1 | **CONFIRMED** | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/_STATUS.md`: `IN_PROGRESS` (line 3); `## Remaining` holds exactly one item (line 7) matching the brief's selected item 1 verbatim. |
| C2 | **CONFIRMED** | `.../DEL-04-04_Nonlinear support active-set solver/_STATUS.md`: `IN_PROGRESS`; Remaining item 1 (line 7) matches the brief's selected item 2 verbatim; items 2–6 (lines 8–12) are the PDU-035 REVIEW/dimensional hold, the friction path-history "(gated: new D-XX ruling…)" row, and three threshold-promotion rows (DEC-046 / PRD D6/D9 / DEC-052/DEC-054; one "stage-gated: R5 release evidence") — all excluded by brief §9. |
| C3 | **CONFIRMED** | `grep -rn "ResultEnvelope {" core --include="*.rs"` (excluding `ResultEnvelopeRef`): constructors only at `core/reporting/result_export/src/lib.rs` 941/1000/1226 (inside the crate's test module) and test file `core/reporting/report_package/tests/container.rs` 155. `core/runner/headless` does not even import the crate today. No non-test constructor at HEAD. |
| C4 | **CONFIRMED** | `core/runner/headless/src/lib.rs`: `validate_result_with_optional_envelope_payload` (line 563) → payload checks `deliverable_id == "DEL-08-04"` (590), envelope-id vs `result_envelope_ref` (607), `HUMAN_REVIEW_REQUIRED` (622/629), non-empty `result_sets` (634), `deterministic_ordering` (649), result-envelope checksum presence (660). Callers passing `Some(...)` only at 1126/1149, inside `#[cfg(test)]` (begins 907); none in `src/bin/`. |
| C5 | **CONFIRMED** | `core/solver/nonlinear_integration/src/lib.rs`: `pub struct NonlinearFrameSolveResult` (303) with `pub assumptions: Vec<String>` (311), `pub limitations: Vec<String>` (312), populated from `pub fn assembled_loop_assumptions()` (506) / `pub fn assembled_loop_limitations()` (518); crate version `0.1.0` (`Cargo.toml` line 3). `grep -c "assumption"` and `"limitation"` in `core/product_physics/src/lib.rs` both return 0 — no consumer; the vectors are dropped. |
| C6 | **CONFIRMED** | `core/product_physics/src/lib.rs`: `pub struct MechanicsEnvelope` (590) with `pub results` (597) / `pub diagnostics` (598); `Diagnostic.source: Option<String>` (662); `append_nonlinear_support_loop_results` (1540) maps diagnostics/result rows. The brief maps existing envelope content; it recomputes nothing. |
| C7 (v2) | **CONFIRMED** | Five del1005 witnesses byte-current at HEAD: relied on the v1 verifier's recorded offline reproduction, executed at this identical HEAD `6152908b3` on an unchanged tracked tree (all witness paths git-clean now; DEL-10-05 `_STATUS.md` history corroborates the del1005 witnesses landing with "frozen TP-RUNNER-015 surfaces byte-unchanged" in R12). Three tp_runner_015 witnesses historical: verified directly — manual's dated 2026-07-19 note ("remain truthful for their pinned pre-#287 commits and are not edited"), live `CliOutput.suite_run` field (bin line 87) absent from the pre-#287 witness shape, and live benchmark diagnostic `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (bin line 299) vs the witness's stub code. The amended §3.7 predicate is satisfiable and protects both surfaces (committed witnesses untouched; baseline captured pre-edit). |
| C8 | **CONFIRMED** | Every §4 task writes inside the §5 fence: headless crate has its own `Cargo.lock` (no shared workspace lockfile — `core/runner/headless/` listing; no `[workspace]` in either manifest), tests live in `src/` (no `tests/` dir in headless or product_physics — product_physics contains only `src`, `examples`, `target`), baseline captures are ephemeral scratch (§5 ephemeral clause), instance-dir writes incl. `CHANGE_SCOPE_CONTAINMENT.json` and `EXECUTE_RETURN.md` are fence item 8. Exclusions genuinely unneeded: `result_export` public API suffices (`ResultEnvelope`, `validate_result_envelope`, `result_export_document` all `pub`); DEL-10-05 `_STATUS.md` Remaining still carries the open `export-results` bullet, truthfully deferred. |
| C9 | **CONFIRMED** (descriptor defect V2-D4) | `DEL-04-02/Dependencies.csv`: `DAG-002-E0105..E0109, E0432..E0434` all `SATISFIED` (8/8). `DEL-04-04/Dependencies.csv`: `TP-DAG-004-DEL-04-04-E001..E005` (root targets DEL-00-01/02/03/06/08) `SATISFIED`; `E006..E009` `PENDING` targeting DEL-04-03, DEL-04-01, DEL-04-06, DEL-02-02, each with committed implementation evidence live (`core/solver/linear_supports`+`nonlinear_supports`, `core/solver/frame_kernel`, `core/solver/diagnostics`, `core/units`), consumed read-only. The brief names the PENDING posture and resolves/edits no row. Descriptor defect: E009 is `DependencyType` CONSTRAINT (a non-root constraint), so "non-constraint" mislabels it — row identity, status, and treatment are nonetheless exact. |
| C10 | **CONFIRMED** | No predicate or task in the amended brief introduces a tolerance constant, threshold, acceptance criterion, or release/CI vocabulary; new diagnostics follow the existing `HEADLESS_RUNNER_*` fail-closed pattern; solver-version content is crate-constant-derived; §9/§10 preserve PDU-035, DEC-046, and every threshold hold; lifecycle stays `IN_PROGRESS`; no external action (§5). |
| C11 | **CONFIRMED** | The v2 rationale's ten-class screen was re-derived item by item against the ratified D-52 §4.1 list (`execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md`, lines 85–119) — the ten classes map one-to-one and none is omitted. My independent re-screen (§3 below) reaches the same result on the amended brief: no class hit. The v1 class-9/class-3 hits attached to the v1 brief's false premise and unattributed sweep deviation, both removed by the amendment; D-52's own §7.1 history records BLOCK → rework → fresh post-rework verifier `COMMIT-SAFE` as the lane's recognized cure path, so re-candidacy after cure is lawful rather than an owner-referral bypass. |
| C12 | **CONFIRMED** | cargo 1.92.0 and Python 3.13.7 present offline; `schemas/headless_runner.schema.yaml`, `tests/test_headless_runner_contract.py`, `loop/WORKPLAN_2026-07-18b_piping_loop.md`, `software-workflow.json` exist; root tools exist with the exact CLI surfaces the brief states (`validate_change_scope.py`: `repo` positional, `--allowed` required/append, `--base`; `run_registered_checks.py`: `profile` positional, `--check`, `--output` required). The headless crate built offline for the v1 reproduction at this same tree. |
| C13 | **CONFIRMED** (stale pointer V2-D2) | §10 keeps `OwnerStandingApproval` (DEC-085/D-52 §2, SHA `f14fa77518a0…` verified an ancestor of HEAD via `git merge-base --is-ancestor`; D-54 landing `8825065d5` likewise an ancestor) distinct from `AgentClassification: CLASSIFY_ELIGIBLE` and `AgentJudgment: SELECT_AND_ADVANCE`; records `OwnerCaseSelection: NONE`, `EffectStatus: HELD`, rejected alternatives, and names the rationale artifact. Defect: the `IndependentVerifier` row still reads "PENDING — instances/W1/T1/VERIFY_BRIEF.md", naming the completed v1 BLOCK artifact rather than this v2 artifact; §10 is the record the manager progresses on verdict, so the pointer is corrected in the normal progression act (rationale §6 carries the same stale pointer). |
| C14 | **CONFIRMED** | `ORCHESTRATION_PLAN.md` (version 1, posture MIXED, Selection authority HUMAN) names W1/PKG-04 T1 as exactly this combined tranche with one integration owner; execution rules prescribe the serialized author→verify→execute→verify chain, per-tranche commits, one PR per wave, HELP_HUMAN-only merge/receipt. The plan does not literally name the branch; the live `claude/piping-r14-pkg04-mechanics` at `6152908b3` is consistent with "one branch per package wave" (unchanged v1 caveat). |
| C15 (v2) | **CONFIRMED** (caveat V2-D5; vestige V2-D1) | Brief §6 records the single wave-level sweep explicitly under the named controlling HELP_HUMAN dispatch, quotes the plan's execution-rules sentence it refines (verified present in `ORCHESTRATION_PLAN.md`; quote omits the trailing "(code-touching)" — V2-D3), preserves DEC-025's pre-push gate semantics as independently verified against `docs/BUILD_AND_RELEASE.md` §5.1 ("required pre-push/fan-in evidence"; sweep at clean committed HEAD before push), and commits the refinement to the W1 manager return for HELP_HUMAN fan-in. Caveat: the dispatch itself is live parent-mediated instruction with no durable artifact in the tree; its existence cannot be independently confirmed here and is validated by its own author (HELP_HUMAN) at fan-in before any push — see V2-D5. |

**Tally: 15/15 CONFIRMED, 0 REFUTED** (C9, C13, C15 carry recorded minor
defects/caveats that do not refute their operative content).

## 3. Independent 10-Class Fast-Reject Re-Screen (D-52 §4.1, re-derived item by item against the amended brief)

1. **Irreducible owner preference / two defensible project-coherent outcomes remaining:** not hit. Design multiplicity is handled under D-54 reasoned selection with recorded rejected alternatives; the selected shape is groundable in live artifacts (the headless crate owns both the producer path and the DEL-08-04 payload-validation contract — verified in code, C4).
2. **Professional/safety/legal/fiduciary/residual-risk accountability:** not hit. The envelope vocabulary pins `HUMAN_REVIEW_REQUIRED` and the professional boundary, enforced by the validator the tranche must pass (C4); no reliance, hazard, or compliance outcome is created or implied.
3. **Conflict/contradiction ruling not determined by the authority chain:** not hit in v2. Both v1 conflicts are resolved inside the accepted chain: (a) the witness-currency conflict is gone — the brief now agrees with the R13-refreshed manual and the live tree; (b) the sweep-gating deviation is now attributed to the controlling parent dispatch refining the parent's own plan sentence, with DEC-025's accepted pre-push semantics preserved (verified) and the refinement surfaced to the plan's author at fan-in before any push. The §5.9 vestigial clause (V2-D1) does not reopen the determination: §6 of the same document states which arrangement controls, and both readings produce identical execution behavior.
4. **Scope/boundary change, new normative content, new acceptance criteria:** not hit. Both items are recorded deliverable Remaining scope; DEL-08-04 schema/validators consumed unchanged; no new tolerance/threshold/criterion (§3.6, §9); the §3.7 amendment narrows an evidence predicate to a truthful basis, creating no new normative content.
5. **Lifecycle/stage/issuance/release/acceptance/evidence-posture act:** not hit. Lifecycle stays `IN_PROGRESS`; only the two named Remaining rows are struck on success; PDU-035/DEC-046/threshold holds expressly preserved.
6. **Third-party/procurement/spending/publication/external action:** not hit. Offline, local-only; push/PR/merge/receipt reserved to later HELP_HUMAN/owner acts.
7. **Merge/integration authority over an accepted baseline, destructive action:** not hit. Ordinary local commits by the W1 manager after a fresh implementation verification; frozen witness surfaces read-only; nothing rewritten or deleted.
8. **Protected/private data exposure:** not hit. Invented fixtures only; F-PIP-1/F-PIP-4 undisturbed.
9. **Evidence unavailable / stale basis / claim beyond warrant:** not hit in v2. The v1 false premise is removed; every outcome-determining premise re-verified live (C1–C6, C8–C10, C12–C14) or spot-verified against the recorded v1 reproduction at this identical HEAD (C7). Residual descriptor inaccuracies (V2-D3, V2-D4) and the stale §10 pointer (V2-D2) are not outcome-determining. The live parent dispatch (V2-D5) is not an "unverifiable external fact" in the class-9 sense: it is in-hierarchy parent-mediated agency, lawful under the runtime doctrine, durably surfaced in this brief and in the manager return for validation by its own author at fan-in.
10. **Protected domain-engine paths, prover/tool activation, higher-order human boundaries:** not hit. No `_DomainEngines/**`, no prover, no accountable-party assignment.

**Re-screen result: PASS — no class hit on the amended brief.** The v1 hits
(classes 9 and 3) attached to brief text the amendment removed or attributed;
they do not survive re-derivation.

## 4. Residual Defects (none blocking)

- **V2-D1 — MINOR.** Brief §5 item 9 retains the v1 clause "…runs once at W1
  closeout per the campaign plan…", the attribution the v1 D2 flagged;
  contradicted and corrected by the same document's §6. No execution
  divergence is possible (the fence item's operative content — no sweep
  artifact written this tranche — is identical under either attribution).
  Recommend a one-clause editorial amendment ("per the controlling HELP_HUMAN
  W1 dispatch (§6)") at or before manager progression.
- **V2-D2 — MINOR (self-healing).** §10 `IndependentVerifier` still reads
  "PENDING — instances/W1/T1/VERIFY_BRIEF.md" (the completed, preserved v1
  BLOCK artifact); the pending v2 artifact is this file. §10 is the record the
  W1 manager progresses on verdict; the progression act should name both
  verifier artifacts and their verdicts. Rationale §6 carries the same stale
  pointer.
- **V2-D3 — MINOR.** §6's quotation of the plan's execution-rules sentence
  omits the trailing "(code-touching)" without ellipsis. Meaning unchanged
  (this tranche is code-touching, so the omitted qualifier could only
  strengthen the quoted sentence's applicability).
- **V2-D4 — MINOR.** §2 and rationale C9 label `TP-DAG-004-DEL-04-04-E009`
  "non-constraint"; its `DependencyType` is CONSTRAINT (target DEL-02-02, a
  non-root constraint). Row identity, `PENDING` status, live evidence, and
  no-resolution treatment are exact.
- **V2-D5 — INFORMATIONAL.** The "controlling HELP_HUMAN dispatch to the W1
  manager" quoted in §6 has no durable artifact in the tree (the R14 run
  directory holds only the plan, the W1/T1 instance files, and the W3
  return). Live parent-mediated dispatch is lawful under the runtime
  doctrine, and the refinement is durably recorded here and routed to
  HELP_HUMAN — the dispatch's own author — at fan-in before any push, with
  DEC-025's owner-level semantics independently preserved either way. The
  dispatch's existence therefore cannot be refuted or confirmed from durable
  state by this verifier; HELP_HUMAN's fan-in validation is the closing check.
- **V2-D6 — INFORMATIONAL.** §7's "any committed-witness byte drift" is best
  read as edits/drift of the frozen witness files themselves (per §4.3's
  do-not-touch instruction); the adverse reading (tp_runner_015 stdout vs
  committed witnesses) is expressly foreclosed by §3.7(b) ("NOT the
  comparison target"). A future editorial pass could say "any edit to a
  committed witness file" to remove the last ambiguity.

Everything else probed held on re-verification: fence coverage and exclusion
truthfulness (C8, incl. headless-local `Cargo.lock`, no `tests/` dirs, and the
still-open DEL-10-05 `export-results` Remaining bullet), no
normative-content smuggling in any predicate (C10), claim-calibrated language
throughout both artifacts (repo-wide `validate_claims_language.py` returns
VALID, 262 files scanned, DEC-081 taxonomy satisfied), attribution integrity
and governance-SHA ancestry (C13), DAG pointer `DAG-007`
`approved_active_graph_authority` (`execution/_DAG/_LATEST.md`), receipt
cursor `Receipt-60` (`loop/LOOP_RECEIPTS.md` line 1454), and both MEMORY.md
basis entries plus the cited 2026-07-12 run record present.

## 5. Verdict

The amendment cures both v1 blocking premises: the CLI-stability predicate is
now truthful and satisfiable (del1005 vs committed witnesses; tp_runner_015 vs
pre-tranche baseline captures at the base commit; no predicate anywhere
requires the historical tp_runner_015 witnesses to match at HEAD), and the
DEC-025 single wave-level sweep is truthfully recorded under the controlling
HELP_HUMAN dispatch with the refined plan sentence named and the pre-push gate
semantics preserved. The amendment record preserves the v1 BLOCK without
softening. All fifteen enumerated claims withstood independent refutation, the
independent ten-class re-screen finds no hit, and the residual defects
(V2-D1..D6) are minor or informational, none outcome-determining, none
authority-converting, and each durably surfaced here for the W1 manager and
HELP_HUMAN fan-in.

**VERDICT: COMMIT-SAFE**

Precisely: no acceptance predicate is unsatisfiable; no premise contradicts
the live tree or the accepted record; no fast-reject class is hit; owner and
HELP_HUMAN gates (PDU-035, DEC-046, thresholds, lifecycle, sweep-before-push,
fan-in, merge, receipt) remain intact downstream of this verdict. The W1
manager should apply the V2-D1 one-clause editorial correction and name both
verifier artifacts when progressing the §10 record; neither act is a
precondition of this verdict's validity.

No release-readiness, acceptance, review-disposition, or reliance conclusion
is expressed or implied by any statement in this return; all verdicts are
bounded to the refutation task defined by the parent brief.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
