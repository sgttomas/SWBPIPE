# P5 activation V1 — PKG-05 pressure-reference investigation

Status: ACCEPTED_FOR_EXECUTION. Selection authority: HUMAN through `RUN_ROOT/OWNER_DIRECTION.md` and the frozen root work graph. Run: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`. Parent: `/root` (`HELP_HUMAN`, Agent 0). Instance: `/root/pressure_reference` (`WORKING_ITEMS`, Agent 1). Delegation class: delegated-harness-native; role and non-delegation are instruction/config asserted where the harness does not expose mechanical proof. Requested and actual visible configuration: `gpt-5.6-sol`, reasoning `high`; no `gpt-6-astra` grant.

## Package activation

- `PackageID`: `PKG-05` — Loads, Load Cases, and Stress Recovery.
- Selected deliverable: `DEL-05-03` — Fundamental stress recovery module.
- Scope/objective: `SOW-015` / `OBJ-003`.
- Scope path: `{WORKING_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module`.
- Accepted source basis: branch `codex/piping-physics-ui-execution-20260908`, source SHA `779dedb8670625b36af07b89fc5557470e47c50e`; decomposition revision 0.12 / SCA-009 / DAG-010; Receipt 135; committed loop plan `loop/WORKPLAN_2026-07-18b_piping_loop.md`; accepted preparation `HELP-HUMAN-PIPING-20260907-PHYSICS-UI-PREPARATION`.
- Format state: `SOW_V1`, verified by `tools/scope_of_work/validate_scope_of_work.py`; live register rows bind `DEL-05-03` to `SOW-015` and `OBJ-003`.
- Upstream selection: preparation E1 manager Option C, a bounded four-case pressure-reference investigation. The pressure child’s limited Option A remains preserved as a candidate alternative, not adopted physical truth.

## Objective and completion contract

Complete, rather than merely propose, an independent four-case pressure-reference investigation covering: free closed pipe; axially restrained closed pipe; separately supported closures/open pipe; and combined thermal plus pressure. Freeze auditable free-body and constitutive derivations, numerical worked cases, and proposed future test expectations before comparing them with current output. Resolve candidate meanings and signs for wall force, effective force, endpoint actions, and section cuts; compare cap-area and mean-radius conventions; distinguish thin-wall and thick-wall models; identify required Poisson/material fields and pressure-boundary/support load paths. Return an exact recommended reference interpretation plus the remaining physical and compatibility choices requiring Owner vetting.

Completion does not mean project adoption, source repair, public compatibility selection, code-compliance acceptance, or blanket pressure-physics closure.

## Declared reads and tools

Reads: root/project instructions; `agents/AGENT_WORKING_ITEMS.md`; frozen root controls; committed loop plan; live `DEL-05-03` SOW/context/status/references/dependencies and registers; accepted E1 pressure packet; relevant current pressure/load/stress source and existing witnesses; primary authoritative engineering publications retrieved directly from their publishers. `domains/piping-design` OCR/extracted equations are excluded.

Tools: repository search/read, read-only Git and source inspection, deterministic local arithmetic/check scripts whose outputs remain run evidence, web search/read of primary authoritative publications, `apply_patch` within allowed targets, and at most one bounded Agent 2 independent derivation/check under the required model/configuration.

## Exact write scope

- `{RUN_ROOT}/instances/P5/**`.
- `{WORKING_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_run_records/PHYSICS_UI_EXECUTION_20260908/**`.

No production source/tests, thresholds, public-output migration, DAG/local status/authority/pointer/register/receipt edits, other instance subtrees, or historical evidence changes. Cross-package information routes to `/root` only.

## Return and fan-in criteria

Required outputs are an investigation report, reproducible independent calculations, source inventory with retrieval/applicability, current-output comparison kept downstream of frozen expectations, validation evidence, lean manifest, status, handoff, and manager return. All material claims distinguish observation, derivation/inference, candidate recommendation, and accepted state. Any future implementation needs Owner-vetted physical and compatibility choices and a separately authorized brief.
