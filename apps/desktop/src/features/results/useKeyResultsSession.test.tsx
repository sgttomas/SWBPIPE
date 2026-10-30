import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { healthStanding, row } from "../../test-support/resultOverlayFixtures";
import { COMBINED, keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { SUPPORT_LOAD_WITHHELD_NO_STATE } from "./supportLoadSummary";
import { SUPPORT_STATE_ROW_KIND } from "./keyResults";
import { useKeyResultsSession } from "./useKeyResultsSession";

// A6's session wiring (DEL-07-05 Design `key-results-panel.md` §2, §4): each "Show support load
// summary" is a new request for A2, and "Show contact and lift-off state" asks the results table for
// the exact kind with the standing's withheld rows marked. Invented data only.

function mount(result = keyResult()) {
  const setActiveSection = vi.fn();
  const selectEntity = vi.fn();
  const hook = renderHook(() => useKeyResultsSession({
    result,
    standing: healthStanding({ eligible: true }),
    historical: false,
    setActiveSection,
    selectEntity,
    viewCommandRef: { current: null },
    enterModelStage: vi.fn(),
    openRuleChecks: vi.fn()
  }));
  return { hook, setActiveSection, selectEntity };
}

describe("useKeyResultsSession", () => {
  it("makes every Show support load summary a new request, for the same support too", () => {
    const { hook, selectEntity, setActiveSection } = mount();
    expect(hook.result.current.supportRequest).toBe(0);
    act(() => hook.result.current.actions.onShowSupport("support:S-A"));
    expect(hook.result.current.supportRequest).toBe(1);
    act(() => hook.result.current.actions.onShowSupport("support:S-A"));
    expect(hook.result.current.supportRequest).toBe(2);
    expect(selectEntity).toHaveBeenNthCalledWith(2, { type: "support", id: "support:S-A" });
    expect(setActiveSection).toHaveBeenCalledWith("results");
  });

  it("asks the results table for exactly the support-state kind, with the withheld rows and their reasons", () => {
    const { hook } = mount(keyResult([...keyRows(), row("state:support:S-D:comb:C", SUPPORT_STATE_ROW_KIND, "support:S-D", 1, "state_code", { basis_ref: COMBINED })]));
    act(() => hook.result.current.actions.onShowContactStatus());
    expect(hook.result.current.resultsRequest).toMatchObject({
      runId: "run:invented",
      kind: SUPPORT_STATE_ROW_KIND,
      withheld: { "state:support:S-D:comb:C": SUPPORT_LOAD_WITHHELD_NO_STATE }
    });
  });
});
