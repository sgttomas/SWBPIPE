// DEL-08-01 REQ-022: the session's report output outcomes (`reportOutputOutcomes.ts`),
// rendered by App outside the Report section's dormant subtree so that each save's and
// print's outcome stays visible while Report is hidden, after its panel or render is
// retired and after a project switch, until the person dismisses it.
import type { ReportOutputEntry, ReportOutputOutcomes } from "./reportOutputOutcomes";
import { reportOutputText } from "./reportOutputOutcomes";

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
      {entries.map((entry) => (
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
