# N3C Return — Candidate v3 independent refutation

**Verdict:** `BLOCK`

The wrapper-owned-intent correction is adequate: v3 strips top-level and
nested payload intent before passing wrapper intent to the unchanged contract;
the frozen parity corpus retains item-intent cases, and local-private unknowns
remain `warning_only`.

`AFFECTED_OWNER_MAP.csv` remains materially inexact against the accepted
reconciliation register:

1. The combined release-tools row has only six fields under the seven-column
   schema; the deliverable/effect fields shift and the final disposition is
   absent. Its paths are separately registered as `SURF-223`
   (`DEL-08-05;DEL-10-04`) and `SURF-217`
   (`DEL-01-03;DEL-10-04`), not `SURF-109`.
2. `core/adapters/framework/adapter_framework.py` cites desktop `SURF-009`;
   its registered core surface is `SURF-067`, attributed to
   `DEL-10-02;DEL-10-04`.
3. `apps/desktop/src/services/reportRenderService.ts` cites feature-folder
   `SURF-038`; its registered surface is `SURF-059`, attributed solely to
   `DEL-08-01`.
4. The post-adoption write fence names `fixtures/redaction_export_controls`
   (`SURF-161`) and `schemas/redaction_export_controls.schema.yaml`
   (`SURF-198`), but the claimed exact owner map omits both.

Route dispositions, held authority, preservation constraints, N4/N5
sequencing, and post-N5 closeout otherwise remain intact. Owner adoption would
authorize an inaccurate cross-package map, so v3 is not ready for Gate 3.

