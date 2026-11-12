import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { healthStanding, row } from "../../test-support/resultOverlayFixtures";
import { COMBINED, keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { createNativeResult, type NativeCustody, type SessionResult } from "../../services/nativeResult";
import type { ResultOverlayStanding } from "./numericalHealth";
import { SUPPORT_LOAD_WITHHELD_NO_STATE, supportLoadWithheldStateText } from "./supportLoadSummary";
import { SUPPORT_STATE_ROW_KIND } from "./keyResults";
import { useKeyResultsSession } from "./useKeyResultsSession";

// A6's session wiring: each "Show support load
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

/** The hook re-rendered with each result the session publishes in turn, each with its standing. */
function mountSession(first: { result: SessionResult | null; standing: ResultOverlayStanding | null }) {
  return renderHook((props: { result: SessionResult | null; standing: ResultOverlayStanding | null }) => useKeyResultsSession({
    result: props.result,
    standing: props.standing,
    historical: false,
    setActiveSection: vi.fn(),
    selectEntity: vi.fn(),
    viewCommandRef: { current: null },
    enterModelStage: vi.fn(),
    openRuleChecks: vi.fn()
  }), { initialProps: first });
}

// Every solve publishes the same run id ("run:preview-linear-static-001" in the producer); the
// invented results here share "run:invented" in the same way. A and B are two solves of the same
// run id under different standings: A withholds the combination's row, B the L-1 row.
const COMBINED_ROW = row("state:support:S-D:comb:C", SUPPORT_STATE_ROW_KIND, "support:S-D", 1, "state_code", { basis_ref: COMBINED });
const SOLVE_A = () => ({ result: keyResult([...keyRows(), COMBINED_ROW]), standing: healthStanding({ eligible: true }) });
const SOLVE_B = () => ({ result: keyResult(keyRows()), standing: healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } }) });

/** A native handle carrying only what the hook reads: S's envelope and its withheld rows table. */
function nativeSolve(withheld: Record<string, string>): SessionResult {
  const envelope = { run_id: "run:preview-linear-static-001", status: { mechanics: "MECHANICS_SOLVED", rule_check: "not_run", professional_acceptance: "not_claimed" } };
  return createNativeResult({ summary: { header: { envelope, tables: { withheld_support_state_rows: withheld } } } } as unknown as NativeCustody);
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

  it("keeps the request while the session's result is the same object", () => {
    const a = SOLVE_A();
    const hook = mountSession(a);
    act(() => hook.result.current.actions.onShowContactStatus());
    const request = hook.result.current.resultsRequest;
    expect(request).toMatchObject({ withheld: { [COMBINED_ROW.id]: SUPPORT_LOAD_WITHHELD_NO_STATE } });
    hook.rerender({ ...a });
    expect(hook.result.current.resultsRequest).toBe(request);
  });

  it("drops the request when the session publishes another result object with the same run id", () => {
    const a = SOLVE_A();
    const b = SOLVE_B();
    expect(b.result.run_id).toBe(a.result.run_id);
    const hook = mountSession(a);
    act(() => hook.result.current.actions.onShowContactStatus());
    expect(hook.result.current.resultsRequest).not.toBeNull();
    hook.rerender(b);
    expect(hook.result.current.resultsRequest).toBeNull();
    // Returning to A's object does not bring the dropped request back: it was released, not hidden.
    hook.rerender(a);
    expect(hook.result.current.resultsRequest).toBeNull();
    // B's own request carries B's withheld rows only.
    hook.rerender(b);
    act(() => hook.result.current.actions.onShowContactStatus());
    expect(hook.result.current.resultsRequest?.withheld).toEqual({ "state:support:S-D:load:L-1": supportLoadWithheldStateText("sensitive") });
  });

  it("drops the request when the result is cleared, as by an edit or the next solve's start", () => {
    const hook = mountSession(SOLVE_A());
    act(() => hook.result.current.actions.onShowContactStatus());
    hook.rerender({ result: null, standing: null });
    expect(hook.result.current.resultsRequest).toBeNull();
    hook.rerender(SOLVE_B());
    expect(hook.result.current.resultsRequest).toBeNull();
  });

  it("binds a native result's request to its handle in the same way", () => {
    const first = nativeSolve({ [COMBINED_ROW.id]: SUPPORT_LOAD_WITHHELD_NO_STATE });
    const second = nativeSolve({ "state:support:S-D:load:L-1": supportLoadWithheldStateText("sensitive") });
    const hook = mountSession({ result: first, standing: null });
    act(() => hook.result.current.actions.onShowContactStatus());
    expect(hook.result.current.resultsRequest).toMatchObject({ runId: "run:preview-linear-static-001", withheld: { [COMBINED_ROW.id]: SUPPORT_LOAD_WITHHELD_NO_STATE } });
    hook.rerender({ result: first, standing: null });
    expect(hook.result.current.resultsRequest).not.toBeNull();
    hook.rerender({ result: second, standing: null });
    expect(hook.result.current.resultsRequest).toBeNull();
  });
});
