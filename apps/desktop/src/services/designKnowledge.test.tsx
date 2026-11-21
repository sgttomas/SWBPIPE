// Issue 529: the bundled design knowledge's findings belong to the model the knowledge declares
// (`model_ref`) and are shown, counted and carried for that model only. One function family
// (`designKnowledge.ts`) applies the rule; every gatherer goes through it.
import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import knowledgeJson from "../../../../fixtures/product_preview/invented_design_knowledge.json";
import refusedPreviewModelJson from "../../../../fixtures/product_preview/invented_preview_model.json";
import missingDataModelJson from "../../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";
import demoModelJson from "../../../../fixtures/product_preview/invented_demo_model.json";
import demoResultJson from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import { designKnowledgeDiagnostics, designKnowledgeFor, designKnowledgeRecords } from "./designKnowledge";
import { buildAnalysisRunPreview } from "./previewService";
import { buildCurrentSessionInputManifest } from "./inputManifestService";
import { issueCountFor } from "../App";
import { DiagnosticsPanel } from "../features/diagnostics/DiagnosticsPanel";
import { buildHandoffPackage } from "../features/handoff/HandoffPanel";
import { ReportPanel } from "../features/report/ReportPanel";
import { ExportReviewPanel } from "../features/export-review/ExportReviewPanel";
import { DesignWorkspacePanel } from "../features/design-workspace/DesignWorkspacePanel";
import { KnowledgePanel } from "../features/knowledge/KnowledgePanel";
import { buildDiagnosticInterpretation, resolveDiagnosticEntitySelection } from "../features/results/resultInterpretation";
import type { AnalysisRunEnvelope, DesignKnowledge, MechanicsResult, PreviewModel } from "../types";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

const KNOWLEDGE = knowledgeJson as unknown as DesignKnowledge;
const CODES = ["SUPPORT_STIFFNESS_UNRESOLVED", "RULE_CHECK_NOT_PERFORMED"];
/** The model the knowledge declares (`model_ref`): the bundled demo, the app's default session
 * model; and models it is not bound to. */
const BOUND = demoModelJson as unknown as PreviewModel;
const MISSING_DATA = missingDataModelJson as unknown as PreviewModel;
const REFUSED_PREVIEW = refusedPreviewModelJson as unknown as PreviewModel;
const models = [
  { name: "the bundled demo model", model: BOUND, carried: true },
  { name: "the missing-data model", model: MISSING_DATA, carried: false },
  { name: "the refused preview model", model: REFUSED_PREVIEW, carried: false }
] as const;

describe("designKnowledgeDiagnostics", () => {
  it("returns the knowledge's diagnostics for the model it is bound to and none for any other", () => {
    expect(KNOWLEDGE.model_ref).toBe(BOUND.project.id);
    expect(KNOWLEDGE.diagnostics.map((item) => item.code)).toEqual(CODES);
    expect(designKnowledgeDiagnostics(KNOWLEDGE, BOUND)).toEqual(KNOWLEDGE.diagnostics);
    expect(designKnowledgeDiagnostics(KNOWLEDGE, BOUND.project.id)).toEqual(KNOWLEDGE.diagnostics);
    expect(designKnowledgeRecords(KNOWLEDGE, BOUND)).toEqual(KNOWLEDGE.records);
    expect(designKnowledgeFor(KNOWLEDGE, BOUND)).toBe(KNOWLEDGE);
    // another model
    expect(MISSING_DATA.project.id).not.toBe(KNOWLEDGE.model_ref);
    expect(designKnowledgeDiagnostics(KNOWLEDGE, MISSING_DATA)).toEqual([]);
    expect(designKnowledgeRecords(KNOWLEDGE, MISSING_DATA)).toEqual([]);
    expect(designKnowledgeFor(KNOWLEDGE, MISSING_DATA)).toBeNull();
    expect(designKnowledgeDiagnostics(KNOWLEDGE, REFUSED_PREVIEW)).toEqual([]);
    // a knowledge with no model_ref belongs to no model
    const unbound = { ...KNOWLEDGE, model_ref: undefined } as unknown as DesignKnowledge;
    expect(designKnowledgeDiagnostics(unbound, BOUND)).toEqual([]);
    expect(designKnowledgeDiagnostics({ ...KNOWLEDGE, model_ref: "" }, { project: { id: "" } } as PreviewModel)).toEqual([]);
    // no knowledge, no model
    expect(designKnowledgeDiagnostics(null, BOUND)).toEqual([]);
    expect(designKnowledgeDiagnostics(undefined, BOUND)).toEqual([]);
    expect(designKnowledgeDiagnostics(KNOWLEDGE, null)).toEqual([]);
    expect(designKnowledgeDiagnostics(KNOWLEDGE, { diagnostics: [] } as unknown as PreviewModel)).toEqual([]);
  });
});

// Vitest runs with the desktop package as its working directory.
const SOURCE_DIR = path.join(process.cwd(), "src");
const OWN_FILE = path.join(SOURCE_DIR, "services", "designKnowledge.ts");
function sourceFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(directory, entry.name);
    if (entry.isDirectory()) return ["test", "test-support"].includes(entry.name) ? [] : sourceFiles(full);
    return entry.isFile() && /\.tsx?$/.test(entry.name) && !/\.test\.tsx?$/.test(entry.name) ? [full] : [];
  });
}

describe("one rule in one place", () => {
  it("no source file but the function's own reads a knowledge's diagnostics or records directly", () => {
    // `knowledge?.diagnostics`, `loadedKnowledge.records`, `{ diagnostics } = knowledge`, …
    const member = /\b\w*knowledge\w*\s*[?!]?\s*\.\s*(diagnostics|records)\b/i;
    const destructured = /\{[^{}]*\b(diagnostics|records)\b[^{}]*\}\s*=\s*[\w.?!]*knowledge\b/i;
    const reads = (line: string) => member.test(line) || destructured.test(line);
    // the check sees what it is for
    expect(["...(knowledge?.diagnostics ?? [])", "knowledge!.records.length", "loadedKnowledge.diagnostics", "const { diagnostics } = session.knowledge"].every(reads)).toBe(true);
    expect(reads("current_design_knowledge_diagnostic_count: designKnowledgeDiagnostics(knowledge, model).length")).toBe(false);
    const files = sourceFiles(SOURCE_DIR);
    expect(files).toContain(OWN_FILE);
    const offences = files.filter((file) => file !== OWN_FILE).flatMap((file) =>
      readFileSync(file, "utf8").split("\n").flatMap((line, index) =>
        reads(line) ? [`${path.relative(SOURCE_DIR, file)}:${index + 1}: ${line.trim()}`] : []
      )
    );
    expect(offences).toEqual([]);
  });
});

async function runBasis() {
  const model = demoModelJson as unknown as PreviewModel;
  const result = structuredClone(demoResultJson) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = (await buildAnalysisRunPreview(result, { inputManifest })) as AnalysisRunEnvelope;
  return { result, analysisRun };
}

function exportedJson(container: HTMLElement, linkTestId: string) {
  fireEvent.click(within(container).getByTestId(`${linkTestId}-local-private-intent`));
  const href = within(container).getByTestId(linkTestId).getAttribute("href") ?? "";
  return JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
}

describe.each(models)("the knowledge's findings with $name open", ({ model, carried }) => {
  const expected = carried ? CODES : [];
  const present = (codes: string[]) => CODES.filter((code) => codes.includes(code));

  it("are in the Issues list and its count only when carried", () => {
    render(<DiagnosticsPanel model={model} knowledge={KNOWLEDGE} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(present(CODES.filter((code) => screen.queryByTestId(`diagnostic-${code}`) !== null))).toEqual(expected);
    expect(issueCountFor(model, KNOWLEDGE, null, {}) - issueCountFor(model, null, null, {})).toBe(expected.length);
    // a finding of the knowledge resolves (detail, entity selection) only where it is listed
    const detail = buildDiagnosticInterpretation({ model, knowledge: KNOWLEDGE, result: null, diagnosticId: "SUPPORT_STIFFNESS_UNRESOLVED" });
    expect(detail !== null).toBe(carried);
    expect(resolveDiagnosticEntitySelection({ model, knowledge: KNOWLEDGE, result: null, diagnosticId: "SUPPORT_STIFFNESS_UNRESOLVED" }) !== null).toBe(carried);
  });

  it("are in the Design Knowledge records only when carried", () => {
    render(<KnowledgePanel knowledge={KNOWLEDGE} model={model} result={null} />);
    for (const record of KNOWLEDGE.records) {
      expect(screen.queryByTestId(`knowledge-record-${record.id}`) !== null).toBe(carried);
    }
    expect(screen.queryByTestId("knowledge-none-for-model") !== null).toBe(!carried);
  });

  it("are in the report, the handoff, the export review and the design workspace only when carried", async () => {
    const { result, analysisRun } = await runBasis();
    const common = { model, knowledge: KNOWLEDGE, result, analysisRun, comparison: null, editorIntents: [], proposal: null, selectedReviewTarget: null };

    const handoff = buildHandoffPackage(common);
    expect(present(handoff.diagnostic_refs)).toEqual(expected);

    const policy = await import("../features/report/reportRedactionProjector"), actual = policy.controlReportDomAndJson;
    let observed: { diagnostic_refs: string[] } | undefined;
    vi.spyOn(policy, "controlReportDomAndJson").mockImplementation((input) => { observed = input as typeof observed; return actual(input); });
    const report = render(<ReportPanel {...common} projectOperation="invented" projectSummary={null} storageCapability={null} />);
    expect(present(observed!.diagnostic_refs)).toEqual(expected);
    report.unmount();

    const review = render(<ExportReviewPanel {...common} projectOperation="invented" projectSummary={null} storageCapability={null} />);
    const manifest = JSON.stringify(exportedJson(review.container, "export-review-link"));
    expect(present(CODES.filter((code) => manifest.includes(code)))).toEqual(expected);
    review.unmount();

    const withoutKnowledge = render(<DesignWorkspacePanel {...common} knowledge={null} />);
    const baseline = exportedJson(withoutKnowledge.container, "design-workspace-export-link");
    withoutKnowledge.unmount();
    const workspace = render(<DesignWorkspacePanel {...common} />);
    const packet = exportedJson(workspace.container, "design-workspace-export-link");
    expect(packet.summary.current_design_knowledge_diagnostic_count).toBe(expected.length);
    expect(packet.summary.current_total_diagnostic_count - baseline.summary.current_total_diagnostic_count).toBe(expected.length);
    expect(packet.summary.current_design_knowledge_record_count).toBe(carried ? KNOWLEDGE.records.length : 0);
    expect(packet.current_workspace_state.design_knowledge_panel).toBe(carried ? "available" : "unavailable_missing_input");
  });
});
