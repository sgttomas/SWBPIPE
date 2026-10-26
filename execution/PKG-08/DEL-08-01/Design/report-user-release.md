# Per-report user release of the user's own project data (W-18b)

**Basis:** `origin/main` at `c7f52fd12a` (2026-10-25), merged into this branch
at `ad79d9d72f`. Every file:line citation is at that tree. At revision 3,
`main` at `3027cf2f35` (W-15 slice b, #1608) was merged at `b2bc7e8a27`. Of
the files cited here it changes only `docs/COMPLETENESS_MAP.md`, whose W-18
row moved from :546 to :547 (§9), and `tools/hosted-ci-routing.json`, cited
without a line; every other citation holds at both trees. Revision 2 reduced
the design to the owner's scope R (§0), after Codex round 1 (three blocking,
four should-fix) and round 2 (one blocking, three should-fix). Revision 3
answers round 3 (one blocking, three should-fix, one non-blocking, and A7's
wording). §7 says where each finding is resolved. Design before
implementation; nothing here is built. This design records the mechanism by which a user can state,
for one report, that they release their own project data for sharing. It
records no release, and it records, implies or derives no acceptance,
issuance, approval, compliance or lifecycle standing (F-PIP-3,
`loop/LOOP_INIT.md:68-72`).
Standard claim fence applies (claims taxonomy per DEC-081).

## 0. Rulings

- **Owner, 2026-10-24:** "I accept your recommendations for each of these
  three: G10, W-12, W-18." For W-18 this is option B (W-18a design §3, §7
  O-1): the user explicitly releases their own project data, in v0.1. As
  recorded, the shape is:
  - an explicit opt-in recorded in the document, on the basis of
    `docs/IP_AND_DATA_BOUNDARY.md` §6 (:99);
  - private rule-pack and library values stay at safe metadata;
  - the protected-content linter and the disposition blocks still apply;
  - the notice states the release;
  - the private default is unchanged;
  - a DEL-12-02 runtime vocabulary addition;
  - commitment text goes through the wording grant with Codex;
  - targeted regression checks: the flag cannot leak private values; redaction
    applies to every other context; the release is recorded and visible.
- **HELP_HUMAN, 2026-10-24, option (1), per report:**
  - recorded in that report's `report_context` plus the package manifest record;
  - bound to the model hash or result-set identity the report covers;
  - an unmistakable notice;
  - without the selection, the report stays local-private exactly as today;
  - tests prove no replay onto a different model or result set;
  - the DEL-12-02 classification and any commitment text go through the wording
    grant.
- **The selection does not persist.** It is not stored in the session, the
  project or the model document, and the model document does not change
  (W-18a design §7 O-1).
- **HELP_HUMAN, 2026-10-25:** "Release itself stays the owner's act, so W-18b
  designs only the mechanism for recording a per-report release (as decided
  earlier) and records no release." Software release and issuance are owner
  acts. This mechanism records a *user's* statement of their own
  redistribution right for one report.
- **Ruling 10A (owner, 2026-10-21):** exact-route results stay refused for
  reports and the report package. W-11 opens them, separately. The user release
  admits only what the local-private package admits (§3.4).
- **DEC-028 (owner, 2026-10-10):** "Close: one version". The package has one
  version, 2.0.0 today (`report_package/src/lib.rs:75`); the reader refuses
  every other version by name (`reader.rs:269-293`).
- **Owner, 2026-10-25 (W-18b decisions):** "I approve your recommended options
  for the three W-18b decisions." As relayed by HELP_HUMAN, the approved
  recommendations are:
  - **Scope R first.** One result member. No rule outcomes, comparisons or
    handoff records. Library-referenced models are refused. A minimal native
    renderer input. Solve attestation, and no rule-check attestation. The rest
    (rule outcomes at safe metadata, library-referenced models, comparisons)
    stays in v0.1 as a later slice on the same mechanism; it is not dropped.
  - **O-A = V1.** The package stays at 2.0.0 with the optional record.
    DEC-028's "one version" is read as admitting an optional writer-defined
    record that the reader at the same head accepts.
  - **O-G: equality-only guarantee,** stated plainly in the reader text and in
    DEL-02-05 (A8).

  O-E is moot under R. O-B, O-C, O-D and O-F stay as recommended (HELP_HUMAN,
  2026-10-25).
- **HELP_HUMAN, 2026-10-25: V1 covers R's package form.** The ruling adopted the
  recommended V1 in full, including its second clause: "the reader at each
  head accepts what the writer at that head emits". A user-released package
  with no state/comparison/handoff member is emitted by this head's writer and
  accepted by its reader, so no handoff set is kept (§2.1, §4).
- **HELP_HUMAN, 2026-10-25: rule-pack-referenced models are refused under R**
  (`REPORT-USER-RELEASE-RULE-PACK-REFERENCED`, §2.7). This is a stated
  consequence of the owner's "no rule outcomes" together with W-18a's rule
  that a package names every active rule pack of the solve. It is sequencing
  within the ruled scope, not a narrowing of v0.1: the later slice, kept in
  v0.1 (§10), lifts it together with the library refusal. The dialog says
  plainly which models cannot be shared yet, and why (§2.2).

§2.1 states scope R as an exact member contract. §10 lists what the later
slice adds on the same mechanism.

**Naming rule.** The word "release" never appears alone in a product text,
identifier or record. It is always "user release", "user-released" or "release
your project data". No text uses "issued", "approved", "accepted",
"certified", "released version" or "release of SWBPIPE". The record's
`standing` member is the constant `user_statement_of_data_rights_only` (§2.3).

**What "the user's own project data" means here.** It is the content of the
model this application solved in the current session, the result its own
solver produced for that model, and the run record of that solve, as native
admission binds and derives them (§2.5, §2.6). Under R the model carries no
detectable library or rule-pack reference (§2.7).

**The reliance limit.** The detector finds the marks that library applies
have written into a model (§2.7). It cannot find a library origin that a user
has erased, and it does not show that any value is the user's own. For every
value without a detectable library origin, the release relies on the user's
own statement of their rights: the checkbox statement the record carries
(§2.2, §2.3). The native host does not verify that statement, and the record
does not prove it (§2.9).

## 1. Where things stand

- **Every session value is private.** The report builder classifies the
  session, every component and every spring hanger as `private_project_data`
  with `redistribution_status` `private_only` (`renderableReportInput.ts:121-132,178-194,258-273`).
- **Two report surfaces, two contexts.**
  - The rendered-report panel renders under `public_report` (blocked for every
    session) or, with its own checkbox, `local_private`
    (`RenderedReportPanel.tsx:79-80,237-245`). Its HTML save is a webview
    `data:` download (`:359-375`) and its PDF is the browser print of the
    derived view (`:376-383,399-408`), which the document itself says is not
    hash-bound (`report_renderer/src/lib.rs:1299`). The native render command
    is a pure renderer: it receives no model, no route evidence and no
    screening (`src-tauri/src/lib.rs:1540-1566`).
  - Save Report Package… renders under `local_private` only, with
    `generatedAt: null` (`reportPackageRequest.ts:197-205`), through route
    `DREP-PACKAGE-SAVE-009` (`reportRedactionProjector.ts:58-68`). The native
    admission re-checks the evidence, binds the request, model and input-manifest
    texts by SHA-256, screens dispositions in the request and the model, and binds
    the context (`report_package_save.rs:591-662`): "caller evidence alone
    admits nothing" (DEL-12-02 `ScopeOfWork.md:486`, REXC-OI-007).
- **Context vocabulary.** Five contexts: `public_report`, `public_example`,
  `shared_model`, `downstream_tool`, `local_private`
  (`redactionExportControls.ts:17-22,127-133`; `controls.py:18-19`; schema
  `redaction_export_controls.schema.yaml:402-411`; DEL-12-02
  `ScopeOfWork.md:58`). The public and shared contexts redact private values
  (`redactionExportControls.ts:465-477`; `controls.py:419-426`), so none of them
  carries a project value unredacted.
- **The decision order today.** `actionFor` (`redactionExportControls.ts:417-479`)
  and `_action_for` (`controls.py:372-426`) decide, in order: the claim,
  network, SQL, storage-bypass, secret, payload and path checks; the
  `protected_suspected` block (`:439-441`; `controls.py:402-403`); the
  quarantined and rejected block (`:442-444`; `controls.py:404-405`); then
  **unknown metadata** (missing metadata, unknown redistribution, unknown
  privacy, `:445-453`; `controls.py:406-411`); and only then the safe and private
  branches. An object whose privacy is `private_rule_pack_data` and whose
  redistribution is `unknown` is therefore decided by the unknown branch, never
  by the private one (Codex round 1, reproduced in memory).
- **The route projector labels by path, not by source.** `projectRouteValue`
  (`redactionExportControls.ts:844-925`) labels a leaf private when its path
  contains a private token and `unknown` otherwise. It strips the payload's
  intent keys globally first (`stripRouteIntent`, `:708-712,825-842`).
- **Library values reach result rows through genuine solver output** (Codex
  round 2, blocking). A library-applied component's flexibility factor and SIF
  are model values that the solver consumes:
  - the curved-bend flexibility row carries the factor as its value
    (`core/product_physics/src/lib.rs:13341-13343`);
  - the user-multiplier row's basis carries the SIF, the flexibility and the
    source reference (`:13245`);
  - the intensified-bending rows carry the SIF and its source in their sign
    convention, contract evidence and diagnostics
    (`preview_physics.rs:687,696-698,700-705`).

  Codex cited these at `05ec850eca` as `lib.rs:13230` and
  `preview_physics.rs:694`. The solve's own attestation reproduces them, so no
  derivation check can tell a library value from a user-entered one once it is
  in the result.
- **How a library apply marks the model.** Every apply writes a
  `provenance_record.library_ref` (`libraryApply.ts:224`) and a library
  provenance string `library:<kind>:<library id>#<record id>`
  (`libraryProvenanceString`, `libraryApply.ts:81-83`). That string goes into
  component source-reference strings (`:701-708`) and a hanger's `provenance`
  text (`hangerSelection.ts:41-42,54-55`). The native save already walks every
  `library_ref` at any depth (`export_disposition_screen::library_ref_sites`,
  `export_disposition_screen.rs:263-319`), pinned to
  `fixtures/model_document/library_ref_sites.json` with the webview's
  `libraryRefSites` (`libraryImportService.ts:440`). The same walk visits every
  `rule_pack_refs[]` entry (`:231-296`).
- **How the hanger selection marked the model before W-17** (Codex round 3,
  blocking). From `46f3aed341` (2026-09-05) until `00af9112c0` (2026-10-23),
  the hanger selection wrote neither mark. It stored a support's `provenance`
  as the RFC 8785 JSON text of `{hanger_id, kind:
  "user_imported_hanger_selection", library_id, library_kind: "hanger",
  library_metadata, record_snapshot}` (`hangerSelection.ts:35-39,52` at
  `00af9112c0^`). A model saved in that period still carries it. Its hanger
  values reach genuine `spring_hanger_user_input_review` rows
  (`core/product_physics/src/lib.rs:13429,13530`), and nothing current
  detects their library origin. `core/library_import/README.md:45-51` still
  describes that form as current.
- **Rule packs in the package.** A package names the rule packs of its
  rule-check run, and admission requires them to equal the solve's
  `active_rule_packs` (`report_package_save.rs:572-588`;
  `reportPackageRequest.ts:210`). The session's analysis record composes the
  rule-check aggregate into its status (`previewService.ts:565-581`). Since W-17
  (#1629), outcomes are per check and load case or combination, with an
  optional `basis_ref` on `UserRuleCheckOutcome`
  (`report_renderer/src/lib.rs:129-162`; `renderableReportInput.ts:451-474`).
- **Solve custody.** The host never materializes the solved model: the worker
  process does (`src-tauri/src/solve_jobs.rs:1-41`; `solve_worker.rs:293-298`).
  The worker returns the producer's exact bytes, which are serde's
  serialization, not RFC 8785 (`lib.rs:1828-1836`). The host retains them until
  the webview takes them, and releases them on disposal
  (`solve_jobs.rs:668-800`); nothing native records afterwards which result
  was produced for which model. The webview's Current export requires a
  captured native invocation for the result (`hasNativeMechanicsInvocation`,
  `resultExportAdapter.ts:294`), which is a webview check.
- **Native pieces R can reuse unchanged.**
  - `canonical_json::checked_v1_domain_canonical` with `ByteSink` streams a
    domain-separated canonical form into a hasher without holding the text
    (`core/serialization/canonical_json/src/lib.rs:124-187`).
  - `result_export::derivative::derive_document` (`derivative.rs:120`) derives
    a 0.3.0 document from a base, the model, the source and an origin.
  - Record Run's binding checks of a transient analysis record against its
    result and input manifest (`analysis_runs::checked_submission`,
    `analysis_runs.rs:634-782`).
  - The producer's report-row projection, already derived natively
    (`report_package::bindings::expected_report_result_rows`, item 6,
    `bindings.rs:448-452`; vectors `fixtures/reports/invented/report_result_rows_parity.json`).
- **The package and its reader.**
  - The manifest's fields are fixed by the version except `package_id`,
    `export_profile_id`, the source references and the diagnostics
    (`lib.rs:643-734`).
  - Its third boundary note reads "No timestamps or generated identifiers are
    recorded anywhere in this package; identical inputs produce identical
    bytes." (`lib.rs:722-723`).
  - The reader rebuilds the manifest from those fields and refuses any
    difference (`reader.rs:221-240`).
  - The writer blocks a package without a state/comparison/handoff member
    (`lib.rs:255-259`), the producer requires exactly one handoff record set
    (`bindings.rs:320`), and the reader refuses an unblocked package without
    one (`reader.rs:960-967`).
  - The reader's role forms fix member order, not member count
    (`reader.rs:766-780`), so it admits more than one result member.
- **What is missing.** A runtime way for the user to record, for one report, the
  basis `docs/IP_AND_DATA_BOUNDARY.md:99` names: "unless a user intentionally
  exports or contributes it with documented redistribution rights".

## 2. Design

### 2.1 Scope R: the reduced member contract

A user-released package is the 2.0.0 container with the optional record (§4).
Its members, in archive order after the manifest, are exactly these:

| Member | Count | What it may contain | What is absent |
|---|---|---|---|
| `calculation_report.html`, `calculation_report.pdf` | one each | The rendering of the minimal native renderer input (§2.6): the title; the model summary (project name and id, units, model hash, input-manifest reference, display policy); the load cases (id, label, kind); the result rows (the five-field projection of every `results` row of the solve, its review rows included, such as a spring hanger's `spring_hanger_user_input_review` rows); the solve's diagnostics; the analysis-status disclosures; the fixed technical-preview limitation; the run context (software, solver identity, mode and settings, run name); the record and the notice | Rule-pack references; user-rule outcomes; component and spring-hanger provenance entries; dedicated per-object hanger quantity entries; numerical-health rows; the local persistence reference (always `TBD`); a generation time and a run recording time (not recorded) |
| `audit_manifest.json` | one | Its id (the input-manifest reference); the model and input-manifest hash records; the solver identity; the unit-system reference; `rule_pack_refs: []`; `assets: []`; the boundary constants | Any rule-pack reference; any asset |
| `result_export_<envelope>.json` | exactly one | The DEL-08-04 0.3.0 document that native admission derives from the bound model, the attested solve, the run record and the input manifest (§2.6) | A second result member |
| State/comparison/handoff record set | none | — | The member itself: no state/run, comparison or handoff section |
| `package_manifest.json` | one | The fixed fields; `package_id` `desktop-user-released-report-<run>`; the profile `desktop_user_released_report_package_1`; the source references; the producer's diagnostics; the record with its `content_binding`; the user-released boundary notes (§5.3) | — |

The request on route 010 also carries `rule_check_aggregate: null`, no
`user_rule_checks`, `solve_rule_check_status` equal to the solve's own
`status.rule_check`, and `state_comparison_handoff_records: []`.

The bound model, the solve's result, the input manifest and the run record are
validation inputs only. None is written to the package, and neither is any
rule-pack payload or library record (W-18a design §2.2, "Not a member").

**What R refuses.** The save is refused when:
- the model carries a detectable library reference, in any form a library
  apply has written, or a rule-pack reference (§2.7);
- any object in the model or the request carries a non-project private class
  (§2.7);
- the result was not produced by this session's solver for that model, or its
  attestation has been evicted (§2.4);
- any request member differs from its native derivation (§2.6);
- or any existing refusal applies: an exact route, a blocked disposition, the
  protected-content linter (§3.4).

**The W-17 constraint.** R carries no rule-check outcome, in any shape. The run
record it admits carries the solve's own rule status, not a rule-check
aggregate (§2.5 step 8). The per-(check, basis) outcome shape of #1629 is the
shape the later slice must carry (§10).

### 2.2 Where the selection is made, and what the user does

**Recommendation: the report package only.** The user release is offered on
Save Report Package…'s route, and the package's HTML and PDF members carry it.
The standalone rendered report (panel HTML save and Print / PDF) is not offered
a user release in W-18b (O-C, as recommended). Reasons, from current code:
- the standalone HTML save and the print view are webview sinks with no native
  admission (`RenderedReportPanel.tsx:359-383`), and the render command
  receives no model or evidence (`src-tauri/src/lib.rs:1553-1566`), so a
  standalone release would rest on caller evidence alone, which REXC-OI-007
  forbids for this class of route;
- the PDF is a derived, non-hash-bound view (`report_renderer/src/lib.rs:1299`),
  so a record in it could not be bound to the bytes the user shares;
- HELP_HUMAN's ruling names the package manifest record, which only the
  package has.

A user who wants to share the document shares the `.opsproj` or its HTML or PDF
member; each member carries the notice and the bound identities (§5.1).

**The controls.**
- In the rendered-report panel's package controls
  (`RenderedReportPanel.tsx:252-274`), a second button after
  "Save Report Package…": **"Save Shareable Report Package…"**
  (`data-testid="report-package-user-release"`), enabled under the same
  conditions as the local-private save (`:267`).
- In the File menu, after "Save Report Package…" (`App.tsx:1253`; native menu
  `src-tauri/src/lib.rs:4396`): **"Save Shareable Report Package…"**, command
  id `file.save-shareable-report-package`, disabled exactly when
  `file.save-report-package` is.
- Both open one dialog, `apps/desktop/src/features/report/UserReleaseDialog.tsx`.
  The local-private checkbox "Include known private values in this local
  package" (`RenderedReportPanel.tsx:254-263`) is not read by this route and
  keeps its meaning.

**The dialog** (pattern: `HandoffSaveDialog.tsx`, `role="dialog"`,
`aria-modal="true"` at :81-82). It takes focus in `useLayoutEffect`
(`HandoffSaveDialog.tsx:44`), on the statement checkbox. Escape and Cancel
close it and discard the selection. Exact proposed wording:

> **Release your project data in one shareable report**
>
> This saves a report package you may share. It includes this project's
> values and results, unredacted.
>
> - It covers only this report: model "{project name}" (SHA-256
>   {first 12 hex}…), run {run id}.
> - It carries the model summary, the load cases and this solve's results. It
>   carries no rule-pack reference, user-rule outcome or run comparison.
> - It needs a result solved in this session. A model with a library or
>   rule-pack reference cannot be shared this way.
> - The application checks this model only for library and rule-pack marks.
>   It cannot tell where any other value came from, so your statement below
>   covers every value in this report.
> - Content marked protected, quarantined or rejected still stops the save.
> - Nothing is remembered: your next report is local private unless you
>   choose this again.
>
> ☐ I hold the right to share this project's data, and I release it in this
> report for sharing.
>
> [Save Shareable Report Package…] [Cancel]

When the webview's early check finds a library or rule-pack reference, in any
form §2.7 names,
the dialog shows, in place of the checkbox: "This model uses a library record
or a rule pack. Shareable packages do not yet carry library record values or
rule-pack checks, so a shareable package cannot be saved for this model. Its
local private package is unaffected." The save button stays disabled.

The save button is disabled until the checkbox is ticked. The checkbox starts
unticked every time the dialog opens.

**What the user does:** an explicit selection (opening the dialog from its own
control) plus a statement of the redistribution right (ticking the statement),
then the save. Confirming starts the save at once; the confirmation is one
argument of that one save (§2.3, "Session").

**Exact routes stay refused.** The dialog's controls are disabled whenever
`reportPackageUnavailableReason` refuses (`reportPackageRequest.ts:60-62`), and
the save goes through the same output-policy cell and producer admission as the
local-private package (§3.4). Ruling 10A stands; W-11 is separate.

### 2.3 The record

**Form.** One JSON object, key `user_release`. Every member is required; no
other member is admitted (`deny_unknown_fields` in Rust, an exact key check in
TypeScript and at the reader). Hex values are 64 lowercase hexadecimal
characters.

| Member | Type | Value |
|---|---|---|
| `record_kind` | string, constant | `user_release_of_own_project_data` |
| `record_version` | string, constant | `1.0.0` |
| `statement` | string, constant | `I hold the right to share this project's data, and I release it in this report for sharing.` (the dialog's checkbox text, verbatim) |
| `statement_basis` | string, constant | `docs/IP_AND_DATA_BOUNDARY.md §6: a user intentionally exports their own data with documented redistribution rights` |
| `standing` | string, constant | `user_statement_of_data_rights_only` |
| `scope.scope_kind` | string, constant | `this_report_only` |
| `scope.report_id` | string | `report:<analysis_run_ref>`, which is the report's `calculation_report.report_id` (`renderableReportInput.ts:597`) |
| `scope.package_id` | string | the request's `package_id`, `desktop-user-released-report-<run>` |
| `scope.released_data_class` | string, constant | `private_project_data` |
| `scope.not_carried` | array of strings, constant | `["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"]` |
| `bound_to.model_ref` | string | the model's `project.id` |
| `bound_to.model_document_sha256` | hex | SHA-256 of the RFC 8785 bytes of the session model document |
| `bound_to.analysis_run_ref` | string | the run id the report covers (`result.run_id`) |
| `bound_to.input_manifest_ref` | string | the run's input-manifest reference, `input-manifest:<model id>:<sha256>` |
| `bound_to.input_manifest_sha256` | hex | SHA-256 of the RFC 8785 bytes of the run's input manifest |
| `bound_to.result_envelope_id` | string | the result document's `result_envelope.envelope_id` |
| `bound_to.result_document_derivative_sha256` | hex | the result document's `result_envelope.reproducibility.derivative_hash.value` |
| `released_at` | string | the application clock when the user confirmed, UTC, `YYYY-MM-DDTHH:MM:SSZ` (O-B, §7) |
| `content_binding` | object, **manifest copy only** | written by the package writer, never by the caller (below) |

The free-text members and `scope.not_carried` are constants, so a record cannot
carry any other wording (for example a claim) in their place. The later slice
that carries rule outcomes changes `scope.not_carried`, and with it
`record_version` (§10). `released_at` is the one caller-supplied value that
native admission cannot derive: it checks its form only, and the record states
it as the application clock's reading.

**Content binding (manifest copy only).** The writer adds `content_binding` to
the manifest's copy of the record after the members are materialized:
`{ "algorithm": "sha256", "members": [{ "file_name", "role", "sha256" }, …] }`,
one entry per non-manifest member (HTML, PDF, audit manifest, result member) in
archive order, each the SHA-256 of that member's exact bytes. The renderer never
sees `content_binding`: the request's `report_context.user_release` must not
carry one (`deny_unknown_fields`), and the notice and rows (§5.1) render the
other members only. So the rendered bytes do not depend on it, and binding their
hashes is not circular. The record then covers the report content, excluding
only the binding itself: the caller's record (rendered and in the manifest)
plus the hash of every member it covers.

**The three hashes, precisely.**
- **Model document SHA-256** (`model_document_sha256`): SHA-256 over the RFC
  8785 (JCS) bytes of the session model, as `canonicalSha256Hex(model)`
  computes it (`reportPackageRequest.ts:213`). It is the same value as:
  - the request's `audit_manifest.model_hash.value` (`:231-237`);
  - the result document's `reproducibility.model_hash.value` (bound equal by
    `bindings.rs:229-241`);
  - the native SHA-256 of `modelCanonicalText` (`report_package_save.rs:613`).
- **Result-set identity** (`result_envelope_id` and
  `result_document_derivative_sha256`): the DEL-08-04 0.3.0 document's
  envelope id and its derivative hash, SHA-256 over the RFC 8785 bytes of the
  document without its own hash (`resultExportAdapter.ts:153`).
  - The producer verifies that hash (`document_form::check_self_hash`), and
    the reader re-verifies it for every result member (`reader.rs:927-940`).
  - The derivative hash, not the member's byte hash, is chosen because it is
    the document's own recorded identity and is known before the report is
    built. `content_binding` records the member's byte hash.
- **Audit-manifest input-manifest hash** (`input_manifest_sha256`): the
  request's `audit_manifest.input_manifest_hash.value`
  (`reportPackageRequest.ts:238-244`), which native admission binds to the
  SHA-256 of `inputManifestCanonicalText` (`report_package_save.rs:624-631`).
  `input_manifest_ref` is that hash record's `payload_ref`, which admission
  binds to the session's reference (`:645-652`).

**Where it is written.**
1. `report.report_context.user_release` in the request, from which the
   renderer emits it into the HTML and PDF members (§5.1). The Rust
   `ReportContext` (`report_renderer/src/lib.rs:100-115`) gains
   `#[serde(default)] pub user_release: Option<UserReleaseRecord>`.
2. The package manifest's top-level `user_release`, written by the writer as
   the typed `report_context.user_release` plus `content_binding` (one source
   for every caller member, so the two copies cannot differ in them). Absent
   from every package without a user release.
3. The save receipt gains `user_release_recorded: bool` and
   `user_release_model_document_sha256: String` ("" when not recorded), shown
   in the panel's status line (`RenderedReportPanel.tsx:288-306`).

**Session.** The selection lives only in the dialog's React state and in the
argument of the one save it starts:
`handleSaveShareableReportPackage({ releasedAt })`, beside
`handleSaveReportPackage` (`workspaceSession.ts:2869-2947`).
- Its basis is the same basis without `comparison`, `comparisonUnavailable`,
  `projectSummary`, `ruleCheckAggregate`, `ruleCheckRun` and `privateIntent`,
  and the builder computes every bound value from that basis (§2.6).
- The run record it sends is the solve's own transient record, built by
  `buildAnalysisRunPreview(result, { inputManifest })` with no rule-check
  aggregate (`previewService.ts:565-581`), not the session's composed record.
- Nothing is written to session state that outlives the save, to the SQLite
  project, to the model document, or to any session snapshot.
- The dialog's open flag joins the modal guards of `runMenuCommand`
  (`workspaceSession.ts:2955`).
- If any basis member changes while the dialog is open, the dialog closes, and
  the next attempt needs a new confirmation.

### 2.4 Solve attestation

Native admission cannot derive a result value from a caller's carrier: the
carrier is supplied by the webview, and bindings establish consistency between
members, not that a solver produced them. So the host records what its own
solver produced. There is no rule-check attestation under R.

**In the worker** (`solve_worker.rs`, and the solve wrapper
`solve_preview_mechanics_with_mode`, `lib.rs:1828-1836`):
- before the solve, over the model value the worker has already materialized
  (`solve_worker.rs:293-298`): `d_M`, the SHA-256 of
  `checked_v1_domain_canonical("openpipestress.user_release.solved_model.v1", model)`;
- after `run_linear_static_preview_value_with_mode` returns the result value
  (unchanged), and before it is serialized: `d_S`, the same over
  `"openpipestress.user_release.solved_result.v1"` and the result value.

Each digest streams through `ByteSink` into a SHA-256 hasher, so no canonical
text is held. If the checked profile refuses either value, the worker sends no
digests. The solve and its result are unchanged; that result cannot be
user-released (`REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`).

**Transport.** A new frame `ATST {status, 0, 0}` precedes `RSLT`, followed by
64 bytes (`d_M ‖ d_S`) when `status` is 1 and by nothing when it is 0. The
parent reads it into a fixed 64-byte buffer. An `ATST` anywhere else, a second
one, or another status or body length is a protocol violation, and the job
fails as any protocol violation does today.

**In the registry.** On publication (`solve_jobs.rs:668`), the registry records
`{ job_id, solver_mode, d_M, d_S }` in a bounded session table of the
`MAX_ATTESTED_SOLVES` (32) most recent attested solves, evicting the oldest.
`dispose_all_results` (`:784`, page load) clears it. It is never persisted and
never sent to the webview.

**At admission** (route 010), admission computes the same two digests, by the
same streaming function, over the model value M and the request's
`/source_results/0` (S). The table must hold an entry with those digests whose
mode is the input manifest's `solver_basis.solver_mode`.

Both sides produce the same canonical text, and so the same digest, through the
round trip the Current export already relies on. The producer's bytes are
parsed in the webview, written into the request's RFC 8785 text and parsed
again natively, and the values are equal. A float that serde writes as `5.0`
and RFC 8785 writes as `5` has one canonical form.

A result saved in an earlier session, or one whose attestation has been
evicted, cannot be user-released until it is solved again. The Current export
already requires an in-session native invocation in the webview (§1), so the
user-visible condition is the same; native admission now enforces it.

**What solve attestation costs.**
- **Worker transport:** one frame tag (`worker_alloc.rs:63-79`) and a 64-byte
  body, read by the supervisor before `RSLT` (`solve_worker.rs:720-766`), and
  the protocol text of `solve_worker.rs:10-25`.
- **DEL-04-07 memory accounting:**
  - The worker's two passes allocate only transient pieces in its counted
    heap; no canonical text is held. The pieces live at once are (Codex
    round 3, non-blocking):
    - one key vector per object on the current traversal path:
      `canonical_json::sorted_keys` allocates a `Vec<&String>` of the
      object's member count (8 bytes per member), and it stays alive while
      that object's members, and every object nested in them, are written
      (`core/serialization/canonical_json/src/lib.rs:339-394`). Their sum is
      at most 8 bytes per object member of the value, before allocator
      rounding of each vector. The recursion is at
      most 128 deep, since the checked profile refuses deeper values first
      (`:226-253`);
    - the stable sort's scratch buffer for the object being sorted, freed
      before its members are written;
    - one string's escaped form, from `write_string` (`:335-337`), or the
      bounded simultaneous temporary strings used by `canonical_number`,
      `ecma_number_to_string` and `shortest_digits` (`:400-497`), each freed
      after it is put (Codex round 4, non-blocking).
  - The worker's existing growth path covers that heap. The grant formula, the
    result lease and the 2R custody are unchanged. The allocation checks of §6
    measure the peak on values built to maximize each piece.
  - The parent holds 64 bytes per frame and a fixed table of at most 32
    entries, under 8 KiB, inside its process reserve. There is no new lease.
  - Admission's digests are streamed too, with the same transient pieces,
    inside the command's existing ingress lease. Route 010's lease also
    prices the parse of embedded provenance (§2.7).
  - The memory-budget design's worker paragraph
    (`DEL-04-07/Design/memory-budget-decision.md:37`) names the frame and the
    digest passes. The worker's process-reserve measurement
    (`process_reserve_covers_a_real_workers_measured_footprint`) is run again.
- **Time:** two linear passes in the worker and two at admission.
- **Priced files: none.** Every change is in `apps/desktop/src-tauri`, which is
  outside `core/product_physics`'s dependency closure
  (`DEL-04-07/Design/desktop-exact-outputs.md` §3). `canonical_json` and
  `result_export` are in that closure (`core/product_physics/Cargo.toml`). R
  uses them only through existing public functions, unchanged:
  `checked_v1_domain_canonical`, `ByteSink` and `derive_document`. The solver
  entry `run_linear_static_preview_value_with_mode` is unchanged. **No D3 turn
  is needed.**
  - **Flag.** An implementation that finds it must change
    `core/serialization/canonical_json`, `core/reporting/result_export` or
    `core/product_physics`, for example to add a plain RFC 8785 streaming
    writer, moves priced source. It then needs a HELP_HUMAN D3 turn before it
    proceeds. This design avoids that on purpose.

**Why the worker, not the host.** The host never materializes the model, and
the bytes it receives are not RFC 8785 (§1). A host-side digest would need the
host to parse the result, which needs a new lease and changes the custody
design. The worker already holds both values.

### 2.5 Native admission on route 010

`admit_report_package` (`report_package_save.rs:591-662`). The command stays
`save_report_package`; the route id in the evidence selects the branch. Every
existing check runs on both routes in its current order. Route 010 adds these,
in this order:

1. **Evidence** (`validate_report_package_save_evidence`, `:294-328`).
   `ReportPackageSaveScreening` (`:110-125`, `deny_unknown_fields`) and
   `ReportPackageLocalFirstEvidence` (`:130-139`) gain
   `#[serde(default)] explicit_user_release_intent: bool`.
   - Route 009: unchanged, plus `explicit_user_release_intent` false, and no
     `analysisRunCanonicalText` argument.
   - Route 010: context `user_released_report`, release intent true,
     local-private intent false, lossless, not withheld, exact payload, zero
     blocking; local-first evidence with storage context `private_report`,
     reason `USER_RELEASE_METADATA_ALLOWED`.
   - Any other route id: `REPORT-PACKAGE-REDACTION-EVIDENCE-INVALID`.
2. **Texts.** The request, model and input-manifest texts, as today. Route 010
   adds `analysisRunCanonicalText`, the solve's DEL-14-02 0.3.0 transient
   record. It must be its own canonical text, within
   `MAX_ANALYSIS_RUN_TEXT_BYTES` (one model-document cap), and priced in the
   command's ingress lease as the input-manifest text is.
3. **Context** (`check_context`, `:451-491`).
   - Route 009: unchanged, plus `/report/report_context/user_release` absent,
     else `REPORT-USER-RELEASE-NOT-THIS-ROUTE`.
   - Route 010: `export_context` `user_released_report`, profile
     `desktop_user_released_report_package_1`
     (`REPORT-PACKAGE-SAVE-PROFILE-NOT-DESKTOP` otherwise), and the record
     present (`REPORT-USER-RELEASE-RECORD-INVALID` otherwise).
4. **Record form and bindings.** Exact members and constants as §2.3, no
   `content_binding`, the hex and `released_at` forms, else
   `REPORT-USER-RELEASE-RECORD-INVALID`. Then each bound member against a value
   admission itself computed or already bound, never against another caller
   field alone; any inequality is `REPORT-USER-RELEASE-UNBOUND`, naming the
   member:
   - `model_document_sha256` = `model_hex`, the SHA-256 of the model text
     admission received (`:613`);
   - `model_ref` = the model text's `/project/id` = `/source_model_ref/ref_id`;
   - `input_manifest_sha256` = `input_manifest_hex` (`:624`), and
     `input_manifest_ref` = the reference admission derives
     (`analysis_runs::input_manifest_ref`, `:645`);
   - `analysis_run_ref` = `/source_results/0/run_id`;
   - `result_envelope_id` and `result_document_derivative_sha256` =
     `/result_documents/0/result_envelope/envelope_id` and
     `…/reproducibility/derivative_hash/value` (the shared producer then
     verifies that hash over the document);
   - `scope.report_id` = `report:` + `analysis_run_ref` =
     `/report/calculation_report/report_id`, and `scope.package_id` =
     `/package_id` = `desktop-user-released-report-<run>`.
5. **Reference refusals** (§2.7): `REPORT-USER-RELEASE-LIBRARY-REFERENCED` and
   `REPORT-USER-RELEASE-RULE-PACK-REFERENCED`.
6. **Private classes** (§2.7): `REPORT-USER-RELEASE-PRIVATE-CLASS`.
7. **Solve attestation** (§2.4): `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`.
8. **The run record.** Record Run's binding checks
   (`analysis_runs::checked_submission`, `analysis_runs.rs:634-782`, factored
   to take parsed values) apply to the run record with S and the input
   manifest. They require:
   - the transient form (`created_at` null, the synthetic
     `state:<model>:preview`);
   - the record's own checksum;
   - the members the manifest fixes;
   - the exact two-hash inventory;
   - every row's checksum;
   - diagnostics equal to the result's, with no rule-pack reference
     diagnostics, since step 5 has refused every model with a rule-pack
     reference.

   Route 010 adds:
   - `analysis_status` is exactly `HUMAN_REVIEW_REQUIRED`, S's
     `status.mechanics` and S's own `status.rule_check`, so no rule-check
     aggregate (`previewService.ts:579`);
   - `run_name`, `unresolved_tbd` and `professional_boundary` are the
     builder's values (`analysisRunCompatibility.ts:192-206`).

   Any failure is `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`. The record
   reaches the package only through these members and its record hash.
9. **The derivation** (§2.6). The input manifest, then the whole request, must
   equal their derivations canonically. Otherwise
   `REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the first differing JSON
   pointer.
10. **Unchanged, on both routes:**
    - the disposition screens (`screen_content`, `:412-448`);
    - the rule-pack agreement (`:572-588`), whose two sides are both empty on
      route 010 after step 5;
    - the shared producer (§2.8).

### 2.6 The minimal native renderer input and the derivation

The local-private package admits any content the request carries, because it
never leaves the user. A shareable package cannot. Under R, native admission
does not label or bound caller content; it **derives the whole request** from
the bound basis and requires canonical equality. A new module,
`core/reporting/report_package/src/user_release.rs`, does the derivation.
`report_package` is outside `core/product_physics`'s closure.

**The bound basis:**
- **M**, the model text (SHA-256-bound and screened, `report_package_save.rs:613,412-448`);
- **S**, the source carrier, attested for M (§2.4), with its attested solver
  mode **m**;
- **I**, the input-manifest text (bound, `:624-652`);
- **R**, the run record (checked, §2.5 step 8);
- **H**, the host's own software identity (`app.package_info()`, already
  checked by `check_context`);
- **U**, the record.

**The derivation `E(M, S, m, I, R, H, U)`:**
- **Input manifest.** I must canonically equal the manifest
  `buildCurrentSessionInputManifest` builds (`inputManifestService.ts:75-129`).
  It is built from M, S's producer and m, with the solve's fixed settings for m
  (`workspaceSession.ts:1021-1035`), `active_rule_packs: []` and
  `external_assets: []`. The store state at solve time is not re-read; step 5
  has already refused any model with a rule-pack reference.
- **Result document.** `derive_document(base, M, S, origin, None)`
  (`derivative.rs:120`, unchanged). Its base and origin are built by Rust
  mirrors of `currentResultDocumentBase` (`resultExportAdapter.ts:254-256`) and
  `currentReceivedOrigin` (`:241-245`, with the constant `CURRENT_ORIGIN_LIMIT`)
  from M, S, R and I.
- **Renderer input** (the minimal native renderer input):
  - `report_title`: `{M.project.name} — Calculation Report`;
  - `calculation_report`: as the builder writes it (`renderableReportInput.ts:596-672`),
    with:
    - the persistence reference `TBD`;
    - `rule_pack_refs: []`;
    - the project and model references, model hash and load-case summary
      (id, label, kind) from M;
    - the display summary's result units from S;
    - the result references and row checksums, the two audit references, the
      analysis status and the unresolved TBDs from R;
    - the builder's constants: template slots, rendered sections, boundary,
      session provenance and privacy class;
  - `report_sections`:
    - the model and run references;
    - S's diagnostics only, as `diagnosticsForSections` maps them (`:139-160`);
    - the status disclosures from R's status;
    - `provenance_notes`: the session provenance only;
    - `user_supplied_values: []`;
    - `assumptions: []`;
    - the fixed technical-preview limitation;
    - the unresolved TBDs from R;
    - the boundary;
  - `result_rows`: the existing five-field projection of S's rows
    (`expected_report_result_rows`, which the producer already requires);
  - `report_context`:
    - export context `user_released_report`;
    - `generated_at` null;
    - software name and version from H;
    - build identifier null;
    - solver name, version and build reference, run name and settings
      reference from R;
    - `analysis_run_recorded_at` null (the transient record's `created_at`);
    - solver mode and settings from I (each value's canonical JSON text, keys
      sorted, as `buildReportContext` writes them, `:361-402`);
    - model coordinate system null;
    - result coordinate systems from S's row metadata;
    - `user_release` U;
  - `user_rule_checks`: absent.
- **Audit manifest and top-level members**, as `buildReportPackageRequest`
  writes them (`reportPackageRequest.ts:221-258`), with these differences:
  `rule_pack_refs: []`, the route-010 `package_id` and profile,
  `state_comparison_handoff_records: []`, `rule_check_aggregate: null` and
  `solve_rule_check_status` equal to S's.

The minimal input differs from the local-private input in what it leaves out:
component and spring-hanger provenance entries, dedicated per-object hanger
quantity entries and their provenance diagnostics, rule-pack references and
user-rule outcomes. These return with the later slice (§10).

The solve's review rows are retained. `result_rows` is the projection of every
`results` row of S, so a spring hanger's user-entered stiffness and loads
appear, as the solver wrote them, in S's `spring_hanger_user_input_review` rows
(`core/product_physics/src/lib.rs:13429,13530`), for an ordinary user-entered
hanger as for any other. Only the dedicated per-object entries are left out.

**Parity.** The webview builds the same request with
`buildUserReleasedReportPackageRequest` (`reportPackageRequest.ts`) and a
`userReleasedRenderableReportInput` builder. Each Rust builder is held equal to
its TypeScript counterpart by committed invented vectors,
`fixtures/reports/invented/user_release_derivation_vectors.json` (inputs M, S,
m, I, R, H, U and the expected request), read by a vitest and by a Rust test,
both routed. A change to a builder changes its mirror and the vectors together,
as W-18a's row parity does (W-18a design §2.3 item 6).

**Why derivation replaces round 1's closed table.** Every member is equal to a
fixed derivation from M, S (the solver's own output for M), R, I, H and U, so
no member or value can exist that the derivation does not produce. That closes
Codex round 1 finding 1 (unconstrained annotations and opaque records) for R:
- source annotations come only from the attested S;
- there is no handoff record;
- an extra key added only in the request fails the attestation or the
  equality.

It closes finding 3 (a library quantity under a substituted source) too: R
carries no per-object entries, and every model with a detected library origin
is refused (§2.7).

### 2.7 The library and rule-pack refusals, and private classes

**"Library-referenced" is determined from the bound model M alone.** The
detector recognizes each form in which a library apply has ever written a
library origin into a model. The history of the two writers (`git log` of
`hangerSelection.ts` and `libraryApply.ts`) shows three forms:

| Form | Written by | Where in M | Rule |
|---|---|---|---|
| A `library_ref` object in `provenance_record` | every W-17 apply (material, section, component, hanger) from `00af9112c0`, 2026-10-23 (`libraryApply.ts:224`; `hangerSelection.ts:55,70-82`) | the applied entity's `provenance_record` | 1 |
| The library provenance string `library:<kind>:<library id>#<record id>` | the same applies (`libraryApply.ts:81-83,399,445-446,511,702-708`; `hangerSelection.ts:41-42,54`) | the entity's `provenance`; component source-reference strings; a hanger support's `provenance` | 2 |
| The legacy hanger selection evidence: the RFC 8785 JSON text of `{hanger_id, kind: "user_imported_hanger_selection", library_id, library_kind: "hanger", library_metadata, record_snapshot}`, held as a string | the hanger selection from `46f3aed341` (2026-09-05) until `00af9112c0` replaced it (`hangerSelection.ts:35-39,52` at `00af9112c0^`) | a hanger support's `provenance`, with no `library_ref` and no `library:` string | 3 |

There is no other form. Before `00af9112c0` no material, section or component
apply existed: `libraryApply.ts` arrived in that commit, and the model's
`LibraryRef` with model 0.7.0 (`60ed7f6478`). The hanger selection is the only
writer of `user_imported_hanger_selection` in the history. Committed model
fixtures carry only the W-17 forms (`fixtures/model_document/desktop_emitted.json`,
`library_ref_sites.json`).

**The rules.** A *string* here is every JSON string value at any depth of M:
an object member's value or an array element. Object keys are not tested. The
substring tests are exact and case-sensitive, over the string's characters,
with no trimming. M is library-referenced when any of these holds:
1. **A `library_ref` object at any depth.** The test is
   `export_disposition_screen::library_ref_sites(M)` non-empty
   (`export_disposition_screen.rs:310-319`): the one walk, pinned to
   `fixtures/model_document/library_ref_sites.json`. A site whose identity is
   blank counts too.
2. **A string that holds a library provenance reference**: it contains one of
   `library:material:`, `library:section:`, `library:component:` or
   `library:hanger:`. A model whose `library_ref` was removed but whose values
   still name their library is refused too.
3. **A string that holds legacy hanger selection evidence**, in either of two
   ways:
   - (a) it contains `user_imported_hanger_selection`;
   - (b) it is *embedded JSON*: after removing leading and trailing JSON
     whitespace (space, tab, line feed, carriage return), it begins with `{`,
     `ingress::prescan` (`src-tauri/src/ingress.rs:550`) accepts it as one
     JSON value, and it parses (`serde_json::from_str::<Value>`) to an object.
     It holds legacy evidence when that object has a member named
     `library_kind`, `library_id`, `library_metadata` or `record_snapshot`, or
     when rule 1, 2 or 3 (a) holds within the parsed value
     (`library_ref_sites` over it, and the substring tests over its strings).

   Parsing goes one level deep. Strings inside the parsed value are tested by
   the substring rules, not parsed again: no apply wrote embedded JSON inside
   embedded JSON. A candidate the prescan refuses for nesting deeper than 127
   containers (`ingress.rs:29,605-606`), for serde_json's reserved raw-value
   key (`:653`; both pinned by the test at `:1046-1056`), or as too large to
   price (`:355-360`) cannot be checked, and is refused under rule 3. Every
   other prescan finding is one serde_json's parse would refuse too. A string that does not begin with `{`, that the prescan
   refuses with such a finding, or that parses to a non-object is text only,
   and rules 2 and 3 (a) still apply to it.

The code is `REPORT-USER-RELEASE-LIBRARY-REFERENCED`. It names the first site
found, taking rule 1, then rule 2, then rule 3, each in document order. It
gives the site's JSON pointer, the rule (`library_ref`, `provenance_string` or
`legacy_hanger_selection`), and the library kind and id where the site states
them. For rule 3 the kind is `hanger`.

**Cost.** Rules 1, 2 and 3 (a) are linear passes over the parsed M. Rule 1's
walk returns one path per `library_ref` site, as Record Run's use of it does
today (`analysis_runs.rs:399`); rules 2 and 3 (a) allocate nothing. Admission
refuses at the first site. For rule 3 (b), admission first prescans every
candidate (a string that, trimmed as above, begins with `{`; `ingress::prescan`
allocates nothing) and grows the
command's ingress lease once, by the largest candidate's
`text_parse_bytes(·, Moved)`. It then parses the candidates one at a time and
drops each before the next, so at most one embedded value is held beside M.

**The webview** applies the same three rules as an early check
(`libraryRefSites`, the substring tests, and `JSON.parse` in a `try` for 3 (b)),
which disables the dialog's save (§2.2). Native admission decides; it alone
refuses the unchecked candidates, which `JSON.parse` may accept.

**The reliance limit.** The rules find the marks that library applies wrote.
They cannot find a library origin that a user has erased: a value whose every
mark was removed, or a value typed in by hand from a library or another
source, has no detectable library origin. The detector therefore never shows
that a value is the user's own. For every value without a detectable library
origin, the release relies on the user's own statement of their rights, the
checkbox statement of §2.2 that the record carries (§2.3). The record states
that statement; it does not prove it (§2.9).

This resolves Codex round 2's blocker and round 3's. The library-applied
bend's raw flexibility factor and SIF reach result rows, contract evidence and
diagnostics through genuine solver output (§1), and so do a legacy library
hanger's loads and stiffness, in `spring_hanger_user_input_review` rows
(`core/product_physics/src/lib.rs:13429,13530`). Attestation reproduces them.
Under R a model with a detected library origin never reaches the derivation.

**Rule-pack-referenced.** M is rule-pack-referenced when the walk finds any
`rule_pack_refs[]` entry (`export_disposition_screen.rs:231-296`), or when I
records any active rule pack. The code is
`REPORT-USER-RELEASE-RULE-PACK-REFERENCED`. HELP_HUMAN settled this refusal
on 2026-10-25 (§0). It follows from R:
- R carries no rule-pack reference and no rule-check outcome;
- the package's agreement rule requires the package to name every active pack
  of the solve (`report_package_save.rs:572-588`), which needs the rule-check
  run that R does not carry;
- references the manifest leaves out become rule-pack diagnostics in the run
  record (`analysis_runs.rs:298-343`).

The later slice, which stays in v0.1, carries rule-pack references at safe
metadata and lifts this refusal together with the library refusal (§10). Until
then the dialog says plainly that such a model cannot be shared yet, and why
(§2.2).

**Private classes (defence).** Any object in M or the request whose own
`privacy_classification` or `privacy_class` is a non-project private class is
refused with `REPORT-USER-RELEASE-PRIVATE-CLASS`. The non-project private
classes are `private_material_data`, `private_component_data`,
`private_rule_pack_data`, `owner_standard_data`, `company_design_basis_data`,
`path_data` and `secret_like_data`. A label can add a refusal; no label grants
inclusion.

### 2.8 Shared producer, writer and render command

**Shared producer** (`report_package::bindings::check_request`,
`bindings.rs:180-496`):
- the profile table (`:213-217`) gains
  `(USER_RELEASED_DESKTOP_PROFILE, DESKTOP_ORIGIN) => true`;
- for that profile the request must carry no handoff record set
  (`REPORT-PACKAGE-USER-RELEASE-UNBOUND` otherwise). The checks that read the
  handoff's run section (`:319-366`, `:403-409`, the handoff status at `:446`)
  have nothing to read. The run record they bind is not in the request
  (`bindings.rs:8-11`) and is bound natively by admission (§2.5 step 8). Every
  other item runs unchanged;
- item 7 (`:454-489`) is unchanged: with no aggregate, the report carries no
  outcomes and no rule-pack reference;
- item 8 (`:491-494`) becomes:
  - the local-private profiles (desktop and runner) require `export_context`
    `local_private` and no record (`REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`
    if a record is present);
  - the user-released profile requires `user_released_report` and a record;
- a new item 9 checks the record against the request's members, each
  inequality `REPORT-PACKAGE-USER-RELEASE-UNBOUND`:
  - `model_ref` = `source_model_ref.ref_id` = the document's
    `model_ref.ref_id` = `audit_manifest.model_hash.payload_ref`;
  - `model_document_sha256` = `audit_manifest.model_hash.value` = the
    document's model hash;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit
    input-manifest hash value and payload reference;
  - `analysis_run_ref` = the document's `run_ref.ref_id`;
  - the envelope id and derivative hash = the document's;
  - `scope.report_id` = `report:<analysis_run_ref>` = the report's
    `report_id`, and `scope.package_id` = the request's.

The headless runner keeps its profile (`bindings.rs:36`), so a runner request
with a record is refused. The runner gains no flag.

**Writer.**
- `ReportPackageContainerInput` (`lib.rs:116-136`) and `ManifestFields`
  (`lib.rs:643-650`) gain `user_release: Option<&Value>`.
- With a record, the writer requires zero state/comparison/handoff records and
  blocks any (`lib.rs:255-259` blocks none today). Without a record it requires
  at least one, as today.
- `package_manifest_document` (`lib.rs:652-734`) writes `"user_release"` only
  when present: the caller's record plus `content_binding` over the
  materialized members (`lib.rs:304-335`). It then writes the user-released
  boundary notes (§5.3).

**Render command.** `render_calculation_report_value`
(`src-tauri/src/lib.rs:1553-1566`) refuses an input whose
`report_context.export_context` is `user_released_report`, with
`RENDER-USER-RELEASE-PACKAGE-ONLY`. A user-released document is therefore
produced only inside the package admission. The webview already refuses that
context on this route (`reportRenderService.ts:124-140`).

### 2.9 How the reader verifies it (DEL-02-05), and what that establishes

`read_report_package_container` (`reader.rs:161-263`):
- `manifest_fields` (`:683-721`) reads an optional `user_release` and passes
  it to the rebuild (`:225-240`), so a record outside the defined form fails the
  existing manifest-form check.
- The unblocked-package member rule (`:960-967`) requires at least one
  state/comparison/handoff member only when no record is present.
- After `verify_members` (`:807-969`), when a record is present, each failure
  is `OPSPROJ_USER_RELEASE_UNBOUND` with its reason:
  - the package is unblocked, and `export_profile_id` is
    `desktop_user_released_report_package_1`;
  - **cardinality:** exactly one HTML, one PDF, one audit and one result
    member, and no state/comparison/handoff member;
  - the record's constants and forms hold;
  - `model_ref` = the manifest's `source_model_ref.ref_id` = the result
    member's `model_ref.ref_id` = the audit member's `model_hash.payload_ref`;
  - `model_document_sha256` = the audit member's `model_hash.value`;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit member's
    `input_manifest_hash.value` and `payload_ref`;
  - the result member's `envelope_id`, derivative hash value and
    `run_ref.ref_id` = `result_envelope_id`,
    `result_document_derivative_sha256` and `analysis_run_ref`;
  - `scope.package_id` = the manifest's `package_id`, `scope.report_id` =
    `report:<analysis_run_ref>`, and `analysis_run_ref` =
    `source_basis_refs[0].ref_id`;
  - `content_binding.members` lists exactly the non-manifest members, by file
    name and role in archive order, and each `sha256` equals the SHA-256 of
    that member's bytes (the inventory hash already equals it);
  - the HTML member contains the HTML-escaped text of
    `user_release_notice_text(record)` exactly once, regenerated by the
    renderer's own function from the manifest record. This ties the rendered
    notice to the manifest copy. The PDF is tied by its byte hash only.
- A user-released profile without a record, or a record with any other
  profile, is refused.
- Refusal: a new variant `PackageReadRefusal::UserReleaseUnbound { reason }`,
  code `OPSPROJ_USER_RELEASE_UNBOUND` (`reader.rs:90-121`).
- `ReadReportPackage` (`:64-85`) gains `user_release: Option<Value>`, so a
  consumer can show the record.

The audit and result identities are collected in `verify_members` as the audit
identity already is (`:917-920`).

**What these checks establish, and what they do not (O-G).** The reader's
checks are equality checks, not signatures (`reader.rs:30-41`). They establish
consistency: the record, the members and the member bytes agree with each
other. At save time, native admission also establishes that:
- the record came through route 010 with the wrapper's release intent;
- the model and result were this session's solve;
- every member was derived from them.

Neither establishes authentic user consent (that a person made the statement)
or producer origin (that this application wrote the package), and neither
establishes that the statement is true. Anyone who can write the file can build
a coherent package with a record, its members and every hash re-computed
together, and the reader will not detect it. The reader's module text
(`reader.rs:1-42`) says this in these terms, and so does DEL-02-05 (A8).

### 2.10 No replay

The claim is narrow: **an unchanged record** cannot be carried onto another
model, result set, report, package or member content. A record that is
re-written to match its new place is not detected (§2.9).

| Attempt | Refused by | Code |
|---|---|---|
| Record copied into a request for another model | native admission (model text SHA-256, `model_ref`), then the producer (audit and document model hash and reference) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-PACKAGE-USER-RELEASE-UNBOUND` |
| Same model, another solve or result document | native admission (run, envelope id, derivative hash, attestation); the producer | as above; `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED` |
| Same result, another input manifest or run record | native admission (input-manifest text SHA-256 and reference; the run-record checks) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND` |
| Record with only `model_ref` or only `scope.report_id` changed | native admission; the producer; the reader | as above; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed on the local-private route 009 | native admission; the producer (local-private profile) | `REPORT-USER-RELEASE-NOT-THIS-ROUTE`; `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` |
| Manifest record copied into another package, re-hashed | the reader (bound identities, `content_binding`) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Same package, HTML, PDF or audit member swapped and the inventory restamped, record unchanged | the reader (`content_binding`; the notice text in the HTML) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| A second result member, or a state/comparison/handoff member, added, inventory restamped | the reader (cardinality) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed in a standalone render | webview route check; the render command; the renderer (record outside its context) | `REPORT-REDACTION-CONTEXT-MISMATCH`; `RENDER-USER-RELEASE-PACKAGE-ONLY`; `REPORT-CONTEXT-USER-RELEASE-MISMATCH` |
| A later save reusing an earlier selection | nothing to reuse: the selection is not stored (§2.3) | none needed |
| A record whose statement, standing or `not_carried` text is altered | native form check; the reader | `REPORT-USER-RELEASE-RECORD-INVALID`; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record, members, `content_binding` and inventory all re-written together | not detected (equality checks, §2.9) | none |

## 3. The classification and routes

### 3.1 DEL-12-02 runtime classification

**Name:** a new export context, `user_released_report`.

**Meaning (proposed):** a report of the user's own session that the user has
released for sharing, for that one report, by an explicit selection with a
statement that they hold the redistribution right
(`docs/IP_AND_DATA_BOUNDARY.md` §6). In it:
- the user's private project data (`private_project_data`) is included, with a
  `USER_RELEASED_PROJECT_DATA` warning finding, only with the wrapper's release
  intent;
- every other private class is redacted as in `public_report`, so a lossless
  route refuses;
- `protected_suspected`, `quarantined` and `rejected` content blocks;
- unknown or missing metadata blocks (`USER_RELEASE_VALUE_UNBOUND`); unlike
  `local_private`, it is never warning-only;
- the context is admitted only on route `DREP-PACKAGE-USER-RELEASE-010`, with
  the wrapper's release intent. Payload content never selects or grants it.

It is an export context, not a privacy class, on purpose. A privacy class such
as `user_released_project_data` would be carried in the payload, and any
context that included it would let payload content grant exposure. The existing
design gives that authority only to the wrapper
(`docs/security/redaction_export_controls.md:158-163`).

**Vocabulary additions** (DEL-12-02 CLM-003, the schema and the three mirrors):
- export context `user_released_report`;
- reason codes `USER_RELEASED_PROJECT_DATA` (finding class
  `PRIVATE_DATA_WARNING`, severity WARNING), `USER_RELEASE_INTENT_REQUIRED`
  (`PRIVATE_DATA_WARNING`, BLOCKING), `USER_RELEASE_VALUE_UNBOUND`
  (`PROVENANCE_WARNING`, BLOCKING), and the local-first reasons
  `USER_RELEASE_METADATA_ALLOWED` and `USER_RELEASE_ROUTE_REQUIRED`;
- route id `DREP-PACKAGE-USER-RELEASE-010`.

### 3.2 The decision rule, in exact order (Codex round 1, finding 2)

`actionFor` (`redactionExportControls.ts:417-479`) and `_action_for`
(`controls.py:372-426`), and the Rust mirror, evaluate:
1. professional claim → `block_export` / `PROFESSIONAL_BOUNDARY_BLOCKED`;
2. cloud or network reference → `block_export`;
3. direct SQL access → `block_export`;
4. storage bypass → `block_export`;
5. secret material → `block_export`;
6. contained payload → `block_export` / `PAYLOAD_METADATA_ONLY_REQUIRED`;
7. concrete path → `redact_value` / `CONCRETE_PATH_REDACTED`;
8. privacy or redistribution `protected_suspected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`;
9. review status `quarantined` or `rejected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`.

Steps 1-9 are today's, unchanged, for every context. Then, **only when the
context is `user_released_report`**, steps 10-13 decide and return, before any
unknown-metadata branch:

10. privacy in the non-project private set (§2.7), whatever the redistribution
    status (`unknown` included) → `redact_value` / `PRIVATE_DATA_REDACTED`. On
    the lossless route this withholds the payload (`controlRouteExport`,
    `:968-969`), so the save is refused;
11. privacy `private_project_data`, whatever the redistribution status: with
    the wrapper's release intent → `warning_only` /
    `USER_RELEASED_PROJECT_DATA`; without it → `block_export` /
    `USER_RELEASE_INTENT_REQUIRED`;
12. privacy in `{public, public_metadata, invented_public_example}` and
    redistribution in `{public_permissive, invented_non_engineering_example}`
    → `include` / `SAFE_PUBLIC_METADATA` (or `SAFE_INVENTED_PUBLIC_EXAMPLE`);
13. anything else (missing metadata; unknown, `TBD` or null privacy or
    redistribution; `private_only` with a non-private privacy; any
    unrecognized value) → `block_export` / `USER_RELEASE_VALUE_UNBOUND`.

For the five existing contexts, step 10 onward is today's sequence, unchanged:
missing metadata, unknown redistribution, unknown privacy, the safe include,
the private branch and the fallback (`:445-478`; `controls.py:406-426`).
`unknownAction` and `_unknown_action` keep their bodies; the new context never
reaches them. Payload-carried local-private intent (`itemIntent`, `:394-411`)
is not read in the new context.

**The release projector** (`controlUserReleasedReportPackageRequest(payload,
model, explicitUserReleaseIntent)`, route 010, lossless) labels each leaf with
the privacy and redistribution of its nearest enclosing object that states them,
else `private_project_data` / `private_only`. So:
- a leaf of the derived request is decided by step 11: a warning with the
  wrapper's intent, a block without it;
- an object labelled with a non-project private class is withheld by step 10;
- a protected, quarantined or rejected object blocks (steps 8-9);
- an object with unknown own metadata blocks (step 13).

There is no path-token labelling and no field table on this route. The
webview's check is early; native admission's derivation decides (§2.6).

### 3.3 Context and route

**A new governed route, `DREP-PACKAGE-USER-RELEASE-010`, with the new context.**
Reusing `shared_model` or `public_report` was considered and not chosen: either
would need a release flag inside an existing context, so that context's
behaviour would depend on a flag. The owner's check "redaction applies to every
other context" would then depend on every caller, not on the vocabulary.

Route tables and local-first admission:
- `GOVERNED_LOCAL_FIRST_ROUTE_IDS` (`redactionExportControls.ts:135-149`;
  `core/security/local_first_storage/route_control.py:17-41`, the
  `route_control.py` of this section) gain the route id.
- `enforceLocalFirstRoute` (`redactionExportControls.ts:151-209`) and
  `enforce_local_first_route` (`route_control.py:70-140`) gain an
  `explicitUserReleaseIntent` / `explicit_user_release_intent` option:
  - `user_released_report` on any route other than 010, or route 010 with any
    other context: `block_storage` / `USER_RELEASE_ROUTE_REQUIRED`;
  - route 010 without release intent: `block_storage` /
    `USER_RELEASE_INTENT_REQUIRED`;
  - otherwise `include_metadata_only` / `USER_RELEASE_METADATA_ALLOWED`, with
    storage context `private_report`. The file is written to the user's own
    local path by the same atomic save; the user release governs its content,
    not where it is stored. Python guards the route-metadata record under that
    existing local-private storage context (`local_first_storage/controls.py:25-33`),
    so the storage vocabulary does not change.
- `ROUTE_STORAGE_CONTEXTS` (`route_control.py:44-50`) maps
  `user_released_report` to `private_report`.
- **The intent strip is not widened (Codex round 1, finding 5).**
  `ROUTE_INTENT_KEYS` (`redactionExportControls.ts:708-712`), `stripRouteIntent`
  (`:825-842`) and the Python projector's strip
  (`core/security/redaction/route_control.py:19-23,159-180`) keep their current
  key lists.
  - Release intent is read only from the wrapper's argument, so a payload
    member named `user_release_intent` or `explicit_user_release_intent`
    grants nothing and needs no strip.
  - In the five existing contexts and on routes 009, `DREP-IPC-003` and
    `DREP-HTML-SAVE-005`, such a member is raw content, preserved byte for byte
    as today, so a result document's self-hash and the exact-payload agreement
    are unaffected.
  - On route 010 it is not in the derivation and is refused natively
    (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, or
    `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED` inside the carrier).
- `redactExportPayload` and `controlRouteExport` refuse
  `explicitUserReleaseIntent: true` with any context other than
  `user_released_report` (an exception, as `assertContext` throws,
  `:665-669`). The flag therefore cannot enable export in another context.
- The Rust mirror (`core/runner/headless/src/redaction_binding.rs:166,216-218`)
  implements the same rule so the shared corpus
  (`fixtures/redaction_export_controls/cases.json`, read at `:702-740`) holds.
  No runner route uses the context. Python carries the decision rule for corpus
  parity; no Python route uses the release projector.
- Schema `redaction_export_controls.schema.yaml`: `ExportContext` (:402-411)
  and `ReasonCode` (:485) enums gain the new values.

### 3.4 What still refuses, and what is unchanged

- **Exact routes.** Route 010 uses the same output-policy cell
  (`reportPackageUnavailableReason`, `reportPackageRequest.ts:60-62`), the same
  surface refusal (`:168`) and the same producer preflight
  (`wire.rs:281-283`). Exact routes stay refused with
  `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` and `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`
  (ruling 10A).
- **Protected-content linter.** The renderer runs it before and after rendering
  in every context (`report_renderer/src/lib.rs:1229-1267,1306-1325`); a
  blocking finding sets `export_blocked`, and admission refuses a blocked
  package (`REPORT-PACKAGE-PRODUCER-BLOCKED`, `report_package_save.rs:658-660`).
- **Disposition blocks.** The webview blocks `protected_suspected`,
  `quarantined` and `rejected` before any context branch (steps 8-9 of §3.2).
  Native admission runs both screens on route 010 as on 009
  (`report_package_save.rs:412-448`): `PROTECTED_CONTENT_BLOCKED` and
  `REPORT-PACKAGE-SAVE-DISPOSITION-MALFORMED`.
- **Every other context is unchanged.** `public_report`, `public_example`,
  `shared_model`, `downstream_tool` and `local_private` keep every decision on
  every input, and their controlled payloads keep their bytes, including raw
  members named like the new intent keys (§3.3). Routes 009, `DREP-IPC-003` and
  `DREP-HTML-SAVE-005` keep their contexts and evidence. A local-private
  package is byte-identical to one assembled before W-18b (§4).

## 4. Persistence and format

**Version: V1, stay at 2.0.0 (owner, 2026-10-25).** The owner's DEC-028
reading, recorded in §0, is: "one version" admits an optional writer-defined
record that the reader at the same head accepts. Under that reading the
user-released form is the 2.0.0 container selected by the optional record.
With the record:
- the manifest carries the optional `user_release` member (§2.3) with its
  `content_binding`;
- the manifest's boundary notes are the user-released set (§5.3);
- the writer writes no state/comparison/handoff member (§2.1, §2.8).

Without the record, every rule is today's. The reader at the same head accepts
exactly that form and refuses any mix: a record with a handoff member, a
handoff member missing without a record, a record under another profile.

**Unreleased packages stay byte-identical.** A package without a user release is
byte-identical to one assembled before W-18b. Its members, its manifest and its
notes are unchanged. Every existing package, fixture and witness is unchanged,
and so are the runner's witnesses. The intent strip is not widened (§3.3), so
no payload's bytes change. This is the most literal reading of "without the
selection, the report stays local-private exactly as today". It is held by
`local_private_package_bytes_unchanged_by_user_release` (§6).

**Consequence.** A reader built before W-18b refuses a user-released package as
`OPSPROJ_MANIFEST_MALFORMED` ("manifest differs from the version 2.0.0 manifest
form at: user_release", `reader.rs:236-240`), not by version name. No such
reader has been released.

**The absent handoff member is part of the record-selected form.** HELP_HUMAN
settled on 2026-10-25 that V1 covers R's package form (§0): the reader at this
head accepts what the writer at this head emits, and that includes a
user-released package with no state/comparison/handoff member. So R keeps no
handoff record set, and the reader admits its absence only with the record.

**Rendered reports.** No JSON schema file describes the renderer input; its
contract is the Rust type (`report_renderer/src/lib.rs:184-194`) and the
TypeScript builder. Changes:
- `ReportExportContext` (`:80-83`) gains `UserReleasedReport`
  (`user_released_report`); `export_context_label` (`:365-370`) gains it;
- `ReportContext` gains `user_release` (§2.3), defaulting to `None`;
- `UserReleaseRecord` is a new typed struct with `deny_unknown_fields` and no
  `content_binding` member;
- `report_context_blocking_codes` (`:1182-1213`) adds
  `REPORT-CONTEXT-USER-RELEASE-MISMATCH` when the context is
  `user_released_report` without a record, or a record is present in another
  context;
- TypeScript `ReportExportContext` (`renderableReportInput.ts:67`) gains the
  value.

`report_generator` and `schemas/report_generator.schema.yaml` do not change:
the record is in `report_context`, not in `calculation_report`. Existing
local-private and public renders are byte-identical, since the new member
defaults to absent and adds no row or banner then.

**Witnesses and fixtures.**
- Unchanged:
  - the headless runner inputs and witnesses
    (`validation/witness/inputs/del1005_*`, `validation/witness/generated/del1005_*`);
  - every existing reader and container fixture;
  - the existing corpus cases;
  - the owner-held `apps/desktop/native-smoke/expected.json` (not touched).
- New, invented:
  - a user-released profile in the shared basis builder
    (`core/reporting/report_package/tests/common/package_basis.rs`);
  - a committed record for that basis,
    `fixtures/reports/invented/user_release_record.json`;
  - the derivation vectors (§2.6);
  - new corpus cases in `fixtures/redaction_export_controls/cases.json`, each
    with `explicit_user_release_intent`.

## 5. The notice

### 5.1 Notice text (proposed exact)

The renderer gains `user_release_notice_text(input) -> Option<String>`. The
HTML body uses it (beside `local-private-banner`, `report_renderer/src/lib.rs:1281-1285`),
and so do the PDF (`pdf_emitter/src/lib.rs:236`) and the reader's notice check
(§2.9). It is emitted only for `user_released_report` with a record, in a
banner of its own,
`<div class="user-release-banner" data-export-context="user_released_report">`:

> USER-RELEASED REPORT — the user released their own project data in this
> report for sharing, stating that they hold the right to redistribute it
> (IP and data boundary §6). The release covers this report only: model
> SHA-256 {model_document_sha256}, result document {result_envelope_id}
> (SHA-256 {result_document_derivative_sha256}), released {released_at}.
> The report carries this model's summary, load cases and solve results, and
> no rule-pack reference, user-rule outcome or run comparison. The application
> found no library or rule-pack mark in this model. It cannot verify who holds
> the rights in the data, which rest on the user's statement. This statement
> concerns rights in the data only; the analysis status and the Professional
> Boundary notice below are unchanged.

The run-context table (`run_context_rows`, `:372`) shows "Report export
context: user_released_report" and three added rows:
- "User release statement": the statement;
- "User release time (application clock)": `released_at`;
- "User release bound to": the model, run, input-manifest and result-document
  identities.

`content_binding` is not rendered.

### 5.2 Relation to the other notices

- **PRD §19.3** (`docs/PRD.md:1290-1296`; renderer `:1007-1030`): unchanged, in
  every report, user-released or not. A user release changes no analysis status
  and adds no review record.
- **The local-private notice** (W-3, `local_private_notice_text`,
  `report_renderer/src/lib.rs:1169-1178`): unchanged and still emitted for
  every `local_private` report. A user-released report carries the user-release
  notice in its place, because the two contexts exclude each other.
- **The panel's local-private warning** (`RenderedReportPanel.tsx:46-47`):
  unchanged. After a user-released save the status line reads:
  "User-released report package saved: it contains this project's values and
  results for sharing, as you stated in the release dialog."

### 5.3 Package boundary notes (user-released packages only)

The three notes of `lib.rs:718-724` become, for a package with a record:
1. unchanged: "This container is an artifact format, not an issuance act;
   package contents are engineering decision-support information."
2. unchanged: "Human professional review is required before any reliance on
   package contents."
3. "No timestamps or generated identifiers are recorded in this package except
   user_release.released_at, the time the user released their project data;
   identical inputs, including that time, produce identical bytes."
4. added: "The user released their own project data in this package for
   sharing (user_release). The record is the user's statement about rights in
   that data; it is not an issuance act."

Note 3 is exact under R. The run record is the solve's transient record, with
`created_at: null` and the `created_at_unavailable` note
(`analysisRunCompatibility.ts:192,197`). The renderer input records no
generation time and no run recording time (§2.6). So `released_at` is the one
timestamp in the package. Packages without a record keep the three notes
exactly.

### 5.4 Claims-language check

Every proposed text in §2.2, §5.1, §5.2, §5.3 and Appendix A was run through
GEN-13 (`tools/validation/validate_claims_language.py`) at revision 3, both as
product source (`apps/desktop/src/…`) and as a project document, through the
tool's own line and retired-text scans. Result: no finding.
- No line carries three litany terms (`:44-56`). The highest count is one:
  "professional", in "Professional Boundary notice" and in note 2, which is
  unchanged text.
- No text uses the hedge phrasing that registry §1 retires, or a BS-ACCEPT or
  BS-MATURITY text (`docs/claims_registry.md` §1).
- The texts state a user's data right. They do not restate PRD §21 and add no
  prohibition list (registry §4).
- The PRD §19.3 notice and its lint anchors are untouched.

Round 1's lint-scope finding is resolved on main: #1626 changed the
ScopeOfWork glob to `execution/PKG-*/DEL-*/ScopeOfWork.md`
(`validate_claims_language.py:268`), so the full lint now scans the live
ScopeOfWork files, Appendix A's targets among them.

## 6. Verification plan

The work touches the data boundary, persistence and file output. Each slice
therefore has targeted regression tests and an independent Codex review on its
own head. Pytest runs under `t3_slot.sh` with `-n 4`, vitest with
`--maxWorkers=3`, and cargo only through the T3 wrappers.

**Round 1 to round 3 counterexamples, as named regressions.** Each is refused,
or, where stated, admitted as a control or preserved. Each refused native case
leaves the sandbox directory empty, with no picker request.
- `user_release_library_applied_bend_refused` (round 2):
  - a model whose bend has `modifiers` and a `provenance_record.library_ref`
    from a component apply is refused natively with
    `REPORT-USER-RELEASE-LIBRARY-REFERENCED`, before attestation or
    derivation;
  - the same model with the `library_ref` removed, but its
    `modifiers.source_reference` still `library:component:…#…`, is refused
    the same way;
  - the same model with both marks removed is admitted: no library origin is
    detected, and the release relies on the user's statement (the reliance
    limit, §2.7). The test pins that limit; it does not show the values are
    the user's own.
- `user_release_library_reference_sites_refused`: one case per site of
  `library_ref_sites.json`, including a blank-identity site, and a spring
  hanger applied from a library (its `library_ref` and its `provenance`
  string, `library:hanger:…`). Each is refused natively; in the webview the
  dialog's save is disabled with the same reason.
- `user_release_legacy_hanger_selection_refused` (round 3, blocking): an
  invented model whose support `provenance` is the legacy hanger selection
  evidence exactly as the pre-W-17 writer built it (the RFC 8785 text of the
  six members, from `hangerSelection.ts:35-39` at `00af9112c0^`), with no
  `library_ref` and no `library:` string, and whose solve has genuine
  `spring_hanger_user_input_review` rows. One case per recognition path, each
  refused natively with `REPORT-USER-RELEASE-LIBRARY-REFERENCED` and the rule
  `legacy_hanger_selection`, before attestation or derivation, and disabled in
  the webview:
  - the full legacy string (rules 3 (a) and 3 (b));
  - the same string with `kind` removed, so only the embedded-JSON members
    remain (3 (b)), once for each of `library_kind`, `library_id`,
    `library_metadata` and `record_snapshot` kept alone;
  - the same object with every one of those four members removed but `kind`
    kept (3 (a));
  - the full string with leading and trailing JSON whitespace;
  - an embedded object holding only a nested `library_ref` (3 (b) through
    rule 1), and one holding only a `library:hanger:…` string (rule 2);
  - the legacy string held in an array element and at a nested object member,
    not only at `supports[].provenance`;
  - a candidate beginning with `{` that the prescan refuses for nesting deeper
    than 127 containers, and one with the reserved raw-value key (unchecked,
    refused under rule 3).

  Controls, each admitted (the reliance limit, §2.7): a provenance string that
  begins with `{` but is not JSON; an embedded JSON array; an embedded object
  with none of the members and no mark; and a legacy object reduced to its
  `hanger_id` alone. A legacy string embedded inside a string of an embedded
  object is refused by 3 (a), since the substring test sees it at any depth.
  The hanger form is the only legacy form (§2.7): the material, section and
  component applies have always written rules 1 and 2's marks, which the two
  cases above cover.
- `user_release_rule_pack_reference_refused`: a model with one
  `rule_pack_refs[]` entry, with or without a rule-check run, and an input
  manifest with one active pack. Each is
  `REPORT-USER-RELEASE-RULE-PACK-REFERENCED`.
- `user_release_rule_aggregate_refused`: a run record whose status carries
  `USER_RULE_CHECKED` or `USER_RULE_FAILED` (the session's composed record) is
  `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`.
- `user_release_unknown_annotation_refused` (round 1): a source row whose
  `metadata` carries an extra key with an invented marker value, added only in
  the request, is refused by the attestation
  (`REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`). The same extra member at any
  other request pointer is refused by the derivation
  (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the pointer).
- `user_release_rule_pack_class_unknown_redistribution_redacted` (round 1): an
  object with `privacy_classification: private_rule_pack_data` and
  `redistribution_status: unknown` (and `TBD`, and null):
  - is `redact_value` / `PRIVATE_DATA_REDACTED` in `user_released_report` in
    TypeScript, Python and the Rust mirror (a corpus case), and the lossless
    route is withheld;
  - natively, is `REPORT-USER-RELEASE-PRIVATE-CLASS`.

  Its twin with `private_project_data` and unknown redistribution, without
  wrapper intent, is `block_export` / `USER_RELEASE_INTENT_REQUIRED`.
- `user_release_record_model_ref_unbound` and
  `user_release_record_report_id_unbound` (round 1): a record with only
  `model_ref`, or only `scope.report_id`, changed, every other member
  coherent. Each is refused natively (`REPORT-USER-RELEASE-UNBOUND`), by the
  producer (`REPORT-PACKAGE-USER-RELEASE-UNBOUND`) and, in a re-hashed package,
  by the reader (`OPSPROJ_USER_RELEASE_UNBOUND`).
- `user_release_extra_member_refused`: a valid released package with one of
  these added and the inventory restamped:
  - a second, valid result member;
  - a valid state/comparison/handoff member.

  The reader refuses each.
- `user_release_same_source_member_substitution_refused` (round 1): a valid
  released package whose HTML member is replaced by the HTML of another
  released save of the same model and run, with a different `released_at`.
  Its PDF is swapped the same way, and its audit member with an
  equal-identity variant. The inventory is restamped each time and the record
  left unchanged. The reader refuses each (`content_binding`; and the notice
  text for the HTML).
- `existing_contexts_preserve_raw_intent_named_members` (round 1): a request
  whose source-row `metadata` and a handoff section carry raw members named
  `user_release_intent` and `explicit_user_release_intent`, under
  `local_private` on route 009 and on `DREP-IPC-003` and `DREP-HTML-SAVE-005`.
  - The controlled payload is byte-identical to the input, the result
    document's self-hash verifies, and the exact-payload agreement holds, as at
    the parent head (a vitest and a pytest of the projector, plus a Tauri
    admission test on route 009).
  - The same members on route 010 are refused natively.

**Owner property 1: the flag cannot leak private values.**
- vitest `redactionExportControls.test.ts`:
  - for every corpus item and each of the five existing contexts,
    `explicitUserReleaseIntent: true` throws, so it cannot enable export there;
  - payload-carried `user_release_intent` and `explicit_user_release_intent`, at
    the top level and nested, grant nothing: the `user_released_report` context
    without wrapper intent blocks (`USER_RELEASE_INTENT_REQUIRED`);
  - in `user_released_report`, each non-project private class is redacted,
    with every redistribution status, and the lossless route is withheld;
  - step order: for each of steps 10-13, a case that the existing order would
    decide in an unknown branch is decided by the new step;
  - the release projector admits each derived request of the vectors with
    intent, and blocks it without.
- Corpus parity: new cases in `cases.json`, checked by the vitest corpus test,
  `tests/security/test_redaction_export_controls.py` and the Rust mirror test
  `canonical_parity_corpus_matches_rust_contract_mirror`
  (`redaction_binding.rs:702`).
- Rust, Tauri (`report_package_save.rs` tests, sandbox directory, counting
  picker): on route 010, each of the following is refused before the picker,
  and the directory stays empty:
  - a request carrying `private_material_data` at depth;
  - each reference refusal above;
  - an unattested carrier, an evicted attestation and a mode mismatch;
  - a run record that fails each step-8 check;
  - an input manifest with a non-fixed setting or an external asset;
  - a request whose `user_rule_checks` or rule-pack reference is present;
  - a request with a handoff record set;
  - each case of `user_release_legacy_hanger_selection_refused`.
- Rust, Tauri, rule 3 (b)'s lease: the ingress lease grows by the largest
  candidate's priced parse before any embedded parse, and a counting-allocator
  test over a model with several large embedded candidates shows at most one
  embedded value held at a time, within that growth.
- Rust, `report_package::user_release`: the derivation vectors pass. A
  mutation of each member of a derived request is refused, naming its pointer.
- Parity: the derivation vectors in vitest and in Rust.
- Attestation (`solve_worker.rs`, `solve_jobs.rs` and `lib.rs` tests):
  - the solved demo's `d_M` and `d_S` equal admission's digests over the
    session model and the received result, taken through the webview round
    trip;
  - the `ATST` frame's protocol violations fail the job;
  - a value the checked profile refuses sends status 0, and its result is not
    attested;
  - the table is bounded, evicts the oldest entry, is cleared by
    `dispose_all_results`, and is never written to disk;
  - the worker's counted peak with the digest passes stays within its grant
    (counting allocator), and the process-reserve measurement still holds;
  - allocation checks (counting allocator) of the streamed digest, in the
    worker and at admission, each on an invented value built to maximize one
    transient piece of §2.4: objects nested to the checked profile's depth
    limit, each with many members,
    so the key vectors on the open path are largest; one object with the most
    members, for the sort scratch; and one long string of characters that
    escape, for `write_string`; and one numeric-heavy value exercising
    exponential formatting. Each measured peak is within the §2.4 allocation
    account, including number-formatting temporaries, and within the grant or
    lease that holds it.

**Owner property 2: redaction applies to every other context.**
- The existing corpus cases are not edited. A vitest and a pytest pin the
  SHA-256 of their canonical form, so editing an existing expectation fails.
- Existing route tests for `DREP-PACKAGE-SAVE-009`, `DREP-IPC-003` and
  `DREP-HTML-SAVE-005` pass unchanged (`reportRedactionProjector.test.ts`,
  `renderedReport.test.tsx`, `reportPackageSaveService.test.ts`), and
  `existing_contexts_preserve_raw_intent_named_members` holds.
- Rust, `report_package`:
  `local_private_package_bytes_unchanged_by_user_release` assembles the
  invented local-private basis and compares its container SHA-256 with the
  value recorded from the parent head before (r2c) changes the writer.
- Renderer (`report_renderer/tests/render.rs`): a local-private and a public
  render are byte-identical to the parent head's for the same input.

**Owner property 3: the release is recorded and visible.**
- Renderer: a released input shows the banner in the HTML and in the PDF text,
  with the bound hashes and `released_at`, and the run-context rows. It has no
  local-private banner and no `content_binding` text.
- `report_package` (new `tests/user_release_package.rs`):
  - the manifest's `user_release` equals the report context's plus a
    `content_binding` that matches the four members;
  - the four boundary notes are written;
  - no handoff member is written;
  - a round trip through the reader returns `ReadReportPackage.user_release`.
- vitest:
  - the dialog's wording and its refusal line;
  - its initial focus on the checkbox, set in `useLayoutEffect`;
  - the save stays disabled until the checkbox is ticked;
  - the status line and the receipt fields after a save.

**Owner property 4 and HELP_HUMAN: no replay.** One test per row of §2.10:
- native (`report_package_save.rs` tests): another model's text; another
  result document; another input manifest; another run record; a record
  on route 009; an altered statement, standing or `not_carried`; `model_ref` or
  `report_id` alone;
- producer (`tests/user_release_package.rs`): each bound member mismatched
  alone, with every other member coherent; a record under the desktop
  local-private and runner profiles; a handoff record under the user-released
  profile;
- reader (`tests/reader.rs`):
  - the record moved into another valid package and the package re-hashed;
  - `user_release_extra_member_refused`;
  - each member swap of `user_release_same_source_member_substitution_refused`;
  - a record in a blocked package;
  - the user-released profile without a record;
  - an unreleased package without a handoff member;
  - as a documented limit, a package whose record, members,
    `content_binding` and inventory are all re-written together is accepted;
- render command (`src-tauri/src/lib.rs` tests) and renderer: a released input
  outside the package path;
- webview (`reportPackageSave.session.test.tsx`):
  - after a user-released save, the next Save Report Package… is local private
    with no record;
  - the project save payload, the model document text and the session state
    hold no trace of the selection;
  - a basis change while the dialog is open closes it.

**Also:**
- **Linter and dispositions:**
  - a released request with a quarantined provenance at depth, or a
    `protected_suspected` privacy class, is refused natively
    (`PROTECTED_CONTENT_BLOCKED`);
  - a released report whose section text trips the protected-content linter is
    refused (`REPORT-PACKAGE-PRODUCER-BLOCKED`);
  - the webview blocks the same in the new context.
- **Forged evidence:** route-010 evidence with `local_private`; route-009
  evidence over a released request; release intent false; evidence for other
  bytes; local-first evidence with another storage context.
- **Native save and read-back:** admitted released bytes written to the sandbox
  read back with `read_report_package_container`, with the record, its content
  binding and the member hashes intact.
- **Native smoke:** a user-released save on the solved demo, which carries no
  library or rule-pack reference, through `native_smoke::take_chosen_path`
  (beside the W-18 (b2) step, `native-smoke/run.mjs:93-95,191`). The harness
  checks:
  - the file's SHA-256 against the receipt;
  - the record's model hash against the session model's;
  - the HTML member's banner;
  - that the package has no handoff member.

  It adds no pinned value; `apps/desktop/native-smoke/expected.json` is not
  changed.
- **Claims lint:** `validate_claims_language.py` on each slice's head; the new
  product strings are in scanned `apps/desktop/src` files.
- **Codex scrutiny:** one independent round per slice on its implementation
  head, plus the Appendix A texts as exact text before HELP_HUMAN. Points for
  review:
  - the decision order (§3.2);
  - the completeness of the derivation and its vectors;
  - the attestation's trust, transport and bounds;
  - the library and rule-pack rules, the legacy form's recognition and the
    reliance limit;
  - the run-record checks;
  - the reader's equality-only checks and the content binding;
  - the naming rule (§0).

## 7. Decisions

**Settled.**
- **Scope (owner, 2026-10-25): R first** (§2.1). The later slice stays in v0.1
  (§10).
- **O-A (owner, 2026-10-25): V1**, with the DEC-028 reading recorded in §0 and
  applied in §4.
- **O-B (HELP_HUMAN, as recommended):** record `released_at`, and change the
  third boundary note for released packages only (§5.3).
- **O-C (HELP_HUMAN, as recommended):** the package only (§2.2).
- **O-D (HELP_HUMAN, as recommended): refuse.** Under R the refusal covers every
  model with a detected library origin, in each form §2.7 names. For values
  without one, the release relies on the user's statement (the reliance
  limit, §2.7).
- **O-E: moot under R.** R carries no rule-check identifier. The question
  returns with the later slice (§10).
- **O-F (HELP_HUMAN, as recommended): exclude.** Under R that means no
  comparison section and no handoff record set, the persistence reference
  `TBD`, and assets `[]` (§2.1).
- **O-G (owner, 2026-10-25): equality only,** stated in §2.9 and in A8.
- **V1 covers R's package form (HELP_HUMAN, 2026-10-25, §0).** The reader at
  this head accepts what the writer at this head emits, so the user-released
  package has no state/comparison/handoff member and R keeps no handoff record
  set (§2.1, §4).
- **Rule-pack-referenced models are refused under R (HELP_HUMAN, 2026-10-25,
  §0).** `REPORT-USER-RELEASE-RULE-PACK-REFERENCED` (§2.7) is sequencing within
  the ruled scope. The later slice, which stays in v0.1, lifts it (§10).
- **Wording grant.** Appendix A and the product texts of §2.2 and §5 go to
  Codex as exact text and then to HELP_HUMAN.

**Where each review finding is resolved.**
- Round 1:
  - findings 1 and 3: the derivation and the refusals (§2.6, §2.7);
  - finding 2: the decision order (§3.2);
  - finding 4: the bindings, cardinality, content binding and the narrowed
    replay claim (§2.5, §2.9, §2.10);
  - finding 5: the strip is not widened (§3.3);
  - finding 6: the notice (§5.1);
  - finding 7: A13.
- Round 2's blocker: every model with a detected library origin is refused
  (§2.7). The
  should-fix items:
  - (a) the recorded-revision alternative is removed, since O-B stands as
    recommended. That alternative conflicted with the unchanged Current-export
    and producer checks that require `state:<model>:preview`
    (`resultExportAdapter.ts:278-279`; `bindings.rs:327`). Route 010 admits
    only the transient record (§2.5 step 8), so note 3's statement holds
    (§5.3);
  - (b) the derivation wording of A1 and A7, reduced to R;
  - (c) the library-value refusal appended to A7, as the refusal of every
    model with a detected library origin;
  - (d) A9's forbidden cell, replaced as worded.
- Round 3:
  - the blocker: the legacy hanger selection evidence is recognized (§1, §2.7
    rule 3), with its regressions (§6), and the reliance limit is stated in
    §0, §2.7, the dialog (§2.2) and the notice (§5.1);
  - the two HELP_HUMAN readings are recorded as settled in §0 and stated so in
    §2.7, §4 and the settled list above; neither is open;
  - the hanger rows: "dedicated per-object hanger quantity entries" are left
    out, and the solve's review rows are retained (§2.1, §2.6, §10);
  - A5's derivation clause, replaced as worded;
  - the digest passes' transient key vectors (§2.4) and their allocation
    checks (§6);
  - A7's library clause, replaced as worded, and its detector extended to the
    legacy form.

**Open.** None. Every decision this design rests on is settled.

## 8. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(r1) Vocabulary and redaction** | The context, reason codes and route id in TypeScript, Python, the Rust mirror and the schema; the decision order of §3.2 (steps 10-13); the flag assertion; the local-first rule; the release projector (nearest-metadata labelling; unused until r3); corpus cases, the pinned-digest tests and `existing_contexts_preserve_raw_intent_named_members`. No route uses the context. | M (~650 lines with tests) | Data boundary (vocabulary only) |
| **(r2a) Solve attestation** | The worker's streamed digests, the `ATST` frame, the registry table, clearing and bounds, the memory-budget text with the digest passes' transient pieces (§2.4), its measurement and the allocation checks, tests. No rule-check attestation. | S (~450) | Data boundary; DEL-04-07 memory accounting; no priced file |
| **(r2b) Derivation and refusals** | `report_package::user_release`: Rust builders of the input manifest, the document base and origin, the minimal renderer input, and the audit and top-level members; the library rules 1-3 of §2.7, including the legacy hanger form, its embedded-JSON parse and lease, in `src-tauri` beside `export_disposition_screen`; the rule-pack and private-class rules; the run-record checks (Record Run's, factored); the derivation vectors and their vitest and Rust parity; `user_release_legacy_hanger_selection_refused`. | M (~1.05k) | **Data boundary** (what may be shared) |
| **(r2c) Record, renderer, producer, writer, reader, native** | `UserReleaseRecord`; context and blocking codes; notice and rows in HTML and PDF; producer items 8-9, the profile and the handoff rule; writer manifest, content binding, notes and member rule; reader checks, cardinality and refusal; native route-010 branch (§2.5 steps 1-10) with the run-record text; the render-command refusal; the Rust tests of §6. The desktop does not offer it yet. | M-L (~1.3k) | **Persistence** (manifest, member set and reader) and **data boundary** (native admission) |
| **(r3) Desktop** | Dialog, panel button, menu item (webview and native menu), session wiring and modal guard, the route-010 builders, the early reference check (the three rules of §2.7), the dialog's reliance line, save-service evidence and the run-record text, receipt display, vitest, native smoke step. | M (~900) | Data boundary (webview evidence) and file output |

The order is (r1), then (r2a) and (r2b) in either order or together, then
(r2c), then (r3). (r2c) needs (r2a) and (r2b) in its ancestry; (r3) offers the
user release and needs (r2c). (r2c) and (r3) may land as one PR if review
prefers. The total is about 4.35k lines (4.1k at revision 2; the legacy
detector, its regressions and the allocation checks add about 250), down from
about 5.8k for revision 1's
full scope. The savings are:
- the builder mirrors (no component, hanger or rule-pack entries, no handoff
  sections, no analysis-record rebuild);
- the closed table;
- the rule-check attestation.

## 9. Reconciliation, per slice

Each text changes in the PR of the slice that makes it true. Commitment,
acceptance and ScopeOfWork texts are applied only as Appendix A's reviewed
exact text.
- **(r1)**
  - DEL-12-02 `ScopeOfWork.md` :58, the context vocabulary (A3).
  - `docs/security/redaction_export_controls.md`:
    - the Export Contexts table (:59-65) gains the `user_released_report` row,
      with its decision order;
    - the Final-Sink Route Binding section (:158-163) says that release intent
      comes only from the wrapper, and that the strip list is unchanged.
  - DEL-12-02 `deliverable.yaml` check pointers; `tools/hosted-ci-routing.json`
    for the changed sources and the tests.
- **(r2a)**
  - The `solve_worker.rs` protocol text (:10-25) names `ATST`; the
    `solve_jobs.rs` module text (:1-41) names the attestation table.
  - DEL-04-07 `Design/memory-budget-decision.md` :37, the worker paragraph,
    names the frame, the digest passes and their transient pieces (the key
    vectors on the open path, one sort scratch, one escaped string; §2.4);
    DEL-04-07 `deliverable.yaml`
    checks; CI routing for the Tauri sources and tests.
- **(r2b)**
  - DEL-08-01 `deliverable.yaml` code paths (`user_release` module and the
    library rules) and checks (the derivation vectors, the legacy-form
    regressions); CI routing.
  - `core/library_import/README.md` :45-51 still describes the pre-W-17 hanger
    provenance as what selection persists. It is changed to name the W-17 form
    (`library_ref` and the `library:hanger:` string) as current, and the
    legacy form as one that models saved before `00af9112c0` may still carry,
    which route 010 refuses (§2.7).
- **(r2c)**
  - DEL-02-05 `ScopeOfWork.md` :214, the reader (A8).
  - DEL-12-02 REXC-OI-007 :486, the native checks (A7), and CLM-030 :414 (A5;
    see its note).
  - DEL-08-01 `deliverable.yaml` checks (new `tests/user_release_package.rs`);
    the `reader.rs` module text (:1-42), with the §2.9 statement of what the
    checks establish; CI routing.
  - DEL-08-01 Design `report-package-current-results.md` keeps its text; it
    carries only a pointer to this design.
- **(r3)**
  - DEL-08-01 `ScopeOfWork.md` :81 (A1) and OQ-08-01-003 :383 (A2).
  - DEL-12-02 REXC-REQ-016 after :189 (A4), REXC-OI-002 :481 (A6), CLM-036
    :470 (A9).
  - User guide `docs/user_guide/index.md` :91 (A10), :109-111 (A11), :393
    (A13) and :482-483 (A12).
    - The §10 sentence's "renders a report only as a local private document"
      would mislead once a shareable package exists, so A13 names the
      standalone report.
    - Its table row (:148) stays true: the package is still saved locally.
  - `docs/COMPLETENESS_MAP.md` records W-18b (R) and the later slice: gap 1
    (:94-100), FR-REP-001 (:205), the not-ready list (:461-464) and the W-18
    row (:547).
  - `apps/desktop/native-smoke/run.mjs`, the user-released step (§6).
  - DEL-08-01 `deliverable.yaml` code paths (`UserReleaseDialog.tsx`) and
    checks.
- **No change:**
  - DEL-04-08 REQ-006 (c) :219 and AC-006 :240 stay true (the same policy
    cell and producer boundary);
  - the DEL-08-02 audit manifest, and its ScopeOfWork;
  - the PRD §19.3 notice, and `docs/report_notice_template.md`;
  - no priced source (§2.4).

## 10. Later slice on the same mechanism (stays in v0.1)

The owner kept these in v0.1 (§0). Each extends R on the same context, route,
record, attestation, derivation and reader. None is designed here beyond its
constraints.
- **Rule outcomes at safe metadata.** It needs:
  - a rule-check attestation recorded by `run_rule_checks`
    (`src-tauri/src/lib.rs:2834`), of the safe projection of the pack reference
    and the outcomes;
  - the derivation's renderer input carrying that projection;
  - the run record's composed status admitted.

  It assumes W-17's per-basis shape (#1629). A check has one outcome per load
  case or combination its stress category maps, `UserRuleCheckOutcome` carries
  an optional `basis_ref` (`report_renderer/src/lib.rs:129-162`), and a check id
  repeats once per basis. The attestation and the derivation therefore key each
  outcome by (check id, basis), and bind the outcome count. The renderer's count
  note (`USER_RULE_CHECK_COUNT_SCOPE_NOTE`, `:160-162`) stays. Whether a
  released outcome may omit `basis_ref` ("load case not recorded") is that
  slice's question; the desktop always writes it
  (`renderableReportInput.ts:451-474`). O-E's question returns with it: whether
  check ids and missing-input names are safe metadata.
- **Rule-pack references at safe metadata**, with the agreement rule, which
  lifts the rule-pack-reference refusal of §2.7.
- **Library-referenced models.** It needs a rule for library values that
  genuine solver output carries (§1): refuse, or mark them, in result rows,
  annotations, diagnostics, witnesses and contract evidence. It also needs
  revision 1's source resolution for report entries.
- **Comparisons** (FR-REP-004), by native derivation from attested results (O-F's
  alternative), and the handoff record set's state/run sections.
- **The report entries R leaves out:** component and spring-hanger provenance
  entries and dedicated per-object hanger quantity entries. The solve's review
  rows, hanger rows included, are already in R (§2.6).
- **The record:** a new `scope.not_carried` value and `record_version` for each
  widening. The reader admits each version it writes.

## Appendix A. Wording-route drafts (old → new; not applied)

Each Old is copied byte-exact from its file. At revision 3 each was re-checked
with a Python `count == 1` against the branch tree after main `3027cf2f35` was
merged (`b2bc7e8a27`). Every Old still matches once, at the line given, and
none needed updating: the four target files are unchanged since revision 1's
basis `8cec2178c1`. Line numbers are where the Old begins. In an Old or New,
`\n` stands for a line break and `\"` for a quotation mark inside the quoted
text. These drafts go to Codex as exact
text, then to HELP_HUMAN under the wording grant. Nothing here is applied.

**Depends on** names what a New needs before it is applied:
- the slice that makes it true;
- the decisions it rests on, each **settled** (§7), or **none**.

Every decision these texts rest on is settled. Each New is written to be exactly
true of scope R once its slice lands.

**Changed since round 3:** A5 (its derivation clause) and A7 (its library
clause and detector). **Unchanged since round 3:** A1, A2, A3, A4, A6, A8, A9,
A10, A11, A12 and A13.

**A1. DEL-08-01 `ScopeOfWork.md` :81** (current-implementation paragraph, last sentence; slice r3; depends on: scope R, O-D, O-F, settled)
- Old: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21)."
- New: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21). Save Shareable Report Package… saves a reduced package for sharing after the user's per-report release of their own project data (route `DREP-PACKAGE-USER-RELEASE-010`, export context `user_released_report`; DEL-08-01 Design `report-user-release.md`): the release record in `report_context.user_release` and in the package manifest states the user's redistribution right and binds the model reference and document SHA-256, the run's input manifest and the result document, and the manifest copy also binds the hash of every other member; native admission requires every request member to equal its derivation from the bound model, its attested solve, the input manifest, the bound run, the host software identity and the user-release record; the package carries the rendered report, the audit manifest and one result member, and no user-rule outcome, rule-pack reference, run comparison or handoff record; a model that carries a library reference or a rule-pack reference is refused; and the rendered members carry the user-release notice in place of the local-private notice. Without that selection the package is local private as above, and the selection is not stored."

**A2. DEL-08-01 `ScopeOfWork.md` :383** (OQ-08-01-003, last cell; slice r3; depends on: scope R, settled)
- Old: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. |"
- New: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. A shared report carries private project data only through the user's per-report release of their own data, and only as values derived natively from the bound model, its solve in this session and its run (DEL-12-02 `user_released_report`; DEL-08-01 Design `report-user-release.md`). |"

**A3. DEL-12-02 `ScopeOfWork.md` :58** (CLM-003; slice r1; depends on: none)
- Old: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private` |"
- New: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private`, `user_released_report` |"

**A4. DEL-12-02 `ScopeOfWork.md` :189** (CLM-012; a row added after REXC-REQ-015; slice r3; depends on: scope R, O-D, settled)
- Old: "> | REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |"
- New: the Old line unchanged, followed by this line:
  "> | REXC-REQ-016 | A shareable report may include the user's own private project data only after the user's explicit release of that data for that one report, with the user's statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). The export context is `user_released_report`; the release is recorded in the report and its package, bound to the model and result it covers, and stated in the report's notice; it is not stored for later reports; a model that carries a library or rule-pack reference is refused, and no user-rule outcome or run comparison is carried; protected, quarantined and rejected content still blocks; every other export context is unchanged. | OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` §6; owner, 2026-10-24 (W-18 option B); owner, 2026-10-25 (W-18b scope R) | Focused tests: the release flag cannot enable export in another context; redaction is unchanged in every other context; the record is visible in the report and the package; an unchanged record with a mismatched bound model, result or package identity, or with member content other than the content it binds, is refused; a model with a library or rule-pack reference is refused. |"

**A5. DEL-12-02 `ScopeOfWork.md` :414** (CLM-030, last sentence; slice r2c; depends on: scope R, O-D, settled) — *changed since round 3*
- Note: CLM-030 states context policy, but its last clause ("the release is recorded in the report") is true only once the recording path exists, so A5 is applied in (r2c), not with the vocabulary in (r1).
- Old: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks."
- New: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks.\n> `user_released_report` carries the user's own project data in one report only\n> after the user's explicit release of it, with their statement of the\n> redistribution right, and only as values derived from the bound model, its\n> attested solve and its run; a model that carries a library or rule-pack\n> reference is refused, no user-rule outcome or run comparison is carried,\n> content outside the permitted source-bound projection is refused, protected,\n> quarantined and rejected content still blocks, and the release is recorded\n> in the report."

**A6. DEL-12-02 `ScopeOfWork.md` :481** (REXC-OI-002; slice r3; depends on: none)
- Old: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent."
- New: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent. The desktop's per-report user release (`DREP-PACKAGE-USER-RELEASE-010`, `user_released_report`) is selected in its own dialog for one save and is not stored."

**A7. DEL-12-02 `ScopeOfWork.md` :486** (REXC-OI-007; slice r2c; depends on: scope R, O-D, O-F, settled) — *changed since round 3*
- Old: "before the save dialog; caller evidence alone admits nothing."
- New: "before the save dialog; caller evidence alone admits nothing. On its user-release route (`DREP-PACKAGE-USER-RELEASE-010`) it admits the `user_released_report` context only with a release record whose bound model, input-manifest and result-document identities equal the texts and members it received, for a result this session's solver produced for that model, and only when every request member equals its own derivation from the bound model, its attested solve, the input manifest, the bound run, the host software identity and the user-release record; it refuses a record on the local-private route, a rule-pack reference, a user-rule outcome, a comparison section, a handoff record set and any request object with a non-project private class. It also refuses every model that carries a library reference, so no value with a detected library origin reaches result rows, annotations, diagnostics, witnesses or contract evidence. A library reference is a `library_ref` object at any depth; a string value that contains `library:material:`, `library:section:`, `library:component:` or `library:hanger:`; or a string value that holds the legacy hanger selection evidence, being one that contains `user_imported_hanger_selection`, or one that, trimmed of JSON whitespace, begins with `{` and parses as a JSON object that has a `library_kind`, `library_id`, `library_metadata` or `record_snapshot` member or within which a `library_ref` object or a string containing one of those five texts occurs; a string so beginning that admission cannot check is refused too. For a value without a detected library origin, the release relies on the user's statement of their redistribution right."

**A8. DEL-02-05 `ScopeOfWork.md` :214** (reader; slice r2c; depends on: O-A, O-G, settled)
- Old: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value;"
- New: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value; when the manifest records a user release, it checks the record's form, exactly one result member and no state/comparison/handoff member, the equality of its bound model reference and hash, input-manifest hash, result-document identity, report and package identities with the manifest, audit and result members, and the member hashes the record binds, and refuses a record in a blocked package or under another export profile (equality checks: they establish consistency between the record and the members, not authentic user consent or producer origin);"

**A9. DEL-12-02 `ScopeOfWork.md` :470** (CLM-036, a row added after "Local private report"; slice r3; depends on: scope R, settled)
- Old: "> | Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |"
- New: the Old line unchanged, followed by this line:
  "> | User-released report | The user's own project values in one report after the explicit per-report release, derived natively from the bound model, its attested solve and its run, recorded in the report and stated in its notice; no user-rule outcome, rule-pack reference, library reference, run comparison or handoff record | An unchanged user-release record whose bound identities or member hashes do not match this report, a selection remembered for later reports, or content outside the permitted source-bound projection |"

**A10. User guide `docs/user_guide/index.md` :91** (slice r3; depends on: none)
- Old: "- **Reports are local-private only, and pressure-free only.**"
- New: "- **Reports are pressure-free only. Shareable packages require your per-report statement of rights to share your own project data.**"

**A11. User guide `docs/user_guide/index.md` :109** (a bullet added; slice r3; depends on: scope R, O-D, O-F, settled)
- Old: "  - Save Report Package… saves a local private package for a pressure-free\n    result after its own local-private selection; exact-route results are\n    refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`."
- New: the Old bullet unchanged, followed by:
  "\n  - Save Shareable Report Package… saves a reduced package for sharing after\n    you tick, in its dialog, \"I hold the right to share this project's data,\n    and I release it in this report for sharing.\" The release covers that one\n    report. Its documents carry a \"USER-RELEASED REPORT\" notice naming the\n    model and result it covers. The package carries the model summary, load\n    cases and results of this session's solve. It carries no rule-pack\n    reference, user-rule outcome or run comparison. It needs a result solved\n    in this session. A model with a library reference, such as a material,\n    section, component or hanger applied from a library, or with a rule-pack\n    reference, is refused. The selection applies only to this save."

**A12. User guide `docs/user_guide/index.md` :482** (limitations list; slice r3; depends on: none)
- Old: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents and only for pressure-free results,"
- New: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents or shareable packages with your per-report statement of rights to share your own project data, and only for pressure-free results,"
- Note: the words are the reviewed New exactly; the line break after "local private" is the Old's own, kept so the list item's wrapping is unchanged.

**A13. User guide `docs/user_guide/index.md` :393** (§10, first sentence; slice r3; depends on: O-C, settled)
- Old: "renders a report only as a local private document"
- New: "renders the standalone report only as a local private document"
