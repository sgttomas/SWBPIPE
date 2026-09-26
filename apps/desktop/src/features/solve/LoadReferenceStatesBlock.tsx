import { useEffect, useState } from "react";
import type { LoadReferenceContractEvidence, LoadReferenceStateRecord, MechanicsResult, PreviewModel } from "../../types";
import { validateLoadReferenceEvidence } from "../results/loadReferenceEvidence";
import {
  LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED, loadReferenceSourceStanding,
  validateLoadReferenceSourceEvidence
} from "../results/loadReferenceSourceEvidence";
import { numericalResultStanding, sourceContract } from "../results/numericalResultQuality";

/* Read-only view of the published `contract_evidence.load_reference_states`.
 * Values are shown only after the WP2 reader for the route has admitted the
 * result, and every standing label comes from the WP2 dispatch and standing
 * functions. Nothing is computed, converted or written anywhere. */

export const NEEDS_RECOMPUTE_NOT_CURRENT = "needs recompute — not Current";
export const INTEGRITY_CHECKED = "integrity checked";

type Route = "load_reference" | "load_reference_source";
type Admission = { state: "admitted" } | { state: "refused"; reason: string } | { state: "reading" };

function routeOf(result: MechanicsResult | null): Route | null {
  if (!result) return null;
  try {
    const route = sourceContract(result);
    return route === "load_reference" || route === "load_reference_source" ? route : null;
  } catch { return null; }
}

function admission(result: MechanicsResult, route: Route): Admission {
  if (route === "load_reference") {
    try { validateLoadReferenceEvidence(result); return { state: "admitted" }; }
    catch (error) { return { state: "refused", reason: error instanceof Error ? error.message : "SOURCE_LOAD_REFERENCE_MALFORMED" }; }
  }
  const [finding] = loadReferenceSourceStanding(result).findings;
  if (finding === LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE) return { state: "admitted" };
  if (finding === LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED) return { state: "reading" };
  return { state: "refused", reason: finding };
}

export function LoadReferenceStatesBlock({ result, model }: { result: MechanicsResult | null; model: PreviewModel }) {
  const route = routeOf(result);
  const [, setRead] = useState(0);
  const admitted = result && route ? admission(result, route) : null;
  useEffect(() => {
    if (!result || route !== "load_reference_source" || admitted?.state !== "reading") return;
    let live = true;
    // The joined reader registers its own outcome; the block only re-reads it.
    validateLoadReferenceSourceEvidence(result).catch(() => undefined).finally(() => { if (live) setRead(n => n + 1); });
    return () => { live = false; };
  }, [result, route, admitted?.state]);
  if (!result || !route || !admitted) return null;
  const joined = route === "load_reference_source";
  const routeLabel = joined
    ? `joined load-reference-source-1; ${NEEDS_RECOMPUTE_NOT_CURRENT} in T1`
    : "ordinary load-reference-1";
  return <section aria-label="Resolved load/reference states" className="report-list" data-testid="load-reference-states-block">
    <div className="panel-subtitle">Resolved load/reference states (published, read-only)</div>
    <p data-testid="load-reference-states-route">Route: {routeLabel}</p>
    {admitted.state === "reading" && <p role="status" data-testid="load-reference-states-reading">Reading the joined result with its reader before any value is shown.</p>}
    {admitted.state === "refused" && <p role="status" data-testid="load-reference-states-refused">No value is shown: the load/reference-state reader refused this result ({admitted.reason}).</p>}
    {admitted.state === "admitted" && <AdmittedStates result={result} model={model} joined={joined} />}
  </section>;
}

function AdmittedStates({ result, model, joined }: { result: MechanicsResult; model: PreviewModel; joined: boolean }) {
  const standing = numericalResultStanding(result, model);
  const evidence = result.contract_evidence as unknown as LoadReferenceContractEvidence;
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
