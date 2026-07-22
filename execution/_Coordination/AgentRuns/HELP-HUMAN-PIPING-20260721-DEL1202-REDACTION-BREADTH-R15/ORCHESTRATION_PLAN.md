# ORCHESTRATION PLAN — HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15

- **Plan version:** 2 (v1 N3 `BLOCK` preserved; five defects cured pre-effect)
- **Prepared:** 2026-07-22T03:29:43Z
- **Parent:** HELP_HUMAN (Agent 0)
- **Manager:** WORKING_ITEMS (Agent 1), package `PKG-12`, selected deliverable `DEL-12-02`
- **Frozen basis:** `0c066652cd527eb1559f715e914262d2bda42602`
- **Branch:** `codex/piping-pkg12-redaction`
- **Selection authority:** adopted HELP_HUMAN execution plan; only pre-effect nodes 3.1–3.3 are released
- **Posture:** `MIXED` — parallel read-only inventory, manager fan-in/brief authorship, serialized fresh refutation

## Objective

Prepare, but do not adopt or execute, a governed candidate brief for the
DEL-12-02 `REXC-REQ-012` breadth remainder. Freeze every current report,
export, CLI, adapter, and downstream-handoff exposure route at the basis SHA;
route later implementation through the existing redaction contract; and stop
at owner Gate 3 after independent refutation.

## Work graph

| Node | Layer | Objective | Dependency | Writes | Gate |
|---|---:|---|---|---|---|
| N1 | Agent 2 | Core/CLI/adapter/plugin/downstream read-only inventory | — | none | manager validates evidence and coverage |
| N2 | Agent 2 | Desktop report preview/export and bug-report read-only inventory | — | none | manager validates evidence and coverage |
| W1 | Agent 1 | Fan-in, deterministic supplemental desktop-download scan, route matrix, rationale, candidate brief | N1 + N2 | this run root and candidate brief only | full frozen route set and decision-complete brief |
| N3 | Agent 2 | Fresh adversarial refutation of v1 | W1 | `instances/N3/RETURN.md` only | returned `BLOCK`; immutable history |
| W2 | Agent 1 | Cure all N3 defects without product/state effect | N3 | this run root and candidate brief only | v2 complete and internally validated |
| N3B | Agent 2 | Fresh post-amendment refutation of v2 | W2 | `instances/N3B/RETURN.md` only | `COMMIT-SAFE` or `BLOCK` |
| G3 | Human owner | Adopt/reject/amend v2 and explicitly authorize the bounded cross-package integration map | N3B | governed decision effect only | no execution before decision |
| N4 | Agent 2 | Sole serialized implementation owner, `TASK + software-bounded-implementation` | G3 adoption | product/test/doc fence only; no state/receipt writes | not released in this run |
| N5 | Agent 2 | Fresh implementation verifier, `TASK + software-code-review` | N4 accepted return | verifier evidence only | not released in this run |
| W3 | Agent 1 | DEL-12-02-only state/run-record closeout | N5 `COMMIT-SAFE` | DEL-12-02 state/evidence only | not released in this run |

N1 and N2 are the only parallel children. They were read-only and disjoint.
All later nodes are serialized. N4 is the single project-root/Bash-bearing
integration owner only if the owner adopts the brief and explicitly authorizes
`AFFECTED_OWNER_MAP.csv`. N4 cannot close deliverable state; W3 runs only after
fresh N5 `COMMIT-SAFE`.

## v2 amendment after N3 BLOCK

N3's five defects are cured by: adding `REXC-CLI-002` and `REXC-LINT-001`;
hard-coding writer contexts; adding `AFFECTED_OWNER_MAP.csv` and making its
bounded integration scope part of the exact owner decision; adding exact
per-route verification dispositions; and moving all deliverable-state,
run-record, and receipt effects after N5. N3 remains failed history and cannot
support Gate 3. N3B is fresh.

## Preserved gates

- Candidate-brief adoption is the human owner's act; `EffectStatus` stays `HELD`.
- No product code, deliverable status, memory, run record, receipt, lifecycle,
  stage, release, acceptance, publication, merge, push, or external action.
- No plugin runtime, bug-report feature, public transport, cloud/network path,
  new runner verb, storage root, or protected-data use.
- Any material route-set or contract drift after the frozen SHA returns the
  brief to HELP_HUMAN before execution.
