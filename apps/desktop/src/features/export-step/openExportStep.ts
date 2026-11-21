import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { expect } from "vitest";
import { FIXED_ENTRIES, type ExportFormat, type PackageRecord } from "./exportForm";

/** Test support: an export's record, opened as a person opens it from its row: by the row's button where the record
 * opens first, or by its Record control where the row writes on the press. It returns the open record, after
 * asserting that it is open, so that a check reads inside it only then. */
export async function openExportStep(prefix: string, timeout?: number): Promise<HTMLElement> {
  const options = timeout === undefined ? undefined : { timeout };
  const control = await waitFor(() => {
    const found = screen.queryByTestId(`${prefix}-open`) ?? screen.queryByTestId(`${prefix}-record-open`);
    expect(found).not.toBeNull();
    expect(found).toBeEnabled();
    return found!;
  }, options);
  fireEvent.click(control);
  const step = screen.getByTestId(`${prefix}-step`);
  expect(step).toHaveAttribute("role", "dialog");
  return step;
}

/** Test support: the element that holds an export's save controls, wherever its row's form puts them: the row
 * itself where it writes on the press, otherwise its record, opened from the row's button. */
export async function exportSaves(prefix: string, panelTestId: string, timeout?: number): Promise<HTMLElement> {
  const options = timeout === undefined ? undefined : { timeout };
  const control = await waitFor(() => {
    const found = screen.queryByTestId(`${prefix}-open`) ?? screen.queryByTestId(`${prefix}-record-open`);
    expect(found).not.toBeNull();
    return found!;
  }, options);
  if (control.dataset.testid === `${prefix}-record-open`) return screen.getByTestId(panelTestId);
  await waitFor(() => expect(control).toBeEnabled(), options);
  fireEvent.click(control);
  return screen.getByTestId(`${prefix}-step`);
}

/** Test support: leaves an export's open record by its Close button, and asserts that it is gone. */
export function closeExportStep(prefix: string): void {
  fireEvent.click(screen.getByTestId(`${prefix}-step-close`));
  expect(screen.queryByTestId(`${prefix}-step`)).toBeNull();
}

/** Test support: the form the row is in, read from its controls. */
export function exportRowForm(prefix: string, panelTestId: string): "write-on-press" | "record-first" | "none" {
  const row = screen.getByTestId(panelTestId);
  if (within(row).queryByTestId(`${prefix}-record-open`)) return "write-on-press";
  return within(row).queryByTestId(`${prefix}-open`) ? "record-first" : "none";
}

/** Test support: the entry terms two packages' records hold in common (the result's content aside), sorted. */
export function commonTerms(first: PackageRecord, second: PackageRecord): string[] {
  const terms = (record: PackageRecord) => new Set(record.entries.filter((entry) => entry.kind !== "content").map((entry) => entry.term));
  const other = terms(second);
  return [...terms(first)].filter((term) => other.has(term)).sort();
}

/** Test support: a format's fixed entry terms (`FIXED_ENTRIES`), sorted. */
export function fixedTerms(format: ExportFormat): string[] {
  return FIXED_ENTRIES[format].map((entry) => entry.term).sort();
}
