# VERIFY_IMPL — Fresh-Context Adversarial Implementation Verification (W4 / T6)

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T6
**Role:** Fresh-context implementation verifier (governed Agent 2, read-only over product state except this artifact; model claude-fable-5)
**Date:** 2026-07-20
**Object:** the uncommitted T6 tranche (executor return `instances/W4/T6/EXECUTE_RETURN.md`, OVERALL STATUS: BLOCKED) plus the W4 manager's post-return cure of `instances/W4/W4_DISPATCH_TRANSCRIPT.md`
**Basis:** live working tree, branch `claude/piping-r14-pkg09-evidence`, HEAD `e315fb8406d44dce684cbec091f3174c261efee4` (= brief base; verified `git rev-parse HEAD`); all checks re-run offline (`CARGO_NET_OFFLINE=true`, `--offline`; nothing installed or fetched)
**Verdict:** `COMMIT-SAFE`

Method: every §6 check was re-executed independently in this session; every
executor E-claim was tested against the live tree, not against the return's
prose. Capture and assembly re-runs used scratch outputs or verified
byte-identity in place; no product file was modified by this verification.

## Independently Re-Run §6 Check Tally

| # | Check (re-run by this verifier) | Outcome |
|---|---|---|
| 1 | `CARGO_NET_OFFLINE=true cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml` | PASS — 38 passed; 0 failed; 0 ignored; doc-tests 0; matches executor |
| 2 | Whole-suite capture re-run: `cargo run --offline ... run-benchmark --input <bundle>/suite_run_mechanics_input.json --output <scratch>` | PASS — exit code 1 reproduced; re-run output byte-identical (`cmp`) to the committed-in-tranche `SUITE_RUN_MECHANICS.json`; stdout byte-identical to `--output` |
| 3 | `assemble_index.py` rerun determinism (recorded args: `--generated-at 2026-07-20T06:23:42Z --base-commit e315fb84... --branch claude/piping-r14-pkg09-evidence --capture-exit-code 1`) | PASS — SHA-256 of all 6 bundle files identical before and after rerun (index CSV, MANIFEST.json, and every other file byte-identical) |
| 4 | Index integrity (independent script, not the bundle's own) | PASS — 24 data rows == live `fixture_inventory()` length; row order equals the source constructor order (fixture_id-by-fixture_id); all 24 `witness_anchor_exists=true` with each anchor an existing committed file under `validation/hand_calcs/mechanics/` equal to the crate's `public_original(...)` source_location; all `redistribution_status=PublicOriginal` (crate-recorded variant re-derived from lib.rs); every `head_suite_run_status` equals the captured per-case status; every blocked row carries the captured blocking code |
| 5 | JSON parse (input, capture, manifest) + CSV parse (index) | PASS — all parse |
| 6 | Frozen-surface guard: `git status --porcelain` per subtree over `validation/witness`, `validation/benchmarks`, `validation/hand_calcs`, `fixtures` | PASS — all four empty; the only `validation/evidence` entry is the new `benchmarks/` bundle home |
| 7 | DEL-09-01 `_STATUS.md` vs base `e315fb840` (`git diff`) | PASS — diff is exactly the `Last Updated` date line (2026-07-12 → 2026-07-20) plus ONE appended History line; `## Remaining` byte-identical (all four rows present and untouched, re-counted = 4); `Current State: IN_PROGRESS` unchanged |
| 8 | `python3 tools/validation/validate_claims_language.py --repo-root .` (from REPO_ROOT) | PASS — 262 files scanned; DEC-081 registry taxonomy satisfied (matches executor's post-amendment count) |
| 9 | `python3 tools/validation/validate_path_anchors.py . --text` (from REPO_ROOT) | PASS — 0 findings in 686 live path-anchor surfaces (post-cure; the executor's recorded FAIL is assessed below) |
| 10 | `git diff --check` (from REPO_ROOT) | PASS — clean |
| 11 | Containment: `validate_change_scope.py "$REPO_ROOT" --base e315fb840` with the six §5 fence `--allowed` roots and the 11 tranche `--path` writes | PASS — status PASS, 0 violations; my re-run JSON is byte-identical (`diff`) to the persisted `instances/W4/T6/CHANGE_SCOPE_CONTAINMENT.json` |

Re-run tally: **11 of 11 checks PASS** on the current (cured) tree. The
executor's tally (10 PASS / 1 FAIL at check 9) is consistent with the
pre-cure state it truthfully recorded.

## Per-E-Claim Refutation Table

| Claim | Verdict | Evidence (this session) |
|---|---|---|
| E1 — bundle contains exactly 6 files; nothing else under `validation/evidence/benchmarks/` | **CONFIRMED** | `find` over `validation/evidence/benchmarks`: exactly the 6 named files inside `BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`; manifest `file_sha256` key set + MANIFEST.json equals the directory listing (no extra, no missing) |
| E2 — SHA-256 of each of the 5 non-manifest files matches `MANIFEST.json` | **CONFIRMED** | All 5 recomputed hashes MATCH |
| E3 — capture content: suite/deliverable/whole-suite-default/24 requested/11-0-13 tallies/13 blocking diagnostics all coded `..._CASE_COMPARISON_BASIS_NOT_REUSABLE` | **CONFIRMED** | Parsed `SUITE_RUN_MECHANICS.json`: `suite=mechanics`, `suite_deliverable=DEL-09-01`, `whole_suite_default_applied=true`, `requested_case_count=24`, 24 cases tallying `{executed_and_matched: 11, blocked: 13}` (0 mismatched), 13 diagnostics, single code `HEADLESS_RUNNER_BENCHMARK_CASE_COMPARISON_BASIS_NOT_REUSABLE` |
| E4 — re-running the capture reproduces exit 1 and the same content; exit semantics per DEC-065 | **CONFIRMED (stronger than claimed)** | Offline re-run at this head: exit 1; entire output file byte-identical to the captured one (not merely same `suite_run` content); manifest records `recorded_exit_code: 1` with the DEC-065 semantics statement |
| E5 — 24 index rows in live `fixture_inventory()` order; rerun byte-identical | **CONFIRMED** | Constructor order re-parsed from `validation/benchmarks/mechanics/src/lib.rs`; fixture_id sequence equals the index row sequence; assembly rerun left all bundle files byte-identical |
| E6 — every witness anchor exists under `validation/hand_calcs/mechanics/` and equals the crate `public_original(...)` source_location | **CONFIRMED** | 24/24 anchors are existing committed files; `witness_anchor_paths == provenance_source_location` for every row; all under `validation/hand_calcs/mechanics/` |
| E7 — suite tests 38/0 at this head with the crate clean | **CONFIRMED** | Re-run: 38 passed / 0 failed; `git status --porcelain -- validation/benchmarks` empty |
| E8 — `## Remaining` byte-identical to base; only date line + one History line changed; IN_PROGRESS | **CONFIRMED** | `git diff e315fb840` over `_STATUS.md` shows exactly those two hunks; numstat 2 added / 1 removed; Remaining rows re-counted = 4, byte-identical in the diff |
| E9 — durable changes = the 11 tranche writes + lawful pre-existing R14 W4 state; containment PASS | **CONFIRMED** | Full `git status --porcelain` yields exactly: the 2 modified DEL-09-01 state files, the new run record, the new bundle dir (6 files), the new `instances/W4/` dir (T6's 4 artifacts + the manager's dispatch transcript), and the T6 brief — nothing else. The 11 executor writes are exactly the claimed set; the brief, `VERIFY_BRIEF.md`, `CURRENT_CANDIDATE_RATIONALE.md`, and `W4_DISPATCH_TRANSCRIPT.md` are the W4 manager's lawful artifacts (brief = fence item 1; instance dir = fence item 4; transcript = pre-tranche manager artifact). Containment re-run byte-identical to the persisted JSON, 0 violations |
| E10 — path-anchors: exactly one finding, at the transcript line 19, out-of-fence; zero in tranche writes | **CONFIRMED** (pre-cure state corroborated, not directly re-observable) | Post-cure the validator reports 0 findings; independent `grep` for `/Users/` literals over all 11 tranche writes: zero matches; the validator's pattern (lib line 32) flags `/Users|/home` literals and exempts `{`-prefixed placeholders, so the tranche's `{REPO_ROOT}` style is clean by construction. The pre-cure single finding at line 19 is corroborated arithmetically: the cure inserted the 8-line redaction-note block (current lines 16–23), and the literal path sat on the `REPO_ROOT =` line now at line 27 — 27 − 8 = 19, exactly the executor's recorded location |
| E11 — no receipt append, no Remaining edit anywhere, no lifecycle change, no commit/branch/push/merge beyond base | **CONFIRMED** | `loop/LOOP_RECEIPTS.md` clean; only DEL-09-01 `_STATUS.md`/`MEMORY.md` modified tree-wide and neither touches a Remaining row; branch tip == HEAD == `e315fb840`; `for-each-ref` shows no new refs at or beyond this work |

No refutation attempt succeeded. Every E-claim held against the live tree;
E4 held more strongly than stated (full byte-identity, not just content
equality).

## Blocked→Cured Chain Assessment

**(a) The cure is real and the redaction note is truthful.** The transcript's
`REPO_ROOT =` line (line 27) now carries the placeholder
`<machine-local checkout root; literal home-dir prefix redacted per
path-anchor policy>`; the note (lines 16–22) states exactly one substitution
on exactly that line and correctly states the policy ground
(`validate_path_anchors.py` line 32 forbids literal `/Users|/home` absolute
paths on live surfaces). The line-number arithmetic (current line 27 minus
the 8-line inserted note block = pre-cure line 19) matches the executor's
recorded finding location exactly. Because the tranche is uncommitted, the
pre-cure bytes cannot be replayed from git; the executor return, the run
record's check-9 row, the note, and the arithmetic are mutually consistent
with no contrary evidence.

**(b) The three re-run checks now pass.** Independently re-run this session:
claims-language PASS (262 files), path-anchors PASS (0 findings / 686
surfaces), `git diff --check` clean.

**(c) The executor's fail-closed halt was correct and its BLOCKED status is
attributable solely to the out-of-fence finding.** Brief §6 makes every check
failure stop state-changing closeout and §7 makes an out-of-fence repair
need report-and-return, never fix-here; the transcript is a W4-manager
artifact outside every §5 fence item, so the executor had no lawful cure
path. My re-run confirms every other §6 check passes and every §3
constructed-state predicate holds; removing the single check-9 finding
leaves nothing else blocking. The executor's disposition (truthful partial
state, amended run-record tally, follow-on F1 routed to the manager) is
exactly the §7 posture.

**(d) The cure touched no tranche or product surface.** All 5 hashed bundle
files still match the manifest hashes computed at assembly (pre-cure), the
capture re-run is byte-identical, the frozen-surface guards are empty, the
DEL-09-01 diffs contain only the §3.8 changes, and containment re-runs
byte-identical to the persisted JSON. The only artifact carrying cure
content is `instances/W4/W4_DISPATCH_TRANSCRIPT.md` (manager-owned,
instance-dir surface, outside product state).

## Defects

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| V1 | INFO | The pre-cure validator FAIL cannot be directly re-observed (uncommitted working-tree history is not replayable); it is corroborated by four independent consistent records and the line-number arithmetic above. | Not load-bearing: what the commit will contain is the CURRENT tree, which passes all 11 checks as re-run here. |
| V2 | INFO | The new MEMORY entry's "24 project-original fixture/hand-calc families/fixtures" continues the deliverable's pre-existing convention of counting fixture/hand-calc families as fixtures (the prior entry's "21 families" = 21 fixtures); the 24 fixtures span 16 distinct `BenchmarkFamily` enum values. The entry hedges with "families/fixtures" and states the count basis (21 pre-R14 + three R14 additions). | No claims-strength or gate effect; terminology continuation, recorded for the owner's awareness. |
| V3 | INFO | The transcript's "verbatim except one redaction" property is not independently checkable against a source (the live HELP_HUMAN dispatch has no other durable record — that absence is the VERIFY_BRIEF D1 finding this artifact cures). Its substantive constraints remain independently grounded in `instances/W3/RETURN.md` and the brief's own binding text, per VERIFY_BRIEF C2/D1. | Not load-bearing for the T6 commit; D1's fan-in cure obligation (durable dispatch transcription) is now satisfied in form by this transcript. |

No MEDIUM or higher defect. No DEC-081 violation found on any new durable
text surface (validator PASS plus direct reading of `TEST_REFRESH.md`,
`MANIFEST.json` standing statements, the History entry, the MEMORY entry, and
the run record: every one carries the derivative/non-authoritative label
where required, the PDU-037-row-stays-open, PDU-013-hold, and owner-gate
statements, and no statement closes, strikes, or weakens any Remaining row or
owner gate).

## Terminal Verdict

**`COMMIT-SAFE`** — the W4 manager may commit this tranche. All 11 §6 checks
pass on the current tree as independently re-run; all 11 executor E-claims
are CONFIRMED; the executor's fail-closed BLOCKED return was correct behavior
on the pre-cure tree and its sole blocker is cured in the manager's own
out-of-fence artifact with a truthful redaction note; the cure touched no
tranche or product surface; HEAD, branch, refs, and receipts are unchanged at
base `e315fb840`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
