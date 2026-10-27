/** The key-results panel's session wiring (survey item A6; DEL-07-05 Design
 * `key-results-panel.md` §2, §3). It holds the panel's open state and the "Open after every solve"
 * choice as session view state (never persisted, never model data), applies the auto-open rule to
 * each published result without moving focus, and turns each panel action into the existing
 * setter it names: the viewport's view command, the session's selection, the Results section and
 * the results table's filter. It writes no model data and records no undo entry. */

import { useEffect, useRef, useState, type RefObject } from "react";
import type { EntityRef, MechanicsResult } from "../../types";
import type { ViewportViewCommand } from "../viewport/viewportSelection";
import type { WorkspaceSectionId } from "../workspace/workspaceSections";
import { canSurfaceRevealNow } from "../workspace/surfaceReveal";
import type { KeyResultsTable } from "./KeyResultsPanel";
import type { ResultsViewRequest } from "./ResultsPanel";
import { KEY_RESULTS_AUTO_OPEN_START, SUPPORT_STATE_ROW_KIND, keyResultsAutoOpen, withheldSupportStateRows } from "./keyResults";
import type { ResultOverlayStanding } from "./numericalHealth";

/** The Results section's panels the actions reveal, by their default test ids. */
export const KEY_RESULTS_REVEAL_TARGETS: Readonly<Record<KeyResultsTable | "support", string>> = Object.freeze({
  results: "results-panel",
  "stress-ratio": "stress-ratio-panel",
  "displacement-extremes": "displacement-extremes-panel",
  support: "support-load-summary"
});

export function useKeyResultsSession(input: {
  result: MechanicsResult | null;
  /** App's `viewportResultStanding`: marks the support-state rows it withholds. */
  standing: ResultOverlayStanding | null | undefined;
  historical: boolean;
  setActiveSection: (section: WorkspaceSectionId | null) => void;
  selectEntity: (entity: EntityRef) => void;
  viewCommandRef: RefObject<((command: ViewportViewCommand) => void) | null>;
  enterModelStage: () => void;
  openRuleChecks: () => void;
}) {
  const { result, historical } = input;
  const [open, setOpen] = useState(false);
  const [everySolve, setEverySolve] = useState(false);
  const [resultsRequest, setResultsRequest] = useState<ResultsViewRequest | null>(null);
  /** Counts "Show support load summary" requests, so A2 shows the support each time, even when
   * the session's selection is unchanged and the user stepped A2 to another support meanwhile. */
  const [supportRequest, setSupportRequest] = useState(0);
  const [reveal, setReveal] = useState<{ testId: string; serial: number } | null>(null);
  const serial = useRef(0);
  const panelRef = useRef<HTMLElement | null>(null);
  /** The element focused when the panel was opened on demand; focus returns there on close. */
  const returnFocus = useRef<HTMLElement | null>(null);
  const autoOpen = useRef(KEY_RESULTS_AUTO_OPEN_START);

  // Auto-open (§3): decided once per published result. Opening renders the panel and moves no focus.
  useEffect(() => {
    const decision = keyResultsAutoOpen(autoOpen.current, { result, historical, everySolve });
    autoOpen.current = decision.memory;
    if (decision.open) setOpen(true);
  }, [result, historical, everySolve]);

  // A revealed panel is scrolled into view once laid out; focus stays where the user is.
  useEffect(() => {
    if (!reveal) return;
    const host = document.querySelector(`[data-testid="${reveal.testId}"]`);
    if (canSurfaceRevealNow(host)) host.scrollIntoView?.({ block: "nearest" });
  }, [reveal]);

  function revealInResults(testId: string) {
    input.setActiveSection("results");
    serial.current += 1;
    setReveal({ testId, serial: serial.current });
  }

  function toggle() {
    if (open) { close(); return; }
    const active = document.activeElement;
    returnFocus.current = active instanceof HTMLElement && active !== document.body ? active : null;
    setOpen(true);
  }

  /** Closes the panel. Only focus that was inside it moves, so it is not lost with the panel: to
   * where it was when the panel was opened on demand, else to the rail's button for `stage`. */
  function close(stage: string | null = null) {
    const hadFocus = panelRef.current?.contains(document.activeElement) ?? false;
    setOpen(false);
    const target = returnFocus.current;
    returnFocus.current = null;
    if (!hadFocus) return;
    if (target?.isConnected && stage === null) { target.focus(); return; }
    const rail = document.querySelector<HTMLElement>(stage ? `[data-testid="rail-stage-${stage}"]` : '[data-testid^="rail-stage-"][aria-current="page"]');
    rail?.focus();
  }

  return {
    open,
    toggle,
    close,
    everySolve,
    setEverySolve,
    panelRef,
    supportRequest,
    /** For the results table: a request for the current run only. */
    resultsRequest: resultsRequest && result && resultsRequest.runId === result.run_id ? resultsRequest : null,
    actions: {
      onShowRatioContour: (quantityId: string, caseKey: string) => input.viewCommandRef.current?.({ type: "show-result-colour", quantityId, caseKey }),
      onShowDeflectedShape: (caseKey: string) => input.viewCommandRef.current?.({ type: "show-deflected-shape", caseKey }),
      onShowSupport: (supportId: string) => {
        input.selectEntity({ type: "support", id: supportId });
        setSupportRequest((count) => count + 1);
        revealInResults(KEY_RESULTS_REVEAL_TARGETS.support);
      },
      onShowContactStatus: () => {
        if (!result) return;
        serial.current += 1;
        setResultsRequest({ serial: serial.current, runId: result.run_id, kind: SUPPORT_STATE_ROW_KIND, withheld: withheldSupportStateRows(result, input.standing) });
        revealInResults(KEY_RESULTS_REVEAL_TARGETS.results);
      },
      onOpenTable: (table: KeyResultsTable) => revealInResults(KEY_RESULTS_REVEAL_TARGETS[table]),
      onBackToEditing: () => {
        input.enterModelStage();
        close("model");
      },
      onOpenRuleChecks: () => input.openRuleChecks(),
      onClose: () => close()
    }
  };
}
