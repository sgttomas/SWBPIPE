# V3 candidate correction

Status: `V2_RUST_HUNK_REPLACED_CANDIDATE_HELD`

V2 decoded patch SHA `9210169b6968d7f319506cfb53b8d6bc71132b713fba8fa52736b2ee4bc8e8d7` was incomplete: it captured `friction_reaction` but omitted the new exact friction assertion, retained stale normal `48.952652`, and omitted the `round6(mu*N)` assertion. V2 bytes remain frozen as failed candidate evidence.

V3 changes only the Rust candidate post-image relative to V2. The generated fixture, Python candidate, desktop candidate, affine evidence, and inventory are byte-identical to V2. The V3 Rust test captures both values, asserts friction `0.489527`, normal `48.952719`, and exact `round6(0.01 * normal_evidence.value)`, while retaining the prior loop/state/sign/metadata/diagnostic checks. The live repository remains unchanged.
