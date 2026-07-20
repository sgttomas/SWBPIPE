# VERIFY_BRIEF — Fresh-Context Adversarial Verification of CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Verifier:** fresh-context adversarial verifier (governed Agent 2, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Objects reviewed:**
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CURRENT_CANDIDATE_RATIONALE.md`

**Verified against:** live tree at HEAD `6152908b3246df61150dc91e3558788b05dfb643`,
branch `claude/piping-r14-pkg04-mechanics` (confirmed by `git rev-parse HEAD` /
`git branch --show-current`). Working tree clean except the lawful pre-existing
state named in the brief (untracked R14 AgentRuns directory and the candidate
brief itself). All checks offline; no install, fetch, or network use. No file
was modified other than this return.

---

## 1. Per-Claim Verdict Table (C1–C14)

| Claim | Verdict | Evidence (file / line, live tree) |
|---|---|---|
| C1 | **CONFIRMED** | `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/_STATUS.md` line 3 (`IN_PROGRESS`), lines 6–7: `## Remaining` contains exactly one item whose text matches the brief's selected item 1 verbatim. |
| C2 | **CONFIRMED** | `.../DEL-04-04_Nonlinear support active-set solver/_STATUS.md` line 3 (`IN_PROGRESS`), line 7: item 1 matches the brief's selected item 2 verbatim. Items 2–6 (lines 8–12): PDU-035 formal REVIEW disposition (owner-gated), friction path-history "(gated: new D-XX ruling…)", and three threshold-promotion rows citing DEC-046 / PRD D6/D9 / DEC-052/DEC-054 (one expressly "stage-gated: R5 release evidence"). The brief excludes all five (§9). |
| C3 | **CONFIRMED** | `grep -rn "ResultEnvelope {" core --include="*.rs"` finds constructors only in `core/reporting/result_export/src/lib.rs` lines 941/1000/1226 — all inside `#[cfg(test)] mod tests` (begins line 861–862) — and in the test file `core/reporting/report_package/tests/container.rs` line 155. `core/reporting/report_package/src/lib.rs` line 117 only consumes `&[ResultEnvelope]`; `core/runner/headless/src/benchmark_binding.rs` has no reference. No non-test constructor exists at HEAD. |
| C4 | **CONFIRMED** | `core/runner/headless/src/lib.rs` lines 563–667: `validate_result_with_optional_envelope_payload` → `validate_result_envelope_payload` checks `deliverable_id == "DEL-08-04"` (line 590), envelope-id match (lines 607–614), `HUMAN_REVIEW_REQUIRED` (616–631), non-empty `result_sets` (633–644), `deterministic_ordering` (646–658), checksum presence (660–666). Callers passing `Some(...)` are only at lines 1126/1149, both inside the test module (`#[cfg(test)]` at line 907); no caller in `src/bin/`. |
| C5 | **CONFIRMED** | `core/solver/nonlinear_integration/src/lib.rs` lines 303–313: `NonlinearFrameSolveResult` has public `assumptions: Vec<String>` and `limitations: Vec<String>` (populated at 497–498 from `assembled_loop_assumptions()`/`assembled_loop_limitations()`, defined 506/518); crate version `0.1.0` (`Cargo.toml` line 3). `grep -n "assumptions\|limitations" core/product_physics/src/lib.rs` returns zero hits — no consumer; `append_nonlinear_support_loop_results` (line 1540) maps `solve.diagnostics` (1602–1608) and emits iteration/residual rows but never reads the two vectors. |
| C6 | **CONFIRMED** | `core/product_physics/src/lib.rs` lines 590–601: `MechanicsEnvelope` carries `results: Vec<ResultItem>` and `diagnostics: Vec<Diagnostic>`; `Diagnostic` (lines 656–664) has `source: Option<String>`. Straight-pipe elements build the solve (`StraightPipeElement` at lines 43, 3022, 3039) and populate result rows; the brief maps rather than recomputes. |
| C7 | **REFUTED (partially — decisive)** | The eight committed witness files exist (`validation/witness/inputs/`, `validation/witness/generated/`, git-clean), but the three `tp_runner_015` witnesses are **not current for HEAD**. Demonstrated by offline build (`CARGO_NET_OFFLINE=true cargo build --offline`, cargo 1.92.0) and running all eight documented commands, diffing both stdout and `--output` bytes against the committed witnesses: the five `del1005_payload_binding_*` cases are **byte-identical**; all three `tp_runner_015_final_cli_*` cases **differ**. (a) `..._solve`: live stdout contains additional `result:nonlinear-support:max-*-delta` result refs/rows absent from the witness; (b) `..._validation_blocking`: live `CliOutput` serializes a `"suite_run": null` field (post-PR-#287 shape) absent from the witness; (c) `..._benchmark_stub`: live code emits `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (`core/runner/headless/src/bin/openpipestress-runner.rs` lines 299, 306–316) while the committed witness records `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`. The project's own R13-refreshed manual (`docs/validation_manual/headless_runner_reproduction.md`, table row for the benchmark stub and the dated note) records that the committed tp_runner_015 witnesses "remain truthful for their pinned pre-#287 commits" — i.e., historically pinned, not HEAD-current. Consequence: the brief's §2 currency statement is false for 3/8 cases, and the §3.7 predicate / §4.3 task / §6 witness diff ("all byte-identical") is unsatisfiable at the implementation head regardless of implementation quality; under §7 fail-closed the tranche is guaranteed to fail. |
| C8 | **CONFIRMED** | Every §4 write target is inside the §5 fence: headless `Cargo.toml`/`Cargo.lock`/`src/lib.rs`/new module (item 2; the crate has no external `tests/` dir — tests live in `src/lib.rs`), product_physics additive (item 3), nonlinear_integration accessor (item 4), conditional schema/contract-test (item 5), both deliverable folders (items 6–7), instance dir incl. `CHANGE_SCOPE_CONTAINMENT.json` (item 8). Existing headless deps already include `product_physics` (`core/runner/headless/Cargo.toml` line 23); the new `result_export` path dep is a local, offline-resolvable addition. Excluded surfaces genuinely not required: `result_export` public API suffices (`ResultEnvelope`, `validate_result_envelope`, `result_export_document` all `pub`); no `validation/**`, `docs/**`, or receipts write is needed for the §4 tasks. No unlawful permission found. (`core/runner/headless/src/bin/headless_preview_runner.rs` is outside the fence and not required — acceptable.) |
| C9 | **CONFIRMED** | `DEL-04-02/Dependencies.csv` (csv-parsed): eight EXECUTION UPSTREAM rows `DAG-002-E0105..E0109, E0432..E0434` all `SATISFIED`. `DEL-04-04/Dependencies.csv`: five root-constraint rows `TP-DAG-004-DEL-04-04-E001..E005` (targets DEL-00-01/02/03/06/08) `SATISFIED`; four non-constraint rows `E006..E009` `PENDING` targeting DEL-04-03, DEL-04-01, DEL-04-06, DEL-02-02. Committed implementation evidence for each target exists live (`core/solver/linear_supports` + `core/solver/nonlinear_supports`, `core/solver/frame_kernel`, `core/solver/diagnostics`, `core/units`) and is consumed read-only. The brief names the PENDING posture (§2) and resolves/edits no row — truthfully recorded. |
| C10 | **CONFIRMED** | No predicate or task introduces a tolerance constant, threshold, acceptance criterion, or release/CI vocabulary; new diagnostics follow the existing fail-closed `HEADLESS_RUNNER_*` pattern; solver-version content is crate-constant-derived. §9/§10 expressly preserve PDU-035, DEC-046, and every threshold hold; §3.8 strikes only the two named Remaining rows, lifecycle stays `IN_PROGRESS`; no lifecycle/stage/release act; no external action (no push/PR/network per §5). |
| C11 | **REFUTED** | The rationale's screen wrongly passes class 9 (D-52 §4.1) / class 10 (D-54 §3.1). "Every outcome-determining premise was verified in the live tree at preparation (… witness currency)" is a claim stronger than its warrant: witness currency for the three tp_runner_015 cases is demonstrably false at HEAD (see C7). The independent re-screen below records class 9 as HIT (stale basis / unsatisfiable predicate) and class 3 as HIT-by-ambiguity (sweep-gating contradiction, defect D2). Per D-52 §4.1, ambiguity is a hit; a hit returns the matter without four-lens balancing. |
| C12 | **CONFIRMED (narrowly)** | The §6 plan is executable offline in this worktree: cargo 1.92.0 confirmed; the headless crate builds with `CARGO_NET_OFFLINE=true --offline`; python3 3.13.7 present; `tests/test_headless_runner_contract.py`, `schemas/headless_runner.schema.yaml`, `software-workflow.json`, and root tools `tools/software_workflow/run_registered_checks.py`, `tools/software_workflow/validate_change_scope.py`, `tools/validation/validate_claims_language.py`, `tools/validation/validate_path_anchors.py` all exist at the stated paths. Caveat: executable is not passable — the §4.3/§6 witness diff step deterministically fails at HEAD (see C7/D1). |
| C13 | **CONFIRMED** | Brief §10 keeps `OwnerStandingApproval` (DEC-085/D-52 §2, SHA-bound at `f14fa775…`, verified an ancestor of HEAD via `git merge-base --is-ancestor`) distinct from `AgentClassification: CLASSIFY_ELIGIBLE` and `AgentJudgment: SELECT_AND_ADVANCE`; records `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL`, `OwnerCaseSelection: NONE`, `EffectStatus: HELD`, `IndependentVerifier: PENDING`, rejected alternatives, and names the rationale artifact. Form matches D-52 §4.5 extended by D-54 §3.3. D-54 (`DEC-087`) is durably committed (`8825065d5`). |
| C14 | **CONFIRMED (with recorded caveat)** | `ORCHESTRATION_PLAN.md` (plan version 1, posture MIXED, HUMAN selection authority) names W1/PKG-04 T1 as this exact combined tranche with one integration owner, prescribes the serialized author→verify→execute→verify chain, per-tranche commits, one PR per wave, and reserves merge/receipt to HELP_HUMAN. The plan does not literally name the branch; the live checked-out wave branch `claude/piping-r14-pkg04-mechanics` at `6152908b3` is consistent with "one branch per package wave". Caveat: the plan's commit-gating sentence conflicts with the brief's sweep deferral — recorded as defect D2, not as a refutation of C14's authorization statement. |

**Tally: 12/14 CONFIRMED, 2 REFUTED (C7, C11).**

## 2. Independent 10-Class Fast-Reject Re-Screen (D-52 §4.1, re-derived item by item; D-54 §3.1 refinement applied)

1. **Irreducible owner preference / choice not project-groundable:** not hit. Design-shape multiplicity is lawfully handled under D-54 reasoned selection; the selected shape is groundable in live artifacts (headless crate owns the producer path and payload-validation contract).
2. **Professional/safety/legal/fiduciary/residual-risk accountability:** not hit. The binding produces an envelope whose vocabulary pins `HUMAN_REVIEW_REQUIRED` (enforced by the validator, `core/runner/headless/src/lib.rs` lines 616–631); no reliance, hazard, or compliance outcome is created.
3. **Conflict/contradiction ruling not determined by the authority chain:** **HIT (by ambiguity).** Two accepted-record conflicts exist on this surface: (a) the brief's §5.9/§6 statement that "the single wave-level DEC-025 sweep runs once at W1 closeout **per the campaign plan**" contradicts the plan's execution rule that per-tranche commits are "each gated by the tranche's full check set **including the DEC-025 evidence sweep** (code-touching)" (`ORCHESTRATION_PLAN.md`, Execution rules); (b) the brief's §2 witness-currency statement conflicts with the R13-refreshed reproduction manual's own record that the tp_runner_015 witnesses are truthful only "for their pinned pre-#287 commits". Neither conflict is resolved by the authority chain inside the brief. Ambiguity is a hit.
4. **Scope/boundary change, new normative content, new acceptance criteria:** not hit. Both items are recorded deliverable Remaining scope; DEL-08-04 schema/validators consumed unchanged; no new tolerance/threshold/criterion (§3.6, §9).
5. **Lifecycle/stage/issuance/release/acceptance/evidence-posture act:** not hit. Lifecycle stays `IN_PROGRESS`; only Remaining rows are struck; PDU-035/DEC-046/threshold holds expressly preserved.
6. **Third-party/procurement/spending/publication/external action:** not hit. Offline, local-only; no push/PR/merge/receipt at this level; network expressly unauthorized.
7. **Merge/integration authority over an accepted baseline, destructive action:** not hit. Ordinary commits on the wave branch by the W1 manager after implementation verification; frozen witness surfaces are read-only under the fence (no witness edit is authorized — correctly).
8. **Protected/private data exposure:** not hit. Invented fixtures only; F-PIP-1/F-PIP-4 undisturbed.
9. **Evidence unavailable / stale basis / claim beyond warrant:** **HIT.** The brief's outcome-determining premise that all eight committed witnesses are "current for HEAD" (§2) is false for the three tp_runner_015 cases, demonstrated by direct offline reproduction at HEAD (see C7). The §3.7 acceptance predicate is therefore unsatisfiable as written, and the rationale's class-9 pass ("witness currency … verified in the live tree") is a claim stronger than its warrant.
10. **Protected domain-engine paths, prover/tool activation, higher-order human boundaries:** not hit. No `_DomainEngines/**`, no prover, no accountable-party assignment.

**Re-screen result: FAIL — classes 9 and 3 are hit.** Under D-52 §4.1 the matter returns without four-lens balancing; under D-54 §3.1 the same boundary (class 10 stale evidence / claim beyond warrant) applies.

## 3. Additional Defects Found

- **D1 — BLOCKING. Witness byte-stability predicate contradicts the live tree.**
  Brief §2 asserts all eight committed witnesses are current for HEAD; §3.7/§4.3/§6
  require stdout byte-identity for all eight at the implementation head. At HEAD
  `6152908b3`, before any tranche change, three of eight already fail
  (solve: extra nonlinear-support delta rows; validation_blocking: new
  `"suite_run": null` field; benchmark_stub: changed diagnostic code — full
  evidence under C7). A perfect implementation still fails §6; §7 fail-closed
  then guarantees a failed tranche, and the §5 fence (correctly) forbids the
  only in-tranche "fixes" (editing witnesses or docs). The brief needs a
  superseding amendment that scopes the byte-stability predicate truthfully
  (e.g., to the five del1005 witnesses plus a no-new-drift formulation for the
  three historically pinned tp_runner_015 surfaces, consistent with
  `docs/validation_manual/headless_runner_reproduction.md`), which is a
  brief-authoring act, not an executor interpretation.
- **D2 — MODERATE (blocking under ambiguity-is-a-hit). Evidence-sweep gating
  misattributed to the campaign plan.** Brief §5.9/§6 defer the DEC-025
  evidence sweep to a single W1-closeout run "per the campaign plan"; the
  plan's execution rules state per-tranche commits are each gated by the full
  check set including the DEC-025 evidence sweep for code-touching work. The
  deviation may be defensible under DEC-025's pre-push semantics, but the
  brief presents it as the plan's rule, which it is not; the discrepancy needs
  an explicit ruling or plan/brief amendment before effect.
- **D3 — MINOR (informational).** The campaign plan does not name the wave
  branch; the live branch `claude/piping-r14-pkg04-mechanics` is consistent
  with "one branch per package wave". No action required beyond noting it.
- **D4 — MINOR (informational).** `core/runner/headless/src/bin/headless_preview_runner.rs`
  exists alongside the fenced `openpipestress-runner.rs` and is outside the §5
  fence; the §4 tasks do not require it. No defect; recorded for completeness.

Everything else probed held: write-fence coverage and containment (C8), no
normative-content smuggling in the acceptance predicates other than D1's false
premise (C10), claim-calibrated language throughout both artifacts with no
release-readiness/acceptance claims, DEL-10-05's open `export-results` scope
truthfully deferred (its `_STATUS.md` Remaining still names the export-results
binding), DEL-04-04 PENDING dependency rows truthfully recorded (C9), and §10
attribution integrity (C13).

## 4. Verdict

Because claim C7 and claim C11 are refuted, the independent fast-reject
re-screen hits classes 9 and 3, and defect D1 makes the brief's acceptance
predicates unsatisfiable at the implementation head:

**VERDICT: BLOCK**

Blocking reasons, precisely:
1. Brief §2's premise that the three tp_runner_015 committed witnesses are
   current for HEAD is false (demonstrated by offline reproduction at HEAD);
   the §3.7 byte-stability predicate and the §4.3/§6 witness diff are
   therefore unsatisfiable as written, guaranteeing tranche failure under §7
   (defect D1; refuted C7; fast-reject class 9 hit).
2. The rationale's class-9 screen pass rests on that false premise (refuted
   C11), so the D-52/D-54 eligibility classification does not stand as
   recorded.
3. The brief's DEC-025 evidence-sweep deferral is misattributed to the
   campaign plan and contradicts the plan's per-tranche commit-gating text
   (defect D2; fast-reject class 3 ambiguity hit).

No release-readiness, acceptance, review, or reliance conclusion is expressed
or implied by any statement in this return; all verdicts are bounded to the
refutation task defined by the parent brief.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
