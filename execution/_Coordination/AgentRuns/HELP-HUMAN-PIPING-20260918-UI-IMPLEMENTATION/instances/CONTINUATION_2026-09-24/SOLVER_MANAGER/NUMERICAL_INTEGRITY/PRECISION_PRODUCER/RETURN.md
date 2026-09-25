# Precision producer return — frozen for allocated tests

Implemented the bounded M34 source producer cohort; no Cargo/test/native execution claimed. Exact source and diff hashes are in FROZEN.json, with CANDIDATE.diff preserving the three-file candidate. Scoped rustfmt and git diff --check passed. Actual instruction/reference origins, dispatch and execution boundaries are in SOURCES.json.

Full finite computed ResultItem and LocatedQuantity values now publish without round6. Machine numeric optional/basis strings preserve round-trip decimals. Public quantity serializers reject NaN/infinity rather than writing null; the existing mechanics finite guard remains. Historical round6 is test-only, explicitly limited to historical carrier checks and a superseded-quantum loss demonstration. No equation, model version, pressure/stress/reaction formula or acceptance tolerance was changed.

Solved and blocked raw mechanics 0.2.0 carry actual product crate 0.2.0/precision-1 producer identity, finite_binary64/none M03-INTEGRITY-v1 numerical_quality=not_assessed/cases=[] and the product_preview_mechanics_v1 formulation limitations. They do not claim integrity-qualified Current or engineering qualification. Typed case fields are ready for the separate structural integration owner.

Public API: MechanicsProducer, NumericalQuality, NumericalCaseQuality, NumericalQualityStatus, StructuralStatus, ModelMatrixFidelity, AccuracyEvidence, FormulationBasis; constructors mechanics_producer(), unassessed_numerical_quality(), preview_formulation_basis(); constants MECHANICS_SCHEMA_VERSION and PRECISION_SEMANTIC_CONTRACT_ID. Direct MechanicsEnvelope literals in consumers now require the three header fields. serde_json 1.0.149 is locked; float_roundtrip is explicitly enabled in product Cargo.toml. No install/network dependency change occurred.

## Exact lane commands

Run from repository root only after allocation:

```
cargo test --offline --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml precision_
cargo test --offline --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml
```

The first filter includes five new prospective tests: signed subquantum torsion in both modes versus independent TL/GJ; N01 EB displacement/rotation and summary; raw bit/string transport edges; nonfinite rejection; solved/blocked headers. Whole crate also checks six DEC092 independent TL/GJ comparisons without rounding, unchanged protected integration limits and explicit historical carrier checks. FROZEN.json records expected lockfile resolution changes caused by producer version and the sibling sparse dependency. Lock updates invalidate that file's freeze and must be captured after resolution.

Independent analytical reference basis is the reviewed N01/N08/N09 annular/EB packet; the new test formulas do not call production assembly or solvers for expectations. Existing macro-element tests remain integration comparisons, not independent engineering proof. Raw JSON f64 edge preservation is not universal derivative exportability: canonical I-JSON unsafe-integer rejection remains separate and unchanged. Actual native bridge/store/read/export/rule evidence and independent frozen-diff review remain outstanding with their assigned lanes. Parent has product ownership for subsequent M03 integration after this handoff.
