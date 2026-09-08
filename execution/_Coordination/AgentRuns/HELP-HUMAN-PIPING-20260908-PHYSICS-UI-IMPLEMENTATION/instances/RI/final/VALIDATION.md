# RI final validation

- Sealed amendment: `PASS`; SHA-256 `eb9028f4badf7fd697bc80954c03447c25ad9b4ad46fbc8cd34a9f91cc3a9257`.
- Accepted precheck packet: `PASS`; all seven active/archive/correction bindings rehashed exactly.
- R2 manifest: `PASS`; manifest hash exact, 45/45 output hashes and sizes exact, 7/7 sealed-input hashes exact, and 6/6 predecessor hashes exact.
- Structured model comparison: `PASS`; generated project identity exact, 8/8 invented-provenance carriers exact, one load case and one primitive exact, final magnitude 500 N, Y reference `(0,0,1)`, and six restraint DOFs exact.
- Native tuples: `PASS`; two complete saved-store records, model hashes independently recomputed, submitted/result model identity exact, distinct jobs/model/input/analysis/result hashes, common canonical run ID, solved status, and 67 rows each.
- Visual/AX review: `PASS`; all 20 PNG members inspected, all dimensions read, final image exactly 1024 by 768, and AX no-change files treated as delta responses.
- Live source: `PASS`; 10/10 members match `REPO_ROOT/projects/chirality-piping/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/NATIVE_SOURCE_BINDING_FINAL_V1.json`.
- Live bundle: `PASS`; 3/3 members match the bound inventory, sizes match, and canonical entry-stream digest recomputes to `d54e06c6229163ad27bfb3d3ad27b1383b6cb50383d70316b0b6c83e71bd0b11`.
- Portability classification: `PUBLICATION_CORRECTION_REQUIRED`; raw host paths in three immutable R2 JSON members do not alter the technical verdict and must be addressed through the amendment's additive successor process.
- Scope: read/search, hashing, structured comparison, and image inspection only. No build, test, app launch, store access, source edit, native-packet edit, Git write, or source-review restart was performed.
