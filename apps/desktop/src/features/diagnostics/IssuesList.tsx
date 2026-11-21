import { useId, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import type { EntityRef } from "../../types";
import type { MenuCommandId } from "../workspace/menuCommands";
import { countIssues, issueCountLine, issueSourceLabel, type IssueList, type IssueRow, type IssueTier } from "./issueRows";
import "./IssuesList.css";

/** The word each row shows for where it stands. */
export const ISSUE_TIER_WORDS: Readonly<Record<IssueTier, string>> = Object.freeze({
  fix: "Fix before Run",
  warning: "Warning",
  note: "Note"
});

/**
 * The Issues list (`buildIssueList`) with its count line (`countIssues`): one row per fact, its
 * sentence first, then each entity it names (pressing one selects it), the word for where it
 * stands, the act that repairs it where one act does, and its code. Pressing the sentence opens the
 * row in place: its code, severity, class, members, the objects and references it names, where it
 * came from, its remediation and provenance (a field not supplied says so), and what else the
 * caller's detail shows for it. The solver's messages heading stands at the head of the list while
 * any row is one of the solver's own messages; the caller gives its words.
 */
export function IssuesList({
  list,
  onSelectEntity,
  onAct,
  solverMessagesHeading,
  renderDetail,
  onOpenRow
}: {
  list: IssueList;
  onSelectEntity: (entity: EntityRef) => void;
  onAct: (command: MenuCommandId) => void;
  /** The heading shown above the solver's own messages (`SOLVER_MESSAGES_HEADING`). */
  solverMessagesHeading?: string;
  /** What else an opened row shows: linked results, explanation, knowledge, boundary. */
  renderDetail?: (row: IssueRow) => ReactNode;
  /** A row was opened (to select its diagnostic, which the detail reads). */
  onOpenRow?: (row: IssueRow) => void;
}) {
  const [open, setOpen] = useState<ReadonlySet<string>>(() => new Set());
  const listRef = useRef<HTMLOListElement>(null);
  const baseId = useId();
  const count = countIssues(list);
  const showsSolverMessage = list.rows.some((row) => row.solverMessage);

  function toggle(row: IssueRow) {
    const opening = !open.has(row.key);
    setOpen((current) => {
      const next = new Set(current);
      if (opening) next.add(row.key);
      else next.delete(row.key);
      return next;
    });
    if (opening) onOpenRow?.(row);
  }

  function moveFocus(event: KeyboardEvent<HTMLOListElement>) {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp" && event.key !== "Home" && event.key !== "End") return;
    const sentences = Array.from(listRef.current?.querySelectorAll<HTMLButtonElement>("button[data-issue-sentence]") ?? []);
    const at = sentences.indexOf(document.activeElement as HTMLButtonElement);
    if (sentences.length === 0 || at === -1) return;
    const next = event.key === "Home" ? 0
      : event.key === "End" ? sentences.length - 1
      : event.key === "ArrowDown" ? Math.min(sentences.length - 1, at + 1)
      : Math.max(0, at - 1);
    event.preventDefault();
    sentences[next]?.focus();
  }

  return (
    <section aria-label="Issues" className="issues-list" data-testid="issues-list">
      <p aria-live="polite" className="issues-count-line" data-testid="issues-count-line" role="status">{issueCountLine(count)}</p>
      {showsSolverMessage && solverMessagesHeading ? <p className="muted" data-testid="issues-solver-messages-heading">{solverMessagesHeading}</p> : null}
      {list.rows.length > 0 ? (
        <ol aria-label="Issues to read" className="issues-rows" onKeyDown={moveFocus} ref={listRef}>
          {list.rows.map((row, index) => {
            const detailId = `${baseId}-${index}`;
            const isOpen = open.has(row.key);
            return (
              <li className={`issue-row ${row.tier}`} data-testid="issue-row" data-tier={row.tier} data-code={row.code} key={row.key}>
                <button aria-controls={detailId} aria-expanded={isOpen} className="issue-sentence" data-issue-sentence="" onClick={() => toggle(row)} type="button">
                  {row.message}
                </button>
                <div className="issue-meta">
                  {row.entities.map((entity) => (
                    <button aria-label={`Select ${entity.id}`} className="issue-entity" data-testid="issue-entity" key={`${entity.type}:${entity.id}`} onClick={() => onSelectEntity(entity)} type="button">
                      {entity.id}
                    </button>
                  ))}
                  <span className="issue-tier" data-testid="issue-tier">{ISSUE_TIER_WORDS[row.tier]}</span>
                  {row.act ? (
                    <button className="issue-act" data-testid="issue-act" onClick={() => onAct(row.act!.command)} type="button">
                      {row.act.label}
                    </button>
                  ) : null}
                  {row.code ? <small className="issue-code" data-testid="issue-code">{row.code}</small> : null}
                </div>
                {isOpen ? <IssueDetail id={detailId} row={row} extra={renderDetail?.(row)} /> : null}
              </li>
            );
          })}
        </ol>
      ) : null}
    </section>
  );
}

/** A field the producer did not supply (DEL-07-04 REQ-004), in the present detail pane's words. */
const NOT_SUPPLIED = "not supplied by producer";

function IssueDetail({ id, row, extra }: { id: string; row: IssueRow; extra: ReactNode }) {
  const refs = [...new Set(row.sources.flatMap((source) => source.diagnostic.affected_refs ?? []))];
  const remediation = row.sources.map((source) => source.diagnostic.remediation).find(Boolean) ?? NOT_SUPPLIED;
  const kind = row.sources.map((source) => source.diagnostic.diagnostic_class ?? source.diagnostic.class).find(Boolean) ?? NOT_SUPPLIED;
  const provenance = row.sources.map((source) => source.diagnostic.provenance).find(Boolean);
  // Each entry's own sentence where it differs from the row's (a member's, or the result's
  // presented copy), so nothing an entry says is lost.
  const otherMessages = [...new Set(row.sources.map((source) => source.diagnostic.message))].filter((message) => message !== row.message);
  return (
    <dl className="issue-detail" data-testid="issue-detail" id={id}>
      {row.code ? <IssueLine label="Code" value={row.code} /> : null}
      <IssueLine label="Severity" value={row.severity} />
      {row.members.length > 0 ? <IssueLine label="Members" value={row.members.join(", ")} /> : null}
      <IssueLine label="Model objects" value={row.entities.length > 0 ? row.entities.map((entity) => entity.id).join(", ") : "none named"} />
      <IssueLine label="Affected refs" value={refs.length > 0 ? refs.join(", ") : "none"} />
      <IssueLine
        label={row.sources.length === 1 ? "Source" : "Sources"}
        value={
          <ul>
            {row.sources.map((source, index) => (
              <li key={index}>
                {issueSourceLabel(source)}: {source.diagnostic.source ?? "not stated"}
                {source.diagnostic.id ? <> (<code>{source.diagnostic.id}</code>)</> : null}
              </li>
            ))}
          </ul>
        }
      />
      <IssueLine label="Class" value={kind} />
      {otherMessages.length > 0 ? (
        <IssueLine label="Also stated as" value={<ul>{otherMessages.map((message) => <li key={message}>{message}</li>)}</ul>} />
      ) : null}
      <IssueLine label="Remediation" value={remediation} />
      <IssueLine label="Provenance" value={provenance === undefined ? NOT_SUPPLIED : typeof provenance === "string" ? provenance : JSON.stringify(provenance)} />
      {extra}
    </dl>
  );
}

function IssueLine({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
