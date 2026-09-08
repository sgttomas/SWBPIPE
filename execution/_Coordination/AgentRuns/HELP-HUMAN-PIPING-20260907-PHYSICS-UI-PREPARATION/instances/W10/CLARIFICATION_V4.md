# W10 witness-log representation clarification V4

Parent authorized an evidence-representation correction after CHANGE identified the raw WITNESS.log extra blank EOF line as a staged whitespace blocker. No original log byte is removed or changed: `_run_records/PORTABILITY_ORIGINALS_V1/WITNESS.log.gz` losslessly stores the original. The current WITNESS.log is a portable pointer. No raw duplicate is introduced elsewhere, no diagnostic rerun occurs, and all findings and prior candidate decisions remain unchanged.

Original/decoded SHA256: `716876cc8510868115af832dbd214c23c4e36c0b692d72487ae474f44b31a8b4`. Gzip SHA256: `6a07258156f4b8004bdfe03600604e70cfd1f45dfe6eb64f6036eaaefbb62354`. Current pointer SHA256: `053ab2365c1a489ec93341fa06b04733e00246d025a198309847abb4de855c63`. WITNESS_RELOCATION_V4.json supplies historical V1/V2/V3 manifest resolution. Earlier manifests and BUILD_CONTEXT relocation remain unchanged; MANIFEST_V4.json binds their successor representations plus this record.

For independent inspection from the W10 directory, use `gzip -dc _run_records/PORTABILITY_ORIGINALS_V1/WITNESS.log.gz` to emit exact original bytes to stdout. If a file is needed, direct it only to a fresh temporary directory outside the staged tree. Compute SHA256 over decoded bytes and compare the original/decoded hash above. This preserves the original final blank line as evidence without adding raw whitespace violations to the candidate diff.

Verification: gzip round-trip byte equality and all V4 manifest hashes pass. R1 narrow review and CHANGE whitespace/check rerun remain pending; no suite PASS is asserted here. BUILD_CONTEXT V3 and log V4 are representation-only corrections, not changes to numerical evidence.
