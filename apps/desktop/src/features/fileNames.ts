/** The suggested name of every file the desktop offers to save or download (DEL-07-11 REQ-009): a fixed word for
 * each kind of export and, where the export covers a case, filter, run or state, a qualifier naming it, in lower-case
 * ASCII letters and digits joined by single hyphens, with the file's extension. No suggested name contains a
 * project name, a project or model identifier, or a path: the rule takes no project, and a qualifier is drawn only
 * from what the export covers, never from the project's or the model's identity. A person may change the name in
 * the save panel, and the native dialog resolves a name already taken. Every kind and its extension is an entry of
 * `FILE_KINDS`, so that no panel composes a name for itself.
 *
 * The qualifier's rule: lower case; every run of characters outside a to z and 0 to 9 one hyphen; no hyphen at
 * either end; at most 32 characters, cut at the limit with any trailing hyphen removed. Where that leaves nothing,
 * the name has no qualifier. The shared cases are fixtures/file_names/suggested_name_cases.json. */

const QUALIFIER_LIMIT = 32;

/** A qualifier as it stands in a suggested name; empty where the rule leaves nothing. */
export function qualifierSlug(text: string | null | undefined): string {
  return (text ?? "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, QUALIFIER_LIMIT).replace(/-+$/, "");
}

/** Every kind of file the desktop suggests a name for, and its extension. */
export const FILE_KINDS = Object.freeze({
  accessibilityBaseline: { kind: "accessibility-baseline", extension: "json" },
  adapterFramework: { kind: "adapter-framework", extension: "json" },
  buildReadiness: { kind: "build-readiness", extension: "json" },
  designWorkspace: { kind: "design-workspace", extension: "json" },
  editorContract: { kind: "editor-contract", extension: "json" },
  exportAdapterSdk: { kind: "export-adapter-sdk", extension: "json" },
  exportReview: { kind: "export-review", extension: "json" },
  externalProver: { kind: "external-prover", extension: "json" },
  externalRunJson: { kind: "external-run", extension: "json" },
  externalRunCsv: { kind: "external-run", extension: "csv" },
  externalRunResultsCsv: { kind: "external-run-results", extension: "csv" },
  handoff: { kind: "handoff", extension: "json" },
  headlessRunner: { kind: "headless-runner", extension: "json" },
  // Qualified by the handoff's local region: the model's own element and node ids, as the person named them, which
  // say what the file covers and tell two handoffs of one model apart. Not project or model identity.
  localFeaHandoff: { kind: "local-fea-handoff", extension: "json" },
  missingData: { kind: "missing-data", extension: "json" },
  nativePackage: { kind: "native-package", extension: "json" },
  operationDiff: { kind: "operation-diff", extension: "json" },
  operationLedger: { kind: "operation-ledger", extension: "json" },
  pcfPackage: { kind: "pcf-package", extension: "json" },
  pcfText: { kind: "pcf", extension: "pcf" },
  projectValidation: { kind: "validation", extension: "json" },
  redactedExport: { kind: "redacted-export", extension: "json" },
  reportPacket: { kind: "report-packet", extension: "json" },
  reportLint: { kind: "report-lint", extension: "json" },
  ruleCompleteness: { kind: "rule-completeness", extension: "json" },
  secretPrivateLibrary: { kind: "secret-private-library", extension: "json" },
  securityThreatModel: { kind: "security-threat-model", extension: "json" },
  solveJob: { kind: "solve-job", extension: "json" },
  storageAudit: { kind: "storage-audit", extension: "json" },
  stressNeutralRows: { kind: "stress-neutral-rows", extension: "csv" },
  telemetryBoundary: { kind: "telemetry-boundary", extension: "json" },
  validationEvidence: { kind: "validation-evidence", extension: "json" }
} as const);

/** Kinds reserved for the names other lanes move with their native saves: no entry of `FILE_KINDS` takes one. */
export const RESERVED_KINDS: ReadonlyArray<string> = Object.freeze(["report", "local-private-report", "result", "stress-neutral", "review-geometry"]);

/** The suggested names not yet built by this rule, each with the reason: the file, the beginning of the name, and
 * how many places in that file compose a name with that beginning. The lane that owns each removes its entry when
 * its native change lands. A test fails if a file holds a different number of such places, or if any other source
 * composes a name with the product's prefix or "preview". */
export const FILE_NAME_EXCEPTIONS: ReadonlyArray<{ file: string; starts: string; count: number; reason: string }> = Object.freeze([
  { file: "features/report/RenderedReportPanel.tsx", starts: "openpipestress-local-private-report-", count: 1, reason: "the local-private rendered report's download; another lane decides its form (reserved kind local-private-report)" },
  { file: "features/report/RenderedReportPanel.tsx", starts: "openpipestress-report-", count: 1, reason: "the rendered report's download; another lane decides its form (reserved kind report)" },
  { file: "features/result-export/ResultExportPanel.tsx", starts: "openpipestress-preview-results-", count: 1, reason: "the result JSON; its native save checks this prefix (nativeResultSave.ts and native_result_download.rs; reserved kind result)" },
  { file: "features/stress-neutral/StressNeutralExportPanel.tsx", starts: "openpipestress-preview-stress-neutral-", count: 1, reason: "the stress-neutral JSON; its native save checks this prefix (nativeResultSave.ts and native_result_download.rs; reserved kind stress-neutral)" },
  { file: "features/review-geometry/ReviewGeometryPanel.tsx", starts: "openpipestress-preview-review-geometry-", count: 2, reason: "the review geometry download, the twin of the name its package records and its native save forms; and the review geometry package's member file name, which the download repeats (reserved kind review-geometry)" }
]);

export type FileKind = keyof typeof FILE_KINDS;

/** A suggested name from a kind word, an extension and the qualifier where one names what the export covers. */
export function fileNameOf(kind: string, extension: string, qualifier?: string | null): string {
  const told = qualifierSlug(qualifier);
  return `${kind}${told ? `-${told}` : ""}.${extension}`;
}

/** The suggested name of a kind of file in `FILE_KINDS`. */
export function suggestedFileName(file: FileKind, qualifier?: string | null): string {
  const { kind, extension } = FILE_KINDS[file];
  return fileNameOf(kind, extension, qualifier);
}
