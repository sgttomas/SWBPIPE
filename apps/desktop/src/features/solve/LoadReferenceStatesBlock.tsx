import { useEffect, useState } from "react";
import { isNativeResult, nativeCustody, readNativeValidation, type SessionResult } from "../../services/nativeResult";
import type { LoadStateContractEvidence, LoadReferenceStateRecord, MechanicsResult, PreviewModel } from "../../types";
import { readerValidate, sourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";

/* Read-only view of the published `contract_evidence.load_reference_states`.
 * Values are shown only after the Rust reader of the route has admitted the
 * result, and every standing label comes from the Rust dispatch and standing
 * (`resultStanding.ts`). Nothing is computed, converted or written anywhere.
 *
 * B7b: a load/reference-state result is published under the current exact
 * identities, pressure-2 (ordinary) or pressure-source-1 (joined with the
 * retained-source method), whose readers are synchronous; the retired v2
 * load-reference-1 and load-reference-source-1 read unsupported and show no block. */

export const NEEDS_RECOMPUTE_NOT_CURRENT = "needs recompute — not Current";
export const INTEGRITY_CHECKED = "integrity checked";

type Route = "pressure" | "pressure_source";
type Admission = { state: "admitted" } | { state: "refused"; reason: string };

/** The route of a statement that publishes load/reference states, or null. */
function routeOf(result: SessionResult | null): Route | null {
  if (!result || !Array.isArray((result.contract_evidence as { load_reference_states?: unknown } | null | undefined)?.load_reference_states)) return null;
  try {
    const route = sourceContract(result);
    return route === "pressure" || route === "pressure_source" ? route : null;
  } catch { return null; }
}

/** The route's Rust reader on these bytes (no invocation: the joined reader admits the
 * publication; its standing, with any registered invocation, is read separately). */
function admission(result: MechanicsResult, route: Route): Admission {
  try { readerValidate(route === "pressure" ? "pressure" : "pressure-source", result); return { state: "admitted" }; }
  catch (error) { return { state: "refused", reason: error instanceof Error ? error.message : "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" }; }
}
/** The same reader on a native result's held bytes (C1), asynchronously; null while it reads. */
function useNativeAdmission(result: SessionResult | null, route: Route | null): Admission | null {
  const [read, setRead] = useState<{ of: SessionResult; admission: Admission } | null>(null);
  useEffect(() => {
    if (!result || !route || !isNativeResult(result)) return;
    let live = true;
    readNativeValidation(nativeCustody(result).jobId, route === "pressure" ? "pressure" : "pressure-source", {}).then(
      (outcome) => { if (live) setRead({ of: result, admission: outcome.ok ? { state: "admitted" } : { state: "refused", reason: outcome.error.detail ?? outcome.error.code } }); },
      (error) => { if (live) setRead({ of: result, admission: { state: "refused", reason: error instanceof Error ? error.message : String(error) } }); }
    );
    return () => { live = false; };
  }, [result, route]);
  return read && read.of === result ? read.admission : null;
}

export function LoadReferenceStatesBlock({ result, model }: { result: SessionResult | null; model: PreviewModel }) {
  const route = routeOf(result);
  const nativeAdmission = useNativeAdmission(result, route);
  if (!result || !route) return null;
  const admitted = isNativeResult(result) ? nativeAdmission : admission(result, route);
  if (!admitted) return <p role="status" data-testid="load-reference-states-reading">Reading the load/reference-state evidence natively…</p>;
  const joined = route === "pressure_source";
  const routeLabel = joined
    ? `joined pressure-source-1; ${NEEDS_RECOMPUTE_NOT_CURRENT} in T1`
    : "ordinary pressure-2";
  return <section aria-label="Resolved load/reference states" className="report-list" data-testid="load-reference-states-block">
    <div className="panel-subtitle">Resolved load/reference states (published, read-only)</div>
    <p data-testid="load-reference-states-route">Route: {routeLabel}</p>
    {admitted.state === "refused" && <p role="status" data-testid="load-reference-states-refused">No value is shown: the load/reference-state reader refused this result ({admitted.reason}).</p>}
    {admitted.state === "admitted" && <AdmittedStates result={result} model={model} joined={joined} />}
  </section>;
}

function AdmittedStates({ result, model, joined }: { result: SessionResult; model: PreviewModel; joined: boolean }) {
  const standing = numericalResultStanding(result, model);
  const evidence = result.contract_evidence as unknown as LoadStateContractEvidence;
  const cases = result.numerical_quality?.cases ?? [];
  return <>
    <p data-testid="load-reference-states-standing">
      Result standing: {standing.status}{standing.findings.length ? `; findings: ${standing.findings.join(", ")}` : ""}
    </p>
    {evidence.load_reference_states.map(record => {
      const published = cases.find(c => c.basis_ref.ref_type === "load_case" && c.basis_ref.ref_id === record.load_case_id);
      const label = !joined && standing.eligible && published?.solve_quality === "checks_passed" ? INTEGRITY_CHECKED : NEEDS_RECOMPUTE_NOT_CURRENT;
      return <CaseState key={record.load_case_id} record={record} label={label} quality={published?.solve_quality ?? "not published"} />;
    })}
  </>;
}

const shown = (value: unknown) => value === null || value === undefined ? "—" : String(value);

function CaseState({ record, label, quality }: { record: LoadReferenceStateRecord; label: string; quality: string }) {
  const id = record.load_case_id;
  return <article aria-label={`Load/reference state ${id}`} data-testid={`load-reference-state-${id}`}>
    <strong>Case {id}</strong>
    <p data-testid={`load-reference-state-standing-${id}`}>Numerical standing: {label} (published solve quality: {quality}).</p>
    <p>Reference configuration {record.reference_configuration_id}; mode {record.solve.requested_mode}; source recovery {record.source_recovery.status === "selected" ? `selected (${record.source_recovery.method})` : `not joined (${record.source_recovery.code})`}.</p>
    <table data-testid={`load-reference-members-${id}`}>
      <caption>Resolved element states</caption>
      <thead><tr>
        <th>Pipe</th><th>Material</th><th>Selection</th><th>Operating temperature (K)</th><th>Selection temperature (K)</th><th>Installation temperature (K)</th>
        <th>Thermal definition</th><th>Expansion law</th><th>Thermal strain (1)</th><th>Fit</th><th>Fit strain (1)</th><th>Total eigenstrain (1)</th><th>E (Pa)</th><th>nu (1)</th>
      </tr></thead>
      <tbody>{record.members.map(m => <tr key={m.pipe_id}>
        <td>{m.pipe_id}</td><td>{m.material_id}</td><td>{m.material_selection_kind}</td><td>{shown(m.operating_temperature_k)}</td><td>{shown(m.material_selection_temperature_k)}</td>
        <td>{shown(m.installation_temperature_k)}</td><td>{m.thermal_definition}</td><td>{shown(m.expansion_law_id)}</td><td>{shown(m.thermal_strain)}</td><td>{m.fit_kind}</td>
        <td>{shown(m.fit_strain)}</td><td>{shown(m.total_eigenstrain)}</td><td>{shown(m.selected_E_pa)}</td><td>{shown(m.selected_nu)}</td>
      </tr>)}</tbody>
    </table>
    <table data-testid={`load-reference-supports-${id}`}>
      <caption>Support motions</caption>
      <thead><tr><th>Support</th><th>Node</th><th>DOF</th><th>Prescribed value</th><th>Unit</th><th>Meaning</th></tr></thead>
      <tbody>{record.support_components.map(s => <tr key={`${s.support_id}-${s.dof}`}>
        <td>{s.support_id}</td><td>{s.node_id}</td><td>{s.dof}</td><td>{shown(s.prescribed_value)}</td><td>{s.unit}</td><td>{s.meaning}</td>
      </tr>)}</tbody>
    </table>
    <table data-testid={`load-reference-sources-${id}`}>
      <caption>Sources</caption>
      <thead><tr><th>Source</th><th>Owner</th><th>Classification</th><th>Factor</th><th>Published value</th></tr></thead>
      <tbody>
        {record.contributions.map(c => <tr key={`${c.owner_kind}-${c.source_id}`}>
          <td>{c.source_id}</td><td>{c.owner_kind}</td><td>{c.classification}</td>
          <td>{"factor" in c ? shown(c.factor) : "—"}</td>
          <td>{"applied_magnitude" in c ? `${shown(c.applied_magnitude)} (${c.dimension})` : "value" in c ? shown(c.value) : "—"}</td>
        </tr>)}
        {record.excluded_sources.map(x => <tr key={`excluded-${x.source_id}`}>
          <td>{x.source_id}</td><td>{x.owner_kind}</td><td>excluded ({x.reason})</td><td>—</td><td>—</td>
        </tr>)}
      </tbody>
    </table>
  </article>;
}
