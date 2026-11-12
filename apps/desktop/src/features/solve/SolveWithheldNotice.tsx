import { useId } from "react";
import type { Diagnostic, EntityRef, PreviewModel } from "../../types";
import { FindingButton } from "../diagnostics/ModelChecksPanel";
import { resolveModelCheckEntity } from "../diagnostics/modelChecks";
import type { SolveGateReading } from "../workspace/solveGates";

/** How many standing findings the notice lists; the rest are in the Model checks list. */
export const SOLVE_WITHHELD_LISTED = 5;
const NOT_SUPPLIED = "not supplied by producer";

type Withheld = Extract<SolveGateReading, { kind: "withheld" }>;

/**
 * The Solve command's withheld notice (DEL-07-07 REQ-008, AC-005; DEL-07-04
 * AC-001): it states that Solve is withheld and names each standing finding with
 * the fields its producer supplied, a field not supplied reading as not supplied
 * (DEL-07-04 REQ-004, DEL-07-07 REQ-005). A finding is shown as the Model checks
 * list shows it, and choosing it, or an affected object it names, selects that
 * entity. `summaryId` is the summary line's id, which the Solve control names as
 * its description.
 */
export function SolveWithheldNotice({
  reading,
  model,
  summaryId,
  onSelectEntity,
  onOpenModelChecks
}: {
  reading: Withheld;
  model: PreviewModel;
  summaryId: string;
  onSelectEntity: (entity: EntityRef) => void;
  onOpenModelChecks: () => void;
}) {
  const titleId = useId();
  const listed = reading.findings.slice(0, SOLVE_WITHHELD_LISTED);
  const total = reading.findings.length + reading.notListedBlocking;
  // Listed in Model checks but beyond this notice's bound; and counted by the bounded report but listed nowhere.
  const restListed = reading.findings.length - listed.length;
  const counted = reading.notListedBlocking;
  return (
    <section aria-labelledby={titleId} className="memory-pool-message warning solve-withheld-notice" data-testid="solve-withheld-notice">
      <strong id={titleId}>Solve is withheld</strong>
      <p data-testid="solve-withheld-summary" id={summaryId}>
        {total > 0 ? `${total} solve-blocking ${total === 1 ? "finding stands" : "findings stand"} for this model. ` : null}
        {reading.refusal !== null ? `The solve cannot read this model: ${reading.refusal}. ` : null}
        Solve becomes available once the model checks find no solve-blocking finding; they run again by themselves after each model change.
      </p>
      {reading.laterCheckNotCompleted !== null ? (
        <p data-testid="solve-withheld-later-not-completed">
          A later run of the checks on this model could not finish ({reading.laterCheckNotCompleted}). The findings below are from the last run that finished, and they still apply.
        </p>
      ) : null}
      {listed.length > 0 ? (
        <ol aria-label="Findings that withhold Solve" className="model-checks-list" data-testid="solve-withheld-findings">
          {listed.map((finding) => (
            <li key={finding.key}>
              <FindingButton finding={finding} onSelectEntity={onSelectEntity} />
              <FindingFields diagnostic={finding.diagnostic} model={model} selected={finding.entity} onSelectEntity={onSelectEntity} />
            </li>
          ))}
        </ol>
      ) : null}
      {restListed > 0 || counted > 0 ? (
        <p data-testid="solve-withheld-rest">
          {restListed > 0 ? `${restListed} more solve-blocking ${restListed === 1 ? "finding is" : "findings are"} listed in Model checks. ` : null}
          {counted > 0 ? `${counted} more solve-blocking ${counted === 1 ? "finding was" : "findings were"} counted by the bounded report but not listed. ` : null}
          <button className="secondary-action" data-testid="solve-withheld-open-checks" onClick={onOpenModelChecks} type="button">
            Show Model checks
          </button>
        </p>
      ) : null}
    </section>
  );
}

/** The fields the finding's button does not show, each as its producer supplied it. */
function FindingFields({
  diagnostic,
  model,
  selected,
  onSelectEntity
}: {
  diagnostic: Diagnostic;
  model: PreviewModel;
  selected: EntityRef | null;
  onSelectEntity: (entity: EntityRef) => void;
}) {
  const refs = diagnostic.affected_refs ?? [];
  const provenance = diagnostic.provenance === undefined ? NOT_SUPPLIED
    : typeof diagnostic.provenance === "string" ? diagnostic.provenance : JSON.stringify(diagnostic.provenance);
  return (
    <dl className="solve-withheld-fields" data-testid={`solve-withheld-fields-${diagnostic.code}`}>
      <dt>Class</dt><dd>{diagnostic.diagnostic_class ?? diagnostic.class ?? NOT_SUPPLIED}</dd>
      <dt>Source</dt><dd>{diagnostic.source ?? NOT_SUPPLIED}</dd>
      <dt>Remediation</dt><dd>{diagnostic.remediation ?? NOT_SUPPLIED}</dd>
      <dt>Provenance</dt><dd>{provenance}</dd>
      <dt>Affected objects</dt>
      <dd>
        {refs.length === 0 ? NOT_SUPPLIED : (
          <ul aria-label={`Objects ${diagnostic.code} names`}>
            {refs.map((ref, index) => {
              const entity = resolveModelCheckEntity(model, ref);
              if (!entity) return <li key={`${ref}:${index}`}>{ref} (names no model entity to select)</li>;
              if (selected && entity.type === selected.type && entity.id === selected.id) {
                return <li key={`${ref}:${index}`}>{ref} (the finding above selects it)</li>;
              }
              return (
                <li key={`${ref}:${index}`}>
                  <button className="secondary-action" data-entity={`${entity.type}:${entity.id}`} onClick={() => onSelectEntity(entity)} type="button">
                    Select {entity.id}
                  </button>
                </li>
              );
            })}
          </ul>
        )}
      </dd>
    </dl>
  );
}
