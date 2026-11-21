// DEL-08-01 REQ-022: the report panel's Save canonical HTML and Print in the
// desktop runtime go to the native commands (`renderedReportRoutes.ts`), name
// the render by its SHA-256 (the desktop keeps the document; no HTML is sent),
// carry the controlled export's evidence, and show the
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
// The composer renders one invented control, so a test can make the person's composition change.
vi.mock("./ReportComposer", async () => {
  const { createElement } = await import("react");
  return {
    ReportComposer: ({ onChange }: { onChange: (value: unknown) => void }) =>
      createElement("button", { type: "button", "data-testid": "invented-composition-change", onClick: () => onChange({ invented: true }) }, "compose")
  };
});
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
import { reportViewSourceRefusalText } from "../../services/renderedReportRoutes";
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
    // DEL-08-01 REQ-023 Coverage: where an HTML save or a print is offered, the panel says, in exactly these
    // words, that the project keeps and lists no saved or printed report, and that an HTML save and a saved
    // report package are each a file the person keeps (REQ-023's list is not built).
    expect(screen.getByTestId("rendered-report-list-notice").textContent).toBe(
      "The project does not keep or list saved or printed reports. An HTML save and a saved report package are each a file you keep."
    );

    fireEvent.click(save);
    await waitFor(() => expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-report.html"));
    expect(screen.getByTestId("rendered-report-save-status")).toHaveAttribute("role", "status");
    expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent(`SHA-256 ${SHA}`);
    expect(routeCalls("save_rendered_report_html").map((args) => "html" in args)).toEqual([false]);
    expect(routeCalls("save_rendered_report_html")).toEqual([
      {
        sha256Hex: SHA,
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
    expect(routeCalls("print_rendered_report").map((args) => "html" in args)).toEqual([false]);
    expect(routeCalls("print_rendered_report")).toEqual([
      {
        sha256Hex: SHA,
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
  /** The native refusal of a request naming a render other than the one held, exactly as
   * `rendered_report_routes.rs` writes it. */
  const UNBOUND_MESSAGE =
    "The render the request names is not the current one: the SHA-256 it names is not the one the desktop recorded for the current render. Render the report again.";
  const unbound = () => ({ code: "REPORT-ROUTE-HTML-UNBOUND", stage: "admission", message: UNBOUND_MESSAGE, destination: "untouched", cleanup_code: null, selected_basename: null });
  function modelledNative() {
    const native = {
      incarnation: 0, registered: 0, ticket: 0, record: null as null | Mark,
      active: null as null | { source: Mark; request: number }, watermark: 0,
      retirements: [] as Array<{ source: Mark; request: number }>, holdRetires: false, heldRetires: [] as Array<() => void>,
      entered: [] as number[],
      held: null as null | { html: string; sha256Hex: string }, published: [] as string[], advancesSent: 0, retiresSent: 0, holdAdvances: false, failAdvances: false,
      heldAdvances: [] as Array<() => void>, pendingSaves: [] as Array<() => void>, heldRenders: [] as Array<() => void>, holdRenders: false,
      // `report_view.rs`: the hashes of the open view windows of the current render, and the
      // pending closure notices (kinds) its sweep and withdrawal record.
      views: [] as string[], closures: [] as string[]
    };
    // The sweep beside every retirement and replacement: a view of a render no longer current closes.
    const sweep = () => {
      if (native.views.length === 0) return;
      native.views = [];
      native.closures.push("current_render");
    };
    const register = (mark: Mark): Mark => {
      if (mark.incarnation !== native.incarnation) throw { code: "REPORT-SOURCE-INCARNATION-RETIRED", stage: "source", message: "retired" };
      if (mark.sequence > native.registered) {
        native.registered = mark.sequence;
        native.record = null;
        native.held = null;
        native.ticket += 1;
        native.active = null;
        sweep();
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
        native.held = null;
        native.ticket += 1;
        native.active = null;
        native.watermark = 0;
        sweep();
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
        native.held = null;
        native.active = { source, request };
        sweep();
        const ticket = native.ticket;
        nativeRender.pending = () => {
          if (ticket !== native.ticket || !current(source)) throw new Error("REPORT-RENDER-SUPERSEDED: not recorded.");
          native.record = source;
          // The desktop keeps the recorded render's canonical HTML and its hash; a save
          // publishes exactly it.
          native.held = outcomeRef.current ? { html: outcomeRef.current.html as string, sha256Hex: outcomeRef.current.sha256_hex as string } : null;
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
            native.held = null;
            native.ticket += 1;
            native.active = null;
            sweep();
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
        const named = ingressArgs([command, args, options]) as { sha256Hex: string; source: Mark };
        const { source } = named;
        if ("html" in named) return Promise.reject(new Error("the print request carries no document"));
        if (!current(source) || native.record?.sequence !== source.sequence) {
          return Promise.reject({ code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        if (named.sha256Hex !== native.held?.sha256Hex) return Promise.reject(unbound());
        return Promise.resolve({ outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: SHA, print_sha256_hex: "c".repeat(64) });
      }
      if (command === "open_rendered_report_view") {
        // The view's admission (`RenderCustody::admit_view`): no context, intent or evidence.
        const named = ingressArgs([command, args, options]) as { sha256Hex: string; source: Mark };
        if ("html" in named) return Promise.reject(new Error("the view request carries no document"));
        if (!current(named.source) || native.record?.sequence !== named.source.sequence) {
          return Promise.reject({ code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        if (named.sha256Hex !== native.held?.sha256Hex) return Promise.reject(unbound());
        const open = native.views.includes(named.sha256Hex);
        if (!open) native.views.push(named.sha256Hex);
        return Promise.resolve({ outcome: open ? "brought_forward" : "opened", kind: "current_render", sha256_hex: named.sha256Hex });
      }
      if (command === "close_rendered_report_views") {
        sweep();
        return Promise.resolve(null);
      }
      if (command === "take_report_view_closures") {
        return Promise.resolve(native.closures.splice(0).map((kind) => ({ kind, reason: "no_longer_showable" })));
      }
      if (command === "save_rendered_report_html") {
        const named = ingressArgs([command, args, options]) as { sha256Hex: string; source: Mark };
        const { source } = named;
        if ("html" in named) return Promise.reject(new Error("the save request carries no document"));
        if (!current(source) || native.record?.sequence !== source.sequence) {
          return Promise.reject({ code: "REPORT-SOURCE-RETIRED", stage: "source", message: "The report's source changed.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        if (named.sha256Hex !== native.held?.sha256Hex) return Promise.reject(unbound());
        const html = native.held.html;
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

  it("refuses a save and a print naming a render the desktop no longer holds, and says so", async () => {
    const context = modelledNative();
    await renderedLocalPrivate(context);
    // Without the panel knowing, the native record now holds another document under the same
    // source and incarnation.
    const held = context.native.record;
    context.native.held = { html: "<!DOCTYPE html>\n<html><body>another invented render</body></html>", sha256Hex: "d".repeat(64) };
    expect(context.native.record).toBe(held);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-save-status").textContent).toBe(`Not saved (REPORT-ROUTE-HTML-UNBOUND): ${UNBOUND_MESSAGE}`));
    expect(screen.getByTestId("rendered-report-save-status")).not.toHaveTextContent("left as it was");
    await waitFor(() => expect(screen.getByTestId("rendered-report-print")).toBeEnabled());
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status").textContent).toBe(`Not printed (REPORT-ROUTE-HTML-UNBOUND): ${UNBOUND_MESSAGE}`));
    expect(context.native.published).toEqual([]);
    expect(context.native.pendingSaves).toEqual([]);
    expect(routeCalls("save_rendered_report_html").map((args) => args.sha256Hex)).toEqual([SHA]);
    expect(routeCalls("print_rendered_report").map((args) => args.sha256Hex)).toEqual([SHA]);
    expect(context.outcomes.snapshot().map((entry) => [entry.kind, entry.state, entry.code])).toEqual([
      ["save", "refused", "REPORT-ROUTE-HTML-UNBOUND"],
      ["print", "refused", "REPORT-ROUTE-HTML-UNBOUND"]
    ]);
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

// Open in a window (DEL-08-01 REQ-022; `report_view.rs`), against the modelled native side.
describe("open in a window", () => {
  beforeEach(() => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  });
  const CLOSED_LINE = "The report window closed because the result it showed is no longer current.";
  const flush = () => act(async () => { await new Promise((resolve) => setTimeout(resolve, 0)); });
  type Mark = { incarnation: number; sequence: number };
  /** The modelled native side of the currency tests above, as far as the view needs it. */
  function modelledView() {
    const native = {
      incarnation: 0, registered: 0, record: null as null | Mark, held: null as null | string, active: null as null | number,
      views: [] as string[], closures: [] as string[], holdAdvances: false, heldAdvances: [] as Array<() => void>, entered: 0,
      // `close_rendered_report_views`: the reasons it was called with (a late close, which withdraws
      // nothing from view, as `late:<reason>`), and withdrawals held in transit.
      withdrawals: [] as string[], holdWithdrawals: false, heldWithdrawals: [] as Array<() => void>,
      // The render's generation (one per completion) and the one withdrawn from view.
      generation: 0, withdrawnGeneration: -1,
      // `open_rendered_report_view`: the next open's reply is lost, the opening itself made when the
      // reply is let go ("held") or at once ("opened").
      loseNextOpen: false as false | "held" | "opened", heldOpens: [] as Array<() => void>
    };
    const sweep = () => {
      if (native.views.length === 0) return;
      native.views = [];
      native.closures.push("current_render");
    };
    const register = (mark: Mark): Mark => {
      if (mark.sequence > native.registered) {
        native.registered = mark.sequence;
        native.record = null;
        native.held = null;
        native.active = null;
        sweep();
      }
      return { incarnation: native.incarnation, sequence: native.registered };
    };
    const transport = {
      observe: async (): Promise<Mark> => ({ incarnation: native.incarnation, sequence: native.registered }),
      activate: async (): Promise<Mark> => {
        native.incarnation += 1;
        native.registered = 0;
        return { incarnation: native.incarnation, sequence: 0 };
      },
      advance: (mark: Mark): Promise<Mark> => {
        if (!native.holdAdvances) return Promise.resolve().then(() => register(mark));
        return new Promise<Mark>((resolve) => native.heldAdvances.push(() => resolve(register(mark))));
      }
    };
    nativeRender.current = {
      enter: async (source, request) => {
        native.entered += 1;
        native.record = null;
        native.held = null;
        native.active = request;
        sweep();
        nativeRender.pending = () => {
          native.record = source;
          native.held = SHA;
          native.generation += 1;
        };
      },
      completion: async () => { nativeRender.pending?.(); }
    };
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (command === "retire_rendered_report") {
        if ((args as { request: number }).request === native.active) {
          native.record = null;
          native.held = null;
          native.active = null;
          sweep();
        }
        return Promise.resolve(null);
      }
      if (command === "open_rendered_report_view") {
        const named = ingressArgs([command, args, options]) as { sha256Hex: string; source: Mark };
        if ("html" in named) return Promise.reject(new Error("the view request carries no document"));
        if (native.record === null || named.source.sequence !== native.registered) {
          return Promise.reject({ code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current: it was retired or replaced. Render the report again.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        if (named.sha256Hex !== native.held) {
          return Promise.reject({ code: "REPORT-ROUTE-HTML-UNBOUND", stage: "admission", message: "The render the request names is not the current one.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        if (native.withdrawnGeneration === native.generation) {
          return Promise.reject({ code: "REPORT-VIEW-NOT-SHOWABLE", stage: "view", message: "This report is no longer the current one. No window was opened.", destination: "untouched", cleanup_code: null, selected_basename: null });
        }
        const open = native.views.includes(named.sha256Hex);
        if (!open) native.views.push(named.sha256Hex);
        return Promise.resolve({ outcome: open ? "brought_forward" : "opened", kind: "current_render", sha256_hex: named.sha256Hex });
      }
      if (command === "close_rendered_report_views") {
        // `close_kind_in`: a withdrawal marks the current render as withdrawn from view; a source
        // change records a notice when it closed a window; the person's own act records none and
        // clears any pending one of the kind.
        const { reason, withdraw } = args as { reason: string; withdraw: boolean };
        native.withdrawals.push(withdraw ? reason : `late:${reason}`);
        const process = () => {
          if (withdraw && native.record !== null) native.withdrawnGeneration = native.generation;
          if (reason === "source_changed") sweep();
          else if (reason === "own_act") {
            native.views = [];
            native.closures = native.closures.filter((kind) => kind !== "current_render");
          }
        };
        if (!native.holdWithdrawals) {
          process();
          return Promise.resolve(null);
        }
        return new Promise((resolve) => native.heldWithdrawals.push(() => { process(); resolve(null); }));
      }
      if (command === "take_report_view_closures") {
        return Promise.resolve(native.closures.splice(0).map((kind) => ({ kind, reason: "no_longer_showable" })));
      }
      return Promise.resolve(undefined);
    });
    const opening = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (command !== "open_rendered_report_view" || !native.loseNextOpen) return opening(command, args, options);
      const when = native.loseNextOpen;
      native.loseNextOpen = false;
      // The opening is made natively (at once, or when the reply is let go), but its reply is lost.
      const made = when === "opened" ? opening(command, args, options).catch(() => undefined) : null;
      return new Promise((_resolve, reject) => native.heldOpens.push(() => {
        if (made === null) void opening(command, args, options).catch(() => undefined);
        reject(new Error("invented lost reply"));
      }));
    });
    const source = new ReportSourceSequence(transport);
    const outcomes = new ReportOutputOutcomes();
    return {
      native, source, outcomes, sweep,
      deliverAdvances: () => native.heldAdvances.splice(0).forEach((deliver) => deliver()),
      deliverWithdrawals: () => native.heldWithdrawals.splice(0).forEach((deliver) => deliver()),
      deliverOpens: () => native.heldOpens.splice(0).forEach((deliver) => deliver())
    };
  }
  type Context = ReturnType<typeof modelledView>;
  const results = new Map<string, ReturnType<typeof result>>();
  const stable = (run: string) => {
    if (!results.has(run)) results.set(run, result(run));
    return results.get(run)!;
  };
  function Harness({ context, sequence, run, showPanel = true }: { context: Context; sequence: number; run: string; showPanel?: boolean }) {
    return showPanel ? (
      <RenderedReportPanel model={model} result={stable(run)} analysisRun={analysisRun} projectSummary={null}
        reportSource={context.source} reportSourceSequence={sequence} reportOutputs={context.outcomes} />
    ) : null;
  }
  const harness = (context: Context, run = "run:invented-1", showPanel = true) =>
    <Harness context={context} sequence={context.source.sequence} run={run} showPanel={showPanel} />;
  async function rendered(context: Context) {
    context.source.start();
    const view = render(harness(context));
    await flush();
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    return view;
  }
  async function openWindow() {
    await waitFor(() => expect(screen.getByTestId("rendered-report-open-window")).toBeEnabled());
    fireEvent.click(screen.getByTestId("rendered-report-open-window"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-status")).not.toHaveTextContent("Opening"));
  }
  const replaceSource = (context: Context, view: ReturnType<typeof render>, run: string, showPanel = true) => {
    act(() => {
      context.source.advance();
      view.rerender(harness(context, run, showPanel));
    });
  };

  // Must stay the first test that opens a window in this file: it is the session's first open.
  // Lane 4's gap and F4: the opening is outstanding when the source changes; the drop's
  // withdrawal is sent (the flag is set as the opening is sent) and withdraws the render from view
  // natively, so the opening, made after it, opens nothing; the late close follows.
  it("withdraws the render on the session's first open when the source changes before its lost reply, so nothing opens", async () => {
    const context = modelledView();
    const view = await rendered(context);
    const viewCalls = () => invokeMock.mock.calls.map(([command]) => String(command)).filter((command) => command.includes("view"));
    expect(viewCalls()).toEqual([]); // no opening has been sent in this session: the panel asked nothing
    context.native.loseNextOpen = "held";
    context.native.holdAdvances = true;
    fireEvent.click(screen.getByTestId("rendered-report-open-window"));
    await waitFor(() => expect(context.native.heldOpens).toHaveLength(1));
    replaceSource(context, view, "run:invented-2");
    await flush();
    expect(context.native.withdrawals).toEqual(["source_changed"]); // sent while the first opening is outstanding
    expect(context.native.record).not.toBeNull(); // the advance is in transit: the render is still current natively
    context.deliverOpens();
    await flush();
    await flush();
    expect(context.native.withdrawals).toEqual(["source_changed", "late:source_changed"]);
    expect(context.native.views).toEqual([]);
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument(); // no window closed: no line
    context.deliverAdvances();
    await flush();
  });

  it("opens the shown render by its hash and source, sends no document and writes no output outcome", async () => {
    const context = modelledView();
    await rendered(context);
    const button = screen.getByTestId("rendered-report-open-window");
    expect(button.tagName).toBe("BUTTON");
    expect(button).toHaveAttribute("type", "button");
    expect(button).toHaveTextContent(/^Open in a window$/);
    await openWindow();
    expect(screen.getByTestId("rendered-report-view-status").textContent).toBe("Report opened in a window.");
    expect(screen.getByTestId("rendered-report-view-status")).toHaveAttribute("role", "status");
    expect(routeCalls("open_rendered_report_view")).toEqual([{ sha256Hex: SHA, source: { incarnation: 1, sequence: 0 } }]);
    expect(context.native.views).toEqual([SHA]);
    expect(context.outcomes.snapshot()).toEqual([]);
    await openWindow();
    expect(screen.getByTestId("rendered-report-view-status").textContent).toBe("The report's window was brought forward.");
    expect(context.native.views).toEqual([SHA]);
    expect(context.outcomes.snapshot()).toEqual([]);
  });

  it("states a refusal with its message first and its code last", async () => {
    const context = modelledView();
    await rendered(context);
    context.native.held = "d".repeat(64);
    await openWindow();
    expect(screen.getByTestId("rendered-report-view-status").textContent).toBe(
      "The report could not be opened in a window: The render the request names is not the current one. (REPORT-ROUTE-HTML-UNBOUND)"
    );
    expect(context.native.views).toEqual([]);
    expect(context.outcomes.snapshot()).toEqual([]);
  });

  // B4 (a): the window is no more current than the panel's preview. Each event on which the panel
  // drops its preview closes the window, whether or not the native render is retired by it.
  // Case 1: the model or result changed. The line shows, whichever of the panel's withdrawal and
  // the native sweep closed the window, including while the change is still in transit.
  it.each(["registered source change", "source change in transit"] as const)(
    "closes the window on a %s, and says so",
    async (event) => {
      const context = modelledView();
      const view = await rendered(context);
      await openWindow();
      expect(context.native.views).toEqual([SHA]);
      context.native.withdrawals = []; // the setup's own Render withdrew silently before the window opened
      if (event === "source change in transit") context.native.holdAdvances = true;
      replaceSource(context, view, "run:invented-2");
      await flush();
      await flush();
      expect(context.native.views).toEqual([]);
      expect(context.native.withdrawals).toEqual(["source_changed"]);
      if (event === "source change in transit") {
        expect(context.native.record).not.toBeNull(); // the native render is still current: only the window closed
      }
      await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toHaveTextContent(CLOSED_LINE));
      expect(screen.getByTestId("rendered-report-view-closed")).toHaveAttribute("role", "status");
      expect(context.outcomes.snapshot()).toEqual([]);
      context.deliverAdvances();
      await flush();
    }
  );

  // Cases 2 to 4: the person's own act (the composition, the local-private selection, Render)
  // closes the window and stays silent, whichever closed it, including when the desktop's sweep
  // recorded a notice before the panel's withdrawal arrived.
  it.each([
    ["composition", false], ["composition", true],
    ["local-private selection", false], ["local-private selection", true],
    ["new render", false], ["new render", true]
  ] as const)(
    "closes the window on the person's own %s and shows no line (the desktop recorded a notice first: %s)",
    async (act, noticeFirst) => {
      const context = modelledView();
      await rendered(context);
      await openWindow();
      expect(context.native.views).toEqual([SHA]);
      context.native.withdrawals = []; // the setup's own Render withdrew silently before the window opened
      if (noticeFirst) context.native.holdWithdrawals = true;
      const press = act === "composition" ? "invented-composition-change" : act === "local-private selection" ? "rendered-report-local-private-intent" : "rendered-report-render";
      fireEvent.click(screen.getByTestId(press));
      await flush();
      await flush();
      if (noticeFirst) {
        // A sweep closed the window first and the desktop recorded a notice (for the composition and
        // the local-private selection, the act's own source advance; a render's entry waits for
        // the withdrawal, so for Render it is a sweep for another cause); any take meanwhile shows
        // nothing.
        context.sweep();
        expect(context.native.views).toEqual([]);
        expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
        context.deliverWithdrawals();
        await flush();
        await flush();
      }
      expect(context.native.withdrawals).toEqual(["own_act"]);
      expect(context.native.views).toEqual([]);
      expect(context.native.closures).toEqual([]);
      expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    }
  );

  it("clears the line when a window is next opened", async () => {
    const context = modelledView();
    await rendered(context);
    await openWindow();
    // The desktop reports a closure while this render is shown (taken when the render returns).
    const enter = nativeRender.current!.enter;
    nativeRender.current!.enter = async (source, request) => {
      await enter(source, request);
      context.native.closures.push("current_render");
    };
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toBeInTheDocument());
    expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument();
    await openWindow();
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
  });

  it("says it is not known whether a window opened when the open's reply is lost", async () => {
    const context = modelledView();
    await rendered(context);
    context.native.loseNextOpen = "held";
    fireEvent.click(screen.getByTestId("rendered-report-open-window"));
    await waitFor(() => expect(context.native.heldOpens).toHaveLength(1));
    expect(screen.getByTestId("rendered-report-view-status")).toHaveTextContent("Opening the report window…");
    context.deliverOpens();
    await waitFor(() =>
      expect(screen.getByTestId("rendered-report-view-status")).toHaveTextContent("It is not known whether the report window opened: invented lost reply")
    );
    expect(context.native.views).toEqual([SHA]);
  });

  it("refuses a stale view in its own words, and any other source refusal as it came", () => {
    expect(reportViewSourceRefusalText("REPORT-SOURCE-STALE-VIEW: the workspace changed after this report was prepared; prepare it again.")).toBe(
      "The report could not be opened in a window: The workspace changed after this report was rendered. Render it again. (REPORT-SOURCE-STALE-VIEW)"
    );
    expect(reportViewSourceRefusalText("REPORT-SOURCE-UNREGISTERED: invented reason.")).toBe(
      "The report could not be opened in a window: invented reason. (REPORT-SOURCE-UNREGISTERED)"
    );
    expect(reportViewSourceRefusalText("invented uncoded reason")).toBe("The report could not be opened in a window: invented uncoded reason");
  });

  // Lane 4's repair: an open whose reply is lost may have opened a window. Here it opened before the
  // source changed: the drop's withdrawal closes it, and says so; the late close follows.
  it("closes a window whose open reply was lost when the source changed meanwhile, and says so", async () => {
    const context = modelledView();
    const view = await rendered(context);
    await openWindow();
    fireEvent.click(screen.getByTestId("rendered-report-render")); // closes it silently
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    await flush();
    context.native.withdrawals = [];
    context.native.loseNextOpen = "opened";
    context.native.holdAdvances = true;
    fireEvent.click(screen.getByTestId("rendered-report-open-window"));
    await waitFor(() => expect(context.native.heldOpens).toHaveLength(1));
    expect(context.native.views).toEqual([SHA]); // the window opened; its reply is outstanding
    replaceSource(context, view, "run:invented-2");
    await flush();
    expect(context.native.withdrawals).toEqual(["source_changed"]);
    expect(context.native.views).toEqual([]);
    context.deliverOpens(); // the reply is lost
    await flush();
    await flush();
    expect(context.native.withdrawals).toEqual(["source_changed", "late:source_changed"]);
    expect(context.native.views).toEqual([]);
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toHaveTextContent(CLOSED_LINE));
    context.deliverAdvances();
    await flush();
  });

  // F4, the reader's scenario: an opening is outstanding, the person presses Render, the new
  // render's preparation fails before it enters natively (so the earlier render stays current
  // there), and the opening's reply is lost. The Render's withdrawal was sent, so the opening opens
  // nothing, and nothing is said.
  it("withdraws the render when Render is pressed during an opening and the new render fails before entry", async () => {
    const context = modelledView();
    await rendered(context);
    context.native.withdrawals = [];
    context.native.loseNextOpen = "held";
    fireEvent.click(screen.getByTestId("rendered-report-open-window"));
    await waitFor(() => expect(context.native.heldOpens).toHaveLength(1));
    const entered = context.native.entered;
    assemblyControl.failure = true;
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await flush();
    await flush();
    expect(context.native.withdrawals).toEqual(["own_act"]);
    expect(context.native.entered).toBe(entered); // the new render never entered
    expect(context.native.record).not.toBeNull(); // the earlier render is still current natively
    context.deliverOpens();
    await flush();
    await flush();
    expect(context.native.views).toEqual([]);
    expect(context.native.withdrawals).toEqual(["own_act", "late:own_act"]);
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
  });

  it("closes the window with the render the panel retires on unmount, and says so when the Report page returns", async () => {
    const context = modelledView();
    const view = await rendered(context);
    await openWindow();
    view.rerender(harness(context, "run:invented-1", false));
    await flush();
    expect(context.native.views).toEqual([]);
    expect(context.native.closures).toEqual(["current_render"]);
    view.rerender(harness(context));
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toHaveTextContent(CLOSED_LINE));
    expect(context.native.closures).toEqual([]);
  });

  it("shows the line only on what the desktop returns, until dismissed or the next render", async () => {
    const context = modelledView();
    const view = await rendered(context);
    // No window open: the panel's own close request and its preview's drop report nothing.
    await openWindow();
    context.native.views = [];
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    view.rerender(harness(context));
    await flush();
    await flush();
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    view.rerender(harness(context));
    await flush();
    // A notice of another kind is not the current render's: no line.
    context.native.closures.push("saved_report");
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    await flush();
    expect(context.native.closures).toEqual([]);
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    // A window the desktop closed: the line, until dismissed; taken once, it does not return.
    await openWindow();
    replaceSource(context, view, "run:invented-2");
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toHaveTextContent(CLOSED_LINE));
    fireEvent.click(screen.getByTestId("rendered-report-view-closed-dismiss"));
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    await flush();
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    // Shown again after another closure, the next render clears it.
    await openWindow();
    replaceSource(context, view, "run:invented-3");
    await waitFor(() => expect(screen.getByTestId("rendered-report-view-closed")).toBeInTheDocument());
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
    await flush();
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    expect(context.outcomes.snapshot()).toEqual([]);
  });
});

describe("browser runtime (no native shell)", () => {
  it("offers no window and asks the desktop nothing about one", async () => {
    await renderShown();
    expect(screen.queryByTestId("rendered-report-open-window")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-view-closed")).not.toBeInTheDocument();
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toEqual(
      expect.arrayContaining(["open_rendered_report_view", "close_rendered_report_views", "take_report_view_closures"])
    );
    expect(invokeMock.mock.calls.filter(([command]) => String(command).includes("view"))).toEqual([]);
  });

  it("keeps the controlled download link and the print iframe and calls no native route", async () => {
    await renderShown(true);
    const save = screen.getByTestId("rendered-report-save-local-private");
    expect(save.tagName).toBe("A");
    expect(save).toHaveAttribute("href", `data:text/html;charset=utf-8,${encodeURIComponent(HTML)}`);
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    expect(screen.getByTestId("rendered-report-print-frame")).toHaveAttribute("srcdoc", outcome().derived_print_html);
    expect(screen.getByTestId("rendered-report-list-notice")).toHaveTextContent("The project does not keep or list saved or printed reports.");
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
