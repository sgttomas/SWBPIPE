# RF V3 correction review brief

Status: `SEALED_TINY_SUCCESSOR_FOR_ROOT_DISPATCH`

Target `/root/friction_code_review`, independent read-only Agent 2, `gpt-5.6-sol` high, no delegation. Reuse the V2 numeric/runtime review; assess only this correction. Inputs: V2 brief SHA `d2f536c8540e7bdf0be3c136df976592dbe998afbe382ae7db6e1469e7273402`; V3 correction SHA `c0c8e2b2db5863f01dfb0d7480ab5db68327ab45be9ec214df6930081b999391`; V3 patch envelope SHA `ab4bde7f07d6f6bab9be44f5f7f8b19f2eaf8037d69f59ff6cd5333bce6257c8`; decoded patch SHA `9daf0cfb0ca9795731f691cce0f80c11a093fe5f9d93875dd34dd51ae0249e8a`; in-memory verification SHA `045ac51f390a9bb433dcb33aba6926fcb3c10bdf4b9504202af1911b04d067df`.

Confirm the decoded Rust hunk contains exactly one assertion each for friction `0.489527`, current normal `48.952719`, and `round6(0.01 * normal_evidence.value)`; removes stale Rust literals from that test; retains all state/metadata/diagnostic assertions; and is confined to the affected test. Confirm V2 fixture/Python/desktop candidate post-images are unchanged and the full decoded patch still has exactly the four V2 paths. Return only `PASS` or `CHANGES_REQUIRED` under `{RUN_ROOT}/instances/RF/post_sweep_g0/successor_v3/**`. No writes outside that output, tests, builds, Git acts, authority, lifecycle, or release acts.
