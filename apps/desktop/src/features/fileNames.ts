/** The suggested name of every file the desktop offers to save or download: the project's name, then the kind of
 * file, in lower case with hyphens, no spaces and no product or brand name. A person may change it in the save
 * panel. Every kind and its extension is an entry of `FILE_KINDS`, so that no panel composes a name for itself.
 *
 * The project's name, as a slug: its display name; where that leaves nothing, its id without the leading
 * "project:"; where that leaves nothing too, "project". Lower case; every run of characters outside a to z and 0
 * to 9 one hyphen; no hyphen at either end; at most 64 characters, cut at the limit with any trailing hyphen
 * removed. A kind that writes one file per run, request or record of the same project takes a qualifier after the
 * kind (that record's own reference, by the same rule, at most 32 characters), so that two such files are told
 * apart; where the qualifier's slug is empty, the name has none. */

const PROJECT_LIMIT = 64;
const QUALIFIER_LIMIT = 32;

function slug(text: string | null | undefined, limit: number): string {
  return (text ?? "").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, limit).replace(/-+$/, "");
}

/** The project's name as a file-name slug. */
export function projectSlug(project: { id: string; name?: string | null }): string {
  return slug(project.name, PROJECT_LIMIT) || slug(project.id.replace(/^project:/, ""), PROJECT_LIMIT) || "project";
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
  localFeaHandoff: { kind: "local-fea-handoff", extension: "json" },
  missingData: { kind: "missing-data", extension: "json" },
  nativePackage: { kind: "native-package", extension: "json" },
  operationDiff: { kind: "operation-diff", extension: "json" },
  operationLedger: { kind: "operation-ledger", extension: "json" },
  pcfPackage: { kind: "pcf-package", extension: "json" },
  pcfText: { kind: "pcf", extension: "pcf" },
  projectValidation: { kind: "validation", extension: "json" },
  redactedExport: { kind: "redacted-export", extension: "json" },
  report: { kind: "report", extension: "json" },
  reportLint: { kind: "report-lint", extension: "json" },
  ruleCompleteness: { kind: "rule-completeness", extension: "json" },
  secretPrivateLibrary: { kind: "secret-private-library", extension: "json" },
  securityThreatModel: { kind: "security-threat-model", extension: "json" },
  solveJob: { kind: "solve-job", extension: "json" },
  storageAudit: { kind: "storage-audit", extension: "json" },
  stressNeutralCsv: { kind: "stress-neutral", extension: "csv" },
  telemetryBoundary: { kind: "telemetry-boundary", extension: "json" },
  validationEvidence: { kind: "validation-evidence", extension: "json" },
  // The intended names of three files whose present names a native save checks or forms: they move with it, and
  // nothing builds a name from these entries until then.
  resultJson: { kind: "result", extension: "json" },
  stressNeutralJson: { kind: "stress-neutral", extension: "json" },
  reviewGeometry: { kind: "review-geometry", extension: "gltf.json" }
} as const);

export type FileKind = keyof typeof FILE_KINDS;

/** The suggested name: the project's slug, the kind, the qualifier where one tells such files apart, and the
 * extension. */
export function suggestedFileName(project: { id: string; name?: string | null }, file: FileKind, qualifier?: string | null): string {
  const { kind, extension } = FILE_KINDS[file];
  const told = slug(qualifier, QUALIFIER_LIMIT);
  return `${projectSlug(project)}-${kind}${told ? `-${told}` : ""}.${extension}`;
}
