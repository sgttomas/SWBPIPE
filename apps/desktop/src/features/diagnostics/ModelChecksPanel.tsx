import { ListChecks } from "lucide-react";
import { useEffect, useId, useMemo, useRef, type KeyboardEvent, type ReactNode } from "react";
import type { EntityRef, PreviewModel } from "../../types";
import { MODEL_CHECK_GROUP_LABELS, type ModelCheckFinding, type ModelCheckGroupId, type ModelCheckRun } from "./modelChecks";
import { groupIssues, type IssueItem } from "./issueItems";
import { IssueItemView } from "./IssueItemView";

/** The command's keyboard equivalent, as the menus show it. */
export const MODEL_CHECKS_SHORTCUT = "⇧⌘M";
/** The same keys in `aria-keyshortcuts` form (Command on macOS, Control elsewhere). */
export const MODEL_CHECKS_ARIA_SHORTCUTS = "Shift+Meta+M Shift+Control+M";

const GROUP_ORDER: readonly ModelCheckGroupId[] = ["solve", "slots", "library"];
/** The runs whose outcome has had keyboard focus once. */
const announcedRuns = new WeakSet<ModelCheckRun>();

/**
 * On-demand model checks. The
 * Analyze menu's Check Model (⇧⌘M) or this panel's button runs the
 * model's existing checks (the session's `handleRunModelChecks`), and the panel
 * lists their findings with each check's own code and text. Choosing a finding
 * selects the entity it names. The checks read the model and never change it;
 * a later edit marks the list as out of date until the checks run again.
 * Findings that repeat one code on one object are listed as one item
 * (`issueItems.ts`); in the Issues drawer an item also gathers the same
 * finding where the solve or the model reported it.
 */
export function ModelChecksPanel({
  model,
  run,
  busy = false,
  onRun,
  onSelectEntity,
  items: givenItems
}: {
  model: PreviewModel;
  /** The last run of the checks in this session; null before the first. */
  run: ModelCheckRun | null;
  /** A run is in progress (the native check is admitted and running). */
  busy?: boolean;
  onRun: () => void;
  onSelectEntity: (entity: EntityRef) => void;
  /** The drawer's items this list shows; without them the run's findings are grouped here. */
  items?: readonly IssueItem[];
}) {
  const summaryRef = useRef<HTMLParagraphElement>(null);
  const listRef = useRef<HTMLOListElement>(null);
  const titleId = useId();
  // A new run moves keyboard focus to its outcome, which is announced; showing
  // an earlier run again (the drawer reopened) does not.
  useEffect(() => {
    if (run === null || announcedRuns.has(run)) return;
    announcedRuns.add(run);
    summaryRef.current?.focus();
  }, [run]);

  const items = useMemo(() => givenItems ?? groupIssues({ modelChecks: run?.findings ?? [] }), [givenItems, run]);
  const grouped = useMemo(() => GROUP_ORDER.map((group) => [
    group,
    items.filter((item) => item.group === group),
    run?.findings.filter((finding) => finding.group === group).length ?? 0
  ] as const), [items, run]);
  const stale = run !== null && run.model !== model;

  function moveFocus(event: KeyboardEvent<HTMLOListElement>) {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp" && event.key !== "Home" && event.key !== "End") return;
    const buttons = Array.from(listRef.current?.querySelectorAll<HTMLButtonElement>("button[data-finding]") ?? []);
    if (buttons.length === 0) return;
    const at = buttons.indexOf(document.activeElement as HTMLButtonElement);
    const next = event.key === "Home" ? 0
      : event.key === "End" ? buttons.length - 1
      : event.key === "ArrowDown" ? Math.min(buttons.length - 1, at + 1)
      : Math.max(0, at - 1);
    event.preventDefault();
    buttons[next]?.focus();
  }

  return (
    <section aria-labelledby={titleId} className="panel model-checks-panel" data-testid="model-checks-panel">
      <div className="panel-title" id={titleId}>
        <ListChecks size={16} aria-hidden="true" />
        Model checks
        {givenItems ? <span className="issues-section-count" data-testid="model-checks-item-count" aria-label={`${items.length} ${items.length === 1 ? "item" : "items"}`}>{items.length}</span> : null}
      </div>
      <div className="model-checks-actions">
        <button
          aria-keyshortcuts={MODEL_CHECKS_ARIA_SHORTCUTS}
          className="secondary-action"
          data-testid="model-checks-run"
          aria-disabled={busy}
          onClick={() => { if (!busy) onRun(); }}
          type="button"
        >
          {busy ? "Running model checks" : run ? "Run model checks again" : "Run model checks"}
        </button>
        <kbd>{MODEL_CHECKS_SHORTCUT}</kbd>
      </div>
      <p aria-live="polite" data-testid="model-checks-summary" ref={summaryRef} role="status" tabIndex={-1}>
        {run ? summaryText(run, items.length) : "The checks have not been run on this model. They read the model and never change it."}
      </p>
      {stale ? (
        <p className="memory-pool-message warning" data-testid="model-checks-stale">
          The model has changed since these checks ran; run them again to check the current model.
        </p>
      ) : null}
      {run && notShownTotal(run) > 0 ? (
        <p className="memory-pool-message info" data-testid="model-checks-not-shown">
          {notShownTotal(run)} more {notShownTotal(run) === 1 ? "finding is" : "findings are"} not listed: the report is bounded. Correct the listed findings and run the checks again.
        </p>
      ) : null}
      {run?.laterCheckNotCompleted !== undefined ? (
        <p className="memory-pool-message warning" data-testid="model-checks-later-not-completed">
          {laterCheckNotCompletedText(run.laterCheckNotCompleted, !stale, isRefusalOnly(run))}
        </p>
      ) : null}
      {run && run.solve.kind !== "checked" ? (
        <p className="memory-pool-message warning" data-testid="model-checks-solve-unchecked">
          {run.solve.kind === "refused"
            ? `The solve's checks did not run: the solve cannot read this model (${run.solve.refusal}).`
            : run.solve.started ? `The model checks did not complete: ${run.solve.reason}` : `The model checks did not run: ${run.solve.reason}`}
        </p>
      ) : null}
      {run?.libraryNotCompleted !== undefined ? (
        <p className="memory-pool-message warning" data-testid="model-checks-library-not-completed">
          The library-reference findings could not be collected ({run.libraryNotCompleted}); that group is not complete.
        </p>
      ) : null}
      {run ? (
        <ol aria-label="Model check findings" className="model-checks-list" data-testid="model-checks-findings" onKeyDown={moveFocus} ref={listRef}>
          {grouped.map(([group, groupItems, findingCount]) => (
            <li className="model-checks-group" data-group={group} key={group}>
              <strong>{MODEL_CHECK_GROUP_LABELS[group]}</strong>{" "}
              <span className="muted">
                {groupCount(run, group, groupItems.length, findingCount)}
              </span>
              {groupItems.length > 0 ? (
                <ul>
                  {groupItems.map((item) => {
                    const finding = item.members.find((member) => member.finding)?.finding;
                    return finding ? (
                      <li key={item.key}>
                        <IssueItemView
                          item={item}
                          main={(body) => <FindingButton finding={finding} onSelectEntity={onSelectEntity} body={body} />}
                          onSelectEntity={onSelectEntity}
                        />
                      </li>
                    ) : null;
                  })}
                </ul>
              ) : null}
            </li>
          ))}
        </ol>
      ) : null}
    </section>
  );
}

/** One finding as the Model checks list shows it: choosing it selects the entity
 * it names. The Solve panel's withheld notice shows its findings with it too. */
export function FindingButton({ finding, onSelectEntity, body }: { finding: ModelCheckFinding; onSelectEntity: (entity: EntityRef) => void; body?: ReactNode }) {
  const { diagnostic, entity } = finding;
  return (
    <button
      aria-disabled={entity === null}
      className={`diagnostic ${diagnostic.severity}`}
      data-entity={entity ? `${entity.type}:${entity.id}` : undefined}
      data-finding={finding.key}
      data-testid={`model-check-${diagnostic.code}`}
      onClick={() => { if (entity) onSelectEntity(entity); }}
      title={entity ? `Select ${entity.id}` : "This finding names no model entity to select."}
      type="button"
    >
      <strong>{diagnostic.code}</strong> <span className="muted">{diagnostic.severity}</span>
      {body ?? <p>{diagnostic.message}</p>}
      {entity ? <small>Selects {entity.id}</small> : null}
      {(diagnostic as { truncated?: boolean }).truncated ? <small data-testid="model-check-truncated"> Text shortened in the bounded report.</small> : null}
    </button>
  );
}

/** The sentence beside evidence kept over a later check that could not finish. The
 * evidence is said to still apply only while the run is of the model value the
 * session holds; for an earlier model value it is named as about that model. A kept
 * refusal with no findings is named as the refusal, never as findings. The kept
 * evidence follows the sentence. */
export function laterCheckNotCompletedText(reason: string, sameModel: boolean, refusalOnly: boolean): string {
  const kept = refusalOnly ? "The refusal below is" : "The findings below are";
  return sameModel
    ? `A later run of the checks on this model could not finish (${reason}). ${kept} from the last run that finished, and ${refusalOnly ? "it still applies" : "they still apply"}.`
    : `A later run of the checks on the earlier model could not finish (${reason}). ${kept} from the last run that finished on that earlier model.`;
}

/** The solve refused to read the model, and its checks list no finding. */
function isRefusalOnly(run: ModelCheckRun): boolean {
  return run.solve.kind === "refused" && !run.findings.some((finding) => finding.group === "solve");
}

function notShownTotal(run: ModelCheckRun): number {
  return run.notShown.solve + run.notShown.slots + run.notShown.library;
}

function groupCount(run: ModelCheckRun, group: ModelCheckGroupId, listed: number, findings: number): string {
  if (run.solve.kind === "unavailable") return run.solve.started ? "not completed" : "not run";
  if (group === "solve" && run.solve.kind !== "checked") return "not run";
  if (group === "library" && run.libraryNotCompleted !== undefined) return "not completed";
  const counted = run.notShown[group];
  const shown = listed === findings
    ? `${listed} ${listed === 1 ? "finding" : "findings"}`
    : `${listed} ${listed === 1 ? "item" : "items"} from ${findings} ${findings === 1 ? "finding" : "findings"}`;
  return `${shown}${counted > 0 ? `; ${counted} more not listed` : ""}`;
}

function summaryText(run: ModelCheckRun, items: number): string {
  const count = (severity: string) => run.findings.filter((item) => item.diagnostic.severity === severity).length;
  const blocking = count("blocking") + count("error");
  const total = run.findings.length;
  const omitted = notShownTotal(run);
  // Codex review r3 of #1665: a run whose findings were all counted rather
  // than listed has findings; it is not an empty result.
  const omittedBlocking = `${run.notShownBlocking} of them blocking or error`;
  if (total === 0 && omitted === 0) return run.solve.kind === "checked" ? "The checks found nothing to report." : "The checks that ran found nothing to report.";
  if (total === 0) return `${omitted} ${omitted === 1 ? "finding" : "findings"}, none listed; ${omittedBlocking}.`;
  const listed = `${total} ${total === 1 ? "finding" : "findings"}: ${blocking} blocking or error, ${count("warning")} ${count("warning") === 1 ? "warning" : "warnings"}, ${count("info")} info.`;
  const asItems = items !== total ? ` Listed as ${items} ${items === 1 ? "item" : "items"}: a finding repeated on one object is one item.` : "";
  return omitted > 0 ? `${listed} ${omitted} more not listed; ${omittedBlocking}.${asItems}` : `${listed}${asItems}`;
}
