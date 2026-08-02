# N3A exact edge/evidence crosscheck — sealed brief

- Parent: EVALUATION N3 under HELP_HUMAN R23.
- Form: read-only ephemeral Agent 2; no delegation.
- Frozen Git basis: `23d15899fd0acf5d1d0513f3fe396438375c9e25`.
- Accepted parent graph: approved immutable
  `projects/chirality-piping/execution/_DAG/DAG-008/`.
- Candidate under review:
  `projects/chirality-piping/execution/_Evaluation/DAG009_CANDIDATE_EVALUATION_2026-08-02_R23/bundle/DAG-009/`.
- R23 basis: `OWNER_RULING.md`, `OWNER_AMENDMENT.md`, `FAN_IN.md`,
  `TARGET_MATURITY_EVIDENCE.csv`, `WRITE_MATRIX.csv`, and all five N2
  `LAUNCH_BRIEF.md` files.
- Evidence scope: the 17 target `_STATUS.md` files and the exact 11 consumer
  evidence files/hash/line ranges named by the N2 briefs; the current five
  local `Dependencies.csv` files; approved DAG-008 `DependencyEdges.csv`.
- Required audit:
  1. Independently verify target maturity and exact consumer integration for
     each of the 24 candidate execution rows.
  2. Confirm exactly 12 pass both gates and exactly 12 remain unchanged.
  3. Confirm five named holds remain unchanged and the anchor normalization is
     evidence-consistent and limited to three fields.
  4. Compare DAG-009 to DAG-008 by DependencyID and ordered field; require
     exactly the 13 accepted IDs to differ and only `SatisfactionStatus`,
     `LastSeen`, and `Notes` to differ.
  5. Confirm changed candidate fields equal the current accepted local rows,
     while DAG-008 extension columns remain preserved.
- Return schema: `Verdict`; `Basis hashes`; `Coverage`; table of all 30 IDs with
  target-gate/consumer-gate/disposition; exact changed IDs and fields;
  unchanged candidates; holds; anchor; contradictions/unknowns; no-write and
  no-delegation confirmation.
- Tools: read-only filesystem/search/hash/CSV comparison only. No writes,
  patches, Git mutation, network, or delegation.
- Stop and return FAIL on any missing evidence, hash mismatch, extra field/row
  delta, or ambiguous integration claim. Do not repair.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
