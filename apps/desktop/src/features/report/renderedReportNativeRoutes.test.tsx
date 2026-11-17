// DEL-08-01 REQ-022: the report panel's Save canonical HTML and Print in the
// desktop runtime go to the native commands (`renderedReportRoutes.ts`), carry
// the canonical HTML and the controlled export's evidence, and show the
// outcome in words; each keeps the gate its link or iframe had (the save the
// HTML route's control, which withholds a public render's HTML whole; the
// print of a public render the render's own gate); a blocked render offers
// neither; a discarded render is retired natively; outside the desktop the
// link and the print iframe stay.
// The render path is mocked: these tests exercise only the two controls.
// All fixture data is invented. Mocked invoke only.
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useSyncExternalStore } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { ingressArgs } from "../../test/nativeIngress";

const invokeMock = vi.hoisted(() => vi.fn());
const assemblyControl = vi.hoisted(() => ({ failure: false, blocked: false, pending: null as Promise<void> | null }));
const outcomeRef = vi.hoisted(() => ({ current: null as Record<string, unknown> | null }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
vi.mock("./ReportComposer", () => ({ ReportComposer: () => null }));
vi.mock("../results/LoadReferenceOutputGate", () => ({
  LoadReferenceOutputGate: ({ children }: { children: unknown }) => children
}));
vi.mock("../results/outputPolicy", async (original) => ({
  ...(await original<typeof import("../results/outputPolicy")>()),
  surfaceOutputRefusal: () => null
}));
vi.mock("./renderableReportInput", async (original) => ({
  ...(await original<typeof import("./renderableReportInput")>()),
  buildUnitDisplaySummary: () => ({ model_units: {}, result_units: [], conversion_performed: false })
}));
vi.mock("./renderedReportAssembly", () => ({
  assembleRenderedReport: async ({ context }: { context: string }) => {
    await assemblyControl.pending;
    if (assemblyControl.failure) throw new Error("INVENTED-ASSEMBLY-FAILURE");
    return {
      controlled: {
        payload: { report_context: { export_context: context } },
        decisions: [],
        findings: [],
        blocked: assemblyControl.blocked,
        summary: {
          route_id: "DREP-IPC-003",
          decision_count: 0,
          finding_count: 0,
          warning_count: 0,
          blocking_count: 0,
          local_first: { explicit_local_private_intent: context === "local_private" }
        }
      },
      exactSubmission: null
    };
  }
}));
/** A render request's native entry and completion, when a test models them (`nativeRender`);
 * otherwise the render answers at once. */
const nativeRender = vi.hoisted(() => ({
  current: null as null | { enter: (source: { incarnation: number; sequence: number }, request: number) => Promise<void> | void; completion: () => Promise<void> },
  pending: null as null | (() => void),
  complete: null as null | (() => void),
  autoComplete: true
}));
vi.mock("../../services/reportRenderService", () => ({
  renderCalculationReport: async (_input: unknown, options: { source?: { incarnation: number; sequence: number }; request?: number } = {}) => {
    if (nativeRender.current) {
      await nativeRender.current.enter(options.source ?? { incarnation: 0, sequence: 0 }, options.request ?? 0);
      await nativeRender.current.completion();
    }
    return { route: "tauri_renderer", outcome: outcomeRef.current };
  }
}));

import { RenderedReportPanel } from "./RenderedReportPanel";
import { ReportSourceSequence } from "../workspace/reportSourceSequence";
import { ReportOutputOutcomes } from "./reportOutputOutcomes";
import { ReportOutputStatus } from "./ReportOutputStatus";
import { NO_LIBRARY_REF_FINDINGS, type LibraryRefFindingsState } from "../library/libraryRefFindings";

const HTML = "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>";
const SHA = "b".repeat(64);

function outcome(blocked = false) {
  return {
    html: HTML,
    sha256_hex: SHA,
    export_blocked: blocked,
    blocking_reasons: blocked ? ["pre-render lint: InventedBlock"] : [],
    report_validation_diagnostics: [],
    section_validation_diagnostics: [],
    pre_render_findings: [],
    post_render_findings: [],
    derived_print_html: "<div>DERIVED VIEW bbbb</div>"
  };
}

const model = { project: { id: "project:invented", units: {} } } as unknown as PreviewModel;
const analysisRun = { analysis_run: { analysis_status: [] } } as unknown as AnalysisRunEnvelope;
function result(id = "run:invented-1") {
  return { run_id: id, results: [], diagnostics: [] } as unknown as MechanicsResult;
}

function panel(source = result()) {
  return <RenderedReportPanel model={model} result={source} analysisRun={analysisRun} projectSummary={null} />;
}

/** The route command calls, with their raw ingress bodies decoded: each must be a
 * listed ingress command sent as a raw UTF-8 JSON body with the ingress header
 * (`ingressArgs`, which states the wire contract independently of the production
 * helper). */
function routeCalls(command: string): Record<string, unknown>[] {
  return invokeMock.mock.calls
    .filter(([name]) => name === command)
    .map((call) => {
      expect(Object.prototype.toString.call(call[1])).toBe("[object Uint8Array]");
      expect(call[2]).toEqual({ headers: { "x-swbpipe-ingress": "args-json-v1" } });
      return ingressArgs(call);
    });
}

async function renderShown(localPrivate = false) {
  const view = render(panel());
  if (localPrivate) fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
  fireEvent.click(screen.getByTestId("rendered-report-render"));
  await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
  return view;
}

const SAVED = {
  outcome: "saved",
  route_id: "DREP-HTML-SAVE-005",
  export_context: "local_private",
  sha256_hex: SHA,
  byte_count: HTML.length,
  file_name: "invented-report.html",
  replaced_existing: false,
  durability: "durable"
};

beforeEach(() => {
  invokeMock.mockReset();
  assemblyControl.failure = false;
  assemblyControl.blocked = false;
  assemblyControl.pending = null;
  outcomeRef.current = outcome();
  nativeRender.current = null;
  nativeRender.pending = null;
  nativeRender.complete = null;
  nativeRender.autoComplete = true;
});

afterEach(() => {
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("native runtime", () => {
  beforeEach(() => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  });

  it("saves a local-private render's canonical HTML through the native command and states the outcome", async () => {
    invokeMock.mockImplementation(async (command: string) => (command === "save_rendered_report_html" ? SAVED : undefined));
    await renderShown(true);
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    const save = screen.getByTestId("rendered-report-save-local-private");
    expect(save.tagName).toBe("BUTTON");
    expect(save).toHaveAttribute("type", "button");
    expect(save).not.toHaveAttribute("href");
    expect(save).toBeEnabled();
    save.focus();
    expect(document.activeElement).toBe(save);
    expect(save.closest("[data-route-id]")).toHaveAttribute("data-route-id", "DREP-HTML-SAVE-005");
    expect(screen.getByTestId("rendered-report-save-local-private-redaction-summary")).toHaveTextContent("blocked=false");
    // DEL-08-01 REQ-023 Coverage: where an HTML save or a print is offered, the panel says neither is listed and
    // that the package is what the project keeps.
    expect(screen.getByTestId("rendered-report-list-notice")).toHaveTextContent(
      "HTML saves and printed reports are not listed in the project. Save a report package to keep it in the project’s report list."
    );

    fireEvent.click(save);
    await waitFor(() => expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-report.html"));
    expect(screen.getByTestId("rendered-report-save-status")).toHaveAttribute("role", "status");
    expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent(`SHA-256 ${SHA}`);
    expect(routeCalls("save_rendered_report_html")).toEqual([
      {
        html: HTML,
        exportContext: "local_private",
        explicitLocalPrivateIntent: true,
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "local_private", explicit_local_private_intent: true, blocked: false, blocking_count: 0 },
        // DEL-08-01 REQ-022: the render's report source (no session sequence here: 0/0).
        source: { incarnation: 0, sequence: 0 }
      }
    ]);
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
  });

  it("keeps a public render's HTML save withheld, as its link was, and sends nothing", async () => {
    await renderShown();
    const save = screen.getByTestId("rendered-report-save");
    expect(save.tagName).toBe("BUTTON");
    expect(save).toBeDisabled();
    expect(screen.getByTestId("rendered-report-save-redaction-summary")).toHaveTextContent("blocked=true");
    fireEvent.click(save);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("states a cancel, a refusal, an unconfirmed durability and a failed write in words", async () => {
    const answers: unknown[] = [
      { ...SAVED, outcome: "cancelled", durability: "not_applicable" },
      { ...SAVED, durability: "saved_durability_uncertain" }
    ];
    const rejections: unknown[] = [
      { code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current.", destination: "untouched", cleanup_code: null, selected_basename: null },
      { code: "REPORT-HTML-SAVE-RENAME-FAILED", stage: "rename", message: "injected", destination: "as_it_was", cleanup_code: null, selected_basename: "r.html" }
    ];
    invokeMock.mockImplementation(async (command: string) => {
      if (command !== "save_rendered_report_html") return undefined;
      if (answers.length) return answers.shift();
      throw rejections.shift();
    });
    await renderShown(true);
    const status = () => screen.getByTestId("rendered-report-save-status");
    const click = async (expected: string) => {
      await waitFor(() => expect(screen.getByTestId("rendered-report-save-local-private")).toBeEnabled());
      fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
      await waitFor(() => expect(status()).toHaveTextContent(expected));
    };
    await click("Save cancelled: nothing was written.");
    await click("did not confirm it is durable on disk");
    await click("Not saved (REPORT-ROUTE-NOT-CURRENT): No rendered report is current.");
    await click("The destination was left as it was.");
  });

  it("prints a public render through the native command on its own render gate and mounts no print iframe", async () => {
    invokeMock.mockImplementation(async (command: string) =>
      command === "print_rendered_report"
        ? { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: SHA, print_sha256_hex: "c".repeat(64) }
        : undefined
    );
    await renderShown();
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent(`Print window opened with the derived print view of SHA-256 ${SHA}`));
    expect(routeCalls("print_rendered_report")).toEqual([
      {
        html: HTML,
        exportContext: "public_report",
        explicitLocalPrivateIntent: false,
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "public_report", explicit_local_private_intent: false, blocked: false, blocking_count: 0 },
        // DEL-08-01 REQ-022: the render's report source (no session sequence here: 0/0).
        source: { incarnation: 0, sequence: 0 }
      }
    ]);
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
  });

  it("states a refused print by name", async () => {
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "print_rendered_report") {
        throw { code: "REPORT-PRINT-ALREADY-OPEN", stage: "print", message: "A print window is already open.", destination: "untouched", cleanup_code: null, selected_basename: null };
      }
      return undefined;
    });
    await renderShown();
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent("Not printed (REPORT-PRINT-ALREADY-OPEN)"));
  });

  it("offers neither route for a blocked render and sends nothing", async () => {
    outcomeRef.current = outcome(true);
    await renderShown();
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-save-status")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-list-notice")).not.toBeInTheDocument();
    expect(invokeMock).not.toHaveBeenCalled();
  });

  // Currency (DEL-08-01 REQ-022 and AC-011, as Agent 0 narrowed them under the owner's grant):
  // a source change the desktop application's native process has registered before atomic
  // publication retires the earlier output; one still in transit may leave the earlier selected
  // report saved, and the session's report output outcome then says so. The real panel, the real
  // report source sequence and outcome owner run against mocked IPC whose native side models the
  // contract `rendered_report_routes.rs` implements and its own tests check: an incarnation, the
  // highest registered source number, a render ticket taken at entry, the request it was issued
  // to (source and ordinal) and the entry watermark, a retirement naming its request, the current
  // record, and a publication gate at the rename. The session's sequenced setters are stood in for by calling
  // `advance()` beside each source replacement. Expected values come from REQ-022, not the model.
  type Mark = { incarnation: number; sequence: number };
  function modelledNative() {
    const native = {
      incarnation: 0, registered: 0, ticket: 0, record: null as null | Mark,
      active: null as null | { source: Mark; request: number }, watermark: 0,
      retirements: [] as Array<{ source: Mark; request: number }>, holdRetires: false, heldRetires: [] as Array<() => void>,
      entered: [] as number[],
      published: [] as string[], advancesSent: 0, retiresSent: 0, holdAdvances: false, failAdvances: false,
      heldAdvances: [] as Array<() => void>, pendingSaves: [] as Array<() => void>, heldRenders: [] as Array<() => void>, holdRenders: false
    };
    const register = (mark: Mark): Mark => {
      if (mark.incarnation !== native.incarnation) throw { code: "REPORT-SOURCE-INCARNATION-RETIRED", stage: "source", message: "retired" };
      if (mark.sequence > native.registered) {
        native.registered = mark.sequence;
        native.record = null;
        native.ticket += 1;
        native.active = null;
      }
      return { incarnation: native.incarnation, sequence: native.registered };
    };
    const current = (mark: Mark) => mark.incarnation === native.incarnation && mark.sequence === native.registered;
    const transport = {
      observe: async (): Promise<Mark> => ({ incarnation: native.incarnation, sequence: native.registered }),
      activate: async (expected: number): Promise<Mark> => {
        if (expected !== native.incarnation) throw { code: "REPORT-SOURCE-ACTIVATION-CONFLICT", stage: "source", message: "conflict" };
        native.incarnation += 1;
        native.registered = 0;
        native.record = null;
        native.ticket += 1;
        native.active = null;
        native.watermark = 0;
        return { incarnation: native.incarnation, sequence: 0 };
      },
      advance: (mark: Mark): Promise<Mark> => {
        native.advancesSent += 1;
        if (native.failAdvances) return Promise.reject(new Error("invented transport failure"));
        if (!native.holdAdvances) return Promise.resolve().then(() => register(mark));
        return new Promise<Mark>((resolve, reject) => native.heldAdvances.push(() => {
          try { resolve(register(mark)); } catch (error) { reject(error); }
        }));
      }
    };
    nativeRender.current = {
      enter: async (source, request) => {
        if (native.holdRenders) await new Promise<void>((resolve) => native.heldRenders.push(resolve));
        if (!current(source)) throw new Error("REPORT-SOURCE-RETIRED: the report's source changed after this request was prepared.");
        if (request <= native.watermark) throw new Error("REPORT-RENDER-SUPERSEDED: a later request entered, or this one was retired before it entered.");
        native.watermark = request;
        native.entered.push(request);
        native.ticket += 1;
        native.record = null;
        native.active = { source, request };
        const ticket = native.ticket;
        nativeRender.pending = () => {
          if (ticket !== native.ticket || !current(source)) throw new Error("REPORT-RENDER-SUPERSEDED: not recorded.");
          native.record = source;
        };
      },
      completion: async () => {
        await new Promise<void>((resolve) => { nativeRender.complete = resolve; if (nativeRender.autoComplete) resolve(); });
        nativeRender.pending?.();
      }
    };
    nativeRender.autoComplete = true;
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (command === "retire_rendered_report") {
        native.retiresSent += 1;
        const named = args as { source: Mark; request: number };
        native.retirements.push(named);
        // `RenderCustody::retire_request`: only the exact active request; one not yet entered
        // raises the watermark; another incarnation's, or an older request's, changes nothing.
        const process = () => {
          if (named.source.incarnation !== native.incarnation) return;
          const active = native.active;
          if (active && active.request === named.request && active.source.incarnation === named.source.incarnation && active.source.sequence === named.source.sequence) {
            native.record = null;
            native.ticket += 1;
            native.active = null;
          } else if (named.request > native.watermark) {
            native.watermark = named.request;
          }
        };
        if (native.holdRetires) native.heldRetires.push(process);
        else process();
        return Promise.resolve(null);
      }
      if (command === "print_rendered_report") {
        // Admission against the current record; the window and its hand-off are native.
        const { source } = ingressArgs([command, args, options]) as { source: Mark };
        if (!current(source) || native.record?.sequence !== source.sequence) {
          return Promise.reject({ code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        return Promise.resolve({ outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: SHA, print_sha256_hex: "c".repeat(64) });
      }
      if (command === "save_rendered_report_html") {
        const { html, source } = ingressArgs([command, args, options]) as { html: string; source: Mark };
        if (!current(source) || native.record?.sequence !== source.sequence) {
          return Promise.reject({ code: "REPORT-SOURCE-RETIRED", stage: "source", message: "The report's source changed.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        const admittedTicket = native.ticket;
        return new Promise((resolve, reject) => native.pendingSaves.push(() => {
          // The publication gate at the rename: the render and its source must still be current.
          if (native.record === null || native.ticket !== admittedTicket || !current(source)) {
            reject({ code: "REPORT-HTML-SAVE-RETIRED", stage: "currency", message: "The report was retired, rendered again or its source changed before the file was published: nothing was written to the destination.", destination: "as_it_was", cleanup_code: null, selected_basename: "invented-report.html" });
          } else {
            native.published.push(html);
            resolve(SAVED);
          }
        }));
      }
      return Promise.resolve(undefined);
    });
    return {
      native,
      source: new ReportSourceSequence(transport),
      outcomes: new ReportOutputOutcomes(),
      publishPendingSaves: () => native.pendingSaves.splice(0).forEach((publish) => publish()),
      deliverAdvances: () => native.heldAdvances.splice(0).forEach((deliver) => deliver()),
      deliverRenders: () => native.heldRenders.splice(0).forEach((deliver) => deliver()),
      deliverRetires: () => native.heldRetires.splice(0).forEach((deliver) => deliver())
    };
  }

  // One result object per run, as the session holds it: a re-render is not a source change.
  const results = new Map<string, ReturnType<typeof result>>();
  const stableResult = (run: string) => {
    if (!results.has(run)) results.set(run, result(run));
    return results.get(run)!;
  };

  /** The session's two owners as App holds them: the panel with its committed sequence, and the
   * outcome status outside it. */
  function Harness({ source, outcomes, sequence, run, showPanel = true, reportModel = model, findings = NO_LIBRARY_REF_FINDINGS }: {
    source: ReportSourceSequence; outcomes: ReportOutputOutcomes; sequence: number; run: string; showPanel?: boolean;
    reportModel?: PreviewModel; findings?: LibraryRefFindingsState;
  }) {
    const entries = useSyncExternalStore(outcomes.subscribe, outcomes.snapshot);
    return (
      <>
        {showPanel ? (
          <RenderedReportPanel model={reportModel} result={stableResult(run)} analysisRun={analysisRun} projectSummary={null}
            reportSource={source} reportSourceSequence={sequence} reportOutputs={outcomes} libraryRefFindingsState={findings} />
        ) : null}
        <ReportOutputStatus outcomes={outcomes} entries={entries} currentSequence={sequence} />
      </>
    );
  }

  const flush = () => act(async () => { await new Promise((resolve) => setTimeout(resolve, 0)); });

  async function renderedLocalPrivate(context: ReturnType<typeof modelledNative>) {
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" />);
    await flush();
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    // The panel's context change advanced the sequence; the session commits its number.
    view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" />);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    return view;
  }

  /** The session replaces the report's source (a new solve): its sequenced setter advances. */
  function replaceSource(context: ReturnType<typeof modelledNative>, view: ReturnType<typeof render>, run: string, showPanel = true) {
    act(() => {
      context.source.advance();
      view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run={run} showPanel={showPanel} />);
    });
  }

  it("refuses publication of the earlier report when the source change is registered first", async () => {
    const context = modelledNative();
    const view = await renderedLocalPrivate(context);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves, JSON.stringify(context.outcomes.snapshot())).toHaveLength(1));
    replaceSource(context, view, "run:invented-2");
    await flush(); // the advance is delivered and registered
    context.publishPendingSaves();
    await flush();
    expect(context.native.published).toEqual([]);
    const [entry] = context.outcomes.snapshot();
    expect([entry.state, entry.code, entry.basename]).toEqual(["refused", "REPORT-HTML-SAVE-RETIRED", "invented-report.html"]);
    const text = screen.getByTestId(`report-output-${entry.id}-text`).textContent ?? "";
    expect(text).toContain("Not saved (REPORT-HTML-SAVE-RETIRED)");
    expect(text).toContain("The destination was left as it was.");
    expect(text).not.toContain("Saved the selected report");
  });

  it("lets an in-transit source change leave the earlier selected report saved, and says so truthfully", async () => {
    const context = modelledNative();
    const view = await renderedLocalPrivate(context);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves, JSON.stringify(context.outcomes.snapshot())).toHaveLength(1));
    context.native.holdAdvances = true;
    replaceSource(context, view, "run:invented-2");
    // The native side publishes before the advance is registered.
    context.publishPendingSaves();
    await flush();
    context.deliverAdvances();
    await flush();
    expect(context.native.published).toEqual([HTML]);
    const [entry] = context.outcomes.snapshot();
    expect(entry.state).toBe("saved");
    const status = () => screen.getByTestId(`report-output-${entry.id}`);
    expect(status()).toHaveAttribute("data-workspace-changed", "true");
    const text = () => screen.getByTestId(`report-output-${entry.id}-text`).textContent ?? "";
    expect(text()).toContain("Saved the selected report. The workspace has changed since it was prepared.");
    expect(text()).toContain("File: invented-report.html.");
    expect(text()).toContain("Report basis: model project:invented, run run:invented-1.");
    // The panel shows neither the stale render nor its receipt as the current report's.
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    expect(screen.queryByText(/^Saved invented-report/)).not.toBeInTheDocument();
    // The outcome survives the panel's unmount (Report hidden or replaced) and a project switch.
    replaceSource(context, view, "run:invented-3", false);
    expect(screen.queryByTestId("rendered-report-panel")).not.toBeInTheDocument();
    expect(text()).toContain("Saved the selected report. The workspace has changed since it was prepared.");
    fireEvent.click(screen.getByTestId(`report-output-${entry.id}-dismiss`));
    expect(screen.queryByTestId(`report-output-${entry.id}`)).not.toBeInTheDocument();
  });

  it("refuses at entry a render request delivered after the source advance it predates", async () => {
    const context = modelledNative();
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" />);
    await flush();
    context.native.holdRenders = true;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(context.native.heldRenders).toHaveLength(1));
    replaceSource(context, view, "run:invented-2");
    await flush(); // registered
    context.deliverRenders(); // the old request reaches the native side only now
    await flush();
    expect(context.native.record).toBeNull();
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
  });

  it("retires a pending render natively when the source advances before any response is accepted", async () => {
    const context = modelledNative();
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" />);
    await flush();
    nativeRender.autoComplete = false;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(nativeRender.complete).toBeTypeOf("function"));
    replaceSource(context, view, "run:invented-2");
    await flush(); // the advance is registered; the pending ticket is superseded
    nativeRender.complete?.();
    await flush();
    expect({ advanceSent: context.native.advancesSent > 0, nativeHoldsTheEarlierRender: context.native.record !== null })
      .toEqual({ advanceSent: true, nativeHoldsTheEarlierRender: false });
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
  });

  it("withholds output, not editing, while the source cannot be registered, and resumes once it can", async () => {
    const context = modelledNative();
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" />);
    await flush();
    context.native.failAdvances = true;
    // Editing proceeds: the advances neither wait nor throw.
    replaceSource(context, view, "run:invented-2");
    replaceSource(context, view, "run:invented-3");
    expect(context.source.sequence).toBe(2);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-error")).toHaveTextContent("REPORT-SOURCE-UNREGISTERED"));
    expect(context.native.record).toBeNull();
    context.native.failAdvances = false;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    expect(context.native.record).toEqual({ incarnation: 1, sequence: 2 });
  });

  it("settles a save in the session's outcome after its panel unmounts", async () => {
    const context = modelledNative();
    const view = await renderedLocalPrivate(context);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves, JSON.stringify(context.outcomes.snapshot())).toHaveLength(1));
    const [pending] = context.outcomes.snapshot();
    expect(pending.state).toBe("pending");
    view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" showPanel={false} />);
    expect(context.native.retiresSent).toBe(1); // the unmounted panel retired its held render
    context.publishPendingSaves(); // refused at the gate: the render was retired before publication
    await flush();
    const [entry] = context.outcomes.snapshot();
    expect([entry.state, entry.code]).toEqual(["refused", "REPORT-HTML-SAVE-RETIRED"]);
    expect(screen.getByTestId(`report-output-${entry.id}-text`)).toHaveTextContent("Not saved (REPORT-HTML-SAVE-RETIRED)");
  });

  // M06 and M04 (round-three response): each render request carries its owner's ordinal, and the
  // panel retires only the exact requests it issued. Expected outcomes are the settled design's.

  /** A panel with the local-private context selected, before any render. */
  async function readyPanel(context: ReturnType<typeof modelledNative>) {
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" />);
    await flush();
    return view;
  }
  const hide = (context: ReturnType<typeof modelledNative>, view: ReturnType<typeof render>) =>
    view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" showPanel={false} />);
  const show = (context: ReturnType<typeof modelledNative>, view: ReturnType<typeof render>) =>
    view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" />);

  it("sends nothing and retires nothing when the panel unmounts during registration", async () => {
    const context = modelledNative();
    context.native.holdAdvances = true;
    const view = await readyPanel(context);
    replaceSource(context, view, "run:invented-2"); // the number to register is held in transit
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await flush();
    hide(context, view);
    context.deliverAdvances();
    await flush();
    expect({ entered: context.native.entered, retires: context.native.retirements, record: context.native.record })
      .toEqual({ entered: [], retires: [], record: null });
  });

  it("retires exactly its issued request on unmount, so a late entry of it is refused", async () => {
    const context = modelledNative();
    const view = await readyPanel(context);
    context.native.holdRenders = true;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(context.native.heldRenders).toHaveLength(1));
    hide(context, view);
    expect(context.native.retirements).toEqual([{ source: { incarnation: 1, sequence: 0 }, request: 1 }]);
    context.deliverRenders(); // the issued command reaches native entry only now
    await flush();
    expect({ entered: context.native.entered, record: context.native.record, retires: context.native.retiresSent })
      .toEqual({ entered: [], record: null, retires: 1 });
  });

  it("retires an entered but unanswered request on unmount, so its completion is refused", async () => {
    const context = modelledNative();
    const view = await readyPanel(context);
    nativeRender.autoComplete = false;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(nativeRender.complete).toBeTypeOf("function"));
    hide(context, view);
    nativeRender.complete?.();
    await flush();
    expect({ entered: context.native.entered, record: context.native.record, retires: context.native.retirements })
      .toEqual({ entered: [1], record: null, retires: [{ source: { incarnation: 1, sequence: 0 }, request: 1 }] });
  });

  it("keeps a replacement panel's render when the old panel's retirement is processed late", async () => {
    const context = modelledNative();
    const view = await readyPanel(context);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    context.native.holdRetires = true;
    hide(context, view); // the old panel's retirement of request 1 is still queued
    show(context, view);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    context.deliverRetires();
    await flush();
    expect({ entered: context.native.entered, active: context.native.active?.request, record: context.native.record })
      .toEqual({ entered: [1, 2], active: 2, record: { incarnation: 1, sequence: 0 } });
    // Ordinary output stays available on the replacement's render.
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent(`Print window opened with the derived print view of SHA-256 ${SHA}`));
  });

  it("retires at once a request whose render arrives after the panel discarded it", async () => {
    const context = modelledNative();
    const view = await readyPanel(context);
    nativeRender.autoComplete = false;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(nativeRender.complete).toBeTypeOf("function"));
    // The panel discards it while it is outstanding: the composition (panel-local) changes
    // with its advance held in transit, so the native side still holds the request.
    context.native.holdAdvances = true;
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    nativeRender.complete?.(); // completes natively: the request becomes the current record
    await flush();
    expect({ record: context.native.record, active: context.native.active, retires: context.native.retirements })
      .toEqual({ record: null, active: null, retires: [{ source: { incarnation: 1, sequence: 0 }, request: 1 }] });
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
  });

  // R4-F1: clearing a preview is not evidence that native custody was replaced.
  it.each(["assembly failure", "blocked assembly", "pre-entry refusal"] as const)(
    "retires the held report on unmount after a replacement ends in %s",
    async (failure) => {
      const context = modelledNative();
      const view = await renderedLocalPrivate(context);
      fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
      await waitFor(() => expect(context.native.pendingSaves).toHaveLength(1));
      const held = context.native.active;
      if (failure === "pre-entry refusal") {
        nativeRender.current!.enter = async () => { throw new Error("INVENTED-PRE-ENTRY-REFUSAL"); };
      } else if (failure === "assembly failure") assemblyControl.failure = true;
      else assemblyControl.blocked = true;
      fireEvent.click(screen.getByTestId("rendered-report-render"));
      await waitFor(() => expect(screen.getByTestId("rendered-report-render")).toBeEnabled());
      expect(context.native.entered).toEqual([1]);
      expect(context.native.active).toEqual(held);
      hide(context, view);
      await flush();
      expect(context.native.retirements).toContainEqual(held);
      context.publishPendingSaves();
      await flush();
      expect(context.native.record).toBeNull();
      expect(context.native.published).toEqual([]);
      expect(context.outcomes.snapshot()[0].state).toBe("refused");
    }
  );

  it("retires the held report when unmounted during replacement preparation", async () => {
    const context = modelledNative();
    const view = await renderedLocalPrivate(context);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves).toHaveLength(1));
    const held = context.native.active;
    let finishPreparation!: () => void;
    assemblyControl.pending = new Promise<void>((resolve) => { finishPreparation = resolve; });
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    expect(screen.getByTestId("rendered-report-render")).toBeDisabled();
    hide(context, view);
    finishPreparation();
    await flush();
    expect(context.native.entered).toEqual([1]);
    expect(context.native.retirements).toEqual([held]);
    context.publishPendingSaves();
    await flush();
    expect(context.native.published).toEqual([]);
    expect(context.outcomes.snapshot()[0].state).toBe("refused");
  });

  it("an accepted replacement retires the earlier save and becomes the request retired on unmount", async () => {
    const context = modelledNative();
    const view = await renderedLocalPrivate(context);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves).toHaveLength(1));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    expect(context.native.entered).toEqual([1, 2]);
    context.publishPendingSaves();
    await flush();
    expect(context.native.published).toEqual([]);
    expect(context.outcomes.snapshot()[0].state).toBe("refused");
    const replacement = context.native.active;
    hide(context, view);
    expect(context.native.retirements).toEqual([replacement]);
    expect(context.native.record).toBeNull();
  });

  // Round-two F1: a library-reference findings publication is a source change of the report.
  // The session's bridge advances the source and commits the new publication with it; here
  // that step is stood in for by `advance()` beside the new publication prop, as for the
  // other sources (the bridge itself is tested on the actual session in renderedReport.test).
  const refModel = {
    project: { id: "project:invented", units: {} },
    materials: [{ id: "material:with-ref", label: "Invented referenced steel", provenance: "", provenance_record: {
      source_name: "Invented datasheet", library_ref: { library_kind: "material", library_id: "invented-library", library_version: "invented-v1", record_id: "steel-a" }
    } }]
  } as unknown as PreviewModel;
  const refFinding = {
    entity_ref: "material:with-ref", library_kind: "material" as const, library_id: "invented-library", library_version: "invented-v1",
    record_id: "steel-a", code: "MODEL-LIBRARY-REF-VERSION-CHANGED" as const, severity: "warning" as const,
    recorded_version: "invented-v1", stored_version: "invented-v2", message: "Invented library version changed."
  };
  const withFinding: LibraryRefFindingsState = { projectId: "project:invented", findings: [refFinding] };

  async function renderedWithFindings(context: ReturnType<typeof modelledNative>) {
    context.source.start();
    const view = render(<Harness source={context.source} outcomes={context.outcomes} sequence={0} run="run:invented-1" reportModel={refModel} />);
    await flush();
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" reportModel={refModel} />);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(context.native.pendingSaves).toHaveLength(1));
    return view;
  }

  function publishFinding(context: ReturnType<typeof modelledNative>, view: ReturnType<typeof render>) {
    act(() => {
      context.source.advance();
      view.rerender(<Harness source={context.source} outcomes={context.outcomes} sequence={context.source.sequence} run="run:invented-1" reportModel={refModel} findings={withFinding} />);
    });
  }

  it("refuses publication of a report prepared before a registered findings change, and discards its render", async () => {
    const context = modelledNative();
    const view = await renderedWithFindings(context);
    publishFinding(context, view);
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    await flush(); // registered
    context.publishPendingSaves();
    await flush();
    expect(context.native.published).toEqual([]);
    const [entry] = context.outcomes.snapshot();
    expect([entry.state, entry.code]).toEqual(["refused", "REPORT-HTML-SAVE-RETIRED"]);
  });

  it("lets a findings change still in transit leave the earlier selected report saved, with the change stated", async () => {
    const context = modelledNative();
    const view = await renderedWithFindings(context);
    context.native.holdAdvances = true;
    publishFinding(context, view);
    context.publishPendingSaves();
    await flush();
    context.deliverAdvances();
    await flush();
    expect(context.native.published).toEqual([HTML]);
    const [entry] = context.outcomes.snapshot();
    expect(entry.state).toBe("saved");
    expect(screen.getByTestId(`report-output-${entry.id}-text`).textContent).toContain("Saved the selected report. The workspace has changed since it was prepared.");
  });

  it("prints the same report again, again after a refusal, and after the report changes", async () => {
    const receipt = { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: SHA, print_sha256_hex: "c".repeat(64) };
    const answers: Array<"ok" | "refused"> = ["ok", "refused", "ok", "ok"];
    invokeMock.mockImplementation(async (command: string) => {
      if (command !== "print_rendered_report") return undefined;
      if (answers.shift() === "refused") {
        throw { code: "REPORT-PRINT-ALREADY-OPEN", stage: "print", message: "A print window is already open.", destination: "untouched", cleanup_code: null, selected_basename: null };
      }
      return receipt;
    });
    const view = await renderShown();
    const press = async (calls: number, expected: string) => {
      await waitFor(() => expect(screen.getByTestId("rendered-report-print")).toBeEnabled());
      fireEvent.click(screen.getByTestId("rendered-report-print"));
      await waitFor(() => expect(routeCalls("print_rendered_report")).toHaveLength(calls));
      await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent(expected));
    };
    await press(1, "Print window opened");
    await press(2, "Not printed (REPORT-PRINT-ALREADY-OPEN)");
    await press(3, "Print window opened");
    view.rerender(panel(result("run:invented-2")));
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print")).toBeInTheDocument());
    expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent(/^$/);
    await press(4, "Print window opened");
  });
});

describe("browser runtime (no native shell)", () => {
  it("keeps the controlled download link and the print iframe and calls no native route", async () => {
    await renderShown(true);
    const save = screen.getByTestId("rendered-report-save-local-private");
    expect(save.tagName).toBe("A");
    expect(save).toHaveAttribute("href", `data:text/html;charset=utf-8,${encodeURIComponent(HTML)}`);
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    expect(screen.getByTestId("rendered-report-print-frame")).toHaveAttribute("srcdoc", outcome().derived_print_html);
    expect(screen.getByTestId("rendered-report-list-notice")).toHaveTextContent("HTML saves and printed reports are not listed in the project.");
    expect(screen.queryByTestId("rendered-report-save-status")).not.toBeInTheDocument();
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("prints from a sandboxed frame, again after the print ends, on a second press, and after the report changes", async () => {
    const view = await renderShown();
    /** Presses Print, answers the frame's load and returns the frame and its print spy. */
    const press = () => {
      fireEvent.click(screen.getByTestId("rendered-report-print"));
      const frame = screen.getByTestId("rendered-report-print-frame") as HTMLIFrameElement;
      expect(frame).toHaveAttribute("sandbox", "allow-same-origin allow-modals");
      expect(frame.getAttribute("sandbox")).not.toContain("allow-scripts");
      const print = vi.fn();
      Object.defineProperty(frame.contentWindow!, "print", { value: print, configurable: true });
      fireEvent.load(frame);
      expect(print).toHaveBeenCalled();
      return frame;
    };
    const first = press();
    // The print ends: the frame is cleared, and the next press mounts a new one.
    first.contentWindow!.dispatchEvent(new Event("afterprint"));
    await waitFor(() => expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument());
    const second = press();
    expect(second).not.toBe(first);
    // A press while a frame is still mounted (no afterprint seen) mounts a fresh frame.
    const third = press();
    expect(third).not.toBe(second);
    // A changed report discards the frame; its own render prints afresh.
    view.rerender(panel(result("run:invented-2")));
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print")).toBeInTheDocument());
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
    press();
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("keeps the public save a withheld span, unchanged", async () => {
    await renderShown();
    const save = screen.getByTestId("rendered-report-save");
    expect(save.tagName).toBe("SPAN");
    expect(save).toHaveAttribute("aria-disabled", "true");
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
