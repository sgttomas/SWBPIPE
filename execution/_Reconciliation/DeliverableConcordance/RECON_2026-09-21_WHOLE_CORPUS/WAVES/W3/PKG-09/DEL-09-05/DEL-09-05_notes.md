# DEL-09-05 Release quality gate checklist — worker notes (W3, PKG-09)

The forward ledger is sealed at SHA-256
`50a6756dd20a00ab3bc2933b64f26f48a9aa2c798014d933b2daf31eb57f0989`
(`DEL-09-05_SEAL.txt`). It was assessed against the frozen state `00115c719`.
All evidence was read from the frozen checkout. No build, test or suite was run.

## Path aliases

- `RQG` is `projects/chirality-piping/docs/RELEASE_QUALITY_GATES.md`, the
  deliverable's checklist; its frontmatter says `implements: DEL-09-05`. The
  line anchors used are §1 L17, §2 L29, §3 L44, §4 L69, §5 L91, §6 L107,
  §7 L125, §8 L139, §9 L151 and §10 L165.
- The gate records are the five
  `validation/evidence/gates/GATE_*_20260711T032542Z_e2ea37194c8a.json` files,
  emitted by `tools/release/run_release_gate_records.py` (PR #166,
  TP-E8-GATERECORDS-001). The schema is `tools/release/release_gate_record_schema.json`.
  The tests are in `tests/test_release_gate_records_script.py`.
- The parity records are at the root path
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-09-05/`,
  in commit `3ea904a6e`.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is the suite-level record
  of the Python tests passing. It was not rerun.

## Judgment calls

- **F3 origin.** The SOW text first appears at `7bee9ae41`, carried by the
  migration `380fa0b10`. Stale setup text is therefore
  `STALE_SETUP_SPECIFICATION`.
- **Write scope and "no repo artifacts" (FG-DEL-09-05-01).** Affected rows:
  CLM-003, CLM-005, CLM-011 and CLM-022. The deliverable owns
  `docs/RELEASE_QUALITY_GATES.md`, the gate-record and coverage-telemetry tools
  and their Python tests, and `validation/evidence/gates/`. These landed under
  merged PR #166 and `e9cd80681`. Cause: `SCOPE_GREW_BY_DIRECTION`. For CLM-016
  the dominant cause taken is the four-document representation (CP-01).
- **Accessibility/usability (FG-DEL-09-05-02, `PARTIALLY_IMPLEMENTED`,
  confidence MEDIUM).** Affected rows: CLM-004 and CLM-021.s02. The SOW
  requires GUI accessibility/usability evidence (step 5, thresholds TBD).
  Checklist §6 names only an accessibility *threshold* as TBD and does not
  require that evidence. The GUI gate record carries it as `governed_value_tbd`.
  D-68's bounded accessibility criteria belong to a UI plan with no recorded
  release effect, so the checklist was not taken to be overtaken.
- **Outcome vocabulary (FG-DEL-09-05-03, `IMPLEMENTED_DIFFERENTLY` /
  `DOC_BEHIND_CODE`).** Affected rows: CLM-006 and CLM-021. The SOW defines
  gate-level PASS, FAIL, BLOCKED_TBD and HUMAN_REVIEW_REQUIRED. The
  implementation records a per-criterion `pass`, `fail` or `TBD` with a
  `tbd_reason`, plus status counts, and has no gate-level outcome.
- **Four-document pointers (CP-01, FG-DEL-09-05-04).** Affected rows: CLM-008,
  RQG-001, RQG-009, RQG-011, RQG-012, CLM-013, CLM-014/RQG-001, CLM-015 and
  CLM-016. `DEL-00-08/Specification.md` no longer exists; PKG-00 uses
  `ArchitectureBasis.md`.
- **Section pointers (FG-DEL-09-05-05).** RQG-002 cites SPEC 4.5; numerical
  quality is 5.5. RQG-003 cites SPEC 6; the rule-pack evaluator is 7. RQG-004
  cites SPEC 7; the GUI section is 8. RQG-005 cites SPEC 8; reporting is 9.
  RQG-010 cites AGENTIC_DEVELOPMENT_WORKFLOW §5, which was a numbered "Review
  checklist" at `7bee9ae41` and is now restructured. In each case the substance
  holds in the checklist. Confidence HIGH on the pointer facts.
- **Engineering-beta floor (FG-DEL-09-05-06; RQG-007, CLM-029).**
  `VALIDATION_STRATEGY.md` §4 lost its "must not be labeled engineering beta
  unless" list on 2026-06-07 (`c8748a04a`). Checklist §8 still calls it "the
  governing release-label floor". `AuthorityNeeded` is OWNER, because the
  replacement is a PB-TBD-003 label decision. CLM-014/RQG-007, the check that
  the wording is conditional, is `ALIGNED` because the wording is conditional.
- **Ruled TBDs (FG-DEL-09-05-07; CLM-032, CLM-014/RQG-009, CONTEXT ABI .s02).**
  The following were ruled after setup, yet the SOW still lists them as TBD:
  - CI location and hosted CI (DEC-025, DEC-059, DEC-093);
  - release matrix and attestation (DEC-057, DEC-089);
  - maintainer quorum (DEC-027);
  - coverage tooling (DEC-060);
  - tolerance classes (DEC-024/026, DEC-046).

  Cause: `SCOPE_REDIRECTED_BY_RULING`. CLM-005 states the same TBDs, but
  qualifies them with "unless later approved", so it is judged on its
  write-scope clause.
- **STATUS surface.** CP-05: Last Updated 2026-07-12 is older than the
  2026-07-16 History entry.
- **R01 (PB-TBD-003 label vocabulary).** Disposed `DOCUMENTED_UNIMPLEMENTED`
  with cause `NOT_STARTED`, tier `INVARIANT` and layer `CLAIMS` (F2, F8).
  Confidence MEDIUM. The text is accurate, and no SOW row carries the label
  vocabulary as its gap. `claims_registry.md` retired BS-MATURITY on 2026-09-18
  (DEC-105), which leaves the label vocabulary open.
- **R02 (coverage-floor promotion).** Disposed `DOCUMENTED_UNIMPLEMENTED` with
  cause `DEFERRED_BY_RULING`, tier `PROJECT_BASELINE`, baseline
  `RULED_CRITERION` and layer `BASELINE`. One telemetry artifact exists, so the
  DEC-060 trigger is not met.
- **R03 (W1–W7 issuance packets).** Disposed CP-07 `REMAINING_STATE_MISMATCH`
  with baseline `RULED_CRITERION`. D-40 (DEC-072, ruled 2026-07-11) moved the
  project's only CHECKING deliverables to IN_PROGRESS, and no deliverable is
  CHECKING at the freeze.
- **MEMORY.** Disposed `HISTORY` and `ALIGNED`. `MEMORY.s01` holds the undated
  D-41 current declaration at the top of MEMORY, which pins 0.8 and DAG-007
  (C1 undated-declaration rule; CP-03). MEMORY has no July entries for the
  TP-E8 work; that work is in the run records. This is an omission, not an
  inaccuracy.
- **CONTEXT anticipated artifacts.** Disposed `ALIGNED`, confidence MEDIUM.
  The "CI quality gates" are realised as commit-bound gate records over the
  DEC-025 sweep, which DEC-025 makes the merge gate in place of hosted CI.
- `.rNN` blocks were not split. The only splits are the `.sNN` rows on
  CLM-021, where steps 5 and 6 take different dispositions, CONTEXT ABI and
  MEMORY.

## Canonical departures

- `CANONICAL_DEPARTURE` on RQG-002, RQG-003, RQG-004, RQG-005 and RQG-010. This
  is the same F3-over-CP-02 reading as for DEL-09-04: setup-origin section
  references are `STALE_SETUP_SPECIFICATION`.
- The keyed CS rows follow `CANONICAL_ASSIGNMENTS.csv` exactly.

## Convention friction

- The CP-02 / F3 overlap is the same as for DEL-09-04.
- Some implementation drift has no DEL-09-05 key. It is recorded here for R3:
  - Checklist §8 relies on the removed VALIDATION_STRATEGY engineering-beta
    condition.
  - Checklist §10 still says hosted CI is "re-decided at D-05b"; D-05b was
    ruled on 2026-07-04 as DEC-059, and D-65/DEC-093 later accepted a CI path
    for surface 4.
  - The gate-record artifact identifier `openpipestress.release_gate_record` is
    an active identifier carrying the former name. It is not named by the SOW,
    so it is not on the CP-04 row.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass (368 capabilities: 2 CLAIMED_BY, 1 UNKEYED, 3 COVERS,
362 NOT_MINE) did not change my view of any sealed row.
- The coverage-telemetry seam (RC-09-0065) was built by a DEL-09-05 run, but
  no key claims it. It is answered UNKEYED, with nearest key CLM-032.
- Release-readiness tooling (RC-09-0211) and the DEC-025 sweep (RC-09-0244)
  belong to build and CI work. The sweep is COVERS.
- Every NOT_MINE on a capability whose path this ledger cites has its own
  specific reason (F5).

## Batch consistency

`validate_ledger_v2.py --batch` over the DEL-09-04 and DEL-09-05 forward
ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping has selected work through owner-steered work graphs, not
through `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
