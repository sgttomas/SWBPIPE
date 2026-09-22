# DEL-11-04 notes — Invented educational example models (W3, PKG-11, worker G2)

Forward ledger sealed at `e3f5de210cf97f2c0774350439a4dab0c4f964a44fda428087e14faa1cbfb62b`
(107 rows: 69 required keys, 32 optional `.rNN` rows for the split blocks
CLM-003, CLM-004, CLM-007 and CLM-020, and 6 `.sNN` sub-claims). Validator
with `--notes-gap`: PASS, 0 findings. Not an R0 pilot. All values are agent
judgments, not owner rulings.

## Path aliases

- `examples/models/invented/mechanics_only_toy_span.json` and
  `fake_rule_pack_toy_model.json` are the materialized "future example model
  files" of the setup text (commit `abdecbd`, 2026-05-04; schema alignment
  `bdc6d0df7`; checksum maintenance `c60f3af4c` and later restamps).
- `tests/test_invented_example_models.py` is the fixture check. It ran in the
  gate sweep (log line 1947 of `GATE_EVIDENCE/B4_4_SWEEP_9D55/sweep.log.gz`)
  and the Python suite passed (`SUMMARY.json`). Not rerun.
- Project `INIT.md` (cited by CLM-007.r01 and CLM-020.r02) was removed by
  `9c4caf8fd` on 2026-07-04. The four-document kit named in CLM-015/016/022/023
  was replaced by `ScopeOfWork.md` at the 2026-07-14 migration (`4f2c64cef`).

## Judgment calls

- **Setup-era text** is judged by F3. Text present at `7bee9ae41` (checked
  with `git log -S`) that says the deliverable creates no example files, or
  that treats the fixtures as future work, is `STALE_SETUP_SPECIFICATION` ·
  `DOC_BEHIND_CODE` (FG-DEL-11-04-02). This covers CLM-004.r01/r04,
  CLM-006.s01, CLM-010, R-007, CLM-019, CLM-021.s02, CLM-030 and CF-001/002.
- **Status and readiness fields** (CLM-003.r09 "Current production mode",
  CLM-020.r03/r04 "Future dependency") keep `STALE_REVIEW_OR_EVIDENCE`
  under the F3 exception for review and readiness states.
- **R-DEL-11-04-002 is `PARTIALLY_IMPLEMENTED`, INVARIANT (professional
  boundary), CLAIMS.** The fixtures say invented, non-code, non-project,
  educational, not suitable for engineering reliance, and (mechanics) not a
  design basis. They do not name certification, approval, sealing or code
  compliance, which the requirement lists explicitly. Confidence is MEDIUM.
  CLM-032 asks only for the notice "in substance", so it is ALIGNED.
- **R-DEL-11-04-003 is ALIGNED (MEDIUM).** The mechanics fixture's result
  values are invented toy values, not solver output. The claim asks for
  unit-aware reproducibility. That is shown by the JCS project hash and the
  persistence round-trip test, and CLM-028 permits toy values.
- **CLM-005 and CLM-006.s02 are ALIGNED (MEDIUM).** Conditions that did not
  arise (real standard examples, commercial comparison) are not treated as
  met by construction, because the governed behaviour (authoring examples)
  exists and followed the rules that applied. The "protected-content review
  record" element is met by the Tranche A focused scans recorded in MEMORY and
  by the PKG-02 audit. Neither is legal clearance, and no row claims one.
- **CONTEXT#anticipated-artifacts is `PARTIALLY_IMPLEMENTED`.** No tutorial
  for the invented examples exists. The user guide mentions the folder in one
  sentence. MEMORY records tutorial integration as deferred, not ruled.
- **STATUS#remaining/R01 is `DOCUMENTED_UNIMPLEMENTED` · NOT_STARTED ·
  REVIEW.** This follows F2's second branch. Both PKG-02 findings keep
  `HumanDisposition=TBD`, and no governing row in this ledger carries the
  open action.
- **Architecture Basis Injection** adds `.s01` for the stale "PKG-00 at
  SEMANTIC_READY" statement (all eight PKG-00 deliverables are IN_PROGRESS) and
  `.s02` for the Still TBD list, which still names the rule grammar settled by
  DEC-022.
- **FindingGroups:** 01 is the D-41 declarations plus MEMORY; 02 is setup-era
  text; 03 is the notice wording; 04 is the parity rows (AC-001, VER-001,
  matrix OUT-001); 05 is the INIT.md pointers.
- **PRODUCT_CALLER: NONE** is marked on every ALIGNED row that rests on the
  fixtures. No product code loads `examples/models/invented/`. A fixture with
  no product caller satisfies claims about the fixture's own content (F7).

## Canonical departures

- CLM-007.r01 and CLM-020.r02 (INIT.md pointers) use CP-02 with
  `STALE_SETUP_SPECIFICATION` rather than CP-02's `STALE_REVIEW_OR_EVIDENCE`.
  F3 assigns setup-origin text that is not a pin, review state or metadata to
  `STALE_SETUP_SPECIFICATION`. Each row carries `CANONICAL_DEPARTURE:`.
- No CS assignment was departed from. All 7 keyed rows inherit their
  situation.

## Convention friction

- CP-02's class conflicts with F3 for removed-file pointers that date from
  the initial migration. The conventions give no tie-break for CP rows, so
  F3 was applied.
- CP-04 asks that rename residue be recorded only on the SURFACE row. Here the
  residue also sits in the fixture data (provenance `source_name` and
  `contributor` strings "OpenPipeStress project"). That is recorded in the SOW
  SURFACE notes. It is not an active code identifier of the four listed
  kinds.
- The SOW SURFACE row can carry only one cause. CP-04 (rename) was chosen.
  The setup-era body is carried item by item under FG-DEL-11-04-02.

## UNKNOWN rows

None.

## Reverse pass

267 capabilities answered. One is `CLAIMED_BY`: RC-11-0015, the invented
example models. Two are `COVERS`: RC-11-0207, the DEL-06-05 demo rule pack
the fake-rule fixture references, and RC-11-0176, the example-data policy
notice. The other 264 are `NOT_MINE`. Thirteen capabilities overlap paths
cited in this ledger, 11 of them answered `NOT_MINE`. Each has a
capability-specific reason (F5), as do five non-overlapping neighbours
(invented fixtures and linter fixtures). The overlapping ones are governance
docs cited as references, the registers, the model, persistence and
rule-pack schemas, and the user guide.

The reverse pass did not change my view of any sealed row. It confirmed
that no tutorial capability exists for the invented examples: the user guide
(RC-11-0173) is a separate deliverable's artifact.

## Batch consistency

`--batch` over DEL-11-04 and DEL-11-05 forward ledgers: PASS, 0 consistency
findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping
selects work through owner-steered work graphs, not `## Remaining`.

## Protected checks, invariants, ISSUED, authority conflicts

- No protected check was removed or weakened. No ISSUED artifact is involved.
  No AUTHORITY_CONFLICT row.
- One INVARIANT row: R-DEL-11-04-002 (professional boundary, notice wording).
- Possible defect for the owner: the fake-rule fixture's embedded report
  snapshot (`project.reports[0].rule_pack_refs[0]`) still pins pack version
  0.1.0 with a TBD checksum, while the project-level reference is 0.2.0 with a
  concrete digest. This is explicit, not silent, so R-005 stays ALIGNED. It is
  still an inconsistency inside the fixture.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
