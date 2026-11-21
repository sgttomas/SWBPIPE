/** The tabs under the Results stage's header: one table at a time in place of one long scroll.
 * Each tab keeps its table mounted while another is shown, so its filters, sort and page survive a
 * switch; an inactive tab is hidden from layout and from the accessibility tree. */
import { useId, type KeyboardEvent, type ReactNode } from "react";

export type StageTab = Readonly<{ id: string; label: string; content: ReactNode }>;

export function ResultsStageTabs({ tabs, current, onChoose, toolbar }: { tabs: readonly StageTab[]; current: string; onChoose: (id: string) => void; toolbar?: ReactNode }) {
  const prefix = useId();
  const active = tabs.some((tab) => tab.id === current) ? current : tabs[0]?.id;
  const tabId = (id: string) => `${prefix}-tab-${id}`;
  const panelId = (id: string) => `${prefix}-panel-${id}`;
  function onKeyDown(event: KeyboardEvent<HTMLElement>) {
    const at = tabs.findIndex((tab) => tab.id === active);
    const next = event.key === "ArrowRight" ? at + 1 : event.key === "ArrowLeft" ? at - 1 : event.key === "Home" ? 0 : event.key === "End" ? tabs.length - 1 : null;
    if (next === null || event.altKey || event.ctrlKey || event.metaKey) return;
    event.preventDefault();
    const target = tabs[(next + tabs.length) % tabs.length];
    onChoose(target.id);
    document.getElementById(tabId(target.id))?.focus();
  }
  return (
    <>
      <div className="results-stage-tabbar">
        <div className="results-stage-tabs" role="tablist" aria-label="Results tables" data-testid="results-stage-tabs" onKeyDown={onKeyDown}>
          {tabs.map((tab) => (
            <button key={tab.id} type="button" role="tab" id={tabId(tab.id)} aria-controls={panelId(tab.id)} aria-selected={tab.id === active}
              tabIndex={tab.id === active ? 0 : -1} data-testid={`results-stage-tab-${tab.id}`} onClick={() => onChoose(tab.id)}>
              {tab.label}
            </button>
          ))}
        </div>
        {toolbar ? <div className="results-stage-toolbar">{toolbar}</div> : null}
      </div>
      {tabs.map((tab) => (
        <div key={tab.id} role="tabpanel" id={panelId(tab.id)} aria-labelledby={tabId(tab.id)} hidden={tab.id !== active}
          className="results-stage-tabpanel" data-testid={`results-stage-content-${tab.id}`}>
          {tab.content}
        </div>
      ))}
    </>
  );
}

/** The caller's tabs with the panel's own "All rows" in its place: where the caller lists the id,
 * else after Displacements, else first. */
export function withRowsTab(tabs: readonly StageTab[], rows: StageTab): StageTab[] {
  if (tabs.some((tab) => tab.id === rows.id)) return tabs.map((tab) => (tab.id === rows.id ? { ...tab, content: rows.content } : tab));
  const after = tabs.findIndex((tab) => tab.id === "displacements");
  return after < 0 ? [rows, ...tabs] : [...tabs.slice(0, after + 1), rows, ...tabs.slice(after + 1)];
}
