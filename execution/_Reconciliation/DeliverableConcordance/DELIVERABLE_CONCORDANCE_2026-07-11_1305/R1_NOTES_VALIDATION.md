# R1 Notes — Validation-and-Provenance Index

Run: `DELIVERABLE_CONCORDANCE_2026-07-11_1305` (activation D-41/DEC-073, owner-adopted scale-out).
Phase: R1 whole-corpus inventory, READ-ONLY, per plan §8 R1 and `R1_CONVENTIONS.md`.
Companion artifact: `VALIDATION_AND_PROVENANCE_INDEX.csv` (396 rows, RFC-4180, CRLF, header row).

## 1. Source-state binding

- All reads from the frozen worktree
  `/Users/ryan/ai-env/projects/chirality/.claude-worktrees/piping-frozen-551f84ef6`,
  project root `projects/chirality-piping`, verified at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (`git rev-parse HEAD` checked at
  start; `git status --porcelain` verified clean at start and end of the pass).
- Side-effect-free method (addendum 9): file reads, `grep`/`find`, JSON parsing,
  and read-only `git cat-file` / `git merge-base --is-ancestor` ancestry checks
  only. No build, no test execution, no writes into the frozen tree. Outputs
  written only to this run folder in the concordance working tree.
- `BindingToFrozenSHA` column semantics: every inventoried file is
  `IN_TREE_AT_FROZEN_SHA` by construction (it exists in the frozen tree);
  commit-bound evidence records additionally carry an ancestry disposition for
  their recorded `commit_hash` (and, for gate records, both `evaluated_commit`
  and `emitted` commit): `ANCESTOR_OF_FROZEN` / `NOT_ANCESTOR_OF_FROZEN` /
  `COMMIT_NOT_IN_REPO`.

## 2. Counts by Kind (396 rows)

| Kind | Count |
|---|---:|
| EVIDENCE_SWEEP_RECORD | 274 |
| HAND_CALC_WITNESS | 64 |
| TOLERANCE_POLICY_RECORD | 17 |
| DECLARED_STATE_INDEX | 7 |
| RELEASE_GATE_RECORD | 5 |
| GENERATED_RUNNER_WITNESS | 4 |
| WITNESS_INPUT | 4 |
| BENCHMARK_SUITE_CRATE | 3 |
| GENERATED_WITNESS_RENDERING | 2 |
| EXTERNAL_ORACLE_HARNESS | 2 |
| EXTERNAL_ORACLE_FIXTURE | 2 |
| EXTERNAL_ORACLE_SCHEMA | 2 |
| EXTERNAL_ORACLE_TEST | 2 |
| HAND_CALC_OBSERVATION_NOTE | 1 |
| FORMAL_WITNESS_SOURCE | 1 |
| WITNESS_CONTENT_DICTIONARY | 1 |
| WITNESS_SCHEMA | 1 |
| WITNESS_TOOLING | 1 |
| BENCHMARK_FIXTURE_PAYLOAD | 1 |
| COVERAGE_TELEMETRY_RECORD | 1 |
| RELEASE_ARTIFACT_RECORD | 1 |

Hand-calc witness split: mechanics 21, nonlinear 28 (+1 observation note),
stress 15. One formal machine-readable witness pipeline
(`WITNESS-TP-PHYS-015-SECTION-PROPERTY-STRESS`: JSON source authoritative, MD +
MathML deterministic renderings, pytest-gated).

## 3. Fixture counts (exact, crate-asserted)

The named R1 repair in `R1_CONVENTIONS.md` Part C (DEL-09-01 count = 21, not
22) is confirmed three ways at the frozen SHA:

- **DEL-09-01 mechanics**: `validation/benchmarks/mechanics/src/lib.rs:5495`
  asserts `fixtures.len() == 21`; 21 distinct `MECH-*` fixture IDs appear in
  the crate; 21 hand-calc notes are inventoried in
  `validation/hand_calcs/mechanics/README.md`. (An unrelated line-5930 assert
  `expected_values.len() == 21` is a per-fixture value count, not the fixture
  count; a raw grep of struct-field occurrences yields 22 hits and is the
  plausible source of the prior 22-vs-21 miscount.)
- **DEL-09-02 stress**: `validation/benchmarks/stress/src/lib.rs:1730` asserts
  `fixtures.len() == 15`; 15 hand-calc notes mirrored in the README.
- **DEL-09-03 nonlinear**: four crate-asserted inventories totalling 28
  fixtures — point-support 5 (`src/lib.rs:4750`), assembled 9 (`:4758`),
  multisupport acceptance 13 (`:4830`), depth observation 1 (`:4765`) — with
  28 fixture notes plus 1 convergence-observation note in the README.

## 4. Tolerance tiers observed

- **DEC-024/DEC-026** (D-04 ruling, 2026-06-11, revised same day to T-C with
  riders): class-tiered governed relative+absolute pairs; governed release
  values remain `TBD` across all three benchmark suites. The DEC-026
  analytic-class `1.0e-9` relative tier is concretely cited for the curved-bend
  comparisons (`MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`,
  `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END`, absolute floor `1.0e-3` N / N-m on
  exact-zero rows).
- **DEC-046 CV-B** (D-19 ruling): 12 machine-readable policy/observation
  records in `validation/benchmarks/nonlinear/*.dec046.json` — seed set
  (active-set count rel 0.0 / abs 0.0 / cap 4; free-DOF force/moment 0.0 N /
  0.0 N-m; work and general-energy 0.0 N-m; class-tiered displacement/reaction
  deltas 100/50 mm, 10/5 N) and multisupport set (100.0 mm, 0.005 rad, 10.0 N,
  3.0 N-m, general-energy 0.0 N-m). All records are explicitly scope-limited to
  the named invented fixture sets.
- **DEC-050 / DEC-053** (D-17 / D-26 rulings): 5 sparse suitability /
  conditioning / default-promotion records in `validation/benchmarks/*.json`;
  dense path recorded as default solver and parity oracle.
- **DEC-025** sweeps and **DEC-060** coverage carry pass/fail and
  recorded-never-blocking semantics respectively (no numeric tier).
- The depth-observation inventory uses `TP-R4-D9-MULTISUPPORT-OBS-TBD` with
  `ConvergencePolicyStatus::Tbd` — intentionally outside the DEC-046 policies.

## 5. SourceReliability histogram (addendum-6 ladder, applied conservatively)

| Value | Count |
|---|---:|
| UNVERIFIED | 378 |
| NOT_APPLICABLE | 18 |
| REVIEWED | 0 |
| VETTED | 0 |

Rationale:

- **VETTED = 0**: no named vetted external reference is used anywhere in the
  validation corpus. Every hand-calc witness declares project-original invented
  public content derived from elementary open mechanics (force-method /
  unit-load / Castigliano derivations authored in-repo for the curved-bend
  pair), explicitly "not copied from protected standards, commercial software
  examples, or proprietary data".
- **UNVERIFIED (conservative)**: all technical evidence rows (witnesses,
  crates, policy records, sweeps, gates, coverage, release-artifact record).
  These are project-original agent-generated technical evidence with agent
  audit. The governing *mechanisms* are covered by named human rulings
  (DEC-024/025/026/046/050/053/057/058/060, owner Ryan Tufts per the decisions
  register), but no in-record human disposition covering the cited record
  itself was found (the DEC-046/050/053 JSON records carry no human-approval
  field), so per addendum 6 the rows stay UNVERIFIED rather than REVIEWED. Not
  adjudicated here: whether the D-26/D-27 R4-exit acceptance of the sparse
  residual evidence packet, or the D-34/DEC-070 exit-evidence ruling for the
  curved-bend witnesses, constitutes a "recorded human disposition covering the
  cited record" — a human call could promote those specific rows to REVIEWED
  (see §7).
- **NOT_APPLICABLE**: the 7 declared-state README index surfaces (addendum 6:
  declared-state prose rows) plus pure infrastructure rows (witness schema,
  content dictionary, validator tooling, witness inputs, oracle schemas and
  contract tests) that carry no technical-evidence claim of their own.

## 6. Boundary flags (HARD BOUNDARY grep)

Repo-wide grep for `domains/piping-design` over the frozen project tree was
run. **Two** validation-evidence files were flagged
`EQUATION_SOURCE_BOUNDARY_VIOLATION_CANDIDATE` (expected zero; flagged, not
adjudicated per instruction):

1. `validation/evidence/sweeps/SWEEP_20260710T151446Z_b77e721b2028-dirty.json`
   — `git.dirty_paths` lists
   `domains/piping-design/_Sources/Process-Piping-Design-Rip-Weaver-Volume-1/audit/equations_flagged.json`
   (a PDF/OCR-derived equations audit artifact) as dirty in the working tree at
   sweep time.
2. `validation/evidence/sweeps/SWEEP_20260613T234909Z_4d4a4a7ee955-dirty.json`
   — `git.dirty_paths` lists `domains/piping-design/_LocalIndexes/` (source
   index area) among 17 dirty paths.

Characterization (factual only): in both cases the reference occurs solely in
the sweep's captured `git status` dirty-path list; no validation artifact,
witness, fixture, or benchmark *content* in the frozen tree cites, imports, or
derives from `domains/piping-design` material. No equation or formulation in
any hand-calc witness cites a PDF/OCR source. Adjudication of whether a
dirty-path listing constitutes an evidence chain "touching" the boundary is
left to the human reviewer.

## 7. Pending-human-disposition items (addendum 13)

- All three benchmark-suite READMEs declare final tolerance policy, release
  thresholds, CI gate policy, benchmark publication scope, external validation
  claims, and professional reliance as `TBD` pending human approval — every
  witness/crate row is therefore capped per addendum 13 (Confidence MEDIUM,
  AuthorityNeeded=OWNER) if consumed as validation evidence in R2 ledgers.
- DEC-046/DEC-050/DEC-053 policy records: mechanism ruled, record values
  agent-measured; possible promotion to REVIEWED needs a human call (see §5).
- Curved-bend witnesses (D-34/DEC-070 §5 exit-evidence bar): ruling defines the
  evidence bar; disposition of the delivered witnesses vs. that bar is a human
  call.
- CAEPIPE external oracle: `human_review_required=True` is embedded in the
  harness `PROFESSIONAL_BOUNDARY`; no real CAEPIPE outputs exist at the frozen
  SHA (see §8), so there is no oracle comparison awaiting disposition — only
  the harness/fixture surface itself.
- Release-gate records leave TBD criteria unresolved by design ("no value is
  invented, estimated, or defaulted"); release-label vocabulary is human-gated
  (PB-TBD-003).

## 8. External-oracle source state

The CAEPIPE surface (DEL-17-04/17-05) consists of: evidence-package builders
(`core/handoff/caepipe_external/run.py`, `core/handoff/caepipe_mbf/package.py`),
schemas, contract tests, and **invented** parser fixtures
(`fixtures/caepipe_external/invented/caepipe_results.csv`,
`fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json`). **No actual
CAEPIPE-generated output or external-oracle comparison result exists anywhere
in the frozen tree**; the harness statuses are skipped / parser-only /
user-owned-run, and the module boundary explicitly forbids treating parsed
output as compatibility or validation evidence. Related run-record mentions
(TP-UNITS CAEPIPE conversion witnesses under `execution/PKG-02` /
`PKG-17` `_run_records/`) are deliverable run records, not validation-tree
evidence, and were left to the R2 deliverable-grain ledgers.

## 9. Evidence-record commit bindings

- **Sweeps (274)**: 180 clean-tree, 94 dirty-tree (`-dirty` suffixed; not
  commit-reproducible). Ancestry of recorded `commit_hash` against the frozen
  SHA: 273 `ANCESTOR_OF_FROZEN`, 1 `NOT_ANCESTOR_OF_FROZEN` (see §10).
  Overall surface status: 265 all-pass, 9 with at least one non-pass surface
  (see §10).
- **Gate records (5)**: all five gate families (gui, mixed, report_template,
  rule_engine, solver) evaluated at
  `e2ea37194c8acf8e4a77c5fb2fecf92adb1c939c`, emitted at
  `6dcb328ac5cf789fefc0586af00b59cd2ae9e819`; both ancestors of the frozen SHA.
- **Coverage (1)**: `e9cd806811b3…` clean, ancestor of frozen.
- **Release artifact (1)**: `8e436704b52b…` clean, ancestor of frozen;
  DEC-057 O-A unsigned Apple-Silicon bundle with sha256 checksum.

## 10. Anomalies

1. **One sweep commit not an ancestor of the frozen SHA**:
   `SWEEP_20260615T043935Z_acd0f6bfbdbf-dirty.json` records
   `acd0f6bfbdbf8409dd41d53eb3849d81c2be3c86` (branch `main`, dirty). The
   commit exists in the repository object store but is not on the frozen
   lineage (plausibly rebased/superseded). The sweep file itself is committed
   at the frozen SHA; the *evidence* it carries is not commit-reproducible on
   the frozen lineage.
2. **Nine sweeps with non-pass surfaces** (all retained in the corpus, none
   deleted): `SWEEP_20260613T200943Z_add584756bb1-dirty`,
   `SWEEP_20260621T070045Z_251dbcd8ce97-dirty`,
   `SWEEP_20260710T232952Z_b1ff985c72eb`,
   `SWEEP_20260711T012340Z_968b4f7de74e`,
   `SWEEP_20260711T012843Z_dfa5dd344429`,
   `SWEEP_20260711T020214Z_50f230b09885`,
   `SWEEP_20260711T024049Z_f940b5b7c1af`,
   `SWEEP_20260711T024551Z_73e4b6f7c848`,
   `SWEEP_20260711T024949Z_3c12cc27b158` — failing surfaces are desktop lanes
   (`desktop_vitest` and/or `desktop_playwright_e2e` +
   `desktop_production_build`). Seven of the nine are clean-tree records; the
   cluster of six on 2026-07-10/11 sits close to the frozen SHA and is a
   candidate input to any "latest clean all-pass sweep" selection an R2 ledger
   might make.
3. **Gate-record absolute path leakage**: the gate records embed an absolute
   `sweep_dir` under a different (agent) worktree path
   (`…/.claude/worktrees/agent-a30e381bd027b103d/…`). Content anomaly only; the
   relative artifact selection (`SWEEP_20260711T032140Z_e2ea37194c8a.json`) is
   resolvable in the frozen tree.
4. **Miscount trap documented** (§3): raw ID-occurrence greps over the
   mechanics crate over-count; the crate's own `fixture_inventory()` length
   assertion (21) is authoritative.
5. `validation/benchmarks/nonlinear/.gitignore` and
   `validation/benchmarks/stress/.gitignore` exist (build-artifact hygiene);
   noted only because addendum 9 forbids writes even on git-ignored paths.

## 11. Method notes

- Inventory grain: one row per witness note, per machine-readable record, per
  evidence JSON, per crate (suite grain, with exact fixture counts embedded in
  `Validates`), per oracle surface file, per declared-state README.
- `EvidenceID` scheme: `<KIND-PREFIX>-NNN` (HC-MECH, HC-NL, HC-STRESS, WIT,
  BENCH, POLICY, ORACLE, IDX, GATE, COV, REL, SWEEP), stable within this run
  only.
- Sweep rows' `CommitBinding` uses `commit=<full-sha>[-dirty]`; the `-dirty`
  marker mirrors the artifact filename convention.
- Frozen-tree cleanliness: `git status --porcelain` re-verified clean after the
  final read pass.
