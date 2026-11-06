// DEL-04-11 increment 3c part 2 (AC-012): the hanger-sizing request form and its mounted table. A new project's form
// has no pre-set choice; an incomplete request names what is missing and sends nothing; a complete one is the exact
// headless input (`fixtures/results/hanger_sizing_input_two_span.json`); and the capped worker's reply to that input
// (`hanger_sizing_reply_two_span.json`, pinned by the Rust command test) is shown by 3b's panel with its standings
// bound and the library's record details. A stale reply never replaces the shown result.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import twoSpanReply from "../../../../../fixtures/results/hanger_sizing_reply_two_span.json";
import type { PreviewModel } from "../../types";
import { assembleHangerSizingInput, EMPTY_HANGER_SIZING_FORM } from "./hangerSizingRequest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

type Form = typeof import("./HangerSizingForm");
let HangerSizingForm: Form["HangerSizingForm"];

beforeEach(async () => {
  vi.resetModules();
  ({ HangerSizingForm } = await import("./HangerSizingForm"));
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as Record<string, unknown>;
const LIBRARY_ID = (twoSpanInput.library.hanger_library as { library_id: string }).library_id;

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((yes) => { resolve = yes; });
  return { promise, resolve };
}

/** A native host with one stored hanger library; `size` answers `size_hangers`. */
function host(size: () => Promise<unknown> = () => Promise.resolve(structuredClone(twoSpanReply))) {
  invokeMock.mockImplementation((command: string) => {
    if (command === "list_local_libraries") {
      return Promise.resolve([{ project_id: model().project.id, library_kind: "hanger", library_id: LIBRARY_ID, library_name: "Invented sizing hangers", privacy_class: "private", storage_mode: "local", created_at_unix: 0, updated_at_unix: 0 }]);
    }
    if (command === "open_local_library") {
      return Promise.resolve({ project_id: model().project.id, library_kind: "hanger", library_id: LIBRARY_ID, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0, document: library(), validation: { accepted: true, has_blocking_findings: false, findings: [] }, message: "" });
    }
    if (command === "size_hangers") return size();
    if (command === "cancel_hanger_sizing") return Promise.resolve({ cancelled: true, duplicate: false });
    return Promise.reject(new Error(`unexpected ${command}`));
  });
}

const select = (testId: string, value: string) => fireEvent.change(screen.getByTestId(testId), { target: { value } });

/** Fills the form with the two-span fixture's own request, as a person would. */
async function fillTwoSpan() {
  fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
  await screen.findByRole("option", { name: "Invented sizing hangers" });
  select("hanger-sizing-library", LIBRARY_ID);
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "open_local_library")).toBe(true));
  const picker = screen.getByTestId("hanger-sizing-candidates");
  fireEvent.click(within(picker).getByRole("option", { name: /support:B/ }));
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
}

describe("the request form", () => {
  it("starts with no pre-set choice in a new project and sends nothing on mount", () => {
    host();
    render(<HangerSizingForm model={model()} />);
    for (const testId of ["hanger-sizing-library", "hanger-sizing-weight-case", "hanger-sizing-design-case", "hanger-sizing-load-design",
      "hanger-sizing-variation-reference", "hanger-sizing-selection-rule", "hanger-sizing-solver-mode"]) {
      expect((screen.getByTestId(testId) as HTMLSelectElement).value, testId).toBe("");
    }
    expect((screen.getByTestId("hanger-sizing-variation-limit") as HTMLInputElement).value).toBe("");
    expect((screen.getByTestId("hanger-sizing-travel-allowance") as HTMLInputElement).value).toBe("");
    expect(screen.getAllByRole("checkbox").every((box) => !(box as HTMLInputElement).checked)).toBe(true);
    expect(within(screen.getByTestId("hanger-sizing-candidates")).getByText("0 selected")).toBeInTheDocument();
    expect(screen.getByTestId("hanger-sizing-weight-support-state")).toHaveTextContent("own_solve");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses an incomplete request by naming every missing input, and sends nothing", () => {
    host();
    render(<HangerSizingForm model={model()} />);
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    const missing = within(screen.getByTestId("hanger-sizing-missing")).getAllByRole("listitem").map((item) => item.textContent?.split(":")[0]);
    expect(missing).toEqual(["library", "candidates", "weight case", "design operating case", "check operating cases", "load design",
      "variation reference", "variation limit", "travel allowance", "selection rule", "solver mode"]);
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("assembles nothing for an invalid number or a candidate without its policy", () => {
    const base = { ...EMPTY_HANGER_SIZING_FORM, libraryId: "l", candidates: [{ support: "support:B", policy: "" as const }], weightCase: "load:W",
      designOperatingCase: "load:O1", noCheckOperatingCases: true, loadDesign: "hot" as const, variationReference: "design_load" as const,
      variationLimit: "0", travelAllowanceM: "-1", selectionRule: "least_variation" as const, solverMode: "sparse_interactive" as const };
    const assembled = assembleHangerSizingInput(base, model(), library());
    expect(assembled.ok).toBe(false);
    if (assembled.ok) return;
    expect(assembled.missing).toEqual([
      "family policy of support:B: choose variable, constant or variable else constant",
      "variation limit: enter a positive finite ratio",
      "travel allowance: enter a finite length of 0 m or more"
    ]);
    for (const text of ["", " ", "1e", "Infinity", "0x1", "1,5"]) {
      const result = assembleHangerSizingInput({ ...base, candidates: [{ support: "support:B", policy: "constant" }], variationLimit: text, travelAllowanceM: "0" }, model(), library());
      expect(result.ok, JSON.stringify(text)).toBe(false);
    }
    const stated = assembleHangerSizingInput({ ...base, candidates: [{ support: "support:B", policy: "constant" }], variationLimit: "0.1", travelAllowanceM: "0" }, model(), library());
    expect(stated.ok && stated.input.sizing.check_operating_cases).toEqual([]);
    expect(stated.ok && stated.input.sizing.method.travel_allowance_m).toBe(0);
  });

  it("assembles exactly the headless input from the person's entries, and shows the worker's reply in the panel with its standings bound", async () => {
    host();
    render(<HangerSizingForm model={model()} />);
    await fillTwoSpan();
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    const panel = await screen.findByTestId("hanger-sizing-panel");
    const sent = invokeMock.mock.calls.find(([command]) => command === "size_hangers")![1] as { inputText: string };
    expect(JSON.parse(sent.inputText)).toEqual(twoSpanInput);
    await within(panel).findByTestId("hanger-sizing-sized");
    // The library's record details pass 3a's hash gate; the standings are bound to the document's hash.
    expect(within(panel).getByTestId("hanger-sizing-record-support:B").textContent).toBe("invented.hangers.sizing; variable_spring; VS-200-L; Invented VS-200-L; variable_spring_hanger");
    expect(within(panel).getByTestId("hanger-sizing-f-cold-support:B").textContent).toBe("5197.458880641297 N");
    expect(within(panel).getByTestId("hanger-sizing-standings-binding").textContent).toBe(
      `Reader evidence is bound to this document by its checked hash (sha256 ${twoSpanReply.evidence.document_sha256}).`
    );
    expect(screen.queryByTestId("hanger-sizing-failure")).toBeNull();
  });

  it("sends the library the person selected last, whatever order the library opens complete in", async () => {
    const OTHER_ID = "invented.hangers.other";
    const other = () => {
      const document = library() as { hanger_library: { library_id: string; name: string } };
      document.hanger_library = { ...document.hanger_library, library_id: OTHER_ID, name: "Invented other hangers" };
      return document as unknown as Record<string, unknown>;
    };
    const opens = new Map<string, ReturnType<typeof deferred<unknown>>>([[LIBRARY_ID, deferred()], [OTHER_ID, deferred()]]);
    const envelope = (libraryId: string, document: Record<string, unknown> | null) => ({
      project_id: model().project.id, library_kind: "hanger", library_id: libraryId, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0,
      document, validation: { accepted: true, has_blocking_findings: false, findings: [] }, message: ""
    });
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "list_local_libraries") {
        return Promise.resolve([LIBRARY_ID, OTHER_ID].map((library_id) => ({ project_id: model().project.id, library_kind: "hanger", library_id, library_name: library_id, privacy_class: "private", storage_mode: "local", created_at_unix: 0, updated_at_unix: 0 })));
      }
      if (command === "open_local_library") {
        const id = JSON.stringify(args ?? {}).includes(OTHER_ID) ? OTHER_ID : LIBRARY_ID;
        return opens.get(id)!.promise;
      }
      if (command === "size_hangers") return Promise.resolve(structuredClone(twoSpanReply));
      if (command === "cancel_hanger_sizing") return Promise.resolve({ cancelled: true, duplicate: false });
      return Promise.reject(new Error(`unexpected ${command}`));
    });
    render(<HangerSizingForm model={model()} />);
    fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
    await screen.findByRole("option", { name: OTHER_ID });
    // Choose the first library, then the other; the other's open completes first, the first's late.
    select("hanger-sizing-library", LIBRARY_ID);
    select("hanger-sizing-library", OTHER_ID);
    await waitFor(() => expect(invokeMock.mock.calls.filter(([command]) => command === "open_local_library")).toHaveLength(2));
    await act(async () => { opens.get(OTHER_ID)!.resolve(envelope(OTHER_ID, other())); });
    await act(async () => { opens.get(LIBRARY_ID)!.resolve(envelope(LIBRARY_ID, library())); });
    expect((screen.getByTestId("hanger-sizing-library") as HTMLSelectElement).value).toBe(OTHER_ID);
    const picker = screen.getByTestId("hanger-sizing-candidates");
    fireEvent.click(within(picker).getByRole("option", { name: /support:B/ }));
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
    await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "size_hangers")).toBe(true));
    const sent = invokeMock.mock.calls.find(([command]) => command === "size_hangers")![1] as { inputText: string };
    expect(JSON.parse(sent.inputText).library.hanger_library.library_id).toBe(OTHER_ID);
  });

  it("drops an obsolete library open's failure, and refuses to send a library still opening", async () => {
    const first = deferred<unknown>();
    const OTHER_ID = "invented.hangers.other";
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "list_local_libraries") {
        return Promise.resolve([LIBRARY_ID, OTHER_ID].map((library_id) => ({ project_id: model().project.id, library_kind: "hanger", library_id, library_name: library_id, privacy_class: "private", storage_mode: "local", created_at_unix: 0, updated_at_unix: 0 })));
      }
      if (command === "open_local_library") {
        return JSON.stringify(args ?? {}).includes(OTHER_ID) ? new Promise(() => {}) : first.promise;
      }
      return Promise.reject(new Error(`unexpected ${command}`));
    });
    render(<HangerSizingForm model={model()} />);
    fireEvent.click(screen.getByRole("button", { name: "Refresh hanger libraries" }));
    await screen.findByRole("option", { name: OTHER_ID });
    select("hanger-sizing-library", LIBRARY_ID);
    select("hanger-sizing-library", OTHER_ID);
    // The first selection's open now fails: the library no longer exists. It is obsolete, so nothing is shown.
    await act(async () => { first.resolve({ project_id: model().project.id, library_kind: "hanger", library_id: LIBRARY_ID, storage_mode: "local", created_at_unix: 0, updated_at_unix: 0, document: null, validation: { accepted: true, has_blocking_findings: false, findings: [] }, message: "" }); });
    expect(screen.queryByText("This library no longer exists.")).toBeNull();
    // The other library is still opening: a request names the library as missing and sends nothing.
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    const missing = within(screen.getByTestId("hanger-sizing-missing")).getAllByRole("listitem").map((item) => item.textContent?.split(":")[0]);
    expect(missing).toContain("library");
    expect(invokeMock.mock.calls.some(([command]) => command === "size_hangers")).toBe(false);
  });

  it("shows a refused request's failure truthfully, by its code, and no table", async () => {
    host(() => Promise.reject("HANGER-SIZING-REPLY-TOO-LARGE: the sizing reply of 67108865 bytes is above the desktop's 67108864-byte limit for one reply; it was not read and its worker was stopped."));
    render(<HangerSizingForm model={model()} />);
    await fillTwoSpan();
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    const failure = await screen.findByTestId("hanger-sizing-failure");
    expect(failure).toHaveTextContent("HANGER-SIZING-REPLY-TOO-LARGE: the sizing reply of 67108865 bytes");
    expect(screen.queryByTestId("hanger-sizing-panel")).toBeNull();
  });

  it("never shows a reply that arrives after the person cancelled, and an unmount (a project-session change) retires the request", async () => {
    const pending = deferred<unknown>();
    host(() => pending.promise);
    const view = render(<HangerSizingForm model={model()} />);
    await fillTwoSpan();
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    await screen.findByTestId("hanger-sizing-running");
    fireEvent.click(screen.getByTestId("hanger-sizing-cancel"));
    await waitFor(() => expect(invokeMock.mock.calls.filter(([command]) => command === "cancel_hanger_sizing")).toHaveLength(1));
    pending.resolve(structuredClone(twoSpanReply));
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.queryByTestId("hanger-sizing-panel")).toBeNull();
    // A second request, unmounted while it runs: its cancel is sent and nothing is shown after.
    const second = deferred<unknown>();
    host(() => second.promise);
    fireEvent.click(screen.getByTestId("hanger-sizing-run"));
    await waitFor(() => expect(invokeMock.mock.calls.filter(([command]) => command === "size_hangers")).toHaveLength(2));
    const [, sent] = invokeMock.mock.calls.filter(([command]) => command === "size_hangers")[1] as [string, { runId: unknown }];
    view.unmount();
    await waitFor(() => expect(invokeMock.mock.calls.filter(([command]) => command === "cancel_hanger_sizing").at(-1)).toEqual(["cancel_hanger_sizing", { runId: sent.runId }]));
    second.resolve(structuredClone(twoSpanReply));
  });
});
