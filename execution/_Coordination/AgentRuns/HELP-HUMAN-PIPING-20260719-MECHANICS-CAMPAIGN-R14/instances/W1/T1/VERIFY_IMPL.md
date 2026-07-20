# VERIFY_IMPL — T1 Implementation Verification (Fresh-Context Adversarial)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Verifier:** fresh-context implementation verifier (governed Agent 2, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Object under review:** the uncommitted T1 working-tree implementation on
branch `claude/piping-r14-pkg04-mechanics` at base
`6152908b3246df61150dc91e3558788b05dfb643`, as claimed by
`instances/W1/T1/EXECUTE_RETURN_V2.md` (claims E1–E18) against the v3
EFFECTIVE brief `CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001`
(§3 predicates, §5 fence, §6 checks)
**Date:** 2026-07-19

This return records an adversarial refutation attempt. Every check below was
re-run by this verifier from the live tree; no executor-produced evidence was
accepted without independent re-derivation, with the one trust boundary named
in §4 (pre-tranche baseline SHA provenance).

---

## 1. Independent Fence-Containment Derivation

Enumerated from `git status --porcelain` and
`git diff --stat 6152908b3` by this verifier (not taken from the executor's
containment JSON). Paths relative to `WORKING_ROOT`.

Tracked modifications (8):

| Path | Fence clause | Verdict |
|---|---|---|
| `core/runner/headless/Cargo.toml` | §5.2 | IN FENCE |
| `core/runner/headless/src/lib.rs` | §5.2 | IN FENCE |
| `core/product_physics/src/lib.rs` | §5.3 | IN FENCE |
| `core/solver/nonlinear_integration/src/lib.rs` | §5.4 | IN FENCE |
| `execution/PKG-04_…/DEL-04-02_Straight pipe element/_STATUS.md` | §5.6 | IN FENCE |
| `execution/PKG-04_…/DEL-04-02_Straight pipe element/MEMORY.md` | §5.6 | IN FENCE |
| `execution/PKG-04_…/DEL-04-04_Nonlinear support active-set solver/_STATUS.md` | §5.7 | IN FENCE |
| `execution/PKG-04_…/DEL-04-04_Nonlinear support active-set solver/MEMORY.md` | §5.7 | IN FENCE |

Untracked additions:

| Path | Fence clause | Verdict |
|---|---|---|
| `core/runner/headless/src/result_envelope_binding.rs` | §5.2 (new binding module under `src/`) | IN FENCE |
| `…/DEL-04-02_…/_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md` | §5.6 | IN FENCE |
| `…/DEL-04-04_…/_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md` | §5.7 | IN FENCE |
| `execution/_Coordination/AgentRuns/…R14/instances/W1/T1/**` (10 files) | §5.8 | IN FENCE |
| `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md` | §5.1 (brief status record; v3 progression) | IN FENCE |
| `execution/_Coordination/AgentRuns/…R14/ORCHESTRATION_PLAN.md` (mtime 18:03) | lawful pre-existing campaign state (parent-owned, pre-dates T1 execution) | SET ASIDE |
| `execution/_Coordination/AgentRuns/…R14/instances/W3/RETURN.md` (mtime 18:06) | lawful pre-existing campaign state (W3 read-only assessor return, pre-dates T1 execution) | SET ASIDE |
| `CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md` (mtime 19:17:14) | manager-owned concurrent T2 authoring (header: `Prepared by: WORKING_ITEMS`, status `HELD`) | SET ASIDE |
| `CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md` (mtime 19:20:45) | manager-owned concurrent T3 authoring (same header form, status `HELD`) | SET ASIDE |
| `…R14/instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md`, `…/W1/T3/CURRENT_CANDIDATE_RATIONALE.md` | manager-owned concurrent rationale artifacts | SET ASIDE |

Manager-owned-state plausibility: the T2/T3 briefs carry
`doc_kind: coordination.candidate_brief`, `status: proposed_effect_held`,
`Prepared by: WORKING_ITEMS (W1, PKG-04 package manager)`, and
`HELD — PENDING FRESH-CONTEXT ADVERSARIAL VERIFICATION` — the manager's
authoring form, not an execution write; their mtimes (19:17/19:20) sit inside
the manager's authoring window during T1 execution (executor implementation
writes span 19:20:29–19:32). Consistent with executor claim E1 and the
executor's own concurrent-state observation. Set aside per the parent
dispatch.

**No changed or untracked path lies outside the §5 fence for the executor's
write set; no fence-external tracked file has any diff.** Explicit negative
checks (all git-clean): `core/reporting/**` (incl. all of `result_export`),
`core/runner/headless/Cargo.lock` (the lock graph already carried
`open_pipe_stress_result_export`; verified present at base),
`core/product_physics/Cargo.toml`/`Cargo.lock`,
`schemas/**`, `tests/**`, `validation/**`, `docs/**`, `loop/**`. HEAD is
`6152908b3` (no commit was made); branch is
`claude/piping-r14-pkg04-mechanics`.

## 2. Independently Re-Run Checks (§6)

| # | Check (re-run by this verifier) | Result |
|---|---|---|
| 1 | `cargo fmt --check` core/runner/headless | PASS |
| 2 | `cargo fmt --check` core/product_physics | PASS |
| 3 | `cargo fmt --check` core/solver/nonlinear_integration | PASS |
| 4 | `cargo test --offline` core/runner/headless | PASS: 23 lib tests (7 in `result_envelope_binding`) + 1 `headless_preview_runner` + 15 bin tests; 0 failed — matches executor counts exactly |
| 5 | `cargo test --offline` core/product_physics | PASS: 75 tests, 0 failed (incl. `nonlinear_context_passthrough_tests`) |
| 6 | `cargo test --offline` core/solver/nonlinear_integration | PASS: 17 tests, 0 failed (incl. `component_identity_tests`) |
| 7 | `cargo test --offline` core/reporting/result_export | PASS: 12 tests, 0 failed; crate verified byte-unchanged via git (no diff, no status entry) |
| 8 | `python3 tests/test_headless_runner_contract.py` (WORKING_ROOT) | PASS (exit 0) |
| 9 | CLI-stability empirical (see §3) | PASS: 8/8 byte-identical |
| 10 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS: VALID, 262 files, DEC-081 taxonomy satisfied |
| 11 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | PASS: 648 surfaces, no literal home-dir absolute paths |
| 12 | `git diff --check` (REPO_ROOT) | PASS |
| 13 | JSON parse of `CHANGE_SCOPE_CONTAINMENT_V2.json` | PASS (`status: PASS`, `violations: []`) |

All cargo invocations ran with `CARGO_NET_OFFLINE=true` and `--offline`; no
network access, no dependency installation, no toolchain update.

## 3. CLI-Stability Empirical Results (independent build and run)

Runner built offline from the live implementation tree
(`cargo build --offline … --bin openpipestress-runner`); command shapes per
`docs/validation_manual/headless_runner_reproduction.md`; stdout captured to
ephemeral scratch outside durable project paths.

Five del1005 cases, stdout `cmp` byte-for-byte against the five committed
`validation/witness/generated/del1005_payload_binding_*.json` witnesses:

| Case | Exit | vs committed witness |
|---|---|---|
| benchmark_single_case | 0 | BYTE-IDENTICAL |
| benchmark_multi_case | 0 | BYTE-IDENTICAL |
| regression_full_suite | 0 | BYTE-IDENTICAL |
| benchmark_payload_missing | 1 | BYTE-IDENTICAL |
| regression_payload_missing | 1 | BYTE-IDENTICAL |

Three tp_runner_015 cases, stdout SHA-256 against the pre-tranche baselines
recorded in `EXECUTE_RETURN.md` §6 (recorded 18:55, before the first
implementation write at 19:20:29; the committed tp witnesses are historical
and are not the comparison target, per brief §3.7(b)):

| Case | Exit | Observed SHA-256 | vs baseline |
|---|---|---|---|
| solve | 0 | `738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614` | EQUAL |
| validation_blocking | 1 | `5fb2f8a9c8264be665581d58fb55e342a96636a87dc23f7c0db743c3b31a5505` | EQUAL |
| benchmark_stub | 1 | `9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4` | EQUAL |

Exit codes 0/0/0/1/1 and 0/1/1 match the recorded expectations.

## 4. Per-Claim Refutation Table (E1–E18)

| Claim | Verdict | Independent evidence |
|---|---|---|
| E1 (clean start; T2/T3 concurrent, not executor writes) | CONFIRMED | Tracked tree diffs only at the 8 fence paths; ORCHESTRATION_PLAN.md/W3 return pre-date execution (18:03/18:06); T2/T3 briefs mtime 19:17/19:20 with manager authoring headers and `HELD` status; executor implementation writes 19:20:29–19:32. |
| E2 (16 durable writes, all in fence) | CONFIRMED | §1 derivation above (independent enumeration); `CHANGE_SCOPE_CONTAINMENT_V2.json` re-parsed: PASS, 0 violations, 16 allowed paths matching §5. |
| E3 (public producer, exact wrapper, export-crate vocabulary only) | CONFIRMED | Read `result_envelope_binding.rs` in full: `pub fn build_result_export_document(&RunnerRequest, &RunnerResult, &MechanicsEnvelope) -> Result<Value, Diagnostic>`; document built via `export::result_export_document(&envelope)`; only `open_pipe_stress_result_export` public types constructed; `deliverable_id` asserted `"DEL-08-04"` in test (re-run, passing). |
| E4 (envelope_id == result_envelope_ref ref_id) | CONFIRMED | Source: `envelope_id` cloned from `runner_result.result_envelope_ref.envelope_ref.ref_id`; asserted in `nonlinear_bearing_solve_attaches_validated_envelope_with_context` (re-run, passing). |
| E5 (exported + disclosed == result-row count, both fixtures) | CONFIRMED | Asserted in both fixture tests (`values.len() + disclosures.len() == mechanics.results.len()`); re-run, passing. |
| E6 (no blocking disclosure/assumption/limitation; disclosure only on the envelope-document surface) | CONFIRMED | Source: disclosure severity `Info`, assumption `Info`, limitation `Warning`; disclosure diagnostics pushed only into the envelope's own `diagnostics` vector, never `runner_result.diagnostics` (which gains entries only on the `Err` structural path); tests assert severities; empirically zero exit-code/stdout effect on all eight pinned cases (§3). This also discharges the V3-D6 placement confirmation the brief verifier requested. |
| E7 (no semantically false table entry; stiffness/energy/count/state absent) | CONFIRMED | Table read against `docs/SPEC.md` §4 (D-01: `stiffness` must classify as `linear_stiffness`/`rotational_stiffness`; neither exists in DEL-08-04 `DimensionId`, re-read: Dimensionless/Length/Angle/Force/Moment/Stress/Area/SectionModulus/SecondMomentArea/Ratio/Time/Temperature/Pressure/Tbd). All table entries are dimension-truthful (mm→Length, rad→Angle, N→Force, N*m→Moment, MPa→Stress); the `N*m` work residual, stiffness echoes, count/flag/state/mode rows have no entry; table is unit-exact (`("element_local_axial_force","kN")→None` asserted). In-dimension judgment rows (e.g. `nonlinear_support_final_reaction`→Reaction, force-valued review echoes→Force) sit inside the V3-D7 bounded-discretion envelope. |
| E8 (diagnostic source preserved in provenance) | CONFIRMED | `export_mechanics_diagnostic` maps `diagnostic.source` into both the source `Reference` and `provenance.source_name` (fallback `core/product_physics`); every mechanics diagnostic mapped unconditionally before row export. |
| E9 (nonlinear context present iff nonlinear-bearing) | CONFIRMED | `nonlinear_exercised` gate (result-kind/diagnostic-code prefixes); both directions asserted (`nonlinear_bearing…with_context`, `linear_only…without_nonlinear_context` incl. absence of `nonlinear_component=` in build ref); 7 assumptions / 6 limitations counts asserted equal to the pass-through context; provenance `source_name` asserted equal to the component crate name. |
| E10 (crate-constant identities, no hardcoded duplicates) | CONFIRMED | Diffs show `env!("CARGO_PKG_NAME")`/`env!("CARGO_PKG_VERSION")` accessors in both crates; `nonlinear_assembled_loop_context()` is a pure pass-through (asserted by the product_physics test); the binding module consumes only the accessors. |
| E11 (serde-skipped field; CliOutput shape unchanged) | CONFIRMED | `PreviewRunnerOutput` derives `Debug, Clone, Serialize` only; new field `#[serde(skip_serializing)]`; serialization-exclusion test re-run, passing; no reference to `result_envelope_document` anywhere under `src/bin/`; bin file, schema, and contract test all git-clean; contract test re-run PASS. |
| E12 (structural failure → blocking `HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED`) | CONFIRMED | `production_failed` uses `Diagnostic::runner_blocking`; appended to `runner_result.diagnostics` in `attach_result_envelope_document`; asserted for envelope-ref mismatch and missing envelope checksum (tests re-run, passing); the unchanged DEC-065 bin exit logic then reports not-clean. |
| E13 (eight pinned cases byte-identical) | CONFIRMED | Independently rebuilt and re-run: §3 (5/5 witness-identical, 3/3 baseline-SHA-equal, exit codes exact). Trust boundary: the three baselines are the executor's pre-change captures; their SHAs were durably recorded in the v1 BLOCKED return (18:55) before any implementation write (first code mtime 19:20:29), and the five witness-anchored cases independently demonstrate an unchanged CLI surface. |
| E14 (test suites + contract pass; result_export unchanged) | CONFIRMED | §2 rows 4–8: counts match the executor's exactly (23+1+15 / 75 / 17 / 12); `core/reporting/**` has no git diff and no status entry. |
| E15 (state edits exactly §3.8) | CONFIRMED | `git diff 6152908b3` on both `_STATUS.md`: DEL-04-02 sole Remaining line removed (section now empty), exactly one new History line, `Last Updated` 2026-07-19; DEL-04-04 Remaining item 1 line removed with items 2–6 appearing only as unchanged context lines (byte-identical by diff construction), exactly one new History line; each MEMORY.md gains exactly one newest-first entry; exactly one new run record per folder (verified by directory listing; the two records byte-identical to each other). No new Remaining row anywhere in the diff. |
| E16 (no fence-external writes; no git/network actions) | CONFIRMED | §1 negative checks; HEAD unchanged at `6152908b3`; no new commit, branch, or push (branch state inspected). |
| E17 (additive-only crate edits; no MechanicsEnvelope shape change) | CONFIRMED | Full diffs read: product_physics is a pure append (accessors + struct + pass-through + test) after `round6`; nonlinear_integration is a pure append (two accessors + test module); no existing signature, body, or serialized-shape line touched. Headless `lib.rs` non-additive lines are exactly `let runner_result` → `let mut runner_result` and the two `clone()` calls on `privacy`/`provenance` (behavior-equivalent; empirically byte-stable per §3). |
| E18 (follow-on recorded; no new Remaining rows) | CONFIRMED | Vocabulary-extension follow-on present in both History entries, both MEMORY entries, both run records (grep-verified), and EXECUTE_RETURN_V2 §7; no `_STATUS.md` gained any Remaining line. |

**Tally: 18/18 CONFIRMED; 0 REFUTED.**

## 5. Defects

No blocking or outcome-material defect found. Observations (informational,
severity none/minor, none requiring re-execution):

- **VI-O1 (informational).** The vocabulary-boundary disclosure reuses
  `DiagnosticClass::UnitWarning` rather than a dedicated class (the closed
  DEL-08-04 class enum offers no better fit without a fence-external crate
  edit). The dedicated code string
  `HEADLESS_RUNNER_ENVELOPE_VOCABULARY_BOUNDARY_ROW` disambiguates; bounded
  judgment consistent with brief §3.2's no-new-identifier rule.
- **VI-O2 (informational).** E13's tp_runner_015 leg rests on baseline
  captures made by the executor at the base commit; independence is partial
  by design (the parent dispatch pins these as the comparison target, and
  their SHAs were durably recorded before any implementation write). The
  witness-anchored del1005 leg is fully independent.
- **VI-O3 (informational).** The `MECHANICS_SOLVED` production gate
  (attach/append nothing for incomplete solves) is a bounded judgment the
  brief text does not spell out; it is recorded in EXECUTE_RETURN_V2 §2/§6,
  preserves pre-existing not-clean signaling byte-for-byte (unit-tested), and
  is the only reading consistent with §3.7 byte-stability.

## 6. Verdict

All eighteen executor claims withstood refutation on the live tree; the
independently derived write set is fence-contained with the manager's
concurrent T2/T3 authoring state correctly excluded from the executor's
containment accounting; every §6 check re-ran green under this verifier's own
invocations; and the CLI surface is empirically byte-stable across all eight
pinned cases at the implementation head. No acceptance predicate of the v3
brief is unmet; no preserved gate (PDU-035, DEC-046, thresholds, lifecycle,
DEL-10-05 export-results, sweep-before-push) is touched.

**VERDICT: COMMIT-SAFE**

This return is development verification and screening evidence for one
bounded tranche; it expresses no run-quality, review-disposition,
release-readiness, or reliance conclusion.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
