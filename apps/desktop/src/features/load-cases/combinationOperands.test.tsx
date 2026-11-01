// Model document 0.9.0 combination operands on the desktop (§2.4; DEL-07-11).
// The every-slot model carries a result-state subtraction with no `terms`
// (C-EXP), an absolute sum whose terms name a load case and a combination
// (C-ABS) and an SRSS of a combination (C-SRSS). It opens through File > Open
// unchanged, every view that reads a combination renders it with its actual
// operands (never an empty sum for an absent `terms`), and its save text
// reopens to the same document. The native write and read of the same
// canonical text are `model_document_save.rs`
// `save_then_open_gives_identical_bytes_and_an_equal_model_hash`.
import { act, render, screen, waitFor, within, fireEvent } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { App } from "../../App";
import { checkNativeOutcome } from "../../services/modelDocumentFile";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { prepareModelDocumentSave } from "../model-file/modelDocumentSave";
import { modelDocumentShapeDiagnostics } from "../model-file/modelDocumentShape";
import { selectedProperties } from "../model-workspace/modelView";
import { combinationTermsText } from "./combinationOperands";

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;
const model = () => clone(everySlotModel) as unknown as PreviewModel;
const combination = (id: string) => model().combinations!.find((item) => item.id === id)!;

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

describe("the 0.9.0 combination operands", () => {
  it("the fixture carries the forms under test", () => {
    expect(Object.hasOwn(combination("combination:C-EXP"), "terms")).toBe(false);
    expect(combination("combination:C-ABS").terms).toEqual([{ load_case: "load:L-1", factor: 1 }, { combination: "combination:C-EXP", factor: 1 }]);
    expect(modelDocumentShapeDiagnostics(model())).toEqual([]);
  });

  it("shows each operand as stored: a load case by its id, a combination named as one, an absent terms as absent", () => {
    expect(combinationTermsText(combination("combination:C-EXP"))).toBeNull();
    expect(combinationTermsText(combination("combination:C-OP"))).toBe("");
    expect(combinationTermsText(combination("combination:C-ABS"))).toBe("load:L-1 x 1; combination combination:C-EXP x 1");
    expect(combinationTermsText(combination("combination:C-SRSS"))).toBe("combination combination:C-ABS x 1");
    const rows = (id: string) => Object.fromEntries(selectedProperties(model(), { type: "combination", id }));
    expect(rows("combination:C-EXP")).toEqual({
      ID: "combination:C-EXP", Basis: "result_state_subtraction", Minuend: "load:L-2", Subtrahend: "load:L-1",
      "Difference basis": "operating_minus_sustained", "Subtrahend support state": "minuend_final", Provenance: "invented_example"
    });
    expect(rows("combination:C-ABS").Terms).toBe("load:L-1 x 1; combination combination:C-EXP x 1");
    expect(rows("combination:C-OP")).toEqual({ ID: "combination:C-OP", Basis: "mechanics", Terms: "", Provenance: "invented_example" });
  });

  it("opens through File > Open and renders the load manager and the inspector without throwing", async () => {
    invokeMock.mockImplementation((command: string) => {
      if (command === "open_model_document_file") {
        return Promise.resolve({ outcome: "opened", file_name: "slots.json", byte_count: 1, schema_status: "current", document: model() });
      }
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await screen.findByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    act(() => nativeMenuCommand("file.open-model"));
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened model document slots.json"));

    act(() => nativeMenuCommand("view.section.loads"));
    const manager = within(await screen.findByTestId("workspace-section-loads")).getByTestId("load-case-manager");
    const row = (id: string) => within(manager).getByTestId(`load-manager-combination-${id}`).textContent ?? "";
    expect(row("combination:C-EXP")).toContain("basis=result_state_subtraction; minuend=load:L-2; subtrahend=load:L-1");
    expect(row("combination:C-EXP")).not.toContain("terms=");
    expect(row("combination:C-ABS")).toContain("basis=absolute_sum; terms=load:L-1 x 1; combination combination:C-EXP x 1");
    expect(row("combination:C-SRSS")).toContain("basis=srss; terms=combination combination:C-ABS x 1");
    // The term rows are the stored terms only: none for the subtraction.
    expect(within(manager).getByTestId("load-manager-combination-term-combination:C-ABS-1")).toHaveTextContent("combination combination:C-EXP x 1");
    expect(within(manager).queryByTestId("load-manager-combination-term-combination:C-EXP-0")).toBeNull();

    const tree = screen.getByLabelText("Model tree");
    for (const id of ["combination:C-EXP", "combination:C-ABS"]) {
      fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), { target: { value: id } });
      fireEvent.click(await within(tree).findByTestId(`tree-row-combination-${encodeURIComponent(id)}`));
      const inspector = screen.getByTestId("property-inspector");
      await waitFor(() => expect(inspector).toHaveTextContent(id));
      if (id === "combination:C-EXP") {
        expect(inspector).toHaveTextContent("Minuendload:L-2");
        expect(inspector).not.toHaveTextContent("Terms");
      } else {
        expect(inspector).toHaveTextContent("Termsload:L-1 x 1; combination combination:C-EXP x 1");
      }
    }
  });

  it("saves the canonical text of the opened document and reopens it with the subtraction still without terms", async () => {
    const opened = checkNativeOutcome({ outcome: "opened", file_name: "slots.json", byte_count: 1, schema_status: "current", document: model() });
    expect(opened.outcome).toBe("opened");
    if (opened.outcome !== "opened") return;
    const preparation = await prepareModelDocumentSave({
      model: opened.document,
      modelHash: (await computeModelHash(opened.document))!,
      explicitLocalPrivateIntent: true
    });
    expect(preparation.status).toBe("ready");
    if (preparation.status !== "ready") return;
    const { canonicalText } = preparation.args;
    expect(canonicalText).toBe(await canonicalJsonString(everySlotModel));
    const reopened = checkNativeOutcome({ outcome: "opened", file_name: "slots.json", byte_count: canonicalText.length, schema_status: "current", document: JSON.parse(canonicalText) });
    expect(reopened.outcome === "opened" && reopened.document).toEqual(everySlotModel);
    const subtraction = reopened.outcome === "opened" ? reopened.document.combinations!.find((item) => item.id === "combination:C-EXP")! : null;
    expect(subtraction && Object.hasOwn(subtraction, "terms")).toBe(false);
  });

  it("refuses malformed operands by name at open", () => {
    const document = clone(everySlotModel) as unknown as Record<string, any>;
    document.combinations[2].terms[1] = { load_case: "load:L-1", combination: "combination:C-EXP", factor: 1 };
    document.combinations[3].terms[0] = { factor: 1 };
    document.combinations[1].minuend_id = 7;
    document.combinations[0].terms = undefined;
    const checked = checkNativeOutcome({ outcome: "opened", file_name: "bad.json", byte_count: 1, schema_status: "current", document: JSON.parse(JSON.stringify(document)) });
    expect(checked.outcome === "refused" && checked.diagnostics.map(({ code, path }) => [code, path])).toEqual([
      ["MODEL-FILE-SHAPE", "combinations[0].terms"],
      ["MODEL-FILE-SHAPE", "combinations[1].minuend_id"],
      ["MODEL-FILE-SHAPE", "combinations[2].terms[1]"],
      ["MODEL-FILE-SHAPE", "combinations[3].terms[0]"]
    ]);
  });
});
