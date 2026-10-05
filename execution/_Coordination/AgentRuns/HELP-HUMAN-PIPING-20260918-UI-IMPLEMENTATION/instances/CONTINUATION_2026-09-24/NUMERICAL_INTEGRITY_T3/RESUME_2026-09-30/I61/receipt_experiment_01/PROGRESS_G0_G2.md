# I61 receipt experiment: progress note at the G0–G2 point

Written 2026-10-04T01:00Z. This is the progress note the standing assignment asks for; it is not a return.

**Iteration 1 already passes G0–G3 in all three draft readers, for the milestone in both modes.**
- **What was emitted:** the first emission of the milestone receipts, for both modes, from the actual private prepared producer.
- **The basis:** NUM `c817a86cb1` and READER `b36739112a`, both as `git archive` copies.
- **What passes:**
  - the closed schema, with 0 jsonschema violations;
  - G0, the identity, policies, definition and table;
  - G1, the receipt, publication, source-identity and preparation hashes. The producer's canonicalizer hashes match all three readers;
  - G2, the encodings;
  - G3, coverage.
- **First failure, the same in all three readers and both modes:** G4 `RETAINED_PRECISION_DIAGNOSTIC_MISMATCH` (PY:1617; Rust and TS identical). The actual ordinary route attempts the legacy source-block method for this case. It fails at source closure ("actual transform is not a signed permutation", 46,628 charged), and the route emits `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming the case.
  - C1's G4 row (WIRE_CONTRACT:147) forbids a "source-unavailable naming a retained-selected case".
  - C2 (CONTRACT_DELTA:160) keeps `legacy_source {unavailable, diagnostic_ref}` because "the original diagnostic_ref preserves original source failure disclosure".
  
  Proceeding needs a contract reading: the facade either removes or keeps that disclosure on a selected case.
- **Next:** the work that needs no reading continues: the refusal receipt and the records. See RETURN.md when it is written.
