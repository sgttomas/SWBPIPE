import { ListChecks } from "lucide-react";
import { useEffect, useId, useMemo, useRef, type KeyboardEvent } from "react";
import type { EntityRef, PreviewModel } from "../../types";
import { MODEL_CHECK_GROUP_LABELS, type ModelCheckFinding, type ModelCheckGroupId, type ModelCheckRun } from "./modelChecks";

/** The command's keyboard equivalent, as the menus show it. */
export const MODEL_CHECKS_SHORTCUT = "⇧⌘M";
/** The same keys in `aria-keyshortcuts` form (Command on macOS, Control elsewhere). */
export const MODEL_CHECKS_ARIA_SHORTCUTS = "Shift+Meta+M Shift+Control+M";

const GROUP_ORDER: readonly ModelCheckGroupId[] = ["solve", "slots", "library"];
/** The runs whose outcome has had keyboard focus once. */
const announcedRuns = new WeakSet<ModelCheckRun>();

/**
 * On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`). The
 * Analyze menu's Check Model (⇧⌘M) or this panel's button runs the
 * model's existing checks (the session's `handleRunModelChecks`), and the panel
 * lists their findings with each check's own code and text. Choosing a finding
 * selects the entity it names. The checks read the model and never change it;
 * a later edit marks the list as out of date until the checks run again.
 */
export function ModelChecksPanel({
  model,
  run,
  busy = false,
  onRun,
  onSelectEntity
}: {
  model: PreviewModel;
  /** The last run of the checks in this session; null before the first. */
  run: ModelCheckRun | null;
  /** A run is in progress (the native check is admitted and running). */
  busy?: boolean;
  onRun: () => void;
  onSelectEntity: (entity: EntityRef) => void;
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

  const grouped = useMemo(() => GROUP_ORDER.map((group) => [group, run?.findings.filter((item) => item.group === group) ?? []] as const), [run]);
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
        {run ? summaryText(run) : "The checks have not been run on this model. They read the model and never change it."}
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
      {run && run.solve.kind !== "checked" ? (
        <p className="memory-pool-message warning" data-testid="model-checks-solve-unchecked">
          {run.solve.kind === "refused"
            ? `The solve's checks did not run: the solve cannot read this model (${run.solve.refusal}).`
            : `The model checks did not run: ${run.solve.reason}`}
        </p>
      ) : null}
      {run ? (
        <ol aria-label="Model check findings" className="model-checks-list" data-testid="model-checks-findings" onKeyDown={moveFocus} ref={listRef}>
          {grouped.map(([group, findings]) => (
            <li className="model-checks-group" data-group={group} key={group}>
              <strong>{MODEL_CHECK_GROUP_LABELS[group]}</strong>{" "}
              <span className="muted">
                {groupCount(run, group, findings.length)}
              </span>
              {findings.length > 0 ? (
                <ul>
                  {findings.map((finding) => (
                    <li key={finding.key}>
                      <FindingButton finding={finding} onSelectEntity={onSelectEntity} />
                    </li>
                  ))}
                </ul>
              ) : null}
            </li>
          ))}
        </ol>
      ) : null}
    </section>
  );
}

function FindingButton({ finding, onSelectEntity }: { finding: ModelCheckFinding; onSelectEntity: (entity: EntityRef) => void }) {
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
      <p>{diagnostic.message}</p>
      {entity ? <small>Selects {entity.id}</small> : null}
      {(diagnostic as { truncated?: boolean }).truncated ? <small data-testid="model-check-truncated"> Text shortened in the bounded report.</small> : null}
    </button>
  );
}

function notShownTotal(run: ModelCheckRun): number {
  return run.notShown.solve + run.notShown.slots + run.notShown.library;
}

function groupCount(run: ModelCheckRun, group: ModelCheckGroupId, listed: number): string {
  if (run.solve.kind === "unavailable" || (group === "solve" && run.solve.kind !== "checked")) return "not run";
  const counted = run.notShown[group];
  return `${listed} ${listed === 1 ? "finding" : "findings"}${counted > 0 ? `; ${counted} more not listed` : ""}`;
}

function summaryText(run: ModelCheckRun): string {
  const count = (severity: string) => run.findings.filter((item) => item.diagnostic.severity === severity).length;
  const blocking = count("blocking") + count("error");
  const total = run.findings.length;
  if (total === 0) return run.solve.kind === "checked" ? "The checks found nothing to report." : "The checks that ran found nothing to report.";
  return `${total} ${total === 1 ? "finding" : "findings"}: ${blocking} blocking or error, ${count("warning")} ${count("warning") === 1 ? "warning" : "warnings"}, ${count("info")} info.`;
}
