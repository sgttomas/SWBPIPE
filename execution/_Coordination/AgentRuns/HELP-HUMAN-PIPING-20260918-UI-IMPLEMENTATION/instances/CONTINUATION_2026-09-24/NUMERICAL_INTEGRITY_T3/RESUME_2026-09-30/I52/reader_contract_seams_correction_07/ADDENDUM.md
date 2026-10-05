# RS-F1 — exact raw-row shape and gate precedence

Prospective corrective addendum to sealed reader_contract_seams_06, seal
`118d9be253`. The original bytes remain unchanged. Same I52 TASK; no dependent
code, schema installation or public selection follows from this draft.

**Accept RS-F1.** The earlier promise that null/nonstring or misplaced fields
reach G6 was wrong. Hash recomputation cannot bypass an earlier shape failure.
Replace 06 §2's type/value and expected-gate language with the rules below.
The direct `results[i].recovery_method` path and exact G7 removal remain proposed.

## Exact shape boundary

G1 checks each raw row against the following closed shape. This describes the
actual raw ResultItem boundary, not a new canonical derivative or metadata format.
All existing required keys/optional fields and closed metadata are preserved.
The only added raw-row field is the optional direct recovery_method string.

```text
RawRow = closed {
  id: nonempty string,
  kind: nonempty string,
  value: JSON number,
  unit: nonempty string,
  entity_ref: nonempty string,
  basis_ref?: closed {ref_type: nonempty string, ref_id: nonempty string},
  source_result_refs?: [nonempty string],
  metadata?: closed {
    component: string,
    coordinate_system: string,
    location: string,
    basis: string,
    sign_convention: string
  },
  recovery_method?: string
}
```

`?` means the member may be absent; it never admits null. If metadata is present,
all five fields are required. G1 places **no enum, const or minLength constraint**
on recovery_method. Metadata enum/signature/physical-meaning checks remain with
their existing base validator; this correction does not widen its closed keys.
Raw numeric encoding/canonicalization obligations remain unchanged. Nonfinite or
unhashable input cannot bypass the earlier checked-profile/hash gate.

Accordingly row.evidence is an unknown raw-row key, and metadata.recovery_method
is an unknown metadata key. Either fails G1, even when the direct method field
is also present and correct. There is no alternate container, nested-path search,
metadata deletion or fallback path to satisfy G6. Duplicate keys remain refused
at the applicable existing parsing boundary; no last-key-wins interpretation is
introduced by this shape correction.

## G6 and exact G7 projection

After all earlier gates pass, G6 requires the exact direct string
`contribution_preserving_multiprecision_v1` on every retained-selected row,
including nonquantity rows. On an ordinary/not-required/unavailable row the
member must be absent. Therefore an absent selected token, empty/wrong string on
a selected row, or any string at the direct path on a nonselected row fails G6.

G7 runs only after successful G6. It removes this direct member solely from the
already G6-bound selected rows, plus the previously authorized envelope additions,
then invokes the literal unchanged base validator. No misplaced member is erased
to make the projection pass. The public publication hash binds the unprojected
final rows; all source/freeze obligations remain unchanged.

## Complete first-failure matrix for the correction

For intended semantic mutations, recompute receipt/publication hashes with the
selected checked profile. Unless a row explicitly names a second defect below,
all other earlier gates and the model/source/ordinary bindings must be valid.
Unrehashed publication changes fail the existing G1 hash check before later gates.

| Mutation | First gate / code |
|---|---|
| Direct method=null, number, boolean, object or array | G1 / RETAINED_PRECISION_RECEIPT_MISMATCH (raw shape) |
| row.evidence object, whether empty or containing the token | G1 / RETAINED_PRECISION_RECEIPT_MISMATCH (unknown key) |
| metadata.recovery_method or another unknown metadata key | G1 / RETAINED_PRECISION_RECEIPT_MISMATCH (closed metadata) |
| Correct direct token plus misplaced alternate token/container | G1 / RETAINED_PRECISION_RECEIPT_MISMATCH |
| Absent selected token, empty selected string or wrong selected string | G6 / RETAINED_PRECISION_ROW_METHOD_MISMATCH |
| Any direct string, including empty/wrong/exact token, on ordinary/not-required/unavailable row | G6 / RETAINED_PRECISION_ROW_METHOD_MISMATCH |
| Absent method on a nonselected row | No method failure; remaining gates still apply |
| Correct direct token on a selected row | No method failure; remaining gates still apply |
| Null/nonstring or misplaced method plus invalid canonical Bits (for example nonfinite stop bits) | G1 shape before G2 encoding |
| Empty/wrong/absent selected string plus invalid canonical Bits, with shape and hashes valid | G2 / RETAINED_PRECISION_ENCODING_MISMATCH before G6 |
| Empty/wrong/absent selected string plus well-shaped duplicate/missing case coverage, with G0–G2 valid | G3 / RETAINED_PRECISION_COVERAGE_MISMATCH before G6 |
| Invalid Bits and bad case coverage plus an absent selected token, with G0–G1 valid | G2 before G3/G6 |
| Null/nonstring method plus bad case coverage | G1 before G3/G6 |
| Valid shapes/encoding/coverage but another real G4/G5/G5a/G5b/G5c defect and a bad method string | That earlier gate's selected code before G6 |

An absent basis_ref is shape-permitted by the unchanged optional raw field, but
cannot be used to evade G3's required case/row association. Invalid basis_ref
shape fails G1; a well-shaped foreign/duplicate/missing coverage relation is G3.
These controls distinguish syntax, encoding and ownership from token meaning.
No test may delete a misplaced field or skip earlier checks merely to force G6.

## Source, definition and selection effects

The preserved raw field/metadata shape is grounded in immutable `24af17c470`,
P/core/product_physics/src/lib.rs ResultItem:1976–1988 and ResultMetadata;
G1→G2→G3→…→G6→G7 order is selected C1/C3. The correction changes only the
prospective row-method shape/gate account; the outer-cause proposal and
quantity_kind proposal in 06 remain subject to their independent review.

DEFINITION.json and its domain hash `a7ed7ca0bf` remain unchanged. No new
identity, tolerance, coverage relaxation, source interpretation, standing,
export or interval-binding choice is made. There is no new owner-held numerical
decision; independent review and ROOT technical selection still precede code.
Shared Rust/Python/TS controls must pin this entire matrix, including dual defects.
