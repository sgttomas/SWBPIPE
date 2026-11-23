// DEL-08-01 REQ-022: every resolve and reject class of the HTML save and the print, as the
// session's report output outcome classifies and words it. An unknown outcome is never
// "not saved"; a changed workspace is stated beside a success; a print window is never
// "printed". All data are invented.
import { describe, expect, it } from "vitest";
import { ResourceUnavailableError } from "../../services/memoryPool";
import type { RenderedReportHtmlSaveReceipt, RenderedReportRouteError } from "../../services/renderedReportRoutes";
import type { ReportPackageSaveReceipt } from "../../services/reportPackageSaveService";
import {
  packageFailureSettlement,
  packageNotSentSettlement,
  packageReceiptSettlement,
  packageRetiredSettlement,
  packageSizeRefusedSettlement,
  printSettlement,
  ReportOutputOutcomes,
  reportOutputDetails,
  reportOutputText,
  saveSettlement,
  type ReportOutputBasis,
  type ReportOutputEntry
} from "./reportOutputOutcomes";

const basis = { sequence: 4, projectId: "project:invented", model: "project:invented", run: "run:invented-1" };
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

  it("keeps an HTML save's and a print's entries as they were beside the package kinds (issue 508)", () => {
    const outcomes = new ReportOutputOutcomes();
    const save = outcomes.begin("save", basis);
    const print = outcomes.begin("print", basis);
    const pkg = outcomes.begin("report_package", basis);
    // A package settlement settles a save's entry with the common members only.
    outcomes.settle(save, packageReceiptSettlement(packageReceipt("durable")));
    outcomes.settle(print, printSettlement({ kind: "failed", message: "lost", failure: new Error("lost") }));
    const [saved, printed, packaged] = outcomes.snapshot();
    expect(Object.keys(saved).sort()).toEqual(["basename", "basis", "code", "detail", "id", "kind", "state"]);
    expect(Object.keys(printed).sort()).toEqual(["basename", "basis", "code", "detail", "id", "kind", "state"]);
    expect(packaged).toMatchObject({ id: pkg, kind: "report_package", state: "pending", package: { sent: true, warnings: [], receipt: null, listEntry: null } });
  });
});

const packageReceipt = (durability: ReportPackageSaveReceipt["durability"], outcome: "saved" | "cancelled" = "saved"): ReportPackageSaveReceipt => ({
  outcome, stage: "complete", code: "REPORT-PACKAGE-SAVED", container_file_name: "invented-suggested.opsproj", byte_count: 7,
  package_identity_sha256_hex: "a".repeat(64), container_sha256_hex: "b".repeat(64), members: [], replaced_existing: false, durability,
  redaction_route_id: "DREP-PACKAGE-SAVE-009", redaction_decision_count: 0, redaction_finding_count: 0, redaction_blocking_count: 0,
  local_first_route_id: "DREP-PACKAGE-SAVE-009", local_first_export_context: "local_private", local_first_storage_context: "local_private",
  local_first_action: "include_metadata_only", local_first_reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", local_first_blocked: false,
  local_first_metadata_only: true, local_first_explicit_local_private_intent: true, selected_basename: "invented.opsproj",
  path_containment: "best_effort_non_adversarial", limitation: "Invented limitation."
});
const commandError = (code: string, stage: string, members: Record<string, unknown> = {}) =>
  ({ code, stage, message: `${code}: invented.`, cleanup_code: null, selected_basename: "invented.opsproj", ...members });

describe("report package outcomes (issue 508)", () => {
  const line = (settlement: ReturnType<typeof packageReceiptSettlement>, current = basis.sequence, kind: "report_package" | "shareable_report_package" = "report_package",
    project: string | null = basis.projectId, entryBasis: ReportOutputBasis = basis) =>
    reportOutputText({ id: 1, kind, basis: entryBasis, ...settlement }, current, project);

  it("classifies the native reply: refusal stages, writer stages, the worker's failure and an unreadable rejection", () => {
    const state = (failure: unknown) => packageFailureSettlement(failure).state;
    for (const stage of ["admit", "assemble", "validate"]) expect(state(commandError("REPORT-PACKAGE-INVENTED", stage))).toBe("refused");
    for (const stage of ["create_temp", "write", "flush", "temp_sync", "rename"]) expect(state(commandError("REPORT-PACKAGE-INVENTED", stage))).toBe("failed");
    expect(state(commandError("REPORT-PACKAGE-SAVE-WORKER-FAILED", "assemble"))).toBe("unknown");
    expect(state(commandError("REPORT-PACKAGE-INVENTED", "invented_later_stage"))).toBe("unknown");
    expect(state(new Error("invented lost reply"))).toBe("unknown");
    expect(state({ unreadable: true })).toBe("unknown");
    expect(state(new Error("INVALID_REQUEST: invented"))).toBe("refused");
    expect(state(new ResourceUnavailableError({ kind: "headroom_insufficient", required_bytes: 1, available_bytes: 0 } as never, null))).toBe("refused");
  });

  it("words the line: outcome first, the file, the code, and a changed workspace with the model and run", () => {
    expect(line(packageReceiptSettlement(packageReceipt("durable")))).toBe("Report package saved as invented.opsproj.");
    expect(line(packageReceiptSettlement(packageReceipt("durable")), 5, "shareable_report_package"))
      .toBe("Shareable report package saved as invented.opsproj. It is of an earlier run: project:invented, run run:invented-1. The model or its results have changed since.");
    expect(line(packageReceiptSettlement(packageReceipt("saved_durability_uncertain"))))
      .toBe("Report package saved as invented.opsproj. The system did not confirm it is safely on disk.");
    expect(line(packageReceiptSettlement(packageReceipt("not_applicable", "cancelled")))).toBe("Report package not saved: the save was cancelled.");
    expect(line(packageFailureSettlement(commandError("REPORT-PACKAGE-INVENTED", "admit")))).toBe("Report package not saved: invented (REPORT-PACKAGE-INVENTED).");
    expect(line(packageNotSentSettlement("REPORT-PACKAGE-INVENTED-BLOCKED: invented block.")))
      .toBe("Report package not saved: invented block (REPORT-PACKAGE-INVENTED-BLOCKED). Nothing was written.");
    const unknown = line(packageFailureSettlement(new Error("invented lost reply")));
    expect(unknown).toBe("It is not known whether the report package was saved. Check the file you selected.");
    expect(unknown).not.toMatch(/not saved|nothing was/i);
  });

  it("keeps a size refusal's code and exact sizes behind its details, not on its line; every other entry keeps its code on its line", () => {
    const words = { reading: "this run record is too large for it (88 MiB; the most it can take is 64 MiB)", details: "RESULT-DETAIL-WEBVIEW-LIMIT: run record 92274688 bytes; limit 67108864 bytes." };
    const sized = packageSizeRefusedSettlement("RESULT-DETAIL-WEBVIEW-LIMIT", words);
    expect(sized).toMatchObject({ state: "refused", code: "RESULT-DETAIL-WEBVIEW-LIMIT", package: { sent: false, nothingWritten: true, detailsLine: words.details } });
    expect(line(sized)).toBe("Report package not saved: this run record is too large for it (88 MiB; the most it can take is 64 MiB). Nothing was written.");
    expect(line(sized, 5, "shareable_report_package")).toBe("Shareable report package not saved: this run record is too large for it (88 MiB; the most it can take is 64 MiB). Nothing was written. It is of an earlier run: project:invented, run run:invented-1. The model or its results have changed since.");
    expect(reportOutputDetails({ id: 1, kind: "report_package", basis, ...sized })).toEqual(["Report basis: model project:invented, run run:invented-1.", words.details]);
    // Every other settlement keeps no details line, so its line reads as it did (the line cases above).
    for (const other of [
      packageReceiptSettlement(packageReceipt("durable")), packageReceiptSettlement(packageReceipt("not_applicable", "cancelled")),
      packageFailureSettlement(commandError("REPORT-PACKAGE-INVENTED", "admit")), packageFailureSettlement(commandError("REPORT-PACKAGE-INVENTED", "write")),
      packageFailureSettlement(new Error("invented lost reply")), packageNotSentSettlement("REPORT-PACKAGE-INVENTED-BLOCKED: invented block."),
      packageRetiredSettlement(), packageRetiredSettlement(true)
    ]) expect(other.package.detailsLine).toBeNull();
  });

  it("settles a receipt outcome it does not know as unknown, never as cancelled", () => {
    const strange = packageReceiptSettlement({ ...packageReceipt("durable"), outcome: "invented_outcome" as never });
    expect(strange.state).toBe("unknown");
    expect(line(strange)).toBe("It is not known whether the report package was saved. Check invented.opsproj.");
    expect(packageReceiptSettlement(packageReceipt("not_applicable", "cancelled")).state).toBe("cancelled");
  });

  it("names another project by its name, the same project's earlier run by model and run, and does not guess without an identity", () => {
    const saved = packageReceiptSettlement(packageReceipt("durable"));
    expect(line(saved, 5, "report_package", "project:other"))
      .toBe("Report package saved as invented.opsproj. It is of another project: project:invented.");
    expect(line(saved, 5, "report_package", "project:invented"))
      .toBe("Report package saved as invented.opsproj. It is of an earlier run: project:invented, run run:invented-1. The model or its results have changed since.");
    // An entry with no project identity, or a workspace whose identity is not known: the earlier-run sentence.
    expect(line(saved, 5, "report_package", "project:other", { ...basis, projectId: null })).toMatch(/It is of an earlier run: /);
    expect(line(saved, 5, "report_package", null)).toMatch(/It is of an earlier run: /);
    // The HTML save and the print follow the same rule after their changed-workspace sentence.
    const html = reportOutputText(entry("save", saveSettlement({ kind: "done", receipt: receipt("durable") })), 5, "project:other");
    expect(html).toMatch(/^Saved the selected report\. The workspace has changed since it was prepared\. It is of another project: project:invented\. File: invented-report\.html\./);
    expect(reportOutputText(entry("save", saveSettlement({ kind: "done", receipt: receipt("durable") })), 5, "project:invented")).not.toContain("another project");
  });

  it("words the memory pool's refusal, alone and before a retirement, without naming the native side", () => {
    const headroom = new ResourceUnavailableError({ kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 } as never, null);
    expect(line(packageFailureSettlement(headroom))).toBe("Report package not saved: the memory pool had too little free at that moment (RESOURCE_UNAVAILABLE).");
    expect(line(packageRetiredSettlement(true), basis.sequence, "shareable_report_package")).toBe(
      "Shareable report package not saved: the memory pool had too little free at that moment, and the workspace changed before it could be tried again (REPORT-REQUEST-RETIRED). Nothing was written."
    );
    expect(line(packageRetiredSettlement())).toBe("Report package not saved: the workspace changed before the save began (REPORT-REQUEST-RETIRED). Nothing was written.");
    for (const words of [line(packageFailureSettlement(headroom)), line(packageRetiredSettlement(true))]) expect(words).not.toMatch(/native|headroom/i);
  });
});
