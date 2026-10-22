/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.4, §5.4, §6.1 "The disclosure
 * line"; owner ruling 11B, 2026-10-21): the two rule-check panels show one disclosure line
 * for an admitted pressure-2 result that carries resolved load/reference states.
 * - The completeness panel (`RuleCheckPanel`) computes it from its Current result and keeps
 *   its download (`rule-check-load-reference-disclosure`).
 * - The run panel (`RuleCheckRunPanel`) takes it from the service's route on the Current
 *   and the transient path (`rule-check-run-load-reference-disclosure`), and clears it with
 *   the run result on a basis change or a new run.
 * Neither line appears for a plain result. Unit transport replay of committed producer
 * bytes through mocked IPC; the rule backend's answer is a unit simulation, NOT
 * rule-evaluation evidence or a native UI qualification witness. All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { hasNativeMechanicsInvocation, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE } from "../../services/ruleCheckService";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, numericalResultStanding } from "../results/resultStanding";
import { N_LOAD_REFERENCE_RULE_CHECK } from "../results/loadReferenceDisclosure";
import { RuleCheckPanel } from "./RuleCheckPanel";
import { RuleCheckRunPanel } from "./RuleCheckRunPanel";
import type { MechanicsResult, PreviewModel } from "../../types";

type Mode = PreviewSolverMode;
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const ORDINARY = "fixtures/product_preview/pressure/load_reference";
const JOINED = "fixtures/product_preview/pressure_source/load_reference_source";
const RULE_ANSWER = {
  document_kind: "openpipestress.rule_check.run",
  rule_pack_id: "invented-test-only",
  grammar_version: "1.0.0",
  aggregate_status: "RULE_INPUTS_INCOMPLETE",
  checks: [],
  professional_boundary_notice: "notice"
};
/** A pack with no inputs: the run needs no unit catalog and no bindings. */
const PACK_TEXT = JSON.stringify({ metadata: { rule_pack_id: "invented-test-only" } });
type Input = { model: PreviewModel; bytes: string };

/** A committed pressure-2 load/reference-state request and its producer bytes, with the
 * document metadata a desktop model carries where the request lacks it (invented case
 * labels, an empty diagnostics list and the exact profile document's data boundary; the
 * product reads no physics from them). */
function loadState(name: "connected" | "pressure", mode: Mode): Input {
  const model = structuredClone(JSON.parse(text(`${ORDINARY}/${name}.request.json`)).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  const document = nativeMechanicsReplayPair(mode, { profile: "pressure" }).model as unknown as Record<string, unknown>;
  Object.assign(model as unknown as Record<string, unknown>, { diagnostics: [], data_boundary: structuredClone(document.data_boundary) });
  return { model, bytes: text(`${ORDINARY}/${name}-${mode}.raw.json`) };
}
/** The exact profile's plain pressure-2 pair (no load/reference-state group). */
function plain(mode: Mode): Input {
  const pair = nativeMechanicsReplayPair(mode, { profile: "pressure" });
  return { model: pair.model, bytes: pair.bytes };
}
/** A committed pressure-source-1 request, sent unchanged (its receipt binds it). */
function joined(name: string, mode: Mode): Input {
  return { model: JSON.parse(text(`${JOINED}/${name}.request.json`)).model as PreviewModel, bytes: text(`${JOINED}/${name}-${mode}.raw.json`) };
}
/** The native job transport for this input and mode, and the mocked rule backend. */
function install({ model, bytes }: Input, mode: Mode, ruleAnswer: () => unknown = () => structuredClone(RULE_ANSWER)) {
  const transport = createSolveJobTransport((sent) => {
    expect(sent.mode).toBe(mode);
    expect(sent.model).toEqual(model);
    return bytes;
  });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => command === "run_rule_checks" ? ruleAnswer() : transport.invoke(command, args, options));
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return transport;
}
/** A natively solved, registered source. */
async function solved(input: Input, mode: Mode) {
  const transport = install(input, mode);
  const result = await runPreviewMechanics(input.model, mode);
  expect(hasNativeMechanicsInvocation(result, input.model, mode)).toBe(true);
  invokeMock.mockClear();
  return { result, transport };
}
/** The line, from the published records, in published order, without duplicates. */
function expectedLine(bytes: string): string {
  const records = JSON.parse(bytes).contract_evidence.load_reference_states as { reference_configuration_id: string; load_case_id: string; history: { kind: string } }[];
  const distinct = (values: string[]) => [...new Set(values)].join(", ");
  return N_LOAD_REFERENCE_RULE_CHECK
    .replace("{configurations}", distinct(records.map(r => r.reference_configuration_id)))
    .replace("{cases}", distinct(records.map(r => r.load_case_id)))
    .replace("{histories}", distinct(records.map(r => r.history.kind)));
}
const ruleRuns = () => invokeMock.mock.calls.filter(([command]) => command === "run_rule_checks");
async function pasteAndRun() {
  fireEvent.change(screen.getByTestId("rule-check-pack-json"), { target: { value: PACK_TEXT } });
  fireEvent.click(screen.getByTestId("rule-check-run"));
}
function deferred<T>() {
  let resolvePromise!: (value: T) => void;
  const promise = new Promise<T>((done) => { resolvePromise = done; });
  return { promise, resolve: resolvePromise };
}

beforeEach(() => { invokeMock.mockReset(); });
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const MODES: Mode[] = ["sparse_interactive", "dense_scrutiny"];

describe("the rule-check completeness panel (RuleCheckPanel)", () => {
  it.each((["connected", "pressure"] as const).flatMap(name => MODES.map(mode => [name, mode] as const)))(
    "shows its download and the disclosure line for an eligible pressure-2 load/reference-state %s (%s) result", async (name, mode) => {
      const input = loadState(name, mode);
      const { result } = await solved(input, mode);
      expect(numericalResultStanding(result, input.model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
      render(<RuleCheckPanel model={input.model} result={result} />);
      expect(screen.getByTestId("rule-check-export-link")).toBeInTheDocument();
      expect(screen.queryByTestId("rule-check-load-reference-output-unavailable")).toBeNull();
      const line = screen.getByTestId("rule-check-load-reference-disclosure");
      expect(line).toHaveTextContent(expectedLine(input.bytes));
      expect(line.textContent).toBe(expectedLine(input.bytes));
      // It names the published reference configuration, load cases and history kind.
      for (const record of JSON.parse(input.bytes).contract_evidence.load_reference_states) {
        expect(line.textContent).toContain(record.reference_configuration_id);
        expect(line.textContent).toContain(record.load_case_id);
        expect(line.textContent).toContain(record.history.kind);
      }
    });

  it.each(MODES)("shows its download and no disclosure line for a plain pressure-2 result (%s)", async mode => {
    const input = plain(mode);
    const { result } = await solved(input, mode);
    render(<RuleCheckPanel model={input.model} result={result} />);
    expect(screen.getByTestId("rule-check-export-link")).toBeInTheDocument();
    expect(screen.queryByTestId("rule-check-load-reference-disclosure")).toBeNull();
  });

  it("shows no disclosure line without a result", () => {
    render(<RuleCheckPanel model={plain("sparse_interactive").model} result={null} />);
    expect(screen.queryByTestId("rule-check-load-reference-disclosure")).toBeNull();
  });
});

describe("the rule-check run panel (RuleCheckRunPanel)", () => {
  it.each(MODES)("Current path (%s): shows the service's disclosure line after a run over an eligible pressure-2 load/reference-state result", async mode => {
    const input = loadState("connected", mode);
    const { result } = await solved(input, mode);
    render(<RuleCheckRunPanel model={input.model} result={result} />);
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
    await pasteAndRun();
    await screen.findByTestId("rule-check-run-result");
    expect(screen.getByTestId("rule-check-run-load-reference-disclosure").textContent).toBe(expectedLine(input.bytes));
    expect(ruleRuns()).toHaveLength(1);
    expect((ruleRuns()[0][1] as { solvedEnvelope: MechanicsResult }).solvedEnvelope.run_id).toBe(result.run_id);
    // UI text only: the line is not part of the run result the panel shows.
    expect(screen.getByTestId("rule-check-run-result").textContent).not.toContain("load/reference states");
  });

  it("transient path: with no Current result the service solves, the panel shows the line, and the solve is released", async () => {
    const mode: Mode = "sparse_interactive";
    const input = loadState("pressure", mode);
    const transport = install(input, mode);
    render(<RuleCheckRunPanel model={input.model} result={null} />);
    await pasteAndRun();
    await screen.findByTestId("rule-check-run-result");
    expect(screen.getByTestId("rule-check-run-load-reference-disclosure").textContent).toBe(expectedLine(input.bytes));
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual([
      "start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result",
      "run_rule_checks", "dispose_preview_mechanics_result"
    ]);
    expect(transport.custody()).toEqual([]);
  });

  it("clears the line with the run result when the basis changes", async () => {
    const mode: Mode = "sparse_interactive";
    const input = loadState("connected", mode);
    const { result } = await solved(input, mode);
    const basis = { projectSessionGeneration: 2, modelRevision: 3, resultSequence: 4 };
    const view = render(<RuleCheckRunPanel basis={basis} model={input.model} result={result} />);
    await pasteAndRun();
    await screen.findByTestId("rule-check-run-load-reference-disclosure");
    view.rerender(<RuleCheckRunPanel basis={{ ...basis, resultSequence: 5 }} model={input.model} result={result} />);
    expect(screen.queryByTestId("rule-check-run-result")).toBeNull();
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
    // Returning to the old basis key does not revive it: the presentation was cleared.
    view.rerender(<RuleCheckRunPanel basis={basis} model={input.model} result={result} />);
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
  });

  it("clears the line when a new run starts, and shows the new run's line when it completes", async () => {
    const mode: Mode = "sparse_interactive";
    const input = loadState("connected", mode);
    const { result } = await solved(input, mode);
    const second = deferred<unknown>();
    let runs = 0;
    invokeMock.mockImplementation(async (command: string) => {
      if (command !== "run_rule_checks") throw new Error(`unexpected command ${command}`);
      return ++runs === 1 ? structuredClone(RULE_ANSWER) : second.promise;
    });
    render(<RuleCheckRunPanel model={input.model} result={result} />);
    await pasteAndRun();
    await screen.findByTestId("rule-check-run-load-reference-disclosure");
    fireEvent.click(screen.getByTestId("rule-check-run"));
    await waitFor(() => expect(runs).toBe(2));
    expect(screen.queryByTestId("rule-check-run-result")).toBeNull();
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
    await act(async () => { second.resolve({ ...RULE_ANSWER, aggregate_status: "USER_RULE_CHECKED" }); });
    await screen.findByTestId("rule-check-run-result");
    expect(screen.getByTestId("rule-check-run-load-reference-disclosure").textContent).toBe(expectedLine(input.bytes));
  });

  it.each([
    ["Current", (input: Input, result: MechanicsResult) => <RuleCheckRunPanel model={input.model} result={result} />],
    ["transient", (input: Input) => <RuleCheckRunPanel model={input.model} result={null} />],
  ] as const)("%s path: no line for a plain pressure-2 result", async (path, panel) => {
    const mode: Mode = "sparse_interactive";
    const input = plain(mode);
    let result: MechanicsResult | null = null;
    if (path === "Current") ({ result } = await solved(input, mode));
    else install(input, mode);
    render(panel(input, result!));
    await pasteAndRun();
    await screen.findByTestId("rule-check-run-result");
    expect(ruleRuns()).toHaveLength(1);
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
  });

  it("a pressure-source-1 load/reference-state solve stops before the backend: no result and no line", async () => {
    const mode: Mode = "sparse_interactive";
    const input = joined("n05", mode);
    const transport = install(input, mode);
    render(<RuleCheckRunPanel model={input.model} result={null} />);
    await pasteAndRun();
    await waitFor(() => expect(screen.getByTestId("rule-check-run-status").textContent)
      .toContain(`${LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(ruleRuns()).toHaveLength(0);
    expect(screen.queryByTestId("rule-check-run-result")).toBeNull();
    expect(screen.queryByTestId("rule-check-run-load-reference-disclosure")).toBeNull();
    expect(transport.custody()).toEqual([]);
  });
});
