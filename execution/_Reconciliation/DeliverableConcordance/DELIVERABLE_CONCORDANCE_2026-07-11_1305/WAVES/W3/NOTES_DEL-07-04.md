# NOTES — DEL-07-04 Missing-data warning and blocking UX (R2 wave W3)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-07 / DEL-07-04, status IN_PROGRESS (UX_UI_SLICE).

**NormativeSource path alias:** `SPEC` = the deliverable's `Specification.md`;
requirement rows cite `Specification.md#Requirements | R-DEL-07-04-0NN` (native
ID recorded in `NormativeSource` per addendum 12; ClaimID is the fixed
`DEL-07-04-REQ-NNN` form). All `ImplementationEvidence`/`VerificationEvidence`
paths are relative to `WORKING_ROOT` = `projects/chirality-piping`.

## 1. Histograms (recount from the CSV; must reproduce exactly)

Disposition histogram (21 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 15 |
| STALE_SETUP_SPECIFICATION | 4 |
| PARTIALLY_IMPLEMENTED | 2 |
| **Total** | **21** |

ClaimType histogram (21 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 13 |
| DECLARED_STATE | 6 |
| EXCLUSION | 2 |
| **Total** | **21** |

ACCEPTANCE = 0, REMAINING_WORK = 0, IMPLEMENTED_UNMAPPED = 0 (rationale in §4).

## 2. Self-flagged rows

- **DEL-07-04-REQ-002** — ALIGNED at the warning/marking grain the requirement's
  own verification approach specifies ("UX tests show solve action/result state
  is blocked or marked incomplete"). The brief's solve-gating caution applies:
  end-to-end solve-*action* invalidation is cross-surface (solve-runner /
  application-service) and is not validated in this deliverable's evidence.
  Verification ≠ validation — `ValidationEvidence=NONE_FOUND`, Confidence MEDIUM.
- **DEL-07-04-REQ-003** — same display-vs-enforcement grain for RULE_CHECK
  blocking/qualifying; rule-check-action gating proper is cross-surface.
- **DEL-07-04-REQ-008** — chose PARTIALLY_IMPLEMENTED. Grain used: *this
  deliverable's warning/blocking panel*, where NONLINEAR_WARNING is inventory /
  status-map only with no active producer. Active surfacing of solver nonlinear
  uncertainty exists upstream (`core/solver/diagnostics/src/lib.rs`,
  `core/reporting/result_export/src/lib.rs`). An auditor preferring a
  contract-grain read could call this ALIGNED (class preserved + produced
  system-wide). (Calibration item 6 precedent: grain stated, row flagged.)
- **DEL-07-04-REQ-009** — chose PARTIALLY_IMPLEMENTED, same panel grain.
  IP_BOUNDARY_WARNING is inventory-only in this panel; production +
  quarantine/routing is realized in export/contribution/redaction surfaces
  (`core/security/redaction/controls.py`, `core/handoff/*`, redaction-controls
  export) — largely PKG-08 orbit.
- **DEL-07-04-REQ-011** — ALIGNED at the app-service-routing + update-after-edit
  grain; undo/redo diagnostic preservation ("where applicable") is not exercised
  by this display-only panel (AB-00-05 / DEL-00-05 orbit).
- **DEL-07-04-DECL-003 (Guidance.md)** and **DEL-07-04-DECL-004 (Procedure.md)**
  — STALE_SETUP_SPECIFICATION is a softer call here than for Specification /
  Datasheet: these advisory docs do not assert a false present-tense
  "no implementation exists" fact; their *substance* is honored by the
  implemented slice, and only the pervasive setup-era future-tense framing is
  stale. Encoded STALE for consistency with widened addendum 4 (which expressly
  "covers setup-era future-tense prose") and the W1 item-1 precedent; flagged
  for reviewer eyes because ALIGNED-with-note was the defensible alternative.
- **DEL-07-04-DECL-006 (MEMORY.md)** — MEMORY carries no undated current-state
  header block; every entry is dated (historical, addendum 1 / W2 calibration
  item 9). Included as a DECL surface because the dated log collectively
  declares the implemented state; dispositioned ALIGNED-with-note. The
  2026-06-04 dated entry cites SOFTWARE_DECOMP revision 0.7 / DAG-006 while the
  frozen decomp header is revision 0.8 `current_basis` — recorded as an in-row
  historical note, never a staleness disposition (addendum 1).

## 3. Evidence-execution log

**Re-executed side-effect-free (addendum 9):**
`tests/test_missing_data_warning_ux.py`, run from the frozen worktree with
`PYTHONDONTWRITEBYTECODE=1`, `PYTHONPYCACHEPREFIX=<scratch>/pycache` (outside the
frozen tree), and pytest `-p no:cacheprovider`:
- direct script invocation `python3 tests/test_missing_data_warning_ux.py` → exit 0;
- `python3 -m pytest -p no:cacheprovider -q …` → **1 passed**.
`git -C <frozen> status --porcelain` was **empty before and after** both runs.
This live-verifies the Python contract engine (`core/gui/warnings/engine.py`):
class/blocking preservation, `auto_fill_missing_data=False`, the analysis-status
map incl. `USER_RULE_FAILED`, and the negative professional-acceptance check.

**Cited as recorded (not re-executed at frozen SHA 551f84ef6):** the desktop
GUI suites — `apps/desktop/src/App.test.tsx` missing-data assertions (class
coverage asserts all six classes incl. `IP_BOUNDARY_WARNING:available_no_active_preview_item`
and `NONLINEAR_WARNING`, status separation, assistive fields, boundary,
export-review survival), full desktop **Vitest 399/399** (MEMORY 2026-06-17 /
2026-06-18), **Playwright 10/10** (TP-C4-CHECKGUI-001) and **18/18**
(TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001), and `apps/desktop/SMOKE.md`
TP-MAC-220 / TP-MAC-264. Per the GUI fence I did **not** run node/vitest/
playwright (they cannot guarantee zero frozen-tree writes).

## 4. Convention friction notes

- **Setup-boundary vs implemented slice.** The four-document kit disclaims
  implementation ("future GUI"; "does not implement GUI components, … tests");
  a bounded slice is nonetheless present in the frozen tree
  (`apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx`,
  `core/gui/warnings/engine.py`, `tests/test_missing_data_warning_ux.py`).
  Requirement rows take substance dispositions (never STALE, convention 1); the
  four kit DECL surfaces take STALE_SETUP_SPECIFICATION under widened addendum 4.
  AuthorityNeeded=NO — pure setup-era drift, no overtaken TBD registers (the
  kit Conflict Tables record conflict = "None"; the `_CONTEXT.md`/Datasheet
  "Still TBD" list is standing architecture-level TBDs, not overtaken).
- **Rev-0.7 authority-pointer drift is off-census here.** `_CONTEXT.md` and
  `_REFERENCES.md` cite SOFTWARE_DECOMP revision 0.7 (frozen header is 0.8), but
  neither is an addendum-1 DECL census surface (only the four-doc kit +
  `_STATUS.md` + `MEMORY.md` + in-tree READMEs are). No kit surface carries a
  revision pointer, so the W1 item-1 STALE encoding attaches to no DECL row.
  The only census surface touching rev 0.7 is `MEMORY.md`, in a **dated**
  entry — handled as an in-row historical note on DECL-006 (see §2).
- **No ACCEPTANCE rows.** The `Specification.md#Verification` bullets and
  `Procedure.md#Verification` list restate the per-requirement verification
  approaches; they introduce no distinct acceptance criteria at addendum-12
  grain, so no mirrored ACCEPTANCE rows are emitted. There is no separate
  acceptance-criteria artifact in the kit.
- **No REMAINING_WORK rows.** `_STATUS.md ## Remaining` contains only the seeded
  `(gated: D-41)` bootstrap item, transcribed byte-exact into the DECL-005
  `RecordedRemaining` and excluded from all residual/gate/selectability analysis
  (addendum 2). Its gate/source cells use the **exclusion variant**
  (`NONE_RECORDED`, not annotated with `D-41`) per W2 calibration item 11.
  Deferred scope named in the 2026-05-11 MEMORY entry (live solver/rule
  execution, full GUI packaging, final styling) is a dated historical record,
  not a current `_STATUS.md` residual, so it is not a REMAINING_WORK row.
- **No IMPLEMENTED_UNMAPPED rows.** The material surfaces in this deliverable's
  orbit (the missing-data panel and the warnings engine) are explicitly
  DEL-07-04-stamped (`deliverable_id: "DEL-07-04"`), i.e. mapped, not unmapped.
  `core/gui/pkg02_boundary.py` is shared PKG-02-boundary code mapped to the
  DEL-02-03 analysis-boundary contract, outside this deliverable's ledger grain.
- **Review findings are RESOLVED, not pending.** `Review_Findings.csv`
  (PKG07-DEL0704-PKG02-001/002) shows `HumanDisposition=ACCEPT_AS_IS`,
  `Status=RESOLVED`. The `TECHNICALLY_ADDRESSED_PENDING_HUMAN` wording survives
  only in dated MEMORY log entries (historical), so addendum 13 (pending
  disposition → Confidence cap MEDIUM + OWNER) does not apply and no row routes
  OWNER on that basis.
- **SourceReliability keyed to the weakest load-bearing leg (W2 item 13).**
  Every requirement/exclusion row is `UNVERIFIED`: the load-bearing verification
  legs (Vitest/Playwright/pytest) are project-original agent-generated evidence
  with agent audit but no human validation ruling covering the requirement
  behavior. The DEV-001 Stage-2 human disposition covers only the PKG-02
  boundary-mapping concern, not the full verification, so it does not lift rows
  to `REVIEWED`. DECL prose rows are `NOT_APPLICABLE` (addendum 6).
- **AuthorityNeeded as adjudication router, not work queue (W2 item 14).** No
  numeric threshold or authority claim is being promoted, so every row is `NO`;
  the two PARTIALLY rows (R-008/R-009) describe cross-surface realization, not a
  decision owed here.

## 5. Boundary-compliance statement

All fences held. Discovery was read-only outside the two W3 output files
(`CLAIM_CONCORDANCE_DEL-07-04.csv`, `NOTES_DEL-07-04.md`). No `_STATUS.md`,
register, DAG, or product file was edited; no lifecycle transition was applied
(`LIFECYCLE_REASSESSMENT_REQUIRED` was neither warranted nor recorded); no
cross-project edits. No release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim appears anywhere in these
outputs (F-PIP-1..5) — findings that the software *avoids* such claims (R-010,
EXC rows) are statements about the audited artifact, not pilot claims of
compliance. Dispositions are agent judgments routed via `AuthorityNeeded`, never
represented as owner or engineering rulings. The frozen worktree porcelain was
empty before and after every read and re-execution
(`551f84ef6be656f1603ce0acfa5e3935aa9683c7`, verified clean at close).
