# Report package for current results (W-18)

**Basis:** `origin/main` at `8b4f584e7a` (2026-10-24), after W-3 (#1553, the
local-private rendered report). Revision 1, design before implementation.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **FR-REP-001 (Must):** "Report includes inputs, versions, states, runs,
  assumptions, warnings, results, and rule-pack references." PRD §19.2 lists the
  content; §22.6 names the full report package.
- **FR-REP-004 (Should):** comparison summaries in reports. The package carries
  the within-run comparison sections today (DEL-08-06).
- **DEL-12-02 REXC-REQ-006:** local-private exports may retain private values
  only with explicit local-private intent and a warning or audit note.
- **DEC-028:** the package is a multi-member archive; one package version, older
  versions refused by name with no migration (owner, 2026-10-10: "Close: one
  version").
- **Ruling 10A (owner, 2026-10-21):** exact-route results stay refused for
  reports and the report package until DEL-08-01 designs their limitation
  sections. Opening them returns to the owner (W-11). This design does not open
  them.

## 1. Where things stand

- **The gate.** `reportPackageUnavailableReason` refuses the pressure-free route
  (`preview_physics`) and every fresh identity with
  `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` (`reportPackageRequest.ts:27-32`).
  The refusal is T0R's, "until T6" (`knownSemanticLimitations.ts:27,36`). No
  later unit delivered the package for a current result, so Save Report
  Package is disabled for every result the desktop produces (`App.tsx:215,1194`).
- **Why it was closed.** Below the gate, assembly transcodes the result into the
  DEL-08-04 1.0.0 result envelope, whose metadata vocabulary predates the
  current semantics (`reportPackageRequest.ts:65-78`). Current force, moment and
  stress rows are not representable there: they are dropped into
  `REPORT_SOURCE_EVIDENCE_DISCLOSED` diagnostics (`:146-152,359`). The Rust
  boundary refuses any value carrying current metadata, from content
  (`core/reporting/report_package/src/wire.rs:656-718`).
- **The current result document already exists.** The Result Export panel's
  `buildCurrentResultExport` (`result-export/resultExportAdapter.ts:233-276`)
  produces the DEL-08-04 0.3.0 document for a Current result. It checks the
  output policy, the reader's numerical standing, the live native invocation,
  the verified input manifest, the 0.3.0 analysis record and every hash binding.
- **Rule checks.** The package refuses after any rule check
  (`REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE`, `reportPackageRequest.ts:266-269`;
  `wire.rs:719-724`), and its audit manifest and result envelope hard-code
  `rule_pack_refs: []` (`:374,405`). W-3's rendered report already carries the
  rule-pack identity, name, version, checksum and the user-rule outcomes
  (DEL-08-01 R-08-01-008 safe metadata).
- **Data boundary.** The save is already local-private: route
  `DREP-PACKAGE-SAVE-009`, its own intent control (`reportPackagePrivateIntent`,
  `workspaceSession.ts:2549-2610`), and Rust's re-check of the redaction and
  local-first evidence (`src-tauri/src/lib.rs:64-86,207-224`). The rendered
  members are built under `local_private` (`reportPackageRequest.ts:277-284`).

## 2. Design

### 2.1 Availability

The package is available for a pressure-free result exactly when the Result
Export panel would export it:
- **Admission is the result-export builder's.** The package builds its result
  member by calling `buildCurrentResultExport` for the same session inputs. Its
  checks are the package's admission; there is no second copy of them. A
  refusal is shown with the builder's code.
- **The route gate is the output policy's.** `reportPackageUnavailableReason`
  becomes the policy's `report-package` surface decision
  (`outputPolicy.ts`, `refuseSurfaceRoute`):
  - `preview_physics` (pressure-free) is ungated: the builder's checks apply.
  - `retained_preview_physics` and `retained_physics` keep their entries'
    `refused`, with their reasons.
  - `pressure` and `pressure_source` (exact) keep `refused`, with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` (ruling 10A).
  - `unsupported` refuses through the builder's contract check.
- **Retired.** `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` and `N_REPORT` are
  retired in TypeScript and Rust, with their tests, the user guide entry and the
  DEL-04-07/DEL-04-08/DEL-08-06 text that cites them.
- **Menu.** Save Report Package… is enabled when the builder's precheck passes
  (`App.tsx:215`); the status line shows the policy or builder reason
  otherwise.

### 2.2 Members (package version 2.0.0)

| Role | Member | Source |
|---|---|---|
| `package_manifest` | `package_manifest.json` | Unchanged form, `schema_version` `2.0.0` |
| `canonical_rendered_report_html`, `canonical_rendered_report_pdf` | as today | `buildRenderableReportInput` under `local_private`, `generatedAt: null` (unchanged). The W-3 content and the local-private notice are in both |
| `audit_manifest` | as today | DEL-08-02; `rule_pack_refs` take the session's rule-pack references (the W-3 safe metadata), not `[]` |
| `result_export` | `result_export_<envelope id>.json` | **The current DEL-08-04 0.3.0 document** from `buildCurrentResultExport`, canonically equal to what Export Result JSON saves for the same session. It replaces the 1.0.0 transcoding |
| `state_comparison_handoff_sections` | as today | `buildStateComparisonHandoffSections`, unchanged. W-12 later supplies real state and run records |

- **Version.** The `result_export` member's form changes, so
  `PACKAGE_MANIFEST_SCHEMA_VERSION` moves from `1.0.0` to `2.0.0` and the
  reader refuses `1.0.0` by name (`OlderVersion`), as DEC-028 rules. No current
  result has ever produced a 1.0.0 package (§1).
- **Retired from the package.** The 1.0.0 transcoding in the package path
  (`projectReceivedReportResults`'s package use, `reportMetadata`,
  `reportSolverIdentity`, the result-envelope DTO in `wire.rs`). Whatever else
  still consumes them stays; anything left with no consumer is deleted.

### 2.3 The Rust boundary

The bridge re-checks the request from content, because the wire carries no
semantic identity:
- the `result_export` member is a DEL-08-04 `0.3.0` document whose
  `result_envelope` names the run in `source_basis_refs` and the model hash in
  the audit manifest;
- its semantic identity is the pressure-free identity, `preview-physics-2`.
  Every other identity is refused (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`). This
  replaces the fresh-metadata refusal and keeps the exact routes closed at both
  boundaries;
- the audit manifest's `rule_pack_refs` equal the rendered report input's;
- the existing evidence checks (`lib.rs:64-86,207-224`) and the producer's
  `export_blocked` are unchanged.

### 2.4 Rule checks

- The package is available after a rule check. The rendered members carry the
  W-3 rule-pack section and user-rule outcomes; the audit manifest carries the
  same references.
- `rule_check_aggregate` and `solve_rule_check_status` are carried as today.
  Rust refuses a request whose aggregate is present while the report input has
  no rule-pack reference, and the reverse.
- Only safe metadata is carried (R-08-01-008, REXC-REQ-009): no formulas,
  limits or rule values.

### 2.5 Data boundary

Unchanged: `local_private`, `DREP-PACKAGE-SAVE-009`, the package's own explicit
intent control, the lossless binding and Rust's evidence re-check. The members
carry private project data. The rendered members carry the local-private notice.
The disposition screener blocks `protected_suspected`, `quarantined` and
`rejected` records in every context.

### 2.6 Exact routes (W-11)

Nothing opens here. When the owner opens exact-route reports (W-11), the
exact-route result document already exists (the result-export surface admits
it, ruling 10A). The package then needs only the policy entry, the Rust identity
list and W-11's rendered limitation sections.

## 3. A shareable report path: the DEL-12-02 check

**What the product does today.**
- Every session value is classified `private_project_data` / `private_only`
  (`renderableReportInput.ts:119-130`).
- In a public or shared context, private values get `redact_value`
  (`redactionExportControls.ts:466-473`).
- The render route requires lossless materialization, so any redaction blocks
  it (`redactionExportControls.ts:962-967`). That is why the public render is
  `blocked` with no blocking finding (#1553's native probe).

**What DEL-12-02 allows.**
- Shared and public exports redact or block private values (REXC-REQ-002;
  ScopeOfWork :73). "Silent unredacted export to a shareable/public location" is
  the named anti-pattern (:470).
- Rule-pack detail in shared reports is limited to safe metadata (REXC-REQ-009).
- Neither DEL-12-02 nor PRD §20 provides a way for a user to release their own
  project data for sharing.

**Finding.** A shareable report needs a data-boundary decision:
- **(A) Redacted shareable report.** Designable within DEL-12-02 as written: a
  non-lossless route and a renderer that shows redaction markers. Because every
  session value is private, it would carry no project values and no results:
  the notice, the structure, the software identity and the rule-pack safe
  metadata only. M.
- **(B) User release of project data.** The user declares the session's project
  data released for sharing (a new user-asserted classification, recorded in the
  document). Project values and results are then included. Private rule-pack and
  library values stay redacted to safe metadata, the protected-content linter
  and the disposition blocks still apply, and the notice says the user released
  the data. This adds a classification to the DEL-12-02 vocabulary and changes
  the PRD §20.3 default. M.
- **(C) No shareable path in v0.1.** The local-private report and package are
  the product's reports. Sharing one is the user's own act outside the product
  (PRD §20.4), and the document's notice ("not for sharing or publication")
  stays. No work.

This is reserved for the owner. It does not block §2.

## 4. Verification

- **Rust (`report_package`, the bridge):**
  - a 2.0.0 package from a current pressure-free request assembles, reads back
    and verifies its member hashes;
  - a 1.0.0 container is refused by name;
  - an exact-route, retained or unsupported identity in the result member is
    refused;
  - the run, model-hash and rule-pack binding refusals.
- **vitest:**
  - the gate follows the output policy per route;
  - the result member equals `buildCurrentResultExport`'s document;
  - the package is available after a rule check, with the references in the
    audit manifest and the report input;
  - each builder refusal is surfaced.
- **pytest:** the headless determinism test (`tests/test_headless_runner_contract.py`)
  and the package reader tests, updated for 2.0.0.
- **Native smoke:** Save Report Package… on the solved demo, then the reader's
  verification of the saved container, with assertions that need no new pinned
  `expected.json` values.
- **Review:** an independent Codex round on the implementation (persistence,
  file output and data boundary).

## 5. Reconciliation

- **DEL-08-01:** the "Current implementation" paragraph (:81) and this design;
  `deliverable.yaml` pointers.
- **DEL-08-06:** "Desktop report-package request" (:118).
- **DEL-08-04, DEL-08-02:** pointers only.
- **DEL-04-07 Design §4.5, DEL-04-08 ScopeOfWork :219:** the report package's
  refusal text. DEL-04-08 :219 is commitment text, so its exact old → new goes
  through the wording route.
- **DEL-12-02:** the §3 finding, as an open item, after the owner's choice.
- **The completeness map:** FR-REP-001 and FR-REP-004, gap 1, the W-18 row.
- **The user guide** entry for the package refusal.
