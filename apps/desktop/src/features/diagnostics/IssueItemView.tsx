import type { ReactNode } from "react";
import type { Diagnostic, EntityRef } from "../../types";
import { needsSolverReportNote, SOLVER_REPORT_NOTE } from "../results/integrityStatus";
import { classText, ISSUE_ORIGIN_LABELS, itemClassText, memberDifference, notSuppliedFields, type IssueItem, type IssueMember } from "./issueItems";

/** Why a report offers no detail of its own. */
export const NO_OWN_DETAIL_REASON = "No detail of its own: another report has this code and neither has an identifier.";

/**
 * One item of the Issues drawer. Every entry reads, top to bottom: an eyebrow (one
 * small muted line: the severity, its mark and word first, then the code, the class
 * where one is supplied, and the source); what the entry
 * also names (its references beyond the item's object, its shortened-text mark, what
 * choosing it selects); then the message's own block, where the solver-report note,
 * where `needsSolverReportNote` says the report takes one (release exit criterion 18),
 * comes directly before the message. The message is the thing the eye reads and
 * nothing follows it inside the entry.
 *
 * An item with one report is that report's entry. An item that gathers several
 * reports of one code on one object (a spring hanger's installed, cold and hot load,
 * or the same finding from the model checks and the solve) leads with its most severe
 * report's message, says how many reports it holds and its class once in its eyebrow
 * and, where the item may show excerpts, what tells the reports apart above the
 * message; it keeps every report in a disclosure inside the item, each its own entry
 * in the same order, the message showing its excerpt where excerpts are allowed. A
 * report that is a diagnostic opens its own detail wherever its item is listed, or
 * says why it cannot and shows the detail's fields above its message. The item's
 * severity is its worst member's. An item with any report that takes the note shows
 * no excerpts.
 */
export function IssueItemView({
  item,
  main,
  lead = item.lead,
  selects = null,
  selectedDiagnosticId = null,
  onSelectDiagnostic,
  onSelectEntity
}: {
  item: IssueItem;
  /** The item's own row, given the text it shows. */
  main: (body: ReactNode) => ReactNode;
  /** The report the collapsed row leads with: the item's own lead unless a report was asked for. */
  lead?: IssueMember;
  /** The entity the item's own row selects, where it selects one. */
  selects?: EntityRef | null;
  selectedDiagnosticId?: string | null;
  onSelectDiagnostic?: (diagnosticId: string) => void;
  onSelectEntity?: (entity: EntityRef) => void;
}) {
  if (item.members.length === 1) {
    const only = item.members[0];
    return (
      <>
        {main(
          <>
            <Eyebrow severity={only.diagnostic.severity} code={only.diagnostic.code} classes={classText(only.diagnostic)} origins={ISSUE_ORIGIN_LABELS[only.origin]} />
            <OwnRefs member={only} single />
            <Selects entity={selects} />
            <MessageBlock report={only.shown} />
          </>
        )}
      </>
    );
  }
  const origins = [...new Set(item.members.map((member) => member.origin))];
  return (
    <div className={`issue-group ${item.severity}`} data-testid={`issue-group-${item.code}`}>
      {main(
        <>
          <Eyebrow
            severity={item.severity}
            code={item.code}
            classes={itemClassText(item)}
            origins={joinWords(origins.map((origin) => ISSUE_ORIGIN_LABELS[origin]))}
            count={item.members.length}
          />
          {item.differences.length > 0 ? (
            <small className="issue-group-also" data-testid="issue-group-also">
              Reported for each of: {item.differences.join("; ")}.
            </small>
          ) : null}
          <Selects entity={selects} />
          <MessageBlock report={lead.shown} />
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
                selected={selectedDiagnosticId !== null && member.ownDetail && (member.diagnostic.id ?? member.diagnostic.code) === selectedDiagnosticId && member.source === "diagnostics"}
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

/** The entry's eyebrow: severity (mark and word) first, then code, class and source, and an item's report count. */
function Eyebrow({ severity, code, classes, origins, count }: { severity: string; code: string; classes: string | null; origins: string; count?: number }) {
  return (
    <small className="issue-meta muted" data-testid="issue-meta">
      <span aria-hidden="true" className={`issue-severity-mark ${severity}`} />
      {severity} · {code} · {classes ? <><span className="issue-class" data-testid="issue-class">{classes}</span> · </> : null}from {origins}
      {count !== undefined ? <> · <span className="issue-group-count" data-testid="issue-group-count">{count} reports</span></> : null}
    </small>
  );
}

/** The message's own block: the note first where the report takes one, then the message; nothing after it. */
function MessageBlock({ report, excerpt = null }: { report: Diagnostic; excerpt?: string | null }) {
  const message = report.message;
  const at = excerpt && excerpt !== "(no other words)" ? message.indexOf(excerpt) : -1;
  return (
    <div className="issue-message-block">
      <SolverReportNote report={report} />
      <p className="issue-message">
        {at >= 0 && excerpt ? (
          <>
            {message.slice(0, at)}
            <strong className="issue-member-difference" data-testid="issue-member-difference">{excerpt}</strong>
            {message.slice(at + excerpt.length)}
          </>
        ) : message}
      </p>
    </div>
  );
}

/** What choosing the row selects, where it selects an entity. */
function Selects({ entity }: { entity: EntityRef | null }) {
  return entity ? <small className="issue-selects">Selects {entity.id}</small> : null;
}

/** A report's references beyond its item's object, in display form, and its shortened-text mark. */
function OwnRefs({ member, single = false }: { member: IssueMember; single?: boolean }) {
  const truncated = (member.diagnostic as { truncated?: boolean }).truncated === true;
  return (
    <>
      {member.otherRefs.length > 0 ? (
        <small className="issue-member-refs" data-testid="issue-member-refs">Also about: {member.otherRefs.join("; ")}</small>
      ) : null}
      {truncated ? (
        <small data-testid={single && member.source === "model-checks" ? "model-check-truncated" : "issue-member-truncated"}>Text shortened in the bounded report.</small>
      ) : null}
    </>
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
  const act = source === "diagnostics" && onSelectDiagnostic && member.ownDetail
    ? () => onSelectDiagnostic(diagnostic.id ?? diagnostic.code)
    : source === "model-checks" && entity && onSelectEntity
      ? () => onSelectEntity(entity)
      : null;
  const noDetail = source === "diagnostics" && !member.ownDetail;
  return (
    <button
      aria-disabled={act === null}
      className={`issue-member diagnostic ${diagnostic.severity}${selected ? " active" : ""}`}
      data-source={source}
      data-testid={`issue-member-${diagnostic.code}`}
      onClick={() => act?.()}
      title={act ? undefined : noDetail ? NO_OWN_DETAIL_REASON : "This report names nothing more to show."}
      type="button"
    >
      <small className="issue-meta muted">
        <span aria-hidden="true" className={`issue-severity-mark ${diagnostic.severity}`} />
        {diagnostic.severity} · {diagnostic.code} · {classText(diagnostic) ? <><span data-testid="issue-member-class">{classText(diagnostic)}</span> · </> : null}from {ISSUE_ORIGIN_LABELS[member.origin]}
      </small>
      <NotSupplied diagnostic={diagnostic} />
      <OwnRefs member={member} />
      {noDetail ? <NoDetailFields diagnostic={diagnostic} /> : null}
      <Selects entity={source === "model-checks" ? entity : null} />
      <MessageBlock report={shown} excerpt={difference} />
    </button>
  );
}

/** In an opened row, which fields its producer did not supply, so a person knows there is nothing
 * more to find; nothing where every field was supplied. */
function NotSupplied({ diagnostic }: { diagnostic: Diagnostic }) {
  const absent = notSuppliedFields(diagnostic);
  return absent.length > 0 ? <small className="issue-member-not-supplied" data-testid="issue-member-not-supplied">Not supplied: {absent.join(", ")}</small> : null;
}

/** Where a report cannot open its own detail, the supplied fields that detail would show, on its
 * row; the fields not supplied are named once, on the row's "Not supplied" line. */
function NoDetailFields({ diagnostic }: { diagnostic: Diagnostic }) {
  const provenance = typeof diagnostic.provenance === "string" ? diagnostic.provenance : diagnostic.provenance ? JSON.stringify(diagnostic.provenance) : null;
  const affected = (diagnostic.affected_refs ?? []).join("; ");
  return (
    <span className="issue-member-fields" data-testid="issue-member-no-detail">
      <small>{NO_OWN_DETAIL_REASON}</small>
      {diagnostic.source ? <small>Source: {diagnostic.source}</small> : null}
      {affected ? <small>Affected: {affected}</small> : null}
      {diagnostic.remediation ? <small>Remediation: {diagnostic.remediation}</small> : null}
      {provenance ? <small>Provenance: {provenance}</small> : null}
    </span>
  );
}

/** The note directly before a report's text, where `needsSolverReportNote` says that report takes
 * one; nothing otherwise. One per shown text of one report; never one for the item. */
function SolverReportNote({ report }: { report: Diagnostic | null }) {
  return report && needsSolverReportNote(report)
    ? <span className="muted issue-solver-report-note" data-testid={`diagnostic-solver-report-note-${report.code}`}>{SOLVER_REPORT_NOTE}</span>
    : null;
}

function joinWords(words: readonly string[]): string {
  if (words.length <= 1) return words.join("");
  return `${words.slice(0, -1).join(", ")} and ${words[words.length - 1]}`;
}

/** In place of an item that the Diagnostics list shows as requested: where it is, so the item is
 * shown once. */
export function ShownUnderDiagnostics({ item }: { item: IssueItem }) {
  return (
    <p className="muted issue-shown-elsewhere" data-testid="issue-shown-under-diagnostics">
      {item.code}{item.object ? ` on ${item.object}` : ""}: shown under Diagnostics below, as requested.
    </p>
  );
}
