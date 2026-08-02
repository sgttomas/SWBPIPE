# N3 EVALUATION — DAG-009 candidate launch brief

- Parent: HELP_HUMAN R23.
- Manager: EVALUATION (Agent 1).
- Frozen Git basis: `23d15899fd0acf5d1d0513f3fe396438375c9e25` on
  `codex/piping-dec092-dependency-refresh`.
- Accepted upstream dependency authority: approved immutable `execution/_DAG/DAG-008/`.
- Accepted subject state: N2 fan-in PASS over the five SHA-bound local consumer
  registers and indexes recorded in R23 `FAN_IN.md`.
- Objective: independently revalidate all 24 candidate execution rows, 12
  accepted closures, 12 unchanged candidates, five unchanged holds, and one
  anchor normalization; then author and validate a quarantined derivative
  `DAG-009` proposal.
- Write scope: only
  `execution/_Evaluation/DAG009_CANDIDATE_EVALUATION_2026-08-02_R23/**` and
  R23 N3/N3A/N3B triplets plus manager-owned runtime updates.
- Toolbelt: deterministic read/hash/CSV/JSON/schema/DAG/topology/SCC/manifest/
  claims/path/pointer/decomposition checks; two read-only Agent 2 crosschecks;
  EVALUATION synthesis.
- Candidate transformation: preserve approved DAG-008 rows, order, extension
  fields, nodes, graph membership, and topology; copy only the accepted local
  `SatisfactionStatus`, `LastSeen`, and `Notes` fields for the 12 closure IDs and
  one anchor ID.
- Bundle contract: exactly the same 16 artifact roles as approved DAG-008;
  `APPROVAL_RECORD.md` is a proposed-pending placeholder with `approved` and
  `approved_by` set to `TBD`; bundle-local `_LATEST.md` is proposal-only; root
  `_DAG/_LATEST.md` and DAG-008 remain untouched.
- Fan-in: N3 closes PASS only if deterministic checks and both independent
  returns agree. Any mismatch stops without repair or materialization.
- Exclusions: no live `_DAG/DAG-009`, root pointer, receipt, decision/register,
  product/status/memory, Git closeout, acceptance, lifecycle effect, or DEC-092
  implementation.
- Owner correction applied: the root arbitrary-Bash paragraph is misplaced
  app-harness policy and does not apply to Codex-native development agents.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). All
  F-PIP-1 through F-PIP-5 fences remain in force.
