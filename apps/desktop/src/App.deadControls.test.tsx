import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "./App";

// TP-APP-R2-UXSHELL-001 dead-control audit (permanent regression test).
//
// The human packaged-app pass (SMOKE.md TP-MAC-141 attempt 2) reported
// unresponsive controls: "Many things are unresponsive (just a button, but
// no action)." This audit renders the full shell, enumerates every button
// (the shell's only clickable control primitive besides plain links), and
// requires each one to be either:
//   (a) enabled and wired so that clicking produces an observable DOM
//       change within the audit window, or
//   (b) disabled with an accessible reason (non-empty `title`,
//       `aria-label`, or `aria-describedby` target text).
//
// Method: controls are normalized into classes (entity ids and sequence
// numbers stripped from data-testid), and one representative per class is
// audited on a FRESH render so clicks cannot contaminate each other. Three
// shell states are covered, because some controls only exist in later
// authoring phases:
//   1. initial fixture state,
//   2. queued state (one explicit-node intent queued),
//   3. solved state (mechanics preview completed).
//
// Exemption (documented): a button with aria-pressed="true" at click time is
// an already-active toggle/filter; re-selecting it is idempotent by design
// and produces no DOM change, so active toggles are not counted as dead.

type ControlClass = {
  key: string;
  testId: string | null;
  label: string;
};

type Violation = {
  scenario: string;
  key: string;
  problem: string;
};

const AUDIT_TIMEOUT_MS = 600_000;
const CLICK_OBSERVATION_TIMEOUT_MS = 3_000;

function normalizeKey(element: HTMLButtonElement): ControlClass {
  const testId = element.getAttribute("data-testid");
  const label = (element.getAttribute("aria-label") ?? element.textContent ?? "").trim();
  if (testId) {
    const key = testId.split(":")[0].replace(/\d+/g, "#");
    return { key: `testid:${key}`, testId, label };
  }
  return { key: `label:${label}`, testId: null, label };
}

function enumerateButtons(container: HTMLElement): HTMLButtonElement[] {
  return Array.from(container.querySelectorAll<HTMLButtonElement>("button, [role='button']"));
}

function isDisabled(element: HTMLButtonElement): boolean {
  return element.disabled || element.getAttribute("aria-disabled") === "true";
}

function disabledReason(element: HTMLButtonElement): string | null {
  const title = element.getAttribute("title");
  if (title && title.trim().length > 0) return title.trim();
  const ariaLabel = element.getAttribute("aria-label");
  if (ariaLabel && ariaLabel.trim().length > 0) return ariaLabel.trim();
  const describedBy = element.getAttribute("aria-describedby");
  if (describedBy) {
    const text = describedBy
      .split(/\s+/)
      .map((id) => document.getElementById(id)?.textContent?.trim() ?? "")
      .join(" ")
      .trim();
    if (text.length > 0) return text;
  }
  return null;
}

async function renderReadyShell(): Promise<HTMLElement> {
  const { container } = render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  // Engine warmup is async; wait for a terminal engine state so click
  // observations are not confused with warmup re-renders.
  await waitFor(
    () => {
      const status = screen.getByTestId("operation-engine-chip").textContent ?? "";
      if (!/Engine (ready|unavailable)/.test(status)) {
        throw new Error(`engine not settled: ${status}`);
      }
    },
    { timeout: 15_000 }
  );
  return container;
}

async function prepareScenario(scenario: string, container: HTMLElement): Promise<void> {
  if (scenario === "initial") return;
  if (scenario === "queued") {
    // Queue one structured edit through the load-case manager (the same
    // visible path the unit suites use) so the apply/validate row controls
    // exist for the audit.
    fireEvent.click(screen.getByTestId("load-manager-primitive-load:L-100-P"));
    fireEvent.change(screen.getByTestId("load-manager-magnitude-value"), { target: { value: "1500000" } });
    fireEvent.click(screen.getByTestId("queue-load-magnitude-intent"));
    await screen.findByTestId("apply-intent-editor-intent-1");
    return;
  }
  if (scenario === "solved") {
    fireEvent.click(screen.getByTestId("run-mechanics-preview"));
    await waitFor(
      () => {
        const summary = screen.getByTestId("solve-job-summary").textContent ?? "";
        if (!summary.includes("state=completed")) throw new Error(`solve not completed: ${summary}`);
      },
      { timeout: 30_000 }
    );
    return;
  }
  throw new Error(`unknown audit scenario: ${scenario}`);
}

function findRepresentative(container: HTMLElement, control: ControlClass): HTMLButtonElement | null {
  const candidates = enumerateButtons(container);
  return candidates.find((candidate) => normalizeKey(candidate).key === control.key) ?? null;
}

async function auditScenario(
  scenario: string,
  alreadyAudited: Set<string>,
  violations: Violation[]
): Promise<void> {
  // Enumeration render: collect the control classes present in this state.
  let container = await renderReadyShell();
  await prepareScenario(scenario, container);
  const classes = new Map<string, ControlClass>();
  for (const element of enumerateButtons(container)) {
    const control = normalizeKey(element);
    if (!alreadyAudited.has(control.key) && !classes.has(control.key)) {
      classes.set(control.key, control);
    }
  }
  cleanup();

  for (const control of classes.values()) {
    alreadyAudited.add(control.key);
    container = await renderReadyShell();
    await prepareScenario(scenario, container);
    const element = findRepresentative(container, control);
    if (!element) {
      violations.push({
        scenario,
        key: control.key,
        problem: "control class disappeared between enumeration and audit render (non-deterministic shell state)"
      });
      cleanup();
      continue;
    }

    if (isDisabled(element)) {
      const reason = disabledReason(element);
      if (!reason) {
        violations.push({
          scenario,
          key: control.key,
          problem: `disabled control has no accessible reason (no title/aria-label/aria-describedby); text="${control.label}"`
        });
      }
      cleanup();
      continue;
    }

    if (element.getAttribute("aria-pressed") === "true") {
      // Documented exemption: already-active toggle/filter.
      cleanup();
      continue;
    }

    const before = container.innerHTML;
    fireEvent.click(element);
    try {
      await waitFor(
        () => {
          if (container.innerHTML === before) throw new Error("no DOM change observed");
        },
        { timeout: CLICK_OBSERVATION_TIMEOUT_MS }
      );
    } catch {
      violations.push({
        scenario,
        key: control.key,
        problem: `enabled control produced no observable DOM change after click; text="${control.label}"`
      });
    }
    cleanup();
  }
}

describe("dead-control audit (TP-APP-R2-UXSHELL-001)", () => {
  afterEach(() => cleanup());

  it(
    "every button is either responsive or disabled with a stated reason",
    async () => {
      const violations: Violation[] = [];
      const audited = new Set<string>();
      for (const scenario of ["initial", "queued", "solved"]) {
        await auditScenario(scenario, audited, violations);
      }
      const report = violations.map((entry) => `[${entry.scenario}] ${entry.key}: ${entry.problem}`);
      expect(report).toEqual([]);
      // Sanity floor: the audit must actually have seen the shell's control
      // surface; a near-empty enumeration means the audit itself broke.
      expect(audited.size).toBeGreaterThan(40);
    },
    AUDIT_TIMEOUT_MS
  );
});
