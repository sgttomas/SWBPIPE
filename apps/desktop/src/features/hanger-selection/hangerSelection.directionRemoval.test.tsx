// DEL-04-03 REQ-014, DEL-04-11 REQ-014: a hanger takes no direction, and a stored direction is an entered value that is
// never dropped silently. Applying a hanger selection to an existing support that holds one writes the explicit removal
// (update_support, field path `direction`, before the stored direction, after `not_present`) as its own change ahead of
// the configuration, and the preview the person reads before applying names it with the stored value. A selection
// applied to a support without a direction, and one that creates a support, are unchanged. The model and library are the
// invented two-span sizing input with support:B given a direction here; each batch is applied by the wasm operation
// engine, the route the person's acceptance runs.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import * as libraryImportService from "../../services/libraryImportService";
import { applyOperationBatch, type OperationBatch } from "../../services/operationBatchService";
import * as previewService from "../../services/previewService";
import { chooseVirtualTarget } from "../../test-support/workspaceTestControls";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { HangerSizingApply } from "../hanger-sizing/HangerSizingApply";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import {
  buildHangerSelectionBatch,
  directionRemovals,
  directionRemovalText,
  DIRECTION_REMOVAL_FIELD_LABEL,
  type HangerDocument,
  type HangerSizingApplication
} from "./hangerSelection";
import { HangerSelectionPanel } from "./HangerSelectionPanel";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const SKEWED = { x: 0.1, y: -0.2, z: 5e-300 };
const AXIS_ALIGNED = { x: 0, y: 0, z: 1 };
const LIBRARY = "invented.hangers.sizing";
const TARGET = { supportId: "support:B", restraints: ["UZ"] };
/** An invented sized application: the library's records carry no loads of their own. */
const SIZED: HangerSizingApplication = {
  run: { sizingRequestSha256: "1".repeat(64), solveRequestSha256: "2".repeat(64), libraryId: LIBRARY, librarySha256: "3".repeat(64), hangerId: "VS-200-L", recordSha256: "4".repeat(64) },
  loads: { family: "variable_spring", coldLoadN: 5197.5, hotLoadN: 5000, designOperatingCase: "load:O1" }
};
const REMOVAL_TEXT = (stored: string) =>
  `Applying this selection removes the stored direction ${stored} of support support:B. ` +
  "A hanger takes no direction: its load acts along the positive axis of its stiffness degree of freedom (or of its single translational restraint).";

/** The two-span model; with a direction, support:B is a line stop on UZ holding it (the record rules admit that). */
function model(direction?: { x: number; y: number; z: number }): PreviewModel {
  const source = structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
  if (direction) Object.assign(source.supports.find((support) => support.id === "support:B")!, { family: "line_stop", direction: { ...direction } });
  return source;
}
const library = () => structuredClone(twoSpanInput.library) as unknown as HangerDocument;

/** Deterministic ids, so that two batches can be compared byte for byte. */
function counted() {
  let next = 0;
  vi.spyOn(crypto, "randomUUID").mockImplementation(() => `00000000-0000-4000-8000-${String(next++).padStart(12, "0")}` as `${string}-${string}-${string}-${string}-${string}`);
}

async function applied(source: PreviewModel, batch: OperationBatch) {
  const result = await applyOperationBatch(source, batch, await computeModelHash(source));
  expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
  return result.applied_model!;
}

describe("a hanger selection applied to a support that holds a stored direction", () => {
  it("writes the removal as its own change ahead of the configuration, and the engine applies the batch as one", async () => {
    for (const direction of [SKEWED, AXIS_ALIGNED]) {
      const source = model(direction);
      const original = await canonicalJsonString(source);
      const stored = await canonicalJsonString(direction);
      const batch = await buildHangerSelectionBatch(source, library(), "VS-200-L", TARGET, SIZED);
      expect(batch.operations.map((operation) => operation.change.field_path)).toEqual(["direction", "configuration", "provenance_record"]);
      const [removal, configuration] = batch.operations;
      expect(removal).toMatchObject({
        operation_kind: "modify", target: { object_type: "Support", ref: "support:B" },
        change: { change_kind: "update_support", field_label: DIRECTION_REMOVAL_FIELD_LABEL, field_path: "direction", before: stored,
          after: "not_present", unit: "none", dimension: "dimensionless" }
      });
      expect(removal.audit_boundary.requires_user_acceptance).toBe(true);
      // The configuration envelope never carried the direction.
      expect(configuration.change.before).not.toContain("direction");
      expect(configuration.change.after).not.toContain("direction");
      expect(directionRemovals(batch)).toEqual([{ supportId: "support:B", stored, text: REMOVAL_TEXT(stored) }]);
      const support = (await applied(source, batch)).supports.find((entry) => entry.id === "support:B")!;
      expect(support).not.toHaveProperty("direction");
      expect(support.hanger?.hanger_type).toBe("variable_spring_hanger");
      expect(await canonicalJsonString(source)).toBe(original);
    }
  });

  it("keeps the skewed components' bits in the removal's before text", async () => {
    const batch = await buildHangerSelectionBatch(model(SKEWED), library(), "VS-200-L", TARGET);
    expect(batch.operations[0].change.before).toBe('{"x":0.1,"y":-0.2,"z":5e-300}');
  });

  it("is refused by the document rule without the removal, which names the direction", async () => {
    const source = model(SKEWED);
    const batch = await buildHangerSelectionBatch(source, library(), "VS-200-L", TARGET, SIZED);
    const withoutRemoval = { ...batch, operations: batch.operations.slice(1) };
    const result = await applyOperationBatch(source, withoutRemoval, await computeModelHash(source));
    expect(result.validation.application_status).toBe("blocked");
    expect(result.applied_model ?? null).toBeNull();
    expect(result.diagnostics.map((diagnostic) => diagnostic.code), JSON.stringify(result.diagnostics)).toContain("SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
  });

  it("is one undo step: one undo restores the stored direction byte for byte", async () => {
    // The session's own model carries its diagnostics list.
    const source = { ...model(SKEWED), diagnostics: [] } as unknown as PreviewModel;
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(source);
    const { result } = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(result.current.model.model).not.toBeNull());
    const original = JSON.stringify(result.current.model.model);
    const originalHash = await computeModelHash(result.current.model.model!);
    const batch = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.operations.batchMessage).toBe("Batch applied once. Previous solve results were cleared; one undo checkpoint is available.");
    expect(result.current.model.model!.supports.find((support) => support.id === "support:B")).not.toHaveProperty("direction");
    expect(result.current.operations.undoStack).toHaveLength(1);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(JSON.stringify(result.current.model.model)).toBe(original);
    expect(await computeModelHash(result.current.model.model!)).toEqual(originalHash);
    expect(result.current.model.model!.supports.find((support) => support.id === "support:B")!.direction).toEqual(SKEWED);
  });
});

describe("a hanger selection without a stored direction is unchanged", () => {
  it("on an existing support: the same batch, byte for byte, as the directed record's without its removal", async () => {
    counted();
    const plain = await buildHangerSelectionBatch(model(), library(), "VS-200-L", TARGET, SIZED);
    expect(plain.operations.map((operation) => operation.change.field_path)).toEqual(["configuration", "provenance_record"]);
    expect(directionRemovals(plain)).toEqual([]);
    // The same support as a line stop without a direction: the configuration's before differs only by its family.
    vi.restoreAllMocks();
    counted();
    const lineStop = model(SKEWED);
    delete (lineStop.supports.find((support) => support.id === "support:B") as { direction?: unknown }).direction;
    const undirected = await buildHangerSelectionBatch(lineStop, library(), "VS-200-L", TARGET, SIZED);
    vi.restoreAllMocks();
    counted();
    // The directed batch spends its first two ids on the removal (its change and its operation).
    const directed = await buildHangerSelectionBatch(model(SKEWED), library(), "VS-200-L", TARGET, SIZED);
    const renumbered = JSON.stringify({ ...directed, operations: directed.operations.slice(1) })
      .replace(/00000000-0000-4000-8000-(\d{12})/g, (_, n: string) => `00000000-0000-4000-8000-${String(Number(n) - 2).padStart(12, "0")}`);
    expect(renumbered).toBe(JSON.stringify(undirected));
    await applied(model(), plain);
  });

  it("on a support it creates: no removal, and the batch does not depend on another support's direction", async () => {
    const create = { supportId: "support:NEW", restraints: ["UZ"], newSupport: { label: "New hanger", node: "node:B" } };
    counted();
    const plain = await buildHangerSelectionBatch(model(), library(), "VS-200-L", create, SIZED);
    vi.restoreAllMocks();
    counted();
    const besideDirected = await buildHangerSelectionBatch(model(SKEWED), library(), "VS-200-L", create, SIZED);
    expect(JSON.stringify(besideDirected)).toBe(JSON.stringify(plain));
    expect(plain.operations.map((operation) => operation.change.change_kind)).toEqual(["create_support"]);
    expect(plain.operations[0].change.after).not.toContain("direction");
    expect(directionRemovals(plain)).toEqual([]);
  });
});

describe("the previews a person reads before applying", () => {
  it("the imported selection names the removal with the stored value, and shows the configuration's before and after", async () => {
    const source = model(SKEWED);
    const stored = await canonicalJsonString(SKEWED);
    const doc = library();
    const envelope = { document: doc, validation: { accepted: true, has_blocking_findings: false, findings: [] } } as never;
    vi.spyOn(libraryImportService, "listLocalLibraries").mockResolvedValue({ route: "tauri_backend", entries: [{ library_id: LIBRARY, library_name: "Invented", library_kind: "hanger" }] } as never);
    vi.spyOn(libraryImportService, "openLocalLibrary").mockResolvedValue({ route: "tauri_backend", envelope });
    vi.spyOn(libraryImportService, "validateLibraryImport").mockResolvedValue({ route: "tauri_backend", validation: { accepted: true, has_blocking_findings: false, findings: [] } } as never);
    const queue = vi.fn();
    render(<HangerSelectionPanel model={source} selection={{ type: "project", id: source.project.id }} onQueueBatch={queue} />);
    fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
    await waitFor(() => expect(libraryImportService.listLocalLibraries).toHaveBeenCalled());
    // The library opens asynchronously, so its choice is shown only once it is open.
    const picker = screen.getByTestId("hanger-library-picker");
    fireEvent.change(within(picker).getByRole("combobox"), { target: { value: LIBRARY } });
    fireEvent.click(within(picker).getByRole("option", { name: /Invented/ }));
    await waitFor(() => expect(picker.querySelector(".virtual-target-picker-current")).toHaveTextContent("Invented"));
    chooseVirtualTarget("hanger-record-picker", "VS-200-L");
    chooseVirtualTarget("hanger-support-picker", "support:B");
    fireEvent.click(screen.getByLabelText("UZ"));
    fireEvent.click(screen.getByRole("button", { name: "Prepare hanger selection" }));
    const notice = await screen.findByTestId("hanger-selection-direction-removal");
    expect(notice).toHaveTextContent(REMOVAL_TEXT(stored));
    const shown = within(screen.getByText("Before and after").closest("details")!).getAllByText(/./, { selector: "pre" }).map((pre) => pre.textContent);
    const batch = await buildHangerSelectionBatch(source, doc, "VS-200-L", TARGET);
    expect(shown).toEqual([batch.operations[1].change.before, batch.operations[1].change.after]);
    fireEvent.click(screen.getByRole("button", { name: "Queue hanger selection for review" }));
    expect(queue.mock.calls[0][0].operations.map((operation: { change: { field_path: string } }) => operation.change.field_path))
      .toEqual(["direction", "configuration", "provenance_record"]);
  });

  it("the sized selection names the removal with the stored value, and shows the configuration's before and after", async () => {
    const source = model(SKEWED);
    const stored = await canonicalJsonString(SKEWED);
    const lib = structuredClone(twoSpanInput.library) as Record<string, unknown>;
    const record = (lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === "VS-200-L");
    const candidate = {
      support: "support:B", node: "node:B", dof: "UZ", family_policy: "variable_else_constant", w_n: 5000, u_cold_m: 0, operating: [], span_m: [0, 0.001],
      required_travel_m: 0.005, failure_counts: { variable: null, constant: null }, outcome: "selected",
      selected: { family: "variable_spring", hanger_id: "VS-200-L", record_sha256: await canonicalSha256HexCheckedV1(record), k_n_per_m: 200000, f_cold_n: 5197.5,
        f_cold_condition_n: 5197.5, envelope_n: [4600, 5716], operating: [{ case: "load:O1", f_n: 5000, variation: 0.04 }] },
      governing: { travel_case: "load:O1", variation_case: "load:O1" },
      verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: 5197.5 }, verification: null
    } as unknown as HangerSizingCandidate;
    const document = {
      status: "sized",
      inputs: { canonical_profile: "openpipestress_jcs_ijson_v1", sizing_request: { design_operating_case: "load:O1" }, sizing_request_sha256: "5".repeat(64),
        solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) }, library: { library_id: LIBRARY, sha256: await canonicalSha256HexCheckedV1(lib) } },
      result: { candidates: [candidate] }
    } as unknown as HangerSizingSizedDocument;
    const queue = vi.fn();
    render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={queue} />);
    fireEvent.click(await screen.findByRole("button", { name: "Prepare applying record VS-200-L to support:B" }));
    const review = await screen.findByTestId("hanger-sizing-apply-review");
    expect(within(review).getByTestId("hanger-sizing-apply-direction-removal")).toHaveTextContent(REMOVAL_TEXT(stored));
    expect(JSON.parse(within(review).getByTestId("hanger-sizing-apply-before").textContent!)).toEqual({ family: "line_stop", restraints: ["UZ"], provenance: "invented_test_value" });
    expect(JSON.parse(within(review).getByTestId("hanger-sizing-apply-after").textContent!).family).toBe("variable_spring_hanger");
    expect(within(review).getByTestId("hanger-sizing-apply-source-note").textContent).toContain("Hanger sizing run");
    expect(directionRemovalText("support:B", stored)).toBe(REMOVAL_TEXT(stored));
  });
});
