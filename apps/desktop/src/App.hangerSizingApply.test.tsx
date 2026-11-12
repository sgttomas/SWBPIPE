// DEL-04-11 REQ-008 / AC-008 (review round 1 of pull request 258, finding 2): nothing is applied without the person's
// acceptance, shown through the connected desktop path. The App is rendered with its own session, review queue and
// undo; the hanger-sizing form is filled and sized, a batch is prepared and queued, validated in the review, and
// applied only by the review's "Apply entire batch"; one Undo restores the model.
//
// What stands in for the native host, after the shell has mounted (as the other native-path App tests do):
// - `list_local_libraries` and `open_local_library` serve one stored library, the invented four-span input's;
// - `size_hangers` answers with a product result, with its solve-request binding replaced by the checked hash of the
//   request this App sent: for the apply-all act (two or more selections), the four-span result
//   (`fixtures/results/hanger_sizing_result_four_span.json`: CE-2 on support:B by fallback, CE-1 on support:C, no fit
//   on support:D); for the per-candidate act (exactly one selection), the two-span reply's document
//   (`hanger_sizing_reply_two_span.json`: VS-200-L on support:B). The session's model is that input's model with the
//   session's `diagnostics: []`, which the headless input does not carry, so the product's own binding cannot match.
//   The reply's reader evidence is not bound to this document, so its standings show as not checked;
// - `validate_model_operation_batch` and `apply_model_operation_batch` are answered by the operation applier crate
//   compiled to wasm (the browser route's engine), with the arguments the App sent.
// The session's model is observed as the App hands it to the form: the model the sizing request carries, and every
// model the form binds the shown result against, which passes through `hangerSizingSolveRequest`, wrapped to record it.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../fixtures/results/hanger_sizing_result_four_span.json";
import twoSpanInput from "../../../fixtures/results/hanger_sizing_input_two_span.json";
import twoSpanReply from "../../../fixtures/results/hanger_sizing_reply_two_span.json";
import * as previewService from "./services/previewService";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "./services/hashService";
import { loadWasmEngine } from "./services/wasmEngine/loadWasmEngine";
import type { PreviewModel } from "./types";

const invokeMock = vi.hoisted(() => vi.fn());
const seen = vi.hoisted(() => ({ models: [] as unknown[], sized: [] as unknown[] }));
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
vi.mock("./features/hanger-sizing/hangerSizingRequest", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./features/hanger-sizing/hangerSizingRequest")>();
  return { ...actual, hangerSizingSolveRequest: (model: PreviewModel) => { seen.models.push(model); return actual.hangerSizingSolveRequest(model); } };
});

import { App } from "./App";
import { useModelledVisibility } from "./test-support/modelledVisibility";

useModelledVisibility();

/** Which product input and result the session and the native stand-in use. */
const fixture = { fourSpan: true };
const sessionModel = () => ({ ...structuredClone((fixture.fourSpan ? fourSpanInput : twoSpanInput).request.model), diagnostics: [] }) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
/** The library the native stand-in has stored now; a test may replace it after queueing. */
const storedLibrary: { current: () => Record<string, unknown> } = { current: library };
const LIBRARY_ID = "invented.hangers.sizing";
const initialInnerWidth = window.innerWidth;

beforeEach(() => {
  storedLibrary.current = library;
  seen.models = [];
  seen.sized = [];
});
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  Object.defineProperty(window, "innerWidth", { configurable: true, value: initialInnerWidth });
  window.localStorage.clear();
});

function host() {
  const engine = loadWasmEngine();
  invokeMock.mockImplementation(async (command: string, args?: Record<string, unknown>) => {
    if (command === "list_local_libraries") {
      return [{ project_id: sessionModel().project.id, library_kind: "hanger", library_id: LIBRARY_ID, library_name: "Invented sizing hangers", privacy_class: "private", storage_mode: "local", created_at_unix: 0, updated_at_unix: 0 }];
    }
    if (command === "open_local_library") {
      return { project_id: sessionModel().project.id, library_kind: "hanger", library_id: LIBRARY_ID, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0, document: storedLibrary.current(), validation: { accepted: true, has_blocking_findings: false, findings: [] }, message: "" };
    }
    if (command === "cancel_hanger_sizing") return { cancelled: false, epoch: 1, retirement_id: (args as { retirementId: unknown }).retirementId };
    if (command === "size_hangers") {
      const sent = JSON.parse(String(args!.inputText)) as { request: { model: unknown } };
      seen.sized.push(sent.request.model);
      const document = structuredClone(fixture.fourSpan ? fourSpanResult : twoSpanReply.document) as Record<string, unknown> & { inputs: { solve_request: { sha256: string } } };
      document.inputs.solve_request.sha256 = await canonicalSha256HexCheckedV1(sent.request);
      return { document, evidence: { document_sha256: "0".repeat(64), verdict: {} } };
    }
    if (command === "validate_model_operation_batch" || command === "apply_model_operation_batch") {
      const run = (await engine)[command === "apply_model_operation_batch" ? "applyOperationBatchJson" : "validateOperationBatchJson"];
      return JSON.parse(run(JSON.stringify(args!.model), JSON.stringify(args!.batch), JSON.stringify(args!.claimedModelHash)));
    }
    if (command === "sync_native_shell_state") return undefined;
    if (command === "dispose_all_preview_mechanics_results") return { released_results: 0, released_bytes: 0, abandoned_jobs: 0 };
    throw new Error(`unexpected ${command}`);
  });
}

/** The model the App last handed the form. */
const shownModel = () => seen.models[seen.models.length - 1] as PreviewModel;
const support = (model: PreviewModel, id: string) => model.supports.find((entry) => entry.id === id)!;
const select = (form: HTMLElement, testId: string, value: string) => fireEvent.change(within(form).getByTestId(testId), { target: { value } });

async function openSizing(): Promise<HTMLElement> {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(sessionModel());
  Object.defineProperty(window, "innerWidth", { configurable: true, value: 1440 });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  fireEvent.click(screen.getByTestId("toolkit-entry"));
  fireEvent.click(screen.getByTestId("toolkit-properties.hanger-library"));
  return screen.getByTestId("hanger-sizing-form");
}

/** Fills the form with the two-span fixture's own request, as a person would, and sizes it. */
async function sizeTwoSpan(form: HTMLElement) {
  fireEvent.click(within(form).getByRole("button", { name: "Refresh hanger libraries" }));
  await within(form).findByRole("option", { name: "Invented sizing hangers" });
  select(form, "hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "open_local_library")).toBe(true));
  fireEvent.click(within(within(form).getByTestId("hanger-sizing-candidates")).getByRole("option", { name: /support:B/ }));
  select(form, "hanger-sizing-policy-0", "variable");
  select(form, "hanger-sizing-weight-case", "load:W");
  select(form, "hanger-sizing-design-case", "load:O1");
  fireEvent.click(within(form).getByRole("checkbox", { name: /load:O2/ }));
  select(form, "hanger-sizing-load-design", "hot");
  select(form, "hanger-sizing-variation-reference", "operating_load");
  fireEvent.change(within(form).getByTestId("hanger-sizing-variation-limit"), { target: { value: "0.25" } });
  fireEvent.change(within(form).getByTestId("hanger-sizing-travel-allowance"), { target: { value: "0.002" } });
  select(form, "hanger-sizing-selection-rule", "smallest_capacity");
  select(form, "hanger-sizing-solver-mode", "dense_scrutiny");
  fireEvent.click(within(form).getByTestId("hanger-sizing-run"));
}

/** Fills the form with the four-span fixture's own request, as a person would, and sizes it. */
async function size(form: HTMLElement) {
  fireEvent.click(within(form).getByRole("button", { name: "Refresh hanger libraries" }));
  await within(form).findByRole("option", { name: "Invented sizing hangers" });
  select(form, "hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "open_local_library")).toBe(true));
  const picker = within(form).getByTestId("hanger-sizing-candidates");
  for (const id of ["support:B", "support:C", "support:D"]) fireEvent.click(within(picker).getByRole("option", { name: new RegExp(id) }));
  select(form, "hanger-sizing-policy-0", "variable_else_constant");
  select(form, "hanger-sizing-policy-1", "constant");
  select(form, "hanger-sizing-policy-2", "variable");
  select(form, "hanger-sizing-weight-case", "load:W");
  select(form, "hanger-sizing-design-case", "load:O1");
  fireEvent.click(within(form).getByTestId("hanger-sizing-no-check-cases"));
  select(form, "hanger-sizing-load-design", "cold");
  select(form, "hanger-sizing-variation-reference", "design_load");
  fireEvent.change(within(form).getByTestId("hanger-sizing-variation-limit"), { target: { value: "0.001" } });
  fireEvent.change(within(form).getByTestId("hanger-sizing-travel-allowance"), { target: { value: "0" } });
  select(form, "hanger-sizing-selection-rule", "least_variation");
  select(form, "hanger-sizing-solver-mode", "sparse_interactive");
  fireEvent.click(within(form).getByTestId("hanger-sizing-run"));
}

/** The queued batch's review article, validated by the person. */
async function validateQueued(): Promise<HTMLElement> {
  await waitFor(() => expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("1 queued batches; 0 batches applied this session."));
  const article = screen.getAllByRole("button", { name: "Validate batch" })[0].closest("article")!;
  fireEvent.click(within(article).getByRole("button", { name: "Validate batch" }));
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "validate_model_operation_batch")).toBe(true));
  await waitFor(() => expect(within(article).getByRole("button", { name: "Apply entire batch" })).not.toBeDisabled());
  return article;
}

const ACTS = [
  { act: "the apply-all act", fourSpan: true, prepare: "Prepare applying every selection of this run", queue: "Queue every sized selection of this run for review", changed: ["support:B", "support:C"] },
  { act: "the per-candidate act", fourSpan: false, prepare: "Prepare applying record VS-200-L to support:B", queue: "Queue sized selection for review", changed: ["support:B"] }
] as const;

describe("a sized selection through the connected desktop path", () => {
  for (const { act: name, fourSpan, prepare, queue, changed } of ACTS) {
    it(`by ${name}: unchanged after sizing, preparing, queueing and validation; changed only by the person's application; one Undo restores it`, async () => {
      fixture.fourSpan = fourSpan;
      host();
      const form = await openSizing();
      const commands = () => invokeMock.mock.calls.map(([command]) => command);

      await (fourSpan ? size(form) : sizeTwoSpan(form));
      const button = await within(form).findByRole("button", { name: prepare });
      // The model sized is the session's model as loaded, and sizing left it so.
      expect(seen.sized).toHaveLength(1);
      const original = await canonicalJsonString(seen.sized[0]);
      expect(original).toBe(await canonicalJsonString(sessionModel()));
      expect(await canonicalJsonString(shownModel())).toBe(original);

      fireEvent.click(button);
      const review = await within(form).findByTestId(name === "the apply-all act" ? "hanger-sizing-apply-run-review" : "hanger-sizing-apply-review");
      const afters = Object.fromEntries(within(review).getAllByTestId(/^hanger-sizing-apply(-run)?-after$/).map((pre, index) => [changed[index], JSON.parse(pre.textContent!)]));
      expect(Object.keys(afters)).toEqual([...changed]);
      expect(await canonicalJsonString(shownModel())).toBe(original);

      fireEvent.click(within(review).getByRole("button", { name: queue }));
      const article = await validateQueued();
      // Queued and validated: no application was asked of the host, and the model is as it was.
      expect(commands()).not.toContain("apply_model_operation_batch");
      expect(await canonicalJsonString(shownModel())).toBe(original);
      expect(screen.getByTestId("undo-session-model-edit")).toBeDisabled();

      fireEvent.click(within(article).getByRole("button", { name: "Apply entire batch" }));
      await waitFor(() => expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 1 batches applied this session."));
      expect(commands().filter((command) => command === "apply_model_operation_batch")).toHaveLength(1);
      await waitFor(() => expect(canonicalJsonString(shownModel())).resolves.not.toBe(original));
      const applied = shownModel();
      for (const id of changed) {
        const written = support(applied, id);
        expect({ family: written.family, restraints: written.restraints, provenance: written.provenance, hanger: written.hanger }, id).toEqual(afters[id]);
      }
      const before = JSON.parse(original) as PreviewModel;
      for (const id of before.supports.map((entry) => entry.id).filter((other) => !(changed as readonly string[]).includes(other))) {
        expect(await canonicalJsonString(support(applied, id)), id).toBe(await canonicalJsonString(support(before, id)));
      }
      // The run no longer binds the applied model, so nothing more of it is offered.
      expect(await within(form).findByTestId("hanger-sizing-apply-model-changed")).toBeInTheDocument();

      fireEvent.click(screen.getByTestId("operation-tab-review"));
      fireEvent.click(screen.getByTestId("undo-session-model-edit"));
      await waitFor(() => expect(canonicalJsonString(shownModel())).resolves.toBe(original));
      expect(screen.getByTestId("undo-session-model-edit")).toBeDisabled();
      // Restored: the run binds the model again (the form's page is hidden behind the review tab).
      expect(await within(form).findByRole("button", { name: prepare, hidden: true })).toBeInTheDocument();
      expect(within(form).queryByTestId("hanger-sizing-apply-model-changed")).toBeNull();
    });
  }

  it("by the apply-all act: a library replaced after queueing makes the final Apply apply nothing, shown on the entry; restored, Apply proceeds", async () => {
    fixture.fourSpan = true;
    host();
    const form = await openSizing();
    const commands = () => invokeMock.mock.calls.map(([command]) => command);
    await size(form);
    fireEvent.click(await within(form).findByRole("button", { name: "Prepare applying every selection of this run" }));
    const review = await within(form).findByTestId("hanger-sizing-apply-run-review");
    fireEvent.click(within(review).getByRole("button", { name: "Queue every sized selection of this run for review" }));
    const article = await validateQueued();
    const original = await canonicalJsonString(seen.sized[0]);

    // The person replaces the stored library under the same id while the batch waits in the review.
    storedLibrary.current = () => {
      const changed = library();
      (changed.hanger_library as Record<string, unknown>).name = "Invented sizing hangers, revised";
      return changed;
    };
    fireEvent.click(within(article).getByRole("button", { name: "Apply entire batch" }));
    expect(await within(article).findByText(/^Last apply refused: The hanger library has changed since this sizing run/)).toBeInTheDocument();
    expect(commands()).not.toContain("apply_model_operation_batch");
    expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("1 queued batches; 0 batches applied this session.");
    expect(screen.getByTestId("batch-review-message")).toHaveTextContent(/was not applied\. The hanger library has changed since this sizing run/);
    expect(await canonicalJsonString(shownModel())).toBe(original);
    expect(screen.getByTestId("undo-session-model-edit")).toBeDisabled();

    storedLibrary.current = library;
    fireEvent.click(within(article).getByRole("button", { name: "Apply entire batch" }));
    await waitFor(() => expect(screen.getByTestId("batch-review-summary")).toHaveTextContent("0 queued batches; 1 batches applied this session."));
    expect(commands().filter((command) => command === "apply_model_operation_batch")).toHaveLength(1);
    await waitFor(() => expect(canonicalJsonString(shownModel())).resolves.not.toBe(original));
    expect(support(shownModel(), "support:B").family).toBe("constant_effort_support");
  });
});
