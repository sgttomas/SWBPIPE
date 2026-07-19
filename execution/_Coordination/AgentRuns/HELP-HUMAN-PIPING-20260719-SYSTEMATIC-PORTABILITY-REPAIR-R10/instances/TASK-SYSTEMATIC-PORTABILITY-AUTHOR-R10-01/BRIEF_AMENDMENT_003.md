# TASK Brief Amendment 003 — Post-v2 Final Sweep Only

- InstanceID: `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01`
- Parent amendment: `PLAN_AMENDMENT_003.md`
- Code/policy edits: forbidden

Preserve the existing sweep unchanged as superseded/non-admitted. Run the
DEC-025 sweep exactly once against the current post-v2 tree. Validate overall
`pass`, all five expected surfaces `pass`, and bind the new file's SHA-256.
Do not retry on failure. Preserve earlier terminal records and write
`RETURN_V3.md` and `STATUS_V3.json`.
