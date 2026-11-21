// DEL-08-01 REQ-022: the session's report output outcomes (`reportOutputOutcomes.ts`),
// rendered by App outside the Report section's dormant subtree so that each save's and
// print's outcome stays visible while Report is hidden, after its panel or render is
// retired and after a project switch, until the person dismisses it. A report package save's
// entry (issue 508) is one status region: its line, then the receipt's warnings, then its
// figures behind a closed Details; Dismiss once it has settled.
import type { ReportOutputEntry, ReportOutputOutcomes, ReportPackageOutputEntry } from "./reportOutputOutcomes";
import { isReportPackageEntry, reportOutputDetails, reportOutputText } from "./reportOutputOutcomes";

function ReportPackageOutput({ outcomes, entry, currentSequence }: { outcomes: ReportOutputOutcomes; entry: ReportPackageOutputEntry; currentSequence: number }) {
  const pending = entry.state === "pending";
  const details = reportOutputDetails(entry);
  return (
    <div
      role="status"
      data-testid={`report-output-${entry.id}`}
      data-kind={entry.kind}
      data-state={entry.state}
      data-workspace-changed={String(entry.basis.sequence !== currentSequence)}
    >
      <p className={pending ? "operation-running" : undefined} data-testid={`report-output-${entry.id}-text`}>
        {reportOutputText(entry, currentSequence)}
      </p>
      {entry.package.warnings.length > 0 ? (
        <ul aria-label="Warnings" data-testid={`report-output-${entry.id}-warnings`}>
          {entry.package.warnings.map((warning, index) => (
            <li key={index}>{`Warning: ${warning.code}${warning.path ? ` at ${warning.path}` : ""}: ${warning.message}`}</li>
          ))}
        </ul>
      ) : null}
      {details.length > 0 ? (
        <details data-testid={`report-output-${entry.id}-details`}>
          <summary>Details</summary>
          <ul>
            {details.map((line, index) => <li key={index}>{line}</li>)}
          </ul>
        </details>
      ) : null}
      {pending ? null : (
        <button type="button" data-testid={`report-output-${entry.id}-dismiss`} onClick={() => outcomes.dismiss(entry.id)}>
          Dismiss
        </button>
      )}
    </div>
  );
}

export function ReportOutputStatus({
  outcomes,
  entries,
  currentSequence
}: {
  outcomes: ReportOutputOutcomes;
  entries: readonly ReportOutputEntry[];
  /** The workspace's report source number now. */
  currentSequence: number;
}) {
  if (entries.length === 0) return null;
  return (
    <section className="report-output-status" aria-label="Report output outcomes" data-testid="report-output-status">
      {entries.map((entry) => isReportPackageEntry(entry) ? (
        <ReportPackageOutput key={entry.id} outcomes={outcomes} entry={entry} currentSequence={currentSequence} />
      ) : (
        <p
          key={entry.id}
          role="status"
          data-testid={`report-output-${entry.id}`}
          data-kind={entry.kind}
          data-state={entry.state}
          data-workspace-changed={String(entry.basis.sequence !== currentSequence)}
        >
          <span data-testid={`report-output-${entry.id}-text`}>{reportOutputText(entry, currentSequence)}</span>
          {entry.state === "pending" ? null : (
            <button type="button" data-testid={`report-output-${entry.id}-dismiss`} onClick={() => outcomes.dismiss(entry.id)}>
              Dismiss
            </button>
          )}
        </p>
      ))}
    </section>
  );
}
