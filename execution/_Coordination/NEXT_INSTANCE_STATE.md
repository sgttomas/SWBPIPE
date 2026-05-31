# NEXT INSTANCE STATE

Last updated: 2026-05-31
Updated by: WORKING_ITEMS

## Authority Pointers

- Decomposition authority:
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`.
- Graph authority: `execution/_DAG/DAG-005/`, as pointed to by
  `execution/_DAG/_LATEST.md`.
- Graph approval record: `execution/_DAG/DAG-005/APPROVAL_RECORD.md`.
- Coordination record: `execution/_Coordination/_COORDINATION.md`.
- Implementation evidence:
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- Blocker queue derivatives:
  `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` and `.csv`.
- Latest integrated verification snapshot:
  `execution/_Aggregation/TP-INTEGRATED-VERIFY-002_2026-05-31/`.
- Latest aggregation pointer:
  `execution/_Aggregation/_LATEST.md` now points to
  `TP-INTEGRATED-VERIFY-002_2026-05-31`.
- Latest release gap refresh:
  `execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/`.
- Latest lifecycle readiness audit:
  `execution/_Aggregation/TP-LIFECYCLE-READINESS-AUDIT-001_2026-05-31/`.
- Latest next-tranche selection:
  `execution/_Coordination/TP-NEXT-TRANCHE-SELECTION-001_2026-05-31.md`.
- Handoff prompt: `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.

## Current Program State

- `SOFTWARE_DECOMP` says what must be built and why.
- `DAG-005` says what depends on what, using approved active edges.
- `DEV-001` says what is currently unblocked or blocked for implementation
  based on committed evidence.
- `_COORDINATION.md` defines the Integrated Verification and Tranche Selection
  Loop: authority intake, state verification, optional read-only integrated
  verification snapshot, gap-to-tranche selection, human approval, bounded
  execution, fan-in, validation, evidence updates, and handoff.
- `DAG-005` remains approved active graph authority. Candidate rows are
  non-gating unless later promoted by explicit human gate and graph
  revalidation.
- `DEV-001_BLOCKER_QUEUE.md` reports 101 implementation-unblocked deliverables
  and 0 blocked deliverables using active `DAG-005` edges only.

## Recent Context

- `TP-VERIFY-017` remediated the three `TP-VERIFY-016` release-readiness gaps:
  witness tooling import surface, DEL-12-04 security wording, and DEL-10-04
  coordination regression command surface.
- `TP-VERIFY-017` recorded `TP-VERIFY-017-RESIDUAL-001`: the DEL-11-04
  invented fake-rule model fixture no longer validated against the current
  model schema and persistence-envelope checks.
- `TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31` was approved and executed by
  WORKING_ITEMS. The tranche aligned the two DEL-11-04 invented model fixtures
  with current straight-pipe orientation and load-record schema requirements,
  recomputed their project hashes, and recorded closeout in
  `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-04_Invented educational example models/_run_records/TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31.md`.
- Focused validation passed for the DEL-11-04 schema-example remediation:
  `python3 -m pytest tests/test_invented_example_models.py -q` (`7 passed`),
  `python3 -m pytest tests/test_model_schema.py -q` (`4 passed`),
  `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
  (`VALID`), and `git diff --check`.
- `TP-INTEGRATED-VERIFY-001_2026-05-31` ran the next read-only integrated
  verification snapshot under
  `execution/_Aggregation/TP-INTEGRATED-VERIFY-001_2026-05-31/`.
- The provider-neutral release readiness profile passed:
  `python3 tools/release/check_release_readiness.py --profile all --execute`
  completed all 29 planned checks successfully, including 269 repository
  Python contract tests, security/privacy tests, coordination tests, DAG schema
  validation, release-readiness script tests, and 24 discovered crate-local
  Cargo test surfaces.
- `TP-INTEGRATED-VERIFY-001_2026-05-31` observed a local desktop bootstrap
  blocker:
  `npm run test:desktop` failed with `sh: vitest: command not found`, and
  `npm run build:desktop` failed with `sh: tsc: command not found`. No
  `node_modules` or `apps/desktop/node_modules` directory was present during
  verification.
- `TP-DESKTOP-BOOTSTRAP-001_2026-05-31` addressed the desktop bootstrap gap by
  running `npm ci` from the project root, then rerunning supplemental desktop
  checks. `npm run test:desktop` passed 1 Vitest file / 5 tests, and
  `npm run build:desktop` completed the TypeScript/Vite production build.
- `npm audit --omit=dev --audit-level=moderate` reports zero production
  vulnerabilities. Full `npm audit --audit-level=moderate` reports 6 moderate
  dev-tooling vulnerabilities through `vitest`/`vite`/`esbuild` and `ws`;
  remediation may require dependency/lockfile changes and should be handled as
  a separate bounded dependency-maintenance tranche.
- `TP-DESKTOP-DEPS-001_2026-05-31` addressed the dev-tooling vulnerabilities:
  `npm audit fix` updated `ws` to `8.21.0`, and the desktop workspace upgraded
  `vitest` from `^2.1.0` to `^4.1.7`. Full
  `npm audit --audit-level=moderate` now reports zero vulnerabilities.
- Desktop validation still passes after the dependency update:
  `npm run test:desktop` passed 1 Vitest file / 5 tests, and
  `npm run build:desktop` completed the TypeScript/Vite production build.
- Current integrated verification status after bootstrap and dependency
  maintenance is `PASS_FOR_EXECUTED_CHECKS`. No lifecycle state, DAG artifact,
  DEV-001 evidence row, blocker queue, release record, acceptance record,
  professional claim, or code-compliance claim was changed.
- `TP-INTEGRATED-VERIFY-002_2026-05-31` was approved and executed by
  WORKING_ITEMS as a read-only integrated verification derivative snapshot.
  DEV-001 coordination freshness passed, full `npm audit --audit-level=moderate`
  reported zero vulnerabilities, the provider-neutral release readiness profile
  passed all 29 planned checks, `npm run test:desktop` passed 1 Vitest file / 5
  tests, `npm run build:desktop` completed the TypeScript/Vite production
  build, and `git diff --check` passed.
- `TP-INTEGRATED-VERIFY-002_2026-05-31` recorded no open verification gap in
  executed checks. Its status is `PASS_FOR_EXECUTED_CHECKS`; it is not a
  release, acceptance, professional-reliance, or code-compliance claim.
- `TP-AGG-LATEST-POINTER-001` updated `execution/_Aggregation/_LATEST.md` to
  point to `TP-INTEGRATED-VERIFY-002_2026-05-31` as the latest aggregation
  snapshot. This was pointer hygiene only and did not change decomposition,
  DAG, DEV-001, lifecycle, release, acceptance, professional, or
  code-compliance authority.
- `TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31` refreshed the May 11
  release-readiness gap register against May 31 evidence. It closed only the
  transient executed-check gaps directly supported by May 31 evidence and
  preserved release, lifecycle-complete, professional-reliance, CI authority,
  artifact archive, and release-threshold items as open or human-gated.
- `TP-LIFECYCLE-READINESS-AUDIT-001_2026-05-31` audited 101 deliverables
  read-only. DEV-001 reports 0 blocked deliverables. The audit found 7 missing
  `MEMORY.md` files, all in PKG-00 architecture-basis surfaces; 100 local
  status-text surfaces differ from the DEV-001 lifecycle view; 84 deliverables
  were classified as `STATUS_TEXT_STALE_NEEDS_HUMAN_REVIEW`; 9 PKG-17
  deliverables were classified as `HUMAN_GATE_REQUIRED`; and 8 PKG-00
  deliverables were classified as `ARCHITECTURE_BASIS_NO_ACTION`.
- `TP-NEXT-TRANCHE-SELECTION-001_2026-05-31` selected exactly one next bounded
  tranche: `TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-05-31`. The selected
  tranche was not executed.
- `execution/_Coordination/_COORDINATION.md` now records the canonical
  Integrated Verification and Tranche Selection Loop for steering development
  through remaining objectives.
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` was updated to initiate the
  loop before proposing work.

## Immediate Next Actions

1. Read `NEXT_INSTANCE_PROMPT.md`.
2. Read `_COORDINATION.md` and this `NEXT_INSTANCE_STATE.md`.
3. Read `SOFTWARE_DECOMP`, `DAG-005`, and `DEV-001` surfaces as stipulated.
4. Apply the Integrated Verification and Tranche Selection Loop from
   `_COORDINATION.md`.
5. Run `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
   and record `git status --short` before coordination-sensitive tranche
   selection.
6. No pinned implementation blocker or open executed-check verification gap
   remains in this handoff state after
   `TP-INTEGRATED-VERIFY-002_2026-05-31`.
7. If development continues and the human approves lifecycle/status-surface
   work, execute only the selected tranche
   `TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-05-31` within the write bounds and
   human-gate requirements recorded in
   `execution/_Coordination/TP-NEXT-TRANCHE-SELECTION-001_2026-05-31.md`.
8. Do not implement the selected tranche or change lifecycle files until
   explicitly approved.

## Active Residuals

| ID | Owner | Summary | Required next action |
|---|---|---|---|
| `TP-VERIFY-017-RESIDUAL-001` | `DEL-11-04` / schema-example alignment | Locally remediated by approved `TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31` and verified through `TP-INTEGRATED-VERIFY-001_2026-05-31` and `TP-INTEGRATED-VERIFY-002_2026-05-31`; provider-neutral release readiness profile passed. | No bounded implementation action remains for this residual; do not convert this into a release/reliance claim without the normal human release/acceptance gates. |
| `TP-INTEGRATED-VERIFY-001-GAP-001` | `PKG-10` / `PKG-07` desktop workspace bootstrap | Closed by `TP-DESKTOP-BOOTSTRAP-001_2026-05-31`; root `npm ci` restored local Node dependencies, `npm run test:desktop` passed 5 tests, and `npm run build:desktop` completed. | No further bootstrap action required unless dependencies or local environment are removed. |
| `TP-DESKTOP-BOOTSTRAP-001-GAP-001` | `PKG-10` / `PKG-07` desktop dependency maintenance | Closed by `TP-DESKTOP-DEPS-001_2026-05-31`; full `npm audit --audit-level=moderate` now reports zero vulnerabilities, and desktop test/build checks pass. | No further dependency-audit action required for the current installed desktop workspace; rerun audit/test/build after future dependency changes. |
| `AGG-LATEST-POINTER-001` | `execution/_Aggregation/_LATEST.md` | Closed by `TP-AGG-LATEST-POINTER-001`; pointer now references `TP-INTEGRATED-VERIFY-002_2026-05-31`. | No further pointer-hygiene action required for the current aggregation pointer. |
| `RGAP-001` through `RGAP-007` | Release, lifecycle, CI, artifact, runtime, and professional gates | Preserved as open or human-gated by `TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31`; May 31 passing checks do not close release/professional/lifecycle-complete claims. | Handle only through future human-approved release, lifecycle, CI, artifact, runtime, or professional-boundary tranches. |
| `LIFE-AUDIT-001` | Deliverable lifecycle/status hygiene | `TP-LIFECYCLE-READINESS-AUDIT-001_2026-05-31` found stale local status text relative to DEV-001 lifecycle and a PKG-17 human-gated lifecycle disposition need. | Selected next tranche is `TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-05-31`; do not execute until explicitly approved. |

## Do Not Change Without Explicit Human Approval

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- candidate-edge promotion or graph authority;
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv`;
- blocker queues except through approved deterministic coordination workflow;
- release records or acceptance records;
- professional, certification, sealing, authentication, code-compliance, or
  release-readiness-for-reliance claims.
