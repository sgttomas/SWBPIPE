# DEL-09-04 Clean-Checkout Reproduction Evidence

Disposition: `FAIL`

This immutable derivative bundle records one local clean-checkout reproduction
at source commit `f14fa77518a06f112ae72a8fcce4de0fab958d47`. The core bounded
runner procedure passed: the clone was clean before and after execution, the
generator reproduced all three input fixtures byte-for-byte, the runner exits
were `0`, `1`, and `1`, all three JSON predicates passed, the solve contained
830 result references, and both documented review checks passed.

The tranche fails the complete candidate contract. The single mandatory
registered evidence sweep exited `1`: its desktop Vitest surface returned exit
`127`, and only three of five sweep surfaces ran. The sweep's retained output
also contains `Updating crates.io index` from internally dispatched Cargo
commands that did not carry the candidate-required `--offline` flag. The run
therefore cannot establish the absolute offline/no-external-contact condition
and records the condition as a failure without provisioning or repair.

The DEL-09-04 clean-checkout Remaining item remains open, lifecycle remains
`IN_PROGRESS`, and no Receipt-56 was appended. A rerun must use a new run ID
after the evidence-sweep prerequisite and offline execution posture are
resolved under a new bounded selection.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
