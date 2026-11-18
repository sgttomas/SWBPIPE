# Spectrum result fixtures (DEL-04-10 REQ-006)

Maintained test fixtures of the experimental spectrum result family `openpipestress.spectrum_result` 0.1.0 (profile `product_preview_spectrum_v1`, DEL-04-10 REQ-006). They are not run evidence and not spectrum solve results. Every document is experimental and not a public engineering result.

Each document is constructed from a frozen reference under `validation/references/f2_spectrum/`: its signed contributions, rigid responses and modal basis are the reference's, and its combined magnitudes are formed from them by the product's combinations. No reference combined value is copied into a document. Its `origin` names the reference and states everything constructed rather than taken. Its `modal_result.binding` is `not_applicable`, because no modal result exists, and its modal basis is attributed to the reference.

| File | What it is |
|---|---|
| `semantic_contract_spectrum_1.json` | The semantic table `openpipestress.result_semantics/spectrum-1` |
| `spectrum_<name>.json` | A document |
| `spectrum_<name>.model.json` | The model document stand-in it binds: the reference's entities and spectra in the model document's shape, not a schema-checked model |
| `spectrum_<name>.request.json` | The spectrum request it binds |

| Document | Reference | Shows |
|---|---|---|
| `c9_y_two_frames` | discrete C9-Y | straight members on a space diagonal, end actions in global and member axes |
| `c7_cancelling_cluster` | discrete C7-PAIRS, reduced roster | an exactly repeated pair whose contributions cancel exactly; an exact zero magnitude |
| `w2_arc_oblique` | recovery_w2 V-ARC-OBL | an arc's total end action in global axes and its tangent frame; oblique straights in member axes |
| `w2_joint_rods` | recovery_w2 V-JOINT-R | a connector's generalized forces and end actions, and its tie rods' axial forces; no rows for the replaced span |
| `c8_shared_constraint` | discrete C8-III-TRUNCATION | one rigid constraint held by a restraint and a snubber, as one row naming both |
| `c2_separator_identities` | discrete C2-SPRING, renamed | identities containing dots, spaces, colons and `_m` |
| `refused_arc_member_axes` | V-ARC-OBL, mutated | an arc's end action stated in member axes: refused |

## Generation

The documents, their inputs and the table are generated deterministically by the crate's own assembler:

```bash
cd core/analysis/response_spectrum
<cargo wrapper> test --test spectrum_documents write_spectrum_documents -- --ignored
```

`documents_are_the_projection_of_their_inputs` requires the committed bytes. The other tests in `tests/spectrum_documents.rs` hold the documents to their references' combined values at 1e-9 relative and to REQ-006's worked rows.
