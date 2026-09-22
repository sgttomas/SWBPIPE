# DEL-09-01 Mechanics benchmark suite — worker notes (W3, PKG-09, worker G1)

Forward ledger `DEL-09-01_forward.csv`: 124 rows, all 74 required keys. It
also includes the `.rNN` rows of the split blocks CLM-003, 004, 006, 008,
015, 024, 032 and 035, plus two `.sNN` sub-claims of the Architecture Basis
Injection block. It was sealed before the routing file was read (see
`DEL-09-01_SEAL.txt`). Recurring-situation treatments are shared with DEL-09-02
and DEL-09-03 through `../_WORKER_DEL-09-01_NOTES.md`.

## Path aliases

- Project-root evidence tokens (`validation/…`, `core/…`, `tests/…`) resolve
  under `projects/chirality-piping/`. Project documents are cited as
  `projects/chirality-piping/docs/…`.
- Deliverable-local records are cited by their full path, which contains
  spaces and commas: `projects/chirality-piping/execution/PKG-09_Verification,
  Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark
  suite/…`.
- The parity records are at the repository root:
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-09-01/`.
- `::symbol` suffixes name the Rust item inside the cited file. `#Lnn` on
  `SOFTWARE_DECOMP.md` points at the §12 row: DEC-018 L609, DEC-026 L617,
  DEC-054 L645, DEC-065 L656 and DEC-070 L661.
- Suite-level test status is cited as
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`. No tests were rerun.

## Judgment calls

1. **Product caller.** `openpipestress-runner` (`core/runner/headless`,
   DEC-065) depends on the mechanics crate and runs its cases through
   `fixture_observations` and `fixture_recorded_comparison_holds`. Suite rows
   therefore have a product caller, so no `PRODUCT_CALLER: NONE` markers are
   needed.
2. **Unit-catalog premise (FG-DEL-09-01-03).** CLM-009, CLM-018, CLM-036,
   AC-001 and Remaining R02 say the canonical unit catalog and conversion
   constants are unresolved or unaccepted. DEC-018 (2026-06-10) accepted
   them, and `core/units` implements them, before these texts were written on
   2026-07-12.
   - Declarations take STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT (MEDIUM).
     R02 takes CP-07 with RULED_CRITERION.
   - The project-grain gap itself is real and stays on RQ-004, CLM-015.r04
     and CLM-024.r04 (FG-DEL-09-01-04, PARTIALLY_IMPLEMENTED ·
     PROJECT_BASELINE · BASELINE). DEL-02-02 still holds the B2/B3 wiring,
     alias-namespace and witness items.
   - Smallest check if contested: the E2/E4/E8 texts of the D-41 R4 slate,
     which sit in excluded July-run material and were not read.
3. **Expansion-loop comparison tolerance (FG-DEL-09-01-06; possible
   defect).**
   - `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` compares at a fixture-local
     5.0e-7 relative constant, with measured reasons in code.
   - That is looser than the DEC-026 analytic seed of 1e-9. DEC-026 makes any
     loosening a governance event, not a fixture-local edit. The witness note
     proposes 1e-9 and says adoption into the governed record is a separate
     step; no governed record was found.
   - Rows CLM-015.r03, CLM-023 and CLM-024.r05 are IMPLEMENTED_DIFFERENTLY ·
     POSSIBLE_DEFECT · PROJECT_BASELINE · RULED_CRITERION · BASELINE;VALIDATION,
     OWNER.
   - An alternative reading is that the case is not analytic-class, in which
     case the per-kind value is TBD. Either way the constant is neither cited
     as approved nor TBD. For the owner.
4. **Result posture (FG-DEL-09-01-01).** Solver version, fixture assumptions
   and provenance are not carried into suite-run outputs; only the
   TP-PHYS-015A fixture carries a solver version. The affected rows are
   PARTIALLY_IMPLEMENTED.
5. **Provenance index (FG-DEL-09-01-02).** The only index is the derivative
   BENCHEVID bundle. It covers 24 of the 25 fixtures and lacks
   MECH-TP-DEC092, added on 2026-08-03. Remaining R01 is ALIGNED with
   `OPEN_ACTION` pointing at CLM-006.r08.
6. **CP-10 (FG-DEL-09-01-05).** The "approved fixture schema and
   result-envelope comparison format" hold (CLM-008.r03, CLM-035.r03) was
   settled in code with no ruling naming it. The runner interface itself is
   ruled (DEC-065).
7. **Open questions still open** (CLM-008.r01 and r04; CLM-035.r01 and r04)
   are ALIGNED as accurate declared state. The release-versus-advisory work
   sits on CLM-032.r04, which is DOCUMENTED_UNIMPLEMENTED (CP-11 reading).
8. **CLM-004.r06 (tolerance posture)** is ALIGNED (MEDIUM). Final tolerances
   and release thresholds are still TBD in RELEASE_QUALITY_GATES §10. Its
   "pending solver prototype" rationale is dated, but that is not an unmet
   element.
9. **Remaining R03** (DEC-054's §16.2 evidence-system residual) has no SOW
   row that carries it, so the Remaining row holds the gap under F2
   (PARTIALLY_IMPLEMENTED · RULED_CRITERION). "PRD §16.2" follows DEC-054's
   numbering; the PRD v0.4 benchmark lists are §22.2–22.3.
10. **Remaining R04** is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED ·
    AuthorityNeeded REVIEW. The human disposition of
    PKG09-0901-PKG02-001 is still TBD.

## Canonical departures

None. All CS rows use the assigned values. The CP-02, CP-03, CP-07, CP-09,
CP-10 and CP-11 rows follow the pattern table. CLM-015.r03, CLM-023 and
CLM-024.r05 carry no CP ID: their cause is POSSIBLE_DEFECT, not the CP-10
pattern.

## Convention friction

- Rename residue in code with no mention in the SOW text.
  - The crate identifier `open_pipe_stress_mechanics_benchmarks` and the
    in-code provenance strings ("OpenPipeStress original mechanics
    benchmark", "OpenPipeStress agentic development workflow") are active
    identifiers that carry the former name.
  - The DEL-09-01 SOW names neither, so CP-04 ("the deliverable, or an
    active code identifier it names") has no row here.
  - Recorded for R3's rename class. The provenance strings also appear in the
    BENCHEVID index.
- **Missing row type.** C6 has no disposition for a Remaining item that is
  accurate but has no governing row. F2's fallback was applied.

## UNKNOWN rows

None.

## Reverse pass

- The reverse pass did not change my view of any sealed row.
- It surfaced `validation/benchmarks/physics_audit_regression`, answered
  PARTIAL. Its R10 run record maps it across DEL-09-01, 09-02 and 09-03. The
  sealed ledger mentions the crate only through MEMORY history, and no sealed
  row depends on it.
- The runner binding (RC-09-0183) is COVERS, not ownership.

## Batch consistency

`--batch` over the three G1 ledgers returned PASS with 0 findings.

## Selectability

SelectableUnderCurrentLoop is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
