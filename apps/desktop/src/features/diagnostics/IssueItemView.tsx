import type { ReactNode } from "react";
import type { EntityRef } from "../../types";
import { classText, ISSUE_ORIGIN_LABELS, itemClassText, memberDifference, type IssueItem, type IssueMember } from "./issueItems";

/**
 * One item of the Issues drawer. An item with one report is that report's own
 * button, with its class. An item that gathers several reports of one code on one
 * object (a spring hanger's installed, cold and hot load, or the same finding
 * from the model checks and the solve) says how many reports it holds, shows the
 * first report's text, its class once and what tells the reports apart, and keeps
 * every report in a disclosure inside the item, each with its own code, severity,
 * class ("class not supplied" where its producer gave none), source, what tells it
 * apart and message. A report that is a diagnostic opens its detail wherever its
 * item is listed. The item's severity is its worst member's.
 */
export function IssueItemView({
  item,
  main,
  selectedDiagnosticId = null,
  onSelectDiagnostic,
  onSelectEntity
}: {
  item: IssueItem;
  /** The item's own button, given the text it shows. */
  main: (body: ReactNode) => ReactNode;
  selectedDiagnosticId?: string | null;
  onSelectDiagnostic?: (diagnosticId: string) => void;
  onSelectEntity?: (entity: EntityRef) => void;
}) {
  const first = item.members[0];
  if (item.members.length === 1) {
    return <>{main(<><p>{first.shown.message}</p><small className="issue-class" data-testid="issue-class">{classText(first.diagnostic)}</small></>)}</>;
  }
  const origins = [...new Set(item.members.map((member) => member.origin))];
  return (
    <div className={`issue-group ${item.severity}`} data-testid={`issue-group-${item.code}`}>
      {main(
        <>
          <small className="issue-group-count" data-testid="issue-group-count">{item.members.length} reports</small>
          <p>{first.shown.message}</p>
          <small className="issue-class" data-testid="issue-class">{itemClassText(item)}</small>
          {item.differences.length > 0 ? (
            <small className="issue-group-also" data-testid="issue-group-also">
              Reported for each of: {item.differences.join("; ")}.
            </small>
          ) : null}
          {origins.length > 1 ? (
            <small className="issue-group-sources">Reported by {joinWords(origins.map((origin) => ISSUE_ORIGIN_LABELS[origin]))}.</small>
          ) : null}
        </>
      )}
      <details className="issue-group-members" data-testid="issue-group-members">
        <summary data-testid="issue-group-members-toggle">
          All {item.members.length} reports{item.object ? ` for ${item.object}` : ""}
        </summary>
        <ol>
          {item.members.map((member, index) => (
            <li key={`${member.source}-${member.diagnostic.id ?? member.diagnostic.code}-${index}`}>
              <MemberButton
                member={member}
                difference={memberDifference(item, member)}
                selected={selectedDiagnosticId !== null && (member.diagnostic.id ?? member.diagnostic.code) === selectedDiagnosticId && member.source === "diagnostics"}
                onSelectDiagnostic={onSelectDiagnostic}
                onSelectEntity={onSelectEntity}
              />
            </li>
          ))}
        </ol>
      </details>
    </div>
  );
}

function MemberButton({ member, difference, selected, onSelectDiagnostic, onSelectEntity }: {
  member: IssueMember;
  difference: string | null;
  selected: boolean;
  onSelectDiagnostic?: (diagnosticId: string) => void;
  onSelectEntity?: (entity: EntityRef) => void;
}) {
  const { diagnostic, shown, source } = member;
  const entity = member.finding?.entity ?? null;
  const act = source === "diagnostics" && onSelectDiagnostic
    ? () => onSelectDiagnostic(diagnostic.id ?? diagnostic.code)
    : source === "model-checks" && entity && onSelectEntity
      ? () => onSelectEntity(entity)
      : null;
  return (
    <button
      aria-disabled={act === null}
      className={`issue-member diagnostic ${diagnostic.severity}${selected ? " active" : ""}`}
      data-source={source}
      data-testid={`issue-member-${diagnostic.code}`}
      onClick={() => act?.()}
      title={act ? undefined : "This report names nothing more to show."}
      type="button"
    >
      <strong>{diagnostic.code}</strong>{" "}
      <span className="muted">
        {diagnostic.severity} · <span data-testid="issue-member-class">{classText(diagnostic)}</span> · from {ISSUE_ORIGIN_LABELS[member.origin]}
      </span>
      {difference ? <small className="issue-member-difference" data-testid="issue-member-difference">For: {difference}</small> : null}
      <p>{shown.message}</p>
    </button>
  );
}

function joinWords(words: readonly string[]): string {
  if (words.length <= 1) return words.join("");
  return `${words.slice(0, -1).join(", ")} and ${words[words.length - 1]}`;
}
