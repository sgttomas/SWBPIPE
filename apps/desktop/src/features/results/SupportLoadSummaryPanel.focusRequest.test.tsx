import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { keyModel, keyResult } from "../../test-support/keyResultsFixtures";
import { SupportLoadSummaryPanel } from "./SupportLoadSummaryPanel";

// A6's repeated "Show support load summary" (DEL-07-05 Design `key-results-panel.md` §2): each new
// focus request shows the requested support again, even when the session's selection is unchanged
// and the user stepped the panel to another support meanwhile. Invented data only.

describe("SupportLoadSummaryPanel focus request", () => {
  it("shows the requested support again after the user stepped away, and not before a new request", () => {
    const model = keyModel();
    const result = keyResult();
    const standing = healthStanding({ eligible: true });
    const view = (request: number) => (
      <SupportLoadSummaryPanel model={model} result={result} standing={standing} focusSupportId="support:S-A" focusRequest={request} />
    );
    const { rerender } = render(view(1));
    const select = () => screen.getByTestId("support-load-select");
    expect(select()).toHaveValue("support:S-A");
    fireEvent.click(screen.getByTestId("support-load-next"));
    expect(select()).toHaveValue("support:S-C");
    // A re-render with the same request keeps the user's step.
    rerender(view(1));
    expect(select()).toHaveValue("support:S-C");
    // S for the same support again: a new request, so the panel shows it.
    rerender(view(2));
    expect(select()).toHaveValue("support:S-A");
  });
});
