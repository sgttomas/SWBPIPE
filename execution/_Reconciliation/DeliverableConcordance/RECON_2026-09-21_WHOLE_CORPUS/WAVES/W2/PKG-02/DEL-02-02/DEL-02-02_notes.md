# DEL-02-02 notes — Unit system and dimensional-analysis core contract (W2, PKG-02)

Forward ledger: 119 rows. It has 101 required keys, the 15 `.rNN` rows of
CLM-015, and 3 sub-claims. It is sealed in `DEL-02-02_SEAL.txt`. Frozen
state is `00115c71931bcae79909602d653740d3bb72dfa1`. These are agent
judgments, not owner rulings.

## Path aliases

- `DEL02-02/` in `NormativeSource` means
  `projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/`.
  Run records in that folder appear only in ContextRefs, because the path
  has spaces.
- The unit crate is `core/units` (crate name `open_pipe_stress_units`). Its
  product callers are `core/product_physics`,
  `core/model_operations/operation_applier`,
  `core/rules/rule_check_runner` and the desktop backend. For that reason,
  no aligned row needed `PRODUCT_CALLER: NONE`.

## Judgment calls

- **DEC-018 overtakes most setup TBDs.** DEC-018 (2026-06-10) accepted:
  - the SI-canonical catalog;
  - the dual display catalog;
  - definitional constants;
  - absolute and interval temperature;
  - gauge and absolute pressure;
  - the two-tier tolerance policy.

  The crate implements these. So the setup-era TBD and ASSUMPTION text in
  CLM-004, 005, 006, 013, 025, 026, 027, 037, 038, 043 and 044, U-004, and
  CLM-015 r01, r04, r05, r06, r09, r13, r14 and r15 is
  STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING. Its origin at
  `7bee9ae41` was confirmed by `git log -S`.
- **CLM-015 split.** The open decisions have different states, so the block
  is split all-or-none.
  - Still open, accurate TBDs: r02 (namespace and aliases) and r12
    (diagnostic codes). These are DOCUMENTED_UNIMPLEMENTED · NOT_STARTED ·
    OWNER, because the required human decision has not been taken.
  - Settled in code without a located ruling: r03 and r07 (QuantityKind) and
    r11 (schema layout). These follow CP-10.
  - r08 (angle and rotation) is PARTIALLY_IMPLEMENTED.
- **FG-DEL-02-02-01: persistence hash basis.** AB-00-04, DEC-010 and DEC-017
  make canonical JSON with JCS-compatible canonicalization the basis for
  JSON payload hashes. SPEC says model states use that basis. The Python
  persistence service hashes sorted compact ASCII-escaped JSON, labels it
  `SORTED_COMPACT_JSON` and disclaims JCS. The desktop Rust path uses
  RFC 8785.
  - I found no ruling that relaxes the baseline. The D-41 R5 T2A relabelling
    is a record, not an amendment.
  - Rows in this group take IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
    PROJECT_BASELINE · FROZEN_CONTRACT · RECORD;BASELINE · OWNER. They are
    U-008, CLM-015.r10, CLM-040, CLM-046 and
    CONTEXT#architecture-basis-injection.s03.
  - These rows restate or steer toward the non-JCS basis. The rows that only
    describe or test the honest labels (CLM-008, CLM-019, CLM-029,
    CLM-017/U-008) are ALIGNED.
  - This is an owner item for R4.
- **U-001 PARTIALLY_IMPLEMENTED.** The crate is wired at the solver,
  operation and rule-check boundaries. But the model schema's `Quantity.unit`
  is a free string with no catalog binding and no check that unit and
  dimension agree.
- **U-002 UNKNOWN.** Adapter-wide unit validation was not traced.
- **U-010 and U-016 PARTIALLY_IMPLEMENTED (OWNER).** The diagnostic-code
  namespace and the alias policy have not been selected.
- **U-012 ALIGNED.** Tests exist in every named category. This is
  verification only.
- **CLM-018 PARTIALLY_IMPLEMENTED.** There is no schema-location decision
  record and no diagnostic-code decision record. SPEC §4 still gates
  conversion constants that DEC-018 accepted.
- **Four preserved four-document frontmatter blocks** (CLM-001, 011, 022,
  032) take CP-01 with the metadata class STALE_REVIEW_OR_EVIDENCE (F3
  exception). The DEL-02-01 and DEL-02-03 CP-01 prose rows are
  STALE_SETUP_SPECIFICATION. Batch mode groups patterns by disposition, so
  no conflict arises.
- **Remaining items:**
  - R02, R03 and R04 are ALIGNED with OPEN_ACTION pointing to U-001, U-016
    and U-010 respectively.
  - R05 (independent conversion witness) is DOCUMENTED_UNIMPLEMENTED ·
    VALIDATION_GAP · INVARIANT · ENGINEERING. No governing row requires
    validation, so F2 gives the gap to the Remaining row.
  - R01 is UNKNOWN (see below).
- **CLM-010, CLM-021, CLM-031 and CLM-048 (PDU-054 declarations)** are
  ALIGNED under CP-03. They pin no superseded revision, and their
  delegation to residuals is not relied on (A4).

## Canonical departures

None. CS-06 (package reference) is inherited as DRIFT and the other CS rows
as assigned.

## Convention friction

- Schema definition names that contain gap words (for example
  `MissingInputFinding`) trip the F4 scan. I wrapped them in backticks,
  which the scan strips as identifiers.
- F2 does not say how to treat a conditional Remaining action ("before any
  PDU-048 validation upgrade"). I treated it as open.

## Smallest check for each UNKNOWN row

- `SOW#CLM-014/U-002`: list every import/export adapter and plugin entry
  point, and confirm that each routes units through `core/units` or
  `core/units/schema_vocabulary.py`.
- `STATUS#remaining/R01`: find the definition of the PDU-037 matrix, then
  compare each strand with the present tests: rule-check unit mismatch,
  persistence hash and round trip, and PCF/CAEPIPE units.

## Reverse pass and the sealed forward ledger

The reverse pass gave 6 CLAIMED_BY, 2 PARTIAL, 5 COVERS and 241 NOT_MINE.

**It changed my view of one sealed row.**
- `SOW#CLM-020` (T2B requirement) is sealed ALIGNED. The requirement says:
  "Python adapter/application validation SHALL derive the canonical
  DimensionId vocabulary … rather than maintaining a parallel literal set."
- Routing note RC-02-0188 led me to `core/gui/pkg02_boundary.py`. It keeps a
  literal `CANONICAL_DIMENSIONS` tuple and uses it for dimension membership
  checks in the Python GUI record builders.
- That is a parallel literal set in Python application validation. On this
  evidence the row should likely be PARTIALLY_IMPLEMENTED · PARTIAL_SLICE.
  I would correct it; the verifier or a fresh worker should decide.
- The same finding weakens the ALIGNED guidance row CLM-047 only as
  context. CLM-047 itself warns against inferring closure.

I did not edit the sealed ledger.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed PKG-02 W2 ledgers:
**PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
