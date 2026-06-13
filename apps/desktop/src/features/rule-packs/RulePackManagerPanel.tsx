import { useEffect, useState } from "react";
import { FileText, ShieldAlert } from "lucide-react";
import type { PreviewModel } from "../../types";
import {
  buildDraftRulePackDocument,
  computeRulePackChecksum,
  deleteLocalRulePack,
  listLocalRulePacks,
  openLocalRulePack,
  saveLocalRulePack,
  stampChecksumIntoDocument,
  validateRulePack,
  type LocalRulePackIndexEntry,
  type RulePackComputedChecksum,
  type RulePackDocument,
  type RulePackDocumentValidation,
  type RulePackValidationFinding
} from "../../services/rulePackService";
import { ExpressionComposer, parseRulePackDocument } from "./ExpressionComposer";
import { DeclarationsEditor } from "./DeclarationsEditor";
import { CheckDefinitionsEditor } from "./CheckDefinitionsEditor";

// Rule-Pack Manager (PRD §14.5 / §14.1 "Rule-pack manager"). Pack management
// and document authoring against the local-only store: list/create/open/
// save/delete, validation findings, and checksum generation/stamping.
//
// Slice 1 (TP-C2-EDITOR-001) authored the whole document as raw canonical
// JSON. Slice 2 (TP-C2-COMPOSER-001) added the structured AST expression
// composer (PRD §14.5 "Expression editor"); slice 3 (TP-C2-TABLENODE-001)
// extends it to the table-backed nodes, so the full grammar v1.0.0 node set
// (DEC-022) is built through form/tree controls rather than hand-written JSON.
// Slice 4 (TP-C2-DECLEDITOR-001) adds the variable-declarations editor for the
// `required_inputs` and `value_slots` the composer's variable picker binds to.
// Slice 5 (TP-C2-CHECKDEF-001) adds the check-definitions editor, which binds
// those declarations and a formula into each acceptability check — the last
// document-structure member that had no form builder. The raw document JSON
// stays as the canonical/fallback surface for the remaining metadata members
// (diagnostics, classification, checksums, provenance, professional_boundary,
// open_decisions). No expression text syntax is provided anywhere — D-02b
// awaits the human ruling.

const NO_DRAFT_REASON = "No draft rule-pack document; create a new draft or open a stored pack first.";
const NO_PROJECT_REASON = "Local rule packs are project-scoped; create or open a local project first.";
const BUSY_REASON =
  "A rule-pack backend request is in progress; wait for it to finish before editing or running another action.";

type DraftState = {
  text: string;
  origin: string;
};

export function RulePackManagerPanel({ model }: { model: PreviewModel | null }) {
  const [draft, setDraft] = useState<DraftState | null>(null);
  const [actionStatus, setActionStatus] = useState<string>(
    "No rule-pack action has run in this session."
  );
  const [listStatus, setListStatus] = useState<string>("Local rule-pack list not refreshed yet.");
  const [entries, setEntries] = useState<LocalRulePackIndexEntry[]>([]);
  const [validation, setValidation] = useState<RulePackDocumentValidation | null>(null);
  const [checksum, setChecksum] = useState<RulePackComputedChecksum | null>(null);
  // In-request busy guard: while a backend command is awaiting, the draft
  // editors (document textarea + structured composer) and every action are
  // disabled so an async response that calls setDraft (compute-checksum,
  // open) cannot clobber a mid-request edit. Routed here from the slice-1
  // residual ("a mid-request textarea edit can be clobbered").
  const [inFlight, setInFlight] = useState(false);

  const projectId = model?.project.id ?? null;
  const draftReason = !draft ? NO_DRAFT_REASON : inFlight ? BUSY_REASON : undefined;
  const parsedDraft = draft ? parseRulePackDocument(draft.text) : null;

  // The stored-pack list is project-scoped. When the active project changes
  // (open/create/switch), the previously-listed packs belong to the old
  // project and would contradict the scope banner, so clear them; the
  // session draft is intentionally preserved (it is authored, not bound to a
  // project until saved).
  useEffect(() => {
    setEntries([]);
    setListStatus("Local rule-pack list not refreshed for this project yet.");
  }, [projectId]);

  // Backend (Tauri) commands can reject in desktop mode. Browser preview
  // never reaches them (the services short-circuit to an explicit
  // unavailable route), but a desktop backend error must surface honestly
  // rather than become a silent unhandled rejection.
  function reportBackendError(action: string, error: unknown): void {
    setActionStatus(`RULE-PACK-BACKEND-ERROR (${action}): ${String(error)}`);
  }

  function parseDraft(): RulePackDocument | null {
    if (!draft) return null;
    try {
      const parsed: unknown = JSON.parse(draft.text);
      if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
        setActionStatus("RULE-PACK-DRAFT-JSON-INVALID: the draft must be a JSON object.");
        return null;
      }
      return parsed as RulePackDocument;
    } catch (error) {
      setActionStatus(`RULE-PACK-DRAFT-JSON-INVALID: ${String(error)}`);
      return null;
    }
  }

  function handleNewDraft() {
    const document = buildDraftRulePackDocument();
    setDraft({
      text: JSON.stringify(document, null, 2),
      origin: "new_draft_private_by_default"
    });
    setValidation(null);
    setChecksum(null);
    setActionStatus(
      "New private draft created in memory (privacy_class=private_user_data, " +
        "redistribution_status=private_only). Nothing is stored until you save."
    );
  }

  function handleDiscardDraft() {
    setDraft(null);
    setValidation(null);
    setChecksum(null);
    setActionStatus("Draft discarded from session memory; the local store was not touched.");
  }

  async function handleRefreshList() {
    setInFlight(true);
    try {
      const route = await listLocalRulePacks(projectId);
      if (route.route === "unavailable_browser_preview") {
        setListStatus(route.diagnostic);
        setEntries([]);
        return;
      }
      setEntries(route.entries);
      setListStatus(
        `Local store listed ${route.entries.length} rule pack(s)` +
          (projectId ? ` for ${projectId}.` : " across all local projects.")
      );
    } catch (error) {
      setEntries([]);
      setListStatus(`RULE-PACK-BACKEND-ERROR (list): ${String(error)}`);
    } finally {
      setInFlight(false);
    }
  }

  async function handleValidate() {
    const document = parseDraft();
    if (!document) return;
    setInFlight(true);
    try {
      const route = await validateRulePack(document);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.validation);
      setActionStatus(
        `Validation completed: blocking_findings=${String(route.validation.has_blocking_findings)}; ` +
          `checksum=${route.validation.checksum.match_status}; ` +
          `grammar_supported=${String(route.validation.grammar_version_supported)}.`
      );
    } catch (error) {
      reportBackendError("validate", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleComputeChecksum() {
    const document = parseDraft();
    if (!document) return;
    setInFlight(true);
    try {
      const route = await computeRulePackChecksum(document);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setChecksum(route.computed);
      const stamped = stampChecksumIntoDocument(document, route.computed);
      setDraft({ text: JSON.stringify(stamped, null, 2), origin: "checksum_stamped" });
      setActionStatus(
        `Checksum computed and stamped into the draft (${route.computed.basis}; ` +
          `payload_excludes=${route.computed.payload_excludes.join(",")}).`
      );
    } catch (error) {
      reportBackendError("compute_checksum", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleSave() {
    if (!projectId) {
      setActionStatus(NO_PROJECT_REASON);
      return;
    }
    const document = parseDraft();
    if (!document) return;
    setInFlight(true);
    try {
      const route = await saveLocalRulePack(projectId, document);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.envelope.validation);
      setActionStatus(
        `${route.envelope.message} (rule_pack_id=${route.envelope.rule_pack_id}, ` +
          `storage=${route.envelope.storage_mode}).`
      );
      await handleRefreshList();
    } catch (error) {
      reportBackendError("save", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleOpen(entry: LocalRulePackIndexEntry) {
    setInFlight(true);
    try {
      const route = await openLocalRulePack(entry.project_id, entry.rule_pack_id);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      if (!route.envelope) {
        setActionStatus(
          `No stored rule pack matched ${entry.rule_pack_id}; refresh the list and retry.`
        );
        return;
      }
      setDraft({
        text: JSON.stringify(route.envelope.document, null, 2),
        origin: `opened_from_local_store:${entry.rule_pack_id}`
      });
      setValidation(route.envelope.validation);
      setChecksum(null);
      setActionStatus(`${route.envelope.message} (rule_pack_id=${route.envelope.rule_pack_id}).`);
    } catch (error) {
      reportBackendError("open", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleDelete(entry: LocalRulePackIndexEntry) {
    setInFlight(true);
    try {
      const route = await deleteLocalRulePack(entry.project_id, entry.rule_pack_id);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setActionStatus(route.receipt.message);
      await handleRefreshList();
    } catch (error) {
      reportBackendError("delete", error);
    } finally {
      setInFlight(false);
    }
  }

  // Structured editors → document: re-serialize the edited document back into
  // the canonical draft text (the single source of truth the validate/
  // checksum/save flow already reads). The composer edits a formula's
  // expression AST; the declarations editor edits the required_inputs /
  // value_slots arrays; the check-definitions editor binds those into checks.
  function handleComposerChange(nextDocument: Record<string, unknown>) {
    setDraft({ text: JSON.stringify(nextDocument, null, 2), origin: "composer_edit" });
  }

  function handleDeclarationsChange(nextDocument: Record<string, unknown>) {
    setDraft({ text: JSON.stringify(nextDocument, null, 2), origin: "declarations_edit" });
  }

  function handleCheckDefinitionsChange(nextDocument: Record<string, unknown>) {
    setDraft({ text: JSON.stringify(nextDocument, null, 2), origin: "check_definitions_edit" });
  }

  return (
    <section
      className="panel rule-pack-manager-panel"
      aria-label="Rule-pack manager"
      data-testid="rule-pack-manager-panel"
    >
      <div className="panel-title">
        <FileText size={16} aria-hidden="true" />
        Rule-Pack Manager (private, local-only)
      </div>

      <div className="report-list">
        <small data-testid="rule-pack-scope-status">
          {projectId
            ? `Project scope: ${projectId}; storage: local SQLite only.`
            : NO_PROJECT_REASON}
        </small>
      </div>

      <div className="report-actions">
        <button
          type="button"
          data-testid="rule-pack-new-draft"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={handleNewDraft}
        >
          New draft rule pack
        </button>
        <button
          type="button"
          data-testid="rule-pack-refresh-list"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={() => void handleRefreshList()}
        >
          Refresh local list
        </button>
      </div>

      <div className="report-list" data-testid="rule-pack-list">
        <small data-testid="rule-pack-list-status">{listStatus}</small>
        {entries.map((entry) => (
          <article
            className="operation-record"
            data-testid={`rule-pack-entry-${entry.rule_pack_id}`}
            key={`${entry.project_id}:${entry.rule_pack_id}`}
          >
            <strong>
              {entry.rule_pack_name} ({entry.rule_pack_id} v{entry.rule_pack_version})
            </strong>
            <small>
              lifecycle={entry.lifecycle_status}; privacy={entry.privacy_class}; storage=
              {entry.storage_mode}
            </small>
            <div className="report-actions">
              <button
                type="button"
                data-testid={`rule-pack-open-${entry.rule_pack_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleOpen(entry)}
              >
                Open
              </button>
              <button
                type="button"
                data-testid={`rule-pack-delete-${entry.rule_pack_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleDelete(entry)}
              >
                Delete
              </button>
            </div>
          </article>
        ))}
      </div>

      {draft ? (
        parsedDraft && parsedDraft.ok ? (
          <>
            <DeclarationsEditor
              document={parsedDraft.document}
              onChange={handleDeclarationsChange}
              disabled={inFlight}
              disabledReason={inFlight ? BUSY_REASON : undefined}
            />
            <ExpressionComposer
              document={parsedDraft.document}
              onChange={handleComposerChange}
              disabled={inFlight}
              disabledReason={inFlight ? BUSY_REASON : undefined}
            />
            <CheckDefinitionsEditor
              document={parsedDraft.document}
              onChange={handleCheckDefinitionsChange}
              disabled={inFlight}
              disabledReason={inFlight ? BUSY_REASON : undefined}
            />
          </>
        ) : (
          <div className="report-list">
            <small data-testid="rule-pack-composer-unavailable">
              The document JSON below is not valid, so the structured expression composer cannot
              read it. Fix the JSON to compose expressions structurally.
            </small>
          </div>
        )
      ) : (
        <div className="report-list">
          <small data-testid="rule-pack-composer-no-draft">
            Create a new draft or open a stored rule pack to compose its expressions.
          </small>
        </div>
      )}

      <div className="report-list">
        <label htmlFor="rule-pack-draft-json-input">
          Rule-pack document JSON (native canonical form; the editors above author the
          required-input/value-slot declarations, the selected formula&apos;s full expression AST
          — declarative-AST per DEC-022, including table-backed nodes — and the acceptability
          check definitions. Edit the remaining metadata members here — diagnostics,
          classification, checksums, provenance, and professional-boundary controls.)
        </label>
        <textarea
          id="rule-pack-draft-json-input"
          data-testid="rule-pack-draft-json"
          rows={14}
          value={draft?.text ?? ""}
          placeholder="No draft document. Create a new draft or open a stored rule pack."
          disabled={!draft || inFlight}
          title={draftReason}
          onChange={(event) =>
            setDraft((current) =>
              current ? { ...current, text: event.target.value } : current
            )
          }
        />
        <div className="report-actions">
          <button
            type="button"
            data-testid="rule-pack-validate"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleValidate()}
          >
            Validate document
          </button>
          <button
            type="button"
            data-testid="rule-pack-compute-checksum"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleComputeChecksum()}
          >
            Compute &amp; stamp checksum
          </button>
          <button
            type="button"
            data-testid="rule-pack-save"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleSave()}
          >
            Save to local store
          </button>
          <button
            type="button"
            data-testid="rule-pack-discard-draft"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={handleDiscardDraft}
          >
            Discard draft
          </button>
        </div>
        <small data-testid="rule-pack-action-status">{actionStatus}</small>
      </div>

      {checksum ? (
        <div className="report-list" data-testid="rule-pack-checksum">
          <strong>Computed checksum</strong>
          <small>
            {checksum.algorithm}; canonicalization={checksum.canonicalization}; basis=
            {checksum.basis}; grammar_version={checksum.grammar_version} (bound=
            {String(checksum.grammar_version_bound)})
          </small>
          <small data-testid="rule-pack-checksum-value">{checksum.value}</small>
        </div>
      ) : null}

      {validation ? (
        <div className="report-list" data-testid="rule-pack-validation">
          <strong>Document validation</strong>
          <small data-testid="rule-pack-validation-summary">
            blocking_findings={String(validation.has_blocking_findings)}; lifecycle_blocked=
            {String(validation.lifecycle_blocked)}; checksum={validation.checksum.match_status};
            grammar={validation.grammar_version} supported=
            {String(validation.grammar_version_supported)}; publicly_exportable=
            {String(validation.publicly_exportable)}
          </small>
          <div className="operation-record-list" data-testid="rule-pack-validation-findings">
            {collectFindings(validation).map((finding, index) => (
              <article
                className="operation-record"
                key={`${finding.code}:${finding.subject}:${index}`}
              >
                <strong>{finding.code}</strong>
                <small>
                  subject={finding.subject}; blocking={String(finding.blocking)}
                </small>
                <p>{finding.message}</p>
              </article>
            ))}
          </div>
          <small>{validation.professional_boundary_notice}</small>
        </div>
      ) : null}

      <small className="report-note" data-testid="rule-pack-boundary-note">
        <ShieldAlert size={12} aria-hidden="true" /> Private rule packs stay in local project
        storage only — never committed to the repository, transmitted, or bundled into public
        artifacts. Expressions are composed as a structured AST (DEC-022 grammar v1.0.0); no
        expression text syntax is provided until the D-02b human ruling. Rule-check output is a
        software computation over user-supplied data, never a code-compliance, certification,
        sealing, approval, or professional acceptance claim.
      </small>
    </section>
  );
}

function collectFindings(validation: RulePackDocumentValidation): RulePackValidationFinding[] {
  return [
    ...validation.document_findings,
    ...validation.lifecycle_findings,
    ...validation.formulas.flatMap((formula) => formula.findings)
  ];
}
