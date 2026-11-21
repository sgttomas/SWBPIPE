// R9: the Issues total counts the blocking missing-data entries; the drawer's Missing data panel
// shows that same number in its heading and lists those entries, and keeps the entries that do
// not block under one closed disclosure that says how many and that they are not counted.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import { buildMissingDataBlockingPacket, countMissingDataBlockers, MissingDataBlockingPanel } from "./MissingDataBlockingPanel";

it("heads the panel with the count the Issues total takes, lists the blocking entries, and keeps the rest apart, closed", async () => {
  const model = await loadPreviewModel();
  const packet = buildMissingDataBlockingPacket({ model, result: null });
  const blocking = packet.warnings.filter((warning) => warning.blocks_mechanics_solve || warning.blocks_rule_check);
  const others = packet.warnings.filter((warning) => !warning.blocks_mechanics_solve && !warning.blocks_rule_check);
  // The fixture has both kinds, so the test reads both.
  expect(blocking.length).toBeGreaterThan(0);
  expect(others.length).toBeGreaterThan(0);
  expect(countMissingDataBlockers({ model, result: null })).toBe(blocking.length);

  render(<MissingDataBlockingPanel model={model} result={null} />);
  expect(screen.getByTestId("missing-data-blocking-count")).toHaveTextContent(String(blocking.length));
  const list = screen.getByTestId("missing-data-warnings");
  const apart = screen.getByTestId("missing-data-not-blocking") as HTMLDetailsElement;
  // The blocking entries are listed outside the disclosure.
  const listed = [...list.querySelectorAll('[data-testid^="missing-data-warning-"]')].filter((entry) => !apart.contains(entry));
  expect(listed).toHaveLength(blocking.length);
  expect(apart.open).toBe(false);
  expect(screen.getByTestId("missing-data-not-blocking-toggle")).toHaveTextContent(
    `${others.length} more ${others.length === 1 ? "entry does" : "entries do"} not block; not counted in Issues`
  );
  fireEvent.click(screen.getByTestId("missing-data-not-blocking-toggle"));
  expect(apart.open).toBe(true);
  expect(within(apart).getAllByTestId(/^missing-data-warning-/)).toHaveLength(others.length);
});
