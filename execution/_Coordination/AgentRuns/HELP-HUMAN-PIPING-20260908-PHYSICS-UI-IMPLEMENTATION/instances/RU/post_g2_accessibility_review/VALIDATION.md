# Validation record

- Final source SHA-256: `d284969e4d52e6140625fe48c8ad482a406307917845b99af58338e5f9048eda`; bytes: 111644.
- Exact in-memory preimage reconstruction SHA-256: `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad`; bytes: 111152.
- Diff archive SHA-256: `ef808b2ef030e9d2c79e320e95dddd1c30b905d622abedae6933287d08ef6766`; decoded bytes: 1848; decoded SHA-256: `18111a2aa9fc445abc53183650ea757b06549ee75026d7adf8c608f0c98131ce`.
- U7 final binding: 17 entries, zero live hash/byte mismatches; computed aggregate equals declared `d80b48eaa2c1c60532c17fbace51eddd9f049123396af7d01c66a3a2ab20ca6a`; 16 predecessor members unchanged and only `PipeViewport.tsx` updated.
- Final dead-control audit: U7-recorded PASS, 1/1, 37.62s.
- Focused behavior checks: U7-recorded PASS, 3/3 with 159 skipped, 3.59s, on V1; reuse accepted because V2 changes title-condition ordering only.
- U7-recorded source diff check: PASS.
- RU executions: static reads, hashing, exact reconstruction, binding verification, and evidence publication only. No test, browser, build, native, Rust, WASM, source, or Git execution.
- Final input-inventory checksum validation: PASS for all 25 entries.
- Two failed evidence commands are preserved losslessly under `_run_records`: one wrong relative source path and one initial inventory check with stale copied metadata hashes. Both were corrected within RU evidence and had no effect on source or conclusions.
