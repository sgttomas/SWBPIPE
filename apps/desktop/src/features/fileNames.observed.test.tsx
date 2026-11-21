import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import type { ReactElement } from "react";
import { cleanup, render } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import modelJson from "../../../../fixtures/product_preview/invented_demo_model.json";
import resultJson from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { buildAnalysisRunPreview } from "../services/previewService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, SolveJobAuditState } from "../types";
import { AccessibilityBaselinePanel } from "./accessibility-baseline/AccessibilityBaselinePanel";
import { AdapterFrameworkPanel } from "./adapter-framework/AdapterFrameworkPanel";
import { BuildReadinessPanel } from "./build-readiness/BuildReadinessPanel";
import { ExportAdapterSdkPanel } from "./export-adapter-sdk/ExportAdapterSdkPanel";
import { ExternalProverBoundaryPanel } from "./external-prover/ExternalProverBoundaryPanel";
import { FILE_KINDS, FILE_NAME_EXCEPTIONS, suggestedFileName, type FileKind } from "./fileNames";
import { HeadlessRunnerPanel } from "./headless-runner/HeadlessRunnerPanel";
import { LocalFeaHandoffPanel, buildLocalFeaHandoffPacket, localFeaHandoffFileName } from "./local-fea-handoff/LocalFeaHandoffPanel";
import { MissingDataBlockingPanel } from "./missing-data/MissingDataBlockingPanel";
import { ReportLintPanel } from "./report-lint/ReportLintPanel";
import { RuleCheckPanel } from "./rule-check/RuleCheckPanel";
import { SecurityThreatModelPanel } from "./security-threat-model/SecurityThreatModelPanel";
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
});

// Every source that composes a suggested name: each call names its kind by a literal key, and only the calls listed
// here take a qualifier, each from what the export covers. A panel that passes the project's or the model's id, or
// any other value, as a qualifier fails here until it is listed and its name is observed below.
const SRC = path.resolve(__dirname, "..");
const QUALIFIED: Readonly<Record<string, ReadonlyArray<FileKind>>> = { "features/local-fea-handoff/LocalFeaHandoffPanel.tsx": ["localFeaHandoff"] };

function sources(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const full = path.join(directory, entry);
    if (statSync(full).isDirectory()) return entry === "test" || entry === "test-support" ? [] : sources(full);
    return /\.(ts|tsx)$/.test(entry) && !/\.test\.(ts|tsx)$/.test(entry) ? [full] : [];
  });
}

/** Each call of `callee` in the text, as its argument text up to the matching parenthesis. */
function calls(text: string, callee: string): string[] {
  const found: string[] = [];
  for (const match of text.matchAll(new RegExp(`\\b${callee}\\(`, "g"))) {
    let depth = 1;
    let index = match.index! + match[0].length;
    const start = index;
    for (; index < text.length && depth > 0; index += 1) {
      if (text[index] === "(") depth += 1;
      else if (text[index] === ")") depth -= 1;
    }
    found.push(text.slice(start, index - 1));
  }
  return found;
}

const SITES = sources(SRC)
  .map((file) => ({ file: path.relative(SRC, file).split(path.sep).join("/"), text: readFileSync(file, "utf8") }))
  .filter(({ file }) => file !== "features/fileNames.ts")
  .flatMap(({ file, text }) => [
    ...calls(text, "suggestedFileName").map((args) => ({ file, callee: "suggestedFileName", args })),
    ...calls(text, "fileNameOf").map((args) => ({ file, callee: "fileNameOf", args }))
  ]);

describe("every place that composes a suggested name", () => {
  it("names its kind by a key of the table, and takes a qualifier only where listed", () => {
    const problems: string[] = [];
    for (const { file, callee, args } of SITES) {
      const kind = /^\s*"([A-Za-z]+)"\s*(,|$)/.exec(args);
      if (callee !== "suggestedFileName" || !kind || !(kind[1] in FILE_KINDS)) problems.push(`${file}: ${callee}(${args.trim()})`);
      else if (kind[2] === "," && !(QUALIFIED[file] ?? []).includes(kind[1] as FileKind)) problems.push(`${file}: a qualifier for ${kind[1]}: ${args.trim()}`);
    }
    expect(problems).toEqual([]);
  });

  it("uses every kind of the table, and every name each kind gives without a qualifier follows the convention", () => {
    const used = new Set(SITES.map(({ args }) => /^\s*"([A-Za-z]+)"/.exec(args)?.[1]));
    for (const file of Object.keys(FILE_KINDS) as FileKind[]) {
      expect(used.has(file), file).toBe(true);
      expect(outOfConvention(suggestedFileName(file)), file).toEqual([]);
    }
  });
});

describe("the names the panels show for the known project", () => {
  let analysisRun: AnalysisRunEnvelope;
  beforeAll(async () => {
    const inputManifest = await buildCurrentSessionInputManifest({
      model: MODEL,
      solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} },
      active_rule_packs: [],
      external_assets: []
    });
    analysisRun = await buildAnalysisRunPreview(RESULT, { inputManifest });
  });
  afterEach(cleanup);

  it("the local FEA handoff's qualifier is its region, drawn from the model's own entity ids", () => {
    const name = localFeaHandoffFileName(buildLocalFeaHandoffPacket({ model: MODEL, result: RESULT, analysisRun }));
    expect(name).toBe("local-fea-handoff-pipe-p-100-node-n-140.json");
    expect(outOfConvention(name)).toEqual([]);
  });

  const solveJob = { job_id: "invented", state: "completed", events: [], cancellation_requested: false, cancellation_status: "not-requested" } as unknown as SolveJobAuditState;
  const PANELS: ReadonlyArray<readonly [string, () => ReactElement, ReadonlyArray<FileKind>]> = [
    ["accessibility baseline", () => <AccessibilityBaselinePanel model={MODEL} />, ["accessibilityBaseline"]],
    ["adapter framework", () => <AdapterFrameworkPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["adapterFramework"]],
    ["build readiness", () => <BuildReadinessPanel model={MODEL} />, ["buildReadiness"]],
    ["export adapter SDK", () => <ExportAdapterSdkPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["exportAdapterSdk"]],
    ["external prover", () => <ExternalProverBoundaryPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["externalProver"]],
    ["headless runner", () => <HeadlessRunnerPanel model={MODEL} result={RESULT} analysisRun={analysisRun} solveJob={solveJob} />, ["headlessRunner"]],
    ["local FEA handoff", () => <LocalFeaHandoffPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["localFeaHandoff"]],
    ["missing data", () => <MissingDataBlockingPanel model={MODEL} result={RESULT} />, ["missingData"]],
    ["report lint", () => <ReportLintPanel model={MODEL} result={RESULT} analysisRun={analysisRun} />, ["reportLint"]],
    ["rule completeness", () => <RuleCheckPanel model={MODEL} result={RESULT} />, ["ruleCompleteness"]],
    ["security threat model", () => <SecurityThreatModelPanel model={MODEL} storageCapability={null} />, ["securityThreatModel"]],
    ["telemetry boundary", () => <TelemetryBoundaryPanel model={MODEL} storageCapability={null} />, ["telemetryBoundary"]],
    ["validation evidence", () => <ValidationEvidencePanel model={MODEL} />, ["validationEvidence"]]
  ];

  it.each(PANELS)("%s", (_name, element, kinds) => {
    const { container } = render(element());
    const names = Array.from(container.querySelectorAll("[download]"), (item) => item.getAttribute("download") ?? "");
    for (const kind of kinds) expect(names.some((name) => name.startsWith(FILE_KINDS[kind].kind)), kind).toBe(true);
    expect(names.flatMap(outOfConvention)).toEqual([]);
  });
});

// Not yet observed: the five names that do not yet go through the rule, each with the lane that moves it with its
// native save. Each is an entry of FILE_NAME_EXCEPTIONS, and this list must name every entry, so the gap is stated.
const NOT_YET_OBSERVED: ReadonlyArray<{ starts: string; owner: string }> = [
  { starts: "openpipestress-local-private-report-", owner: "the rendered report's lane (the local-private report's download)" },
  { starts: "openpipestress-report-", owner: "the rendered report's lane (the report's download)" },
  { starts: "openpipestress-preview-results-", owner: "the native save lane (the result JSON, whose native save checks the name)" },
  { starts: "openpipestress-preview-stress-neutral-", owner: "the native save lane (the stress-neutral JSON, whose native save checks the name)" },
  { starts: "openpipestress-preview-review-geometry-", owner: "the native save lane (the review geometry package and its download)" }
];

describe("the names not yet observed", () => {
  it("are exactly the exceptions, each with its owner", () => {
    expect(NOT_YET_OBSERVED.map((item) => item.starts).sort()).toEqual(FILE_NAME_EXCEPTIONS.map((item) => item.starts).sort());
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
