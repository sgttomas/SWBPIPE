// The indicator's three looks: checked with nothing found (a hollow ring, ordinary text), a check
// that found trouble (the red badge, as built), not checked (no mark, muted words). Never a tick,
// a shield or green; the words are the indicator's own.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { healthLookOf } from "./healthLook";
import { NumericalHealthBadgeText } from "./ResultsPanel";
import { NUMERICAL_HEALTH_STATE_TEXT, type NumericalHealthState } from "./numericalHealthText";
import { badgeOf } from "./numericalHealth";

afterEach(cleanup);

describe("the numerical-health looks", () => {
  it.each([
    ["checks_passed", "checked", true],
    ["sensitive", "trouble", false],
    ["quality_unresolved", "trouble", false],
    ["solve_failed", "trouble", false],
    ["not_assessed", "unchecked", false]
  ] as const)("%s looks %s", (state, look, ring) => {
    expect(healthLookOf(state as NumericalHealthState)).toBe(look);
    render(<NumericalHealthBadgeText badge={badgeOf(state)} state={state} text={NUMERICAL_HEALTH_STATE_TEXT[state]} testId="badge" />);
    const badge = screen.getByTestId("badge");
    expect(badge.getAttribute("data-look")).toBe(look);
    expect(badge.classList.contains(`numerical-health-look-${look}`)).toBe(true);
    // Only a check that found nothing has a mark of its own, a hollow ring; the trouble badge keeps its colour.
    expect(badge.querySelectorAll("svg.numerical-health-ring").length).toBe(ring ? 1 : 0);
    expect(badge.classList.contains("numerical-health-red")).toBe(look === "trouble");
    expect(badge.textContent).toBe(NUMERICAL_HEALTH_STATE_TEXT[state]);
  });
});
