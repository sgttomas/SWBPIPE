import { useMemo, useState } from "react";
import { ListChecks, Play, ShieldAlert } from "lucide-react";
import type { MechanicsResult, PreviewModel } from "../../types";
import {
  listLocalRulePacks,
  openLocalRulePack,
  type LocalRulePackIndexEntry,
  type RulePackDocument
} from "../../services/rulePackService";
import {
  deriveRuleCheckBindingPlan,
  loadDemoRuleCheckPack,
  runRuleChecks,
  type RuleCheckBindingPlan,
  type RuleCheckRunResult,
  type SolverResultSelector,
  type SuppliedValueBinding
} from "../../services/ruleCheckService";

// Run Rule Checks (PRD §22.4 / §14.5; completion plan C4 GUI slice
// TP-C4-CHECKGUI-001). The C4 backend (run_rule_checks command +
// core/rules/rule_check_runner) is engine-true; this panel makes the R3 exit
// criterion GUI-true: a user runs a private rule pack's checks against the
// solved model and sees per-check USER_RULE_CHECKED / USER_RULE_FAILED /
// RULE_INPUTS_INCOMPLETE plus the worst-of aggregate, with pass/fail BLOCKED on
// missing inputs. The pack comes from the bundled invented demo, a saved
// local-store pack (the author -> save -> run journey), or pasted JSON; the
// run itself is desktop-only (the service reports the explicit unavailable
// route in browser preview). No professional/certification/sealing/approval/
// code-compliance claim is made anywhere.

const NO_PACK_REASON = "No rule-pack document loaded; load the demo, open a saved pack, or paste a pack.";
const NO_SOLVE_REASON =
  "No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.";
const NO_PROJECT_REASON = "Saved rule packs are project-scoped; create or open a local project first.";
const BUSY_REASON = "A rule-check backend request is in progress; wait for it to finish.";

type ParsedPack = { ok: true; document: RulePackDocument } | { ok: false; error: string };

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

export function RuleCheckRunPanel({
  model,
  result
}: {
  model: PreviewModel | null;
  result: MechanicsResult | null;
}) {
  const [packText, setPackText] = useState("");
  const [actionStatus, setActionStatus] = useState("No rule-check run in this session.");
  const [listStatus, setListStatus] = useState("Saved rule-pack list not refreshed yet.");
  const [entries, setEntries] = useState<LocalRulePackIndexEntry[]>([]);
  const [solverSelections, setSolverSelections] = useState<Record<string, string>>({});
  const [valueBindings, setValueBindings] = useState<Record<string, { value: string; unit: string }>>({});
  const [runResult, setRunResult] = useState<RuleCheckRunResult | null>(null);
  const [inFlight, setInFlight] = useState(false);

  const projectId = model?.project.id ?? null;
  const parsed = useMemo(() => parsePack(packText), [packText]);
  const plan: RuleCheckBindingPlan | null = parsed?.ok ? deriveRuleCheckBindingPlan(parsed.document) : null;
  const resultRows = result?.results ?? [];

  function resetForNewPack(text: string) {
    setPackText(text);
    setSolverSelections({});
    setValueBindings({});
    setRunResult(null);
  }

  function bindingUnit(refId: string, fallback: string): string {
    return valueBindings[refId]?.unit ?? fallback;
  }

  async function handleLoadDemo() {
    setInFlight(true);
    try {
      const demo = await loadDemoRuleCheckPack();
      resetForNewPack(JSON.stringify(demo, null, 2));
      setActionStatus(
        "Loaded the bundled invented demonstration rule pack (invented non-engineering example; " +
          "not an engineering design basis). Bind its inputs below, then run checks."
      );
    } catch (error) {
      setActionStatus(`RULE-CHECK-DEMO-LOAD-ERROR: ${String(error)}`);
    } finally {
      setInFlight(false);
    }
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
        `Local store listed ${route.entries.length} saved rule pack(s)` +
          (projectId ? ` for ${projectId}.` : " across all local projects.")
      );
    } catch (error) {
      setEntries([]);
      setListStatus(`RULE-CHECK-BACKEND-ERROR (list): ${String(error)}`);
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
        setActionStatus(`No stored rule pack matched ${entry.rule_pack_id}; refresh the list and retry.`);
        return;
      }
      resetForNewPack(JSON.stringify(route.envelope.document, null, 2));
      setActionStatus(`Loaded saved rule pack ${entry.rule_pack_id} for run. Bind its inputs below, then run checks.`);
    } catch (error) {
      setActionStatus(`RULE-CHECK-BACKEND-ERROR (open): ${String(error)}`);
    } finally {
      setInFlight(false);
    }
  }

  function buildSolverBindings(currentPlan: RuleCheckBindingPlan): SolverResultSelector[] {
    return currentPlan.solverInputs
      .map((input) => ({ input_id: input.input_id, result_id: solverSelections[input.input_id] ?? "" }))
      .filter((selector) => selector.result_id.length > 0);
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
    setInFlight(true);
    try {
      const route = await runRuleChecks({
        rulePackDocument: parsed.document,
        model,
        solvedEnvelope: result,
        solverResultBindings: buildSolverBindings(plan),
        suppliedValueBindings: buildSuppliedBindings(plan),
        projectId
      });
      if (route.route === "unavailable_browser_preview") {
        setRunResult(null);
        setActionStatus(route.diagnostic);
        return;
      }
      setRunResult(route.result);
      setActionStatus(
        `Ran ${route.result.checks.length} check(s); aggregate=${route.result.aggregate_status} ` +
          `(grammar ${route.result.grammar_version}).`
      );
    } catch (error) {
      setRunResult(null);
      setActionStatus(`RULE-CHECK-BACKEND-ERROR (run): ${String(error)}`);
    } finally {
      setInFlight(false);
    }
  }

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

          {plan.solverInputs.map((input) => (
            <div className="report-line" key={`solver:${input.input_id}`}>
              <span>
                {input.name} <small>(solver_result; {input.dimension})</small>
              </span>
              <select
                data-testid={`rule-check-solver-select-${input.input_id}`}
                value={solverSelections[input.input_id] ?? ""}
                disabled={inFlight || resultRows.length === 0}
                title={resultRows.length === 0 ? NO_SOLVE_REASON : undefined}
                onChange={(event) =>
                  setSolverSelections((current) => ({ ...current, [input.input_id]: event.target.value }))
                }
              >
                <option value="">— pick a solved result row —</option>
                {resultRows.map((row) => (
                  <option value={row.id} key={row.id}>
                    {row.kind}: {row.id} ({row.value} {row.unit})
                  </option>
                ))}
              </select>
            </div>
          ))}

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
                <input
                  type="text"
                  data-testid={`rule-check-value-unit-${input.ref_id}`}
                  value={bindingUnit(input.ref_id, input.unit_ref)}
                  disabled={inFlight}
                  onChange={(event) =>
                    setValueBindings((current) => ({
                      ...current,
                      [input.ref_id]: { value: current[input.ref_id]?.value ?? "", unit: event.target.value }
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
                <input
                  type="text"
                  data-testid={`rule-check-slot-unit-${slot.slot_id}`}
                  value={bindingUnit(slot.slot_id, slot.unit_ref)}
                  disabled={inFlight}
                  onChange={(event) =>
                    setValueBindings((current) => ({
                      ...current,
                      [slot.slot_id]: { value: current[slot.slot_id]?.value ?? "", unit: event.target.value }
                    }))
                  }
                />
              </span>
            </div>
          ))}

          {plan.libraryInputs.map((input) => (
            <div className="report-line" key={`library:${input.input_id}`} data-testid={`rule-check-library-input-${input.input_id}`}>
              <span>
                {input.name} <small>(private_library_value)</small>
              </span>
              <small>
                {input.library_value_ref
                  ? `Resolves from the local private library ${input.library_value_ref.library_kind}:${input.library_value_ref.library_id} → record ${input.library_value_ref.record_id} → slot ${input.library_value_ref.slot_id} at run time (desktop); the value stays in the library and is never embedded in the rule pack. The result's bound inputs show whether it resolved.`
                  : "No library_value_ref on this input, so it stays unsupplied and the check blocks. Add a library reference to the rule pack to bind it."}
              </small>
            </div>
          ))}
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

      {runResult ? (
        <div className="report-list" data-testid="rule-check-run-result">
          <div
            className="report-line"
            data-testid="rule-check-aggregate-status"
            data-status={runResult.aggregate_status}
          >
            <span>Aggregate status</span>
            <strong>{aggregateLabel(runResult.aggregate_status)}</strong>
          </div>
          <div className="operation-record-list" data-testid="rule-check-outcomes">
            {runResult.checks.map((check) => {
              const findings = [...check.completeness_findings, ...check.evaluator_findings];
              return (
                <article
                  className="operation-record"
                  data-testid={`rule-check-outcome-${check.check_id}`}
                  data-status={check.status}
                  key={check.check_id}
                >
                  <strong>
                    {check.check_id}: {aggregateLabel(check.status)}
                  </strong>
                  <small>
                    relation={check.acceptability_relation}
                    {check.computed_value
                      ? `; computed=${check.computed_value.value} ${check.computed_value.unit_ref}`
                      : ""}
                    {check.limit_value ? `; limit=${check.limit_value.value} ${check.limit_value.unit_ref}` : ""}
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
          <small data-testid="rule-check-professional-boundary">{runResult.professional_boundary_notice}</small>
        </div>
      ) : null}

      <small className="report-note" data-testid="rule-check-boundary-note">
        <ShieldAlert size={12} aria-hidden="true" /> Rule checks run locally over user-supplied data only.
        The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED /
        RULE_INPUTS_INCOMPLETE) and never a code-compliance, certification, sealing, approval,
        authentication, or professional acceptance claim. Human review remains required.
      </small>
    </section>
  );
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
