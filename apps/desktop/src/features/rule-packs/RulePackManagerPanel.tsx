import { C2ResultGate } from "../results/C2ResultGate";
import type { SessionAnalysisRun, SessionResult } from "../../services/nativeResult";
import { useEffect, useRef, useState, type ReactNode } from "react";
import { FileText, ShieldAlert } from "lucide-react";
import type { EditorOperationIntent, MechanicsResult, PreviewModel } from "../../types";
import { RulePackModelReferences } from "../model-data/RulePackModelReferences";
import {
  buildDraftRulePackDocument,
  computeRulePackChecksum,
  deleteLocalRulePack,
  listLocalRulePacks,
  openLocalRulePack,
  RULE_PACK_SCHEMA_VERSION,
  RULE_PACK_SCHEMA_VERSION_UNSUPPORTED,
  saveLocalRulePack,
  stampChecksumIntoDocument,
  validateRulePack,
  type LocalRulePackEnvelope,
  type LocalRulePackIndexEntry,
  type RulePackComputedChecksum,
  type RulePackDocument,
  type RulePackDocumentValidation,
  type RulePackListRoute,
  type RulePackUnitCheckFinding,
  type RulePackUnitCheckOperand,
  type RulePackValidationFinding
} from "../../services/rulePackService";
import { ExpressionComposer, parseRulePackDocument, readFormulaDeclarations, renderExpressionText } from "./ExpressionComposer";
import { DeclarationsEditor, LoadCaseCategoriesEditor } from "./DeclarationsEditor";
import { CheckDefinitionsEditor } from "./CheckDefinitionsEditor";
import { LIFECYCLE_WORDS, PAGE_WORDS, PART_WORDS, countLine, thingsToFix, wordFor } from "./rulePackWords";

// Rule-Pack Manager (PRD §14.5 / §14.1 "Rule-pack manager"). Pack management
// and document authoring against the local-only store: list/create/open/
// save/delete, validation findings, and checksum generation/stamping.
//
// The page reads top to bottom as a person uses it: the project's packs as a
// short list (read when the page opens), one button to start a pack, and,
// only once a pack is open, its name and version, the save controls in the
// order they are pressed with the findings in words beside them, and the
// pack's parts as closed lines. Each closed line carries a summary a person
// can read without opening it, and the words of any fault the last check
// found in it. Every label and option is the product's word for the format's
// field or token (rulePackWords.ts); the format name stays findable as the
// control's title. Codes, key=value lines and the document itself sit in
// closed "Details" and "Document (JSON)" disclosures.
//
// The editors compose the whole document through structured controls:
// required load cases, stress categories and mappings; required inputs and
// value slots; the formula's expression AST (DEC-022 grammar, no writable text
// syntax: DEC-037 permits only the read-only preview); and the check
// definitions. The raw document JSON stays the canonical surface for the
// remaining metadata members. A stored pack in a format other than 0.5.0
// opens with its refusal shown and offers delete only: there is no migration.

const NO_DRAFT_REASON = "No rule pack is open; start a new one or open a saved one first.";
const BUSY_REASON =
  "A rule-pack request is in progress; wait for it to finish before editing or running another action.";

type DraftState = {
  text: string;
  origin: string;
};

/** One line under the save controls: words, with any code kept as its title. */
type ActionStatus = { text: string; code?: string };

type RulePackR3JourneyEvent =
  | "rule_pack_draft_created"
  | "rule_pack_validate_requested"
  | "rule_pack_checksum_requested"
  | "rule_pack_save_requested";

/** The parts of an open pack, in the order the page shows them. */
type PartKey = "loadCases" | "categories" | "inputs" | "formula" | "checks";

const BACKEND_UNAVAILABLE_WORDS =
  "Checking, stamping and saving run in the desktop app; this browser build keeps the pack in memory only.";

export function RulePackManagerPanel({
  model,
  result = null,
  onR3JourneyEvent,
  queuedIntents,
  onQueueIntent,
  operationBusy = false,
  visit = 0
}: {
  model: PreviewModel | null;
  /**
   * How many times the Rules page has been shown again (see useReturnCount): each
   * new count reads the list again, so a failed read recovers and a change to the
   * store made elsewhere shows.
   */
  visit?: number;
  /** The session's current solved result; its rows' selectors are offered to solver inputs. */
  result?: SessionResult | null;
  onR3JourneyEvent?: (event: RulePackR3JourneyEvent) => void;
  /** "Use with this model" / "Remove from model" queue structured operations. */
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
}) {
  const [draft, setDraft] = useState<DraftState | null>(null);
  const [actionStatus, setActionStatus] = useState<ActionStatus | null>(null);
  // The list's own line: shown only when the list cannot be read or is empty.
  const [listStatus, setListStatus] = useState<ActionStatus | null>(null);
  const [entries, setEntries] = useState<LocalRulePackIndexEntry[]>([]);
  const [listRefreshed, setListRefreshed] = useState(false);
  const [validation, setValidation] = useState<RulePackDocumentValidation | null>(null);
  // The draft text the validation above was made of: its findings apply only
  // while the draft still reads the same.
  const [validatedText, setValidatedText] = useState<string | null>(null);
  // The draft text as last opened or saved: anything else is "not saved".
  const [savedText, setSavedText] = useState<string | null>(null);
  const [checksum, setChecksum] = useState<RulePackComputedChecksum | null>(null);
  // The stored entry bound to the current draft in the active project (opened or saved).
  const [openedEntry, setOpenedEntry] = useState<LocalRulePackIndexEntry | null>(null);
  // In-request busy guard: while a backend command is awaiting, the draft
  // editors (document textarea + structured composer) and every action are
  // disabled so an async response that calls setDraft (compute-checksum,
  // open) cannot clobber a mid-request edit. Routed here from the slice-1
  // residual ("a mid-request textarea edit can be clobbered").
  const [inFlight, setInFlight] = useState(false);
  // Reading the list never blocks editing; a newer read supersedes an older one.
  const listRequest = useRef(0);
  // The project whose packs the list may show: a read started for another
  // project (a save or delete that finished after the project changed) is dropped.
  const currentProject = useRef<string | null>(null);
  // The list query the opening read started, while it is still unanswered: the
  // mount effect run again (React's development StrictMode replays it) takes it
  // over instead of starting a second query.
  const openingQuery = useRef<{ project: string; query: Promise<RulePackListRoute> } | null>(null);

  const projectId = model?.project.id ?? null;
  currentProject.current = projectId;
  // A stored pack in another format: shown with its refusal only while its own
  // project is active, delete only. There is no migration; the user re-authors it.
  const hasVersionRefusal = validation !== null && versionRefusal(validation) !== null;
  const refusedStoredPack =
    openedEntry !== null && openedEntry.project_id === projectId && hasVersionRefusal ? openedEntry : null;
  const staleRefusedPack = openedEntry !== null && openedEntry.project_id !== projectId && hasVersionRefusal;
  // Keeps the active opened-pack state readable across project-switch effects and async continuations.
  const openedStateRef = useRef<{ entry: LocalRulePackIndexEntry | null; versionRefused: boolean }>({
    entry: null,
    versionRefused: false
  });
  openedStateRef.current = { entry: openedEntry, versionRefused: hasVersionRefusal };
  const draftReason = !draft
    ? NO_DRAFT_REASON
    : inFlight
      ? BUSY_REASON
      : refusedStoredPack
        ? REFUSED_STORED_PACK_REASON
        : undefined;
  const parsedDraft = draft ? parseRulePackDocument(draft.text) : null;
  const validationCurrent = validation !== null && draft !== null && validatedText === draft.text;
  const modelReferences = model?.rule_pack_refs ?? [];

  // The stored-pack list is project-scoped and read when the page opens and
  // whenever the active project changes. Reading lists the local store only;
  // it writes nothing. A refused stored pack belongs only to its project and
  // clears on project switch; an authored draft stays open but unbinds from
  // the previous project's saved entry and saved state.
  useEffect(() => {
    setEntries([]);
    setListRefreshed(false);
    const previousOpened = openedStateRef.current;
    if (previousOpened.entry !== null && previousOpened.entry.project_id !== projectId) {
      if (previousOpened.versionRefused) {
        setDraft(null);
        acceptValidation(null, null);
        setSavedText(null);
        setChecksum(null);
        setOpenedEntry(null);
        setActionStatus(null);
      } else {
        setOpenedEntry(null);
        setSavedText(null);
      }
    }
    let query: Promise<RulePackListRoute> | undefined;
    if (projectId !== null) {
      const held = openingQuery.current;
      const started = held && held.project === projectId ? held.query : listLocalRulePacks(projectId);
      openingQuery.current = { project: projectId, query: started };
      const release = () => {
        if (openingQuery.current?.query === started) openingQuery.current = null;
      };
      started.then(release, release);
      query = started;
    }
    void readList(projectId, query);
    // A read this effect started never lands after it is cleaned up (a project
    // change, the page closing, or StrictMode's replay, whose second run reads
    // the same query again under its own request).
    return () => {
      listRequest.current += 1;
    };
  }, [projectId]);

  // Back on the page: the list is read again, in place.
  useEffect(() => {
    if (visit > 0) void readList(projectId);
    // Only the page being shown again starts this read.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [visit]);

  /**
   * Read the saved packs of `forProject`. Without a project nothing is read:
   * the store's list with no project names every project's packs, and the
   * page shows one project's only. A read asked for a project that is no
   * longer open (an act that finished after the project changed) starts nothing,
   * so it cannot supersede the open project's own read. Returns the entries read,
   * or null when none were.
   */
  async function readList(
    forProject: string | null,
    query?: Promise<RulePackListRoute>
  ): Promise<LocalRulePackIndexEntry[] | null> {
    if (forProject !== currentProject.current) return null;
    const request = ++listRequest.current;
    if (forProject === null) {
      setEntries([]);
      setListRefreshed(false);
      setListStatus(null);
      return null;
    }
    const stale = () => request !== listRequest.current || currentProject.current !== forProject;
    try {
      const route = await (query ?? listLocalRulePacks(forProject));
      if (stale()) return null;
      // A list that was not read says nothing about which packs are stored.
      if (route.route === "unavailable_browser_preview") {
        setEntries([]);
        setListRefreshed(false);
        setListStatus({ text: PAGE_WORDS.listUnavailable, code: route.diagnostic });
        return null;
      }
      setEntries(route.entries);
      setListRefreshed(true);
      setListStatus(route.entries.length === 0 ? { text: PAGE_WORDS.listEmpty } : null);
      return route.entries;
    } catch (error) {
      if (stale()) return null;
      setEntries([]);
      setListRefreshed(false);
      setListStatus({ text: `The saved rule packs could not be read: ${String(error)}`, code: "RULE-PACK-BACKEND-ERROR (list)" });
      return null;
    }
  }

  // Backend (Tauri) commands can reject in desktop mode. Browser preview
  // never reaches them (the services short-circuit to an explicit
  // unavailable route), but a desktop backend error must surface honestly
  // rather than become a silent unhandled rejection.
  function reportBackendError(action: string, error: unknown): void {
    setActionStatus({ text: `Could not ${actionWord(action)}: ${String(error)}`, code: `RULE-PACK-BACKEND-ERROR (${action})` });
  }

  function reportUnavailable(diagnostic: string): void {
    setActionStatus({ text: BACKEND_UNAVAILABLE_WORDS, code: diagnostic });
  }

  function acceptValidation(next: RulePackDocumentValidation | null, text: string | null) {
    setValidation(next);
    setValidatedText(next ? text : null);
  }

  function parseDraft(): RulePackDocument | null {
    if (!draft) return null;
    try {
      const parsed: unknown = JSON.parse(draft.text);
      if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
        setActionStatus({ text: "The document must be a JSON object.", code: "RULE-PACK-DRAFT-JSON-INVALID" });
        return null;
      }
      return parsed as RulePackDocument;
    } catch (error) {
      setActionStatus({ text: `The document is not valid JSON: ${String(error)}`, code: "RULE-PACK-DRAFT-JSON-INVALID" });
      return null;
    }
  }

  function handleNewDraft() {
    const document = buildDraftRulePackDocument();
    setDraft({
      text: JSON.stringify(document, null, 2),
      origin: "new_draft_private_by_default"
    });
    acceptValidation(null, null);
    setSavedText(null);
    setChecksum(null);
    setOpenedEntry(null);
    onR3JourneyEvent?.("rule_pack_draft_created");
    setActionStatus({
      text: "New rule pack, private to you. Nothing is stored until you save.",
      code: "privacy_class=private_user_data; redistribution_status=private_only"
    });
  }

  function handleDiscardDraft() {
    setDraft(null);
    acceptValidation(null, null);
    setSavedText(null);
    setChecksum(null);
    setOpenedEntry(null);
    setActionStatus({ text: "Draft discarded; nothing saved was changed." });
  }

  async function handleValidate() {
    const document = parseDraft();
    if (!document || !draft) return;
    const text = draft.text;
    onR3JourneyEvent?.("rule_pack_validate_requested");
    setInFlight(true);
    try {
      const route = await validateRulePack(document);
      if (route.route === "unavailable_browser_preview") {
        reportUnavailable(route.diagnostic);
        return;
      }
      acceptValidation(route.validation, text);
      const blocking = faultsOf(route.validation, document).length;
      setActionStatus({
        text: blocking === 0 ? `Checked: ${PAGE_WORDS.noFaults}` : `Checked: ${thingsToFix(blocking)}.`,
        code:
          `blocking_findings=${String(route.validation.has_blocking_findings)}; ` +
          `checksum=${route.validation.checksum.match_status}; ` +
          `grammar_supported=${String(route.validation.grammar_version_supported)}`
      });
    } catch (error) {
      reportBackendError("validate", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleComputeChecksum() {
    const document = parseDraft();
    if (!document) return;
    onR3JourneyEvent?.("rule_pack_checksum_requested");
    setInFlight(true);
    try {
      const route = await computeRulePackChecksum(document);
      if (route.route === "unavailable_browser_preview") {
        reportUnavailable(route.diagnostic);
        return;
      }
      setChecksum(route.computed);
      const stamped = stampChecksumIntoDocument(document, route.computed);
      setDraft({ text: JSON.stringify(stamped, null, 2), origin: "checksum_stamped" });
      setActionStatus({
        text: "Checksum stamped into the pack.",
        code: `${route.computed.basis}; payload_excludes=${route.computed.payload_excludes.join(",")}`
      });
    } catch (error) {
      reportBackendError("compute_checksum", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleSave() {
    // The list is read again for the project this act was made in, and only while it is still open.
    const actProject = projectId;
    if (!actProject) {
      setActionStatus({ text: PAGE_WORDS.noProject });
      return;
    }
    const document = parseDraft();
    if (!document || !draft) return;
    const text = draft.text;
    const previousEntry = openedStateRef.current.entry;
    onR3JourneyEvent?.("rule_pack_save_requested");
    setInFlight(true);
    try {
      const route = await saveLocalRulePack(actProject, document);
      if (route.route === "unavailable_browser_preview") {
        reportUnavailable(route.diagnostic);
        return;
      }
      if (currentProject.current !== actProject) {
        setOpenedEntry(null);
        setSavedText(null);
        return;
      }
      // Saved under another identifier than the pack it was opened from (its
      // identifier edited in the document): the pack it was opened from is no
      // longer the open one, from the moment this text's verdict is shown.
      const renamed =
        previousEntry !== null &&
        (previousEntry.rule_pack_id !== route.envelope.rule_pack_id ||
          previousEntry.project_id !== route.envelope.project_id);
      if (renamed) setOpenedEntry(null);
      acceptValidation(route.envelope.validation, text);
      setSavedText(text);
      const blocking = faultsOf(route.envelope.validation, document).length;
      setActionStatus({
        text: blocking === 0 ? "Saved on this computer." : `Saved on this computer, with ${thingsToFix(blocking)}.`,
        code: `${route.envelope.message} (rule_pack_id=${route.envelope.rule_pack_id}, storage=${route.envelope.storage_mode})`
      });
      const listed = await readList(actProject);
      if (currentProject.current !== actProject) {
        setOpenedEntry(null);
        setSavedText(null);
        return;
      }
      // Bind the saved pack in this project on every save (first save, overwrite, or rename).
      const matched =
        listed?.find(
          (entry) => entry.project_id === route.envelope.project_id && entry.rule_pack_id === route.envelope.rule_pack_id
        ) ?? null;
      setOpenedEntry(matched ?? (route.envelope.project_id === actProject ? entryFromEnvelope(route.envelope) : null));
    } catch (error) {
      reportBackendError("save", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleOpen(entry: LocalRulePackIndexEntry) {
    // Only packs belonging to the active project can be opened, and only while it remains active.
    const actProject = projectId;
    if (!actProject || entry.project_id !== actProject) return;
    setInFlight(true);
    try {
      const route = await openLocalRulePack(actProject, entry.rule_pack_id);
      if (route.route === "unavailable_browser_preview") {
        reportUnavailable(route.diagnostic);
        return;
      }
      if (currentProject.current !== actProject) return;
      if (!route.envelope) {
        setActionStatus({ text: `${packName(entry)} is no longer saved here.`, code: entry.rule_pack_id });
        await readList(actProject);
        return;
      }
      const text = JSON.stringify(route.envelope.document, null, 2);
      setDraft({ text, origin: `opened_from_local_store:${entry.rule_pack_id}` });
      acceptValidation(route.envelope.validation, text);
      setSavedText(text);
      setChecksum(null);
      setOpenedEntry(entry);
      setActionStatus({ text: `Opened ${packName(entry)}.`, code: `${route.envelope.message} (rule_pack_id=${route.envelope.rule_pack_id})` });
    } catch (error) {
      reportBackendError("open", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleDelete(entry: LocalRulePackIndexEntry) {
    // Deletions are scoped strictly to the active project.
    const actProject = projectId;
    if (!actProject || entry.project_id !== actProject) {
      setActionStatus({ text: "That rule pack belongs to another project; nothing was deleted." });
      return;
    }
    setInFlight(true);
    try {
      const route = await deleteLocalRulePack(actProject, entry.rule_pack_id);
      if (route.route === "unavailable_browser_preview") {
        reportUnavailable(route.diagnostic);
        return;
      }
      if (currentProject.current === actProject) {
        setActionStatus({
          text: route.receipt.deleted ? `Deleted ${packName(entry)}.` : `${packName(entry)} was not saved here; nothing was deleted.`,
          code: route.receipt.message
        });
        // A deleted pack's open draft goes with it when still in the same project.
        const currentOpened = openedStateRef.current.entry;
        if (
          currentOpened &&
          currentOpened.project_id === actProject &&
          currentOpened.rule_pack_id === entry.rule_pack_id &&
          route.receipt.deleted
        ) {
          setDraft(null);
          acceptValidation(null, null);
          setSavedText(null);
          setChecksum(null);
          setOpenedEntry(null);
        }
      }
      await readList(actProject);
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

  function handleLoadCaseCategoriesChange(nextDocument: Record<string, unknown>) {
    setDraft({ text: JSON.stringify(nextDocument, null, 2), origin: "load_case_categories_edit" });
  }

  const editorDisabled = inFlight;
  const editorReason = inFlight ? BUSY_REASON : undefined;
  const actionDisabled = !draft || inFlight || refusedStoredPack !== null;
  const document = parsedDraft && parsedDraft.ok ? parsedDraft.document : null;
  const faults = validation ? faultsOf(validation, document) : [];
  const partFaults = validationCurrent ? faultsByPart(faults) : emptyFaults();
  // The saved pack's row carries a check's verdict only when the check was of the pack as saved.
  const savedPackChecked = validationCurrent && savedText !== null && validatedText === savedText;

  // The outcome of the last act, in one line: under the save controls while a
  // pack is open, under the list once it is closed (a discard or a delete).
  const statusLine = actionStatus ? (
    <small data-testid="rule-pack-action-status" title={actionStatus.code}>
      {actionStatus.text}
    </small>
  ) : null;

  return (
    <section
      className="panel rule-pack-manager-panel"
      aria-label="Rule-pack manager"
      data-testid="rule-pack-manager-panel"
    >
      <div className="panel-title">
        <FileText size={16} aria-hidden="true" />
        {PAGE_WORDS.title}
      </div>

      <small className="rule-pack-scope" data-testid="rule-pack-scope-status">
        {projectId ? PAGE_WORDS.scope : PAGE_WORDS.noProject}
      </small>

      <div className="rule-pack-actions">
        <button
          type="button"
          className="rule-pack-primary"
          data-testid="rule-pack-new-draft"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={handleNewDraft}
        >
          {PAGE_WORDS.newPack}
        </button>
      </div>

      <div className="rule-pack-list" data-testid="rule-pack-list">
        {listStatus ? (
          <small data-testid="rule-pack-list-status" title={listStatus.code}>
            {listStatus.text}
          </small>
        ) : null}
        {entries.map((entry) => {
          const used = modelReferences.some((reference) => reference.rule_pack_id === entry.rule_pack_id);
          const isOpen = openedEntry?.project_id === entry.project_id && openedEntry.rule_pack_id === entry.rule_pack_id;
          return (
            <article
              className="rule-pack-list-row"
              data-testid={`rule-pack-entry-${entry.rule_pack_id}`}
              key={`${entry.project_id}:${entry.rule_pack_id}`}
              title={`Format name: ${entry.rule_pack_id}; lifecycle=${entry.lifecycle_status}; privacy=${entry.privacy_class}; storage=${entry.storage_mode}`}
            >
              <strong>{packName(entry)}</strong>
              <span>version {entry.rule_pack_version || "not stated"}</span>
              <span className="rule-pack-row-state">
                {[
                  isOpen && savedPackChecked ? faultsInWords(faults) : wordFor(LIFECYCLE_WORDS, entry.lifecycle_status),
                  used ? PAGE_WORDS.usedByModel : null
                ]
                  .filter(Boolean)
                  .join(" · ")}
              </span>
              <span className="rule-pack-row-actions">
                <button
                  type="button"
                  data-testid={`rule-pack-open-${entry.rule_pack_id}`}
                  disabled={inFlight}
                  title={inFlight ? BUSY_REASON : undefined}
                  onClick={() => void handleOpen(entry)}
                >
                  {PAGE_WORDS.open}
                </button>
                <button
                  type="button"
                  data-testid={`rule-pack-delete-${entry.rule_pack_id}`}
                  disabled={inFlight}
                  title={inFlight ? BUSY_REASON : undefined}
                  onClick={() => void handleDelete(entry)}
                >
                  {PAGE_WORDS.delete}
                </button>
              </span>
            </article>
          );
        })}
      </div>
      {model && onQueueIntent && (entries.length > 0 || modelReferences.length > 0) ? (
        <RulePackModelReferences
          model={model}
          entries={entries}
          listRefreshed={listRefreshed}
          openPack={(entry) => openLocalRulePack(entry.project_id, entry.rule_pack_id)}
          queuedIntents={queuedIntents}
          onQueueIntent={onQueueIntent}
          operationBusy={operationBusy || inFlight}
        />
      ) : null}

      {draft && !staleRefusedPack ? null : statusLine}

      {draft && !staleRefusedPack ? (
        <div className="rule-pack-open" data-testid="rule-pack-open-pack">
          <div className="rule-pack-header" data-testid="rule-pack-header">
            <strong>{document ? documentName(document) : PAGE_WORDS.document}</strong>
            {document ? <span>version {documentVersion(document)}</span> : null}
            <span className="rule-pack-row-state" data-testid="rule-pack-save-state">
              {savedText === null ? PAGE_WORDS.notSaved : savedText !== draft.text ? PAGE_WORDS.unsavedChanges : null}
            </span>
          </div>

          <div className="rule-pack-save-controls">
            <button
              type="button"
              data-testid="rule-pack-validate"
              disabled={actionDisabled}
              title={draftReason}
              onClick={() => void handleValidate()}
            >
              {PAGE_WORDS.validate}
            </button>
            <button
              type="button"
              data-testid="rule-pack-compute-checksum"
              disabled={actionDisabled}
              title={draftReason}
              onClick={() => void handleComputeChecksum()}
            >
              {PAGE_WORDS.checksum}
            </button>
            <button
              type="button"
              className="rule-pack-primary"
              data-testid="rule-pack-save"
              disabled={actionDisabled}
              title={draftReason}
              onClick={() => void handleSave()}
            >
              {PAGE_WORDS.save}
            </button>
            <button
              type="button"
              data-testid="rule-pack-discard-draft"
              disabled={!draft || inFlight}
              title={draftReason}
              onClick={handleDiscardDraft}
            >
              {PAGE_WORDS.discard}
            </button>
          </div>
          {validation ? (
            <div className="rule-pack-findings-words" data-testid="rule-pack-validation-words" role="status">
              {!validationCurrent ? <small>{PAGE_WORDS.staleFindings} </small> : null}
              <small>{faultsInWords(faults)}</small>
              {faults.length > 0 ? (
                <ul>
                  {faults.map((finding, index) => (
                    <li key={`${finding.code}:${finding.subject}:${index}`} title={`${finding.code}; ${finding.subject}`}>
                      {finding.message}
                    </li>
                  ))}
                </ul>
              ) : null}
            </div>
          ) : null}
          {statusLine}

          {refusedStoredPack && validation ? (
            <div className="rule-pack-refusal" data-testid="rule-pack-version-refusal" role="status">
              <small title={RULE_PACK_SCHEMA_VERSION_UNSUPPORTED}>
                {packName(refusedStoredPack)} cannot be opened: {versionRefusal(validation)?.message} This version
                reads rule-pack format {RULE_PACK_SCHEMA_VERSION} only, so its checks do not run; re-author it as a
                new pack. The saved pack can only be deleted.
              </small>
              <div className="rule-pack-actions">
                <button
                  type="button"
                  data-testid="rule-pack-refused-delete"
                  disabled={inFlight}
                  title={inFlight ? BUSY_REASON : undefined}
                  onClick={() => void handleDelete(refusedStoredPack)}
                >
                  Delete saved pack
                </button>
              </div>
            </div>
          ) : null}

          {refusedStoredPack ? null : document ? (
            <div className="rule-pack-parts" data-testid="rule-pack-parts">
              <RulePackPart
                part="loadCases"
                title={PART_WORDS.loadCases}
                summary={String(arrayLength(document.required_load_cases))}
                fault={partFaults.loadCases}
              >
                <LoadCaseCategoriesEditor
                  document={document}
                  onChange={handleLoadCaseCategoriesChange}
                  disabled={editorDisabled}
                  disabledReason={editorReason}
                  model={model}
                  show="load-cases"
                />
              </RulePackPart>
              <RulePackPart
                part="categories"
                title={PART_WORDS.categories}
                summary={String(arrayLength(document.stress_categories))}
                fault={partFaults.categories}
              >
                <LoadCaseCategoriesEditor
                  document={document}
                  onChange={handleLoadCaseCategoriesChange}
                  disabled={editorDisabled}
                  disabledReason={editorReason}
                  model={model}
                  show="categories"
                />
              </RulePackPart>
              <RulePackPart
                part="inputs"
                title={PART_WORDS.inputs}
                summary={inputsSummary(document)}
                fault={partFaults.inputs}
              >
            {/* The selector offers read every row (a rule-check consumer, C2): a native result's
                bounded full copy, loaded when the person asks. */}
            <C2ResultGate result={result ?? null} analysisRun={null} feature="the solved-result selector" explicit>
              {(full) => (
                <DeclarationsEditor
                  document={document}
                  onChange={handleDeclarationsChange}
                  disabled={editorDisabled}
                  disabledReason={editorReason}
                  result={full.result}
                />
              )}
            </C2ResultGate>
              </RulePackPart>
              <RulePackPart
                part="formula"
                title={PART_WORDS.formula}
                summary={formulaSummary(document)}
                fault={partFaults.formula}
              >
                <ExpressionComposer
                  document={document}
                  onChange={handleComposerChange}
                  disabled={editorDisabled}
                  disabledReason={editorReason}
                />
              </RulePackPart>
              <RulePackPart
                part="checks"
                title={PART_WORDS.checks}
                summary={String(arrayLength(document.check_definitions))}
                fault={partFaults.checks}
              >
                <CheckDefinitionsEditor
                  document={document}
                  onChange={handleCheckDefinitionsChange}
                  disabled={editorDisabled}
                  disabledReason={editorReason}
                />
              </RulePackPart>
            </div>
          ) : (
            <small className="rule-pack-part-fault" data-testid="rule-pack-composer-unavailable">
              {PAGE_WORDS.jsonInvalid}
            </small>
          )}

          <details className="rule-pack-disclosure" data-testid="rule-pack-document">
            <summary>{PAGE_WORDS.document}</summary>
            <label htmlFor="rule-pack-draft-json-input" className="rule-pack-node-note">
              The whole pack as it is stored; the forms above write the same document.
            </label>
            <textarea
              id="rule-pack-draft-json-input"
              data-testid="rule-pack-draft-json"
              rows={14}
              value={draft.text}
              disabled={inFlight || refusedStoredPack !== null}
              title={draftReason}
              onChange={(event) =>
                setDraft((current) =>
                  current ? { ...current, text: event.target.value } : current
                )
              }
            />
          </details>
        </div>
      ) : null}

      <details className="rule-pack-disclosure" data-testid="rule-pack-details">
        <summary>{PAGE_WORDS.details}</summary>
        {checksum ? (
          <div className="report-list" data-testid="rule-pack-checksum">
            <strong>{PAGE_WORDS.checksumHeading}</strong>
            <small>
              {checksum.algorithm}; canonicalization={checksum.canonicalization}; basis=
              {checksum.basis}; grammar_version={checksum.grammar_version} (bound=
              {String(checksum.grammar_version_bound)})
            </small>
            <small data-testid="rule-pack-checksum-value">{checksum.value}</small>
          </div>
        ) : null}

        {validation && !staleRefusedPack ? (
          <div className="report-list" data-testid="rule-pack-validation">
            <strong>Document validation</strong>
            <small data-testid="rule-pack-validation-summary">
              blocking_findings={String(validation.has_blocking_findings)}; lifecycle_blocked=
              {String(validation.lifecycle_blocked)}; checksum={validation.checksum.match_status};
              grammar={validation.grammar_version} supported=
              {String(validation.grammar_version_supported)}; publicly_exportable=
              {String(validation.publicly_exportable)}
              {validation.unit_check ? `; unit_check=${validation.unit_check.status}` : null}
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
            {validation.unit_check ? (
              <div className="report-list" data-testid="rule-pack-unit-check">
                <strong>Unit consistency</strong>
                <small data-testid="rule-pack-unit-check-note">
                  Units of solver results and entered values are checked when the check runs.
                </small>
                <div className="operation-record-list">
                  {validation.unit_check.checks.map((check, checkIndex) => (
                    <article
                      className="operation-record"
                      data-testid={`rule-pack-unit-check-${check.check_id}`}
                      key={`${check.check_id}:${checkIndex}`}
                    >
                      <strong>
                        {check.check_id} (formula {check.formula_id || "none"})
                      </strong>
                      <small data-testid={`rule-pack-unit-check-status-${check.check_id}`}>
                        status={check.status}
                      </small>
                      {check.findings.map((finding, index) => (
                        <UnitCheckFindingRow
                          finding={finding}
                          key={`${finding.code}:${finding.path}:${index}`}
                        />
                      ))}
                    </article>
                  ))}
                  {validation.unit_check.declaration_findings.length > 0 ? (
                    <article
                      className="operation-record"
                      data-testid="rule-pack-unit-check-declarations"
                    >
                      <strong>Declared units</strong>
                      {validation.unit_check.declaration_findings.map((finding, index) => (
                        <UnitCheckFindingRow
                          finding={finding}
                          key={`${finding.code}:${finding.path}:${index}`}
                        />
                      ))}
                    </article>
                  ) : null}
                </div>
              </div>
            ) : null}
          </div>
        ) : null}

        <small className="report-note" data-testid="rule-pack-boundary-note">
          <ShieldAlert size={12} aria-hidden="true" /> Private rule packs stay in local project
          storage only — never committed to the repository, transmitted, or bundled into public
          artifacts. Expressions are composed as a structured AST (DEC-022 grammar v1.0.0); the
          text preview is read-only per DEC-037 and no expression text parser is provided. Rule-check output is a
          software computation over user-supplied data.
        </small>
      </details>
    </section>
  );
}

/**
 * One part of an open pack: a closed line that opens in place. Closed, it
 * reads its title and summary, and the words of the first fault the last
 * check found in it (with how many more).
 */
function RulePackPart({
  part,
  title,
  summary,
  fault,
  children
}: {
  part: PartKey;
  title: string;
  summary: string;
  fault: PartFault | null;
  children: ReactNode;
}) {
  return (
    <details className="rule-pack-part" data-testid={`rule-pack-part-${part}`}>
      <summary>
        <span className="rule-pack-part-title">{title}</span>
        <span className="rule-pack-part-summary" data-testid={`rule-pack-part-summary-${part}`}>
          {summary}
        </span>
        {fault ? (
          <span className="rule-pack-part-fault" data-testid={`rule-pack-part-fault-${part}`} title={fault.code}>
            {fault.message}
            {fault.more > 0 ? ` (and ${fault.more} more)` : ""}
          </span>
        ) : null}
      </summary>
      <div className="rule-pack-part-body">{children}</div>
    </details>
  );
}

type PartFault = { message: string; code: string; more: number };
type PartFaults = Record<PartKey, PartFault | null>;

function emptyFaults(): PartFaults {
  return { loadCases: null, categories: null, inputs: null, formula: null, checks: null };
}

function asRecord(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
}

function arrayLength(value: unknown): number {
  return Array.isArray(value) ? value.length : 0;
}

function idsOf(document: RulePackDocument, member: string, idKey: string): string[] {
  return (Array.isArray(document[member]) ? (document[member] as unknown[]) : [])
    .map((entry) => asRecord(entry)?.[idKey])
    .filter((id): id is string => typeof id === "string");
}

/**
 * The part a finding belongs to, read from its subject: a document path
 * (`required_inputs[0].name`) or a declared identifier. An identifier may
 * itself hold dots (`stress.hot`), so the whole subject is looked up as an
 * identifier before any path is read from it. A finding of the whole document
 * belongs to no part and is shown beside the save controls only.
 */
export function partOfSubject(subject: string, document: RulePackDocument): PartKey | null {
  const trimmed = subject.trim();
  const byId: Array<[PartKey, string[]]> = [
    ["loadCases", idsOf(document, "required_load_cases", "required_load_case_id")],
    [
      "categories",
      [...idsOf(document, "stress_categories", "category_id"), ...idsOf(document, "load_case_mappings", "mapping_id")]
    ],
    ["inputs", [...idsOf(document, "required_inputs", "input_id"), ...idsOf(document, "value_slots", "slot_id")]],
    ["formula", idsOf(document, "formula_declarations", "formula_id")],
    ["checks", idsOf(document, "check_definitions", "check_id")]
  ];
  const byIdentifier = (candidate: string): PartKey | null => {
    for (const [part, ids] of byId) if (ids.includes(candidate)) return part;
    return null;
  };
  // 1. The whole subject, or the subject before a parenthesised note, as an identifier.
  const whole = byIdentifier(trimmed) ?? byIdentifier(trimmed.split(" (", 1)[0]);
  if (whole) return whole;
  // 2. A document path: its first member names the part.
  const byMember: Record<string, PartKey> = {
    required_load_cases: "loadCases",
    stress_categories: "categories",
    load_case_mappings: "categories",
    required_inputs: "inputs",
    value_slots: "inputs",
    formula_declarations: "formula",
    check_definitions: "checks"
  };
  const member = /^([a-z_]+)(?:\[|\.|$)/.exec(trimmed)?.[1];
  if (member && byMember[member]) return byMember[member];
  return null;
}

/** A fault the last check found: a blocking document finding or a blocking unit finding. */
type Fault = { message: string; code: string; subject: string; part: PartKey | null };

/**
 * Every blocking fault of the last check, each with the part it belongs to.
 * A blocking unit finding is a fault like any other, so the count, the list
 * beside the save controls and the parts' closed lines always agree.
 */
export function faultsOf(validation: RulePackDocumentValidation, document: RulePackDocument | null): Fault[] {
  const part = (subject: string) => (document ? partOfSubject(subject, document) : null);
  const faults: Fault[] = collectFindings(validation)
    .filter((finding) => finding.blocking)
    .map((finding) => ({ message: finding.message, code: finding.code, subject: finding.subject, part: part(finding.subject) }));
  for (const check of validation.unit_check?.checks ?? []) {
    for (const finding of check.findings) {
      // On the part its path names (a formula's expression, a check's limit),
      // else on the check it was found in.
      if (finding.severity === "blocking") {
        faults.push({ message: finding.message, code: finding.code, subject: finding.path || check.check_id, part: part(finding.path) ?? "checks" });
      }
    }
  }
  for (const finding of validation.unit_check?.declaration_findings ?? []) {
    if (finding.severity === "blocking") {
      faults.push({ message: finding.message, code: finding.code, subject: finding.path, part: part(finding.path) ?? "inputs" });
    }
  }
  return faults;
}

/** The faults of the last check, by the part they belong to. */
function faultsByPart(faults: Fault[]): PartFaults {
  const byPart = emptyFaults();
  for (const fault of faults) {
    if (!fault.part) continue;
    const current = byPart[fault.part];
    byPart[fault.part] = current ? { ...current, more: current.more + 1 } : { message: fault.message, code: fault.code, more: 0 };
  }
  return byPart;
}

/** The last check in words: "No faults found." or "N things to fix". */
function faultsInWords(faults: Fault[]): string {
  return faults.length === 0 ? PAGE_WORDS.noFaults : thingsToFix(faults.length);
}

function inputsSummary(document: RulePackDocument): string {
  const entries = [
    ...(Array.isArray(document.required_inputs) ? document.required_inputs : []),
    ...(Array.isArray(document.value_slots) ? document.value_slots : [])
  ];
  const incomplete = entries.filter((entry) => {
    const status = asRecord(entry)?.completeness_status;
    return typeof status === "string" && status !== "complete";
  }).length;
  return incomplete > 0 ? `${entries.length}, ${incomplete} ${PART_WORDS.markedIncomplete}` : String(entries.length);
}

function formulaSummary(document: RulePackDocument): string {
  const formulas = readFormulaDeclarations(document);
  if (formulas.length === 0) return "none";
  if (formulas.length > 1) return countLine("formulas", formulas.length);
  return renderExpressionText(formulas[0].expression);
}

function documentName(document: RulePackDocument): string {
  const name = asRecord(document.metadata)?.name;
  return typeof name === "string" && name.trim() ? name : "Unnamed rule pack";
}

function documentVersion(document: RulePackDocument): string {
  const version = asRecord(document.metadata)?.rule_pack_version;
  return typeof version === "string" && version.trim() ? version : "not stated";
}

function entryFromEnvelope(envelope: LocalRulePackEnvelope): LocalRulePackIndexEntry {
  const metadata = asRecord(envelope.document.metadata);
  const classification = asRecord(envelope.document.classification);
  const readString = (record: Record<string, unknown> | null, key: string): string => {
    const value = record?.[key];
    return typeof value === "string" ? value : "";
  };
  return {
    project_id: envelope.project_id,
    rule_pack_id: envelope.rule_pack_id,
    rule_pack_name: readString(metadata, "name") || envelope.validation.rule_pack_name || envelope.rule_pack_id,
    rule_pack_version: readString(metadata, "rule_pack_version") || envelope.validation.rule_pack_version || "",
    lifecycle_status: readString(metadata, "lifecycle_status") || "draft",
    privacy_class: readString(classification, "privacy_class") || "private_user_data",
    storage_mode: envelope.storage_mode,
    created_at_unix: envelope.created_at_unix,
    updated_at_unix: envelope.updated_at_unix
  };
}

function packName(entry: LocalRulePackIndexEntry): string {
  return entry.rule_pack_name?.trim() || entry.rule_pack_id;
}

function actionWord(action: string): string {
  return (
    {
      validate: "check the pack",
      compute_checksum: "stamp the checksum",
      save: "save",
      open: "open the pack",
      delete: "delete the pack"
    }[action] ?? action
  );
}

function describeOperand(operand: RulePackUnitCheckOperand): string {
  if (operand.kind !== "quantity") return operand.kind;
  return `${operand.dimension ?? "?"} [${operand.unit_ref ?? "?"}]`;
}

function UnitCheckFindingRow({ finding }: { finding: RulePackUnitCheckFinding }) {
  return (
    <div data-testid="rule-pack-unit-check-finding">
      <small>
        {finding.code} ({finding.severity}) subject={finding.subject}; path={finding.path}
        {finding.operands.length > 0
          ? `; operands=${finding.operands.map(describeOperand).join(" vs ")}`
          : ""}
      </small>
      <p>{finding.message}</p>
    </div>
  );
}

const REFUSED_STORED_PACK_REASON =
  "This saved pack is in a rule-pack format this version does not read; it can only be deleted.";

function versionRefusal(validation: RulePackDocumentValidation): RulePackValidationFinding | null {
  return validation.document_findings.find((finding) => finding.code === RULE_PACK_SCHEMA_VERSION_UNSUPPORTED) ?? null;
}

function collectFindings(validation: RulePackDocumentValidation): RulePackValidationFinding[] {
  return [
    ...validation.document_findings,
    ...validation.lifecycle_findings,
    ...validation.formulas.flatMap((formula) => formula.findings)
  ];
}
