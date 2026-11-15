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
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { ingressArgs } from "../../test/nativeIngress";

const invokeMock = vi.hoisted(() => vi.fn());
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
  assembleRenderedReport: async ({ context }: { context: string }) => ({
    controlled: {
      payload: { report_context: { export_context: context } },
      decisions: [],
      findings: [],
      blocked: false,
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
  })
}));
/** A render request's native entry and completion, when a test models them (`nativeRender`);
 * otherwise the render answers at once. */
const nativeRender = vi.hoisted(() => ({ current: null as null | { enter: () => void; completion: () => Promise<void> } }));
vi.mock("../../services/reportRenderService", () => ({
  renderCalculationReport: async () => {
    if (nativeRender.current) {
      nativeRender.current.enter();
      await nativeRender.current.completion();
    }
    return { route: "tauri_renderer", outcome: outcomeRef.current };
  }
}));

import { RenderedReportPanel } from "./RenderedReportPanel";

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
  outcomeRef.current = outcome();
  nativeRender.current = null;
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
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "local_private", explicit_local_private_intent: true, blocked: false, blocking_count: 0 }
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
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "public_report", explicit_local_private_intent: false, blocked: false, blocking_count: 0 }
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

  it("retires the native render when its source is replaced, and shows no stale outcome", async () => {
    let resolveSave: (value: unknown) => void = () => undefined;
    invokeMock.mockImplementation((command: string) =>
      command === "save_rendered_report_html" ? new Promise((resolve) => { resolveSave = resolve; }) : Promise.resolve(undefined)
    );
    const view = await renderShown(true);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(routeCalls("save_rendered_report_html")).toHaveLength(1));
    view.rerender(panel(result("run:invented-2")));
    expect(invokeMock.mock.calls.filter(([command]) => command === "retire_rendered_report")).toHaveLength(1);
    expect(screen.queryByTestId("rendered-report-save-local-private")).not.toBeInTheDocument();
    resolveSave(SAVED);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.queryByText(/Saved invented-report/)).not.toBeInTheDocument();
  });

  // Currency under delayed delivery (DEL-08-01 REQ-022: "A report or package is produced only for
  // the result and model current when it is saved; a change during preparation retires the save";
  // AC-011: "A report prepared for a result that is replaced before the save completes is not
  // saved"). The real panel and route service run against mocked IPC whose native side models the
  // reviewed contract only: one custody record that a render request clears at entry and its
  // completion sets, that `retire_rendered_report` clears when it is delivered, and that the HTML
  // save consults when it publishes (the guarded check-through-rename the repair settles, R1).
  // Delivery of `retire_rendered_report` is held until the test releases it, as the IPC hop and
  // the native main thread may hold it. The expected values come from REQ-022, not from the
  // model; these fail on the reviewed protocol, and a repair that changes the protocol changes
  // the model with it.
  function modelledNative() {
    const native = { current: null as null | number, renders: 0, published: [] as string[], retiresSent: 0, retiresDelivered: 0, deliveredAtPublication: [] as number[] };
    const heldRetires: Array<() => void> = [];
    const pendingSaves: Array<(publish: boolean) => void> = [];
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (command === "retire_rendered_report") {
        native.retiresSent += 1;
        return new Promise<void>((resolve) => heldRetires.push(() => { native.current = null; native.retiresDelivered += 1; resolve(); }));
      }
      if (command === "save_rendered_report_html") {
        const { html } = ingressArgs([command, args, options]) as { html: string };
        return new Promise((resolve, reject) => pendingSaves.push(() => {
          native.deliveredAtPublication.push(native.retiresDelivered);
          if (native.current === null) reject({ code: "REPORT-HTML-SAVE-RETIRED", stage: "currency", message: "retired", destination: "untouched" });
          else { native.published.push(html); resolve(SAVED); }
        }));
      }
      return Promise.resolve(undefined);
    });
    return {
      native,
      /** The native save publishes now, before any held retirement is delivered. */
      publishPendingSaves: () => pendingSaves.splice(0).forEach((publish) => publish(true)),
      deliverRetires: () => heldRetires.splice(0).forEach((deliver) => deliver())
    };
  }

  it("orders a source change before a pending native HTML save's publication when retirement delivery is delayed", async () => {
    const { native, publishPendingSaves, deliverRetires } = modelledNative();
    const view = await renderShown(true);
    native.current = 1; // the shown render's completion recorded it natively
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(routeCalls("save_rendered_report_html")).toHaveLength(1));
    // The source changes while the save is pending; the frontend has committed the change.
    view.rerender(panel(result("run:invented-2")));
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    // The native save reaches publication before the retirement is delivered.
    publishPendingSaves();
    deliverRetires();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(
      { publishedAfterSourceChange: native.published.length, retiresSent: native.retiresSent, retiresDeliveredAtPublication: native.deliveredAtPublication },
      "REQ-022: once the source has changed, the earlier render is not published; the save's publication is ordered after the change"
    ).toEqual({ publishedAfterSourceChange: 0, retiresSent: 1, retiresDeliveredAtPublication: [0] });
  });

  it("invalidates a pending render's native completion when the source changes before any response is accepted", async () => {
    const { native, deliverRetires } = modelledNative();
    let complete!: () => void;
    nativeRender.current = {
      enter: () => { native.current = null; native.renders += 1; }, // the render command retires at entry
      completion: () => new Promise<void>((resolve) => { complete = () => { native.current = native.renders; resolve(); }; })
    };
    const view = render(panel());
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(native.renders).toBe(1));
    // No native response has been accepted yet; the source changes.
    view.rerender(panel(result("run:invented-2")));
    const retiresSentOnSourceChange = native.retiresSent;
    deliverRetires();
    // The earlier request's worker then completes.
    complete();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    expect(
      { retiresSentOnSourceChange: retiresSentOnSourceChange > 0, nativeHoldsTheEarlierRender: native.current !== null },
      "REQ-022: a source change while a render is pending invalidates it natively, so that its later completion holds nothing admissible"
    ).toEqual({ retiresSentOnSourceChange: true, nativeHoldsTheEarlierRender: false });
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
