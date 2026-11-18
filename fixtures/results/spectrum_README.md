# Spectrum result fixtures (DEL-04-10 REQ-006)

Maintained test fixtures of the experimental spectrum result family `openpipestress.spectrum_result` 0.1.0 (profile `product_preview_spectrum_v1`, DEL-04-10 REQ-006). They are not run evidence and not spectrum solve results. Each document states that it is experimental and not a public engineering result.

Each document is constructed from a frozen reference under `validation/references/f2_spectrum/`. Its signed contributions, rigid responses and modal basis are taken from the reference except where its `origin` lists a construction. Its combined magnitudes are formed from its contributions by the product's combinations; the writer copies no reference combined value into a document. Its `modal_result.binding` is `not_applicable`, because no modal result exists, and its modal basis is attributed to the reference.

| File | What it is |
|---|---|
| `semantic_contract_spectrum_1.json` | The semantic table `openpipestress.result_semantics/spectrum-1` |
| `spectrum_<name>.json` | A document |
| `spectrum_<name>.model.json` | The model document stand-in it binds: the entities the document names and the spectra, in the model document's shape, not a schema-checked model |
| `spectrum_<name>.request.json` | The spectrum request it binds |

| Document | Reference | Shows |
|---|---|---|
| `c9_y_two_frames` | discrete C9-Y | straight members on a space diagonal, end actions in global and member axes |
| `c7_cancelling_cluster` | discrete C7-PAIRS, reduced roster | a repeated pair whose binary64 contributions to some rows are equal and opposite; a zero magnitude in that direction |
| `w2_arc_oblique` | recovery_w2 V-ARC-OBL | an arc's total end action in global axes and its tangent frame; oblique straights in member axes |
| `w2_joint_rods` | recovery_w2 V-JOINT-R | a connector's generalized forces and end actions, and its tie rods' axial forces; no rows for the replaced span |
| `c8_shared_constraint` | discrete C8-III-TRUNCATION | one rigid constraint held by a restraint and a snubber, as one row naming both |
| `c2_separator_identities` | discrete C2-SPRING, renamed | identities containing dots, spaces, colons and `_m` |
| `refused_arc_member_axes` | V-ARC-OBL, mutated | an arc's end action stated in member axes: refused |
| `c3_grouping_100_40_40` | discrete C3-BASE, mutations grid entry GROUPING / rigid ABSOLUTE_SUM / 100-40-40 | the grouping method with a group of two clusters; a governing direction over X, Y and Z with the correction applied |
| `c9_chain_ten_percent` | discrete C9-CHAIN-Y, mutations grid entry TEN_PERCENT, reduced roster | coupled pairs (21, 22) and (22, 23), where the reference's grouping method groups 21 with 22 and starts a new group at 23 |
| `c8_absolute_sums` | discrete C8-III-TRUNCATION, mutations grid entry ABSOLUTE_SUM / rigid ABSOLUTE_SUM / ABSOLUTE_SUM | the absolute-sum modal, rigid and directional rules over Y and Z |

## Generation

The documents, their inputs and the table are generated deterministically by the crate's own assembler:

```bash
cd core/analysis/response_spectrum
<cargo wrapper> test --test spectrum_documents write_spectrum_documents -- --ignored
```

Each direction's formation record (its spectrum, the digest of that spectrum's record in the model stand-in, and its scale factor) is made by the writer from the reference's own request and spectrum, and each origin says so.

`documents_are_the_projection_of_their_inputs` requires the committed bytes. `every_committed_document_conforms_to_the_document_shape` checks each document against the `document_shape` of the committed semantic table. The other tests in `tests/spectrum_documents.rs` compare each combined magnitude for which the reference publishes a value at 1e-9 relative (an expected zero exactly), with the number of compared rows stated per document, and check REQ-006's worked rows to six significant figures.
