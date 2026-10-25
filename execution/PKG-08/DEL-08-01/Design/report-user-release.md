# Per-report user release of the user's own project data (W-18b)

**Basis:** `origin/main` at `8cec2178c1` (2026-10-25), after W-18a (a) #1597,
(b1) #1606 and (b2) #1616. Every file:line citation is at that head. Design
before implementation; nothing here is built. This design records the
mechanism by which a user can state, for one report, that they release their own
project data for sharing. It records no release, and it records, implies or
derives no acceptance, issuance, approval, compliance or lifecycle standing
(F-PIP-3, `loop/LOOP_INIT.md:68-72`).
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

**Naming rule.** The word "release" never appears alone in a product text,
identifier or record. It is always "user release", "user-released" or "release
your project data". No text uses "issued", "approved", "accepted",
"certified", "released version" or "release of SWBPIPE". The record's
`standing` member is the constant `user_statement_of_data_rights_only` (§2.2).

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
- **The rule-pack boundary is already safe metadata by construction.** The
  report reads only a pack's identity, name, version, checksum, source notice
  and redistribution status (`renderableReportInput.ts:404-449`) and a check's
  id, status and missing-input names (`:451-468`; R-08-01-008,
  DEL-08-01 `ScopeOfWork.md:141`). The typed renderer input is the only path
  from the request to the rendered members (`wire.rs:275-305`), so no other
  rule-pack field reaches the HTML or PDF.
- **One library value reaches the report.** A spring hanger's load quantity is
  copied into `report_sections.user_supplied_values[].quantity`
  (`renderableReportInput.ts:275-306`), whatever the hanger's provenance. A
  model provenance record may name a `library_ref` (`types.ts:15-27`). No other
  library record value is in the report (user guide `docs/user_guide/index.md:397-400`:
  "materials, sections and libraries are not yet in it").
- **The package and its reader.** The manifest's fields are fixed by the version
  except `package_id`, `export_profile_id`, the source references and the
  diagnostics (`lib.rs:643-734`). Its third boundary note reads "No timestamps or
  generated identifiers are recorded anywhere in this package; identical inputs
  produce identical bytes." (`lib.rs:722-724`). The reader rebuilds the
  manifest from those fields and refuses any difference (`reader.rs:221-240`).
  It does not parse the rendered HTML or PDF beyond UTF-8 and a PDF header
  (`reader.rs:891-902`).
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
> - Rule packs and libraries appear only as identity, version and checksum.
> - Content marked protected, quarantined or rejected still stops the save.
> - Nothing is remembered: your next report is local private unless you
>   choose this again.
>
> ☐ I hold the right to share this project's data, and I release it in this
> report for sharing.
>
> [Save Shareable Report Package…] [Cancel]

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
| `scope.report_id` | string | the report's `calculation_report.report_id`, `report:<run id>` (`renderableReportInput.ts:591`) |
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

The free-text members are constants, so a record cannot carry any other
wording (for example a claim) in their place.

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
  the manifest inventory already records the member's byte hash.
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
2. The package manifest's top-level `user_release`, written by the producer as
   a copy of the typed `report_context.user_release` (one source, so the two
   copies cannot differ). Absent from every package without a user release.
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

### 2.3 How the record is built and checked natively

**Webview (an early check; native admission decides).**
- `buildReportPackageRequest` (`reportPackageRequest.ts:158-255`) takes
  `userRelease: { releasedAt: string } | null`. With a value it:
  - sets `package_id` to `desktop-user-released-report-<run>` and
    `export_profile_id` to `desktop_user_released_report_package_1`;
  - renders under `{ exportContext: "user_released_report", generatedAt: null }`;
  - builds the record from the same `document`, `modelHash` and
    `inputManifest` it already uses (`:175,209,215`);
  - refuses with `REPORT-USER-RELEASE-LIBRARY-VALUE` when a
    `user_supplied_values` entry with a non-null `quantity` comes from a model
    object whose provenance record names a `library_ref` (§3.2).
- The request is controlled by a new projector,
  `controlUserReleasedReportPackageRequest(payload, explicitUserReleaseIntent)`,
  under route `DREP-PACKAGE-USER-RELEASE-010`, context `user_released_report`,
  lossless (§3.1, §3.3).
- The save service sends the same three canonical texts. The screening evidence
  names route 010, context `user_released_report`,
  `explicit_local_private_intent: false` and `explicit_user_release_intent: true`.

**Native admission** (`admit_report_package`, `report_package_save.rs:591-662`).
The command stays `save_report_package`; the route id in the evidence selects
the branch. Every existing check runs on both routes in its current order.
Changes:
1. **Evidence** (`validate_report_package_save_evidence`, `:294-328`).
   `ReportPackageSaveScreening` (`:110-125`, `deny_unknown_fields`) and
   `ReportPackageLocalFirstEvidence` (`:130-139`) gain
   `#[serde(default)] explicit_user_release_intent: bool`.
   - Route 009: unchanged, plus `explicit_user_release_intent` false.
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
3. **Record form** (route 010). Exact members and constants as §2.2, hex
   form and the `released_at` form, else `REPORT-USER-RELEASE-RECORD-INVALID`.
   The time is the webview's clock reading; native checks its form only.
4. **Bindings** (route 010), each against a value admission itself computed or
   already bound, never against another caller field alone. Any inequality is
   `REPORT-USER-RELEASE-UNBOUND`, naming the member:
   - `model_document_sha256` = `model_hex`, the SHA-256 of the model text
     admission received (`:613`);
   - `model_ref` = the model text's `/project/id`;
   - `input_manifest_sha256` = `input_manifest_hex` (`:624`), and
     `input_manifest_ref` = the reference admission derives
     (`analysis_runs::input_manifest_ref`, `:645`);
   - `analysis_run_ref` = `/source_results/0/run_id`;
   - `result_envelope_id` and `result_document_derivative_sha256` =
     `/result_documents/0/result_envelope/envelope_id` and
     `…/reproducibility/derivative_hash/value` (the shared producer then
     verifies that hash over the document, `document_form::check_self_hash`);
   - `scope.report_id` = `/report/calculation_report/report_id`, and
     `scope.package_id` = `/package_id`.
5. **Release content rules** (route 010; §3.2): `REPORT-USER-RELEASE-PRIVATE-CLASS`,
   `REPORT-USER-RELEASE-LIBRARY-VALUE`, `REPORT-USER-RELEASE-RULE-PACK-DETAIL`.
6. The disposition screens (`screen_content`, `:412-448`), the rule-pack
   agreement (`:572-588`) and the shared producer run unchanged on both routes.

**Shared producer** (`report_package::bindings::check_request`,
`bindings.rs:180-496`). A new item 9, after item 8 (`:491-494`):
- the profile table (`:213-217`) gains
  `(USER_RELEASED_DESKTOP_PROFILE, DESKTOP_ORIGIN) => true`;
- item 8 becomes: the local-private profiles (desktop and runner) require
  `export_context` `local_private` and no record
  (`REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` if a record is present); the
  user-released profile requires `user_released_report` and a record;
- item 9 checks the record against the request's members:
  `model_document_sha256` = `audit_manifest.model_hash.value` = the document's
  model hash; `input_manifest_sha256` and `input_manifest_ref` = the audit
  input-manifest hash value and payload reference; `analysis_run_ref` = the
  document's `run_ref.ref_id`; the envelope id and derivative hash = the
  document's; `scope.report_id` and `scope.package_id` = the report's and the
  request's (`REPORT-PACKAGE-USER-RELEASE-UNBOUND`).

The headless runner keeps its profile (`bindings.rs:36`), so a runner request
with a record is refused. The runner gains no flag.

**Writer.** `ReportPackageContainerInput` (`lib.rs:116-136`) and
`ManifestFields` (`lib.rs:643-650`) gain `user_release: Option<&Value>`.
`package_manifest_document` (`lib.rs:652-734`) writes `"user_release"` only when
present and then uses the user-released boundary notes (§5.3).

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
- After `verify_members` (`:807-969`), when a record is present:
  - the package is unblocked, and `export_profile_id` is
    `desktop_user_released_report_package_1`;
  - the record's constants and forms hold;
  - `model_document_sha256` = the audit member's `model_hash.value`;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit member's
    `input_manifest_hash.value` and `payload_ref`;
  - the one result member's `envelope_id`, derivative hash value and
    `run_ref.ref_id` = `result_envelope_id`,
    `result_document_derivative_sha256` and `analysis_run_ref`;
  - `scope.package_id` = the manifest's `package_id`, and `analysis_run_ref` =
    `source_basis_refs[0].ref_id`.
- A user-released profile without a record, or a record with any other
  profile, is refused.
- Refusal: a new variant `PackageReadRefusal::UserReleaseUnbound { reason }`,
  code `OPSPROJ_USER_RELEASE_UNBOUND` (`reader.rs:90-121`).
- `ReadReportPackage` (`:64-85`) gains `user_release: Option<Value>`, so a
  consumer can show the record.

The audit and result identities are collected in `verify_members` as the audit
identity already is (`:917-920`). These are equality checks, not signatures
(`reader.rs:30-41`): a package whose members and record are all re-forged and
re-hashed together is not detected. The reader checks the manifest copy; the
rendered members are bound to it by their inventory hashes, and their content
is the producer's. The module text is updated to say so.

### 2.5 No replay

A release record copied onto another model, result set or report is refused:

| Attempt | Refused by | Code |
|---|---|---|
| Record copied into a request for another model | native admission (model text SHA-256), then the producer (audit and document model hash) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-PACKAGE-USER-RELEASE-UNBOUND` |
| Same model, another solve or result document | native admission (run, envelope id, derivative hash); the producer | as above |
| Same result, another input manifest | native admission (input-manifest text SHA-256 and reference) | `REPORT-USER-RELEASE-UNBOUND` |
| Record placed on the local-private route 009 | native admission; the producer (local-private profile) | `REPORT-USER-RELEASE-NOT-THIS-ROUTE`; `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` |
| Manifest record copied into another package, re-hashed | the reader | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed in a standalone render | webview route check; the render command; the renderer (record outside its context) | `REPORT-REDACTION-CONTEXT-MISMATCH`; `RENDER-USER-RELEASE-PACKAGE-ONLY`; `REPORT-CONTEXT-USER-RELEASE-MISMATCH` |
| A later save reusing an earlier selection | nothing to reuse: the selection is not stored (§2.2) | none needed |
| A record whose statement or standing text is altered | native form check; the reader | `REPORT-USER-RELEASE-RECORD-INVALID`; `OPSPROJ_USER_RELEASE_UNBOUND` |

## 3. The classification and routes

### 3.1 DEL-12-02 runtime classification

**Name:** a new export context, `user_released_report`.

**Meaning (proposed):** a report of the user's own session that the user has
released for sharing, for that one report, by an explicit selection with a
statement that they hold the redistribution right
(`docs/IP_AND_DATA_BOUNDARY.md` §6). In it:
- the user's private project data (`private_project_data`) is included, with a
  `USER_RELEASED_PROJECT_DATA` warning finding;
- every other private class is redacted as in `public_report`;
- `protected_suspected`, `quarantined` and `rejected` content blocks;
- unresolved metadata is warning-only, as in `local_private`;
- the context is admitted only on route `DREP-PACKAGE-USER-RELEASE-010`, with
  the wrapper's release intent. Payload content never selects or grants it.

It is an export context, not a privacy class, on purpose. A privacy class such
as `user_released_project_data` would be carried in the payload, and any
context that included it would let payload content grant exposure. The existing
design gives that authority only to the wrapper (`stripRouteIntent`,
`redactionExportControls.ts:825-842`; `docs/security/redaction_export_controls.md:158-163`).

**Vocabulary additions** (DEL-12-02 CLM-003, the schema and the three mirrors):
- export context `user_released_report`;
- reason codes `USER_RELEASED_PROJECT_DATA` (finding class
  `PRIVATE_DATA_WARNING`, severity WARNING), `USER_RELEASE_INTENT_REQUIRED`
  (`PRIVATE_DATA_WARNING`, BLOCKING), and the local-first reasons
  `USER_RELEASE_METADATA_ALLOWED` and `USER_RELEASE_ROUTE_REQUIRED`;
- route id `DREP-PACKAGE-USER-RELEASE-010`.

### 3.2 Redaction: released project values and private rule-pack and library values

**The decision rule**, added to `actionFor` (`redactionExportControls.ts:417-479`)
and `_action_for` (`controls.py:372-426`). It sits after the include branch and
before the existing private branch, and applies only when the context is
`user_released_report`:
- privacy `private_project_data` (with or without `private_only`): with
  release intent, `warning_only` / `USER_RELEASED_PROJECT_DATA`; without it,
  `block_export` / `USER_RELEASE_INTENT_REQUIRED`;
- any other privacy class in the private set (`:212-221`; `controls.py:22-31`),
  or `private_only` with a non-project privacy class: `redact_value` /
  `PRIVATE_DATA_REDACTED`. On the lossless package route that redaction
  withholds the payload (`controlRouteExport`, `:968-969`), so the save is
  refused; nothing is shipped redacted or unredacted;
- `unknownAction` (`:413-415`; `controls.py:428-431`) returns `warning_only`
  for `user_released_report`, as for `local_private`. The route projector
  classifies every leaf without a private token as `unknown`
  (`redactionExportControls.ts:885-925`), so redacting `unknown` would withhold
  every released package. REXC-REQ-005 admits a warning for unknown status.
  This choice is a point for Codex review (§6).

Every branch before the new one is unchanged, including the protected,
quarantined and rejected blocks (`:439-444`; `controls.py:402-405`). No branch
reached by another context changes.

**Rule-pack values.** They stay at safe metadata by construction (§1), and the
typed renderer input bounds what reaches the rendered members. Native check 5
of §2.3 also refuses a released request whose
`calculation_report.rule_pack_refs[]` entry has
`private_payload_redacted` other than `true`, or carries a member outside
`{rule_pack_id, name, version, checksum, source_notice, redistribution_status,
completeness_status, private_payload_redacted}`, or whose `user_rule_checks`
checks carry a member outside `{check_id, status, missing_inputs}`
(`REPORT-USER-RELEASE-RULE-PACK-DETAIL`). Check ids and missing-input names are
identifiers, which this design treats as safe metadata under REXC-REQ-009
("such as ID, version, checksum, and source note"); see **O-E** (§7).

**Library values.** A released report must not carry a value taken from a
library record. The one such value the report carries is the spring-hanger
quantity (§1). So, on route 010:
- the webview refuses early, and native admission refuses authoritatively, a
  `report_sections.user_supplied_values[]` entry with a non-null `quantity`
  whose `source` names a model object whose `provenance_record` carries a
  `library_ref` (`REPORT-USER-RELEASE-LIBRARY-VALUE`, naming the object and the
  library);
- native admission refuses any object anywhere in the request whose own
  `privacy_classification` or `privacy_class` is
  `private_material_data`, `private_component_data`,
  `private_rule_pack_data`, `owner_standard_data`,
  `company_design_basis_data`, `path_data` or `secret_like_data`
  (`REPORT-USER-RELEASE-PRIVATE-CLASS`);
- a `library_ref` itself (kind, id, version, record id) is identity and stays.

The rule refuses whatever the stored library's own status is, so it needs no
store read beyond the existing screen. A model that uses libraries can still be
released when no library value appears in the report: computed results are the
user's project results. Classification is explicit-metadata only
(`redactionExportControls.ts:1-13`): an unlabelled value that a forged request
inserts cannot be told from project data, as for every other route. The
alternative, showing library values as redaction markers, is decision **O-D**
(§7).

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
- `stripRouteIntent` (`redactionExportControls.ts:708-712,825-842`) and
  the Python projector (`core/security/redaction/route_control.py:19-23,159-180`) also strip `user_release_intent` and
  `explicit_user_release_intent` from payloads.
- `redactExportPayload` and `controlRouteExport` refuse
  `explicitUserReleaseIntent: true` with any context other than
  `user_released_report` (an exception, as `assertContext` throws,
  `:665-669`). The flag therefore has no effect in another context.
- The Rust mirror (`core/runner/headless/src/redaction_binding.rs:166,216-218`)
  implements the same rule so the shared corpus
  (`fixtures/redaction_export_controls/cases.json`, read at `:702-740`) holds.
  No runner route uses the context.
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
  `quarantined` and `rejected` before any context branch
  (`redactionExportControls.ts:439-444`). Native admission runs both screens on
  route 010 as on 009 (`report_package_save.rs:412-448`):
  `PROTECTED_CONTENT_BLOCKED` and `REPORT-PACKAGE-SAVE-DISPOSITION-MALFORMED`.
- **Every other context is unchanged.** `public_report`, `public_example`,
  `shared_model`, `downstream_tool` and `local_private` keep every decision on
  every input. Routes 009, `DREP-IPC-003` and `DREP-HTML-SAVE-005` keep their
  contexts and evidence. A local-private package is byte-identical to one
  assembled before W-18b (§4).

## 4. Persistence and format

**The package (`.opsproj`).** The member set and the result, audit and handoff
member forms are unchanged. Two things change for a user-released package only:
- the manifest gains the optional `user_release` member (§2.2);
- the manifest's boundary notes become the user-released set (§5.3).

The rendered HTML and PDF members of a user-released package carry the notice
and context rows (§5). A package without a user release is byte-identical to
today's. Version handling is decision **O-A**:
- **V1 (recommended): stay at 2.0.0.** The reader at any head accepts exactly
  what the writer at that head emits, as DEC-028 requires
  (`reader.rs:1-9`). Every existing package, fixture and witness is unchanged,
  which is the most literal reading of "without the selection, the report stays
  local-private exactly as today". Consequence: a reader built before W-18b
  refuses a user-released package as `OPSPROJ_MANIFEST_MALFORMED`
  ("manifest differs from the version 2.0.0 manifest form at: user_release",
  `reader.rs:236-240`), not by version name. No such reader has been released.
- **V2: move to 3.0.0.** The precedent is W-18a, which moved the version when a
  member form changed (W-18a design §2.2). Consequence: the reader refuses
  every 2.0.0 package by name (`OPSPROJ_PACKAGE_VERSION_OLDER`), including
  local-private packages saved since #1616; every non-released package changes
  bytes (its version string); the reader tests, the container tests and the
  headless runner's regenerated failure witnesses move with it.

V1 reads "one version" as allowing an optional, writer-defined record within
that version. That reading is the owner's to confirm.

**Rendered reports.** No JSON schema file describes the renderer input; its
contract is the Rust type (`report_renderer/src/lib.rs:154-163`) and the
TypeScript builder. Changes:
- `ReportExportContext` (`:80-83`) gains `UserReleasedReport`
  (`user_released_report`); `export_context_label` (`:335-340`) gains it;
- `ReportContext` gains `user_release` (§2.2), defaulting to `None`;
- `UserReleaseRecord` is a new typed struct with `deny_unknown_fields`;
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
  new corpus cases in `fixtures/redaction_export_controls/cases.json`, each
  with `explicit_user_release_intent`.
- Under V2, as listed under V2 above.

## 5. The notice

### 5.1 Notice text (proposed exact)

The renderer gains `user_release_notice_text(input) -> Option<String>`, used by
the HTML body (beside `local-private-banner`, `report_renderer/src/lib.rs:1237-1242`)
and the PDF (`pdf_emitter/src/lib.rs:236`). It is emitted only for
`user_released_report` with a record, in a banner of its own,
`<div class="user-release-banner" data-export-context="user_released_report">`:

> USER-RELEASED REPORT — the user released their own project data in this
> report for sharing, stating that they hold the right to redistribute it
> (IP and data boundary §6). The release covers this report only: model
> SHA-256 {model_document_sha256}, result document {result_envelope_id}
> (SHA-256 {result_document_derivative_sha256}), released {released_at}. Rule
> packs and libraries appear only as identity, version and checksum. This
> statement concerns rights in the data only; the analysis status and the
> Professional Boundary notice below are unchanged.

The run-context table (`run_context_rows`, `:342`) shows "Report export
context: user_released_report" and three added rows: "User release statement"
(the statement), "User release time (application clock)" (`released_at`) and
"User release bound to" (the model, run, input-manifest and result-document
identities).

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
(`apps/desktop/src/…`) and as a project document. Result: no finding. No line
carries three litany terms (`:44-56`); the highest count is one
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

**Owner property 1: the flag cannot leak private values.**
- vitest `redactionExportControls.test.ts`:
  - for every corpus item and each of the five existing contexts,
    `explicitUserReleaseIntent: true` throws, so it cannot change a decision;
  - payload-carried `user_release_intent` and `explicit_user_release_intent`, at
    the top level and nested, are stripped and grant nothing: the
    `user_released_report` context without wrapper intent blocks
    (`USER_RELEASE_INTENT_REQUIRED`);
  - in `user_released_report`, each non-project private class is redacted and
    the lossless route is withheld.
- Corpus parity: new cases in `cases.json`, checked by the vitest corpus test,
  `tests/security/test_redaction_export_controls.py` and the Rust mirror test
  `canonical_parity_corpus_matches_rust_contract_mirror`
  (`redaction_binding.rs:702`).
- Rust, Tauri (`report_package_save.rs` tests, sandbox directory, counting
  picker): on route 010, a request carrying `private_material_data` at depth
  (`REPORT-USER-RELEASE-PRIVATE-CLASS`); a spring-hanger quantity from a
  library-referenced support (`REPORT-USER-RELEASE-LIBRARY-VALUE`); a rule-pack
  reference with `private_payload_redacted: false` or an added `formula`
  member (`REPORT-USER-RELEASE-RULE-PACK-DETAIL`). Each is refused before the
  picker, and the directory stays empty.
- Rust, `report_package`: a released request whose rule-pack reference carries
  an extra member with an invented marker string renders HTML and PDF that do
  not contain the marker (the typed boundary).

**Owner property 2: redaction applies to every other context.**
- The 23 existing corpus cases are not edited. A vitest and a pytest pin the
  SHA-256 of their canonical form, so editing an existing expectation fails.
- Existing route tests for `DREP-PACKAGE-SAVE-009`, `DREP-IPC-003` and
  `DREP-HTML-SAVE-005` pass unchanged (`reportRedactionProjector.test.ts`,
  `renderedReport.test.tsx`, `reportPackageSaveService.test.ts`).
- Rust, `report_package`:
  `local_private_package_bytes_unchanged_by_user_release` assembles the
  invented local-private basis and compares its container SHA-256 with the
  value recorded from the parent head before (r2) changes the writer.
- Renderer (`report_renderer/tests/render.rs`): a local-private and a public
  render are byte-identical to the parent head's for the same input.

**Owner property 3: the release is recorded and visible.**
- Renderer: a released input shows the banner in the HTML and in the PDF text,
  with the bound hashes and `released_at`, and the run-context rows; there is no
  local-private banner.
- `report_package` (new `tests/user_release_package.rs`): the manifest's
  `user_release` equals the report context's; the four boundary notes; a
  round trip through the reader returns `ReadReportPackage.user_release`.
- vitest: the dialog's wording, its initial focus on the checkbox (set in
  `useLayoutEffect`), the disabled save until ticked, and the status line and
  receipt fields after a save.

**Owner property 4 and HELP_HUMAN: no replay.** One test per row of §2.5:
- native (`report_package_save.rs` tests): another model's text; another
  result document; another input manifest; a record on route 009; an altered
  statement or standing;
- producer (`tests/user_release_package.rs`): each bound member mismatched
  alone, with every other member coherent; a record under the desktop
  local-private and runner profiles;
- reader (`tests/reader.rs`): the record moved into another valid package and
  the package re-hashed; a record in a blocked package; the user-released
  profile without a record;
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
  read back with `read_report_package_container`, with the record and member
  hashes intact.
- **Native smoke:** a user-released save on the solved demo through
  `native_smoke::take_chosen_path`. The harness checks the file's SHA-256
  against the receipt, the record's model hash against the session model's,
  and the HTML member's banner. It adds no pinned value;
  `apps/desktop/native-smoke/expected.json` is not changed.
- **Claims lint:** `validate_claims_language.py` on each slice's head; the new
  product strings are in scanned `apps/desktop/src` files.
- **Codex scrutiny:** one independent round per slice on its implementation
  head, plus the Appendix A texts as exact text before HELP_HUMAN. Points for
  review: the `unknown` → `warning_only` choice (§3.2), the library rule and its
  stated limit, the reader's equality-only checks, and the naming rule (§0).

## 7. Decisions needed

- **O-A (owner): package version under DEC-028.** V1 (recommended): stay at
  2.0.0 with the optional record; a package without it is unchanged, and a
  pre-W-18b reader refuses a released package as a malformed manifest. V2: move
  to 3.0.0; every 2.0.0 package is refused by name and every package's bytes
  change. §4 gives both in full.
- **O-B (HELP_HUMAN): the release time in the package.** Recommended: record
  `released_at`, and change the third boundary note for released packages only
  (§5.3). Alternative: record no time. Every boundary note then stays as it is,
  but the record has no time. A release is a dated statement, so the time is
  recommended.
- **O-C (HELP_HUMAN): surfaces.** Recommended: the package only (§2.1).
  Alternative: also the standalone rendered HTML. That needs a native save
  command for the rendered HTML, with the package's evidence, bindings and
  screens (about M). The print/PDF view could still not be bound.
- **O-D (HELP_HUMAN): library values.** Recommended: refuse a release that
  would carry a library value (§3.2). Alternative: show such values as
  redaction markers in a released report. That needs renderer marker support
  and a non-lossless package route (option (A)'s work, about M).
- **O-E (wording): rule-check identifiers.** Recommended: user-rule check ids and
  missing-input names stay in released reports as safe metadata under
  REXC-REQ-009. Alternative: omit user-rule outcomes from released reports.
- **Wording grant.** Appendix A, and the product texts of §2.1 and §5, go to
  Codex as exact text and then to HELP_HUMAN.

## 8. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(r1) Vocabulary and redaction** | The context, reason codes and route id in TypeScript, Python, the Rust mirror and the schema; the decision rule; `stripRouteIntent`; the flag assertion; the local-first rule; the projector function (unused until r3); corpus cases and the pinned-digest tests. No route uses the context. | S-M (~600 lines with tests) | Data boundary (vocabulary only) |
| **(r2) Record, renderer, producer, reader, native** | `UserReleaseRecord`; context and blocking codes; notice and rows in HTML and PDF; producer item 9 and the profile; writer manifest and notes; reader checks and refusal; native route-010 branch (§2.3 items 1-5); the render-command refusal; the Rust tests of §6. The desktop does not offer it yet. | M-L (~1.4k) | **Persistence** (manifest and reader) and **data boundary** (native admission) |
| **(r3) Desktop** | Dialog, panel button, menu item (webview and native menu), session wiring and modal guard, builder variant and early refusals, save-service evidence, receipt display, vitest, native smoke step. | M (~900) | Data boundary (webview evidence) and file output |

The order is (r1), (r2), (r3). (r3) offers the user release and needs (r2) in
its ancestry. (r2) and (r3) may land as one PR if review prefers.

## 9. Reconciliation, per slice

Each text changes in the PR of the slice that makes it true. Commitment,
acceptance and ScopeOfWork texts are applied only as Appendix A's reviewed
exact text.
- **(r1)**
  - DEL-12-02 `ScopeOfWork.md` :58, the context vocabulary (A3), and CLM-030
    :414 (A5).
  - `docs/security/redaction_export_controls.md`: the Export Contexts table
    (:59-65) gains the `user_released_report` row, and Final-Sink Route Binding
    (:158-163) names the two new stripped keys.
  - DEL-12-02 `deliverable.yaml` check pointers; `tools/hosted-ci-routing.json`
    for the changed sources and tests.
- **(r2)**
  - DEL-02-05 `ScopeOfWork.md` :214, the reader (A8).
  - DEL-12-02 REXC-OI-007 :486, the native checks (A7).
  - DEL-08-01 `deliverable.yaml` checks (new `tests/user_release_package.rs`);
    `reader.rs` module text (:1-42); CI routing.
  - DEL-08-01 Design `report-package-current-results.md` keeps its text; it
    carries only a pointer to this design.
- **(r3)**
  - DEL-08-01 `ScopeOfWork.md` :81 (A1) and OQ-08-01-003 :383 (A2).
  - DEL-12-02 REXC-REQ-016 after :189 (A4), REXC-OI-002 :481 (A6), CLM-036
    :470 (A9).
  - User guide `docs/user_guide/index.md` :91 (A10), :109-111 (A11) and
    :482-483 (A12). The guide's §10 sentence (:393-395) and its table row
    (:148) stay true: the standalone render stays local private, and the
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
once in it (checked with a Python `count == 1`). Line numbers are where the Old
begins. In an Old or New, `\n` stands for a line break and `\"` for a quotation
mark inside the quoted text. These drafts go to Codex as exact text, then to
HELP_HUMAN under the wording grant. Nothing here is applied.

**A1. DEL-08-01 `ScopeOfWork.md` :81** (current-implementation paragraph, last sentence; slice r3)
- Old: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21)."
- New: "Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21). Save Shareable Report Package… saves the same package for sharing after the user's per-report release of their own project data (route `DREP-PACKAGE-USER-RELEASE-010`, export context `user_released_report`; DEL-08-01 Design `report-user-release.md`): the release record in `report_context.user_release` and in the package manifest states the user's redistribution right and binds the model document SHA-256, the run's input manifest and the result document; rule-pack and library content stays at safe metadata; and the rendered members carry the user-release notice in place of the local-private notice. Without that selection the package is local private as above, and the selection is not stored."

**A2. DEL-08-01 `ScopeOfWork.md` :383** (OQ-08-01-003, last cell; slice r3)
- Old: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. |"
- New: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. A shared report carries private project data only through the user's per-report release of their own data (DEL-12-02 `user_released_report`; DEL-08-01 Design `report-user-release.md`). |"

**A3. DEL-12-02 `ScopeOfWork.md` :58** (CLM-003; slice r1)
- Old: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private` |"
- New: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private`, `user_released_report` |"

**A4. DEL-12-02 `ScopeOfWork.md` :189** (CLM-012; a row added after REXC-REQ-015; slice r3)
- Old: "> | REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |"
- New: the Old line unchanged, followed by this line:
  "> | REXC-REQ-016 | A shareable report may include the user's own private project data only after the user's explicit release of that data for that one report, with the user's statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). The export context is `user_released_report`; the release is recorded in the report and its package, bound to the model and result it covers, and stated in the report's notice; it is not stored for later reports; private rule-pack and library values stay at safe metadata; protected, quarantined and rejected content still blocks; every other export context is unchanged. | OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` §6; owner, 2026-10-24 (W-18 option B) | Focused tests: the release flag changes no decision in another context; redaction is unchanged in every other context; the record is visible in the report and the package; a record copied to another model, result or package is refused. |"

**A5. DEL-12-02 `ScopeOfWork.md` :414** (CLM-030, last sentence; slice r1)
- Old: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks."
- New: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks.\n> `user_released_report` carries the user's own project data in one report only\n> after the user's explicit release of it, with their statement of the\n> redistribution right; private rule-pack and library values stay at safe\n> metadata, protected, quarantined and rejected content still blocks, and the\n> release is recorded in the report."

**A6. DEL-12-02 `ScopeOfWork.md` :481** (REXC-OI-002; slice r3)
- Old: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent."
- New: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent. The desktop's per-report user release (`DREP-PACKAGE-USER-RELEASE-010`, `user_released_report`) is selected in its own dialog for one save and is not stored."

**A7. DEL-12-02 `ScopeOfWork.md` :486** (REXC-OI-007; slice r2)
- Old: "before the save dialog; caller evidence alone admits nothing."
- New: "before the save dialog; caller evidence alone admits nothing. On its user-release route (`DREP-PACKAGE-USER-RELEASE-010`) it admits the `user_released_report` context only with a release record whose bound model, input-manifest and result-document identities equal the texts and members it received; it refuses a record on the local-private route, a value from a library-referenced model object, and any object with a non-project private class."

**A8. DEL-02-05 `ScopeOfWork.md` :214** (reader; slice r2)
- Old: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value;"
- New: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value; when the manifest records a user release, it checks the record's form and the equality of its bound model hash, input-manifest hash and result-document identity with the audit and result members, and refuses a record in a blocked package or under another export profile;"

**A9. DEL-12-02 `ScopeOfWork.md` :470** (CLM-036, a row added after "Local private report"; slice r3)
- Old: "> | Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |"
- New: the Old line unchanged, followed by this line:
  "> | User-released report | The user's own project values in one report after the explicit per-report release, recorded in the report and stated in its notice; rule-pack and library content at safe metadata | A release carried over to another model, result or report, or remembered for later reports |"

**A10. User guide `docs/user_guide/index.md` :91** (slice r3)
- Old: "- **Reports are local-private only, and pressure-free only.**"
- New: "- **Reports are local-private unless you release one for sharing, and pressure-free only.**"

**A11. User guide `docs/user_guide/index.md` :109** (a bullet added; slice r3)
- Old: "  - Save Report Package… saves a local private package for a pressure-free\n    result after its own local-private selection; exact-route results are\n    refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`."
- New: the Old bullet unchanged, followed by:
  "\n  - Save Shareable Report Package… saves the same package for sharing after\n    you tick, in its dialog, \"I hold the right to share this project's data,\n    and I release it in this report for sharing.\" The release covers that one\n    report. Its documents carry a \"USER-RELEASED REPORT\" notice naming the\n    model and result it covers, rule packs and libraries appear only as\n    identity, version and checksum, and your next report is local private\n    again. A report that would show a value taken from a library record is\n    refused."

**A12. User guide `docs/user_guide/index.md` :482** (limitations list; slice r3)
- Old: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents and only for pressure-free results,"
- New: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents or, one at a time, released by you for sharing, and only for pressure-free results,"
