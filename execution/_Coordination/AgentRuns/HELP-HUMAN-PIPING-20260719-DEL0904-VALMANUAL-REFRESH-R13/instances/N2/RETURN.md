# N2 TERMINAL RETURN — Independent Adversarial Verification of CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N2
**Role:** Fresh-context independent adversarial verifier (governed Agent 2,
non-delegating, read-only except this file)
**Date:** 2026-07-19
**Verification basis:** live tree at HEAD
`45ec0524d3b0c155392553a3b3e4190534ff0fe8`, branch
`claude/piping-r13-valmanual-refresh`; working tree clean except this run's
lawful pre-existing state (the R13 AgentRuns directory and the candidate
brief, both untracked — confirmed by `git status --porcelain`).

**Method:** default-refute. Every N1 claim was re-derived from its cited live
source: the runner bin and binding module were read at the cited lines; all
five `del1005_payload_binding_*` generated witnesses and all five input
fixtures were parsed with python3; the frozen `tp_runner_015` stub input and
its committed witness were parsed; the manual page, `_STATUS.md`, DEL-10-05
`_STATUS.md`, receipts tail, DAG pointer and CSV rows, `software-workflow.json`,
the R12 N3/N4 returns and HANDOFF_STATE, the clean-repro brief §8, the R11 run
record, and the completed bundle directory were read directly. No file other
than this return was written; no cargo run; no commit.

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Per-claim verdicts (20 claims)

| # | Claim (short) | Verdict | Evidence found |
|---|---|---|---|
| 1 | Bound verbs route through `execute_suite_verb` | **CONFIRMED** | Bin lines 254–260 (`RunBenchmark` → `execute_suite_verb(..., input.benchmark, ...)`) and 261–267 (`RunRegression` → `input.regression`); no stub diagnostic in either arm. |
| 2 | Stub confined to export-results | **CONFIRMED** | `grep -n` returns exactly three hits: line 270 (`ExportResults` arm) and lines 657/671 (test assertions; 657 in the narrowed `downstream_operation_verbs_are_stable_but_stubbed` test targeting `export-results`, 671 a negative assertion in the run-benchmark missing-payload test). |
| 3 | Payload-missing codes | **CONFIRMED** | Bin lines 298–302: `RunBenchmark` → `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`, `RunRegression` → `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; missing payload returns exit 1 (lines 306–318). |
| 4 | Blocked-case code family + report fields | **CONFIRMED** | `benchmark_binding.rs` line 312 formats `HEADLESS_RUNNER_{verb_token}_CASE_COMPARISON_BASIS_NOT_REUSABLE`; `SuiteRunReport` fields `claim_posture` (line 108) and `whole_suite_default_applied` (line 110). |
| 5 | Witness: benchmark single case | **CONFIRMED** (note A) | Parsed: `command: run-benchmark`, suite `mechanics`, `suite_deliverable: DEL-09-01`, `requested_case_count: 1`, case `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched`, no diagnostics. |
| 6 | Witness: benchmark multi case | **CONFIRMED** (note A) | Parsed: suite `stress`, `DEL-09-02`, the three named cases, 3/3 `executed_and_matched`, no diagnostics. |
| 7 | Witness: regression full suite | **CONFIRMED** (note A) | Parsed: `command: run-regression`, suite `nonlinear`, `DEL-09-03`, `whole_suite_default_applied: true`, 5/5 `executed_and_matched` over the five named NL cases, no diagnostics; input `regression = {"suite": "nonlinear"}` with no `cases` key. |
| 8 | Witness: payload-missing pair | **CONFIRMED** | Parsed all four JSONs: each generated witness carries exactly one blocking code (`..._BENCHMARK_PAYLOAD_MISSING` / `..._REGRESSION_PAYLOAD_MISSING`); each input contains only a top-level `request` key. |
| 9 | Exit codes 0/0/0/1/1 | **CONFIRMED** | R12 N3 RETURN §3 row 6 records exactly `0, 0, 0, 1, 1` in the stated order; R12 N4 `RETURN_V2.md` terminal line `VERDICT: COMMIT-SAFE` (v1 BLOCK preserved). Consistent with the bin exit policy read directly (exit 0 only with no blocking diagnostics). |
| 10 | Fail-closed whole-suite facts | **CONFIRMED** | R12 N3 RETURN §2 tail: "mechanics 11/21 executed_and_matched + 10 blocked; stress 12/15 + 3 blocked; nonlinear 5/5 executed_and_matched", labeled regression evidence; same figures in the DEL-10-05 `_STATUS.md` History entry. No threshold stated anywhere. |
| 11 | Frozen stub input is payload-less | **CONFIRMED** | Parsed `tp_runner_015_final_cli_benchmark_stub_input.json`: top-level keys `['request']` only, `request.operation: run_benchmark`; committed witness diagnostics exactly `['HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD']`. With claims 1–3, on post-#287 sources the frozen command takes the payload-missing path (exit 1, `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`). |
| 12 | Current page stale rows | **CONFIRMED** | Line 43: Fixture Set row expecting the stub diagnostic from `run-benchmark`; line 59: the `run-benchmark` reproduction command; lines 95–99: Remaining E2 paragraph with "`run-benchmark` / `run-regression` remain structured stubs" spanning lines 96–97 and "a clean environment demonstration record" (lines 97–98) still listed although R11/Receipt-58 landed it. |
| 13 | DEL-09-04 Remaining text | **CONFIRMED** | `_STATUS.md` line 3 `Current State: IN_PROGRESS`; exactly two `## Remaining` bullets (lines 7–8); first-bullet text matches the quoted string byte-for-byte through "...GUI-workflow validation evidence open;"; second bullet is the DEC-046 owner-gated tolerance promotion. |
| 14 | Bindings actually landed | **CONFIRMED** | DEL-10-05 `_STATUS.md` Remaining now holds only the `export-results` bullet; History records the R12 binding tranche. Receipts tail: `Receipt-59` with `Examined-Through: 96563e8e09b8...`. `git log`: HEAD `45ec0524d` = merge of PR #287; implementation commit `60841413a`. |
| 15 | DAG posture | **CONFIRMED** (note B) | `_LATEST.md` → `DAG-007`, `approved_active_graph_authority`. `DependencyEdges.csv` lines 766–777 are the only 12 DEL-09-04 mentions in the file: three ANCHOR rows (`NOT_APPLICABLE`), the enumerated EXECUTION UPSTREAM rows `DAG-002-E0286/E0287/E0288/E0289` + `E0543/E0544/E0545` + `E0546` all `SATISFIED`, and downstream `DEL-09-04-E001` `SATISFIED`. No `TBD` execution-upstream row. |
| 16 | Profile check-trigger determination | **CONFIRMED** | `software-workflow.json` lines 13–18 read exactly as claimed: `always_checks: ["harness-self-check"]`; `core/**`,`validation/**`,`tests/**` → `piping-pytest`+`evidence-sweep`; `execution/**`,`docs/**`,`AGENTS.md`,`loop/**` → `harness-pytest`. |
| 17 | Fence completeness | **CONFIRMED** | Brief §5 enumerates the brief status record, the page, three DEL-09-04 files, the R13 AgentRuns directory, and one receipt append. Every §4 task and §6 step writes only inside that set (check JSONs/containment stdout → item 4; receipt → item 5; validators are read-only); no step requires `validation/**`, `core/**`, `schemas/**`, `tests/**`, DEL-10-05, or `index.md`. All §6 tools exist at their cited root paths. |
| 18 | No new criteria | **CONFIRMED** | Every value the brief permits the page to assert (§3.2 fixtures/counts/exits, §3.3 blocked counts, §3.4 historical note) was independently re-derived from the committed witnesses, the R12 preserved evidence, or the bin source in claims 1–11 above; §3.1 makes witness-anchoring an acceptance condition, §3.7 forbids new tolerance/threshold/criterion content, §9 preserves DEC-046 and MAINTAINER_REVIEWED (promotion not performed). |
| 19 | Frozen-surface preservation checkable | **CONFIRMED** | All seven frozen E1 surfaces exist in the live tree; `git status --porcelain` over the seven paths is empty now, and brief §3.4/§6 require the same emptiness at closeout. |
| 20 | Rerun consequence grounded | **CONFIRMED** | `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8 sits at lines 455–468 and requires a new run and new immutable bundle on E1-procedure/documenting-surface change; R12 `HANDOFF_STATE.md` records the fresh-run-ID consequence for the DEL-09-04 follow-on; bundle `REPRO_DEL0904_20260719T202023Z_23eeaabc9040/` exists with manifest/SHA256SUMS and is recorded `INTERNALLY_VERIFIED` in the R11 run record (line 32). |

**Note A (claims 5–7, non-material imprecision):** the three success
witnesses serialize `"diagnostics": []` (empty array), not JSON `null` as the
N1 return's shorthand says. The operative fact — no blocking diagnostic,
hence exit 0 under DEC-065 — is confirmed, and the brief's own §6 cross-check
language ("`diagnostics: null`/empty") matches the actual bytes, so the
executor's mandatory check is stated correctly in the governing instrument.
Not refuted; recorded for precision.

**Note B (claim 15 and brief §2, non-material miscount):** both texts say
"six EXECUTION UPSTREAM rows" while enumerating eight row IDs
(E0286–E0289, E0543–E0545, E0546). I verified the enumeration itself: exactly
eight execution-upstream rows exist and all eight are `SATISFIED`. The
under-count does not weaken the posture conclusion (no `TBD`, nothing
blocking) and no brief predicate or fence term depends on the cardinal. Not
refuted; recorded for precision.

## 2. Independent fast-reject re-screen (D-52 §4.1, D-54 §3.1 refined)

Re-run against the brief's actual §3–§5 scope, hardest on classes 4 and 5:

1. Irreducible preference / two defensible outcomes — **PASS.** The
   documentation-shape choices are exactly the D-54 reasoned-selection case;
   rationale §4 records seven material alternatives with reasons; deferral has
   no positive basis (the live tree contradicts a governed evidence page).
2. Accountability — **PASS.** The page stays `draft_evidence`; §3.7 keeps
   acceptance and professional judgment with the responsible engineer.
3. Authority conflict — **PASS.** Stale derivative page vs live tree is a
   truth-repair; the R12 adopted brief and HANDOFF_STATE already routed this
   exact follow-on here. No two authorities disagree.
4. Scope/criteria change, new normative content — **PASS (attended
   hardest).** I re-derived every value the brief lets the page assert from
   committed evidence (claims 5–11); nothing originates in this tranche. §3.3
   describes fail-closed behavior strictly as regression evidence and leaves
   thresholds/tolerance/CI-gate policy `TBD` owner-gated; §3.1 makes
   witness-anchoring itself an acceptance predicate; the `_STATUS.md` edit
   strikes only the dependency DEL-10-05 discharged (Receipt-59), keeps
   MAINTAINER_REVIEWED promotion and GUI-workflow evidence open in the same
   bullet, does NOT perform the MAINTAINER_REVIEWED promotion, and requires
   the owner-gated DEC-046 tolerance bullet byte-identical (§6 byte-identity
   check). No new acceptance criterion, boundary change, or override.
5. Lifecycle/acceptance acts — **PASS.** `IN_PROGRESS` unchanged; no bullet
   closure, no E2-row closure, no evidence-posture promotion, no reproduction
   acceptance; all named preserved gates in §9/§10.
6. Third-party/publication/external — **PASS.** Local docs+state only; §5
   explicitly withholds push/PR/network.
7. Merge/destructive — **PASS.** Owner merges; struck text is recorded as
   landed, not erased; frozen surfaces, witnesses, and completed bundles
   read-only with byte-identity checks.
8. Protected/private data — **PASS.** Invented public fixtures only.
9. Evidence stale/unverifiable — **PASS.** Every outcome-determining premise
   re-verified live at N2 (this return); the only preserved-evidence reliance
   (exit codes, whole-suite counts) is the R12 N3 return that N4 v2 ruled
   `COMMIT-SAFE`, and it is consistent with the bin source read directly.
10. Domain-engine/prover/secrets/higher-order boundaries — **PASS.**

**Re-screen result: PASS on all 10 classes; no near-miss found.** D-52 §4.2
gates 1–7 are satisfied on the record (live-tree sufficiency, existing
authority only, four lenses present, single surviving outcome with adversarial
self-test, bounded held effect, truthful attribution with
`OwnerCaseSelection: NONE`, rationale artifact in place); gate 8 is this
refutation. D-54 §3.2 items 1–8 and the §3.3 attribution block are present in
the brief §10 / rationale §7.

## 3. Fence, predicate, and profile findings

- **Fence completeness/containment:** VERIFIED. Executing §4.1–§4.4 requires
  no durable path outside the five §5 items; the §6 evidence outputs land in
  the R13 AgentRuns directory (item 4); the receipt append is item 5; every
  named validator/tool exists at its cited path. The negative list correctly
  covers `index.md`, `validation/**`, DEL-10-05, and completed bundles.
- **Predicate checkability:** VERIFIED. Each §3 predicate has a concrete
  check: witness parsing (§6 mandatory cross-check), byte-identity via
  `git status --porcelain` (frozen surfaces) and pre-edit comparison (second
  bullet), containment via `validate_change_scope.py`, registered checks via
  `run_registered_checks.py`, and the three root validators.
- **Profile determination:** INDEPENDENTLY CONFIRMED. The fence's paths fall
  only under the `execution/**`/`docs/**`/`loop/**` rule → `harness-pytest`
  plus always-on `harness-self-check`. `piping-pytest`/`evidence-sweep`
  trigger only on `core/**`/`validation/**`/`tests/**`, which the fence
  forbids; an evidence sweep would itself write
  `validation/evidence/sweeps/SWEEP_*.json`, so the no-sweep determination is
  the self-consistent fixed point. The brief's §6 rule that a late `core/**`
  or `validation/**` path is a fence violation, not a check trigger, is the
  correct fail-closed direction.
- **Parent-verified external fact re-checked:** the branch is 0 ahead / 2
  behind `origin/main`, and
  `git diff --stat HEAD origin/main -- projects/chirality-piping` is empty —
  the drift touches zero piping paths, as stated.

## 4. Verdict

- Claims: 20 total — 20 CONFIRMED, 0 REFUTED. Two non-material precision
  notes recorded (Note A: `diagnostics` is `[]` not literal `null` in the
  three success witnesses; Note B: "six" should read "eight" execution
  upstream rows, all SATISFIED).
- Fast-reject re-screen: PASS on all 10 classes; no near-miss.
- Fence, predicates, profile: verified; no gap, no unauthorized write path,
  no sweep required.

Most material finding: nothing rose to refutation — the closest items are two
label-level imprecisions that do not survive as risks because the governing
instrument itself is accurate where it matters (the brief's §6 cross-check
says "null/empty", matching the committed `[]` bytes, and no predicate or
fence term depends on the DAG row count, whose full enumeration I verified as
eight-for-eight `SATISFIED`); every value the refreshed page is authorized to
assert was independently re-derived from committed witnesses, the R12
preserved evidence chain (N4 v2 `COMMIT-SAFE`), or the live bin source, and
the tranche performs no promotion, closure, or acceptance act.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
return records verification observations only; it makes no acceptance,
release, professional, certification, sealing, authentication, or
code-compliance claim.

VERDICT: COMMIT-SAFE
