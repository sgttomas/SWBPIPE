# Three narrow C3 wire seams — proposal for ROOT/RV review

Same I52 TASK under ROOT. This prospective addendum changes no sealed packet,
maintained code, numerical method or public standing. It is required before the
affected reader/producer mappings can be implemented without inventing a default.
Owning source is immutable `24af17c470`; C3 plus F1 remain selected except for
these explicitly proposed completions. P = projects/chirality-piping.

## 1. Outer unavailable-case cause references actual C3 failure

**Gap:** C2 CONTRACT_DELTA §2 closes preparation causes to SourceError or an
unavailable precondition; it closes facade causes to the earlier recipe/check
map. C3 ProductAttempt.result contains precise PublicFailure, including actual
SectionPreparationError, but supplies no corresponding outer cause branch.
AmbiguousRounding/PrimitiveRange/Accounting must not become an invented
SourceError, source_family trigger or parsed diagnostic string.

**Proposed exact addition to C2's case.unavailable.reason.cause union:**

```text
{kind:"prepared_product_failure",product_attempt_ref:U}
```

This branch is permitted only when the containing case's product_attempt_ref is
the same U, the referenced ProductAttempt belongs to that case and its result is
`{kind:"unavailable",error:PublicFailure}`. It references that exact error; there
is no duplicate error payload or free-text override. It cannot reference a Ready
attempt, another owner, an absent attempt or a combination. Existing C2 cause
branches remain unchanged for outcomes without an actual C3 product attempt.

The following mapping is exhaustive for this new branch; code/phase mismatch is
an invalid receipt, not an alternative interpretation. Every Run reference must
resolve to the same actual C2 Run in the case, ProductAttempt and source graph.

| Referenced actual PublicFailure | Exact outer code / phase | Required evidence |
|---|---|---|
| preparation{capture,section} | source_unavailable / preparation | preparation stage failed; no native Run was entered for this attempt. Preserve actual optional section error, original capture error, complete/prefix old inventory and all entered work. |
| native{run_ref} | kernel_unresolved / kernel when terminal=unresolved; kernel_refused / kernel when terminal=refused | Non-null same Run, its actual terminal/reason/work; native selected is invalid for this error. |
| capture{cause}, with no Run | source_unavailable / preparation | Actual refusal before any recorded native run, including capture/preparation/origin admission failure. No invented zero-work Run; source may exist if its valid construction completed. |
| capture{cause}, with a nonselected Run | kernel_unresolved or kernel_refused / kernel, exactly as above | The actual nonselected Run is retained. This does not infer the native reason from the CaptureError text. |
| capture{cause}, with a selected Run | facade_certificate / facade | Actual post-native association/adapter refusal; a proof need not have started. Preserve the real stage prefix instead of asserting a certificate ran. |
| proof, values, abandoned, numeric, observable, g5a | facade_certificate / facade | Same selected native Run; actual respective proof/value/check/capture errors and entered work retained. Existing stage consistency still applies; facade code does not itself assert final-certificate execution. |

Here source_unavailable names a failure to prepare the source for this product
attempt; the new cause explicitly distinguishes it from native SourceError.
facade_certificate remains the selected outer facade-unavailability family; the
referenced discriminant/stages distinguish failures before certification. No
new outer code, numeric trigger, scope refusal or availability exception is added.
Do not infer code/phase by inspecting an Association string or by counting arrays.

If the native source constructor also fails, existing source_decline keeps its
actual SourceError and constructor/input-owner facts; the new cause reference
does not replace or counterfeit that record. If an encoder/hash/publication fails
after a Ready private product, use the existing C2 receipt_failure and ordinary
transaction rules. Do not mutate Ready into an invented C3 unavailable attempt
to gain this branch. Native-only and not-required cases retain C2 behavior.

**Reader order:** G1 validates the closed branch; G2 validates its U encoding;
G3 validates case/attempt owner and coverage; G4 retains selected/unavailable
diagnostic exclusivity; G5 validates the same-Run references, actual terminal,
code/phase table and C3 stage/error/work consistency. Use the already selected
receipt, encoding, coverage, diagnostic and PRODUCT_ATTEMPT mismatch codes for
their respective gates. No new failure code is needed.

## 2. One exact per-row method path

**Gap:** C1 G6 and D2 §4.9.2 require a method token “in its evidence”, while
current P/core/product_physics/src/lib.rs:1976–1988 ResultItem has no evidence
or recovery_method member. The old D1 line citation does not specify a current
closed JSON row path.

**Proposed path:** `results[i].recovery_method`, a direct optional string member
of the successor raw row, with the sole allowed value
`contribution_preserving_multiprecision_v1` when present.

- Every row whose basis is a retained-selected case (including nonquantity rows)
  has this exact scalar. Ordinary/not-required/unavailable rows have **no member**;
  null, empty, wrong type/value and a token on an unselected basis fail G6.
- Existing retained-selected combination requirements are unchanged, but this
  amendment does not create a prepared combination capability or a new identity.
- G6 binds row identity, basis owner and the method token before G7. G7 removes
  **only this direct member from the G6-bound selected rows**, together with the
  already authorized receipt/identity/profile additions. It removes no metadata,
  evidence object, unrelated field or row value; existing base row/metadata
  validation runs literally on the resulting projection.
- Putting the token at metadata.recovery_method, evidence.recovery_method or
  another location does not satisfy G6. No alternative-path search or “method
  anywhere” heuristic is permitted. Historical rows gain no field or new reading.
- The public candidate must pre-stage and bind this member under the complete
  finalization/freeze contract. It is covered by the final publication hash.
  Do not patch it into already finalized/hash-bound output or claim the current
  private ResultItem already emits it. The later producer serialization seam is
  an explicit dependency; this reader component does not edit producer source.

G1 handles a malformed outer row representation where applicable; G6 is the
method scope/value gate. For tests intended to reach G6, independently recompute
the publication/receipt hashes after mutation. Removing the field, adding it to
ordinary rows, using null, or moving it into metadata must all fail at the stated
gate without erasing the discrepancy during projection.

Alternative: a new row.evidence object would add an unnecessary container and
require separate absent/empty/closed-object rules. Metadata placement would change
the existing closed metadata vocabulary and encourage dropping unrelated metadata
at G7. The direct scalar has one selected-only rule and one exact removal. All
three alternatives could express the same provenance; this selects no new physics.

## 3. Discriminator/payload collision in G5aError

**Gap:** C3 §3 declares error objects as `{kind,...payload}`, but spells
G5aError sanity{body,kind:U} and lower{member,kind:U}. One JSON object cannot carry
both kind="sanity"/"lower" and kind=0/1. The schema construction caught this
collision before installation; no permissive duplicate-key workaround was used.

**Exact replacement shapes:**

```text
{kind:"sanity",body:U,quantity_kind:0|1}
{kind:"lower",member:U,quantity_kind:0|1}
```

quantity_kind is a direct rename of PP G5aFailure::Sanity.kind and
G5aFailure::Lower.kind, at retained_product.rs:2473–2492,2623–2635,2715–2744.
Values remain the actual 0/1 force/moment resolution index. Lower.member remains
the actual native member id; the separate Operational.member_index is unchanged.
No other discriminator or payload is renamed. Unknown/duplicate keys stay refused.

G1 checks the exact required keys/discriminator and closed enum. G5 verifies the
actual typed cause/owner and check-result relationship. Shared controls include
both valid indices, absent quantity_kind, extra numeric kind, out-of-range index,
wrong body/member and swapping sanity/lower while preserving the wrong fields.

## Selection, definition and controls

The canonical numerical DEFINITION.json and domain hash `a7ed7ca0bf` remain
unchanged: these are error-reference/serialization/key completions, not another
formation. The pending schema/table bytes and their final implementation hashes
will include the reviewed corrections. Existing receipt version and policy names
are still prospective; no sealed historical receipt is rewritten.

Fresh bounded search at `24af17c470` found no prepared_product_failure occurrence
in the relevant maintained reader/schema roots. quantity_kind already occurs in
unrelated schema objects; this is a local field name in the new closed G5aError
branches, not a new global identity or a reinterpretation of those existing fields.
Independent review and ROOT selection precede dependent schema/reader/producer
mapping. No owner-held numerical meaning, guarantee, standing, coverage, export
or interval-binding decision is changed by the proposed choices. ROOT still owns
the technical wire selection and later producer/caller integration.

Required shared controls: an actual-shaped preparation ambiguity refusal linked
to its attempt, a selected-native/later-facade failure, a nonselected-native
kernel failure, a pre-native capture refusal, and a Ready-product/receipt failure
that must keep the existing branch; mutate reference/owner/code/phase after
rehashing. Exercise the exact row method path/projection controls and both G5a
error shapes. These are labelled synthetic controls until produced by a reviewed
real public transaction. No synthetic hash is execution evidence.
