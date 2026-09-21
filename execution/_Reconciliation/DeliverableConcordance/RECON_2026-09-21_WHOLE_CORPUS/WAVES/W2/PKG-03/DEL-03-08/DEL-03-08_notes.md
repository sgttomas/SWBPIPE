# DEL-03-08 notes — gate wave W2, PKG-03, worker G3

Deliverable: Pipe section property and mass-property calculator. Forward
ledger 124 rows (66 required keys, 7 canonical assignments inherited). Sealed
forward SHA-256
`c50942fd8f47d8f9911492d32908e2ca263b45531a6ad466f898762d46c1fe78`. Not an R0
pilot.

## Path aliases

- The calculator is `core/section_properties/calculator.py` with its README.
  Tests are `tests/test_section_properties.py` and
  `tests/test_calculation_witness.py` (the PDU-047 binding).
- The witness is
  `validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json`.
  The governed envelope is
  `fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json`.
- The parity records are under the repository-root
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`.
- Deliverable-folder files have spaces in their paths and are cited in
  `ContextRefs` (as in DEL-03-07).
- The product runtime's own section and mass routine is
  `core/product_physics/src/lib.rs` (`derive_pipe_section`,
  `compute_pipe_mass_per_length`). Its doc comment says it mirrors the Python
  calculator.

## Judgment calls

- **F7 throughout**: the Python calculator has no product caller. Its only
  callers are its tests and the witness test. Every `ALIGNED` row citing it
  carries `PRODUCT_CALLER: NONE`. The claims are about the calculator module,
  so the engine satisfies them. The one product-level question is routed in
  the reverse pass (RC-03-0091).
- **FG-DEL-03-08-01: review findings declared pending** (CLM-006.s01,
  CLM-007.r05, CLM-014.s01, CLM-015, CLM-021.r05, CLM-030, CLM-031.s01;
  `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT`). All three findings are
  `ACCEPT_AS_IS / RESOLVED` (human Gate A, 2026-06-05). CLM-015 and CLM-030
  are not split because their three rows share one state. CLM-028.r05 stays
  `ALIGNED` because its rule ("mark nothing resolved without human action")
  held.
- **FG-DEL-03-08-02: unit catalog and conversion "TBD" overtaken by DEC-018**
  (CLM-003.r04, CLM-004.s01, CLM-007.r01, CLM-013.r01, CLM-019.r02,
  CLM-021.r02, CLM-026.s01, AC-001; `STALE_REVIEW_OR_EVIDENCE ·
  SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN`, MEDIUM). DEC-018 (2026-06-10)
  accepted the unit catalog, and `core/units` implements it. The text was
  written on 2026-06-04. Mixed-unit rejection is still true, and whether the
  calculator should bind to the catalog is an implementation step.
  AC-001 is also stale on the review element; the ruling element was taken as
  the cause.
- **RQ-003**: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · BASELINE`,
  MEDIUM. The calculator checks dimension labels and unit-string equality,
  but it never checks a unit against its declared dimension:
  `Quantity(10, "kg", "length")` passes and yields "kg^2" labelled area. The
  tier and layer choice matches DEL-03-07 FG-02.
- **FG-DEL-03-08-03: provenance and redistribution** (RQ-004, CLM-026;
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · IP_DATA`, MEDIUM).
  Inputs need a non-empty provenance mapping, but redistribution status is
  never inspected. Derived outputs carry only the calculator's provenance, so
  input provenance is not carried through. CLM-026's first principle makes an
  unlicensed input a blocking condition, and the code has no such check.
- **RQ-002**: `ALIGNED`. OD and wall are required. An insulation density
  supplied without an insulation thickness is silently left out of mass per
  length. I treated that as falling under the declared open policy on
  optional contributor requiredness, not as a silent default. The verifier may
  weigh it.
- **RQ-005 and CLM-021.r01**: `ALIGNED`, MEDIUM. The fixtures are visibly
  invented (round values, "invented" provenance). An agent review (TASK run
  2026-05-17) and my own reading agree. No human IP review was located. Unlike
  the W1 DEL-07-02 case, the claim is about the fixtures' content, which can
  be read directly, rather than about the existence of a review.
- **Witness evidence** (CLM-008, CLM-016, CLM-023, CLM-032): the witness is
  agent-authored with in-file `review_status: accepted`, and no human
  disposition was located, so `SourceReliability UNVERIFIED`. The SOW itself
  disclaims engineering validation. No row asserts engineering correctness, so
  there is no `VERIFIED_NOT_VALIDATED` row. The witness covers area, section
  modulus and torsion constant; mass-per-length has unit tests only.
- **SOW surface**: CP-04 rename residue (CLM-025 "OpenPipeStress
  data-boundary rules"), `AuthorityNeeded OWNER`. The row also records the
  calculator's runtime provenance strings "OpenPipeStress section property
  calculator" and contributor "OpenPipeStress", which are active code text.
  The frontmatter pin (commit `69ac259a`, revision 0.8) is on `SOW.s01`
  (CP-02).
- **CP-01 four-document residue**: CLM-020.s01 (text from 2026-06-04, later
  declaration) and CLM-022.s01 (text from `7bee9ae41`, setup origin, marked
  `CANONICAL_DEPARTURE` to explain the class difference within CP-01).
- **STATUS surface**: CP-05. Last Updated is 2026-07-12, but the newest history
  entry is 2026-07-16.
- **Parity (CP-09)**: every PASS record hashes `623a3269…`, while the frozen
  SOW hashes `61d42a4b…` (the DEC-081 edit of 2026-07-16). VER-001 and the
  matrix OUT-001 row are `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN`. I
  kept purpose OUT-001 `ALIGNED` on substance: the contract exists and covers
  the named topics.
- **CONTEXT#architecture-basis-injection .s01/.s02**: same treatment and
  reasons as DEL-03-07 (shared body across PKG-03).

## Canonical departures

None from `CANONICAL_ASSIGNMENTS.csv`. For CP-01 on CLM-022.s01, the pattern
allows either stale class by origin; the `CANONICAL_DEPARTURE` note explains
the origin-driven difference.

## Convention friction

- Unit invariant has no C5 layer (BASELINE used).
- Spaces in deliverable paths (ContextRefs only).

## UNKNOWN rows

None.

## Reverse pass

Answers for 376 routed capabilities: 2 `CLAIMED_BY` (RC-03-0175 calculator,
RC-03-0338 input validation and diagnostics), 2 `PARTIAL` (RC-03-0033: only
the section_properties README; RC-03-0326: the calculated-property slots in
the section schema, which MEMORY lists as a DEL-03-08 surface), 1 `UNKEYED`
(RC-03-0091) and 2 `COVERS` (RC-03-0225 witness, RC-03-0034 result fixtures:
the PDU-047 envelope). The rest are `NOT_MINE`, with specific reasons wherever
paths overlap (CONTRACT, registers, units README, product_physics RC-03-0207)
(F5).

**RC-03-0091 (for R3 and the owner)**: SOW-051 ("calculate pipe section and
mass properties from user-entered dimensions and material data") is assigned
only to DEL-03-08. The product runtime's section and mass derivation in
`core/product_physics` is a separate Rust implementation that cites the
Python calculator as its model. It omits corrosion allowance, which the
calculator and the DEL-03-08 description include. No DEL-03-08 key covers a
runtime implementation. I answered `UNKEYED`; the preview-physics deliverable
is the other plausible owner.

Did the reverse pass change my view of anything sealed? No. RC-03-0091 is
consistent with the sealed `PRODUCT_CALLER: NONE` markers and the sealed
"downstream integration TBD" declarations, which stay accurate for the Python
module.

## Batch consistency

`--batch` over DEL-03-07 and DEL-03-08: PASS, 0 consistency findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` (C9). Since 2026-09-19 Piping
selects work through owner-steered work graphs, not `## Remaining`.

These dispositions are agent judgments, not owner rulings. No release,
approval, compliance or certification claim is made or implied, and no
unreviewed external equation artifact is cited (DEC-043).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
