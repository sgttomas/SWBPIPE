# Preparation checks

Candidate c805bcc1a: receipt validation PASS; self-check exit 0 with existing
repository findings plus one new ABS_PATH_IN_UNCLASSIFIED_SURFACE in the
discovery artifact. Practitioner harness: 378 passed, 1 failed in 53.15 s,
`test_live_gen8_semantic_portability_invariants`, on that same classification.

Correction: preserve the original DISCOVERY.json bytes in `_run_records/`
(recognized evidence), and retain a portable hash/pointer at the old path.
No validator change; no raw-output rewrite; adopted package remains byte-identical.
Affected validation rerun recorded separately.

Affected rerun after evidence placement correction: practitioner harness 379 passed in 53.37 s, exit 0. Raw output: `harness-pytest.txt` beside this record.
