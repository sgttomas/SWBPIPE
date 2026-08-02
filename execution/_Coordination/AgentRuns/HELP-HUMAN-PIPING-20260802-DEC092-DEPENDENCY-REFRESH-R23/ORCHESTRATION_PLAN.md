# R23 orchestration plan — DEC-092 dependency-currency refresh

## Frozen basis

- Git basis: `23d15899fd0acf5d1d0513f3fe396438375c9e25`.
- Branch: `codex/piping-dec092-dependency-refresh`.
- Approved dependency authority remains DAG-008.
- Owner ruling and amendment: `OWNER_RULING.md`, `OWNER_AMENDMENT.md`.
- Current stage: execute N2A–N2E only, validate fan-in, stop before EVALUATION.

## Work graph

1. N1 CHANGE branch gate — complete.
2. N2A–N2E five disjoint no-Bash file-only patch runs — eligible concurrently.
3. PROJECT_SETUP read-only fan-in plus control-artifact updates — serialized.
4. N3 EVALUATION/DAG-009 — explicitly not started in this stage.

## Semantic rule

Each execution row closes only when accepted target maturity and exact
consumer-integration evidence both pass. The target maturity facts are frozen
in `TARGET_MATURITY_EVIDENCE.csv`; consumer evidence and hashes are frozen in
each N2 launch brief. No optimistic inference is allowed.

The exact planned closures are:

- N2A: `DAG-002-E0401`, `DAG-002-E0402`.
- N2B: none; all seven candidates remain byte-identical because the cited
  review expressly says the TBD rows were not resolved and the bounded
  implementation evidence is not edge-specific enough.
- N2C: `DAG-002-E0489`, `DAG-002-E0490`, `DAG-002-E0492`,
  `DAG-002-E0493`, `DAG-002-E0494`.
- N2D: `DAG-002-E0534`, `DAG-002-E0535`, `DAG-002-E0536`,
  `TP-DAG-004-DEL-09-01-E002`.
- N2E: `DAG-002-E0827`.

N2A also normalizes anchor `SEMREF-2026-06-16-DEL-03-01-A003` from `TBD`
to `NOT_APPLICABLE`. Five confirmed holds remain raw-row byte-identical.

## Write and tool fences

Each N2 child may use only managed file read/apply-patch tools. Bash, shell,
Python, Git, network, delegation, and arbitrary search are prohibited. Each
child writes only its named consumer `Dependencies.csv` and
`_DEPENDENCIES.md`. PROJECT_SETUP owns all managed instance triplets and R23
control artifacts.

No DAG, `_LATEST`, decision/register, deliverable `_STATUS.md`/`MEMORY.md`,
product, receipt, Git closeout, or EVALUATION write is authorized in this
stage. F-PIP-1 through F-PIP-5 apply; standard claim fence applies.
