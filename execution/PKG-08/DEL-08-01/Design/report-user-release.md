# Per-report user release of the user's own project data (W-18b)

**Basis:** `origin/main` at `8cec2178c1` (2026-10-25), after W-18a (a) #1597,
(b1) #1606 and (b2) #1616. Every file:line citation is at that head. Revision
1, after Codex round 1 (three blocking and four should-fix findings, each
resolved below). Design before implementation; nothing here is built. This
design records the mechanism by which a user can state, for one report, that
they release their own project data for sharing. It records no release, and it
records, implies or derives no acceptance, issuance, approval, compliance or
lifecycle standing (F-PIP-3, `loop/LOOP_INIT.md:68-72`).
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

**Naming rule.** The word "release" never appears alone in a product text,
identifier or record. It is always "user release", "user-released" or "release
your project data". No text uses "issued", "approved", "accepted",
"certified", "released version" or "release of SWBPIPE". The record's
`standing` member is the constant `user_statement_of_data_rights_only` (§2.2).

**What "the user's own project data" means here.** It is the content of the
model this application solved in the current session, the result its own
solver produced for that model, and the run record of that solve, as native
admission binds and derives them (§3.2). The native host cannot judge who holds
the rights to that model. That is the user's statement, which the record
carries and does not prove (§2.4).

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
    hash-bound (`report_renderer/src/lib.rs:1255-1257`). The native render
    command is a pure renderer: it receives no model, no route evidence and no
    screening (`src-tauri/src/lib.rs:1540-1566`).
  - Save Report Package… renders under `local_private` only, with
    `generatedAt: null` (`reportPackageRequest.ts:191-201`), through route
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
- **Unconstrained and opaque members.** The result member's
  `source_annotations[].metadata` copies each source row's `metadata` verbatim
  (`core/reporting/result_export/src/derivative.rs:291`); the schema leaves it
  unconstrained (`RawSourceAnnotationMetadata`, W-18a design §2.3 item 1). The
  handoff record sets are opaque `Value`s written as members
  (`core/reporting/report_package/src/lib.rs:323-335`). The typed renderer input
  (`wire.rs:275-305`) bounds which members reach the HTML and PDF, but not where
  their strings and numbers come from.
- **The rule-pack boundary is safe metadata by construction in the builder.**
  The report reads only a pack's identity, name, version, checksum, source
  notice and redistribution status (`renderableReportInput.ts:404-449`) and a
  check's id, status and missing-input names (`:451-468`; R-08-01-008,
  DEL-08-01 `ScopeOfWork.md:141`). The typed renderer input is the only path
  from the request to the rendered members (`wire.rs:275-305`), so no other
  rule-pack field reaches the HTML or PDF.
- **Library record values reach the report through spring hangers.** Applying
  a hanger record writes its load quantities and its reference strings
  (`source_reference`, `manufacturer_reference`, `load_side_review_reference`,
  `mechanics_consumption`) onto the support
  (`hanger-selection/hangerSelection.ts:41-54`). The report copies the hanger's
  load quantity into `report_sections.user_supplied_values[].quantity`
  (`renderableReportInput.ts:275-306`) and every string hanger member into the
  hanger's provenance `source_location` text (`:248-256`). Applying a component
  record copies dimensions and modifiers, which the report does not carry, and
  rewrites the component's source-reference strings to the library identity
  string (`library/libraryApply.ts:701-708`); the report's component entries
  carry only those strings, the label and the user's provenance text
  (`renderableReportInput.ts:162-176`). A model provenance record may name a
  `library_ref` (`types.ts:15-27`). Materials and sections are not in the
  report (user guide `docs/user_guide/index.md:397-400`).
- **Solve custody.** The host never materializes the solved model: the worker
  process does (`src-tauri/src/solve_jobs.rs:1-41`). The host retains the
  result bytes until the webview takes them, and releases them on disposal
  (`:668-800`); nothing native records afterwards which result was produced for
  which model. The webview's Current export requires a captured native
  invocation for the result (`hasNativeMechanicsInvocation`,
  `resultExportAdapter.ts:288,291`), which is a webview check.
- **The package and its reader.** The manifest's fields are fixed by the version
  except `package_id`, `export_profile_id`, the source references and the
  diagnostics (`lib.rs:643-734`). Its third boundary note reads "No timestamps or
  generated identifiers are recorded anywhere in this package; identical inputs
  produce identical bytes." (`lib.rs:722-724`). The reader rebuilds the
  manifest from those fields and refuses any difference (`reader.rs:221-240`).
  It does not parse the rendered HTML or PDF beyond UTF-8 and a PDF header
  (`reader.rs:891-902`), and its role forms fix member order, not member count
  (`reader.rs:766-780`), so it admits more than one result member.
- **What is missing.** A runtime way for the user to record, for one report, the
  basis `docs/IP_AND_DATA_BOUNDARY.md:99` names: "unless a user intentionally
  exports or contributes it with documented redistribution rights".

## 2. Design

### 2.1 Where the selection is made, and what the user does

**Recommendation: the report package only.** The user release is offered on
Save Report Package…'s route, and the package's HTML and PDF members carry it.
The standalone rendered report (panel HTML save and Print / PDF) is not offered
a user release in W-18b. Reasons, from current code:
- the standalone HTML save and the print view are webview sinks with no native
  admission (`RenderedReportPanel.tsx:359-383`), and the render command
  receives no model or evidence (`src-tauri/src/lib.rs:1553-1566`), so a
  standalone release would rest on caller evidence alone, which REXC-OI-007
  forbids for this class of route;
- the PDF is a derived, non-hash-bound view (`report_renderer/src/lib.rs:1255-1257`),
  so a record in it could not be bound to the bytes the user shares;
- HELP_HUMAN's ruling names the package manifest record, which only the
  package has.

A user who wants to share the document shares the `.opsproj` or its HTML or PDF
member; each member carries the notice and the bound identities (§5.1). The
alternative is decision **O-C** (§7).

**The controls.**
- In the rendered-report panel's package controls
  (`RenderedReportPanel.tsx:252-274`), a second button after
  "Save Report Package…": **"Save Shareable Report Package…"**
  (`data-testid="report-package-user-release"`), enabled under the same
  conditions as the local-private save (`:267`).
- In the File menu, after "Save Report Package…" (`App.tsx:1235`; native menu
  `src-tauri/src/lib.rs:4526`): **"Save Shareable Report Package…"**, command
  id `file.save-shareable-report-package`, disabled exactly when
  `file.save-report-package` is.
- Both open one dialog, `apps/desktop/src/features/report/UserReleaseDialog.tsx`.
  The local-private checkbox "Include known private values in this local
  package" (`:254-263`) is not read by this route and keeps its meaning.

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
> - Rule-pack references and user-rule outcomes retain safe metadata; library
>   record values are excluded.
> - It needs a result solved in this session, and it carries no run
>   comparison.
> - Content marked protected, quarantined or rejected still stops the save.
> - Nothing is remembered: your next report is local private unless you
>   choose this again.
>
> ☐ I hold the right to share this project's data, and I release it in this
> report for sharing.
>
> [Save Shareable Report Package…] [Cancel]

The second bullet holds only with §3.2 and decision **O-E**; the third holds
with the attestation of §2.3 and decision **O-F**.

The save button is disabled until the checkbox is ticked. The checkbox starts
unticked every time the dialog opens.

**What the user does:** an explicit selection (opening the dialog from its own
control) plus a statement of the redistribution right (ticking the statement),
then the save. Confirming starts the save at once; the confirmation is one
argument of that one save (§2.2, "Session").

**Exact routes stay refused.** The dialog's controls are disabled whenever
`reportPackageUnavailableReason` refuses (`reportPackageRequest.ts:59-61`), and
the save goes through the same output-policy cell and producer admission as the
local-private package (§3.4). Ruling 10A stands; W-11 is separate.

### 2.2 The record

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
| `scope.report_id` | string | `report:<analysis_run_ref>`, which is the report's `calculation_report.report_id` (`renderableReportInput.ts:591`) |
| `scope.package_id` | string | the request's `package_id`, `desktop-user-released-report-<run>` (§2.3) |
| `scope.released_data_class` | string, constant | `private_project_data` |
| `scope.safe_metadata_only` | array of strings, constant | `["private_rule_pack_data", "private_material_data", "private_component_data"]` |
| `bound_to.model_ref` | string | the model's `project.id` |
| `bound_to.model_document_sha256` | hex | SHA-256 of the RFC 8785 bytes of the session model document |
| `bound_to.analysis_run_ref` | string | the run id the report covers (`result.run_id`) |
| `bound_to.input_manifest_ref` | string | the run's input-manifest reference, `input-manifest:<model id>:<sha256>` |
| `bound_to.input_manifest_sha256` | hex | SHA-256 of the RFC 8785 bytes of the run's input manifest |
| `bound_to.result_envelope_id` | string | the result document's `result_envelope.envelope_id` |
| `bound_to.result_document_derivative_sha256` | hex | the result document's `result_envelope.reproducibility.derivative_hash.value` |
| `released_at` | string | the application clock when the user confirmed, UTC, `YYYY-MM-DDTHH:MM:SSZ` (decision **O-B**, §7) |
| `content_binding` | object, **manifest copy only** | written by the package writer, never by the caller (below) |

The free-text members are constants, so a record cannot carry any other
wording (for example a claim) in their place. `released_at` is the one
caller-supplied value that native admission cannot derive: it checks its form
only, and the record states it as the application clock's reading.

**Content binding (manifest copy only).** The writer adds `content_binding` to
the manifest's copy of the record after the members are materialized:
`{ "algorithm": "sha256", "members": [{ "file_name", "role", "sha256" }, …] }`,
one entry per non-manifest member (HTML, PDF, audit manifest, result member,
handoff record set) in archive order, each the SHA-256 of that member's exact
bytes. The renderer never sees `content_binding`: the request's
`report_context.user_release` must not carry one (`deny_unknown_fields`), and
the notice and rows (§5.1) render the other members only. So the rendered bytes
do not depend on it, and binding their hashes is not circular. The record then
covers the report content, excluding only the binding itself: the caller's
record (rendered and in the manifest) plus the hash of every member it covers.

**The three hashes, precisely.**
- **Model document SHA-256** (`model_document_sha256`): SHA-256 over the RFC
  8785 (JCS) bytes of the session model, as `canonicalSha256Hex(model)`
  computes it (`reportPackageRequest.ts:209`). It is the same value as the
  request's `audit_manifest.model_hash.value` (`:227-233`), the result
  document's `reproducibility.model_hash.value` (bound equal by
  `bindings.rs`, W-18a design §2.3 item 5) and the native SHA-256 of
  `modelCanonicalText` (`report_package_save.rs:613`).
- **Result-set identity** (`result_envelope_id` and
  `result_document_derivative_sha256`): the DEL-08-04 0.3.0 document's
  envelope id and its derivative hash, SHA-256 over the RFC 8785 bytes of the
  document without its own hash (`core/reporting/result_export/src/derivative.rs:760-773`;
  desktop `resultExportAdapter.ts:150`). The producer verifies that hash
  (`document_form::check_self_hash`, `document_form.rs:112`) and the reader
  re-verifies it for every result member (`reader.rs:927-940`). The
  derivative hash, not the member's byte hash, is chosen because it is the
  document's own recorded identity and is known before the report is built;
  `content_binding` records the member's byte hash.
- **Audit-manifest input-manifest hash** (`input_manifest_sha256`): the
  request's `audit_manifest.input_manifest_hash.value`
  (`reportPackageRequest.ts:234-240`), which native admission binds to the
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
`handleSaveReportPackage({ userRelease: { releasedAt } })`. The save's basis
(`workspaceSession.ts:2847-2860`) gains `userRelease`, and the builder computes
every bound value from that same basis (§2.3). Nothing is written to session
state that outlives the save, to the SQLite project, to the model document, or
to any session snapshot. The dialog's open flag joins the modal guards of
`runMenuCommand` (`workspaceSession.ts:2948`). If any basis member changes while
the dialog is open, the dialog closes, and the next attempt needs a new
confirmation.

### 2.3 How the record and the content are built and checked natively

**Webview (an early check; native admission decides).**
- `buildReportPackageRequest` (`reportPackageRequest.ts:158-255`) takes
  `userRelease: { releasedAt: string } | null`. With a value it:
  - sets `package_id` to `desktop-user-released-report-<run>` and
    `export_profile_id` to `desktop_user_released_report_package_1`;
  - renders under `{ exportContext: "user_released_report", generatedAt: null }`;
  - passes `projectSummary: null`, so the report's persistence reference is
    the builder's existing `TBD` (`renderableReportInput.ts:595-598`), and
    `comparison: null`, so the handoff set carries no comparison section
    (`stateComparisonHandoffSections.ts:214`; decision **O-F**);
  - builds the record from the same `document`, `modelHash` and
    `inputManifest` it already uses (`:175,209,215`).
- The request is controlled by a new projector,
  `controlUserReleasedReportPackageRequest(payload, model, explicitUserReleaseIntent)`,
  under route `DREP-PACKAGE-USER-RELEASE-010`, context `user_released_report`,
  lossless. It labels each leaf from the release projection table, not from
  path tokens (§3.2), so the webview applies the same closed member set and the
  same library rule as native admission, early.
- The save service sends the same three canonical texts plus a fourth,
  `analysisRunCanonicalText` (the run's DEL-14-02 0.3.0 analysis record, which
  the local-private route does not send). The screening evidence names route
  010, context `user_released_report`, `explicit_local_private_intent: false`
  and `explicit_user_release_intent: true`.

**Native attestation of the solve and the rule check (new).** Native admission
cannot derive a result value from a caller's carrier: the carrier is supplied
by the webview, and the existing bindings establish consistency between
members, not that a solver produced them. So the host records what its own
solver and rule checker produced:
- **Solve attestation.** The solve worker (`solve_worker`), which already
  materializes the model and builds the result, also computes the SHA-256 of
  the RFC 8785 bytes of the model it solved and of the result value it
  serializes, and returns both with the result. On publication
  (`solve_jobs.rs:668`) the registry records
  `{ job_id, solver_mode, model_jcs_sha256, result_jcs_sha256 }` in a bounded
  session table of the `MAX_ATTESTED_SOLVES` (32) most recent solves. The host
  process still never materializes the model; the two canonical serializations
  are priced inside the worker's grant (DEL-04-07 memory-budget decision).
  `dispose_all_results` (`:784`, page load) clears the table; it is never
  persisted.
- **Rule-check attestation.** `run_rule_checks` (`src-tauri/src/lib.rs:2833`)
  already holds the pack document, the model and the solved envelope as parsed
  values. After the run it computes the safe projection of the pack reference
  and outcomes with a Rust mirror of `rulePackReference` and `userRuleChecks`
  (`renderableReportInput.ts:404-468`), and records
  `{ model_jcs_sha256, envelope_jcs_sha256, projection_jcs_sha256 }` in a
  bounded table of the same size, cleared with the solve table. The canonical
  copies are priced by the command's ingress lease (counting-allocator tests).

A result saved in an earlier session, or one whose attestation has been
evicted, cannot be user-released until it is solved again. The Current export
already requires an in-session native invocation in the webview (§1), so the
user-visible condition is the same; native admission now enforces it.

**Native admission** (`admit_report_package`, `report_package_save.rs:591-662`).
The command stays `save_report_package`; the route id in the evidence selects
the branch. Every existing check runs on both routes in its current order.
Changes:
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
2. **Context** (`check_context`, `:451-491`).
   - Route 009: unchanged, plus `/report/report_context/user_release` absent,
     else `REPORT-USER-RELEASE-NOT-THIS-ROUTE`.
   - Route 010: `export_context` `user_released_report`, profile
     `desktop_user_released_report_package_1`
     (`REPORT-PACKAGE-SAVE-PROFILE-NOT-DESKTOP` otherwise), and the record
     present (`REPORT-USER-RELEASE-RECORD-INVALID` otherwise).
3. **Record form** (route 010). Exact members and constants as §2.2, no
   `content_binding`, hex form and the `released_at` form, else
   `REPORT-USER-RELEASE-RECORD-INVALID`.
4. **Record bindings** (route 010), each against a value admission itself
   computed or already bound, never against another caller field alone. Any
   inequality is `REPORT-USER-RELEASE-UNBOUND`, naming the member:
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
     verifies that hash over the document, `document_form::check_self_hash`);
   - `scope.report_id` = `report:` + `analysis_run_ref` =
     `/report/calculation_report/report_id`, and `scope.package_id` =
     `/package_id` = `desktop-user-released-report-<run>`.
5. **Attested sources** (route 010).
   - The solve table holds an entry with `model_jcs_sha256` = `model_hex` and
     `result_jcs_sha256` = `digest(/source_results/0)`, else
     `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`. Its `solver_mode` is the
     attested mode.
   - When the request carries `user_rule_checks`, the rule-check table holds an
     entry with that model hash, that carrier digest and
     `projection_jcs_sha256` = the digest of
     `{ /report/calculation_report/rule_pack_refs/0, /report/user_rule_checks }`,
     else `REPORT-USER-RELEASE-RULE-CHECK-NOT-ATTESTED`.
   - The analysis-run text is canonical, within `MAX_ANALYSIS_RUN_TEXT_BYTES`
     (one model-document cap), priced as the input-manifest text is, and is the
     record the request names: its record hash, recomputed by the
     `analysis_record_projection` rule (`core/analysis_runs/compatibility.py:73-80`),
     equals the `analysis_run_record` run-hash value in the document, the
     report's `audit_manifest_refs[0]` and the handoff record, else
     `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`.
6. **The release projection** (route 010; §3.2). Admission derives the expected
   request from the bound basis and requires canonical equality, applies the
   projection table's closed member set, and applies the library and private
   class rules: `REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`,
   `REPORT-USER-RELEASE-VALUE-UNBOUND`, `REPORT-USER-RELEASE-SOURCE-MISSING`,
   `REPORT-USER-RELEASE-SOURCE-AMBIGUOUS`,
   `REPORT-USER-RELEASE-SOURCE-SUBSTITUTED`,
   `REPORT-USER-RELEASE-LIBRARY-VALUE`, `REPORT-USER-RELEASE-PRIVATE-CLASS`,
   `REPORT-USER-RELEASE-RULE-PACK-DETAIL` and
   `REPORT-USER-RELEASE-COMPARISON-EXCLUDED`.
7. The disposition screens (`screen_content`, `:412-448`), the rule-pack
   agreement (`:572-588`) and the shared producer run unchanged on both routes.

**Shared producer** (`report_package::bindings::check_request`,
`bindings.rs:180-496`). A new item 9, after item 8 (`:491-494`):
- the profile table (`:213-217`) gains
  `(USER_RELEASED_DESKTOP_PROFILE, DESKTOP_ORIGIN) => true`;
- item 8 becomes: the local-private profiles (desktop and runner) require
  `export_context` `local_private` and no record
  (`REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` if a record is present); the
  user-released profile requires `user_released_report` and a record;
- item 9 checks the record against the request's members, each inequality
  `REPORT-PACKAGE-USER-RELEASE-UNBOUND`:
  - `model_ref` = `source_model_ref.ref_id` = the document's
    `model_ref.ref_id` = `audit_manifest.model_hash.payload_ref`;
  - `model_document_sha256` = `audit_manifest.model_hash.value` = the
    document's model hash;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit
    input-manifest hash value and payload reference;
  - `analysis_run_ref` = the document's `run_ref.ref_id`;
  - the envelope id and derivative hash = the document's;
  - `scope.report_id` = `report:<analysis_run_ref>` = the report's
    `report_id`, and `scope.package_id` = the request's;
  - the handoff record set has no comparison or handoff section.

The headless runner keeps its profile (`bindings.rs:36`), so a runner request
with a record is refused. The runner gains no flag.

**Writer.** `ReportPackageContainerInput` (`lib.rs:116-136`) and
`ManifestFields` (`lib.rs:643-650`) gain `user_release: Option<&Value>`.
`package_manifest_document` (`lib.rs:652-734`) writes `"user_release"` only when
present: the caller's record plus `content_binding` over the materialized
members (`lib.rs:304-335`), and then uses the user-released boundary notes
(§5.3).

**Render command.** `render_calculation_report_value`
(`src-tauri/src/lib.rs:1553-1566`) refuses an input whose
`report_context.export_context` is `user_released_report`, with
`RENDER-USER-RELEASE-PACKAGE-ONLY`, so a user-released document is produced
only inside the package admission. The webview already refuses that context on
this route (`reportRenderService.ts:124-140`).

### 2.4 How the reader verifies it (DEL-02-05)

`read_report_package_container` (`reader.rs:161-263`):
- `manifest_fields` (`:683-721`) reads an optional `user_release` and passes
  it to the rebuild (`:225-240`), so a record outside the defined form fails the
  existing manifest-form check.
- After `verify_members` (`:807-969`), when a record is present, each failure
  `OPSPROJ_USER_RELEASE_UNBOUND` with its reason:
  - the package is unblocked, and `export_profile_id` is
    `desktop_user_released_report_package_1`;
  - **cardinality:** exactly one HTML, one PDF, one audit and one result
    member, and exactly one handoff member whose `sections` hold the two
    current state/run sections and no comparison or handoff section;
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

**What these checks establish, and what they do not.** They are equality
checks, not signatures (`reader.rs:30-41`). They establish that the record,
the members and the member bytes are consistent with each other. At save time,
native admission also establishes that the record came through route 010 with
the wrapper's release intent, over a model and a result this session's solver
produced, with content derived from them. They do not establish that a person
made the statement (authentic user consent), that the package came from this
application (producer origin), or that the statement is true. Anyone who can
write the file can build a coherent package with a record, its members and
every hash re-computed together, and the reader will not detect it. The module
text (`reader.rs:1-42`) is updated to say so. The guarantee is decision
**O-G**.

### 2.5 No replay

The claim is narrow: **an unchanged record** cannot be carried onto another
model, result set, report, package or member content. A record that is
re-written to match its new place is not detected (§2.4).

| Attempt | Refused by | Code |
|---|---|---|
| Record copied into a request for another model | native admission (model text SHA-256, `model_ref`), then the producer (audit and document model hash and reference) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-PACKAGE-USER-RELEASE-UNBOUND` |
| Same model, another solve or result document | native admission (run, envelope id, derivative hash, attestation); the producer | as above; `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED` |
| Same result, another input manifest or run record | native admission (input-manifest text SHA-256 and reference; analysis-record hash) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND` |
| Record with only `model_ref` or only `scope.report_id` changed | native admission; the producer; the reader | as above; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed on the local-private route 009 | native admission; the producer (local-private profile) | `REPORT-USER-RELEASE-NOT-THIS-ROUTE`; `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` |
| Manifest record copied into another package, re-hashed | the reader (bound identities, `content_binding`) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Same package, HTML, PDF, audit or handoff member swapped and the inventory restamped, record unchanged | the reader (`content_binding`; the notice text in the HTML) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| A second result member added, inventory restamped | the reader (cardinality) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed in a standalone render | webview route check; the render command; the renderer (record outside its context) | `REPORT-REDACTION-CONTEXT-MISMATCH`; `RENDER-USER-RELEASE-PACKAGE-ONLY`; `REPORT-CONTEXT-USER-RELEASE-MISMATCH` |
| A later save reusing an earlier selection | nothing to reuse: the selection is not stored (§2.2) | none needed |
| A record whose statement or standing text is altered | native form check; the reader | `REPORT-USER-RELEASE-RECORD-INVALID`; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record, members, `content_binding` and inventory all re-written together | not detected (equality checks, §2.4) | none |

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

### 3.2 The release projection

The local-private package admits any content the request carries, because it
never leaves the user. A shareable package cannot: an unconstrained annotation
or an opaque handoff record can carry an unlabelled private rule value or
another project's data, and neither a warning nor the disposition screen shows
where a value came from (Codex round 1). So route 010 replaces the blanket
treatment of unknown values with a **source-bound projection across every
emitted member**: each value is shown, by native derivation or verification,
to be the user's own project data or safe metadata, and anything else is
refused.

**The bound basis.** Native admission holds, for route 010:
- **M**, the model text (SHA-256-bound and screened, `report_package_save.rs:613,412-448`);
- **S**, the source carrier, attested for M by the solve table (§2.3 item 5),
  with its attested solver mode **m**;
- **I**, the input-manifest text (bound, `:624-652`);
- **R**, the analysis-run text (bound by its record hash, §2.3 item 5);
- **K**, the attested rule-check projection, when a rule check is present;
- the host's own software identity (`app.package_info()`, W-18a design §2.6
  item 5) and the record **U**.

**Derivation.** A new module, `report_package::user_release`, computes the
expected request `E(M, S, m, I, R, K, U)` with Rust mirrors of the desktop
builders that produce it, for the package path only:
- the input manifest, as `buildCurrentSessionInputManifest` builds it
  (`workspaceSession.ts:1014-1027`): its `solver_basis` from S's producer and
  m with the fixed settings for m, `external_assets: []`, `model_basis` equal
  to M, and each `active_rule_packs` entry equal to an entry of M's
  `rule_pack_refs` by id, version and checksum. E requires I to be canonically
  this manifest (the store state at solve time is not re-read);
- the analysis record, as `analysisRunCompatibility.ts:192-197` builds the
  transient record from M, S and I (its excluded rule-pack diagnostics by the
  `rulePackRefDiagnostic` template, `rulePackRefDiagnostics.ts:75-88`, which
  the Python `compatibility.py` already mirrors), or as
  `recordedAnalysisRunRevision` (`:304-315`) revises it, with `created_at` in
  RFC 3339 UTC form. E requires R to be canonically one of them;
- the result document, as `buildCurrentResultExport` builds it
  (`resultExportAdapter.ts:248-293`): `currentResultDocumentBase` from M, S, R
  and I, `currentReceivedOrigin`, then the existing Rust `derive_document`
  (`derivative.rs:120`);
- the renderer input, as `buildRenderableReportInput` builds it
  (`renderableReportInput.ts:470-682`) with `projectSummary: null`, no
  numerical health, K's projection, and `report_context` with the host's
  software identity and U;
- the audit manifest and the top-level members, as
  `buildReportPackageRequest` builds them (`reportPackageRequest.ts:212-255`);
- the handoff record set, as `buildStateComparisonHandoffSections` builds it
  (`stateComparisonHandoffSections.ts:197-262`) with `comparison: null`.

Admission requires `JCS(request) == JCS(E)` and, on a difference, refuses with
`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the first differing JSON
pointer. Report entries keyed by a model object are compared entry by entry
first, so that a source fault is named precisely (below). Each mirror is held
equal to its TypeScript builder by committed invented vectors,
`fixtures/reports/invented/user_release_projection_vectors.json` (inputs and
expected outputs), read by a vitest and by a Rust test, both routed; the
analysis-record vectors are also read by a pytest against `compatibility.py`.
A change to a builder changes its mirror and the vectors together, as W-18a's
row parity does (W-18a design §2.3 item 6).

**The projection table.** Derivation proves where each value came from; a
closed table bounds which members may exist at all. The table lists every JSON
pointer pattern of a route-010 request (array indices as `*`) with its class:
- `constant`: a value the builders fix (schema versions, deliverable ids,
  boundary flags and texts, policy strings, `TBD` and "not recorded");
- `bound_identity`: an identifier or hash record that names M, S, R, I, the
  run, a row, a state, a load case or a model object (64-hex values,
  references, ids);
- `project_value`: a value taken from M (project name and units; load-case
  labels and kinds; component and support labels, provenance text and source
  references; hanger load quantities; model diagnostics), from S (row values,
  units, the producer's row metadata keys for preview-physics-2, its
  diagnostics), from R (status tokens, `created_at`) or from I (solver mode and
  settings);
- `rule_pack_safe_metadata`: a pack reference's id, name, version, checksum,
  source notice, redistribution status, completeness status and
  `private_payload_redacted: true`; each user-rule check's id, status and
  missing-input names (decision **O-E**);
- `release_record`: the record U.

A leaf whose pointer matches no pattern, or a metadata key outside the
producer's vocabulary, is unbound: `REPORT-USER-RELEASE-VALUE-UNBOUND`
natively, `USER_RELEASE_VALUE_UNBOUND` in the webview. A rule-pack reference
with `private_payload_redacted` other than `true` or a member outside its
pattern set is `REPORT-USER-RELEASE-RULE-PACK-DETAIL`. The table is one
committed file, `fixtures/reports/user_release_projection_table.json`,
embedded by the Rust admission (`include_str!`) and imported by the webview
projector, with a test on each side that pins its SHA-256.

**Per emitted member.**

| Member | What a released package may carry | How it is shown to be the user's own data or safe metadata | Refused or excluded |
|---|---|---|---|
| Result member (`result_export_<envelope>.json`) | The DEL-08-04 0.3.0 document: values, review evidence, disclosures, source annotations (row metadata as S carries it), row accounting, witnesses, producer, numerical quality, formulation basis and contract evidence from S; the base's constants, identities, run hashes and diagnostics from M, R and I | Native derivation: `derive_document` over the derived base, M and S, with S attested for M; equality with the request's member; the table bounds annotation metadata to the producer's keys | Any difference; an annotation key or value outside the table; S not attested |
| Rendered HTML and PDF (the renderer input) | Report title, model summary, units, load cases, component and spring-hanger provenance entries, hanger load quantities, result rows, diagnostics, status disclosures, limitations, the run context (software, solver, mode, settings, run time), rule-pack safe metadata and user-rule outcomes, the record | Native derivation of the renderer input from M, S, R, I, K and U, then the producer's own render of that input; the linter runs before and after (§3.4) | A library record value (below); a missing, ambiguous or substituted source; the local persistence reference (excluded: always `TBD`); numerical-health rows (none on the package path) |
| Audit manifest | Its id, the model and input-manifest hash records, solver identity, unit-system reference, the rule-pack safe projection, `assets: []`, the boundary constants | Native derivation from the derived document, I and K | Any asset; any difference |
| Handoff record set | The two current sections: the model-state and analysis-run sections from M, S and R | Native derivation with `comparison: null` | Any comparison or handoff section (`REPORT-USER-RELEASE-COMPARISON-EXCLUDED`; decision **O-F**) |
| Assets | None | The audit manifest's `assets` is the constant `[]` | Any asset |
| Boundary notes | The four notes of §5.3 | Writer constants | None |
| Package manifest | The fixed fields; `package_id`, profile and source references derived from the run; the producer's diagnostics; the record U with `content_binding` | Derived by the writer from E and the materialized members | None beyond the members' rules |

M, S, I and R themselves are validation inputs only; none is written to the
package (W-18a design §2.2, "Not a member").

**Source resolution, and the library rule (Codex round 1, finding 3).** The
original design trusted an entry's `source.ref_id` and refused only when the
named object carried a `library_ref`, so an entry could keep a library quantity
and name another object. Now each `report_sections.user_supplied_values[]` and
`provenance_notes[]` entry, and each `load_case_summary[]` entry, is derived
from one model object, and is checked against that derivation before the whole
comparison:
- **missing:** an expected entry is absent, or an entry's `source` resolves to
  no model object of its kind (`REPORT-USER-RELEASE-SOURCE-MISSING`);
- **ambiguous:** the model holds more than one object with the source's kind
  and id (`REPORT-USER-RELEASE-SOURCE-AMBIGUOUS`);
- **substituted:** the entry's `value_id` or `source` names one object while
  its quantity, label or provenance text is another object's, or is not the
  value the builder takes from that object
  (`REPORT-USER-RELEASE-SOURCE-SUBSTITUTED`);
- an entry with no expected counterpart is unbound (`…-VALUE-UNBOUND`).

The derivation keeps each source object's own standing attached to the values
it takes from that object, rather than the builder's blanket
`private_project_data` label (`renderableReportInput.ts:178-194`). For a
support whose `provenance_record.library_ref` names a hanger record, the
hanger's load quantity and its hanger reference strings are library record
values (§1). If the derived report would carry either, a non-null
`user_supplied_values[].quantity` or a `hanger.<member>=` part of the hanger's
`source_location`, the save is refused with `REPORT-USER-RELEASE-LIBRARY-VALUE`,
naming the support and the library. Since the builder projects both for every
spring hanger, a model with a spring hanger applied from a library cannot be
user-released while decision **O-D** stands at refusal. What stays: a
`library_ref` itself (kind, id, version, record id) and the library identity
string the apply writes, which are identity. Component entries carry no library
record value (§1), so library-applied components, materials and sections do
not stop a release. Results computed from library values are the solve's
results and are released as such.

The private-class rule stays as defence: any object in the request whose own
`privacy_classification` or `privacy_class` is a non-project private class
(`private_material_data`, `private_component_data`, `private_rule_pack_data`,
`owner_standard_data`, `company_design_basis_data`, `path_data`,
`secret_like_data`) is `REPORT-USER-RELEASE-PRIVATE-CLASS`. A label can add a
refusal; no label grants inclusion.

**The decision rule, in exact order (Codex round 1, finding 2).** `actionFor`
(`redactionExportControls.ts:417-479`) and `_action_for`
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

10. privacy in the non-project private set above, whatever the redistribution
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

The release projector labels each leaf from the table: `project_value` and
`release_record` as `private_project_data` / `private_only`;
`bound_identity`, `constant` and `rule_pack_safe_metadata` as
`public_metadata` / `public_permissive`; a library record value of a
library-referenced support as `private_component_data` / `private_only`; and an
unmatched leaf with no metadata. So step 10 withholds library values, step 11
admits the user's values only with intent, and step 13 refuses unbound
content, in the webview before native admission repeats each rule.

**What a shared report contains, and what is excluded or refused.** A
user-released package carries: the project name, id and units; the load cases;
the component and spring-hanger provenance entries and hanger load quantities
of objects not applied from a library; the result rows, diagnostics and status
of the attested solve; the solver identity, mode and settings; the rule-pack
reference's safe metadata and the user-rule outcomes of the attested rule check
(decision **O-E**); the derived result document, audit manifest and current
state/run sections; and the record and notice. It excludes: run comparisons,
the local storage reference, audit assets, private rule-pack payloads and
library record values. It refuses the save when: the model holds a spring
hanger applied from a library (decision **O-D**); the result or the rule check
was not produced in this session (or its attestation was evicted); any member
differs from its derivation; or any value is unbound.

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
  key lists. Release intent is read only from the wrapper's argument, so a
  payload member named `user_release_intent` or `explicit_user_release_intent`
  grants nothing and needs no strip. In the five existing contexts and on
  routes 009, `DREP-IPC-003` and `DREP-HTML-SAVE-005`, such a member is raw
  content, preserved byte for byte as today, so a result document's
  self-hash and the exact-payload agreement are unaffected. On route 010 it
  matches no table pattern and is refused as unbound.
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
  (`reportPackageUnavailableReason`, `reportPackageRequest.ts:59-61`), the same
  builder admission (`:160-181`) and the same producer preflight
  (`wire.rs:281-283`). Exact routes stay refused with
  `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` and `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`
  (ruling 10A).
- **Protected-content linter.** The renderer runs it before and after rendering
  in every context (`report_renderer/src/lib.rs:1180-1224,1263-1270`); a blocking finding
  sets `export_blocked`, and admission refuses a blocked package
  (`REPORT-PACKAGE-PRODUCER-BLOCKED`, `report_package_save.rs:658-660`).
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

**The package (`.opsproj`).** The member set and the result, audit and handoff
member forms are unchanged. Two things change for a user-released package only:
- the manifest gains the optional `user_release` member (§2.2), with its
  `content_binding`;
- the manifest's boundary notes become the user-released set (§5.3).

The rendered HTML and PDF members of a user-released package carry the notice
and context rows (§5). A package without a user release is byte-identical to
today's. Version handling is decision **O-A**:
- **V1 (recommended): stay at 2.0.0.** This needs an explicit interpretation
  of DEC-028: that "one version" admits an optional, writer-defined record
  within that version, so that the reader at a head accepts exactly what the
  writer at that head emits. DEC-028 alone does not establish that per-head
  reading. Every existing package, fixture and witness is unchanged, which is
  the most literal reading of "without the selection, the report stays
  local-private exactly as today" (unreleased bytes stay identical because the
  intent strip is not widened, §3.3). Consequence: a reader built before W-18b
  refuses a user-released package as `OPSPROJ_MANIFEST_MALFORMED`
  ("manifest differs from the version 2.0.0 manifest form at: user_release",
  `reader.rs:236-240`), not by version name. No such reader has been released.
- **V2: move to 3.0.0.** The precedent is W-18a, which moved the version when a
  member form changed (W-18a design §2.2). Consequence: the reader refuses
  every existing 2.0.0 package by name (`OPSPROJ_PACKAGE_VERSION_OLDER`),
  including local-private packages saved since #1616; every package's bytes
  change (its version string); the reader tests, the container tests and the
  headless runner's regenerated failure witnesses move with it.

**Rendered reports.** No JSON schema file describes the renderer input; its
contract is the Rust type (`report_renderer/src/lib.rs:154-163`) and the
TypeScript builder. Changes:
- `ReportExportContext` (`:80-83`) gains `UserReleasedReport`
  (`user_released_report`); `export_context_label` (`:335-340`) gains it;
- `ReportContext` gains `user_release` (§2.2), defaulting to `None`;
- `UserReleaseRecord` is a new typed struct with `deny_unknown_fields` and no
  `content_binding` member;
- `report_context_blocking_codes` (`:1147-1168`) adds
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
- Unchanged: the headless runner inputs and witnesses
  (`validation/witness/inputs/del1005_*`, `validation/witness/generated/del1005_*`),
  every existing reader and container fixture, the 23 existing corpus cases,
  and the owner-held `apps/desktop/native-smoke/expected.json` (not touched).
- New, invented: a user-released profile in the shared basis builder
  (`core/reporting/report_package/tests/common/package_basis.rs`); a committed
  record for that basis, `fixtures/reports/invented/user_release_record.json`;
  the projection table and the projection vectors (§3.2); new corpus cases in
  `fixtures/redaction_export_controls/cases.json`, each with
  `explicit_user_release_intent`.
- Under V2, as listed under V2 above.

## 5. The notice

### 5.1 Notice text (proposed exact)

The renderer gains `user_release_notice_text(input) -> Option<String>`, used by
the HTML body (beside `local-private-banner`, `report_renderer/src/lib.rs:1237-1242`)
and the PDF (`pdf_emitter/src/lib.rs:236`), and by the reader's notice check
(§2.4). It is emitted only for `user_released_report` with a record, in a
banner of its own,
`<div class="user-release-banner" data-export-context="user_released_report">`:

> USER-RELEASED REPORT — the user released their own project data in this
> report for sharing, stating that they hold the right to redistribute it
> (IP and data boundary §6). The release covers this report only: model
> SHA-256 {model_document_sha256}, result document {result_envelope_id}
> (SHA-256 {result_document_derivative_sha256}), released {released_at}.
> Rule-pack references and user-rule outcomes retain safe metadata; library
> record values are excluded. This statement concerns rights in the data only;
> the analysis status and the Professional Boundary notice below are unchanged.

The sentence on rule packs and libraries holds only with §3.2 and decision
**O-E**.

The run-context table (`run_context_rows`, `:342`) shows "Report export
context: user_released_report" and three added rows: "User release statement"
(the statement), "User release time (application clock)" (`released_at`) and
"User release bound to" (the model, run, input-manifest and result-document
identities). `content_binding` is not rendered.

### 5.2 Relation to the other notices

- **PRD §19.3** (`docs/PRD.md:1290-1296`; renderer `:972-1000`): unchanged, in
  every report, user-released or not. A user release changes no analysis status
  and adds no review record.
- **The local-private notice** (W-3, `local_private_notice_text`,
  `report_renderer/src/lib.rs:1129-1143`): unchanged and still emitted for
  every `local_private` report. A user-released report carries the user-release
  notice in its place, because the two contexts exclude each other.
- **The panel's local-private warning** (`RenderedReportPanel.tsx:46-47`):
  unchanged. After a user-released save the status line reads:
  "User-released report package saved: it contains this project's values and
  results for sharing, as you stated in the release dialog."

### 5.3 Package boundary notes (user-released packages only)

The three notes of `lib.rs:718-726` become, for a package with a record:
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

Packages without a record keep the three notes exactly.

### 5.4 Claims-language check

Every proposed text in §2.1, §5.1, §5.2, §5.3 and Appendix A was run through
the GEN-13 line scanner (`tools/validation/validate_claims_language.py`,
`_scan_file` and `_scan_retired_texts`), both as product source
(`apps/desktop/src/…`) and as a project document, at revision 1. Result: no
finding. No line carries three litany terms (`:44-56`); the highest count is one
("professional", in "Professional Boundary notice" and in note 2, which is
unchanged text). No text uses the hedge phrasing that registry §1 retires, or a
BS-ACCEPT or BS-MATURITY text (`docs/claims_registry.md` §1). The texts state a user's data
right. They do not restate PRD §21 and add no prohibition list (registry §4).
The PRD §19.3 notice and its lint anchors are untouched (`:114-126`).

**Finding (lint scope).** The lint's ScopeOfWork glob is
`execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md` (`:262-271`). The live files
are at `execution/PKG-*/DEL-*/ScopeOfWork.md` (107 files), so today the lint
scans no ScopeOfWork. Appendix A's texts were checked directly, as above. The
glob is outside this design; it is reported for the owner of GEN-13.

## 6. Verification plan

The work touches the data boundary, persistence and file output, so each slice
has targeted regression tests and an independent Codex review on its own head.
Pytest runs under `t3_slot.sh` with `-n 4`; vitest with `--maxWorkers=3`;
cargo only through the T3 wrappers.

**Codex round 1 counterexamples, as named regressions.** Each is refused, or
for the last one preserved, as stated, and each native case leaves the sandbox
directory empty with no picker request:
- `user_release_unknown_annotation_refused`: a source row whose `metadata`
  carries an extra key with an invented marker value. In the webview, the leaf
  matches no table pattern and step 13 blocks it (`USER_RELEASE_VALUE_UNBOUND`).
  Natively, with the extra key also in the attested carrier, the table refuses
  it (`REPORT-USER-RELEASE-VALUE-UNBOUND`); with the key added only in the
  request, the attestation refuses it (`REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`).
  A same-shaped extra member in a handoff section is
  `REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`.
- `user_release_rule_pack_class_unknown_redistribution_redacted`: an object
  with `privacy_classification: private_rule_pack_data` and
  `redistribution_status: unknown` (and `TBD`, and null) is `redact_value` /
  `PRIVATE_DATA_REDACTED` in `user_released_report` in TypeScript, Python and
  the Rust mirror (corpus case), and the lossless route is withheld; natively
  `REPORT-USER-RELEASE-PRIVATE-CLASS`. Its twin with `private_project_data`
  and unknown redistribution, without wrapper intent, is `block_export` /
  `USER_RELEASE_INTENT_REQUIRED`.
- `user_release_library_value_substituted_source_refused`: a library-applied
  spring hanger's `user_supplied_values` entry keeps its quantity while its
  `source.ref_id` is changed to (a) another support, (b) a component, (c) an
  id the model does not hold, (d) an id two supports share. The codes are
  `…-SOURCE-SUBSTITUTED`, `…-SOURCE-SUBSTITUTED`, `…-SOURCE-MISSING` and
  `…-SOURCE-AMBIGUOUS`; the unmodified entry is `…-LIBRARY-VALUE`, and so is
  the same hanger with only its `hanger.source_reference` text in
  `source_location`.
- `user_release_record_model_ref_unbound` and
  `user_release_record_report_id_unbound`: a record with only `model_ref`, or
  only `scope.report_id`, changed, every other member coherent. Refused
  natively (`REPORT-USER-RELEASE-UNBOUND`), by the producer
  (`REPORT-PACKAGE-USER-RELEASE-UNBOUND`) and, in a re-hashed package, by the
  reader (`OPSPROJ_USER_RELEASE_UNBOUND`).
- `user_release_second_result_member_refused`: a valid released package with a
  second, valid result member added and the inventory restamped; the reader
  refuses it.
- `user_release_same_source_member_substitution_refused`: a valid released
  package whose HTML member is replaced by the HTML of another released save
  of the same model and run (a different `released_at`), whose PDF is swapped
  the same way, and whose audit and handoff members are swapped with
  equal-identity variants, each with the inventory restamped and the record
  unchanged; the reader refuses each (`content_binding`; and the notice text
  for the HTML).
- `existing_contexts_preserve_raw_intent_named_members`: a request whose
  source-row `metadata` and a handoff section carry raw members named
  `user_release_intent` and `explicit_user_release_intent`, under
  `local_private` on route 009 and on `DREP-IPC-003` and `DREP-HTML-SAVE-005`.
  The controlled payload is byte-identical to the input, the result document's
  self-hash verifies, and the exact-payload agreement holds, as at the parent
  head (a vitest and a pytest of the projector, plus a Tauri admission test on
  route 009). The same request on route 010 is refused as unbound.

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
    decide in an unknown branch is decided by the new step.
- Corpus parity: new cases in `cases.json`, checked by the vitest corpus test,
  `tests/security/test_redaction_export_controls.py` and the Rust mirror test
  `canonical_parity_corpus_matches_rust_contract_mirror`
  (`redaction_binding.rs:702`).
- Rust, Tauri (`report_package_save.rs` tests, sandbox directory, counting
  picker): on route 010, a request carrying `private_material_data` at depth
  (`REPORT-USER-RELEASE-PRIVATE-CLASS`); a spring-hanger quantity from a
  library-applied support (`REPORT-USER-RELEASE-LIBRARY-VALUE`); a rule-pack
  reference with `private_payload_redacted: false` or an added `formula`
  member (`REPORT-USER-RELEASE-RULE-PACK-DETAIL`); a comparison section
  (`REPORT-USER-RELEASE-COMPARISON-EXCLUDED`); an unattested carrier and an
  unattested rule-check projection; an analysis record whose hash differs.
  Each is refused before the picker, and the directory stays empty.
- Rust, `report_package::user_release`: the projection vectors pass, and a
  mutation of a leaf of each table class in a derived request is refused with
  the named code.
- Parity: the projection vectors in vitest and Rust; the analysis-record
  vectors also in pytest against `compatibility.py`.
- Attestation (`solve_jobs.rs` and `lib.rs` tests): the solved demo's
  attestation equals the webview's `canonicalSha256Hex` of the model and of
  the received result; the tables are bounded, evict the oldest entry, are
  cleared by `dispose_all_results`, and are never written to disk; the worker's
  digests stay within its grant (counting allocator).

**Owner property 2: redaction applies to every other context.**
- The 23 existing corpus cases are not edited. A vitest and a pytest pin the
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
  with the bound hashes and `released_at`, and the run-context rows; there is no
  local-private banner and no `content_binding` text.
- `report_package` (new `tests/user_release_package.rs`): the manifest's
  `user_release` equals the report context's plus a `content_binding` that
  matches the members; the four boundary notes; a round trip through the reader
  returns `ReadReportPackage.user_release`.
- vitest: the dialog's wording, its initial focus on the checkbox (set in
  `useLayoutEffect`), the disabled save until ticked, and the status line and
  receipt fields after a save.

**Owner property 4 and HELP_HUMAN: no replay.** One test per row of §2.5:
- native (`report_package_save.rs` tests): another model's text; another
  result document; another input manifest; another analysis record; a record
  on route 009; an altered statement or standing; `model_ref` or `report_id`
  alone;
- producer (`tests/user_release_package.rs`): each bound member mismatched
  alone, with every other member coherent; a record under the desktop
  local-private and runner profiles;
- reader (`tests/reader.rs`): the record moved into another valid package and
  the package re-hashed; a second result member; each member swap of
  `user_release_same_source_member_substitution_refused`; a record in a blocked
  package; the user-released profile without a record; and, as a documented
  limit, a package whose record, members, `content_binding` and inventory are
  all re-written together is accepted;
- render command (`src-tauri/src/lib.rs` tests) and renderer: a released input
  outside the package path;
- webview (`reportPackageSave.session.test.tsx`): after a user-released save,
  the next Save Report Package… is local private with no record; the project
  save payload, the model document text and the session state hold no trace of
  the selection; a basis change while the dialog is open closes it.

**Also:**
- **Linter and dispositions:** a released request with a quarantined
  provenance at depth, a `protected_suspected` privacy class, and a rejected
  rule-pack reference is refused natively (`PROTECTED_CONTENT_BLOCKED`); a
  released report whose section text trips the protected-content linter is
  refused (`REPORT-PACKAGE-PRODUCER-BLOCKED`); the webview blocks the same in
  the new context.
- **Forged evidence:** route-010 evidence with `local_private`; route-009
  evidence over a released request; release intent false; evidence for other
  bytes; local-first evidence with another storage context.
- **Native save and read-back:** admitted released bytes written to the sandbox
  read back with `read_report_package_container`, with the record, its content
  binding and the member hashes intact.
- **Native smoke:** a user-released save on the solved demo through
  `native_smoke::take_chosen_path`. The harness checks the file's SHA-256
  against the receipt, the record's model hash against the session model's,
  and the HTML member's banner. It adds no pinned value;
  `apps/desktop/native-smoke/expected.json` is not changed.
- **Claims lint:** `validate_claims_language.py` on each slice's head; the new
  product strings are in scanned `apps/desktop/src` files.
- **Codex scrutiny:** one independent round per slice on its implementation
  head, plus the Appendix A texts as exact text before HELP_HUMAN. Points for
  review: the decision order (§3.2), the projection table's closure, the
  builder mirrors and their vectors, the attestation's trust and bounds, the
  source-resolution and library rules, the reader's equality-only checks and
  the content binding, and the naming rule (§0).

## 7. Decisions needed

- **O-A (owner): package version under DEC-028.** V1 (recommended): stay at
  2.0.0 with the optional record. V1 needs an explicit interpretation of
  DEC-028, that "one version" admits an optional writer-defined record and that
  the reader at each head accepts what the writer at that head emits; DEC-028
  alone does not establish that per-head reading. Consequences: V1 keeps every
  unreleased package's bytes identical (the intent strip is not widened, §3.3),
  and a pre-W-18b reader refuses a released package as a malformed manifest.
  V2: move to 3.0.0; every existing 2.0.0 package is refused by name, and every
  package's bytes change. §4 gives both in full.
- **O-B (HELP_HUMAN): the release time in the package.** Recommended: record
  `released_at`, and change the third boundary note for released packages only
  (§5.3). Alternative: record no time. The third boundary note stays
  unchanged; the user-release note is still added. The record then has no
  time. A release is a dated statement, so the time is recommended.
- **O-C (HELP_HUMAN): surfaces.** Recommended: the package only (§2.1).
  Alternative: also the standalone rendered HTML. That needs a native save
  command for the rendered HTML, with the package's evidence, bindings, screens
  and the release projection (about M). The print/PDF view could still not be
  bound.
- **O-D (HELP_HUMAN): library values.** Recommended: refuse a release that
  would carry a library record value (§3.2). With the native projection this
  refusal is enforceable: it is derived from the model, not from caller labels.
  Consequence: a model with a library-applied spring hanger cannot be
  user-released. Alternative: show such values as redaction markers. Marker
  support would have to cover the JSON members (the report-section entries and
  any member that copies them) as well as the rendering, with a non-lossless
  package route (option (A)'s work, about M).
- **O-E (owner; data boundary): rule-check identifiers.** This decides what
  of a private rule pack leaves the user's machine. Recommended: user-rule check
  ids and missing-input names stay in released reports as safe metadata under
  REXC-REQ-009 ("such as ID, version, checksum, and source note"), with the
  pack's name, version, checksum, source notice and redistribution status, all
  taken from the attested rule-check run (§2.3). Consequence: a pack author's
  identifiers are shared as written; if an identifier itself encodes a private
  rule value, no native check can tell. Alternative: omit user-rule outcomes
  from released reports and keep only the pack reference; the shared report
  then shows no check outcomes, and the dialog, notice and Appendix A texts that
  name "user-rule outcomes" change.
- **O-F (HELP_HUMAN): comparison and auxiliary content outside the recorded
  result identity.** The record binds one model, one run and one result. The
  package can also carry run-comparison sections (`stateComparisonHandoffSections.ts:173-195`),
  whose content native admission cannot derive, and auxiliary content (the
  local persistence reference, audit assets). Recommended: exclude them from a
  released package (comparison sections refused, persistence reference `TBD`,
  assets `[]`), and say so in the dialog. Consequence: a shared package carries
  no comparison summary (FR-REP-004, Should). Alternative: bind each comparison
  natively, by deriving it from the attested result with a Rust mirror of the
  comparison builder (about M), or bind it by hash alone, which would not show
  where its values came from.
- **O-G (owner): the integrity-versus-consent guarantee.** What the record
  proves (§2.4): consistency of the record with the members and their bytes,
  and, at save time, that native admission received it on route 010 with the
  wrapper's intent over this session's solved model and result. What it does
  not prove: that a person made the statement, that this application produced
  the package, or that the statement is true; a coherent forgery is not
  detected. Recommended: accept the equality-only guarantee, and state it in the
  reader text and in DEL-02-05 (A8). Alternative: a signature over the record
  and the content binding, which needs a key, its custody and a verification
  path; none exists in the product, and it is outside W-18b.
- **Wording grant.** Appendix A, and the product texts of §2.1 and §5, go to
  Codex as exact text and then to HELP_HUMAN.

## 8. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(r1) Vocabulary and redaction** | The context, reason codes and route id in TypeScript, Python, the Rust mirror and the schema; the decision order of §3.2 (steps 10-13); the flag assertion; the local-first rule; the release projector and the projection table (unused until r3); corpus cases, the pinned-digest tests and `existing_contexts_preserve_raw_intent_named_members`. No route uses the context. | M (~800 lines with tests) | Data boundary (vocabulary only) |
| **(r2a) Attestation** | Worker digests of the solved model and result, the solve table in `SolveJobRegistry`, the rule-check projection mirror and table in `run_rule_checks`, clearing and bounds, memory accounting and tests. | S-M (~500) | Data boundary; DEL-04-07 memory accounting |
| **(r2b) Release projection** | `report_package::user_release`: the Rust mirrors of the input-manifest, analysis-record, document-base, renderer-input, audit and handoff builders for the package path; source resolution and the library rule; the projection table's native use; the projection vectors and their vitest, Rust and pytest parity. | L (~2k) | **Data boundary** (what may be shared) |
| **(r2c) Record, renderer, producer, reader, native** | `UserReleaseRecord`; context and blocking codes; notice and rows in HTML and PDF; producer item 9 and the profile; writer manifest, content binding and notes; reader checks, cardinality and refusal; native route-010 branch (§2.3 items 1-7) with the analysis-run text; the render-command refusal; the Rust tests of §6. The desktop does not offer it yet. | M-L (~1.5k) | **Persistence** (manifest and reader) and **data boundary** (native admission) |
| **(r3) Desktop** | Dialog, panel button, menu item (webview and native menu), session wiring and modal guard, builder variant, save-service evidence and the analysis-run text, receipt display, vitest, native smoke step. | M (~1k) | Data boundary (webview evidence) and file output |

The order is (r1), then (r2a) and (r2b) in either order or together, then
(r2c), then (r3). (r2c) needs (r2a) and (r2b) in its ancestry; (r3) offers the
user release and needs (r2c). (r2c) and (r3) may land as one PR if review
prefers. The total grows from about 2.9k to about 5.8k lines, mostly in the
builder mirrors and their vectors (r2b).

## 9. Reconciliation, per slice

Each text changes in the PR of the slice that makes it true. Commitment,
acceptance and ScopeOfWork texts are applied only as Appendix A's reviewed
exact text.
- **(r1)**
  - DEL-12-02 `ScopeOfWork.md` :58, the context vocabulary (A3).
  - `docs/security/redaction_export_controls.md`: the Export Contexts table
    (:59-65) gains the `user_released_report` row, with its decision order; the
    Final-Sink Route Binding section (:158-163) says that release intent comes
    only from the wrapper, and that the strip list is unchanged.
  - DEL-12-02 `deliverable.yaml` check pointers; `tools/hosted-ci-routing.json`
    for the changed sources, the table and the tests.
- **(r2a)**
  - The `solve_jobs.rs` module text (:1-41) names the attestation table.
  - DEL-04-07 Design memory-budget text, where it lists the worker's result
    frame, names the two digests; DEL-04-07 `deliverable.yaml` checks; CI
    routing for the Tauri sources and tests.
- **(r2b)**
  - DEL-08-01 `deliverable.yaml` code paths (`user_release` module) and checks
    (the projection vectors); DEL-14-02 `deliverable.yaml` check pointer for
    the analysis-record vectors' pytest; CI routing.
- **(r2c)**
  - DEL-02-05 `ScopeOfWork.md` :214, the reader (A8).
  - DEL-12-02 REXC-OI-007 :486, the native checks (A7), and CLM-030 :414 (A5;
    see its note).
  - DEL-08-01 `deliverable.yaml` checks (new `tests/user_release_package.rs`);
    `reader.rs` module text (:1-42), with the §2.4 statement of what the checks
    establish; CI routing.
  - DEL-08-01 Design `report-package-current-results.md` keeps its text; it
    carries only a pointer to this design.
- **(r3)**
  - DEL-08-01 `ScopeOfWork.md` :81 (A1) and OQ-08-01-003 :383 (A2).
  - DEL-12-02 REXC-REQ-016 after :189 (A4), REXC-OI-002 :481 (A6), CLM-036
    :470 (A9).
  - User guide `docs/user_guide/index.md` :91 (A10), :109-111 (A11), :393
    (A13) and :482-483 (A12). The §10 sentence's "renders a report only as a
    local private document" would mislead once a shareable package exists, so
    A13 names the standalone report. Its table row (:148) stays true: the
    package is still saved locally.
  - `docs/COMPLETENESS_MAP.md`: gap 1 (:91-96), FR-REP-001 (:210), the
    not-ready list (:469-472) and the W-18 row (:552) record W-18b.
  - `apps/desktop/SMOKE.md`, the package-save entry (:9881): the new step.
  - DEL-08-01 `deliverable.yaml` code paths (`UserReleaseDialog.tsx`) and
    checks.
- **No change:** DEL-04-08 REQ-006 (c) :219 and AC-006 :240 stay true (the
  same policy cell and producer boundary). The DEL-08-02 audit manifest is
  unchanged, and so is its ScopeOfWork. The PRD §19.3 notice is unchanged, and
  so is `docs/report_notice_template.md`.

## Appendix A. Wording-route drafts (old → new; not applied)

Each Old is copied byte-exact from the file at `8cec2178c1` and occurs exactly
once in it (checked with a Python `count == 1` at revision 1). Line numbers are
where the Old begins. In an Old or New, `\n` stands for a line break and `\"`
for a quotation mark inside the quoted text. These drafts go to Codex as exact
text, then to HELP_HUMAN under the wording grant. Nothing here is applied.

**Depends on** names what each New needs before it is true: F1-F3 are the
release projection of §3.2 with the attestation of §2.3 (Codex round 1,
findings 1-3); F4 is the record bindings, cardinality and content binding of
§2.3-§2.5 (finding 4); O-A to O-G are §7's decisions. If a decision goes the
other way, its dependent New changes before it is applied. A3, A6, A10, A12
and A13 depend on none of them.

**A1. DEL-08-01 `ScopeOfWork.md` :81** (current-implementation paragraph, last sentence; slice r3; depends on F1-F4, O-D, O-E, O-F)
- Old: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21)."
- New: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21). Save Shareable Report Package… saves the same package for sharing after the user's per-report release of their own project data (route `DREP-PACKAGE-USER-RELEASE-010`, export context `user_released_report`; DEL-08-01 Design `report-user-release.md`): the release record in `report_context.user_release` and in the package manifest states the user's redistribution right and binds the model reference and document SHA-256, the run's input manifest and the result document, and the manifest copy also binds the hash of every other member; native admission requires every request member to equal its derivation from the bound model, the result this session's solver produced for it and the bound run; rule-pack references and user-rule outcomes retain safe metadata, library record values are excluded and a report that would show one is refused, and run comparisons are not carried; and the rendered members carry the user-release notice in place of the local-private notice. Without that selection the package is local private as above, and the selection is not stored."

**A2. DEL-08-01 `ScopeOfWork.md` :383** (OQ-08-01-003, last cell; slice r3; depends on F1-F3)
- Old: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. |"
- New: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. A shared report carries private project data only through the user's per-report release of their own data, and only as values derived natively from the bound model, its solve in this session and its run (DEL-12-02 `user_released_report`; DEL-08-01 Design `report-user-release.md`). |"

**A3. DEL-12-02 `ScopeOfWork.md` :58** (CLM-003; slice r1)
- Old: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private` |"
- New: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private`, `user_released_report` |"

**A4. DEL-12-02 `ScopeOfWork.md` :189** (CLM-012; a row added after REXC-REQ-015; slice r3; depends on F1-F4)
- Old: "> | REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |"
- New: the Old line unchanged, followed by this line:
  "> | REXC-REQ-016 | A shareable report may include the user's own private project data only after the user's explicit release of that data for that one report, with the user's statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). The export context is `user_released_report`; the release is recorded in the report and its package, bound to the model and result it covers, and stated in the report's notice; it is not stored for later reports; private rule-pack and library values stay at safe metadata; protected, quarantined and rejected content still blocks; every other export context is unchanged. | OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` §6; owner, 2026-10-24 (W-18 option B) | Focused tests: the release flag cannot enable export in another context; redaction is unchanged in every other context; the record is visible in the report and the package; an unchanged record with a mismatched bound model, result or package identity, or with member content other than the content it binds, is refused. |"

**A5. DEL-12-02 `ScopeOfWork.md` :414** (CLM-030, last sentence; slice r2c; depends on F1-F3, O-D, O-E)
- Note: CLM-030 states context policy, but its last clause ("the release is recorded in the report") is true only once the recording path exists, so A5 is applied in (r2c), not with the vocabulary in (r1).
- Old: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks."
- New: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks.\n> `user_released_report` carries the user's own project data in one report only\n> after the user's explicit release of it, with their statement of the\n> redistribution right, and only as values derived from the bound model, its\n> solve and its run; rule-pack references and user-rule outcomes retain safe\n> metadata, library record values are excluded, unbound content is refused,\n> protected, quarantined and rejected content still blocks, and the release is\n> recorded in the report."

**A6. DEL-12-02 `ScopeOfWork.md` :481** (REXC-OI-002; slice r3)
- Old: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent."
- New: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent. The desktop's per-report user release (`DREP-PACKAGE-USER-RELEASE-010`, `user_released_report`) is selected in its own dialog for one save and is not stored."

**A7. DEL-12-02 `ScopeOfWork.md` :486** (REXC-OI-007; slice r2c; depends on F1-F4, O-D, O-F)
- Old: "before the save dialog; caller evidence alone admits nothing."
- New: "before the save dialog; caller evidence alone admits nothing. On its user-release route (`DREP-PACKAGE-USER-RELEASE-010`) it admits the `user_released_report` context only with a release record whose bound model, input-manifest and result-document identities equal the texts and members it received, for a result this session's solver produced for that model, and only when every request member equals its own derivation from the bound model, that result and the bound run; it refuses a record on the local-private route, a missing, ambiguous or substituted source, a non-null quantity in report_sections.user_supplied_values whose source resolves to a library-referenced model object, or a library hanger reference in that object's provenance text, a comparison section, and any request object with a non-project private class."

**A8. DEL-02-05 `ScopeOfWork.md` :214** (reader; slice r2c; depends on F4, O-G)
- Old: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value;"
- New: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value; when the manifest records a user release, it checks the record's form, exactly one result member, the equality of its bound model reference and hash, input-manifest hash, result-document identity, report and package identities with the manifest, audit and result members, and the member hashes the record binds, and refuses a record in a blocked package or under another export profile (equality checks: they show consistency, not who made the statement or produced the package);"

**A9. DEL-12-02 `ScopeOfWork.md` :470** (CLM-036, a row added after "Local private report"; slice r3; depends on F1-F4, O-E)
- Old: "> | Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |"
- New: the Old line unchanged, followed by this line:
  "> | User-released report | The user's own project values in one report after the explicit per-report release, derived natively from the bound model, its solve and its run, recorded in the report and stated in its notice; rule-pack references and user-rule outcomes at safe metadata; no library record values | A release carried over to another model, result or report, remembered for later reports, or covering content not derived from the bound model and run |"

**A10. User guide `docs/user_guide/index.md` :91** (slice r3)
- Old: "- **Reports are local-private only, and pressure-free only.**"
- New: "- **Reports are pressure-free only. Shareable packages require your per-report statement of rights to share your own project data.**"

**A11. User guide `docs/user_guide/index.md` :109** (a bullet added; slice r3; depends on F1-F3, O-D, O-E, O-F)
- Old: "  - Save Report Package… saves a local private package for a pressure-free\n    result after its own local-private selection; exact-route results are\n    refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`."
- New: the Old bullet unchanged, followed by:
  "\n  - Save Shareable Report Package… saves the same package for sharing after\n    you tick, in its dialog, \"I hold the right to share this project's data,\n    and I release it in this report for sharing.\" The release covers that one\n    report. Its documents carry a \"USER-RELEASED REPORT\" notice naming the\n    model and result it covers. Rule-pack references and user-rule outcomes\n    retain safe metadata; library record values are excluded. The selection\n    applies only to this save. A report that would show a value taken from a\n    library record, such as a spring hanger applied from a library, is\n    refused. It needs a result solved in this session, and it carries no run\n    comparison."

**A12. User guide `docs/user_guide/index.md` :482** (limitations list; slice r3)
- Old: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents and only for pressure-free results,"
- New: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents or shareable packages with your per-report statement of rights to share your own project data, and only for pressure-free results,"
- Note: the words are the reviewed New exactly; the line break after "local private" is the Old's own, kept so the list item's wrapping is unchanged.

**A13. User guide `docs/user_guide/index.md` :393** (§10, first sentence; slice r3)
- Old: "renders a report only as a local private document"
- New: "renders the standalone report only as a local private document"
