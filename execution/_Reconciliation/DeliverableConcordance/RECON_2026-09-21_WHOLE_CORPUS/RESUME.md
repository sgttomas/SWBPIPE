# Resume — RECON_2026-09-21_WHOLE_CORPUS

1. Read `RUN_BASIS.md` (what is frozen and why), `CONVENTIONS.md` (the bound
   rules), `CANONICAL_SITUATIONS.md`, and the orchestration record
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/`
   (`HANDOFF_STATE.md`, `WORK_GRAPH.json`, `PLAN.md`).
2. Replay `RUN_STATE.jsonl`: the last `PHASE_OPEN` names the phase; `LAUNCH`
   events without a matching `RETURN` are in flight (agent contexts do not
   survive a session; relaunch from the stored brief with a fresh agent).
   Count live agents against the latest `CAP` event before launching more.
   The latest `BOUND_INPUTS` event gives the current tool and data hashes.
3. Claim keys: `CLAIM_KEYS_V2.csv` (extractor v2). Canonical assignments:
   `CANONICAL_ASSIGNMENTS.csv`; evidence map: `EVIDENCE_MAP.csv` (both from
   `tools/build_canonical.py`). Rerun either tool to confirm byte identity
   before relying on it.
4. Validate every ledger with `tools/validate_ledger_v2.py` (single mode, then
   `--batch` across the sub-batch). A ledger without its `#END` sentinel or
   failing validation is discarded and rerun by a fresh worker, never patched.
   `tools/validate_ledger.py` and `CLAIM_KEYS.csv` are v1 calibration
   provenance only.
5. Before any sub-batch, diff the frozen Piping tree against `origin/main`
   (`git diff --stat 00115c71931bcae79909602d653740d3bb72dfa1 origin/main --
   projects/chirality-piping ':!projects/chirality-piping/execution'`); any
   product change marks affected rows `STALE_INPUT`.

6. R1 regenerators: `tools/build_r1_indexes.py`, `tools/merge_inventories.py`
   (`IMPLEMENTATION_SURFACES.csv`), `tools/build_path_hints.py`
   (`ROUTING_PATH_HINTS.json`), `tools/route_capabilities.py` (`ROUTING/`,
   `ROUTING_SAMPLE/`). Workers and managers never read `ROUTING_SAMPLE/`.
7. R2 waves follow `WAVE_PLAN.md` and the briefs `R2-MANAGER_brief.md`,
   `R2-WORKER_brief.md`, `R2-VERIFIER_brief.md` in the orchestration record.

Current phase: R1 complete; R0/R1 PR in review. Next: merge, then R2 wave 1.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
