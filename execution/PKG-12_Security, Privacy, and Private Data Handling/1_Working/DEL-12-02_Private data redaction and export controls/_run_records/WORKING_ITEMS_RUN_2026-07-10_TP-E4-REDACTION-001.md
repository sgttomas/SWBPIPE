---
doc_id: WORKING-ITEMS-RUN-2026-07-10-TP-E4-REDACTION-001
doc_kind: task.run_record
run-status: SUCCESS
created: 2026-07-10
agent_type: BOUNDED_IMPLEMENTATION
tranche_id: TP-E4-REDACTION-001
deliverable_id: DEL-12-02
package_id: PKG-12
scope_path: execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls
---

# WORKING ITEMS RUN - TP-E4-REDACTION-001 (completion-plan row E4)

## Mandate echo

Ordinary in-stage R5 work under `DEC-054`; completion-plan row E4
(`plans/PLAN_2026-06-17_prd_completion.md`, Phase E table), no decision gate.
Row text at run start (verbatim): "**Private-data redaction + export
controls** (DEL-12-02, PRD §17.3/§18.3): redaction of private
rule/material/component values from bug reports and shared examples; clear
warnings before exporting private data; checksums; private-by-default. Test
surface `tests/security/test_redaction_export_controls.py` exists".

Run-record location basis: DEL-12-02 "Private data redaction and export
controls" owns the E4 scope — the row cites DEL-12-02 directly, and all prior
redaction tranche run records (`TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`,
setup/reconciliation records) live in this deliverable.

Hard fences honored: F-PIP-1 (local-only — no network behavior added, panel
transports are local browser data-URI downloads); F-PIP-2 (warnings inform,
they do not certify — informational-boundary language asserted by test);
user-entered/invented data only; no new external dependencies (no
`package.json`/`requirements-dev.txt` change).

## Step 1 — Assessment: row scope vs. live tree vs. prior run records

Row requirement decomposition against the live tree at branch start
(`origin/main` = `3c7f26462`):

| Row requirement | Live-tree state before this run | Genuine remainder |
|---|---|---|
| Redaction of private rule/material/component values from shared/public exports | **Done in core.** `core/security/redaction/controls.py` (626 lines): metadata-only classification, five export contexts, six actions, sixteen governed reason codes; hardened 2026-06-07 (`TASK_RUN_2026-06-07_0935`) for storage/privacy metadata gates. `schemas/redaction_export_controls.schema.yaml` governs vocabulary. `tests/security/test_redaction_export_controls.py` (10 tests) passed at run start. | No user-facing binding: nothing in `apps/desktop/src/**` consumed the contract; the Export Safety Review panel reported a static redaction posture string only. |
| Clear warnings before exporting private data | Core emits machine-readable findings; no UI rendered them; no export surface was gated by them. | **Yes — the app-side binding.** |
| Checksums | Hash seam exists (`hashService`/`usePackageHash` via wasm `canonical_sha256_hex`); rule-pack checksum metadata preserved by core redaction. Not bound to a redaction export artifact. | Bind checksum to the offered redacted manifest. |
| Private-by-default | Core: `local_private` retention requires explicit intent; unknown metadata never passes silently. No app default existed. | App context default + explicit-intent control. |
| Test surface exists | Yes (python). No app-side test surface; no cross-language parity pin. | Parity fixture + vitest surfaces. |

Prior-run-record check (grep over `execution/*/1_Working/*/_run_records/`):
DEL-12-02 records confirm the deferred scope list "runtime report/export
integration; GUI controls; UI/CLI approval flow" was never picked up by a
later tranche. Loop receipt 7 (2026-07-10) records the delta note
"Completion-plan E4 row understates the live tree (`core/security/redaction`
+ schema + docs exist, tests pass)".

**Assessment result:** the genuine remainder is exactly the app-side binding —
user-facing redaction warnings and export-workflow integration plus the
core-side seam the binding needs (a cross-language parity pin, since the
desktop app cannot execute the python contract). This matches the tranche
mandate's description; no material delta was found, with one refinement: the
natural "core-side seam" is a shared decision corpus consumed by both suites
(following the existing `fixtures/canonical_hash/cases.json` pattern), not a
second runtime seam.

"Bug reports" note: the app has no bug-report emission surface anywhere; the
export workflow (shared examples / shared models / downstream exports /
public report context) is the surface that exists, and the binding covers all
five governed contexts including `public_report`/`public_example`. A future
bug-report surface must route through the same contract (REXC-REQ-012).

## Step 2 — What was implemented

1. **Shared parity corpus (core-side seam):**
   `fixtures/redaction_export_controls/cases.json` — 23 invented cases
   covering all 16 governed reason codes across contexts, call-level and
   item-level local-private intent, provenance-carrier metadata, missing
   metadata, and every storage-boundary gate.
2. **Python parity test:** `test_shared_parity_fixture_matches_core_decisions`
   added to `tests/security/test_redaction_export_controls.py`; asserts
   decision action/reason/metadata-presence, finding class/severity,
   non-mutation, and that the corpus exercises the complete schema ReasonCode
   enum. Corpus added to the forbidden-content scan list.
3. **TypeScript mirror:**
   `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
   — full port of `classify_export_item` / `redact_export_payload` semantics
   (constants, metadata gates, action ladder, finding vocabulary, walk,
   apply-action stripping, summary).
4. **User-facing panel:**
   `apps/desktop/src/features/redaction-controls/RedactionExportControlsPanel.tsx`
   — export-context selector defaulting to `local_private`
   (private-by-default), explicit local-private intent control (disabled
   outside `local_private`), every warning/blocking finding rendered before
   any artifact is offered, download link withheld entirely while any
   blocking finding exists, redacted manifest (schema-vocabulary
   control-profile/field-policy/export-run members + redacted payload +
   boundary flags) offered as a local data-URI download with a canonical
   sha256 checksum via `usePackageHash` (same wasm hash seam as the engines),
   and the F-PIP-2 informational-boundary note.
5. **Wiring:** panel added to the export section of `apps/desktop/src/App.tsx`
   beside `ExportReviewPanel`.
6. **Vitest surfaces:**
   `redactionExportControls.test.ts` (corpus parity — 23 cases — plus walk
   non-mutation, local-intent gating, unsafe-detail stripping, loud
   unsupported-context rejection) and
   `RedactionExportControlsPanel.test.tsx` (6 behavior tests: default
   private-by-default blocking, explicit-intent retention with visible
   warning and manifest boundary flags, public-context redaction of
   user-entered values with invented-example records retained, disabled
   intent outside `local_private`, checksum, boundary language).
7. **Docs / plan / memory:** "Application Binding" section added to
   `docs/security/redaction_export_controls.md`; completion-plan row E4
   updated to LANDED with named residuals; DEL-12-02 `MEMORY.md` entry added.

## Deltas (live tree wins)

- The "core-side seam the binding needs" landed as the shared parity corpus
  rather than any runtime bridge: the desktop preview is a browser/wasm
  surface and cannot call the python module, and the repo already pins the
  hash seam cross-implementation via a shared fixture corpus
  (`fixtures/canonical_hash/cases.json`). No core python code needed changing;
  the core contract was already complete for this row.
- Model entities carry a user-entered `provenance` string, not the schema's
  privacy metadata; the panel maps provenance to explicit metadata
  conservatively (only the explicit invented-example marker maps to safe
  classes; any other recorded provenance is private-by-default; absent
  provenance stays `unknown` so the contract warns instead of leaking).

## Validation

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest tests/security/test_redaction_export_controls.py -q` — 11 passed (was 10).
- Full python suite — recorded in final checks below.
- `npm ci` + `npm run build:wasm` + `npx tsc -b` — clean.
- `npm test` (apps/desktop, full vitest) — 22 files, 451 tests passed
  (was 21 files / 417 tests before this tranche).
- Repo-root harness self-check, practitioner-harness pytest, and DEC-025
  clean-head evidence sweep — recorded in the final checks section below
  after commit.

## Final checks (committed clean head)

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest tests/ -q` — PASS (see PR).
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py self-check` (repo root) — exit 0.
- Practitioner-harness pytest — PASS.
- DEC-025 sweep at committed clean head from project root
  (`python3 tools/release/run_evidence_sweep.py --execute`) — artifact name
  recorded in the PR body and final report.

## Residuals (explicit, not silently dropped)

- Runtime binding of the report/handoff/CLI emission routes through the same
  redaction seam beyond the export-workflow surface (REXC-REQ-012 breadth);
  the panel is the user-facing export-workflow binding, not a global
  interceptor on every existing export panel.
- Destructive quarantine movement workflow, optional encrypted private-library
  storage, and the §17.5 legal review workflow remain out of scope (E6/E7 and
  human-gated surfaces).
- A future bug-report surface must consume the same contract when it exists.

## Boundary statement

No lifecycle transition, no register change, no release/clearance claim.
Redaction warnings inform the local export decision only; nothing here
certifies redaction sufficiency, legal clearance, release readiness, or
professional acceptance. All fixture data is invented; classification is
explicit-metadata-only; the source model is never mutated; nothing is
transmitted.
