// A2 render cost (DEL-07-05 Design `support-load-summary.md` §6): the panel derives its
// summaries once per (model, result, standing) identity. Re-renders that change none of
// them (the session's focus, the test-id prefix, stepping between supports) reuse the
// derivation. Invented data only (`resultOverlayFixtures`).
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const derivations = vi.hoisted(() => ({ count: 0 }));
vi.mock("./supportLoadSummary", async (original) => {
  const actual = await original<typeof import("./supportLoadSummary")>();
  return {
    ...actual,
    buildSupportLoadSummaries: (input: Parameters<typeof actual.buildSupportLoadSummaries>[0]) => {
      derivations.count += 1;
      return actual.buildSupportLoadSummaries(input);
    }
  };
});
import { healthStanding, overlayModel, overlayResult, reaction } from "../../test-support/resultOverlayFixtures";
import { SupportLoadSummaryPanel } from "./SupportLoadSummaryPanel";

afterEach(cleanup);

it("derives once per (model, result, standing) and not on re-renders that change none of them", () => {
  derivations.count = 0;
  const model = overlayModel();
  const result = overlayResult([reaction("support:S-A", "Fx", 1, "N"), reaction("support:S-C", "Fx", 2, "N")]);
  const standing = healthStanding({ eligible: true });
  const view = render(<SupportLoadSummaryPanel model={model} result={result} standing={standing} />);
  expect(derivations.count).toBe(1);
  view.rerender(<SupportLoadSummaryPanel model={model} result={result} standing={standing} focusSupportId="support:S-C" />);
  expect(screen.getByTestId("support-load-position").textContent).toBe("Support 2 of 3");
  view.rerender(<SupportLoadSummaryPanel model={model} result={result} standing={standing} focusSupportId="support:S-C" testIdPrefix="support-load" />);
  fireEvent.click(screen.getByTestId("support-load-previous"));
  expect(screen.getByTestId("support-load-position").textContent).toBe("Support 1 of 3");
  expect(derivations.count).toBe(1);
  // A new identity of any input derives again, once.
  const nextResult = overlayResult([reaction("support:S-A", "Fx", 3, "N")]);
  view.rerender(<SupportLoadSummaryPanel model={model} result={nextResult} standing={standing} focusSupportId="support:S-C" />);
  expect(derivations.count).toBe(2);
  const nextStanding = healthStanding({ eligible: true });
  view.rerender(<SupportLoadSummaryPanel model={model} result={nextResult} standing={nextStanding} focusSupportId="support:S-C" />);
  expect(derivations.count).toBe(3);
  view.rerender(<SupportLoadSummaryPanel model={overlayModel()} result={nextResult} standing={nextStanding} focusSupportId="support:S-C" />);
  expect(derivations.count).toBe(4);
});
