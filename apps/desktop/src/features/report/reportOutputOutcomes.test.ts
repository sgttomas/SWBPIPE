// DEL-08-01 REQ-022: every resolve and reject class of the HTML save and the print, as the
// session's report output outcome classifies and words it. An unknown outcome is never
// "not saved"; a changed workspace is stated beside a success; a print window is never
// "printed". All data are invented.
import { describe, expect, it } from "vitest";
import { ResourceUnavailableError } from "../../services/memoryPool";
import type { RenderedReportHtmlSaveReceipt, RenderedReportRouteError } from "../../services/renderedReportRoutes";
import { printSettlement, ReportOutputOutcomes, reportOutputText, saveSettlement, type ReportOutputEntry } from "./reportOutputOutcomes";

const basis = { sequence: 4, model: "project:invented", run: "run:invented-1" };
const receipt = (durability: RenderedReportHtmlSaveReceipt["durability"], outcome: "saved" | "cancelled" = "saved"): RenderedReportHtmlSaveReceipt => ({
  outcome, route_id: "DREP-HTML-SAVE-005", export_context: "local_private", sha256_hex: "b".repeat(64), byte_count: 42,
  file_name: "invented-report.html", replaced_existing: true, durability
});
const routeError = (code: string, stage: string, destination: RenderedReportRouteError["destination"]): RenderedReportRouteError => ({
  code, stage, message: `Invented ${code}.`, destination, cleanup_code: null, selected_basename: "invented-report.html"
});

function entry(kind: "save" | "print", settlement: ReturnType<typeof saveSettlement>): ReportOutputEntry {
  return { id: 1, kind, basis, ...settlement };
}

describe("report output outcomes", () => {
  it("words each save outcome class", () => {
    const text = (settlement: ReturnType<typeof saveSettlement>, current = basis.sequence) => [settlement.state, reportOutputText(entry("save", settlement), current)] as const;
    const saved = saveSettlement({ kind: "done", receipt: receipt("durable") });
    expect(text(saved)).toEqual(["saved", "Saved the selected report. File: invented-report.html. 42 bytes, SHA-256 " + "b".repeat(64) + ". It replaced the file that was there. Report basis: model project:invented, run run:invented-1."]);
    expect(text(saved, 5)[1]).toMatch(/^Saved the selected report\. The workspace has changed since it was prepared\. File: invented-report\.html\./);
    const uncertain = text(saveSettlement({ kind: "done", receipt: receipt("saved_durability_uncertain") }), 5);
    expect(uncertain[0]).toBe("saved_durability_unconfirmed");
    expect(uncertain[1]).toContain("The workspace has changed since it was prepared.");
    expect(uncertain[1]).toContain("the system did not confirm it is durable on disk");
    expect(text(saveSettlement({ kind: "done", receipt: receipt("not_applicable", "cancelled") }))).toEqual(["cancelled", "Save cancelled: nothing was written. Report basis: model project:invented, run run:invented-1."]);
    const refused = text(saveSettlement({ kind: "refused", error: routeError("REPORT-SOURCE-RETIRED", "source", "untouched") }));
    expect(refused[0]).toBe("refused");
    expect(refused[1]).toMatch(/^Not saved \(REPORT-SOURCE-RETIRED\): Invented/);
    const retired = text(saveSettlement({ kind: "refused", error: routeError("REPORT-HTML-SAVE-RETIRED", "currency", "as_it_was") }));
    expect(retired[0]).toBe("refused");
    expect(retired[1]).toContain("The destination was left as it was.");
    const failed = text(saveSettlement({ kind: "refused", error: routeError("REPORT-HTML-SAVE-RENAME-FAILED", "rename", "as_it_was") }));
    expect(failed[0]).toBe("failed");
    expect(failed[1]).toContain("The destination was left as it was.");
    const worker = text(saveSettlement({ kind: "refused", error: routeError("REPORT-ROUTE-WORKER-FAILED", "worker", "unknown") }));
    expect(worker[0]).toBe("unknown");
    expect(worker[1]).toMatch(/^Save outcome unknown; check the file you selected\./);
    const transport = text(saveSettlement({ kind: "failed", message: "invented lost reply", failure: new Error("invented lost reply") }));
    expect(transport[0]).toBe("unknown");
    expect(transport[1]).toMatch(/^Save outcome unknown; check the file you selected\./);
    for (const unknown of [worker[1], transport[1]]) expect(unknown).not.toMatch(/Not saved/);
    const pool = saveSettlement({ kind: "failed", message: "refused", failure: new ResourceUnavailableError({ kind: "headroom_insufficient", required_bytes: 1, available_bytes: 0 } as never, null) });
    expect(pool.state).toBe("refused");
    const notSent = text(saveSettlement({ kind: "retired" }));
    expect(notSent[0]).toBe("refused");
    expect(notSent[1]).toContain("nothing was sent");
  });

  it("words each print outcome class without claiming anything was printed", () => {
    const opened = printSettlement({ kind: "done", receipt: { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: "b".repeat(64), print_sha256_hex: "c".repeat(64) } });
    const changed = reportOutputText(entry("print", opened), 5);
    expect(changed).toMatch(/^Print window opened for the selected report\. The workspace has changed since it was prepared\. Opening it does not mean anything was printed/);
    expect(changed).not.toMatch(/^Printed/);
    expect(reportOutputText(entry("print", printSettlement({ kind: "refused", error: routeError("REPORT-PRINT-ALREADY-OPEN", "print", "untouched") })), 4)).toMatch(/^Not printed \(REPORT-PRINT-ALREADY-OPEN\)/);
    expect(reportOutputText(entry("print", printSettlement({ kind: "failed", message: "lost", failure: new Error("lost") })), 4)).toMatch(/^Print outcome unknown\./);
  });

  it("settles an operation once and keeps it until it is dismissed", () => {
    const outcomes = new ReportOutputOutcomes();
    const id = outcomes.begin("save", basis);
    expect(outcomes.snapshot()[0].state).toBe("pending");
    outcomes.settle(id, saveSettlement({ kind: "done", receipt: receipt("durable") }));
    outcomes.settle(id, saveSettlement({ kind: "done", receipt: receipt("durable", "cancelled") }));
    expect(outcomes.snapshot()[0].state).toBe("saved");
    outcomes.dismiss(id);
    expect(outcomes.snapshot()).toEqual([]);
  });
});
