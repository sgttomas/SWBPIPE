# Endpoint Section-Cut Repair — Validation Refinement V2

Status: frozen supplemental recommendation
Applies with: `ENDPOINT_SECTION_CUT_REPAIR_BRIEF_V1.md` and `ENDPOINT_SECTION_CUT_REPAIR_METADATA_REFINEMENT_V1.md`

## Recommendation

Include one bounded metadata-only alignment for the existing curved station rows in the same `core/product_physics/src/lib.rs` change as the endpoint section-cut repair:

- set `coordinate_system` to the already-canonical `element_local`;
- set `basis` to the already-canonical `recovered_from_local_element_stiffness`;
- retain the exact arc-section frame and equilibrium convention in `sign_convention` text (local x tangent toward j, local z bend-plane normal, local y inward radial; section resultant recovered from assembled end actions);
- preserve every numerical result leaf, result kind, raw element-end-action row, load-case key, and result ID.

This is required because `core/runner/headless/src/result_envelope_binding.rs` copies product-physics metadata strings verbatim, while `schemas/results.schema.yaml` has closed enums that do not admit the current curved-station strings `arc_section_frame` and `arc_section_equilibrium_from_assembled_end_forces`. Correcting only new endpoint metadata would leave the full curved envelope invalid for a pre-existing reason and would make a claimed full-schema pass false.

## Exact validation condition

After the endpoint sign repair and the bounded existing-station metadata alignment:

1. validate the complete affected straight and curved result envelopes against `schemas/results.schema.yaml`;
2. require endpoint and station stress rows to use canonical enum values and truthful, frame-specific `sign_convention` text;
3. assert raw element-end-action rows are byte-for-byte unchanged apart from any fixture serialization ordering already proven non-semantic;
4. assert numerical station stress leaves are unchanged;
5. exercise endpoint reversal oracles for pure axial, torsion, and both bending axes, plus combined pressure and temperature;
6. exercise a curved endpoint at each end so the section frame is proven to use the actual endpoint tangent/radial basis rather than the chord frame.

If the same-file station metadata alignment is declined, the truthful fallback is an endpoint-row/envelope-binding subset validation plus a recorded expected whole-envelope schema failure caused by the existing curved-station enum mismatch. A full-envelope PASS must not be claimed in that fallback.

## Scope boundary

This alignment does not add schema enum members or fields. It does not redesign public quantities, modify numerical mechanics, rename result kinds, or alter raw end-action semantics. It repairs an existing producer-to-schema mismatch in the exact producer file already touched by the endpoint correction.
