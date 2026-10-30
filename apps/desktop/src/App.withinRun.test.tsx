// W-12 S3c composition (Codex r2 on #1619): a within-run choice made in Results and
// dropped while the user is elsewhere is named beside the package actions in Report
// and Exports, not only in the inactive Results section. The application solves the
// range-envelope model, the user chooses L-200 in Results and moves to Report; a model
// document that offers no L-200 is opened and solved; the Report and Exports notices
// name the dropped bases and the default pair the packages now carry. Solves are unit
// transport replays of committed producer bytes (NOT a native UI qualification
// witness). Invented data only.
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { App } from "./App";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge } from "./services/previewService";
import { createSolveJobTransport, DISPOSE_ALL_COMMAND, nativeMechanicsReplayPair, SOLVE_JOB_COMMANDS } from "./test/nativeMechanicsReplay";
import { droppedNotice } from "./features/comparison/withinRunComparison";
import type { PreviewModel } from "./types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../");
const RANGE_MODEL = JSON.parse(readFileSync(resolve(root, "core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json"), "utf8")) as PreviewModel;
const RANGE_BYTES = readFileSync(resolve(root, "fixtures/results/preview_physics_range_envelope_sparse.json"), "utf8");
const CONNECTED = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
const RANGE = "combination:U4B-RANGE-MAX";

afterEach(() => {
  invokeMock.mockReset();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}

function openSection(id: string) {
  let section = screen.queryByTestId(`workspace-section-${id}`);
  if (!section || section.classList.contains("inactive")) {
    if (screen.queryByTestId("menu-view")) {
      fireEvent.click(screen.getByTestId("menu-view"));
      fireEvent.click(screen.getByTestId(`menu-item-view.section.${id}`));
    } else {
      nativeMenuCommand(`view.section.${id}`);
    }
    section = screen.getByTestId(`workspace-section-${id}`);
  }
  expect(section).not.toHaveClass("inactive");
  return section;
}

async function solve() {
  const solveSection = openSection("solve");
  await act(async () => { fireEvent.click(within(solveSection).getByTestId("run-mechanics-preview")); });
}

it("names a dropped choice beside the package actions in Report and Exports", async () => {
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((sent) => {
    if (isDeepStrictEqual(sent.model, RANGE_MODEL)) return RANGE_BYTES;
    if (isDeepStrictEqual(sent.model, CONNECTED.model)) return CONNECTED.bytes;
    throw new Error("REPLAY_REQUEST_MISMATCH");
  });
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(RANGE_MODEL);
    if (command === "open_model_document_file") return { outcome: "opened", file_name: "invented-connected.json", byte_count: 1, schema_status: "current", document: structuredClone(CONNECTED.model) };
    if (SOLVE_JOB_COMMANDS.includes(command) || command === DISPOSE_ALL_COMMAND) return transport.invoke(command, args, options);
    throw new Error(`NATIVE_APP_REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  render(<App />);
  await screen.findByTestId("workspace-toolbar");

  await solve();
  const results = openSection("results");
  const left = await within(results).findByTestId("comparison-left-select") as HTMLSelectElement;
  fireEvent.change(left, { target: { value: "load:L-200" } });
  await waitFor(() => expect(within(results).getByTestId("comparison-package-basis-text")).toHaveTextContent(`load:L-200 vs ${RANGE}`));

  // Report: the packages carry the choice.
  const report = openSection("report");
  expect(within(report).getByTestId("report-comparison-package-notice-text")).toHaveTextContent(`load:L-200 vs ${RANGE}`);

  // A model document that offers no L-200 replaces the model; its solve drops the choice.
  nativeMenuCommand("file.open-model");
  await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("invented-connected.json"));
  const units = screen.queryByTestId("project-units-create");
  if (units) fireEvent.click(units);
  await solve();
  const dropped = droppedNotice(["load:L-200", RANGE]);
  const reportAgain = openSection("report");
  await waitFor(() => expect(within(reportAgain).getByTestId("report-comparison-package-notice-dropped")).toHaveTextContent(dropped));
  expect(within(reportAgain).getByTestId("report-comparison-package-notice-text")).toHaveTextContent("Packages include: load:L-100 vs combination:C-OPER-ALT.");
  const exportsSection = openSection("exports");
  expect(within(exportsSection).getByTestId("handoff-comparison-package-notice-dropped")).toHaveTextContent(dropped);
});
