# Exact rerun

From this packet's harness directory:

```sh
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$PWD/target" cargo run --offline --manifest-path Cargo.toml
```

Final harness is V2 and prints all preserved V1 cases plus the finite-solution overflow and explicit extreme residual vectors. Capture new output to a new filename; do not overwrite sealed evidence. Cargo target is isolated and ignored. All linked source hashes must match SOURCE_BINDING_V1.json for an exact baseline replay; use a separate post-repair packet to evaluate changed production source. The performance_exact source prefix is frozen baseline; rebuild its exact copy/exposure intentionally in the post-repair packet to evaluate repaired private functions.

Current preserved raw output is encoded as raw_v2.json with exact UTF8 and SHA256. COMPARISON_V2.json is produced by compare_v2.py using NumPy2.5.1 and Python Decimal80digit residual arithmetic. Run comparison against new captured bytes in a new packet when rerunning; original comparison script targets its immutable baseline artifact. All expected criteria are frozen in EXPECTED_V1/V2.md. Ignore target build products for archival/commit; MANIFEST binds deliverable files only.
