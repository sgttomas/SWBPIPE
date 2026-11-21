// An element state's analysis_basis_override in the analysis-state editor (issue 408):
// two fields while the member is absent or null, otherwise the stored value as JSON
// text with a remove button. The wrapper records the intent passed to validation and
// delegates to the real Wasm validator. All values are invented test inputs.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { warmupOperationEngine } from "../../services/operationService";
import { LoadReferenceStateInputs } from "./LoadReferenceStateInputs";
import { toPayload } from "./loadStateAuthoring";
import connected from "../../../../../fixtures/product_preview/pressure/load_reference/connected.request.json";

const validated = vi.hoisted(() => [] as EditorOperationIntent[]);
vi.mock("../../services/operationService", async importOriginal => {
  const actual = await importOriginal<typeof import("../../services/operationService")>();
  return {
    ...actual,
    validateModelOperation: (...args: Parameters<typeof actual.validateModelOperation>) => {
      validated.push(args[1]);
      return actual.validateModelOperation(...args);
    }
  };
});

afterEach(() => { cleanup(); validated.length = 0; });
beforeAll(async () => { expect((await warmupOperationEngine()).state).toBe("ready"); });

type Row = Record<string, unknown>;
const COPY_NOTE = "Copy replaces this draft's analysis state. Analysis-basis overrides from the source case are left out.";
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const click = (name: string) => fireEvent.click(screen.getByRole("button", { name }));
const field = (n: number, which: "reason" | "provenance") => screen.queryByRole("textbox", { name: `Element state ${n} analysis-basis ${which}` }) as HTMLInputElement | null;
const enterText = (n: number, which: "reason" | "provenance", value: string) => fireEvent.change(field(n, which)!, { target: { value } });
const shown = (n: number) => screen.queryByTestId(`element-state-${n}-analysis-basis-override`);
const removeButton = (n: number) => screen.queryByRole("button", { name: `Remove Element state ${n} analysis-basis override` });
const rowsOf = (model: PreviewModel, caseId: string) => (model.load_cases.find(c => c.id === caseId)!.analysis_state as unknown as { element_states: Row[] }).element_states;
/** Both fields of row n hold exactly these values. */
const fields = (n: number, reason: string, provenance: string) => {
  expect(field(n, "reason")).toHaveValue(reason);
  expect(field(n, "provenance")).toHaveValue(provenance);
};

/** The connected fixture with the given override member on each of case:cold's two element states (undefined: absent). */
function model(first?: unknown, second?: unknown): PreviewModel {
  const m = structuredClone(connected.model) as unknown as PreviewModel;
  const [a, b] = rowsOf(m, "case:cold");
  if (first !== undefined) a.analysis_basis_override = first;
  if (second !== undefined) b.analysis_basis_override = second;
  return m;
}

function renderCase(m: PreviewModel, caseId = "case:cold") {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<LoadReferenceStateInputs model={m} selection={{ type: "load", id: caseId }} onQueueIntent={queue} loadCase={m.load_cases.find(c => c.id === caseId)!} />);
  return queue;
}

/** Clicks Queue, waits for fresh feedback (Queue clears it first), then requires one
 * new validation call, so a local conversion error fails here rather than timing out.
 * Returns the analysis state that call was given. */
async function queueAndSentState(): Promise<{ element_states: Row[]; support_states: Row[] }> {
  const before = validated.length;
  click("Queue analysis state");
  await waitFor(() => expect(screen.queryByTestId("load-state-analysis-notice") ?? screen.queryByTestId("load-state-analysis-error")).not.toBeNull(), { timeout: 20_000 });
  expect(validated.length).toBe(before + 1);
  return JSON.parse(validated[before].change.after);
}
const queueAndSent = async () => (await queueAndSentState()).element_states;

const LONG = "x".repeat(600);
const STORED: Array<[string, unknown]> = [
  ["complete pair, carriage returns and non-ASCII", { reason: "line one\r\nline two — Ünïcode ✓", provenance: "  spaced\ttext  " }],
  ["pair with an empty string", { reason: "", provenance: "p" }],
  ["pair with both strings empty", { reason: "", provenance: "" }],
  ["pair with strings over 512 characters", { reason: LONG, provenance: `${LONG}\r` }],
  ["partial object", { reason: "only a reason" }],
  ["object with an extra key", { reason: "r", provenance: "p", extra: "e" }],
  ["object the conversion would read as an empty quantity", { value: "" }],
  ["object the conversion would read as a quantity", { value: "3", unit: "m" }],
  ["object with a number-converted key", { factor: "x" }],
  ["non-string member", { reason: 5, provenance: "p" }],
  ["nested object", { reason: { value: "1", unit: "m" }, provenance: { nested: [1, "two"] } }],
  ["string", "a string"],
  ["empty string", ""],
  ["number", 0],
  ["positional two-string array", ["reason", "provenance"]],
  ["array holding a quantity-shaped object", [{ value: "" }]]
];

describe("absent and null: the two fields", () => {
  it("absent and null each show the two empty fields and no stored value; untouched, absent is sent absent and null is sent null", async () => {
    renderCase(model(undefined, null));
    for (const n of [1, 2]) {
      fields(n, "", "");
      expect(shown(n)).toBeNull();
      expect(removeButton(n)).toBeNull();
    }
    let sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
    expect(sent[1].analysis_basis_override).toBeNull();
    // Text typed and then cleared is no text: the member is still sent as it was.
    enterText(1, "reason", "typed");
    fields(1, "typed", "");
    enterText(1, "reason", "");
    fields(1, "", "");
    sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
    expect(sent[1].analysis_basis_override).toBeNull();
    enterText(2, "provenance", "typed");
    fields(2, "", "typed");
    enterText(2, "provenance", "");
    fields(2, "", "");
    sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
    expect(sent[1].analysis_basis_override).toBeNull();
    change("Element state 1 operating temperature value", "21");
    sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
    expect(sent[1].analysis_basis_override).toBeNull();
    expect(sent[0].operating_temperature).toEqual({ value: 21, unit: "degC" });
  }, 60_000);

  it("a whitespace-only provenance alone is sent as the half and refused for its missing reason, with the text kept; the reason then queues the exact pair, which a same-row edit keeps", async () => {
    const queue = renderCase(model());
    const provenance = "   ";
    const reason = "  Reason — tested at 20 °C  ";
    enterText(1, "provenance", provenance);
    fields(1, "", provenance);
    let sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ provenance });
    expect(Object.hasOwn(sent[1], "analysis_basis_override")).toBe(false);
    expect(screen.getByTestId("load-state-analysis-error")).toHaveTextContent("Analysis state was refused by the operation engine; nothing was queued.");
    expect(screen.getByTestId("load-state-analysis-diagnostic-OP-LOAD-STATE-PAYLOAD-INVALID")).toHaveTextContent("missing field `reason`");
    expect(queue).not.toHaveBeenCalled();
    fields(1, "", provenance);
    enterText(1, "reason", reason);
    fields(1, reason, provenance);
    sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ reason, provenance });
    expect(Object.hasOwn(sent[1], "analysis_basis_override")).toBe(false);
    expect(queue).toHaveBeenCalledOnce();
    const intent = queue.mock.calls[0][0];
    expect(intent).toBe(validated[validated.length - 1]);
    expect(intent.operation_kind).toBe("modify");
    expect(intent.target).toEqual({ object_type: "Load", ref: "case:cold" });
    expect([intent.change.change_kind, intent.change.field_path]).toEqual(["update_load", "analysis_state"]);
    expect((JSON.parse(intent.change.after) as { element_states: Row[] }).element_states[0].analysis_basis_override).toEqual({ reason, provenance });
    expect(screen.getByTestId("load-state-analysis-notice")).toHaveTextContent("checked by the operation engine and queued");
    // A same-row edit keeps both entered fields.
    change("Element state 1 operating temperature value", "21");
    fields(1, reason, provenance);
    sent = await queueAndSent();
    expect(sent[0].operating_temperature).toEqual({ value: 21, unit: "degC" });
    expect(sent[0].analysis_basis_override).toEqual({ reason, provenance });
    expect(queue).toHaveBeenCalledTimes(2);
    expect(queue.mock.calls[1][0]).toBe(validated[validated.length - 1]);
  }, 60_000);

  it("the fields are bounded at 512 characters and named per row, uniquely", () => {
    renderCase(model(undefined, { reason: "r", provenance: "p" }));
    for (const which of ["reason", "provenance"] as const) {
      expect(screen.getAllByRole("textbox", { name: `Element state 1 analysis-basis ${which}` })).toHaveLength(1);
      expect(field(1, which)).toHaveAttribute("maxlength", "512");
      expect(field(2, which)).toBeNull();
    }
    expect(screen.getAllByRole("button", { name: "Remove Element state 2 analysis-basis override" })).toHaveLength(1);
    expect(removeButton(1)).toBeNull();
    click("Remove Element state 2 analysis-basis override");
    for (const n of [1, 2]) for (const which of ["reason", "provenance"] as const) {
      expect(screen.getAllByRole("textbox", { name: `Element state ${n} analysis-basis ${which}` })).toHaveLength(1);
      expect(field(n, which)).toHaveAttribute("maxlength", "512");
    }
  });
});

describe("Stored override fixtures display as JSON, expose a Remove button and reach validation unchanged", () => {
  for (const [kind, value] of STORED) {
    it(kind, async () => {
      renderCase(model(value));
      expect(shown(1)!.textContent).toBe(JSON.stringify(value));
      expect(field(1, "reason")).toBeNull();
      expect(field(1, "provenance")).toBeNull();
      expect(removeButton(1)).not.toBeNull();
      change("Element state 1 operating temperature value", "21");
      expect(shown(1)!.textContent).toBe(JSON.stringify(value));
      const sent = await queueAndSent();
      expect(sent[0].operating_temperature).toEqual({ value: 21, unit: "degC" });
      expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(true);
      expect(sent[0].analysis_basis_override).toStrictEqual(value);
      expect(shown(1)!.textContent).toBe(JSON.stringify(value));
    }, 30_000);
  }

  it("only the direct element-state member is exempt from conversion; the same key under a support state converts as the payload conversion converts it", async () => {
    const value = { value: "3", unit: "m" };
    const m = model(value);
    const supports = (m.load_cases.find(c => c.id === "case:cold")!.analysis_state as unknown as { support_states: Row[] }).support_states;
    supports[0].analysis_basis_override = structuredClone(value);
    renderCase(m);
    const sent = await queueAndSentState();
    expect(sent.element_states[0].analysis_basis_override).toStrictEqual(value);
    expect(sent.support_states[0].analysis_basis_override).toStrictEqual(toPayload(value));
    expect(sent.support_states[0].analysis_basis_override).toStrictEqual({ value: 3, unit: "m" });
  }, 30_000);

  it("remove deletes the member, the row then shows the two fields, and Queue sends the row without it", async () => {
    renderCase(model({ reason: "stored", provenance: "stored" }));
    click("Remove Element state 1 analysis-basis override");
    expect(shown(1)).toBeNull();
    expect(removeButton(1)).toBeNull();
    fields(1, "", "");
    const sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
  }, 30_000);
});

describe("pending text follows its row", () => {
  it("text entered on row 2, starting from provenance, stays with that row when row 1 is edited and removed and when rows are added", async () => {
    renderCase(model());
    enterText(2, "provenance", "second's provenance");
    fields(2, "", "second's provenance");
    let sent = await queueAndSent();
    expect(sent[1].analysis_basis_override).toEqual({ provenance: "second's provenance" });
    enterText(2, "reason", "second's reason");
    fields(2, "second's reason", "second's provenance");
    sent = await queueAndSent();
    expect(sent[1].analysis_basis_override).toEqual({ reason: "second's reason", provenance: "second's provenance" });
    change("Element state 1 operating temperature value", "21");
    fields(2, "second's reason", "second's provenance");
    fields(1, "", "");
    sent = await queueAndSent();
    expect(Object.hasOwn(sent[0], "analysis_basis_override")).toBe(false);
    expect(sent[1].analysis_basis_override).toEqual({ reason: "second's reason", provenance: "second's provenance" });
    click("Remove Element state 1");
    expect(screen.getByLabelText("Element state 1 pipe")).toHaveValue("pipe:second");
    fields(1, "second's reason", "second's provenance");
    expect(field(2, "reason")).toBeNull();
    sent = await queueAndSent();
    expect(sent).toHaveLength(1);
    expect(sent[0].pipe_ref).toBe("pipe:second");
    expect(sent[0].analysis_basis_override).toEqual({ reason: "second's reason", provenance: "second's provenance" });
    click("Add element state");
    fields(1, "second's reason", "second's provenance");
    fields(2, "", "");
    sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ reason: "second's reason", provenance: "second's provenance" });
    expect(Object.hasOwn(sent[1], "analysis_basis_override")).toBe(false);
  }, 60_000);
});

describe("fill and copy never carry the member; the copy control says so before any copy", () => {
  function fillMissing() {
    change("Element state template", "0");
    change("Element state fill targets", "missing");
    click("Fill element states");
  }

  it("a fill leaves an existing target row's stored and entered overrides intact, and adds rows without the template's", async () => {
    const stored = { reason: "row 2's own", provenance: "row 2's own" };
    renderCase(model(undefined, stored));
    enterText(1, "reason", "row 1's reason");
    fields(1, "row 1's reason", "");
    enterText(1, "provenance", "row 1's provenance");
    fields(1, "row 1's reason", "row 1's provenance");
    // Both pipes already have a row: the fill adds none and changes neither.
    fillMissing();
    fields(1, "row 1's reason", "row 1's provenance");
    expect(shown(2)!.textContent).toBe(JSON.stringify(stored));
    let sent = await queueAndSent();
    expect(sent).toHaveLength(2);
    expect(sent[0].analysis_basis_override).toEqual({ reason: "row 1's reason", provenance: "row 1's provenance" });
    expect(sent[1].analysis_basis_override).toEqual(stored);
    // With row 2 removed, the fill adds a row for pipe:second from row 1, without row 1's text.
    click("Remove Element state 2");
    expect(shown(2)).toBeNull();
    fields(1, "row 1's reason", "row 1's provenance");
    fillMissing();
    expect(screen.getByLabelText("Element state 2 pipe")).toHaveValue("pipe:second");
    fields(1, "row 1's reason", "row 1's provenance");
    fields(2, "", "");
    sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ reason: "row 1's reason", provenance: "row 1's provenance" });
    expect(Object.hasOwn(sent[1], "analysis_basis_override")).toBe(false);
  }, 60_000);

  it("a fill from a row holding a stored override adds rows without it", async () => {
    const m = model({ reason: "row 1's own", provenance: "row 1's own" });
    rowsOf(m, "case:cold").splice(1, 1);
    renderCase(m);
    fillMissing();
    expect(shown(2)).toBeNull();
    fields(2, "", "");
    const sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ reason: "row 1's own", provenance: "row 1's own" });
    expect(Object.hasOwn(sent[1], "analysis_basis_override")).toBe(false);
  }, 30_000);

  it("the copy sentence is shown before any copy, with an empty draft and with entered pairs, and describes the select and the button", () => {
    const m = model();
    delete (m.load_cases.find(c => c.id === "case:hot") as { analysis_state?: unknown }).analysis_state;
    renderCase(m, "case:hot");
    const check = () => {
      expect(screen.getByText(COPY_NOTE)).toBeInTheDocument();
      expect(screen.getByLabelText("Copy analysis state from case")).toHaveAccessibleDescription(COPY_NOTE);
      expect(screen.getByRole("button", { name: "Replace this draft's analysis state with the copy" })).toHaveAccessibleDescription(`${COPY_NOTE} Choose the case to copy from.`);
    };
    check();
    cleanup();
    renderCase(model(), "case:hot");
    enterText(1, "reason", "entered");
    enterText(1, "provenance", "entered");
    fields(1, "entered", "entered");
    check();
  });

  it("a replace by copy replaces the destination's rows, entered pairs included, and imports no override from the source", async () => {
    const m = model({ reason: "cold's own", provenance: "cold's own" }, ["cold", "own"]);
    rowsOf(m, "case:hot")[1].analysis_basis_override = { reason: "hot's stored", provenance: "hot's stored" };
    renderCase(m, "case:hot");
    enterText(1, "reason", "hot's entered");
    enterText(1, "provenance", "hot's entered");
    fields(1, "hot's entered", "hot's entered");
    let sent = await queueAndSent();
    expect(sent[0].analysis_basis_override).toEqual({ reason: "hot's entered", provenance: "hot's entered" });
    expect(sent[1].analysis_basis_override).toEqual({ reason: "hot's stored", provenance: "hot's stored" });
    change("Copy analysis state from case", "case:cold");
    click("Replace this draft's analysis state with the copy");
    for (const n of [1, 2]) {
      expect(shown(n)).toBeNull();
      fields(n, "", "");
    }
    sent = await queueAndSent();
    const source = rowsOf(m, "case:cold").map(row => Object.fromEntries(Object.entries(row).filter(([key]) => key !== "analysis_basis_override")));
    expect(sent).toEqual(source);
    for (const row of sent) expect(Object.hasOwn(row, "analysis_basis_override")).toBe(false);
  }, 30_000);
});
