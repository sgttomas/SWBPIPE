# PR title

Implement friction-coupled supports and reviewed route authoring

# PR body

Friction supports now derive sliding force from the current normal reaction while retaining the configured spring contribution. The preview fixture and its Rust, Python, and desktop oracles reflect that behavior; the default retained-spring L-100 result reports `0.411203 N` friction from `41.120279 N` normal reaction.

The desktop viewport now supports unit-aware straight-route authoring through Add → frozen review → inline Apply for nodes and pipes. Applying a reviewed draft publishes the operation atomically with the expected receipt and model-hash evidence. Disabled Apply states also expose an accessible reason without changing the existing disabled predicate or Apply behavior.

Validation on tested source `e6f694877acf524188d0bd0e5006ea78e2864d0d`:

- complete DEC-025 G0–G4 sweep passed: all 38 Cargo crates, 1,012 Python tests, 804 desktop unit tests, 30 development-browser tests, three production-dist browser tests, and the production desktop build;
- practitioner harness: 379 passed;
- native default-fixture witness on the exact pre-accessibility cut completed one 830-row solve and confirmed the retained-spring result above. The later source delta only adds the disabled-control title; the current source received the complete frontend test and production-build sweep.

Detailed evidence: `projects/chirality-piping/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/CLEAN_VERIFICATION_RESULT_V1.json`.
