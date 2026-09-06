# Newly identified canonical export conformance findings

Derivative diagnostic evidence; no decomposition amendment or professional acceptance. Root requested separate status, not attribution to prior input-boundary finding I1-C07. Owner D05 adoption remains held; no schema/adapter production edits authorized here.

## P5-CANONICAL-METADATA-001 — OPEN, owning coordination PKG10/PKG13

Actual library result_envelope_document produced by the new headless test, independently validated with Draft202012Validator against schemas/results.schema.yaml, contains existing incompatible metadata beyond the repaired station families. Linear:777 quantity rows,739 metadata rows,109 invalid rows/158 field violations. Nonlinear:797/753/123/210. Zero-pressure:774/736/106/150. Breakdown and exact result IDs/values are in ACTUAL_CANONICAL_VALIDATION.json. Example result:pressure-thrust:component-C-150 carries component expansion_joint_pressure_thrust and a freeform component-family basis in closed enum fields. Nodal/component/nonlinear mapped families also appear in the detailed ledger.

Every invalid field/value pair in these three actual documents also exists in the frozen precompatibility0ab fixture (158/158,210/210,150/150). PREEXISTING_VALUE_PROVENANCE.json binds this check. This establishes prior vocabulary provenance; it does not claim identical previous scenario row counts. The production headless adapter prefix is byte-identical before/after. It gates nonempty/non-TBD metadata but copies native strings into closed canonical fields. Existing typed result export validation does not enforce the schema enums.

Recommendation: separately activate PKG10/13 integration work to specify truthful mappings to existing categories where available and define explicit disclosure or Owner decision for unsupported vocabulary. Add full actual-library document schema validation for representative linear/nonlinear/combined scenarios. Do not silently coerce, drop, or expand enums. This bounded P5 repair does not claim whole-document conformance PASS.

## P5-CANONICAL-REPRODUCIBILITY-002 — OPEN, owning coordination PKG10/PKG13

Each actual document has two additional full-schema violations at result_envelope/reproducibility/model_hash/canonicalization and run_hashes/0/canonicalization: rfc8785_jcs is emitted but canonical enum is JCS/NONE/TBD. Existing core/runner/headless/src/lib.rs produces and accepts rfc8785_jcs; unchanged adapter copies it. The source existed at precompatibility HEAD0ab. Preserve checksum algorithms/values; a future separately authorized adapter compatibility repair should reconcile identifiers without claiming a new hash algorithm or changing D05 adoption.

## Existing disclosed unsupported family

pipe_section_pressure_longitudinal_stress is not mapped by existing adapter. The zero-pressure test confirms explicit vocabulary-boundary disclosure and checks its native metadata independently. Other mapped changed station force/moment/stress rows must reach actual canonical output and pass the schema-derived metadata check. This preserved exclusion is not a newly repaired export capability.

## Scope result

Station metadata compatibility can pass independently of these whole-document failures. Canonical documents are library evidence: normal CLI serialization skips result_envelope_document, so these findings must not be represented as inline canonical CLI output observations. Root retains broader findings in final audit/remaining-work map; no production adapter or schema changes in this amendment.
