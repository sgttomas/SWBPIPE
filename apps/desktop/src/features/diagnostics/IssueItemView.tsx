import { CircleAlert, Info, OctagonAlert, TriangleAlert } from "lucide-react";
import { useId, useState, type ReactNode } from "react";
import type { Diagnostic, EntityRef } from "../../types";
import { classText, ISSUE_ORIGIN_LABELS, itemClassText, memberDifference, notSuppliedFields, type IssueItem, type IssueMember } from "./issueItems";

/** Why a report offers no detail of its own. */
export const NO_OWN_DETAIL_REASON = "No detail of its own: another report has this code and neither has an identifier.";

/** A message longer than this many characters (about three lines at the drawer's width) is
 * collapsed to its beginning. */
export const LONG_MESSAGE_CHARACTERS = 280;
/** How much of a collapsed message is shown: at most this many characters, cut at a word. */
const BEGINNING_CHARACTERS = 200;

/** A long message's beginning, cut at a word; null where the message is shown whole. */
export function messageBeginning(message: string): string | null {
  if (message.length <= LONG_MESSAGE_CHARACTERS) return null;
  const cut = message.slice(0, BEGINNING_CHARACTERS);
  const space = cut.search(/\s\S*$/);
  return (space > BEGINNING_CHARACTERS / 2 ? cut.slice(0, space) : cut).trimEnd();
}

/**
 * One item of the Issues drawer. Every entry reads: a first line, the severity's mark and the
 * message at body size, which is what the eye reads; beneath it one small muted line, the
 * severity's word, the code, the class where one is supplied, the source, and what else the
 * entry names (its references beyond the item's object, its shortened-text mark), with a
 * grouped item's count of reports as the control that opens them. Nothing else at rest. A
 * message longer than LONG_MESSAGE_CHARACTERS shows its beginning, cut at a word, and "Show
 * all" with its length; opened, the whole text stands beneath, selectable, unchanged.
 *
 * An item with one report is that report's entry. An item that gathers several reports of one
 * code on one object (a spring hanger's installed, cold and hot load, or the same finding from
 * the model checks and the solve) leads with its most severe report's message; its reports,
 * once opened, are each an entry in the same form, the message marking what tells it apart
 * from the others where the item may show excerpts, and a line naming the fields its producer
 * did not supply. A report that is a diagnostic opens its own detail wherever its item is
 * listed, or says why it cannot and shows the detail's supplied fields on its row. The item's
 * severity is its worst member's.
 */
export function IssueItemView({
  item,
  main,
  lead = item.lead,
  selectedDiagnosticId = null,
  onSelectDiagnostic,
  onSelectEntity
}: {
  item: IssueItem;
  /** The item's own row, given its first line. */
  main: (body: ReactNode) => ReactNode;
  /** The report the collapsed row leads with: the item's own lead unless a report was asked for. */
  lead?: IssueMember;
  selectedDiagnosticId?: string | null;
  onSelectDiagnostic?: (diagnosticId: string) => void;
  onSelectEntity?: (entity: EntityRef) => void;
}) {
  const [membersOpen, setMembersOpen] = useState(false);
  const membersId = useId();
  const grouped = item.members.length > 1;
  const shown = grouped ? lead : item.members[0];
  const long = useLongMessage(shown.shown.message);
  const origins = [...new Set(item.members.map((member) => member.origin))];
  return (
    <div className={grouped ? `issue-entry issue-group ${item.severity}` : "issue-entry"} data-testid={grouped ? `issue-group-${item.code}` : undefined}>
      {main(<FirstLine severity={grouped ? item.severity : shown.diagnostic.severity} text={long.text} />)}
      <p className="issue-meta muted" data-testid="issue-meta">
        {grouped ? item.severity : shown.diagnostic.severity} · {grouped ? item.code : shown.diagnostic.code}
        {(grouped ? itemClassText(item) : classText(shown.diagnostic)) ? (
          <> · <span className="issue-class" data-testid="issue-class">{grouped ? itemClassText(item) : classText(shown.diagnostic)}</span></>
        ) : null}
        {" "}· from {joinWords(origins.map((origin) => ISSUE_ORIGIN_LABELS[origin]))}
        {grouped ? null : <OwnRefs member={shown} single />}
        {grouped ? (
          <>
            {" "}·{" "}
            <button
              aria-controls={membersId}
              aria-expanded={membersOpen}
              aria-label={`All ${item.members.length} reports${item.object ? ` for ${item.object}` : ""}`}
              className="issue-inline-control"
              data-testid="issue-group-members-toggle"
              onClick={() => setMembersOpen((open) => !open)}
              type="button"
            >
              <span className="issue-group-count" data-testid="issue-group-count">{item.members.length} reports</span>
            </button>
          </>
        ) : null}
        {long.control}
      </p>
      {long.full}
      {grouped ? (
        <ol className="issue-group-members" data-testid="issue-group-members" hidden={!membersOpen} id={membersId}>
          {item.members.map((member, index) => (
            <li key={`${member.source}-${member.diagnostic.id ?? member.diagnostic.code}-${index}`}>
              <MemberRow
                member={member}
                difference={memberDifference(item, member)}
                selected={selectedDiagnosticId !== null && member.ownDetail && (member.diagnostic.id ?? member.diagnostic.code) === selectedDiagnosticId && member.source === "diagnostics"}
                onSelectDiagnostic={onSelectDiagnostic}
                onSelectEntity={onSelectEntity}
              />
            </li>
          ))}
        </ol>
      ) : null}
    </div>
  );
}

/** A message's text as its entry shows it at rest, and, where it is long, the "Show all" control
 * and, once opened, the whole text. */
function useLongMessage(message: string): { text: string; control: ReactNode; full: ReactNode } {
  const [open, setOpen] = useState(false);
  const fullId = useId();
  const beginning = messageBeginning(message);
  if (beginning === null) return { text: message, control: null, full: null };
  return {
    text: `${beginning}…`,
    control: (
      <>
        {" "}·{" "}
        <button aria-controls={fullId} aria-expanded={open} className="issue-inline-control" data-testid="issue-show-all" onClick={() => setOpen((value) => !value)} type="button">
          {open ? "Show less" : `Show all (${message.length.toLocaleString("en-US")} characters)`}
        </button>
      </>
    ),
    full: open ? <div className="issue-message-full" data-testid="issue-message-full" id={fullId}>{message}</div> : null
  };
}

const SEVERITY_MARKS = { blocking: OctagonAlert, error: CircleAlert, warning: TriangleAlert, info: Info } as const;

/** The entry's first line: the severity's mark, then the message; an excerpt is marked within it. */
function FirstLine({ severity, text, excerpt = null }: { severity: string; text: string; excerpt?: string | null }) {
  const Mark = SEVERITY_MARKS[severity as keyof typeof SEVERITY_MARKS] ?? Info;
  const at = excerpt && excerpt !== "(no other words)" ? text.indexOf(excerpt) : -1;
  return (
    <span className="issue-first-line">
      <Mark aria-hidden="true" className={`issue-severity-mark ${severity}`} size={14} />
      <span className="issue-message">
        {at >= 0 && excerpt ? (
          <>
            {text.slice(0, at)}
            <strong className="issue-member-difference" data-testid="issue-member-difference">{excerpt}</strong>
            {text.slice(at + excerpt.length)}
          </>
        ) : text}
      </span>
    </span>
  );
}

/** A report's references beyond its item's object, in display form, and its shortened-text mark,
 * as parts of its muted line. */
function OwnRefs({ member, single = false }: { member: IssueMember; single?: boolean }) {
  const truncated = (member.diagnostic as { truncated?: boolean }).truncated === true;
  return (
    <>
      {member.otherRefs.length > 0 ? (
        <> · <span className="issue-member-refs" data-testid="issue-member-refs">Also about: {member.otherRefs.join("; ")}</span></>
      ) : null}
      {truncated ? (
        <> · <span data-testid={single && member.source === "model-checks" ? "model-check-truncated" : "issue-member-truncated"}>Text shortened in the bounded report.</span></>
      ) : null}
    </>
  );
}

function MemberRow({ member, difference, selected, onSelectDiagnostic, onSelectEntity }: {
  member: IssueMember;
  difference: string | null;
  selected: boolean;
  onSelectDiagnostic?: (diagnosticId: string) => void;
  onSelectEntity?: (entity: EntityRef) => void;
}) {
  const { diagnostic, shown, source } = member;
  const long = useLongMessage(shown.message);
  const entity = member.finding?.entity ?? null;
  const act = source === "diagnostics" && onSelectDiagnostic && member.ownDetail
    ? () => onSelectDiagnostic(diagnostic.id ?? diagnostic.code)
    : source === "model-checks" && entity && onSelectEntity
      ? () => onSelectEntity(entity)
      : null;
  const noDetail = source === "diagnostics" && !member.ownDetail;
  return (
    <>
      <button
        aria-disabled={act === null}
        className={`issue-member diagnostic ${diagnostic.severity}${selected ? " active" : ""}`}
        data-source={source}
        data-testid={`issue-member-${diagnostic.code}`}
        onClick={() => act?.()}
        title={act ? (entity && source === "model-checks" ? `Select ${entity.id}` : undefined) : noDetail ? NO_OWN_DETAIL_REASON : "This report names nothing more to show."}
        type="button"
      >
        <FirstLine severity={diagnostic.severity} text={long.text} excerpt={difference} />
        <small className="issue-meta muted">
          {diagnostic.severity} · {diagnostic.code}
          {classText(diagnostic) ? <> · <span data-testid="issue-member-class">{classText(diagnostic)}</span></> : null}
          {" "}· from {ISSUE_ORIGIN_LABELS[member.origin]}
          <OwnRefs member={member} />
        </small>
        <NotSupplied diagnostic={diagnostic} />
        {noDetail ? <NoDetailFields diagnostic={diagnostic} /> : null}
      </button>
      {long.control ? <p className="issue-meta muted issue-member-show-all">{long.control}</p> : null}
      {long.full}
    </>
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
