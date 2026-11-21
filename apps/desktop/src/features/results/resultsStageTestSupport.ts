/** Test support for the Results stage: reach what a person reaches, by the controls a person uses. */
import { fireEvent, within } from "@testing-library/react";
import { expect } from "vitest";

/** Opens "Method and limits" by its summary, as a person does, and asserts that it is open, so a
 * query inside it cannot pass on text a person cannot reach. Returns the disclosure. */
export function openMethodAndLimits(scope: HTMLElement = document.body): HTMLDetailsElement {
  const all = within(scope).getAllByTestId("results-method") as HTMLDetailsElement[];
  const details = all[all.length - 1];
  if (!details.open) fireEvent.click(details.querySelector("summary")!);
  expect(details.open).toBe(true);
  return details;
}

/** The result's standing sentence as the stage shows it: the header's standing line, then the rest
 * of the sentence inside "Method and limits" (opened), joined by a single space. */
export function shownStandingSentence(scope: HTMLElement = document.body): string {
  const lines = within(scope).getAllByTestId("numerical-result-standing");
  const line = lines[lines.length - 1].textContent ?? "";
  const method = openMethodAndLimits(scope);
  const detail = within(method).queryByTestId("numerical-result-standing-detail")?.textContent ?? "";
  return detail ? `${line} ${detail}` : line;
}

/** Brings a tab of the stage forward by its tab control and asserts that it is current. */
export function openStageTab(name: string, scope: HTMLElement = document.body): HTMLElement {
  const tab = within(scope).getByRole("tab", { name });
  fireEvent.click(tab);
  expect(tab).toHaveAttribute("aria-selected", "true");
  return document.getElementById(tab.getAttribute("aria-controls")!)!;
}
