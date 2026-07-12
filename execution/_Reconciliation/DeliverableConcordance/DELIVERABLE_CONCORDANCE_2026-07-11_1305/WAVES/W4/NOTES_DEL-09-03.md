# Notes — CLAIM_CONCORDANCE_DEL-09-03 (R2 wave W4)

Deliverable: DEL-09-03 “Nonlinear support regression suite” (PKG-09).
Discovery pilot: GPT-5 at the standard discovery capability tier, as assigned
at dispatch. All dispositions are agent judgments, never owner or engineering
rulings.

Discovery basis: FROZEN evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` only. Ledger encoding follows
`R1_CONVENTIONS.md` Parts A–D, pinned plan §§6–8, the W1–W3 calibration carried
in `PACKAGE_SUMMARIES/PKG-00..08.md`, and the W4 addendum-9 mitigation in
`RUN_BASIS.md`.

Run-level `NormativeSource` path alias (addendum 12): unqualified deliverable
files (`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REVIEW.md`,
`Review_Findings.csv`, `_run_records/...`) resolve under
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/`;
all other paths are repo-`projects/chirality-piping`-relative.

## Histograms

Row count: 21.

Disposition histogram:

| Disposition | Count |
|---|---:|
| ALIGNED | 15 |
| STALE_SETUP_SPECIFICATION | 4 |
| REMAINING_STATE_MISMATCH | 2 |

ClaimType histogram:

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 8 |
| ACCEPTANCE | 1 |
| EXCLUSION | 3 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 2 |

Mechanical selectability: zero `YES` rows. The seeded D-41 item is copied
byte-exact only into DECL-005 and excluded from residual/gate/selectability
analysis. The two omitted pending-disposition findings have no recorded
`## Remaining` items or gate suffixes, so they use the no-record default
(`NONE_RECORDED`, `NO`) rather than inventing selectable work.

## Sources and evidence checked

- Complete DEL-09-03 kit, current-state surfaces, dependency artifacts,
  `_REVIEW.md`, `Review_Findings.csv`, and all deliverable `_run_records/**`.
- `validation/benchmarks/nonlinear/` including README, Rust source, Cargo
  manifest/lock, and twelve DEC-046 observation/policy JSON records;
  `validation/hand_calcs/nonlinear/` (28 fixture witnesses plus one convergence
  observation note); `tests/test_nonlinear_support_regression.py`.
- R1 indexes: DEL-09-03 inventory row, mapped implementation surfaces,
  BENCH-004, HC-NL-001..029, and relevant decision/ruling records (DEC-023,
  DEC-044, DEC-046, DEC-052, DEC-053, DEC-067).
- Convention-3 homing checks: DEL-04-04 `## Remaining` owns the broader
  non-seed, displacement/reaction/energy, multi-support, and external-threshold
  species with explicit DEL-09-03 cross-references. Neither pending finding ID
  (`PKG09-0903-PKG02-001/002`) appears in any repo-wide `## Remaining` section.

## Re-execution and frozen-tree containment

Before and after the pilot, `git status --porcelain=v1 --ignored=matching --
projects/chirality-piping` in the frozen worktree returned exactly the six
disclosed allow-listed sets and no new path:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The focused suite was re-executed with W4 controls:

`PYTHONDONTWRITEBYTECODE=1 CARGO_TARGET_DIR=<external /tmp dir> python3 -m pytest -q -p no:cacheprovider tests/test_nonlinear_support_regression.py`

Result: **8 passed**. The first pytest case runs the nonlinear crate's **19
Rust tests**, also passing. The crate has a tracked `Cargo.lock`, so the
lockless-crate copy-out rule was not triggered; `CARGO_TARGET_DIR` was external
and removed afterward. No in-tree `py_compile` was used. The ignored-path
porcelain set was byte-for-byte unchanged after execution.

## Encoding judgments and self-flags

1. The eight requirement rows carry substance dispositions. Four setup-era
   kit declarations alone carry `STALE_SETUP_SPECIFICATION`; the historical
   setup acceptance is `ALIGNED` at setup-run grain, following the W3
   setup-run calibration.
2. Addendum-1 census is seven declarations: four-document kit, `_STATUS.md`,
   `MEMORY.md`, and the deliverable-owned nonlinear crate README. The README
   does not spell the deliverable ID in its heading, but DEL-09-03 MEMORY and
   R1 `SURF-227` establish ownership; inclusion is self-flagged for fan-in.
3. Only REQ-003 uses the convention-6 SECURITY sufficiency marker. It is the
   protected-content exclusion behavior whose human sufficiency review is
   genuinely deferred. The marker is not blanket-applied to EXC-002 or other
   SECURITY-class rows.
4. REQ-002 and REQ-006 cite validation evidence in
   `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; addendum 13 therefore caps confidence
   at MEDIUM and routes `AuthorityNeeded=OWNER`. Their `ALIGNED` dispositions
   rest independently on current source artifacts plus the re-executed checks.
5. The two pending human dispositions are stable claims and are encoded
   separately as REM-001/002. Both are `REMAINING_STATE_MISMATCH`: the current
   technical resolution is visible, but the owner act is still TBD and the
   finding is absent from every sole work-discovery `## Remaining` surface.
6. No threshold residual is duplicated into DEL-09-03. DEL-04-04 already homes
   the broader DEC-046/DEC-052 threshold species and explicitly cross-references
   this deliverable. The depth-observation fixture remains policy-TBD without
   undermining the bounded accepted inventories.
7. No `IMPLEMENTED_UNMAPPED` row is warranted. The nonlinear benchmark crate
   is mapped to DEL-09-03; shared solver/product-preview surfaces remain mapped
   at their material-surface owners and are evidence dependencies here.
8. ACC-001 is self-flagged for row grain. It is minted once because the
   Specification has a distinct setup Acceptance Criteria section; mirrored
   requirement-by-requirement acceptance rows were deliberately not created.

No material authority contradiction was found. No lifecycle, DAG, product,
deliverable, decision, or repair surface was modified.
