---
doc_id: R17-DEL1005-N4-WITNESS-PROOF
doc_kind: coordination.witness_proof
status: pass
created: 2026-07-23
---

# Invented witness proof

The R17 witness generator uses only repository invented fixtures. Regenerating
the four inputs twice produced identical bytes; executing the five governed
cases twice produced identical controlled outputs.

## Input hashes

| Input | SHA-256 |
|---|---|
| binding mismatch | `19b541c5176406c973c741929c86fc564c16b135ec9fc10cb2c9e0c0b921bb3b` |
| missing payload | `7f514c4fa9f7e3eb7a73cc4b5b6a22b20269dac5a3633d624e274fd7b9d80614` |
| producer blocked | `f6c7918636d8522e9d7e57cc23cd1a99ebf7929b095ae65f6728b794125c98c3` |
| success | `8f11ad10d3717de8c27bc79210afeb0ef65f058d78920ca171e8ea042c7a15b2` |

## Generated controlled-output hashes

| Output | SHA-256 |
|---|---|
| binding mismatch | `8b33e0c41a7a21674332291b5a9f22e3cfdd2bd2e33cab13a4ea86a10f789be0` |
| intent required | `49423f1b65fb845bff30ea7f870888317489b903031394485248ab56c1f7c5f2` |
| missing payload | `c26e3fa8e56453814d2fed88a37a6ac6dfb9e123f1148ecb0611241ee4ef48e6` |
| producer blocked | `3b2af0cc840128a032e083564e949a04dd63479848806ff5990ba87c69393aaa` |
| success | `6450e5bf80678c4220346c7e72f34ef56b7a37d7c25b8d0ada572cf1e3d78ec1` |

All witness JSON parses. The success witness is deliberately small; the
native-size proof uses transient invented padding and does not add a large
tracked artifact.
