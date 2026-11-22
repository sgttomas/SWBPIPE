import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import type { ReactElement } from "react";
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({
  invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock)
}));
import modelJson from "../../../../fixtures/product_preview/invented_demo_model.json";
import resultJson from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../services/previewService";
import { createSolveJobTransport, nativeMechanicsReplayPair, solveBytes } from "../test/nativeMechanicsReplay";
import type { AnalysisRunEnvelope, EditorOperationIntent, MechanicsResult, PreviewModel, SolveJobAuditState } from "../types";
import { AccessibilityBaselinePanel } from "./accessibility-baseline/AccessibilityBaselinePanel";
import { AdapterFrameworkPanel } from "./adapter-framework/AdapterFrameworkPanel";
import { BuildReadinessPanel } from "./build-readiness/BuildReadinessPanel";
import { CaepipeExternalHarnessPanel } from "./caepipe-external/CaepipeExternalHarnessPanel";
import { DesignWorkspacePanel } from "./design-workspace/DesignWorkspacePanel";
import { DiffPreviewPanel } from "./diff-preview/DiffPreviewPanel";
import { EditorContractPanel } from "./editor-contract/EditorContractPanel";
import { ExportAdapterSdkPanel } from "./export-adapter-sdk/ExportAdapterSdkPanel";
import { ExportReviewPanel } from "./export-review/ExportReviewPanel";
import { ExternalProverBoundaryPanel } from "./external-prover/ExternalProverBoundaryPanel";
import { FILE_KINDS, FILE_NAME_EXCEPTIONS, NATIVE_FILE_NAME_EXCEPTIONS, suggestedFileName, type FileKind } from "./fileNames";
import { HandoffPanel } from "./handoff/HandoffPanel";
import { HeadlessRunnerPanel } from "./headless-runner/HeadlessRunnerPanel";
import { LocalFeaHandoffPanel, buildLocalFeaHandoffPacket, localFeaHandoffFileName } from "./local-fea-handoff/LocalFeaHandoffPanel";
import { MissingDataBlockingPanel } from "./missing-data/MissingDataBlockingPanel";
import { NativePackagePanel } from "./native-package/NativePackagePanel";
import { OfflineProposalIntakePanel } from "./offline-proposal-intake/OfflineProposalIntakePanel";
import { draftIntent } from "./offline-proposal-intake/workflowSupport";
import { OperationLedgerPanel } from "./operations/OperationLedgerPanel";
import { PcfExportPanel } from "./pcf-export/PcfExportPanel";
import { ProjectStorageAuditPanel } from "./project-storage/ProjectStorageAuditPanel";
import { ProjectValidationPanel } from "./project-validation/ProjectValidationPanel";
import { RedactionExportControlsPanel } from "./redaction-controls/RedactionExportControlsPanel";
import { ReportLintPanel } from "./report-lint/ReportLintPanel";
import { ReportPanel } from "./report/ReportPanel";
import { RuleCheckPanel } from "./rule-check/RuleCheckPanel";
import { SecretPrivateLibraryPanel } from "./secret-private-library/SecretPrivateLibraryPanel";
import { SecurityThreatModelPanel } from "./security-threat-model/SecurityThreatModelPanel";
import { SolvePanel } from "./solve/SolvePanel";
import { StressNeutralExportPanel } from "./stress-neutral/StressNeutralExportPanel";
import { TelemetryBoundaryPanel } from "./telemetry/TelemetryBoundaryPanel";
import { ValidationEvidencePanel } from "./validation-evidence/ValidationEvidencePanel";

// DEL-07-11 VER-007 and AC-007: the suggested name of every export, for an open project whose name and identifiers
// are known, follows the convention with its kind's word and extension, and contains none of the project's name,
// its identifier, the model's identifier or a path. The project is invented, with a name and an id that could not
// occur by accident; the model's identifier is the project's id (the engine writes it as the result's model_ref).
const KNOWN = { id: "project:wren-ferrule-9x2q", name: "Quokka Larch Bay 4417" };
const MODEL = { ...modelJson, project: { ...modelJson.project, ...KNOWN } } as unknown as PreviewModel;
const RESULT = { ...structuredClone(resultJson), model_ref: KNOWN.id } as unknown as MechanicsResult;

const slugOf = (text: string) => text.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
const MODEL_ID = RESULT.model_ref;
const IDENTITY = [KNOWN.name, KNOWN.id, KNOWN.id.replace(/^project:/, ""), MODEL_ID];
const FORBIDDEN = [...new Set([...IDENTITY, ...IDENTITY.map(slugOf)].map((text) => text.toLowerCase()))];

// The convention: a kind word and extension from the table, with an optional qualifier of at most 32 characters
// in lower-case ASCII letters and digits joined by single hyphens.
const WORD = "[a-z0-9]+(?:-[a-z0-9]+)*";
const escape = (text: string) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const CONVENTION = Object.values(FILE_KINDS).map(({ kind, extension }) => new RegExp(`^${escape(kind)}(?:-(${WORD}))?\\.${escape(extension)}$`));

/** What is wrong with one suggested name; empty where it follows the convention and carries no identity. */
function outOfConvention(name: string): string[] {
  const problems: string[] = [];
  const match = CONVENTION.map((pattern) => pattern.exec(name)).find((item) => item !== null);
  if (!match) problems.push(`${name}: not a kind word, optional qualifier and extension from the table`);
  else if ((match[1] ?? "").length > 32) problems.push(`${name}: a qualifier over 32 characters`);
  for (const text of FORBIDDEN) if (name.toLowerCase().includes(text)) problems.push(`${name}: contains ${text}`);
  if (/[/\\]/.test(name)) problems.push(`${name}: contains a path separator`);
  return problems;
}

const SRC = path.resolve(__dirname, "..");

function sources(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const full = path.join(directory, entry);
    if (statSync(full).isDirectory()) return entry === "test" || entry === "test-support" ? [] : sources(full);
    return /\.(ts|tsx)$/.test(entry) && !/\.test\.(ts|tsx)$/.test(entry) ? [full] : [];
  });
}

function stripJsComments(text: string): string {
  let out = "";
  for (let i = 0; i < text.length; ) {
    const ch = text[i];
    const next = text[i + 1] ?? "";
    if (ch === "/" && next === "/") {
      out += "  ";
      i += 2;
      while (i < text.length && text[i] !== "\n") {
        out += " ";
        i += 1;
      }
    } else if (ch === "/" && next === "*") {
      out += "  ";
      i += 2;
      while (i < text.length && !(text[i] === "*" && text[i + 1] === "/")) {
        out += text[i] === "\n" ? "\n" : " ";
        i += 1;
      }
      if (i < text.length) {
        out += "  ";
        i += 2;
      }
    } else if (ch === '"' || ch === "'" || ch === "`") {
      const end = skipQuoted(text, i);
      out += text.slice(i, end + 1);
      i = end + 1;
    } else {
      out += ch;
      i += 1;
    }
  }
  return out;
}

/** Advance from the opening quote/backtick at `start` to the index of its matching closing quote/backtick. */
function skipQuoted(text: string, start: number): number {
  const quote = text[start];
  let i = start + 1;
  while (i < text.length) {
    const ch = text[i];
    if (ch === "\\") {
      i += 2;
      continue;
    }
    if (quote === "`" && ch === "$" && text[i + 1] === "{") {
      i += 2;
      let braceDepth = 1;
      while (i < text.length && braceDepth > 0) {
        const inner = text[i];
        if (inner === '"' || inner === "'" || inner === "`") {
          i = skipQuoted(text, i) + 1;
        } else {
          if (inner === "{") braceDepth += 1;
          else if (inner === "}") braceDepth -= 1;
          i += 1;
        }
      }
      continue;
    }
    if (ch === quote) return i;
    i += 1;
  }
  return text.length - 1;
}

/** Each call of `callee` in comment-stripped code, skipping string literals when locating calls and balancing parens. */
function calls(text: string, callee: string): string[] {
  const code = stripJsComments(text);
  const found: string[] = [];
  const token = `${callee}(`;
  for (let i = 0; i < code.length; i += 1) {
    const ch = code[i];
    if (ch === '"' || ch === "'" || ch === "`") {
      i = skipQuoted(code, i);
      continue;
    }
    const prev = i === 0 ? "" : code[i - 1];
    if (code.startsWith(token, i) && !/[A-Za-z0-9_$]/.test(prev)) {
      let depth = 1;
      let index = i + token.length;
      const start = index;
      while (index < code.length && depth > 0) {
        const c = code[index];
        if (c === '"' || c === "'" || c === "`") {
          index = skipQuoted(code, index) + 1;
          continue;
        }
        if (c === "(") depth += 1;
        else if (c === ")") depth -= 1;
        index += 1;
      }
      found.push(code.slice(start, index - 1));
    }
  }
  return found;
}

type DownloadSite = { kind: "jsx" | "dom" | "setAttribute"; expr: string };

/** Every JSX `download=...`, DOM `.download = ...`, and `.setAttribute("download", ...)` site in `text`. */
function downloadSites(text: string): DownloadSite[] {
  const code = stripJsComments(text);
  const sites: DownloadSite[] = [];
  for (let i = 0; i < code.length; i += 1) {
    const ch = code[i];
    if (ch === '"' || ch === "'" || ch === "`") {
      i = skipQuoted(code, i);
      continue;
    }
    const prev = i === 0 ? "" : code[i - 1];
    if (code.startsWith("download", i) && !/[A-Za-z0-9_$]/.test(prev) && !/[A-Za-z0-9_$]/.test(code[i + 8] ?? "")) {
      let j = i + 8;
      while (j < code.length && /\s/.test(code[j])) j += 1;
      if (code[j] !== "=" || code[j + 1] === "=") continue;
      j += 1;
      while (j < code.length && /\s/.test(code[j])) j += 1;
      if (prev === ".") {
        let end = j;
        while (end < code.length && code[end] !== ";" && code[end] !== "\n") {
          if (code[end] === '"' || code[end] === "'" || code[end] === "`") {
            end = skipQuoted(code, end) + 1;
            continue;
          }
          end += 1;
        }
        sites.push({ kind: "dom", expr: code.slice(j, end).trim() });
      } else if (code[j] === '"' || code[j] === "'") {
        const end = skipQuoted(code, j);
        sites.push({ kind: "jsx", expr: code.slice(j, end + 1) });
      } else if (code[j] === "{") {
        let depth = 1;
        let end = j + 1;
        while (end < code.length && depth > 0) {
          const c = code[end];
          if (c === '"' || c === "'" || c === "`") {
            end = skipQuoted(code, end) + 1;
            continue;
          }
          if (c === "{") depth += 1;
          else if (c === "}") depth -= 1;
          end += 1;
        }
        sites.push({ kind: "jsx", expr: code.slice(j + 1, end - 1).trim() });
      }
    }
  }
  for (const args of calls(code, "setAttribute")) {
    const match = /^\s*(["'])download\1\s*,([\s\S]*)$/.exec(args);
    if (match) sites.push({ kind: "setAttribute", expr: match[2].trim() });
  }
  return sites;
}

const AUDITED_DOWNLOAD_HELPERS: ReadonlyArray<{ file: string; expr: string }> = [
  { file: "features/local-fea-handoff/LocalFeaHandoffPanel.tsx", expr: "localFeaHandoffFileName(packet)" },
  { file: "features/result-export/csvSave.ts", expr: "request.file_name" }
];

function downloadSitesOutOfRule(file: string, text: string, exceptions: typeof FILE_NAME_EXCEPTIONS): string[] {
  const problems: string[] = [];
  for (const { kind, expr } of downloadSites(text)) {
    const direct = /^suggestedFileName\(\s*(["'])([A-Za-z0-9_]+)\1(?:\s*,[\s\S]*)?\)$/.exec(expr);
    if (direct && direct[2] in FILE_KINDS) continue;
    if (AUDITED_DOWNLOAD_HELPERS.some((helper) => helper.file === file && helper.expr === expr)) continue;
    const literalPrefix = /^([`"'])([\s\S]*)\1$/.exec(expr)?.[2] ?? "";
    if (exceptions.some((item) => item.file === file && literalPrefix.startsWith(item.starts))) continue;
    problems.push(`${file} (${kind}): download=${expr}`);
  }
  return problems;
}

const SITES = sources(SRC)
  .map((file) => ({ file: path.relative(SRC, file).split(path.sep).join("/"), text: readFileSync(file, "utf8") }))
  .filter(({ file }) => file !== "features/fileNames.ts")
  .flatMap(({ file, text }) => [
    ...calls(text, "suggestedFileName").map((args) => ({ file, callee: "suggestedFileName", args })),
    ...calls(text, "fileNameOf").map((args) => ({ file, callee: "fileNameOf", args }))
  ]);

const QUALIFIED: Readonly<Record<string, ReadonlyArray<FileKind>>> = { "features/local-fea-handoff/LocalFeaHandoffPanel.tsx": ["localFeaHandoff"] };

describe("the check's own ground", () => {
  it("knows a project name and identifiers that occur nowhere in the model's own entity ids", () => {
    const entityIds = JSON.stringify([MODEL.nodes, MODEL.pipe_segments, MODEL.supports, MODEL.components].flat().map((item) => (item as { id: string }).id)).toLowerCase();
    for (const text of FORBIDDEN) expect(entityIds).not.toContain(text);
    expect(MODEL_ID).toBe(KNOWN.id);
  });

  it("finds a name that carries the project's id, name or a path", () => {
    expect(outOfConvention(suggestedFileName("handoff", KNOWN.id))).toEqual(expect.arrayContaining([expect.stringContaining("contains wren-ferrule-9x2q")]));
    expect(outOfConvention(suggestedFileName("handoff", KNOWN.name))).toEqual(expect.arrayContaining([expect.stringContaining("contains quokka-larch-bay-4417")]));
    expect(outOfConvention("pcf-package/x.json")).not.toEqual([]);
    expect(outOfConvention("pcf-package.json")).toEqual([]);
  });

  it("parses calls safely across comments, quoted literals, and single or double quotes", () => {
    const sample = [
      "// suggestedFileName('ignoredComment')",
      '/* suggestedFileName("ignoredBlock") */',
      "const note = \"suggestedFileName('ignoredString')\";",
      "const a = suggestedFileName('pcfPackage');",
      'const b = suggestedFileName("localFeaHandoff", `a)b:${x}`);'
    ].join("\n");
    expect(calls(sample, "suggestedFileName")).toEqual(["'pcfPackage'", '"localFeaHandoff", `a)b:${x}`']);
  });

  it("flags unexempted hardcoded or dynamic download attributes", () => {
    expect(downloadSitesOutOfRule("Panel.tsx", '<a download="piping-operation-capabilities.json" />', [])).toEqual([
      'Panel.tsx (jsx): download="piping-operation-capabilities.json"'
    ]);
    expect(downloadSitesOutOfRule("Panel.tsx", "<a download={otherName} />", [])).toEqual([
      "Panel.tsx (jsx): download=otherName"
    ]);
    expect(downloadSitesOutOfRule("save.ts", 'anchor.download = "foo.json";', [])).toEqual([
      'save.ts (dom): download="foo.json"'
    ]);
    expect(downloadSitesOutOfRule("Panel.tsx", '<a download={suggestedFileName("operationCapabilities")} />', [])).toEqual([]);
  });
});

describe("every place that composes a suggested name", () => {
  it("names its kind by a key of the table, and takes a qualifier only where listed", () => {
    const problems: string[] = [];
    for (const { file, callee, args } of SITES) {
      const kind = /^\s*["']([A-Za-z0-9_]+)["']\s*(,|$)/.exec(args);
      if (callee !== "suggestedFileName" || !kind || !(kind[1] in FILE_KINDS)) problems.push(`${file}: ${callee}(${args.trim()})`);
      else if (kind[2] === "," && !(QUALIFIED[file] ?? []).includes(kind[1] as FileKind)) problems.push(`${file}: a qualifier for ${kind[1]}: ${args.trim()}`);
    }
    expect(problems).toEqual([]);
  });

  it("uses every kind of the table, and every name each kind gives without a qualifier follows the convention", () => {
    const used = new Set(SITES.map(({ args }) => /^\s*["']([A-Za-z0-9_]+)["']/.exec(args)?.[1]));
    for (const file of Object.keys(FILE_KINDS) as FileKind[]) {
      expect(used.has(file), file).toBe(true);
      expect(outOfConvention(suggestedFileName(file)), file).toEqual([]);
    }
  });

  it("routes every download attribute in desktop sources through suggestedFileName or a declared exception", () => {
    const problems: string[] = [];
    for (const file of sources(SRC)) {
      const rel = path.relative(SRC, file).split(path.sep).join("/");
      if (rel === "features/fileNames.ts") continue;
      problems.push(...downloadSitesOutOfRule(rel, readFileSync(file, "utf8"), FILE_NAME_EXCEPTIONS));
    }
    expect(problems).toEqual([]);
  });
});

const SAMPLE_INTENT: EditorOperationIntent = draftIntent({
  operation_kind: "modify",
  target: { object_type: "Element", ref: "pipe:P-100" },
  change: {
    change_id: "change:invented-1",
    change_kind: "set_field",
    field_label: "Outer diameter",
    field_path: "outside_diameter_mm",
    before: "114.3",
    after: "168.3",
    unit: "mm",
    dimension: "length",
    source_note: "invented edit for file-name check"
  },
  rationale: "Invented edit so diff and ledger exports render."
});

const SOLVE_JOB: SolveJobAuditState = {
  job_id: "invented",
  state: "completed",
  progress_basis: "job_state_only",
  percentages_synthesized: false,
  backend_percent_stream_available: false,
  cancellation_requested: false,
  cancellation_status: "not-requested",
  backend_job_seam: "browser_fixture_no_backend_job",
  backend_job_id: null,
  backend_cancellation_token: "not-used",
  backend_cancellation_scope: null,
  events: [],
  error_message: null,
  resource: {
    source: "not_reported",
    invocation_budget_bytes: null,
    pool_bytes: null,
    route_pricing: null,
    ingress_reserved_bytes: null,
    worker_grant_bytes: null,
    worker_peak_grant_bytes: null,
    worker_peak_footprint_bytes: null,
    queue_reason: null,
    refusal: null
  }
};

type PanelEntry = readonly [
  label: string,
  file: string,
  renderElement: () => ReactElement,
  kinds: ReadonlyArray<FileKind>,
  options?: {
    prepare?: (container: HTMLElement) => void | Promise<void>;
    allowedExceptions?: ReadonlyArray<string>;
  }
];

describe("the names the panels show for the known project", () => {
  let analysisRun: AnalysisRunEnvelope;
  let stressModel: PreviewModel;
  let stressResult: MechanicsResult;
  let stressAnalysisRun: AnalysisRunEnvelope;

  beforeAll(async () => {
    const inputManifest = await buildCurrentSessionInputManifest({
      model: MODEL,
      solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} },
      active_rule_packs: [],
      external_assets: []
    });
    analysisRun = await buildAnalysisRunPreview(RESULT, { inputManifest });

    const replay = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    stressModel = { ...replay.model, project: { ...replay.model.project, ...KNOWN } } as PreviewModel;
    const replaySource = { ...replay.source, model_ref: KNOWN.id } as MechanicsResult;
    const stressEntityIds = JSON.stringify([stressModel.nodes, stressModel.pipe_segments, stressModel.supports, stressModel.components].flat().map((item) => (item as { id: string }).id)).toLowerCase();
    for (const text of FORBIDDEN) expect(stressEntityIds).not.toContain(text);

    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    try {
      invokeMock.mockImplementation(createSolveJobTransport(() => solveBytes(replaySource)).invoke);
      stressResult = await runPreviewMechanics(stressModel, "sparse_interactive");
    } finally {
      delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
      invokeMock.mockReset();
    }
    const stressManifest = await buildCurrentSessionInputManifest({
      model: stressModel,
      solver: { solver_name: stressResult.producer!.component_name, solver_version: stressResult.producer!.component_version, solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} },
      active_rule_packs: [],
      external_assets: []
    });
    stressAnalysisRun = await buildAnalysisRunPreview(stressResult, { inputManifest: stressManifest });
  });

  afterEach(() => {
    cleanup();
    resetSessionResultCustodyForTests();
  });

  it("the local FEA handoff's qualifier is its region, drawn from the model's own entity ids", () => {
    const name = localFeaHandoffFileName(buildLocalFeaHandoffPacket({ model: MODEL, result: RESULT, analysisRun }));
    expect(name).toBe("local-fea-handoff-pipe-p-100-node-n-140.json");
    expect(outOfConvention(name)).toEqual([]);
  });

  const PANELS: ReadonlyArray<PanelEntry> = [
    ["accessibility baseline", "features/accessibility-baseline/AccessibilityBaselinePanel.tsx", () => <AccessibilityBaselinePanel model={MODEL} />, ["accessibilityBaseline"]],
    ["adapter framework", "features/adapter-framework/AdapterFrameworkPanel.tsx", () => <AdapterFrameworkPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["adapterFramework"]],
    ["build readiness", "features/build-readiness/BuildReadinessPanel.tsx", () => <BuildReadinessPanel model={MODEL} />, ["buildReadiness"]],
    ["caepipe external harness", "features/caepipe-external/CaepipeExternalHarnessPanel.tsx", () => <CaepipeExternalHarnessPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["externalRunJson", "externalRunCsv"]],
    ["design workspace", "features/design-workspace/DesignWorkspacePanel.tsx", () => <DesignWorkspacePanel model={MODEL} knowledge={null} result={RESULT} analysisRun={analysisRun} comparison={null} editorIntents={[SAMPLE_INTENT]} proposal={null} selectedReviewTarget={null} />, ["designWorkspace"]],
    ["diff preview", "features/diff-preview/DiffPreviewPanel.tsx", () => <DiffPreviewPanel model={MODEL} analysisRun={analysisRun} editorIntents={[SAMPLE_INTENT]} proposal={null} selectedReviewTarget={null} />, ["operationDiff"]],
    ["editor contract", "features/editor-contract/EditorContractPanel.tsx", () => <EditorContractPanel editorIntents={[SAMPLE_INTENT]} model={MODEL} />, ["editorContract"]],
    ["export adapter SDK", "features/export-adapter-sdk/ExportAdapterSdkPanel.tsx", () => <ExportAdapterSdkPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["exportAdapterSdk"]],
    ["export review", "features/export-review/ExportReviewPanel.tsx", () => <ExportReviewPanel model={MODEL} knowledge={null} result={RESULT} analysisRun={analysisRun} comparison={null} editorIntents={[SAMPLE_INTENT]} projectOperation="preview" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />, ["exportReview"]],
    ["external prover", "features/external-prover/ExternalProverBoundaryPanel.tsx", () => <ExternalProverBoundaryPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["externalProver"]],
    ["handoff", "features/handoff/HandoffPanel.tsx", () => <HandoffPanel model={MODEL} knowledge={null} result={RESULT} analysisRun={analysisRun} comparison={null} editorIntents={[SAMPLE_INTENT]} proposal={null} selectedReviewTarget={null} />, ["handoff"]],
    ["headless runner", "features/headless-runner/HeadlessRunnerPanel.tsx", () => <HeadlessRunnerPanel model={MODEL} result={RESULT} analysisRun={analysisRun} solveJob={SOLVE_JOB} />, ["headlessRunner"]],
    ["local FEA handoff", "features/local-fea-handoff/LocalFeaHandoffPanel.tsx", () => <LocalFeaHandoffPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["localFeaHandoff"]],
    ["missing data", "features/missing-data/MissingDataBlockingPanel.tsx", () => <MissingDataBlockingPanel model={MODEL} result={RESULT} />, ["missingData"]],
    ["native package", "features/native-package/NativePackagePanel.tsx", () => <NativePackagePanel analysisRun={analysisRun} editorIntents={[SAMPLE_INTENT]} model={MODEL} modelHash={null} projectSummary={null} proposal={null} result={RESULT} selectedReviewTarget={null} storageCapability={null} />, ["nativePackage"]],
    [
      "offline proposal intake",
      "features/offline-proposal-intake/OfflineProposalIntakePanel.tsx",
      () => <OfflineProposalIntakePanel model={MODEL} selection={{ type: "project", id: MODEL.project.id }} onQueueBatch={() => undefined} capabilityReference={{ invented: true }} />,
      ["operationCapabilities"],
      {
        prepare: (container) => {
          const details = container.querySelector("details");
          expect(details).not.toBeNull();
          details!.open = true;
        }
      }
    ],
    ["operation ledger", "features/operations/OperationLedgerPanel.tsx", () => <OperationLedgerPanel model={MODEL} analysisRun={analysisRun} editorIntents={[SAMPLE_INTENT]} proposal={null} selectedReviewTarget={null} onClearReviewQueue={() => undefined} />, ["operationLedger"]],
    ["PCF export", "features/pcf-export/PcfExportPanel.tsx", () => <PcfExportPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["pcfPackage", "pcfText"]],
    ["project storage audit", "features/project-storage/ProjectStorageAuditPanel.tsx", () => <ProjectStorageAuditPanel model={MODEL} storageCapability={null} projectSummary={null} projectIndex={null} projectMessage="" projectOperation="preview" editorIntents={[SAMPLE_INTENT]} proposal={null} />, ["storageAudit"]],
    ["project validation", "features/project-validation/ProjectValidationPanel.tsx", () => <ProjectValidationPanel model={MODEL} storageCapability={null} projectSummary={null} projectOperation="preview" editorIntents={[SAMPLE_INTENT]} proposal={null} modelHash={null} modelHashIntegrity={null} projectEnvelopeHash={null} projectEnvelopeHashIntegrity={null} modelDocumentVersion={null} />, ["projectValidation"]],
    [
      "redaction export controls",
      "features/redaction-controls/RedactionExportControlsPanel.tsx",
      () => <RedactionExportControlsPanel model={MODEL} />,
      ["redactedExport"],
      {
        prepare: (container) => {
          const toggle = container.querySelector('[data-testid="redaction-intent-toggle"]');
          expect(toggle).not.toBeNull();
          fireEvent.click(toggle!);
        }
      }
    ],
    ["report lint", "features/report-lint/ReportLintPanel.tsx", () => <ReportLintPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["reportLint"]],
    ["report packet", "features/report/ReportPanel.tsx", () => <ReportPanel model={MODEL} knowledge={null} result={RESULT} analysisRun={analysisRun} comparison={null} editorIntents={[SAMPLE_INTENT]} projectOperation="preview" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />, ["reportPacket"]],
    ["rule completeness", "features/rule-check/RuleCheckPanel.tsx", () => <RuleCheckPanel model={MODEL} result={RESULT} />, ["ruleCompleteness"]],
    ["secret private library", "features/secret-private-library/SecretPrivateLibraryPanel.tsx", () => <SecretPrivateLibraryPanel model={MODEL} storageCapability={null} />, ["secretPrivateLibrary"]],
    ["security threat model", "features/security-threat-model/SecurityThreatModelPanel.tsx", () => <SecurityThreatModelPanel model={MODEL} storageCapability={null} />, ["securityThreatModel"]],
    ["solve job", "features/solve/SolvePanel.tsx", () => <SolvePanel analysisRun={analysisRun} model={MODEL} result={RESULT} running={false} solveJob={SOLVE_JOB} solverMode="sparse_interactive" onCancel={() => undefined} onRun={() => undefined} onSolverModeChange={() => undefined} />, ["solveJob"]],
    [
      "stress neutral export",
      "features/stress-neutral/StressNeutralExportPanel.tsx",
      () => <StressNeutralExportPanel model={stressModel} result={stressResult} analysisRun={stressAnalysisRun} />,
      ["stressNeutralRows"],
      {
        prepare: async (container) => {
          await waitFor(() => expect(container.querySelector('[data-testid="stress-neutral-csv-link"]')).not.toBeNull());
        },
        allowedExceptions: ["openpipestress-preview-stress-neutral-"]
      }
    ],
    ["telemetry boundary", "features/telemetry/TelemetryBoundaryPanel.tsx", () => <TelemetryBoundaryPanel model={MODEL} storageCapability={null} />, ["telemetryBoundary"]],
    ["validation evidence", "features/validation-evidence/ValidationEvidencePanel.tsx", () => <ValidationEvidencePanel model={MODEL} />, ["validationEvidence"]]
  ];

  it("covers every panel that calls suggestedFileName and every downloadable FILE_KINDS entry", () => {
    const panelFilesWithCallSite = [...new Set(SITES.filter((site) => site.callee === "suggestedFileName").map((site) => site.file))].sort();
    expect(PANELS.map((entry) => entry[1]).sort()).toEqual(panelFilesWithCallSite);
    const renderedKinds = new Set(PANELS.flatMap((entry) => entry[3]));
    // externalRunResultsCsv is written into JSON fields (CaepipeExternalHarnessPanel.tsx:218-219), not a download link.
    const expectedDownloadKinds = (Object.keys(FILE_KINDS) as FileKind[]).filter((key) => key !== "externalRunResultsCsv").sort();
    expect([...renderedKinds].sort()).toEqual(expectedDownloadKinds);
  });

  const PANEL_CASES: ReadonlyArray<readonly [string, PanelEntry]> = PANELS.map((entry) => [entry[0], entry]);

  it.each(PANEL_CASES)("%s", async (_name, [, file, element, kinds, options]) => {
    const { container } = render(element());
    if (options?.prepare) await options.prepare(container);
    const names = Array.from(container.querySelectorAll("[download]"), (item) => item.getAttribute("download") ?? "");
    for (const kind of kinds) expect(names.some((name) => name.startsWith(FILE_KINDS[kind].kind)), kind).toBe(true);
    const declaredForFile = FILE_NAME_EXCEPTIONS.filter((item) => item.file === file).map((item) => item.starts);
    const allowed = options?.allowedExceptions ?? [];
    for (const prefix of allowed) expect(declaredForFile).toContain(prefix);
    const observedNames = names.filter((name) => !allowed.some((prefix) => name.startsWith(prefix)));
    expect(observedNames.length).toBeGreaterThan(0);
    expect(observedNames.flatMap(outOfConvention)).toEqual([]);
  });
});

// Not yet observed: every desktop and native/cross-boundary export file name exception outside `fileNames.ts`,
// each paired with its owning lane/deliverable so no deferred gap is left unnamed.
const NOT_YET_OBSERVED: ReadonlyArray<{ file: string; starts: string; owner: string }> = [
  { file: "features/report/RenderedReportPanel.tsx", starts: "openpipestress-local-private-report-", owner: "DEL-08-01 rendered report lane (local-private report download)" },
  { file: "features/report/RenderedReportPanel.tsx", starts: "openpipestress-report-", owner: "DEL-08-01 rendered report lane (report download)" },
  { file: "features/result-export/ResultExportPanel.tsx", starts: "openpipestress-preview-results-", owner: "DEL-08-04 native result save lane (result JSON download and native check)" },
  { file: "features/stress-neutral/StressNeutralExportPanel.tsx", starts: "openpipestress-preview-stress-neutral-", owner: "DEL-08-04 native result save lane (stress-neutral JSON download and native check)" },
  { file: "features/review-geometry/ReviewGeometryPanel.tsx", starts: "openpipestress-preview-review-geometry-", owner: "DEL-07-04 review geometry native save lane (package member and download)" },
  { file: "features/result-export/csvTable.ts", starts: "swbpipe-", owner: "DEL-08-04 native CSV export lane (results and comparison CSV tables)" },
  { file: "features/caepipe-mbf/mbfExport.ts", starts: "swbpipe-model", owner: "DEL-17-04 CAEPIPE MBF export lane (browser helper for swbpipe-model[-<slug>].mbf)" },
  { file: "apps/desktop/src-tauri/src/rendered_report_routes.rs", starts: "openpipestress-local-private-report-", owner: "DEL-08-01 rendered report native save lane (local-private HTML chooser name)" },
  { file: "apps/desktop/src-tauri/src/rendered_report_routes.rs", starts: "openpipestress-report-", owner: "DEL-08-01 rendered report native save lane (HTML chooser name)" },
  { file: "apps/desktop/src-tauri/src/report_package_save.rs", starts: "desktop-report- / desktop-user-released-report- (.opsproj)", owner: "DEL-08-01 report package native save lane (.opsproj chooser name)" },
  { file: "apps/desktop/src/features/report/reportPackageRequest.ts", starts: "desktop-report-", owner: "DEL-08-01 report package lane (local-private package_id feeding .opsproj name)" },
  { file: "apps/desktop/src/features/report/reportPackageRequest.ts", starts: "desktop-user-released-report-", owner: "DEL-08-01 report package lane (user-released package_id feeding .opsproj name)" },
  { file: "apps/desktop/src-tauri/src/native_result_download.rs", starts: "openpipestress-preview-results-", owner: "DEL-08-04 native result save lane (Rust result JSON name check)" },
  { file: "apps/desktop/src-tauri/src/native_result_download.rs", starts: "openpipestress-preview-stress-neutral-", owner: "DEL-08-04 native result save lane (Rust stress-neutral JSON name check)" },
  { file: "apps/desktop/src-tauri/src/native_result_download.rs", starts: "swbpipe-", owner: "DEL-08-04 native CSV save lane (Rust CSV table name check)" },
  { file: "apps/desktop/src-tauri/src/mbf_export.rs", starts: "swbpipe-model", owner: "DEL-17-04 CAEPIPE MBF export lane (Rust swbpipe-model[-<slug>].mbf validator)" }
];

describe("the names not yet observed", () => {
  it("are exactly the desktop and native exceptions, each with its owner", () => {
    const key = (item: { file: string; starts: string }) => `${item.file}::${item.starts}`;
    const expected = [...FILE_NAME_EXCEPTIONS, ...NATIVE_FILE_NAME_EXCEPTIONS].map(key).sort();
    expect(NOT_YET_OBSERVED.map(key).sort()).toEqual(expected);
    for (const item of NOT_YET_OBSERVED) expect(item.owner.trim().length, key(item)).toBeGreaterThan(0);
  });
});

describe("the native handoff package name", () => {
  it("uses the fixed kind word and package extension with no project identity or path", () => {
    const tauriSrc = path.resolve(SRC, "..", "src-tauri", "src");
    const handoffSource = readFileSync(path.join(tauriSrc, "handoff_package.rs"), "utf8");
    const publishSource = readFileSync(path.join(tauriSrc, "handoff_package_publish.rs"), "utf8");

    const kindMatch = /pub const SUGGESTED_PACKAGE_KIND:\s*&str\s*=\s*"([^"]+)";/.exec(handoffSource);
    expect(kindMatch).not.toBeNull();
    const kind = kindMatch![1];

    const extensionMatch = /pub const PACKAGE_EXTENSION:\s*&str\s*=\s*"([^"]+)";/.exec(publishSource);
    expect(extensionMatch).not.toBeNull();
    const extension = extensionMatch![1];

    const fnMatch = /pub fn suggested_package_name\([^)]*\)\s*->\s*String\s*\{([\s\S]*?)\n\}/.exec(handoffSource);
    expect(fnMatch).not.toBeNull();
    const fnBody = fnMatch![1];
    expect(fnBody).not.toContain("suggested_model_document_file_name");
    expect(fnBody).not.toContain("/project/id");
    expect(fnBody).not.toContain("/project/name");
    expect(fnBody).toContain('format!("{SUGGESTED_PACKAGE_KIND}.{}", publish::PACKAGE_EXTENSION)');

    const name = `${kind}.${extension}`;
    expect(name).toBe("handoff-package.swbhandoff");
    expect(name).toMatch(new RegExp(`^${WORD}\\.${escape(extension)}$`));
    for (const text of FORBIDDEN) expect(name.toLowerCase()).not.toContain(text);
    expect(/[/\\]/.test(name)).toBe(false);
  });
});
