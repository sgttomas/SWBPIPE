# DEL-02-03 notes — Code-neutral analysis boundary model (W2, PKG-02)

Forward ledger: 71 rows. It has 56 required keys, the 12 `.rNN` rows of
CLM-010, and 3 sub-claims. It is sealed in `DEL-02-03_SEAL.txt`. Frozen
state is `00115c71931bcae79909602d653740d3bb72dfa1`. These are agent
judgments, not owner rulings.

## Path aliases

- `DEL02-03/` in `NormativeSource` means
  `projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-03_Code-neutral analysis boundary model/`.
- Main artifacts:
  - `schemas/analysis_boundary.schema.yaml`;
  - `projects/chirality-piping/docs/architecture/code_neutral_analysis_boundary.md`;
  - `fixtures/analysis_boundary/`;
  - `tests/test_analysis_boundary_schema.py`;
  - SPEC §4.3.
- Product consumers: `core/solver/diagnostics` (maps onto boundary classes),
  the desktop `MissingDataBlockingPanel`, and the Python GUI contract
  modules. So no aligned row needed `PRODUCT_CALLER: NONE`.

## Judgment calls

- **FG-DEL-02-03-01 (CP-10): status axes and field layout settled in code.**
  The SOW says two things:
  - `analysis_status` is one coarse enum;
  - splitting it into solve, rule and human axes needs the human project
    authority (CLM-004 ASSUMPTION, CLM-010 R12, CLM-026, CLM-027).

  The implemented boundary schema has no single `analysis_status` field. It
  carries `mechanics_solve.status` and `user_rule_check.status` on separate
  objects, plus external `human_acceptance_refs`, and fixes the path and
  field layout.
  - I found no ruling that accepts this.
  - The rows in this group are CLM-004.s01, CLM-005, CLM-010.r12, CLM-026
    and CLM-027. They take IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
    PROJECT_BASELINE · OWNER.
  - The single `software_status` field lives in DEL-05-04's
    `schemas/analysis_status.schema.yaml`.
  - Confidence is MEDIUM. A reviewer may read the per-boundary objects as the
    "separate evidence fields" that the SOW's own interface table anticipates.
- **CLM-003 PARTIALLY_IMPLEMENTED (INVARIANT, CLAIMS;BASELINE).** Two parts
  are unmet:
  - The boundary `Diagnostic` has no machine-readable diagnostic code, which
    the AB-00-06 envelope requires. Affected object and remediation are
    optional.
  - Nothing marks a human acceptance reference stale when its bound content
    changes (OPS-K-AUTH-2). No product path creates acceptance records yet,
    so the no-survival rule holds only by construction (CP-11).

  The tier follows the professional-boundary gap (F8).
- **CLM-011 PARTIALLY_IMPLEMENTED (PROJECT_BASELINE).** The same missing
  diagnostic code affects its "diagnostic-backed status" minimum.
- **CLM-013 PARTIALLY_IMPLEMENTED.** V11 has no evidence that no unvalidated
  status write path exists. V05 and V06 conflict with the layout settled in
  code.
- **CLM-010.r10 UNKNOWN.** I did not trace adapter, plugin or API
  status-write paths. This is consistent with DEL-02-01 REQ-02-01-10 and
  DEL-02-02 U-002, which have the same cause, tier and layer.
- **CLM-025 (vocabulary) ALIGNED.** Desktop source outside tests names
  `HUMAN_APPROVED_FOR_PROJECT` only in a rule-check service comment, which
  says the record cannot be produced there.
- **CLM-007 and CLM-015 (PDU-054 declarations)** are ALIGNED under CP-03.
- **Architecture basis injection.** `.s01` and `.s02` are handled as in
  DEL-02-01. The body is shared: SharedTextCount 18, the same text as
  DEL-02-01.
- **F3.** Setup-era strings date from `7bee9ae41`. The revision 0.7 pins date
  from `1f4de36d9` (2026-06-03) and stay as CP-02 pins.
- **SOW SURFACE.** Its text does not name the former product. I recorded the
  schema's `$id` on openpipestress.org only as an R3 observation, because
  the SOW does not name that identifier.

## Canonical departures

None. CS-06 (package reference) is inherited as DRIFT and the other CS rows
as assigned.

## Convention friction

- `.rNN` splitting of CLM-010 was needed because the requirement rows differ
  (R10 is UNKNOWN; R12 is CP-10). Other large tables (CLM-003, CLM-011,
  CLM-013) were assessed directly to keep one disposition per block. That
  hides which individual rows are met; the Notes list them.

## Smallest check for each UNKNOWN row

- `SOW#CLM-010.r10`: enumerate every adapter, plugin and API path that can
  write an analysis status, and confirm each validates against the boundary
  schema before the write.

## Reverse pass and the sealed forward ledger

The reverse pass gave 3 CLAIMED_BY, 1 PARTIAL, 3 COVERS and 247 NOT_MINE.
It did not change my view of any sealed DEL-02-03 row. Routing note
RC-02-0244 confirms that solver statuses map only to incomplete or
mechanics-solved under solver authority, which is consistent with
CLM-010.r03.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed PKG-02 W2 ledgers:
**PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
