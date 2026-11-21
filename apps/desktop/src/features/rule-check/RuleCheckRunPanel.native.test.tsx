// The rule-check run panel over a native handle (C1 custody): the selector preview, the binding
// pre-check and the run read the held result by its handle, and no full copy of the result enters
// the webview. The native solve's readers answer from the actual Rust commands' recorded replies;
// `preview_rule_solver_rows` and `run_rule_checks` are mocked here (their native behaviour is
// pinned by `rule_check_handle_tests.rs`), the preview's reply computed from the same bytes by the
// webview's mirror of the run's selection. Unit transport replay; NOT native UI qualification.
// Invented data only.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { nativeCustody, resetNativeResultsForTests, type NativeResult } from "../../services/nativeResult";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { readBasisRef, selectSolverRows, solverSelectorForRow, type RuleCheckBasisRef, type SolverResultRef } from "../../services/ruleCheckService";
import { ruleBindingRefusal } from "../results/knownSemanticLimitations";
import { RuleCheckRunPanel } from "./RuleCheckRunPanel";
import type { MechanicsResult } from "../../types";
import demoPack from "../../../../../fixtures/product_preview/invented_demo_rule_pack.json";

const RULE_ANSWER = {
  document_kind: "openpipestress.rule_check.run",
  rule_pack_id: "invented_demo_rule_pack",
  grammar_version: "1.0.0",
  aggregate_status: "RULE_INPUTS_INCOMPLETE",
  checks: [],
  professional_boundary_notice: "unit simulation"
};

/** A one-entry catalog: the panel's value fields need one; no conversion is exercised here. */
const UNIT_CATALOG = {
  schema_version: "1.0.0", catalog_id: "unit-system:invented-test", decision_basis: "DEC-018", calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved", entry_count: 1,
  entries: [{ unit_id: "unit:ratio", symbol: "ratio", dimension_id: "dimensionless", canonical: true, transform_kind: "identity", factor_representation: "1", offset_representation: null, provenance: "si_canonical", review_status: "accepted" }]
};
const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
const full = JSON.parse(pair.bytes) as MechanicsResult;
const commands = () => invokeMock.mock.calls.map(([command]) => command as string);

beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); });
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The preview reply for `queries`, from the full bytes, by the webview's mirror of the run's selection. */
function previewReply(queries: { input_id: string; selector: SolverResultRef; bases: RuleCheckBasisRef[] }[]) {
  return {
    document_kind: "swbpipe.desktop.rule_solver_rows_preview",
    version: 1,
    row_count: full.results.length,
    inputs: queries.map((query) => ({
      input_id: query.input_id,
      selector_readable: true,
      bases: query.bases.map((basis) => {
        const selected = selectSolverRows(full, query.selector, basis);
        return {
          basis_ref: basis,
          solved: full.results.some((row) => { const read = readBasisRef(row.basis_ref); return read?.ref_type === basis.ref_type && read.ref_id === basis.ref_id; }),
          matched: Math.min(selected.length, 2),
          rows: selected.length === 1 ? selected.map((row) => ({ id: row.id, value: row.value, unit: row.unit, dimension: row.dimension ?? null, binding_refusal: ruleBindingRefusal(full, row) })) : []
        };
      })
    })),
    job_id: "$JOB"
  };
}

async function nativeResult(): Promise<NativeResult> {
  const replay = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "preview_rule_solver_rows") return Promise.resolve(previewReply((args as { queries: never }).queries));
    if (command === "run_rule_checks") return Promise.resolve(structuredClone(RULE_ANSWER));
    if (command === "get_unit_catalog") return Promise.resolve(UNIT_CATALOG);
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  return (await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model })).result;
}

/** The demo pack with its solver input's selector naming `row`, on the row's own basis. */
function packFor(row: MechanicsResult["results"][number]) {
  const pack = structuredClone(demoPack) as Record<string, unknown> & { required_inputs: Record<string, unknown>[]; required_load_cases: Record<string, unknown>[] };
  pack.required_inputs[0].solver_result_ref = solverSelectorForRow(full, row);
  pack.required_load_cases[0].basis_ref = row.basis_ref;
  return pack;
}

it("previews the selector's rows and runs the check by the result's handle, copying nothing whole", async () => {
  const result = await nativeResult();
  const { jobId } = nativeCustody(result);
  const row = full.results.find((candidate) => candidate.basis_ref && candidate.kind && candidate.entity_ref && solverSelectorForRow(full, candidate))!;
  render(<RuleCheckRunPanel model={pair.model} result={result} />);
  // The row count is S's: no row of the result is read into the webview for it.
  expect(screen.getByTestId("rule-check-run-solve-status").textContent).toContain(`(${full.results.length} result row(s))`);
  fireEvent.change(screen.getByTestId("rule-check-pack-json"), { target: { value: JSON.stringify(packFor(row)) } });

  fireEvent.click(await screen.findByTestId("rule-check-solver-preview-demo_actual_quantity"));
  const basis = row.basis_ref as RuleCheckBasisRef;
  const shown = await screen.findByTestId(`rule-check-solver-basis-demo_actual_quantity--${basis.ref_type}--${basis.ref_id}`);
  expect(shown).toHaveAttribute("data-status", "resolves");
  expect(shown.textContent).toContain(row.id);
  const previews = invokeMock.mock.calls.filter(([command]) => command === "preview_rule_solver_rows");
  expect(previews.length).toBeGreaterThan(0);
  for (const [, args] of previews) expect((args as { jobId: string }).jobId).toBe(jobId);

  fireEvent.click(screen.getByTestId("rule-check-run"));
  await screen.findByTestId("rule-check-run-result");
  const runs = invokeMock.mock.calls.filter(([command]) => command === "run_rule_checks");
  expect(runs).toHaveLength(1);
  expect(runs[0][1]).toMatchObject({ jobId, model: pair.model });
  expect(runs[0][1]).not.toHaveProperty("solvedEnvelope");
  // Nothing of the result was copied whole or taken.
  expect(commands()).not.toContain("read_preview_mechanics_job_rows_all");
  expect(commands()).not.toContain("take_preview_mechanics_job_result");
  await disposeDeliveredMechanicsResult(result);
});

it("refuses a preview once the result's custody has ended, and sends nothing for it", async () => {
  const result = await nativeResult();
  const row = full.results.find((candidate) => candidate.basis_ref && solverSelectorForRow(full, candidate))!;
  render(<RuleCheckRunPanel model={pair.model} result={result} />);
  fireEvent.change(screen.getByTestId("rule-check-pack-json"), { target: { value: JSON.stringify(packFor(row)) } });
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "preview_rule_solver_rows")).toBe(true));
  await disposeDeliveredMechanicsResult(result);
  invokeMock.mockClear();
  fireEvent.click(screen.getByTestId("rule-check-solver-preview-demo_actual_quantity"));
  await waitFor(() => expect(screen.getByTestId("rule-check-run-status").textContent).toContain("RULE_CHECK_RESULT_NOT_HELD"));
  expect(commands()).not.toContain("preview_rule_solver_rows");
});

it("discards a preview reply that arrives after the result's custody ended, and shows no rows from it", async () => {
  const result = await nativeResult();
  const row = full.results.find((candidate) => candidate.basis_ref && candidate.kind && candidate.entity_ref && solverSelectorForRow(full, candidate))!;
  render(<RuleCheckRunPanel model={pair.model} result={result} />);
  fireEvent.change(screen.getByTestId("rule-check-pack-json"), { target: { value: JSON.stringify(packFor(row)) } });
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === "preview_rule_solver_rows")).toBe(true));
  // From here, the preview's reply arrives only after the session released the result.
  const answer = invokeMock.getMockImplementation()!;
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command !== "preview_rule_solver_rows") return answer(command, args, options);
    const reply = await answer(command, args, options);
    await disposeDeliveredMechanicsResult(result);
    return reply;
  });
  fireEvent.click(screen.getByTestId("rule-check-solver-preview-demo_actual_quantity"));
  await waitFor(() => expect(screen.getByTestId("rule-check-run-status").textContent).toContain("RULE_CHECK_RESULT_NOT_HELD"));
  const basis = row.basis_ref as RuleCheckBasisRef;
  expect(screen.queryByTestId(`rule-check-solver-basis-demo_actual_quantity--${basis.ref_type}--${basis.ref_id}`)).toBeNull();
});
