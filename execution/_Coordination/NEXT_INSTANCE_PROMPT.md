# NEXT INSTANCE PROMPT

## Current Authority

- Read `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/_COORDINATION.md` and follow the Application Integration And Issuance Loop directions.
- Discover current state from authoritative surfaces named in `_COORDINATION.md`: `_DAG/_LATEST.md`, approved DAG artifacts, deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`, dependency/review files, current app/build/test surfaces, and current aggregation or review artifacts as needed.
- Treat blockers or dirty git state outside this project/write scope as external-scope noise; record and bypass, do not fix.

## Governing Imperatives

1. `SOFTWARE_DECOMP` says what must be built and why.
2. `DAG-006` says what depends on what, using approved active edges.
3. Deliverable-local `_STATUS.md` files say the current lifecycle state for
   issuance and review gates.
4. `_COORDINATION.md` says how to execute work: app-integration tranches by
   default, local-status discovery, DAG-guided context selection, bounded
   workers, fan-in, validation, and evidence records.
5. Current lifecycle state is application-integration ready: all deliverables
   are `CHECKING` or `ISSUED`; `DEL-01-01` is the sole currently `ISSUED`
   deliverable and remains the accepted governance baseline unless a
   human-approved change path opens it.

## Next Action

1. Enter through the coordination workflow:
   - read this prompt;
   - read `_COORDINATION.md`;
   - perform the baseline authority intake defined in `_COORDINATION.md`;
   - add application-integration, execution, review, or issuance intake
     documents according to the selected tranche type;
   - run `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`;
   - record `git status --short` before coordination-sensitive planning or
     execution.
2. Use the Application Integration And Issuance Loop in `_COORDINATION.md` as
   the governing workflow. If a human-approved implementation, app-integration,
   review, issuance, or release-readiness tranche is already active, continue
   that tranche within its write bounds.
3. If no active human-approved tranche exists, propose exactly one next bounded
   tranche:
   - default to an application-integration tranche aimed at the working desktop
     application;
   - use `CHECKING` deliverables as mature design and review context;
   - select from `CHECKING` only when the human asks for lifecycle closeout or
     issuance review;
   - do not select `ISSUED` deliverables for ordinary work.
4. For app-integration work, inspect current app/build surfaces first:
   `package.json`, `apps/desktop/package.json`, `apps/desktop/src/**`,
   `apps/desktop/src-tauri/**`, `apps/desktop/SMOKE.md`,
   `core/product_physics/**`, and `fixtures/product_preview/**` as needed.
   Use `DAG-006` only to discover upstream/downstream deliverable context, then
   inspect the relevant deliverable-local files directly.
5. Inspect in detail any app, core implementation, fixture, test, or evidence
   files needed to decide what to do next.
6. A working desktop application tranche should preserve the technical-preview
   boundary: local-only invented data, local project storage, preview
   mechanics, results/diagnostics/report/proposal context, and no release,
   professional, certification, sealing, authentication, or code-compliance
   claims.
