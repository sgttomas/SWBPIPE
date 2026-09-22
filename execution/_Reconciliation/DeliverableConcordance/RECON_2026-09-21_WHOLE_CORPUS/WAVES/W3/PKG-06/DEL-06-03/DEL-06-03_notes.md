# DEL-06-03 Required-input completeness checker: worker notes (W3, PKG-06, worker G1)

Forward ledger: 90 rows (65 required keys). `.rNN` splits of CLM-007, CLM-013
and CLM-019. Sub-claims: SOW.s01, CLM-015.s01, CONTEXT .s01 and .s02, and
MEMORY.s01. Sealed in `DEL-06-03_SEAL.txt`. Everything was read from the
evidence checkout at `00115c719`. Agent judgments, not owner rulings.

## Path aliases

- The checker is `core/rules/completeness_checker` (crate
  `open_pipe_stress_completeness_checker`). The product path is
  `core/rules/rule_check_runner`, reached from the desktop
  `apps/desktop/src-tauri/src/lib.rs` `run_rule_checks` command.
- `INIT.md` was removed on 2026-07-04. The bootstrap route is now
  `loop/LOOP_INIT.md` (CLM-007.r01, CP-02).

## Judgment calls

- **Surface rows** follow the DEL-06-02 pattern:
  - SOW → STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE;
  - CONTEXT → setup-era basis;
  - STATUS → CP-05;
  - MEMORY → HISTORY, with the undated "Remaining TBDs" block as .s01.
- **R-DEL-06-03-002 → UNKNOWN, INVARIANT.** Its named verification, a
  protected-content and default-data review of the checker fixtures, was not
  located. The checker itself holds no values.
- **CLM-014 → PARTIALLY_IMPLEMENTED, INVARIANT.** Four of the five named test
  families exist. No test was found that asserts no protected defaults,
  formulas or allowables are shipped. A missing test in the frozen tree can be
  established by search, so this row is PARTIALLY_IMPLEMENTED. A review record
  could exist outside the tree, so those rows are UNKNOWN.
- **CLM-013.r03 (AB-00-06 remediation text) → PARTIALLY_IMPLEMENTED ·
  PROJECT_BASELINE · BASELINE.** Completeness findings carry code, severity,
  input id and message. They have no remediation, class or provenance. This
  matches the DEL-06-02 CLM-006.r04 treatment.
- **CLM-005 (conditions) → ALIGNED.** Provenance gaps block rather than warn,
  which is stricter than the SOW's "surface provenance warning" and still
  meets "do not silently accept". Solve-blocking physical data stays with the
  solver preflight, outside this checker.
- **OUT-001 → ALIGNED, MEDIUM confidence.** `model_input` values reach the
  checker only as caller-supplied bindings. Nothing derives them from the model
  automatically.
- **AC-001** was written at the SOW migration (2026-07-14), after DEC-022, and
  still retains the grammar TBD: STALE_REVIEW_OR_EVIDENCE (FG-01).

## Canonical departures

None.

## Convention friction

F4 flags ALIGNED rows that describe the checker's own "missing" and "absent"
conditions. GAP_WORDING_CHECKED clauses explain each.

## Smallest check for each UNKNOWN row

R-DEL-06-03-002 (FG-DEL-06-03-02): find a protected-content and default-data
review of the checker's test fixtures and the invented demo pack (DEC-058 scan
record or reviewer disposition).

## Reverse pass

- CLAIMED_BY: RC-06-0148 (completeness checker).
- PARTIAL: RC-06-0006 (the runner's completeness stage).
- COVERS: RC-06-0197 (SPEC §7 reference) and RC-06-0257 (analysis-status
  schema, a declared external input).
- Everything else is NOT_MINE, with F5-specific reasons where entry points
  overlap.
- RC-06-0290 (the desktop Rule-check completeness panel) derives its "blocked"
  findings heuristically from status strings. It does not call this checker.
  It is DEL-07-04's surface. It does not change any sealed row, but R3 may
  want to know that two completeness explanations exist.

## Batch consistency

`--batch` over the three sealed ledgers: PASS, 0 findings.

## Other

- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` (C9).
- Rename residue for R3: the crate name carries the former product name. The
  SOW does not name it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
