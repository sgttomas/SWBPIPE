// DEL-04-11 REQ-008 (review round 1 of pull request 258, finding 1): in the form, a shown result's selections are
// applied only while the hanger library stored now under the id it names is the one it binds. The host is a mocked
// native host (`invoke`) with one stored hanger library, the invented two-span input's; the worker's reply is the
// product's `hanger_sizing_reply_two_span.json` (one selection, so the per-candidate act). The apply-all act needs a run
// of two or more selections: the four-span input and the product's four-span result (two selections), whose binding to
// its input's request holds as committed (the form is given that model as it is); that result has no committed reply,
// so its reader evidence is not bound and its standings show as not checked. The library is the same in both inputs.
// Revising the library means the host's stored document changes under the same id (another name), and the person
// reopens it in the form.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import twoSpanReply from "../../../../../fixtures/results/hanger_sizing_reply_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import type { PreviewModel } from "../../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

type Form = typeof import("./HangerSizingForm");
let HangerSizingForm: Form["HangerSizingForm"];
let texts: typeof import("./sizedSelection");

beforeEach(async () => {
  vi.resetModules();
  ({ HangerSizingForm } = await import("./HangerSizingForm"));
  texts = await import("./sizedSelection");
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const fourSpanModel = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const fourSpanReply = () => ({ document: structuredClone(fourSpanResult), evidence: { document_sha256: "0".repeat(64), verdict: {} } });
const library = () => structuredClone(twoSpanInput.library) as Record<string, unknown>;
const LIBRARY_ID = (twoSpanInput.library.hanger_library as { library_id: string }).library_id;
const revised = () => {
  const changed = library();
  (changed.hanger_library as Record<string, unknown>).name = "Invented sizing hangers, revised";
  return changed;
};
const ACTS = [
  { act: "the per-candidate act", prepare: "Prepare applying record VS-200-L to support:B", review: "hanger-sizing-apply-review", fourSpan: false },
  { act: "the apply-all act", prepare: "Prepare applying every selection of this run", review: "hanger-sizing-apply-run-review", fourSpan: true }
] as const;

/** A native host whose stored hanger library is `stored()` at each open. */
function host(stored: { current: () => Record<string, unknown> | null }, reply: () => unknown = () => structuredClone(twoSpanReply)) {
  invokeMock.mockImplementation((command: string, args?: unknown) => {
    if (command === "list_local_libraries") {
      return Promise.resolve([{ project_id: model().project.id, library_kind: "hanger", library_id: LIBRARY_ID, library_name: "Invented sizing hangers", privacy_class: "private", storage_mode: "local", created_at_unix: 0, updated_at_unix: 0 }]);
    }
    if (command === "open_local_library") {
      const document = stored.current();
      return Promise.resolve(document === null ? null : { project_id: model().project.id, library_kind: "hanger", library_id: LIBRARY_ID, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0, document, validation: { accepted: true, has_blocking_findings: false, findings: [] }, message: "" });
    }
    if (command === "size_hangers") return Promise.resolve(reply());
    if (command === "cancel_hanger_sizing") return Promise.resolve({ cancelled: false, epoch: 1, retirement_id: (args as { retirementId: unknown }).retirementId });
    return Promise.reject(new Error(`unexpected ${command}`));
  });
}

const select = (testId: string, value: string) => fireEvent.change(screen.getByTestId(testId), { target: { value } });
const opens = () => invokeMock.mock.calls.filter(([command]) => command === "open_local_library").length;

/** Fills the form with the two-span fixture's own request and sizes it. */
async function sizeTwoSpan() {
  fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
  await screen.findByRole("option", { name: "Invented sizing hangers" });
  select("hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(opens()).toBe(1));
  fireEvent.click(within(screen.getByTestId("hanger-sizing-candidates")).getByRole("option", { name: /support:B/ }));
  select("hanger-sizing-policy-0", "variable");
  select("hanger-sizing-weight-case", "load:W");
  select("hanger-sizing-design-case", "load:O1");
  fireEvent.click(screen.getByRole("checkbox", { name: /load:O2/ }));
  select("hanger-sizing-load-design", "hot");
  select("hanger-sizing-variation-reference", "operating_load");
  fireEvent.change(screen.getByTestId("hanger-sizing-variation-limit"), { target: { value: "0.25" } });
  fireEvent.change(screen.getByTestId("hanger-sizing-travel-allowance"), { target: { value: "0.002" } });
  select("hanger-sizing-selection-rule", "smallest_capacity");
  select("hanger-sizing-solver-mode", "dense_scrutiny");
  fireEvent.click(screen.getByTestId("hanger-sizing-run"));
}

/** Fills the form with the four-span fixture's own request and sizes it. */
async function sizeFourSpan() {
  fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
  await screen.findByRole("option", { name: "Invented sizing hangers" });
  select("hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(opens()).toBe(1));
  const picker = screen.getByTestId("hanger-sizing-candidates");
  for (const id of ["support:B", "support:C", "support:D"]) fireEvent.click(within(picker).getByRole("option", { name: new RegExp(id) }));
  select("hanger-sizing-policy-0", "variable_else_constant");
  select("hanger-sizing-policy-1", "constant");
  select("hanger-sizing-policy-2", "variable");
  select("hanger-sizing-weight-case", "load:W");
  select("hanger-sizing-design-case", "load:O1");
  fireEvent.click(screen.getByTestId("hanger-sizing-no-check-cases"));
  select("hanger-sizing-load-design", "cold");
  select("hanger-sizing-variation-reference", "design_load");
  fireEvent.change(screen.getByTestId("hanger-sizing-variation-limit"), { target: { value: "0.001" } });
  fireEvent.change(screen.getByTestId("hanger-sizing-travel-allowance"), { target: { value: "0" } });
  select("hanger-sizing-selection-rule", "least_variation");
  select("hanger-sizing-solver-mode", "sparse_interactive");
  fireEvent.click(screen.getByTestId("hanger-sizing-run"));
}

/** Renders the form for `fourSpan` or the two-span input and sizes it. */
async function renderSized(fourSpan: boolean, queue: () => void) {
  render(<HangerSizingForm model={fourSpan ? fourSpanModel() : model()} onQueueBatch={queue} />);
  await (fourSpan ? sizeFourSpan() : sizeTwoSpan());
}

/** The person reopens the library under the same id. */
async function reopen() {
  const before = opens();
  select("hanger-sizing-library", "");
  select("hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(opens()).toBe(before + 1));
}

describe("a revised hanger library withdraws a sized selection's application", () => {
  it("offers nothing once the person reopens a revised library under the same id", async () => {
    const stored = { current: library as () => Record<string, unknown> | null };
    host(stored);
    render(<HangerSizingForm model={model()} onQueueBatch={vi.fn()} />);
    await sizeTwoSpan();
    await screen.findByRole("button", { name: ACTS[0].prepare });
    stored.current = revised;
    await reopen();
    expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toHaveTextContent(texts.HANGER_SIZING_LIBRARY_CHANGED_TEXT);
    for (const { prepare } of ACTS) expect(screen.queryByRole("button", { name: prepare })).toBeNull();
  });

  for (const { act, prepare, review, fourSpan } of ACTS) {
    it(`withdraws ${act} when the stored library was revised or removed after the run, though the form still holds the old one`, async () => {
      for (const [now, text] of [[revised, "HANGER_SIZING_LIBRARY_CHANGED_TEXT"], [() => null, "HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT"]] as const) {
        const stored = { current: library as () => Record<string, unknown> | null };
        host(stored, fourSpan ? fourSpanReply : undefined);
        const queue = vi.fn();
        await renderSized(fourSpan, queue);
        const button = await screen.findByRole("button", { name: prepare });
        stored.current = now;
        fireEvent.click(button);
        expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toHaveTextContent(texts[text]);
        expect(screen.queryByTestId(review)).toBeNull();
        expect(screen.queryByRole("button", { name: /^Prepare applying|^Queue/ })).toBeNull();
        expect(queue).not.toHaveBeenCalled();
        cleanup();
        invokeMock.mockReset();
      }
    });

    it(`drops a batch prepared by ${act} when the person reopens a revised library under the same id, so it cannot be queued`, async () => {
      const stored = { current: library as () => Record<string, unknown> | null };
      host(stored, fourSpan ? fourSpanReply : undefined);
      const queue = vi.fn();
      await renderSized(fourSpan, queue);
      fireEvent.click(await screen.findByRole("button", { name: prepare }));
      await screen.findByTestId(review);
      stored.current = revised;
      await reopen();
      expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toBeInTheDocument();
      expect(screen.queryByTestId(review)).toBeNull();
      expect(screen.queryByRole("button", { name: /^Queue/ })).toBeNull();
      expect(queue).not.toHaveBeenCalled();
    });
  }
});
