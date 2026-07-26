---
doc_id: CB-2026-07-25-DEL-07-06-PACKAGED-EDITED-LOAD-SMOKE-001
doc_kind: coordination.candidate_brief
status: candidate_not_adopted
prepared: 2026-07-25
package_id: PKG-07
deliverable_id: DEL-07-06
accepted_basis: HEAD@2f8d35ceb30da734ca6dff24dcab36dded8c9b35, Receipt-70, R5, DAG-008
---

# CANDIDATE Brief — DEL-07-06 packaged edited-load smoke

**Status:** `CANDIDATE / NOT ADOPTED`

**Prepared by:** WORKING_ITEMS `WI-PKG07-DEL0706` for HELP_HUMAN

**Managed run:** `HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18`

This document is proposal-only. It authorizes no execution, implementation,
write, branch, lifecycle action, release claim, or use of owner standing
approval.

## 1. Purpose and accepted basis

Close only this DEL-07-06 `_STATUS.md ## Remaining` item after exact owner
adoption and successful evidence fan-in:

> Run a packaged-Tauri smoke over a saved project with edited load data
> (source: PRD plan §4 FR-007 A4 residual / seam plan §9.5)

The frozen basis is clean detached
`2f8d35ceb30da734ca6dff24dcab36dded8c9b35`, valid Receipt-70, target stage
R5, approved active DAG-008, and the PROJECT_SETUP recomputation that all seven
active DEL-07-06 `EXECUTION / UPSTREAM` rows are `SATISFIED`. DEL-07-06 is
`SOW_V1`; its two independent-usability/measurable-target holds remain
separate and unchanged.

Existing evidence establishes the component seams but not this packaged GUI
journey:

- `apps/desktop/src-tauri/src/lib.rs` contains native saved-project and
  saved-edited-load regressions;
- `apps/desktop/e2e/r2-smoke.spec.ts` exercises browser-visible authoring,
  save, and reopen surfaces but cannot substitute for the packaged webview;
- `apps/desktop/SMOKE.md` records the macOS bundle command and the
  `tauri-driver` limitation.

## 2. Exact future execution objective

At the adopted clean SHA, build the macOS packaged Tauri `.app` and use only
the invented repository fixture:

- project: `project:invented-loop-01`;
- primitive load: `load:L-100-Y`;
- original magnitude: `350 N`;
- edited magnitude: `425 N`.

In one packaged-runtime GUI journey:

1. launch the recorded bundle and confirm the actual Tauri backend route;
2. create/save the invented project locally;
3. queue and apply the exact `350 N -> 425 N` edit through the structured
   operation seam;
4. save the edited project, quit the application, relaunch it, and reopen that
   project;
5. confirm the reopened primitive load remains exactly `425 N` with its unit,
   no queued edit, and no stale pre-edit result represented as current;
6. run mechanics after reopen and require `MECHANICS_SOLVED`, a
   `tauri_backend_job` solve seam, the same project/model identity, and
   non-empty result rows; and
7. record pass/fail evidence, operator, adopted SHA, bundle path, executable
   SHA-256, exact observed values/messages, and every deviation.

This is an evidence-only smoke. A failed predicate is evidence and a routing
trigger; it does not authorize repair.

## 3. Future execution write fence

After exact owner adoption, the future execution may write only:

- `projects/chirality-piping/apps/desktop/SMOKE.md`;
- one new DEL-07-06 `_run_records/WORKING_ITEMS_RUN_<allocated-id>.md`;
- optional sanitized evidence assets below the same deliverable's
  `_run_records/assets/<allocated-id>/**`;
- DEL-07-06 `MEMORY.md`;
- DEL-07-06 `_STATUS.md`, only after accepted evidence, removing exactly the
  packaged-smoke bullet and adding its history entry; and
- the one loop receipt required by adopted closeout.

Required transient writes are limited to existing ignored build products under
`apps/desktop/public/wasm-engine/**`, `apps/desktop/dist/**`, and
`apps/desktop/src-tauri/target/**`, plus an isolated/disposable macOS
app-local-data context for `org.openpipestress.technical-preview`. The
execution must not use an unexamined existing user store because the fixed
invented project ID is persisted by upsert.

A Bash-bearing execution child is serialized as the project-root integration
owner. No concurrent child may write an overlapping path.

## 4. Exclusions

- No product code, schema, solver, fixture, persistence contract, package
  configuration, manifest/lockfile, Playwright configuration, or test change.
- No automation-harness development or substitution of browser/dev/dist
  Playwright for the packaged webview.
- No private project data, protected content, cloud, network, daemon,
  telemetry, or repository-default user-data copy.
- No report journey, broad A12 rehearsal, or unrelated UI review.
- No selection of a WCAG or other measurable target; no independent usability
  validation; no promotion of PDU-045, PDU-046, or
  `DEL-07-06-RQ-004`.
- No lifecycle/stage/issuance/release/reproduction/prover/professional,
  certification, sealing, authentication, or code-compliance effect.
- No defect repair, dependency installation from the network, Git action, or
  external publication.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 5. Environment and tool prerequisites — current execution hold

Execution is currently `HOLD / NOT OFFLINE-EXECUTABLE` in the R18 checkout:

- project and desktop `node_modules` are absent;
- the production `dist` and packaged `.app` are absent;
- `wasm-bindgen` is not available on `PATH`;
- `tauri-driver` does not support macOS packaged-webview automation; and
- recent native automation evidence records that `osascript` lacked macOS
  assistive access.

Before release, record a clean adopted SHA and prove locally available,
lockfile-compatible Node/Rust/wasm tooling without network access. Provide a
macOS GUI session and either a human operator or an explicitly authorized
automation caller with Accessibility permission. Provide an isolated local
project store or an accepted non-destructive handling plan. Any failed offline
dependency probe stops; it does not authorize network installation.

## 6. Acceptance evidence

The future return is accepted only when all are true:

1. A fresh `.app` is built from the adopted clean SHA; bundle path and
   executable SHA-256 are recorded.
2. Evidence proves the actual packaged Tauri webview and backend routes, not a
   browser substitute.
3. The queued operation records before `350 N`, after `425 N`, explicit unit,
   invented provenance/rationale, `tauri_backend_apply`, and no professional
   approval.
4. Save, quit/relaunch, reopen, and post-reopen inspection prove `425 N`
   persisted with unit metadata and no queued operation.
5. Post-reopen solve records `state=completed`, `MECHANICS_SOLVED`,
   `tauri_backend_job`, matching project/model identity, and result rows
   greater than zero.
6. Evidence contains no private/protected content and records the local-only,
   no-network/no-telemetry posture.
7. The SMOKE row and DEL-07-06 run record agree on SHA, bundle hash, operator,
   observations, and pass/fail.
8. Only after WORKING_ITEMS validates that fan-in may the exact residual be
   removed; the two PDU holds remain byte-semantically unchanged.

## 7. Proposed checks

From `projects/chirality-piping`, after prerequisites pass:

```sh
npm test --workspace apps/desktop
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml saved_edited_load_model_round_trips_and_solves_from_restored_payload
cd apps/desktop
npm run tauri -- build --bundles app
test -x 'src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop'
shasum -a 256 'src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop'
open 'src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app'
```

After the manual/native smoke and evidence-only edits, from repository root:

```sh
python3 tools/practitioner_harness/harness.py self-check
PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider -q tools/practitioner_harness
python3 tools/validation/validate_piping_loop_receipts.py --repo-root .
git diff --check
git status --short
```

Because `apps/desktop/SMOKE.md` is under `apps/desktop/**`, the registered
`desktop-test` and `desktop-build` checks apply. The DEC-025 sweep is not
required for this evidence-only brief. If any executable surface must change,
stop and return for a separately adopted code-touching brief; that later brief
must include the DEC-025 sweep.

## 8. Defect routing

| Observed defect | Route; no repair under this brief |
|---|---|
| package/build configuration or bundle failure | PKG-10 / DEL-10-04 |
| local save/reopen or migration failure | PKG-02 / DEL-02-05 |
| load semantics or unit-bearing load defect | PKG-05 owning load deliverable |
| load-manager or accessibility/usability presentation defect | PKG-07 / DEL-07-02 or DEL-07-06, requiring a new brief |
| solve execution/diagnostic defect | PKG-07 / DEL-07-07 |
| analysis-run/result binding defect | PKG-14 / DEL-14-02 |
| structured operation validation/apply defect | PKG-16 / DEL-16-02 or DEL-16-03 |
| test-strategy/evidence-posture defect | PKG-00 / DEL-00-08 |
| private/protected-data exposure | STOP and escalate to HELP_HUMAN |

## 9. Adoption gate, stop conditions, and rerun triggers

Only the human owner may adopt this governed brief. Until an exact adoption
binds its bytes and commit, `Status=CANDIDATE / NOT ADOPTED`; no standing
approval is activated and no execution is released.

Stop and return without repair if:

- the adopted SHA, fixture IDs/values, residual text, DAG basis, or write fence
  differs;
- offline prerequisites, GUI access, isolated storage, build, hash, or any
  acceptance predicate fails;
- execution needs a forbidden or cross-package write;
- existing private data may be exposed or overwritten;
- browser evidence would be substituted for packaged evidence; or
- a lifecycle, release, acceptance, normative, or professional judgment is
  requested.

Rerun from a fresh clean adopted SHA and fresh isolated store after any bundle,
frontend, operation-seam, persistence, load-manager, solve, fixture, evidence
posture, or prerequisite change. Preserve failed evidence and record the rerun
reason; never overwrite or silently reinterpret it.

Successful execution closes only the named residual. It creates no lifecycle,
stage, release, issuance, professional-reliance, or accessibility-conformance
effect.
