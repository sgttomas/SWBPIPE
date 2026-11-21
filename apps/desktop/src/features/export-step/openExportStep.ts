import { fireEvent, screen, waitFor } from "@testing-library/react";
import { expect } from "vitest";

/** Test support: an export's step, opened as a person opens it, from its row's button once that is enabled. It
 * returns the open step, after asserting that it is open, so that a check reads inside it only then. */
export async function openExportStep(prefix: string, timeout?: number): Promise<HTMLElement> {
  const options = timeout === undefined ? undefined : { timeout };
  const button = await screen.findByTestId(`${prefix}-open`, undefined, options);
  await waitFor(() => expect(button).toBeEnabled(), options);
  fireEvent.click(button);
  const step = screen.getByTestId(`${prefix}-step`);
  expect(step).toHaveAttribute("role", "dialog");
  return step;
}

/** Test support: leaves an export's open step by its Close button, and asserts that it is gone. */
export function closeExportStep(prefix: string): void {
  fireEvent.click(screen.getByTestId(`${prefix}-step-close`));
  expect(screen.queryByTestId(`${prefix}-step`)).toBeNull();
}
