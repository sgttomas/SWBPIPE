import { useEffect, useState } from "react";
import { Library, ShieldAlert } from "lucide-react";
import type { PreviewModel } from "../../types";
import {
  buildInventedLibraryImportTemplate,
  deleteLocalLibrary,
  listLocalLibraries,
  openLocalLibrary,
  partitionLibraryImportFindings,
  saveLocalLibrary,
  validateLibraryImport,
  type IntendedVisibility,
  type LibraryImportFinding,
  type LibraryImportValidation,
  type LibraryKind,
  type LocalLibraryIndexEntry
} from "../../services/libraryImportService";

// Private Library Manager (PRD §13 / §14.6 / §14.1 "Library manager"). Phase C3
// GUI slice (TP-C3-LIBGUI-001), replicating the C2 Rule-Pack Manager pattern:
// import an already-parsed material/section/component library document, validate
// it through the DEL-03-07 provenance seam (the desktop `validate_library_import`
// command), and persist accepted private imports to the local-only store
// (`save_local_library`). The findings display is split along the PRD §13.5
// blocking-vs-advisory axis. The local store is private-by-default and persists
// only an accepted (PRIVATE_LOCAL_ONLY) import — a suspected-protected
// (QUARANTINE) or otherwise-blocked (REJECTED) import is refused, never written
// (DEC-036). Imported private libraries are never transmitted, committed, or
// bundled into public artifacts (OPS-K-PRIV-1, PRD §13.5/§17.3). Every status is
// a software finding only — never a legal, certification, sealing,
// authentication, or code-compliance determination.
//
// The backend runs in the desktop (Tauri) runtime only; browser preview reports
// the explicit unavailable route rather than synthesizing a fallback validator
// or store. Reference wiring between rule packs and imported library allowables
// is a separate Phase C3 slice and is intentionally absent here.

const NO_DRAFT_REASON =
  "No import document; load an invented sample or paste a library document first.";
const NO_PROJECT_REASON =
  "Local libraries are project-scoped; create or open a local project first.";
const BUSY_REASON =
  "A library-import backend request is in progress; wait for it to finish before editing or running another action.";

const LIBRARY_KINDS: ReadonlyArray<{ value: LibraryKind; label: string }> = [
  { value: "material", label: "Material library" },
  { value: "section", label: "Section library" },
  { value: "component", label: "Component library" }
];

const VISIBILITY_OPTIONS: ReadonlyArray<{ value: IntendedVisibility; label: string }> = [
  { value: "private", label: "Private (local-only)" },
  { value: "public", label: "Public (validation preview)" }
];

type DraftState = {
  text: string;
  origin: string;
};

export function LibraryManagerPanel({ model }: { model: PreviewModel | null }) {
  const [draft, setDraft] = useState<DraftState | null>(null);
  const [libraryKind, setLibraryKind] = useState<LibraryKind>("material");
  const [intendedVisibility, setIntendedVisibility] = useState<IntendedVisibility>("private");
  const [actionStatus, setActionStatus] = useState<string>(
    "No library-import action has run in this session."
  );
  const [listStatus, setListStatus] = useState<string>("Local library list not refreshed yet.");
  const [entries, setEntries] = useState<LocalLibraryIndexEntry[]>([]);
  const [validation, setValidation] = useState<LibraryImportValidation | null>(null);
  // In-request busy guard: while a backend command is awaiting, the draft
  // textarea, the kind/visibility selectors, and every action are disabled so
  // an async response that calls setDraft (open) cannot clobber a mid-request
  // edit — mirrors the C2 rule-pack manager's slice-1 residual fix.
  const [inFlight, setInFlight] = useState(false);

  const projectId = model?.project.id ?? null;
  const draftReason = !draft ? NO_DRAFT_REASON : inFlight ? BUSY_REASON : undefined;

  // The stored-library list is project-scoped. When the active project changes
  // (open/create/switch), the previously-listed libraries belong to the old
  // project and would contradict the scope banner, so clear them; the session
  // draft is intentionally preserved (it is authored, not bound to a project
  // until saved).
  useEffect(() => {
    setEntries([]);
    setListStatus("Local library list not refreshed for this project yet.");
  }, [projectId]);

  // Backend (Tauri) commands can reject in desktop mode. Browser preview never
  // reaches them (the services short-circuit to an explicit unavailable route),
  // but a desktop backend error must surface honestly rather than become a
  // silent unhandled rejection.
  function reportBackendError(action: string, error: unknown): void {
    setActionStatus(`LIBRARY-IMPORT-BACKEND-ERROR (${action}): ${String(error)}`);
  }

  function parseDraft(): Record<string, unknown> | null {
    if (!draft) return null;
    try {
      const parsed: unknown = JSON.parse(draft.text);
      if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
        setActionStatus("LIBRARY-IMPORT-DRAFT-JSON-INVALID: the import document must be a JSON object.");
        return null;
      }
      return parsed as Record<string, unknown>;
    } catch (error) {
      setActionStatus(`LIBRARY-IMPORT-DRAFT-JSON-INVALID: ${String(error)}`);
      return null;
    }
  }

  function handleLoadTemplate() {
    const document = buildInventedLibraryImportTemplate(libraryKind);
    setDraft({
      text: JSON.stringify(document, null, 2),
      origin: `invented_${libraryKind}_template_private_by_default`
    });
    setValidation(null);
    setActionStatus(
      `Invented private ${libraryKind}-library template loaded in memory ` +
        "(privacy_class=private_user_data, redistribution_status=private_only). " +
        "Nothing is stored until you validate and save."
    );
  }

  function handleDiscardDraft() {
    setDraft(null);
    setValidation(null);
    setActionStatus("Import document discarded from session memory; the local store was not touched.");
  }

  async function handleRefreshList() {
    setInFlight(true);
    try {
      const route = await listLocalLibraries(projectId);
      if (route.route === "unavailable_browser_preview") {
        setListStatus(route.diagnostic);
        setEntries([]);
        return;
      }
      setEntries(route.entries);
      setListStatus(
        `Local store listed ${route.entries.length} library(ies)` +
          (projectId ? ` for ${projectId}.` : " across all local projects.")
      );
    } catch (error) {
      setEntries([]);
      setListStatus(`LIBRARY-IMPORT-BACKEND-ERROR (list): ${String(error)}`);
    } finally {
      setInFlight(false);
    }
  }

  async function handleValidate() {
    const document = parseDraft();
    if (!document) return;
    setInFlight(true);
    try {
      const route = await validateLibraryImport(document, libraryKind, intendedVisibility);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.validation);
      const { blocking, advisory } = partitionLibraryImportFindings(route.validation);
      setActionStatus(
        `Validation completed: outcome=${route.validation.outcome}; ` +
          `accepted=${String(route.validation.accepted)}; ` +
          `blocking_findings=${blocking.length}; advisory_findings=${advisory.length}.`
      );
    } catch (error) {
      reportBackendError("validate", error);
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
      const route = await saveLocalLibrary(projectId, libraryKind, document);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.result.validation);
      const refusedNote = route.result.stored
        ? ""
        : " Import not stored: validation findings block this library (DEC-036 refuse-to-store).";
      setActionStatus(
        `${route.result.message} (library_id=${route.result.library_id}, ` +
          `stored=${String(route.result.stored)}, storage=${route.result.storage_mode}).${refusedNote}`
      );
      await handleRefreshList();
    } catch (error) {
      reportBackendError("save", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleOpen(entry: LocalLibraryIndexEntry) {
    setInFlight(true);
    try {
      const route = await openLocalLibrary(entry.project_id, entry.library_kind, entry.library_id);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      if (!route.envelope) {
        setActionStatus(
          `No stored library matched ${entry.library_id}; refresh the list and retry.`
        );
        return;
      }
      setLibraryKind(route.envelope.library_kind);
      setDraft({
        text: JSON.stringify(route.envelope.document, null, 2),
        origin: `opened_from_local_store:${entry.library_id}`
      });
      setValidation(route.envelope.validation);
      setActionStatus(`${route.envelope.message} (library_id=${route.envelope.library_id}).`);
    } catch (error) {
      reportBackendError("open", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleDelete(entry: LocalLibraryIndexEntry) {
    setInFlight(true);
    try {
      const route = await deleteLocalLibrary(entry.project_id, entry.library_kind, entry.library_id);
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

  return (
    <section
      className="panel library-manager-panel"
      aria-label="Library manager"
      data-testid="library-manager-panel"
    >
      <div className="panel-title">
        <Library size={16} aria-hidden="true" />
        Private Library Manager (local-only)
      </div>

      <div className="report-list">
        <small data-testid="library-scope-status">
          {projectId
            ? `Project scope: ${projectId}; storage: local SQLite only.`
            : NO_PROJECT_REASON}
        </small>
      </div>

      <div className="report-list">
        <label htmlFor="library-kind-select">Library kind</label>
        <select
          id="library-kind-select"
          data-testid="library-kind-select"
          value={libraryKind}
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onChange={(event) => setLibraryKind(event.target.value as LibraryKind)}
        >
          {LIBRARY_KINDS.map((kind) => (
            <option key={kind.value} value={kind.value}>
              {kind.label}
            </option>
          ))}
        </select>
        <label htmlFor="library-visibility-select">Intended visibility (validation preview)</label>
        <select
          id="library-visibility-select"
          data-testid="library-visibility-select"
          value={intendedVisibility}
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onChange={(event) => setIntendedVisibility(event.target.value as IntendedVisibility)}
        >
          {VISIBILITY_OPTIONS.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
        <small data-testid="library-visibility-note">
          Visibility shapes the validation preview only. Save always persists to the private
          local store and stores only an accepted private import (DEC-036).
        </small>
      </div>

      <div className="report-actions">
        <button
          type="button"
          data-testid="library-load-template"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={handleLoadTemplate}
        >
          Load invented sample
        </button>
        <button
          type="button"
          data-testid="library-refresh-list"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={() => void handleRefreshList()}
        >
          Refresh local list
        </button>
      </div>

      <div className="report-list" data-testid="library-list">
        <small data-testid="library-list-status">{listStatus}</small>
        {entries.map((entry) => (
          <article
            className="operation-record"
            data-testid={`library-entry-${entry.library_id}`}
            key={`${entry.project_id}:${entry.library_kind}:${entry.library_id}`}
          >
            <strong>
              {entry.library_name} ({entry.library_id})
            </strong>
            <small>
              kind={entry.library_kind}; privacy={entry.privacy_class}; storage={entry.storage_mode}
            </small>
            <div className="report-actions">
              <button
                type="button"
                data-testid={`library-open-${entry.library_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleOpen(entry)}
              >
                Open
              </button>
              <button
                type="button"
                data-testid={`library-delete-${entry.library_id}`}
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

      <div className="report-list">
        <label htmlFor="library-draft-json-input">
          Library import document JSON (an already-parsed material/section/component library
          document with provenance — this manager validates and stores it; it does not parse
          external file formats)
        </label>
        <textarea
          id="library-draft-json-input"
          data-testid="library-draft-json"
          rows={14}
          value={draft?.text ?? ""}
          placeholder="No import document. Load an invented sample or paste a library document."
          disabled={!draft || inFlight}
          title={draftReason}
          onChange={(event) =>
            setDraft((current) => (current ? { ...current, text: event.target.value } : current))
          }
        />
        <div className="report-actions">
          <button
            type="button"
            data-testid="library-validate"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleValidate()}
          >
            Validate import
          </button>
          <button
            type="button"
            data-testid="library-save"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleSave()}
          >
            Save to local store
          </button>
          <button
            type="button"
            data-testid="library-discard-draft"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={handleDiscardDraft}
          >
            Discard import document
          </button>
        </div>
        <small data-testid="library-action-status">{actionStatus}</small>
      </div>

      {validation ? (
        <div className="report-list" data-testid="library-validation">
          <strong>Import validation</strong>
          <small data-testid="library-validation-summary">
            outcome={validation.outcome}; accepted={String(validation.accepted)};
            blocking_findings={String(validation.has_blocking_findings)}; library_kind=
            {validation.library_kind}; intended_visibility={validation.intended_visibility}
          </small>
          <LibraryFindingsList validation={validation} />
          <small>{validation.professional_boundary_notice}</small>
        </div>
      ) : null}

      <small className="report-note" data-testid="library-boundary-note">
        <ShieldAlert size={12} aria-hidden="true" /> Imported private libraries stay in local
        project storage only — never committed to the repository, transmitted, or bundled into
        public artifacts. Only an accepted private import is stored; a suspected-protected or
        otherwise-blocked import is refused, not stored (DEC-036). Import validation reports
        software findings over an already-parsed payload — never a legal, redistribution,
        certification, sealing, approval, or professional acceptance claim.
      </small>
    </section>
  );
}

// Findings rendered along the PRD §13.5 blocking-vs-advisory axis. `blocking`
// holds blocking/quarantine findings (the import cannot be accepted as-is);
// `advisory` holds review_required findings (human review before acceptance).
function LibraryFindingsList({ validation }: { validation: LibraryImportValidation }) {
  const { blocking, advisory } = partitionLibraryImportFindings(validation);
  return (
    <>
      <div className="operation-record-list" data-testid="library-findings-blocking">
        <small>
          Blocking / quarantine findings ({blocking.length}) — these block import acceptance.
        </small>
        {blocking.map((finding, index) => (
          <LibraryFinding finding={finding} key={`blocking:${finding.code}:${finding.path}:${index}`} />
        ))}
      </div>
      <div className="operation-record-list" data-testid="library-findings-advisory">
        <small>
          Advisory findings ({advisory.length}) — human review before acceptance, not blocking.
        </small>
        {advisory.map((finding, index) => (
          <LibraryFinding finding={finding} key={`advisory:${finding.code}:${finding.path}:${index}`} />
        ))}
      </div>
    </>
  );
}

function LibraryFinding({ finding }: { finding: LibraryImportFinding }) {
  return (
    <article className="operation-record">
      <strong>{finding.code}</strong>
      <small>
        severity={finding.severity}; path={finding.path}
      </small>
      <p>{finding.message}</p>
      <small>Remediation: {finding.remediation}</small>
    </article>
  );
}
