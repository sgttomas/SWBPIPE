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
