import { fireEvent, screen, within } from "@testing-library/react";
import { expect } from "vitest";

function exactSuffix(value: string): RegExp {
  return new RegExp(`${value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`);
}

export function chooseVirtualTarget(testId: string, value: string): HTMLElement {
  const picker = screen.getByTestId(testId);
  fireEvent.change(within(picker).getByRole("combobox"), { target: { value } });
  const option = within(picker).getByRole("option", { name: exactSuffix(value) });
  fireEvent.click(option);
  expect(picker.querySelector(".virtual-target-picker-current")).toHaveTextContent(value);
  return picker;
}

export function chooseVirtualMultiTarget(testId: string, value: string): HTMLElement {
  const picker = screen.getByTestId(testId);
  fireEvent.change(within(picker).getByRole("combobox"), { target: { value } });
  const option = within(picker).getByRole("option", { name: exactSuffix(value) });
  fireEvent.click(option);
  expect(option).toHaveAttribute("aria-selected", "true");
  return picker;
}

export function removeVirtualMultiTarget(testId: string, value: string): HTMLElement {
  const picker = screen.getByTestId(testId);
  fireEvent.change(within(picker).getByRole("combobox"), { target: { value } });
  const option = within(picker).getByRole("option", { name: exactSuffix(value) });
  fireEvent.click(option);
  expect(option).toHaveAttribute("aria-selected", "false");
  return picker;
}

export function startInspectorTask(): HTMLElement {
  const inspector = screen.getByTestId("property-inspector");
  fireEvent.click(within(inspector).getByRole("tab", { name: /^Task$/ }));
  fireEvent.click(within(inspector).getByTestId("inspector-start-task"));
  return within(inspector).getByTestId("editor-intent-panel");
}

/** Activate the actual compact control and choose its rendered option by canonical value. */
export function chooseCompactOption(control: HTMLElement, value: string): void {
  expect(control).toHaveAttribute("role", "combobox");
  if (control.getAttribute("aria-expanded") !== "true") fireEvent.click(control);
  const listId = control.getAttribute("aria-controls");
  const list = listId ? document.getElementById(listId) : null;
  if (!list) throw new Error("Compact selector did not open its listbox");
  const option = Array.from(list.querySelectorAll<HTMLElement>('[role="option"]'))
    .find((candidate) => candidate.dataset.value === value);
  if (!option) throw new Error(`Compact selector has no option for ${value}`);
  expect(option).not.toHaveAttribute("aria-disabled", "true");
  fireEvent.click(option);
  expect(control).toHaveAttribute("aria-expanded", "false");
}

/** Existing field-entry adapters keep native inputs; migrated selectors use real option clicks. */
export function changeFormControl(...args: Parameters<typeof fireEvent.change>): void {
  const [control, init] = args;
  if (control instanceof HTMLElement && control.classList.contains("compact-select-trigger")) {
    chooseCompactOption(control, String((init as { target?: { value?: unknown } } | undefined)?.target?.value ?? ""));
  } else fireEvent.change(...args);
}

export function expectFormControlValue(control: HTMLElement, value: string | number | string[] | null | undefined): void {
  if (control.classList.contains("compact-select-trigger")) expect(control).toHaveAttribute("data-value", value);
  else expect(control).toHaveValue(value);
}

export function compactOptionRecords(control: HTMLElement): Array<{ value: string; label: string; disabled: boolean }> {
  if (control.getAttribute("aria-expanded") !== "true") fireEvent.click(control);
  const list = document.getElementById(control.getAttribute("aria-controls") ?? "");
  if (!list) throw new Error("Compact selector did not open its listbox");
  const records = Array.from(list.querySelectorAll<HTMLElement>('[role="option"]')).map(option => ({
    value: option.dataset.value ?? "", label: option.textContent ?? "", disabled: option.getAttribute("aria-disabled") === "true"
  }));
  fireEvent.keyDown(control, { key: "Escape" });
  return records;
}
