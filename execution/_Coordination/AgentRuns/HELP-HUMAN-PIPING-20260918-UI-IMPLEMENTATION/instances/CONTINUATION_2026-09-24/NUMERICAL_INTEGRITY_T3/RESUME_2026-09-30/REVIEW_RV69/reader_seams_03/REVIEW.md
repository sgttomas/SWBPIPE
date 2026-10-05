# RV69 — reader contract seam backcheck

**Disposition: CLEAR for ROOT technical selection of the three prospective wire corrections, using 06 at c5890453d7 together with 07 at a82b89774f and 08 at 55e6722a40. No unresolved blocking findings.**

Same independent TASK Type 2 /root/rv69_public_formation_contract under ROOT /root. Receipt 2026-10-03T16:21:20Z. Proposal c5890453d7; immutable source 24af17c470; brief 5e61694900. Full identities/hashes are in JSON. This is a bounded design review, not review of the moving reader implementation, a final source review or a PR gate. No maintained/Git/index/API writes, runtime, tool installation or descendants occurred. Prior review seals remain unchanged.

P = projects/chirality-piping; PP = P/core/product_physics/src/retained_product.rs. C1/C2/C3/F1 mean the selected records identified in ORIGINS.json. Source line numbers below are at 24af17c470.

## Finding

**RS-F1 (P2, originally selection-blocking; now CLOSED): pin G1/G6 shape and first-failure rules.** ADDENDUM.md:72–78 declares an optional direct string with a sole accepted value but promises null, wrong types/values and tokens on unselected rows fail G6. Lines95–99 also promise G6 for misplaced metadata tokens. C1 G1 and C3:293–300 enforce closed shapes before G6; a string type excludes null/nonstrings, an enum/const would reject wrong strings early, and a forbidden nested field does not become shape-valid after repairing hashes. The original text therefore does not determine one shared first failure.

The minimal coherent correction is an optional direct string at G1, with no enum/const/minLength there. G1 refuses null/nonstrings and forbidden additional/nested fields. G6 then enforces exact string value, selected-row presence and absence on ordinary/unavailable rows, including selected ancillary rows. G4 remains the diagnostic gate; C3:303's broad wording must not move row-token checks ahead of G6. G7 strips only the direct field already bound by G6. A first-failure matrix must distinguish each mutation and require repaired hashes/otherwise passing earlier gates when the intended target is G6. Metadata or other closed shapes must not be loosened to route a malformed field later.

## Same-reviewer backcheck of 07 and 08

I read each correction in full and verified its seal/payloads. Correction07 was first read from its sealed local packet, then verified byte-identical to preserving commit a82b89774f; correction08 was read from immutable 55e6722a40. Neither original proposal was edited. The combined replacement closes RS-F1:

| Condition, after any stated earlier prerequisites | First failure |
|---|---|
| Direct recovery_method is null or a nonstring | G1 raw shape / RECEIPT_MISMATCH |
| row.evidence, metadata.recovery_method or another forbidden key, even with a correct direct token | G1 closed shape / RECEIPT_MISMATCH |
| A prohibited method field on the unavailable/not-required case object | G1 closed shape, even with later diagnostic/token defects |
| Selected row has no direct token, or an empty/wrong string | G6 / ROW_METHOD_MISMATCH, when prior gates pass |
| Ordinary/not-required/unavailable row has any direct string | G6 / ROW_METHOD_MISMATCH, when prior gates pass |
| Well-shaped token defect plus invalid canonical Bits | G2 before G6, when G0/G1 pass |
| Well-shaped token defect plus case/row coverage defect | G3 before G6, when G0–G2 pass |
| Well-shaped token defect plus real diagnostic exclusivity/code/ref defect | G4 for that diagnostic defect; with valid diagnostics the token defect waits until G6 |
| Another real G5/G5a/G5b/G5c defect plus a token defect | That earlier gate's selected code before G6 |
| Correct token on selected row, or absent token on nonselected row | No method failure; remaining gates still apply |

07's raw row preserves the actual ResultItem fields and five-field ResultMetadata from lib.rs:1976–2018, and adds only an optional direct string. G1 does not place enum/const/minLength on that string. Optional means absent or correctly typed, never null. Metadata signature/physical meanings remain with the unchanged base validator; closed keys remain closed. These are successor raw-row rules, not a change to canonical derivative shape or historical receipt acceptance.

08 explicitly removes the competing interpretation of C3 G4: it checks diagnostics, not raw-row tokens. Both diagnostic and row-token checks still run at their assigned positions; no gate is suppressed to obtain a desired test code. G7 runs only after successful G6 and removes only the direct bound field. Repaired hashes are necessary for intended later-gate mutations but never cure shape/type defects.

The entire 07 matrix plus 08's diagnostic/case-field compound controls must be preserved in the shared corpus. This is design-level confirmation; none of those runtime/parity controls ran in this review.

## Independently rederived mappings

### Outer unavailable reference

C2:72–74 has no preparation/facade branch capable of naming all selected C3 PublicFailure variants. C3:277 already carries the actual closed errors. The proposed `{kind:"prepared_product_failure",product_attempt_ref:U}` is a faithful reference to those records, provided its owner, non-Ready outcome and exact code/phase/Run constraints remain mandatory.

| Actual source outcome | Faithful outer mapping |
|---|---|
| PreparedCaseFailure, including optional SectionPreparationError | source_unavailable/preparation; actual failed preparation stage, no new native Run, retained old/prefix/helper work. PP:3112–3141,3215–3217 retains the error and work; no fabricated SourceError. |
| Origin/capture refusal before a recorded Run | source_unavailable/preparation with no Run; an already constructed source may remain. PP:3231–3237 can fail before storing a RecordedCase. Empty attempts are not used to invent a Run or claim zero cost. |
| Actual native nonselection | kernel_unresolved or kernel_refused according to the referenced Run terminal. PP:3238–3241 stores that Run before NativeUnavailable; :3411–3412 preserves actual missing/nonselected-owner distinction. The CaptureError string is not a classifier. |
| Capture/association/adapter failure after native selection | facade_certificate/facade with that same selected Run and actual stage prefix, even if proof_start was not entered. PP:3413–3417 can refuse before starting the draft; :3452–3458 can fail after numeric certification. |
| Proof, values, abandoned, numeric, observable or G5a failure | facade_certificate/facade; preserve the exact existing error(s), selected Run and entered work. PP:3417–3449 and :3463–3473 support these distinct outcomes. Observable/G5a checks after numeric refusal do not turn it into Ready or selected. |
| Public encoding/hash failure after a Ready private product | Existing C2 receipt_failure and ordinary transaction. Do not rewrite the private Ready outcome merely to use the new reference branch. |

C3 case, ProductAttempt and C2 Run/source references must name the same actual owner. The new reference cannot point to another case, a combination, an absent attempt or Ready. Existing source_decline retains its real constructor error; contradictory constructor/capture facts are invalid evidence. Unrepresentable payloads use existing receipt fallback, not invented finite/count/error values. Native debit and failed-prefix custody remain unchanged.

The proposed G1 shape, G2 encoding, G3 owner/coverage, G4 diagnostics and G5 reference/terminal/stage checks respect the selected gate order. Within the established producer-attestation boundary these are consistency checks, not authentication of execution from unkeyed hashes. No additional blocker was found in this seam.

### Direct row path and restricted G7 projection

P/core/product_physics/src/lib.rs:1976–1988 has neither an evidence container nor recovery_method, so a prospective producer serialization seam is necessary. The direct optional scalar is a technically coherent completion of D2:529 and C1 G6; it is not a claim that the existing private producer emits it.

After shape and earlier gates pass, every row owned by a retained-selected case must carry the exact token, including mode/parity/material/nonquantity rows. Ordinary, not-required and unavailable row bases must omit the member. Existing retained-selected combination obligations remain, but no prepared-combination capability follows.

G7 may remove only results[i].recovery_method from rows whose identity, basis and method were established by G6. It must not erase metadata.recovery_method, an evidence object, another arbitrary field, row values, sections or unrelated metadata. D2:544 and C1:153 permit removal only of bound successor additions. The remaining base evidence validator runs unchanged. The candidate must pre-stage this direct member before final certification/publication hashing; producer mutation after the freeze requires invalidation/rechecking. The corrected RS-F1 basis fixes deterministic rejection order without changing this chosen path or numerical meaning.

### G5a discriminator correction

The original C3:276 `sanity{body,kind}` and `lower{member,kind}` collide with its universal `kind` discriminator. That is a genuine closed-object defect, not a JSON encoding convention to guess.

PP:2473–2492 has Sanity{body,kind} and Lower{member,kind}; :2623–2632 produces sanity indices0/1; :2715–2741 produces lower indices0/1 and the actual native member id. Renaming only the payload key to quantity_kind preserves those facts. Operational.member_index retains its distinct ordinal meaning. Both new shapes are closed, with exact discriminator and quantity_kind enum0|1; absent/extra/wrong-type/out-of-range keys fail shape, and wrong body/member/typed-cause relationships remain later consistency failures. Duplicate JSON keys are not a supported representation.

The correction missed by the earlier C3 review is now independently confirmed. No arithmetic, error cause, owner identity, gate tolerance or fallback behavior changes with this rename.

## Verification and owner implications

The proposal seal and both payload hashes/sizes match. An immutable maintained-root collision search found zero prepared_product_failure occurrences and nineteen quantity_kind occurrences in unrelated schema vocabulary; the latter is a local property name, not a conflicting public identity. The C3 numerical definition and its domain hash remain unchanged.

These are prospective wire completions. The new failure reference preserves unavailable standing and ordinary rollback; the row scalar supplies existing selected-method provenance; the quantity_kind spelling makes existing typed facts representable. No owner-reserved numerical truth, protected criterion, scope coverage, availability or interval-binding choice is introduced. ROOT can now technically select the combined 06+07+08 basis; this reviewer does not select it or reserve it. No new human approval gate is inferred.

After ROOT selects 06+07+08 together, the affected closed schema/three-reader mappings and later producer serialization may be implemented within ROOT's authorized scope. This review grants no moving reader/source acceptance, producer-only merge, public qualification, resource/caller admission, promised exact/combination omission, or final PR readiness. Shared synthetic controls remain labelled synthetic until witnessed by a reviewed real public transaction; no runtime/parity check was run here.

Independent combined backcheck completed at 2026-10-03T16:32:06Z, within the original thirty-minute window. Full hashes, source origins, preservation checks and tool qualifications are in CHECKS.json, ORIGINS.json and EXECUTION.json. Later changes need their own affected review; no final reader-source or PR acceptance is supplied here.
