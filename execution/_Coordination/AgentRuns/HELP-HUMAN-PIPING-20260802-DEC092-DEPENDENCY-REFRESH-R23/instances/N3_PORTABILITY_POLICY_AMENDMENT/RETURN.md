# N3 portability-policy amendment — return

- Verdict: PASS.
- Policy pre-hash: `f5a7d67feb57e74b2bc0763768a9ab145c92a6dd8c931eda22375b9b294b274a`.
- Policy post-hash: `f0dacfee06bdd4aac177a962f4710edc99fda0e073e53e965c88fb4d238fade4`.
- Added: exactly five SHA-bound `historical_role_override` / `EVIDENCE`
  entries and one SHA-bound `control_path_exception` / `CONTROL` entry.
- Existing policy entries: unchanged and order-preserved.
- Classified targets: all six content/hash-identical; no target edit.
- Validation: JSON/unique paths/hash bindings PASS; path-anchor PASS with zero
  findings; surface-role/path-anchor tests 51/51 PASS; candidate manifest,
  dependency schema, canonical strict DAG audit, whitespace, and containment
  PASS.
- Frozen evaluation inputs: 124 unchanged; sole delta is the owner-authorized
  validator-basis policy hash transition above.
- N3 may resume from its preserved bundle and N3A/N3B crosschecks after recording
  this authorized basis delta; no candidate rebuild is required.
- No DAG, receipt, pointer, product/lifecycle, decision/register, or Git closeout
  action occurred.

