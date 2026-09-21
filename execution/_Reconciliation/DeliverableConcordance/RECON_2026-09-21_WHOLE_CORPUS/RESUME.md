# Resume — RECON_2026-09-21_WHOLE_CORPUS

1. Read `RUN_BASIS.md` (what is frozen and why) and the orchestration record
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/`
   (`HANDOFF_STATE.md`, `WORK_GRAPH.json`, `PLAN.md`).
2. Replay `RUN_STATE.jsonl`: the last `PHASE_OPEN` names the phase; `LAUNCH`
   events without a matching `RETURN` are in flight (their agent contexts do
   not survive a session; relaunch from the stored brief with a fresh agent).
   Count live agents against the latest `CAP` event before launching more.
3. Validate every ledger present with `tools/validate_ledger.py`; a ledger
   without its `#END` sentinel or failing validation is discarded and rerun by
   a fresh worker, never patched.
4. Before any sub-batch, diff the frozen Piping tree against `origin/main`
   (`git diff --stat 00115c71931bcae79909602d653740d3bb72dfa1 origin/main --
   projects/chirality-piping ':!projects/chirality-piping/execution'`); any
   product change marks affected rows `STALE_INPUT`.

Current phase: R0 calibration. Next: see the last events in `RUN_STATE.jsonl`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
