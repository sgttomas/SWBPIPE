import { LoadReferenceDisclosure } from "../results/loadReferenceDisclosure";
import { QuantityReadout } from "../display-units";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { ListChecks, Play, ShieldAlert } from "lucide-react";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import {
  listLocalRulePacks,
  openLocalRulePack,
  type LocalRulePackIndexEntry,
  type RulePackDocument
} from "../../services/rulePackService";
import {
  deriveRuleCheckBindingPlan,
  describeSolverSelector,
  loadDemoRuleCheckPack,
  resolveSolverSelector,
  runRuleChecks,
  ruleBindingPrecheck,
  ruleCheckOutcomeIdentity,
  ruleCheckOutcomeTestId,
  type LibraryInputRequirement,
  type LibraryValueRef,
  type RuleCheckBasisRef,
  type RuleCheckBindingPlan,
  type RuleCheckOutcome,
  type RuleCheckRunResult,
  type RuleCheckStatus,
  type SolverInputRequirement,
  type SolverSelectorBasisStatus,
  type SolverSelectorResolution,
  type SuppliedValueBinding
} from "../../services/ruleCheckService";
import {
  classifyLibraryReference,
  indexLibraryRecordsSlots,
  listLocalLibraries,
  openLocalLibrary,
  type LibraryKind,
  type LibraryRecordIndex,
  type LibraryReferenceResolution
} from "../../services/libraryImportService";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

// Run Rule Checks (PRD §22.4 / §14.5; completion plan C4 GUI slice
// TP-C4-CHECKGUI-001). The C4 backend (run_rule_checks command +
// core/rules/rule_check_runner) is engine-true; this panel makes the R3 exit
// criterion GUI-true: a user runs a private rule pack's checks against the
// solved model and sees per-check USER_RULE_CHECKED / USER_RULE_FAILED /
// RULE_INPUTS_INCOMPLETE plus the pack aggregate, with pass/fail BLOCKED on
// missing inputs. The pack comes from the bundled invented demo, a saved
// local-store pack (the author -> save -> run journey), or pasted JSON; the
// run itself is desktop-only (the service reports the explicit unavailable
// route in browser preview).

const NO_PACK_REASON = "No rule-pack document loaded; load the demo, open a saved pack, or paste a pack.";
const NO_SOLVE_REASON =
  "No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.";
const NO_PROJECT_REASON = "Saved rule packs are project-scoped; create or open a local project first.";
const BUSY_REASON = "A rule-check backend request is in progress; wait for it to finish.";

type ParsedPack = { ok: true; document: RulePackDocument } | { ok: false; error: string };
type UnitOption = {
  symbol: string;
  label: string;
};

export type RuleCheckRunBasis = Readonly<{
  projectSessionGeneration: number;
  modelRevision: number;
  resultSequence: number;
}>;

type RunPresentation = Readonly<{
  basisKey: string;
  result: RuleCheckRunResult;
  // Epoch 8 (design §4.4): the service's disclosure line, cleared with the result.
  loadReferenceDisclosure: string | null;
}>;

type ScopedRequest = Readonly<{
  basisKey: string;
  generation: number;
}>;

const NO_CURRENT_BASIS_RUN = "No rule-check run for the current model and solved-result basis.";

function ruleCheckRunBasisKey(basis: RuleCheckRunBasis): string {
  return JSON.stringify([
    basis.projectSessionGeneration,
    basis.modelRevision,
    basis.resultSequence
  ]);
}

function parsePack(text: string): ParsedPack | null {
  const trimmed = text.trim();
  if (!trimmed) return null;
  try {
    const parsed: unknown = JSON.parse(trimmed);
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
      return { ok: false, error: "The rule-pack document must be a JSON object." };
    }
    return { ok: true, document: parsed as RulePackDocument };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, currentSymbol: string): UnitOption[] {
  const current = currentSymbol.trim() || "TBD";
  const basis = describeUnitBasis(route, current, dimensionId);
  const fallback = { symbol: basis.symbol, label: basis.label };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const entryBasis = describeUnitBasis(route, entry.symbol, dimensionId);
      return { symbol: entry.symbol, label: entryBasis.label };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) {
    options.unshift(fallback);
  }
  return options.length > 0 ? options : [fallback];
}

function UnitBindingControl({
  testId,
  basisTestId,
  value,
  dimension,
  disabled,
  unitCatalogRoute,
  onChange
}: {
  testId: string;
  basisTestId: string;
  value: string;
  dimension: string;
  disabled: boolean;
  unitCatalogRoute: UnitCatalogRoute | null;
  onChange: (next: string) => void;
}) {
  const basis = describeUnitBasis(unitCatalogRoute, value, dimension);

  return (
    <>
      {unitCatalogRoute?.route === "tauri_unit_catalog" ? (
        <select
          data-testid={testId}
          value={value.trim() || "TBD"}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value)}
        >
          {unitOptions(unitCatalogRoute, dimension, value).map((option) => (
            <option key={option.symbol} value={option.symbol}>
              {option.label}
            </option>
          ))}
        </select>
      ) : (
        <input
          type="text"
          data-testid={testId}
          value={value}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value)}
        />
      )}
      <small data-testid={basisTestId}>Unit basis: {basis.label}</small>
    </>
  );
}

function ruleCheckUnitBindingPolicySummary(plan: RuleCheckBindingPlan, route: UnitCatalogRoute | null): string {
  // Format 0.5.0 (DEL-06-01 Design `rule-pack-0.5.0.md` §2.5): each solver
  // input binds through its pack's own selector; an input without one is
  // refused by validation (RULE_SOLVER_RESULT_REF_MISSING).
  const solverSelectorCount = plan.solverInputs.filter((input) => input.solver_result_ref).length;
  const missingSelectorCount = plan.solverInputs.length - solverSelectorCount;
  const libraryRefCount = plan.libraryInputs.filter((input) => input.library_value_ref).length;
  const hasRuntimeUnitBindings = plan.valueInputs.length > 0 || plan.valueSlots.length > 0;
  const catalogRoute = !hasRuntimeUnitBindings
    ? "not_required"
    : route?.route === "tauri_unit_catalog"
      ? `dec018_catalog(entries=${route.catalog.entry_count})`
      : route
        ? "browser_manual_text_no_fallback"
        : "loading";
  return [
    `value_inputs=${plan.valueInputs.length}`,
    `value_slots=${plan.valueSlots.length}`,
    `solver_selectors=${solverSelectorCount}`,
    `solver_selectors_missing=${missingSelectorCount}`,
    `required_load_cases=${plan.requiredLoadCases.length}`,
    `private_library_refs=${libraryRefCount}`,
    `catalog=${catalogRoute}`,
    "conversion=false"
  ].join("; ");
}

// --- Library reference resolution preview (Phase C3, TP-C3-LIBREFPICKER-001) --
// A private_library_value input's library_value_ref is authored in the pack
// (DeclarationsEditor) and resolved backend-side at run time. This preview is
// read-only with respect to the pack: it queries the local store to show whether
// the authored reference resolves and lets the user browse the available
// records/slots to discover valid ids. It never mutates the pack, never
// overrides the authored reference at run time (the backend run still resolves
// from the pack reference + projectId), and never renders the private value.
const LIBRARY_KINDS: LibraryKind[] = ["material", "section", "component"];

function asLibraryKind(value: string): LibraryKind | null {
  return (LIBRARY_KINDS as string[]).includes(value) ? (value as LibraryKind) : null;
}

const LIBRARY_RESOLUTION_LABEL: Record<LibraryReferenceResolution, string> = {
  resolves: "resolves in your local store",
  record_missing: "record not found in your local store",
  slot_missing: "record found, but that slot is not in your local store",
  library_missing: "library not found in your local store"
};

type AvailableLibrary = { library_id: string; library_name: string };

type LibraryPreviewState =
  | { kind: "ok"; status: LibraryReferenceResolution; availableLibraries: AvailableLibrary[]; records: LibraryRecordIndex[] }
  | { kind: "note"; message: string }
  | { kind: "unavailable"; diagnostic: string }
  | { kind: "error"; message: string };

function renderLibraryPreview(inputId: string, ref: LibraryValueRef, preview: LibraryPreviewState) {
  if (preview.kind === "unavailable") {
    return (
      <small data-testid={`rule-check-library-resolution-${inputId}`} data-status="unavailable">
        {preview.diagnostic}
      </small>
    );
  }
  if (preview.kind === "error") {
    return (
      <small data-testid={`rule-check-library-resolution-${inputId}`} data-status="error">
        RULE-CHECK-LIBRARY-PREVIEW-ERROR: {preview.message}
      </small>
    );
  }
  if (preview.kind === "note") {
    return (
      <small data-testid={`rule-check-library-resolution-${inputId}`} data-status="unsupported_kind">
        {preview.message}
      </small>
    );
  }
  return (
    <div className="report-list" data-testid={`rule-check-library-browse-${inputId}`}>
      <small data-testid={`rule-check-library-resolution-${inputId}`} data-status={preview.status}>
        Reference {LIBRARY_RESOLUTION_LABEL[preview.status]}.{" "}
        {preview.status === "resolves"
          ? "The private value is read at run time and is never embedded in the rule pack."
          : "The check blocks at RULE_INPUTS_INCOMPLETE until the reference points at a record/slot present in your local store; edit the reference in the rule-pack editor."}
      </small>
      <small>
        {preview.availableLibraries.length > 0
          ? `Available ${ref.library_kind} libraries in your local store: ${preview.availableLibraries
              .map((library) => `${library.library_id} (${library.library_name})`)
              .join("; ")}.`
          : `No ${ref.library_kind} libraries saved in your local store yet — import one in the Libraries section, then point the reference at it.`}
      </small>
      {preview.records.length > 0 ? (
        <div className="report-list">
          {preview.records.map((record) => {
            const isRecord = record.record_id === ref.record_id;
            return (
              <div className="report-line" key={record.record_id}>
                <span>
                  {record.record_id}
                  {isRecord ? " (referenced)" : ""}
                </span>
                <small>
                  {record.slot_ids.length > 0
                    ? `slots: ${record.slot_ids
                        .map((slot) => (isRecord && slot === ref.slot_id ? `${slot} (referenced)` : slot))
                        .join(", ")}`
                    : "no value slots"}
                </small>
              </div>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}

// --- Solver-row selector preview (format 0.5.0; DEL-06-01 Design
// `rule-pack-0.5.0.md` §2.5) -----------------------------------------------
// A solver_result input binds through its pack's own selector, on each of the
// pack's required load cases; it is the one binding route. This preview is
// read-only: it shows which row of the current solved result the selector names
// on each basis; it never mutates the pack or invents a binding.
const SOLVER_BASIS_STATUS_LABEL: Record<SolverSelectorBasisStatus, string> = {
  resolves: "one row matches",
  row_missing: "no row matches (RULE_SOLVER_RESULT_ROW_MISSING at run)",
  row_ambiguous: "more than one row matches (RULE_SOLVER_RESULT_ROW_AMBIGUOUS at run)",
  basis_unsolved: "the solved result publishes no row on this basis"
};

/** "load_case load:L-100" plus the model's label when the model defines it. */
function basisText(basis: RuleCheckBasisRef, model: PreviewModel | null): string {
  const entries = basis.ref_type === "load_case" ? model?.load_cases : model?.combinations;
  const label = entries?.find((entry) => entry.id === basis.ref_id)?.label;
  return `${basis.ref_type} ${basis.ref_id}${label ? ` (${label})` : ""}`;
}

function renderSolverPreview(
  inputId: string,
  preview: SolverSelectorResolution,
  model: PreviewModel | null
) {
  return (
    <div className="report-list" data-testid={`rule-check-solver-browse-${inputId}`}>
      <small data-testid={`rule-check-solver-resolution-${inputId}`} data-status={preview.status}>
        {preview.status === "no_result_rows"
          ? "Run a solve first so the selector can be checked against result rows."
          : preview.status === "resolves"
            ? "The selector names one row of the current solve on every required load case."
            : "On at least one required load case the selector names no row or more than one; the check is RULE_INPUTS_INCOMPLETE there. Edit the selector in the Rule-Pack Manager."}
      </small>
      {preview.status !== "no_result_rows"
        ? preview.bases.map((entry) => (
            <div
              className="report-line"
              key={`${entry.basis_ref.ref_type}:${entry.basis_ref.ref_id}`}
              data-testid={`rule-check-solver-basis-${inputId}--${entry.basis_ref.ref_type}--${entry.basis_ref.ref_id}`}
              data-status={entry.status}
            >
              <span>
                {entry.name || entry.required_load_case_id}: {basisText(entry.basis_ref, model)}
              </span>
              <small>
                {SOLVER_BASIS_STATUS_LABEL[entry.status]}
                {entry.rows.map((row) => (
                  <span key={row.id}>
                    {"; "}
                    {row.id}: <QuantityReadout quantity={{ value: row.value, unit: row.unit, dimension_id: row.dimension ?? "unknown" }} />
                  </span>
                ))}
              </small>
            </div>
          ))
        : null}
    </div>
  );
}

export function RuleCheckRunPanel({
  basis,
  model,
  result,
  onAggregateChange,
  onR3JourneyEvent
}: {
  basis?: RuleCheckRunBasis;
  model: PreviewModel | null;
  result: MechanicsResult | null;
  // Lifts the pack rule-check aggregate to the app so it can be wired into
  // the app-held analysis-run envelope (TP-C4-APPAGG-001). Called with the
  // aggregate on a successful run, and with null whenever there is no current
  // run outcome (new pack loaded/pasted, or a run that did not produce one).
  // On a successful run the loaded pack and its outcomes accompany the
  // aggregate so the rendered report can disclose safe rule-pack metadata and
  // per-check status (DEL-08-01 R-08-01-008).
  onAggregateChange?: (aggregate: RuleCheckStatus | null, run?: SessionRuleCheckRun | null) => void;
  onR3JourneyEvent?: (event: "rule_check_pack_loaded" | "rule_check_run_requested") => void;
}) {
  const effectiveBasis = basis ?? {
    projectSessionGeneration: 0,
    modelRevision: 0,
    resultSequence: result ? 1 : 0
  };
  const basisKey = ruleCheckRunBasisKey(effectiveBasis);
  const currentBasisKeyRef = useRef(basisKey);
  currentBasisKeyRef.current = basisKey;
  const previousBasisKeyRef = useRef(basisKey);
  const runRequestGenerationRef = useRef(0);
  const runBusyGenerationRef = useRef<number | null>(null);
  const scopedRequestGenerationRef = useRef(0);
  const scopedBusyGenerationRef = useRef<number | null>(null);
  const onAggregateChangeRef = useRef(onAggregateChange);
  onAggregateChangeRef.current = onAggregateChange;
  const [packText, setPackText] = useState("");
  const [actionStatus, setActionStatus] = useState("No rule-check run in this session.");
  const [listStatus, setListStatus] = useState("Saved rule-pack list not refreshed yet.");
  const [entries, setEntries] = useState<LocalRulePackIndexEntry[]>([]);
  const [valueBindings, setValueBindings] = useState<Record<string, { value: string; unit: string }>>({});
  const [runPresentation, setRunPresentation] = useState<RunPresentation | null>(null);
  const [libraryPreviews, setLibraryPreviews] = useState<Record<string, LibraryPreviewState>>({});
  const [solverPreviews, setSolverPreviews] = useState<Record<string, SolverSelectorResolution>>({});
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const [inFlight, setInFlight] = useState(false);
  const runResult = runPresentation?.basisKey === basisKey ? runPresentation.result : null;
  const runLoadReferenceDisclosure = runPresentation?.basisKey === basisKey ? runPresentation.loadReferenceDisclosure : null;

  const projectId = model?.project.id ?? null;
  const parsed = useMemo(() => parsePack(packText), [packText]);
  const plan: RuleCheckBindingPlan | null = parsed?.ok ? deriveRuleCheckBindingPlan(parsed.document) : null;
  const resultRows = result?.results ?? [];
  const hasRuntimeUnitBindings = Boolean(plan && (plan.valueInputs.length > 0 || plan.valueSlots.length > 0));

  useLayoutEffect(() => {
    if (previousBasisKeyRef.current === basisKey) return;
    previousBasisKeyRef.current = basisKey;
    runRequestGenerationRef.current += 1;
    runBusyGenerationRef.current = null;
    scopedRequestGenerationRef.current += 1;
    scopedBusyGenerationRef.current = null;
    setInFlight(false);
    setRunPresentation(null);
    setActionStatus(NO_CURRENT_BASIS_RUN);
    setLibraryPreviews({});
    setSolverPreviews({});
    // A real session/model/result transition retires the previously published
    // aggregate exactly once. Initial mount and hide/show keep the same basis.
    onAggregateChangeRef.current?.(null);
  }, [basisKey]);

  function beginScopedRequest(): ScopedRequest {
    const request = {
      basisKey,
      generation: ++scopedRequestGenerationRef.current
    };
    scopedBusyGenerationRef.current = request.generation;
    setInFlight(true);
    return request;
  }

  function isScopedRequestCurrent(request: ScopedRequest): boolean {
    return currentBasisKeyRef.current === request.basisKey &&
      scopedRequestGenerationRef.current === request.generation &&
      scopedBusyGenerationRef.current === request.generation;
  }

  function finishScopedRequest(request: ScopedRequest): void {
    if (!isScopedRequestCurrent(request)) return;
    scopedBusyGenerationRef.current = null;
    setInFlight(false);
  }

  useEffect(() => {
    if (!hasRuntimeUnitBindings) {
      setUnitCatalogRoute(null);
      return undefined;
    }
    let cancelled = false;
    loadUnitCatalog()
      .then((route) => {
        if (!cancelled) setUnitCatalogRoute(route);
      })
      .catch((error: unknown) => {
        if (cancelled) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: error instanceof Error ? error.message : "UNIT-CATALOG-UNAVAILABLE"
        });
      });
    return () => {
      cancelled = true;
    };
  }, [hasRuntimeUnitBindings]);

  function resetForNewPack(text: string) {
    runRequestGenerationRef.current += 1;
    runBusyGenerationRef.current = null;
    setPackText(text);
    setValueBindings({});
    setRunPresentation(null);
    setLibraryPreviews({});
    setSolverPreviews({});
    // A new pack invalidates any prior run's aggregate held by the app.
    onAggregateChange?.(null);
  }

  // Preview whether a private_library_value input's authored library_value_ref
  // resolves in the local store, and capture the available libraries/records/
  // slots for browsing. Read-only: never mutates the pack or the run binding.
  async function handlePreviewLibrary(input: LibraryInputRequirement) {
    const ref = input.library_value_ref;
    if (!ref) return;
    const kind = asLibraryKind(ref.library_kind);
    if (!kind) {
      setLibraryPreviews((current) => ({
        ...current,
        [input.input_id]: {
          kind: "note",
          message: `library_kind "${ref.library_kind}" is not one of material/section/component, so this reference cannot resolve at run time.`
        }
      }));
      return;
    }
    const request = beginScopedRequest();
    try {
      const listRoute = await listLocalLibraries(projectId);
      if (!isScopedRequestCurrent(request)) return;
      if (listRoute.route === "unavailable_browser_preview") {
        setLibraryPreviews((current) => ({
          ...current,
          [input.input_id]: { kind: "unavailable", diagnostic: listRoute.diagnostic }
        }));
        return;
      }
      const availableLibraries: AvailableLibrary[] = listRoute.entries
        .filter((entry) => entry.library_kind === kind)
        .map((entry) => ({ library_id: entry.library_id, library_name: entry.library_name }));
      const matched = listRoute.entries.find(
        (entry) => entry.library_kind === kind && entry.library_id === ref.library_id
      );
      if (!matched) {
        setLibraryPreviews((current) => ({
          ...current,
          [input.input_id]: { kind: "ok", status: "library_missing", availableLibraries, records: [] }
        }));
        return;
      }
      const openRoute = await openLocalLibrary(matched.project_id, kind, ref.library_id);
      if (!isScopedRequestCurrent(request)) return;
      if (openRoute.route === "unavailable_browser_preview") {
        setLibraryPreviews((current) => ({
          ...current,
          [input.input_id]: { kind: "unavailable", diagnostic: openRoute.diagnostic }
        }));
        return;
      }
      if (!openRoute.envelope) {
        setLibraryPreviews((current) => ({
          ...current,
          [input.input_id]: { kind: "ok", status: "library_missing", availableLibraries, records: [] }
        }));
        return;
      }
      const records = indexLibraryRecordsSlots(openRoute.envelope.document, kind);
      const status = classifyLibraryReference(records, ref.record_id, ref.slot_id);
      setLibraryPreviews((current) => ({
        ...current,
        [input.input_id]: { kind: "ok", status, availableLibraries, records }
      }));
    } catch (error) {
      if (!isScopedRequestCurrent(request)) return;
      setLibraryPreviews((current) => ({
        ...current,
        [input.input_id]: { kind: "error", message: String(error) }
      }));
    } finally {
      finishScopedRequest(request);
    }
  }

  function bindingUnit(refId: string, fallback: string): string {
    return valueBindings[refId]?.unit ?? fallback;
  }

  function handlePreviewSolver(input: SolverInputRequirement) {
    const selector = input.solver_result_ref;
    if (!selector || !plan) return;
    setSolverPreviews((current) => ({
      ...current,
      [input.input_id]: resolveSolverSelector(resultRows, selector, plan.requiredLoadCases)
    }));
  }

  async function handleLoadDemo() {
    const request = beginScopedRequest();
    try {
      const demo = await loadDemoRuleCheckPack();
      if (!isScopedRequestCurrent(request)) return;
      resetForNewPack(JSON.stringify(demo, null, 2));
      onR3JourneyEvent?.("rule_check_pack_loaded");
      setActionStatus(
        "Loaded the bundled invented demonstration rule pack (invented non-engineering example; " +
          "not an engineering design basis). Bind its inputs below, then run checks."
      );
    } catch (error) {
      if (!isScopedRequestCurrent(request)) return;
      setActionStatus(`RULE-CHECK-DEMO-LOAD-ERROR: ${String(error)}`);
    } finally {
      finishScopedRequest(request);
    }
  }

  async function handleRefreshList() {
    const request = beginScopedRequest();
    try {
      const route = await listLocalRulePacks(projectId);
      if (!isScopedRequestCurrent(request)) return;
      if (route.route === "unavailable_browser_preview") {
        setListStatus(route.diagnostic);
        setEntries([]);
        return;
      }
      setEntries(route.entries);
      setListStatus(
        `Local store listed ${route.entries.length} saved rule pack(s)` +
          (projectId ? ` for ${projectId}.` : " across all local projects.")
      );
    } catch (error) {
      if (!isScopedRequestCurrent(request)) return;
      setEntries([]);
      setListStatus(`RULE-CHECK-BACKEND-ERROR (list): ${String(error)}`);
    } finally {
      finishScopedRequest(request);
    }
  }

  async function handleOpen(entry: LocalRulePackIndexEntry) {
    const request = beginScopedRequest();
    try {
      const route = await openLocalRulePack(entry.project_id, entry.rule_pack_id);
      if (!isScopedRequestCurrent(request)) return;
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      if (!route.envelope) {
        setActionStatus(`No stored rule pack matched ${entry.rule_pack_id}; refresh the list and retry.`);
        return;
      }
      resetForNewPack(JSON.stringify(route.envelope.document, null, 2));
      onR3JourneyEvent?.("rule_check_pack_loaded");
      setActionStatus(`Loaded saved rule pack ${entry.rule_pack_id} for run. Bind its inputs below, then run checks.`);
    } catch (error) {
      if (!isScopedRequestCurrent(request)) return;
      setActionStatus(`RULE-CHECK-BACKEND-ERROR (open): ${String(error)}`);
    } finally {
      finishScopedRequest(request);
    }
  }

  function buildSuppliedBindings(currentPlan: RuleCheckBindingPlan): SuppliedValueBinding[] {
    const bindings: SuppliedValueBinding[] = [];
    const collect = (refId: string, dimension: string, unitFallback: string) => {
      const entry = valueBindings[refId];
      if (!entry || entry.value.trim() === "") return;
      const value = Number(entry.value);
      if (!Number.isFinite(value)) return;
      bindings.push({ ref_id: refId, value, unit: entry.unit ?? unitFallback, dimension });
    };
    for (const input of currentPlan.valueInputs) collect(input.ref_id, input.dimension, input.unit_ref);
    for (const slot of currentPlan.valueSlots) collect(slot.slot_id, slot.dimension, slot.unit_ref);
    return bindings;
  }

  async function handleRun() {
    if (!parsed || !parsed.ok || !plan) {
      setActionStatus(NO_PACK_REASON);
      return;
    }
    const requestGeneration = ++runRequestGenerationRef.current;
    const requestBasisKey = basisKey;
    const isCurrent = () =>
      runRequestGenerationRef.current === requestGeneration &&
      runBusyGenerationRef.current === requestGeneration &&
      currentBasisKeyRef.current === requestBasisKey;
    runBusyGenerationRef.current = requestGeneration;
    onR3JourneyEvent?.("rule_check_run_requested");
    setInFlight(true);
    setRunPresentation(null);
    onAggregateChange?.(null);
    setActionStatus("Rule-check backend request in progress.");
    try {
      const route = await runRuleChecks({
        rulePackDocument: parsed.document,
        model,
        solvedEnvelope: result,
        suppliedValueBindings: buildSuppliedBindings(plan),
        projectId
      });
      if (!isCurrent()) return;
      if (route.route === "unavailable_browser_preview") {
        setRunPresentation(null);
        onAggregateChange?.(null);
        setActionStatus(route.diagnostic);
        return;
      }
      // One effective aggregate for the headline, the status line, the lifted
      // aggregate and the held run (FR-RULE-003).
      const effectiveResult = withEffectiveAggregate(route.result);
      setRunPresentation({ basisKey: requestBasisKey, result: effectiveResult, loadReferenceDisclosure: route.loadReferenceDisclosure ?? null });
      // Lift the pack aggregate so the app can record it in the app-held
      // analysis-run envelope (TP-C4-APPAGG-001).
      onAggregateChange?.(effectiveResult.aggregate_status, { rulePackDocument: parsed.document, result: effectiveResult });
      const counts = ruleCheckStatusCounts(effectiveResult);
      setActionStatus(
        `Ran ${effectiveResult.checks.length} check(s); aggregate=${effectiveResult.aggregate_status} ` +
          `(${counts.failed} failed; ${counts.incomplete} blocked; ${counts.checked} checked; grammar ${effectiveResult.grammar_version}).`
      );
    } catch (error) {
      if (!isCurrent()) return;
      setRunPresentation(null);
      onAggregateChange?.(null);
      setActionStatus(`RULE-CHECK-BACKEND-ERROR (run): ${String(error)}`);
    } finally {
      if (isCurrent()) {
        runBusyGenerationRef.current = null;
        setInFlight(false);
      }
    }
  }

  const bindingPrecheck = plan ? ruleBindingPrecheck(result, plan) : [];
  const runDisabled = !parsed?.ok || inFlight;
  const runDisabledReason = !parsed?.ok ? NO_PACK_REASON : inFlight ? BUSY_REASON : undefined;

  return (
    <section className="panel rule-check-run-panel" aria-label="Run rule checks" data-testid="rule-check-run-panel">
      <div className="panel-title">
        <ListChecks size={16} aria-hidden="true" />
        Run Rule Checks (user-defined, private)
      </div>

      <div className="report-list">
        <small data-testid="rule-check-run-scope">
          {projectId ? `Project scope: ${projectId}.` : NO_PROJECT_REASON}
        </small>
        <small data-testid="rule-check-run-solve-status">
          {result
            ? `Solved result available (${resultRows.length} result row(s)); mechanics=${result.status.mechanics}.`
            : NO_SOLVE_REASON}
        </small>
        {bindingPrecheck.length > 0 ? (
          <small data-testid="rule-check-binding-precheck" role="status">
            {bindingPrecheck.map((finding) => `${finding.input_id} on ${finding.basis_ref.ref_type} ${finding.basis_ref.ref_id} -> ${finding.result_id}: ${finding.reason}. ${finding.notice}`).join(" ")}
          </small>
        ) : null}
      </div>

      <div className="report-actions">
        <button
          type="button"
          data-testid="rule-check-load-demo"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={() => void handleLoadDemo()}
        >
          Load demo rule pack
        </button>
        <button
          type="button"
          data-testid="rule-check-refresh-list"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={() => void handleRefreshList()}
        >
          Refresh saved packs
        </button>
      </div>

      <div className="report-list" data-testid="rule-check-saved-list">
        <small data-testid="rule-check-run-list-status">{listStatus}</small>
        {entries.map((entry) => (
          <article
            className="operation-record"
            data-testid={`rule-check-saved-entry-${entry.rule_pack_id}`}
            key={`${entry.project_id}:${entry.rule_pack_id}`}
          >
            <strong>
              {entry.rule_pack_name} ({entry.rule_pack_id} v{entry.rule_pack_version})
            </strong>
            <small>
              lifecycle={entry.lifecycle_status}; privacy={entry.privacy_class}
            </small>
            <div className="report-actions">
              <button
                type="button"
                data-testid={`rule-check-open-${entry.rule_pack_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleOpen(entry)}
              >
                Load for run
              </button>
            </div>
          </article>
        ))}
      </div>

      <div className="report-list">
        <label htmlFor="rule-check-pack-json-input">
          Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding
          controls below are derived from this document.)
        </label>
        <textarea
          id="rule-check-pack-json-input"
          data-testid="rule-check-pack-json"
          rows={10}
          value={packText}
          placeholder="No rule-pack document. Load the demo, open a saved pack, or paste one."
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onChange={(event) => resetForNewPack(event.target.value)}
        />
        {parsed && !parsed.ok ? (
          <small data-testid="rule-check-pack-parse-error">RULE-CHECK-PACK-JSON-INVALID: {parsed.error}</small>
        ) : null}
      </div>

      {plan ? (
        <div className="report-list" data-testid="rule-check-binding-plan">
          <strong>Bind inputs</strong>
          <small>
            Unbound or missing inputs block the check at RULE_INPUTS_INCOMPLETE (pass/fail is never
            reported on missing inputs).
          </small>
          <small data-testid="rule-check-unit-catalog-status">
            {!hasRuntimeUnitBindings
              ? "No runtime value units require catalog-backed controls."
              : unitCatalogRoute?.route === "tauri_unit_catalog"
                ? `DEC-018 unit catalog loaded for run-check value bindings; entries=${unitCatalogRoute.catalog.entry_count}`
                : "browser preview keeps run-check unit refs as stored manual text; no fallback catalog synthesized"}
          </small>
          <small data-testid="rule-check-unit-binding-policy">
            {ruleCheckUnitBindingPolicySummary(plan, unitCatalogRoute)}
          </small>

          {plan.checks.length > 0 ? (
            <div className="report-list" data-testid="rule-check-plan-checks">
              {plan.checks.map((check, index) => (
                <small key={`${check.check_id}:${index}`} data-testid={`rule-check-plan-check-${check.check_id}`}>
                  {check.check_id}: category {categoryText(check.category_ref, plan)}; evaluated on{" "}
                  {check.bases.length > 0
                    ? check.bases
                        .map((loadCase) => `${loadCase.name || loadCase.required_load_case_id} (${loadCase.basis_ref ? basisText(loadCase.basis_ref, model) : "no basis"})`)
                        .join(", ")
                    : "no load case (validation refuses this pack)"}
                </small>
              ))}
            </div>
          ) : null}

          {plan.solverInputs.map((input) => {
            const selector = input.solver_result_ref;
            const preview = solverPreviews[input.input_id];
            return (
              <div
                className="report-list"
                key={`solver:${input.input_id}`}
                data-testid={`rule-check-solver-input-${input.input_id}`}
              >
                <span>
                  {input.name} <small>(solver_result; {input.dimension})</small>
                </span>
                <small data-testid={`rule-check-solver-selector-${input.input_id}`}>
                  {selector
                    ? `Bound by the pack's selector (${describeSolverSelector(selector)}) on each required load case, from the current solved result at run time.`
                    : "This input has no usable solver_result_ref selector, so validation refuses the pack (RULE_SOLVER_RESULT_REF_MISSING). Author the selector in the Rule-Pack Manager."}
                </small>
                {selector ? (
                  <div className="report-actions">
                    <button
                      type="button"
                      data-testid={`rule-check-solver-preview-${input.input_id}`}
                      disabled={inFlight}
                      title={
                        inFlight
                          ? BUSY_REASON
                          : "Show which row of the current solved result the selector names on each required load case. Read-only: this never changes the rule pack."
                      }
                      onClick={() => handlePreviewSolver(input)}
                    >
                      Preview selected rows
                    </button>
                  </div>
                ) : null}
                {selector && preview ? renderSolverPreview(input.input_id, preview, model) : null}
              </div>
            );
          })}

          {plan.valueInputs.map((input) => (
            <div className="report-line" key={`value:${input.ref_id}`}>
              <span>
                {input.name} <small>({input.source_kind}; {input.dimension})</small>
              </span>
              <span>
                <input
                  type="number"
                  data-testid={`rule-check-value-input-${input.ref_id}`}
                  value={valueBindings[input.ref_id]?.value ?? ""}
                  disabled={inFlight}
                  placeholder="value"
                  onChange={(event) =>
                    setValueBindings((current) => ({
                      ...current,
                      [input.ref_id]: { value: event.target.value, unit: bindingUnit(input.ref_id, input.unit_ref) }
                    }))
                  }
                />
                <UnitBindingControl
                  testId={`rule-check-value-unit-${input.ref_id}`}
                  basisTestId={`rule-check-value-unit-basis-${input.ref_id}`}
                  value={bindingUnit(input.ref_id, input.unit_ref)}
                  dimension={input.dimension}
                  disabled={inFlight}
                  unitCatalogRoute={unitCatalogRoute}
                  onChange={(next) =>
                    setValueBindings((current) => ({
                      ...current,
                      [input.ref_id]: { value: current[input.ref_id]?.value ?? "", unit: next }
                    }))
                  }
                />
              </span>
            </div>
          ))}

          {plan.valueSlots.map((slot) => (
            <div className="report-line" key={`slot:${slot.slot_id}`}>
              <span>
                {slot.slot_id} <small>(value_slot {slot.slot_kind}; {slot.dimension})</small>
              </span>
              <span>
                <input
                  type="number"
                  data-testid={`rule-check-slot-input-${slot.slot_id}`}
                  value={valueBindings[slot.slot_id]?.value ?? ""}
                  disabled={inFlight}
                  placeholder="limit"
                  onChange={(event) =>
                    setValueBindings((current) => ({
                      ...current,
                      [slot.slot_id]: { value: event.target.value, unit: bindingUnit(slot.slot_id, slot.unit_ref) }
                    }))
                  }
                />
                <UnitBindingControl
                  testId={`rule-check-slot-unit-${slot.slot_id}`}
                  basisTestId={`rule-check-slot-unit-basis-${slot.slot_id}`}
                  value={bindingUnit(slot.slot_id, slot.unit_ref)}
                  dimension={slot.dimension}
                  disabled={inFlight}
                  unitCatalogRoute={unitCatalogRoute}
                  onChange={(next) =>
                    setValueBindings((current) => ({
                      ...current,
                      [slot.slot_id]: { value: current[slot.slot_id]?.value ?? "", unit: next }
                    }))
                  }
                />
              </span>
            </div>
          ))}

          {plan.libraryInputs.map((input) => {
            const ref = input.library_value_ref;
            const preview = libraryPreviews[input.input_id];
            return (
              <div
                className="report-list"
                key={`library:${input.input_id}`}
                data-testid={`rule-check-library-input-${input.input_id}`}
              >
                <span>
                  {input.name} <small>(private_library_value)</small>
                </span>
                <small>
                  {ref
                    ? `Resolves from the local private library ${ref.library_kind}:${ref.library_id} → record ${ref.record_id} → slot ${ref.slot_id} at run time (desktop); the value stays in the library and is never embedded in the rule pack. The result's bound inputs show whether it resolved.`
                    : "No library_value_ref on this input, so it stays unsupplied and the check blocks. Add a library reference to the rule pack to bind it."}
                </small>
                {ref ? (
                  <div className="report-actions">
                    <button
                      type="button"
                      data-testid={`rule-check-library-preview-${input.input_id}`}
                      disabled={inFlight}
                      title={
                        inFlight
                          ? BUSY_REASON
                          : "Check the local store for the referenced library/record/slot and browse what is available. Read-only: this never changes the rule pack."
                      }
                      onClick={() => void handlePreviewLibrary(input)}
                    >
                      Preview resolution
                    </button>
                  </div>
                ) : null}
                {ref && preview ? renderLibraryPreview(input.input_id, ref, preview) : null}
              </div>
            );
          })}
        </div>
      ) : null}

      <div className="report-actions">
        <button
          type="button"
          data-testid="rule-check-run"
          disabled={runDisabled}
          title={runDisabledReason}
          onClick={() => void handleRun()}
        >
          <Play size={14} aria-hidden="true" />
          Run checks
        </button>
        <small data-testid="rule-check-run-status">{actionStatus}</small>
      </div>

      {runResult ? <LoadReferenceDisclosure text={runLoadReferenceDisclosure} surface="rule-check" testIdPrefix="rule-check-run" /> : null}
      {runResult ? (
        <div className="report-list" data-testid="rule-check-run-result">
          <div
            className="report-line"
            data-testid="rule-check-aggregate-status"
            data-status={ruleCheckHeadlineStatus(runResult)}
          >
            <span>Aggregate status</span>
            <strong>{ruleCheckHeadline(runResult)}</strong>
          </div>
          <small data-testid="rule-check-count-scope-note">{RULE_CHECK_COUNT_SCOPE_NOTE}</small>
          {runResult.rule_pack_validation ? (
            <div className="report-list" data-testid="rule-check-run-refusal">
              <small>The rule pack was refused before any check ran:</small>
              {[...runResult.rule_pack_validation.document_findings, ...runResult.rule_pack_validation.lifecycle_findings]
                .filter((finding) => finding.blocking)
                .map((finding, index) => (
                  <p key={`${finding.code}:${finding.subject}:${index}`}>
                    {finding.code} ({finding.subject}): {finding.message}
                  </p>
                ))}
            </div>
          ) : null}
          <div className="operation-record-list" data-testid="rule-check-outcomes">
            {runResult.checks.map((check) => {
              const findings = [...check.completeness_findings, ...check.evaluator_findings];
              const testId = ruleCheckOutcomeTestId(check);
              return (
                <article
                  className="operation-record"
                  data-testid={testId}
                  data-outcome-identity={ruleCheckOutcomeIdentity(check)}
                  data-status={check.status}
                  key={ruleCheckOutcomeIdentity(check)}
                >
                  <strong>
                    {check.check_id}: {aggregateLabel(check.status)}
                  </strong>
                  <small data-testid={`${testId}-basis`}>{outcomeBasisText(check, plan, model)}</small>
                  <small>
                    relation={check.acceptability_relation}
                    {check.computed_value
                      ? <>; computed=<QuantityReadout quantity={{ value: check.computed_value.value, unit: check.computed_value.unit_ref, dimension_id: check.computed_value.dimension }} /></>
                      : ""}
                    {check.limit_value ? <>; limit=<QuantityReadout quantity={{ value: check.limit_value.value, unit: check.limit_value.unit_ref, dimension_id: check.limit_value.dimension }} /></> : ""}
                  </small>
                  {check.bound_inputs.length > 0 ? (
                    <small>
                      inputs:{" "}
                      {check.bound_inputs
                        .map((input) => `${input.input_id}=${input.supplied ? "supplied" : "MISSING"}`)
                        .join(", ")}
                    </small>
                  ) : null}
                  {check.diagnostic_codes.length > 0 ? (
                    <small>diagnostics: {check.diagnostic_codes.join(", ")}</small>
                  ) : null}
                  {findings.map((finding, index) => (
                    <p key={`${finding.code}:${finding.subject_id}:${index}`}>
                      [{finding.severity}] {finding.code} ({finding.subject_id}): {finding.message}
                    </p>
                  ))}
                </article>
              );
            })}
          </div>
        </div>
      ) : null}

      <small className="report-note" data-testid="rule-check-boundary-note">
        <ShieldAlert size={12} aria-hidden="true" /> Rule checks run locally over user-supplied data only.
        The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED /
        RULE_INPUTS_INCOMPLETE).
      </small>
    </section>
  );
}

// FR-RULE-003 (DEL-06-03 Design `pack-aggregate-incomplete.md`): counted from the
// checks themselves, so the pack never headlines pass or fail while any check is
// incomplete, whatever aggregate a run carries; failed checks stay counted and listed.
function ruleCheckStatusCounts(result: RuleCheckRunResult): { failed: number; incomplete: number; checked: number } {
  const count = (status: string) => result.checks.filter((check) => check.status === status).length;
  return { failed: count("USER_RULE_FAILED"), incomplete: count("RULE_INPUTS_INCOMPLETE"), checked: count("USER_RULE_CHECKED") };
}

function ruleCheckHeadlineStatus(result: RuleCheckRunResult): string {
  return ruleCheckStatusCounts(result).incomplete > 0 ? "RULE_INPUTS_INCOMPLETE" : result.aggregate_status;
}

// A result whose checks include a blocked check carries RULE_INPUTS_INCOMPLETE
// as its aggregate, whatever aggregate it arrived with.
function withEffectiveAggregate(result: RuleCheckRunResult): RuleCheckRunResult {
  return ruleCheckStatusCounts(result).incomplete > 0 && result.aggregate_status !== "RULE_INPUTS_INCOMPLETE"
    ? { ...result, aggregate_status: "RULE_INPUTS_INCOMPLETE" }
    : result;
}

function ruleCheckHeadline(result: RuleCheckRunResult): string {
  const counts = ruleCheckStatusCounts(result);
  if (counts.incomplete > 0) {
    return `INCOMPLETE — ${counts.failed} check(s) failed; ${counts.incomplete} check(s) blocked (RULE_INPUTS_INCOMPLETE)`;
  }
  return aggregateLabel(result.aggregate_status);
}

function aggregateLabel(status: string): string {
  switch (status) {
    case "USER_RULE_CHECKED":
      return "USER_RULE_CHECKED (pass)";
    case "USER_RULE_FAILED":
      return "USER_RULE_FAILED (fail)";
    case "RULE_INPUTS_INCOMPLETE":
      return "RULE_INPUTS_INCOMPLETE (blocked on missing inputs)";
    default:
      return status;
  }
}

// Design §2.7: the counts count outcomes, one per check and basis.
export const RULE_CHECK_COUNT_SCOPE_NOTE =
  "Counts are per check and load case or combination; one check can contribute more than once.";

function categoryText(categoryRef: string | undefined, plan: RuleCheckBindingPlan | null): string {
  if (!categoryRef) return "(none)";
  const name = plan?.stressCategories.find((category) => category.category_id === categoryRef)?.name;
  return name ? `${name} (${categoryRef})` : categoryRef;
}

/** Each outcome's category and load case or combination (design §2.7). */
function outcomeBasisText(outcome: RuleCheckOutcome, plan: RuleCheckBindingPlan | null, model: PreviewModel | null): string {
  const basis = outcome.basis_ref;
  const loadCase = basis
    ? plan?.requiredLoadCases.find((entry) => entry.basis_ref?.ref_type === basis.ref_type && entry.basis_ref.ref_id === basis.ref_id)
    : undefined;
  const basisPart = basis
    ? `${loadCase?.name ? `${loadCase.name}: ` : ""}${basisText(basis, model)}`
    : "none (the pack gives this check no load case)";
  return `category=${categoryText(outcome.category_ref, plan)}; load case or combination=${basisPart}`;
}
