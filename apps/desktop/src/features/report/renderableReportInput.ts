import { analysisResultHashScope } from "../results/analysisResultHashScope";
import type {
  AnalysisRunEnvelope,
  LocalProjectSummary,
  MechanicsResult,
  PreviewComponent,
  PreviewModel
} from "../../types";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { libraryRefSites, type LibraryRefFinding } from "../../services/libraryImportService";
import { LIBRARY_REF_CHECK_SOURCE } from "../library/libraryRefFindings";
import {
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestEvidence
} from "../../services/inputManifestService";
import type { RulePackDocument } from "../../services/rulePackService";
import type { RuleCheckRunResult, RuleCheckStatus } from "../../services/ruleCheckService";
import desktopAppConfig from "../../../src-tauri/tauri.conf.json";
import type { NumericalHealthView } from "../results/numericalHealth";
import { reportLineText } from "../results/numericalHealthText";
import { reportResultRows } from "./reportResultRows";
import { USER_RELEASED_REPORT_CONTEXT, type UserReleasedReportContext } from "../redaction-controls/redactionExportControls";
import { checkExactReportAdmission, REPORT_EXACT_ADMISSION_BINDING_MISMATCH, type ExactReportAdmission } from "./exactReportAdmission";
import { exactLimitationEntries, exactRouteCore, isExactReportSource, SECTION_BASIS_HEADLINES } from "./exactReportSections";
import { isPfTimoshenkoReportSource, pfTimoshenkoLimitationEntry, pfTimoshenkoSection } from "./pfTimoshenkoReport";
import { isDisplayedScaled } from "./reportQuantityDisplay";
import { isSpringHangerSupport } from "./springHangerSupport";

// DEC-021 (A7): assemble the renderer input document from session state.
// The frontend composes only references, disclosures, and display rows — the
// Rust renderer crate owns document composition, gating, and hashing. Every
// value here is either copied from session envelopes, computed locally with
// the same canonical-JSON/sha256 discipline as the run hashes, or an explicit
// `TBD` marker that stays visible in the rendered document.

const KNOWN_ANALYSIS_STATUSES = new Set([
  "MODEL_INCOMPLETE",
  "MECHANICS_SOLVED",
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED",
  "HUMAN_REVIEW_REQUIRED"
]);

const SECTION_KINDS = [
  "model_input_summary",
  "load_cases",
  "results",
  "warnings_assumptions_provenance",
  "audit_manifest",
  "rule_pack_references",
  "limitations",
  "professional_boundary_notice"
] as const;

const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false
};

export const DEC018_UNIT_SYSTEM_REF = {
  ref_type: "unit_system",
  ref_id: "unit-system:dec-018-si-dual-display"
};

type Ref = { ref_type: string; ref_id: string };

/** Export context the wrapper renders under. Set only by the wrapper's own
 * route control and intent; it records the outcome in the document and
 * grants no export authority (the redaction route projector decides). */
export type ReportExportContext = "public_report" | "local_private";

/** The rule-check run held by the session for the current result: the loaded
 * rule-pack document and the backend run outcome. Only safe metadata is read
 * from the document (identity, version, checksum, source notice,
 * redistribution status) and only identity, status and missing-input names
 * from the outcomes (DEL-08-01 R-08-01-008). */
export type SessionRuleCheckRun = {
  rulePackDocument: RulePackDocument;
  result: RuleCheckRunResult;
};

export type ReportRenderContext = {
  exportContext: ReportExportContext;
  /** Application clock reading when the render was requested; null when the
   * caller records none. The renderer never reads a clock. */
  generatedAt: string | null;
};

/** The user-released report context (W-18b, DEL-08-01 Design `report-user-release.md` §2.6),
 * the DEL-12-02 vocabulary's single definition (`redactionExportControls.ts`; Rust
 * `report_package::user_release::USER_RELEASED_CONTEXT`), re-exported here. Only the
 * user-released report builder below writes it. */
export { USER_RELEASED_REPORT_CONTEXT };

/** The context a report input is built under: a wrapper context, or the user-released one. */
type BuiltReportContext = { exportContext: ReportExportContext | UserReleasedReportContext; generatedAt: string | null };

const NOT_RECORDED = null;
const REPORT_REDISTRIBUTION_STATUSES = new Set([
  "public_permissive",
  "private_only",
  "unknown",
  "protected_suspected",
  "invented_non_engineering_example",
  "TBD"
]);
const RULE_CHECK_STATUSES = new Set<RuleCheckStatus>([
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED"
]);

export type UnitDisplaySummary = {
  storage_convention: string;
  model_units: Record<string, string>;
  result_units: string[];
  quantity_display_policy: string;
  conversion_performed: boolean;
};

/** The report's display policy (DEL-08-01 Design `report-package-current-results.md` §2.3
 * item 5): the report package's producer requires this text (`report_package::bindings`), and
 * the renderer states the display rule near the results when it reads it. */
export const QUANTITY_DISPLAY_POLICY =
  "display result-row values and stress headlines with explicit units; values stored in Pa are shown in MPa (1 MPa = 10^6 Pa, exact); displayed values are rounded to six significant digits; stored values and units are unchanged";

/** Whether the report displays a value stored in `Pa` (`conversion_performed`, §2.3 item 5):
 * a result row, or, on an exact-route source, a stress headline that is not withheld. The
 * Rust producer's `quantity_display::source_displays_scaled_value` applies the same rule. */
export function displaysScaledValue(result: Pick<MechanicsResult, "results" | "producer" | "summary">): boolean {
  if (result.results.some((item) => isDisplayedScaled(item.unit))) return true;
  if (!isExactReportSource(result as MechanicsResult)) return false;
  const summary = result.summary as unknown as Record<string, { unit?: unknown } | null> | null | undefined;
  return SECTION_BASIS_HEADLINES.some((entry) => isDisplayedScaled(summary?.[entry.key]?.unit));
}

export function buildUnitDisplaySummary(model: PreviewModel, result: MechanicsResult): UnitDisplaySummary {
  return {
    storage_convention: "entered_units_preserved",
    model_units: Object.fromEntries(
      Object.entries(model.project.units).sort(([left], [right]) => left.localeCompare(right))
    ),
    result_units: Array.from(new Set(result.results.map((item) => item.unit).filter(Boolean))).sort(),
    quantity_display_policy: QUANTITY_DISPLAY_POLICY,
    conversion_performed: displaysScaledValue(result)
  };
}

function sessionProvenance(model: PreviewModel) {
  return {
    source_name: `OpenPipeStress desktop session (${model.project.name})`,
    source_location: "local desktop session",
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: "not_asserted",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function mapObjectRef(ref: { object_type: string; ref: string } | undefined, fallbackType: string): Ref {
  if (!ref) return { ref_type: fallbackType, ref_id: "TBD" };
  return { ref_type: ref.object_type, ref_id: ref.ref };
}

/** The report's entry for each result diagnostic, in source order (DEL-08-01 Design
 * `exact-route-report-sections.md` §3.6): the report's first Diagnostics entries. On an exact
 * route the native render and the package's item 9 check them against the delivered result
 * with the Rust mirror `report_package::bindings::expected_report_diagnostics`; parity vectors
 * `fixtures/reports/invented/report_diagnostics_parity.json`. */
export function diagnosticsForSections(result: Pick<MechanicsResult, "diagnostics">, provenance: ReturnType<typeof sessionProvenance>) {
  return result.diagnostics.map((diagnostic, index) => ({
    code: diagnostic.code,
    class:
      diagnostic.severity === "blocking" || diagnostic.severity === "error"
        ? "SOLVE_BLOCKING"
        : "ASSUMPTION_WARNING",
    severity:
      diagnostic.severity === "blocking" || diagnostic.severity === "error"
        ? "blocking"
        : diagnostic.severity,
    source: { ref_type: "diagnostic_source", ref_id: diagnostic.source ?? "core/product_physics" },
    affected_object: {
      ref_type: "affected_refs",
      ref_id: diagnostic.affected_refs?.join(", ") || diagnostic.id || `diagnostic-${index}`
    },
    message: diagnostic.message,
    remediation:
      "Human review required: evaluate this diagnostic against its referenced results before any reliance.",
    provenance
  }));
}

function componentSourceLocation(component: PreviewComponent): string {
  const refs: string[] = [];
  if (component.provenance?.trim()) {
    refs.push(`component.provenance=${component.provenance.trim()}`);
  }
  for (const [key, value] of Object.entries(component.geometry ?? {})) {
    if (typeof value === "string" && key.endsWith("_reference") && value.trim()) {
      refs.push(`geometry.${key}=${value.trim()}`);
    }
  }
  if (component.modifiers?.source_reference?.trim()) {
    refs.push(`modifiers.source_reference=${component.modifiers.source_reference.trim()}`);
  }
  return refs.join("; ") || "component provenance missing";
}

function componentProvenanceRecord(
  component: PreviewComponent,
  base: ReturnType<typeof sessionProvenance>
) {
  const label = component.label?.trim() || component.id;
  return {
    source_name: `${label} component provenance`,
    source_location: componentSourceLocation(component),
    source_license: "user_supplied_or_private",
    contributor: base.contributor,
    contributor_certification:
      "User-local component provenance metadata; redistribution remains private or pending until separately cleared.",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §4.1, FR-REP-003 on
// every route): the provenance of materials, sections, components and supports.

type ReportProvenanceRecord = ReturnType<typeof sessionProvenance>;
type ProvenanceEntity = { provenance_record?: unknown; provenance?: unknown };

function recordObject(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null;
}

/** The one provenance predicate (§4.1): the entity's `provenance_record.source_name` or its
 * `provenance` note is non-empty after trimming. An object-valued note (a section's) counts
 * only when it has a member. */
function hasProvenance(entity: ProvenanceEntity): boolean {
  if (text(recordObject(entity.provenance_record)?.source_name)) return true;
  const note = entity.provenance;
  if (typeof note === "string") return note.trim() !== "";
  const object = recordObject(note);
  return object !== null && Object.keys(object).length > 0;
}

/** A `provenance_record` mapped to the report's record (§4.1): the seven PRD §20.2 fields,
 * each copied when non-empty after trimming, else "not recorded" ("unknown" for
 * redistribution, "pending" for review); `review_status` is the user's source-data review,
 * copied so the screener's blocks keep applying. `reviewed_by`, `reviewed_on`,
 * `verification_note` and `library_ref` are not carried. */
function mappedProvenanceRecord(record: Record<string, unknown>): ReportProvenanceRecord {
  return {
    source_name: text(record.source_name) ?? "not recorded",
    source_location: text(record.source_location) ?? "not recorded",
    source_license: text(record.source_license) ?? "not recorded",
    contributor: text(record.contributor) ?? "not recorded",
    contributor_certification: text(record.contributor_certification) ?? "not recorded",
    redistribution_status: text(record.redistribution_status) ?? "unknown",
    review_status: text(record.review_status) ?? "pending",
    privacy_classification: "private_project_data"
  };
}

/** The disposition members the package save screens at every depth, with the values that
 * block (`src-tauri/src/export_disposition_screen.rs` `SCREENED_MEMBERS`, W-6a's policy). */
const SCREENED_DISPOSITIONS: Readonly<Record<string, readonly string[]>> = Object.freeze({
  redistribution_status: ["protected_suspected", "rejected"],
  review_status: ["quarantined", "rejected"],
  privacy_class: ["protected_suspected", "quarantined", "rejected"],
  privacy_classification: ["protected_suspected", "quarantined", "rejected"],
  classification: ["protected_suspected", "quarantined", "rejected"],
  lifecycle_status: ["quarantined", "rejected"]
});

/** A structured provenance source (a `provenance_record`, or an object- or array-valued
 * note) whose disposition member is an array or an object: the package save refuses it as
 * malformed, and so does the report. */
export const REPORT_PROVENANCE_DISPOSITION_MALFORMED = "REPORT-PROVENANCE-DISPOSITION-MALFORMED";

type FoundDisposition = { path: string; member: string; status: string };

/** Every blocking disposition at any depth of `value` (each object judged by its own
 * members, nothing inherited), and every disposition member that is an array or object. */
function screenDispositions(value: unknown, path: string, found: FoundDisposition[], malformed: string[]): void {
  if (Array.isArray(value)) {
    value.forEach((item, index) => screenDispositions(item, `${path}[${index}]`, found, malformed));
    return;
  }
  const object = recordObject(value);
  if (!object) return;
  for (const [key, child] of Object.entries(object)) {
    const at = `${path}.${key}`;
    const blocking = Object.prototype.hasOwnProperty.call(SCREENED_DISPOSITIONS, key) ? SCREENED_DISPOSITIONS[key] : null;
    if (blocking && typeof child === "string" && blocking.includes(child)) found.push({ path: at, member: key, status: child });
    if (blocking && typeof child === "object" && child !== null) malformed.push(`${at} is ${Array.isArray(child) ? "an array" : "an object"}`);
    screenDispositions(child, at, found, malformed);
  }
}

/** W-11 slice (c) (§6.2): the report record of a structured source keeps the source's
 * blocking dispositions at any depth, so the renderer refuses it as the package save refuses
 * the model: a `quarantined` or `rejected` value anywhere becomes the record's
 * `review_status`, and `protected_suspected` anywhere its `redistribution_status`. A
 * malformed disposition refuses the build, naming the entity and member. */
function withSourceDispositions(record: ReportProvenanceRecord, source: unknown, where: string, root: string): ReportProvenanceRecord {
  const found: FoundDisposition[] = [];
  const malformed: string[] = [];
  screenDispositions(source, root, found, malformed);
  if (malformed.length > 0) {
    throw new Error(`${REPORT_PROVENANCE_DISPOSITION_MALFORMED}: ${where}: ${malformed.join("; ")}; its disposition cannot be judged, so nothing was rendered.`);
  }
  const review = found.find((item) => item.status === "quarantined" || item.status === "rejected");
  const suspected = found.some((item) => item.status === "protected_suspected");
  return {
    ...record,
    ...(review ? { review_status: review.status } : {}),
    ...(suspected ? { redistribution_status: "protected_suspected" } : {})
  };
}

/** The value's review status (§4.1): the record's, except that a missing-data finding whose
 * record says `accepted` carries `pending` (the record keeps `accepted`). */
function valueReviewStatus(record: ReportProvenanceRecord, missing: boolean): string {
  return missing && record.review_status === "accepted" ? "pending" : record.review_status;
}

function componentReportRecord(component: PreviewComponent, base: ReportProvenanceRecord): ReportProvenanceRecord {
  const structured = recordObject(component.provenance_record);
  return structured
    ? withSourceDispositions(mappedProvenanceRecord(structured), structured, `component ${component.id}`, "provenance_record")
    : componentProvenanceRecord(component, base);
}

function componentProvenanceValues(
  model: PreviewModel,
  provenance: ReturnType<typeof sessionProvenance>
) {
  return model.components.map((component) => {
    const record = componentReportRecord(component, provenance);
    const missing = !hasProvenance(component);
    return {
      value_id: `component-provenance:${component.id}`,
      value_category: `component_provenance:${component.kind || "TBD"}`,
      source: { ref_type: "component", ref_id: component.id },
      quantity: null,
      provenance: record,
      privacy_classification: record.privacy_classification,
      required_for: ["reporting", "human_review"],
      review_status: valueReviewStatus(record, missing),
      missing_data_finding: missing
    };
  });
}

function componentProvenanceDiagnostics(
  model: PreviewModel,
  provenance: ReturnType<typeof sessionProvenance>
) {
  return model.components
    .filter((component) => !hasProvenance(component))
    .map((component) => ({
      code: "COMPONENT_PROVENANCE_MISSING",
      class: "PROVENANCE_WARNING",
      severity: "warning",
      source: { ref_type: "component", ref_id: component.id },
      affected_object: { ref_type: "component", ref_id: component.id },
      message: `Component ${component.label || component.id} has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component.`,
      remediation: "Enter component source/provenance before relying on this report.",
      provenance
    }));
}

type ModelMaterial = NonNullable<PreviewModel["materials"]>[number];
type ModelSection = NonNullable<PreviewModel["sections"]>[number];
type ModelSupport = PreviewModel["supports"][number];

/** A material, section or support with its report identity (§4.1). */
type ProvenanceSubject = {
  kind: "material" | "section" | "support";
  id: string;
  label: string;
  entity: ProvenanceEntity;
};

function materialSubjects(model: PreviewModel): ProvenanceSubject[] {
  return (model.materials ?? []).map((material: ModelMaterial) => ({
    kind: "material",
    id: material.id,
    label: text(material.label) ?? material.id,
    entity: material
  }));
}

function sectionSubjects(model: PreviewModel): ProvenanceSubject[] {
  return (model.sections ?? []).map((section: ModelSection) => ({
    kind: "section",
    id: section.id,
    label: text(section.name) ?? section.id,
    entity: section
  }));
}

/** Only the supports that carry a `provenance_record` get a record (§4.1). */
function supportSubjects(model: PreviewModel): ProvenanceSubject[] {
  return model.supports
    .filter((support: ModelSupport) => recordObject(support.provenance_record) !== null)
    .map((support: ModelSupport) => ({
      kind: "support",
      id: support.id,
      label: text(support.label) ?? support.id,
      entity: support
    }));
}

/** The record of a material, section or support (§4.1): its mapped `provenance_record`, or,
 * without one, the component pattern over its `provenance` note (an object-valued section
 * note as its RFC 8785 text). */
async function subjectReportRecord(subject: ProvenanceSubject): Promise<ReportProvenanceRecord> {
  const where = `${subject.kind} ${subject.id}`;
  const structured = recordObject(subject.entity.provenance_record);
  if (structured) return withSourceDispositions(mappedProvenanceRecord(structured), structured, where, "provenance_record");
  const note = subject.entity.provenance;
  const noteText = typeof note === "string"
    ? note.trim()
    : recordObject(note) && Object.keys(recordObject(note)!).length > 0
      ? await canonicalJsonString(note)
      : "";
  const name = subject.kind === "material" ? `${subject.label} material provenance` : `${subject.label} ${subject.kind} provenance`;
  // A structured note is screened before it is flattened into the record's location text.
  return withSourceDispositions({
    source_name: name,
    source_location: noteText ? `${subject.kind}.provenance=${noteText}` : `${subject.kind} provenance missing`,
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: `User-local ${subject.kind} provenance metadata; redistribution remains private or pending until separately cleared.`,
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  }, typeof note === "object" ? note : null, where, "provenance");
}

/** The records, values and diagnostics of the model's materials, sections and supports with
 * a record, in that order and in model order within each (§4.1). */
async function subjectProvenance(model: PreviewModel, provenance: ReportProvenanceRecord) {
  const subjects = [...materialSubjects(model), ...sectionSubjects(model), ...supportSubjects(model)];
  const records = await Promise.all(subjects.map(subjectReportRecord));
  const values = subjects.map((subject, index) => {
    const record = records[index];
    const missing = !hasProvenance(subject.entity);
    return {
      value_id: `${subject.kind}-provenance:${subject.id}`,
      value_category: "report_metadata",
      source: { ref_type: subject.kind, ref_id: subject.id },
      provenance: record,
      privacy_classification: record.privacy_classification,
      required_for: ["reporting", "human_review"],
      review_status: valueReviewStatus(record, missing),
      missing_data_finding: missing
    };
  });
  // Material and section diagnostics only: the design names no support diagnostic; a
  // support record without provenance carries its missing-data flag.
  const diagnostics = subjects
    .filter((subject) => subject.kind !== "support" && !hasProvenance(subject.entity))
    .map((subject) => ({
      code: subject.kind === "material" ? "MATERIAL_PROVENANCE_MISSING" : "SECTION_PROVENANCE_MISSING",
      class: "PROVENANCE_WARNING",
      severity: "warning",
      source: { ref_type: subject.kind, ref_id: subject.id },
      affected_object: { ref_type: subject.kind, ref_id: subject.id },
      message: `${subject.kind === "material" ? "Material" : "Section"} ${subject.label} has neither a provenance_record nor a provenance note; report provenance is incomplete for this ${subject.kind}.`,
      remediation: `Enter ${subject.kind} source/provenance before relying on this report.`,
      provenance
    }));
  return { records, values, diagnostics };
}

/** The model's library references (§4.2): one entry per distinct (kind, library, version,
 * record), in first-occurrence order of the open-time check's walk (`libraryRefSites`), with
 * the owning ids sorted and unique. An absent or empty member is "not recorded". Safe
 * metadata only: no library payload or record snapshot. */
function libraryReferences(model: PreviewModel, provenance: ReportProvenanceRecord) {
  const entries = new Map<string, { library_kind: string; library_id: string; library_version: string; record_id: string; owners: Set<string> }>();
  for (const { entity_ref, reference } of libraryRefSites(model)) {
    const member = (value: unknown) => (typeof value === "string" && value.trim() ? value.trim() : "not recorded");
    const identity = {
      library_kind: member(reference.library_kind),
      library_id: member(reference.library_id),
      library_version: member(reference.library_version),
      record_id: member(reference.record_id)
    };
    const key = JSON.stringify([identity.library_kind, identity.library_id, identity.library_version, identity.record_id]);
    const entry = entries.get(key) ?? { ...identity, owners: new Set<string>() };
    entry.owners.add(entity_ref);
    entries.set(key, entry);
  }
  return Array.from(entries.values()).map(({ owners, ...identity }, index) => ({
    reference_id: `library-ref:${index + 1}`,
    ...identity,
    entity_refs: Array.from(owners).sort(),
    provenance
  }));
}

/** The library store's findings on the model's references (§4.2), as provenance warnings.
 * Resolution is the desktop's existing store check; nothing here resolves anything. */
function libraryFindingDiagnostics(findings: ReadonlyArray<LibraryRefFinding>, provenance: ReportProvenanceRecord) {
  return findings.map((finding) => ({
    code: finding.code,
    class: "PROVENANCE_WARNING",
    severity: "warning",
    source: { ref_type: "diagnostic_source", ref_id: LIBRARY_REF_CHECK_SOURCE },
    affected_object: { ref_type: "model_entity", ref_id: finding.entity_ref },
    message: finding.message,
    remediation: "Compare the library reference with the local library store before relying on this report.",
    provenance
  }));
}

function springHangerSupports(model: PreviewModel) {
  return model.supports.filter(isSpringHangerSupport);
}

function springHangerSourceLocation(support: PreviewModel["supports"][number]): string {
  const refs: string[] = [];
  if (support.provenance?.trim()) refs.push(`support.provenance=${support.provenance.trim()}`);
  if (support.family?.trim()) refs.push(`support.family=${support.family.trim()}`);
  for (const [key, value] of Object.entries(support.hanger ?? {})) {
    if (typeof value === "string" && value.trim()) refs.push(`hanger.${key}=${value.trim()}`);
  }
  return refs.join("; ") || "spring hanger provenance missing";
}

function springHangerProvenanceRecord(
  support: PreviewModel["supports"][number],
  base: ReturnType<typeof sessionProvenance>
) {
  return {
    source_name: `${support.label || support.id} spring-hanger provenance`,
    source_location: springHangerSourceLocation(support),
    source_license: "user_supplied_or_private",
    contributor: base.contributor,
    contributor_certification:
      "User-local spring-hanger provenance metadata; redistribution remains private or pending until separately cleared.",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function springHangerReportQuantity(support: PreviewModel["supports"][number]) {
  const force =
    support.hanger?.constant_load ??
    support.hanger?.hot_load ??
    support.hanger?.cold_load ??
    support.hanger?.installed_load ??
    null;
  if (!force) return null;
  return {
    magnitude: force.value,
    unit: force.unit,
    dimension: "force" as const
  };
}

function springHangerValues(model: PreviewModel, provenance: ReturnType<typeof sessionProvenance>) {
  return springHangerSupports(model).map((support) => {
    const record = springHangerProvenanceRecord(support, provenance);
    const quantity = springHangerReportQuantity(support);
    return {
      value_id: `spring-hanger:${support.id}`,
      value_category: `spring_hanger:${support.hanger?.hanger_type || support.family || "TBD"}`,
      source: { ref_type: "support", ref_id: support.id },
      quantity,
      provenance: record,
      privacy_classification: record.privacy_classification,
      required_for: ["reporting", "human_review"],
      review_status: record.review_status,
      missing_data_finding:
        !support.provenance?.trim() || !support.hanger?.source_reference?.trim() || quantity === null
    };
  });
}

function springHangerDiagnostics(model: PreviewModel, provenance: ReturnType<typeof sessionProvenance>) {
  return springHangerSupports(model)
    .filter((support) => !support.provenance?.trim() || !support.hanger?.source_reference?.trim())
    .map((support) => ({
      code: "SPRING_HANGER_PROVENANCE_MISSING",
      class: "PROVENANCE_WARNING",
      severity: "warning",
      source: { ref_type: "support", ref_id: support.id },
      affected_object: { ref_type: "support", ref_id: support.id },
      message: `Spring hanger ${support.label || support.id} is missing support provenance or hanger.source_reference.`,
      remediation: "Enter spring-hanger source/provenance before relying on this report.",
      provenance
    }));
}

function record(value: unknown): Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function text(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value.trim() : null;
}

/** The input manifest is used only when it is the one the analysis run binds
 * (ref and SHA-256) and its canonical bytes verify; otherwise its settings are
 * reported as not recorded. */
async function boundInputManifest(
  analysisRun: AnalysisRunEnvelope,
  inputManifest: CurrentSessionInputManifestEvidence | null | undefined
): Promise<CurrentSessionInputManifestEvidence | null> {
  if (!inputManifest) return null;
  const reproducibility = analysisRun.analysis_run.reproducibility;
  const refs = reproducibility.input_manifest_refs ?? [];
  const hashes = reproducibility.input_manifest_hashes ?? [];
  if (
    refs.length !== 1 ||
    hashes.length !== 1 ||
    refs[0].ref !== inputManifest.manifest_ref.ref ||
    hashes[0].value !== inputManifest.manifest_sha256
  ) {
    return null;
  }
  try {
    await verifyCurrentSessionInputManifest(inputManifest);
  } catch {
    return null;
  }
  return inputManifest;
}

async function buildReportContext(
  result: MechanicsResult,
  analysisRun: AnalysisRunEnvelope,
  renderContext: BuiltReportContext,
  inputManifest: CurrentSessionInputManifestEvidence | null | undefined
) {
  const run = analysisRun.analysis_run;
  const manifest = await boundInputManifest(analysisRun, inputManifest);
  const solverBasis = manifest?.manifest.solver_basis ?? null;
  const settings = solverBasis
    ? await Promise.all(
        Object.keys(solverBasis.settings)
          .sort()
          .map(async (key) => ({ key, value: await canonicalJsonString(solverBasis.settings[key]) }))
      )
    : [];
  return {
    export_context: renderContext.exportContext,
    generated_at: renderContext.generatedAt,
    software_name: text(desktopAppConfig.productName) ?? "SWBPIPE desktop",
    software_version: text(desktopAppConfig.version),
    // The desktop build records no build/release identifier.
    build_release_identifier: NOT_RECORDED,
    solver_name: text(run.solver_version?.solver_name),
    solver_version: text(run.solver_version?.solver_version),
    solver_build_ref: text(run.solver_version?.build_ref?.ref),
    analysis_run_name: text(run.run_name),
    analysis_run_recorded_at: text(run.created_at),
    solver_mode: text(solverBasis?.solver_mode),
    solver_settings_ref: text(run.settings_ref?.ref),
    solver_settings: settings,
    // The preview model records no model-level coordinate system.
    model_coordinate_system: NOT_RECORDED,
    result_coordinate_systems: Array.from(
      new Set(
        result.results
          .map((item) => item.metadata?.coordinate_system)
          .filter((value): value is string => typeof value === "string" && value.trim() !== "" && value !== "TBD")
      )
    ).sort()
  };
}

export async function rulePackReference(run: SessionRuleCheckRun) {
  const document = record(run.rulePackDocument);
  const metadata = record(document.metadata);
  const classification = record(document.classification);
  const declared = record(record(document.checksums).rule_pack_checksum);
  const declaredValue = text(declared.value);
  const ruleId = run.result.rule_pack_id;
  if (!text(ruleId) || text(metadata.rule_pack_id) !== ruleId) {
    throw new Error("REPORT-RULE-CHECK-PACK-MISMATCH: the rule-check run and the loaded rule pack differ.");
  }
  const redistribution = text(classification.redistribution_status);
  const name = text(metadata.name);
  return {
    rule_pack_id: ruleId,
    // Safe display name (PRD §19.2); omitted when the pack records none, which
    // the renderer discloses as "not recorded".
    ...(name ? { name } : {}),
    version: text(metadata.rule_pack_version) ?? "not recorded",
    checksum: declaredValue
      ? {
          algorithm: text(declared.algorithm) ?? "sha256",
          canonicalization: text(declared.canonicalization) ?? "not recorded",
          payload_ref: { ref_type: "rule_pack", ref_id: ruleId },
          value: declaredValue
        }
      : {
          // No declared checksum: the canonical SHA-256 of the loaded document.
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: { ref_type: "rule_pack_document", ref_id: ruleId },
          value: await canonicalSha256Hex(run.rulePackDocument)
        },
    source_notice: text(metadata.source_notice) ?? "not recorded",
    redistribution_status:
      redistribution && REPORT_REDISTRIBUTION_STATUSES.has(redistribution) ? redistribution : "unknown",
    // Required-input status: incomplete when any check lacks inputs. The
    // aggregate ranks a failed check above an incomplete one, so it cannot be
    // the basis here.
    completeness_status:
      run.result.aggregate_status === "RULE_INPUTS_INCOMPLETE" ||
      run.result.checks.some((check) => check.status === "RULE_INPUTS_INCOMPLETE")
        ? "incomplete"
        : "complete",
    private_payload_redacted: true
  };
}

function userRuleChecks(run: SessionRuleCheckRun) {
  const status = (value: RuleCheckStatus) => {
    if (!RULE_CHECK_STATUSES.has(value)) throw new Error(`REPORT-RULE-CHECK-STATUS-UNKNOWN: ${String(value)}`);
    return value;
  };
  return {
    rule_pack_id: run.result.rule_pack_id,
    grammar_version: run.result.grammar_version,
    aggregate_status: status(run.result.aggregate_status),
    // One outcome per check and load case or combination (rule-pack format
    // 0.5.0; DEL-06-01 Design `rule-pack-0.5.0.md` §2.7, §6): each carries
    // the basis it was evaluated on. An outcome without one (a check the pack
    // gives no basis) omits the member, and the report shows "load case not
    // recorded".
    checks: run.result.checks.map((check) => ({
      check_id: check.check_id,
      ...(check.basis_ref ? { basis_ref: { ref_type: check.basis_ref.ref_type, ref_id: check.basis_ref.ref_id } } : {}),
      status: status(check.status),
      missing_inputs: Array.from(
        new Set(check.bound_inputs.filter((input) => !input.supplied).map((input) => input.input_id))
      ).sort()
    }))
  };
}

type RenderableReportInputArgs = {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  projectSummary: LocalProjectSummary | null;
  renderContext: ReportRenderContext;
  inputManifest?: CurrentSessionInputManifestEvidence | null;
  ruleCheckRun?: SessionRuleCheckRun | null;
  /** DEL-07-05 numerical health §5: the rendered report's one line per load case,
   * placed after the result rows and so before the diagnostics section, on the generic
   * route only. Absent, the input is unchanged (the report package passes none). No
   * figure is exported. */
  numericalHealth?: NumericalHealthView | null;
  /** W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.1): the recorded Export
   * Result JSON admission of an exact-route source (`admitExactReportSource`), required
   * for an exact source and refused for any other. A pressure-free source takes none, so
   * its input is unchanged. */
  exactAdmission?: ExactReportAdmission | null;
  /** W-11 slice (c) (§4.2): the library store's findings that still apply to `model`; the
   * callers pass `currentLibraryRefFindings(model)`. Each becomes a provenance warning. */
  libraryRefFindings?: ReadonlyArray<LibraryRefFinding>;
};

export async function buildRenderableReportInput(args: RenderableReportInputArgs) {
  return renderableReportInput(args, null);
}

/** The user-released record as the report context carries it (W-18b §2.3). Its form and
 * bound members are checked natively; the builder carries it as given. */
export type UserReleaseRecordValue = Record<string, unknown>;

/** The minimal renderer input of a user-released report (W-18b, DEL-08-01 Design
 * `report-user-release.md` §2.6): the local-private builder's title, model summary, load
 * cases, result rows, the solve's own diagnostics, status disclosures, technical-preview
 * limitation and run context, under `user_released_report` with `generated_at` null and the
 * record `userRelease`. It leaves out every component, spring-hanger, material, section and
 * support provenance entry, the library references and library-store findings, every
 * provenance diagnostic, rule-pack references and user-rule outcomes; its persistence
 * reference is `TBD`. D-6 exact sources require the recorded W-11 admission and
 * retain their exact sections and limitations. The Rust mirror is
 * `report_package::user_release::minimal_renderer_input`, held equal by
 * `fixtures/reports/invented/user_release_derivation_vectors.json`. */
export async function userReleasedRenderableReportInput({
  model,
  result,
  analysisRun,
  inputManifest,
  userRelease,
  exactAdmission = null
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  inputManifest: CurrentSessionInputManifestEvidence;
  userRelease: UserReleaseRecordValue;
  exactAdmission?: ExactReportAdmission | null;
}) {
  return renderableReportInput(
    { model, result, analysisRun, projectSummary: null, inputManifest, exactAdmission, renderContext: { exportContext: "local_private", generatedAt: null } },
    userRelease
  );
}

/** The one builder: `userRelease` null builds the wrapper contexts' input exactly as before;
 * a record builds the minimal user-released input (above). */
async function renderableReportInput(
  {
    model,
    result,
    analysisRun,
    projectSummary,
    renderContext,
    inputManifest = null,
    ruleCheckRun = null,
    numericalHealth = null,
    exactAdmission = null,
    libraryRefFindings = []
  }: RenderableReportInputArgs,
  userRelease: UserReleaseRecordValue | null
) {
  const minimal = userRelease !== null;
  const exact = isExactReportSource(result);
  // DEL-05-03 `pf-timoshenko.md` §4: a pressure-free Timoshenko source carries its
  // limitation 0 entry always, and its `pf_timoshenko` section when it took the admission
  // (the rendered report; the package binds the entry against its source instead).
  const pfTimoshenko = isPfTimoshenkoReportSource(result);
  if (!exact && !pfTimoshenko && exactAdmission !== null) {
    throw new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`);
  }
  const run = analysisRun.analysis_run;
  // The exact path's admission is checked at the start and again after the last await
  // (`checkExactReportAdmission`): retirement, an edit or a substitute refuses.
  const exactInputs = exact ? { model, result, analysisRun, inputManifest: inputManifest as CurrentSessionInputManifestEvidence } : null;
  if (exactInputs) checkExactReportAdmission(exactAdmission, exactInputs);
  const pfInputs = pfTimoshenko && exactAdmission !== null ? { model, result, analysisRun, inputManifest: inputManifest as CurrentSessionInputManifestEvidence } : null;
  if (pfInputs) checkExactReportAdmission(exactAdmission, pfInputs);
  const provenance = sessionProvenance(model);
  // The minimal input carries no per-object provenance entry, library reference or finding.
  const subjects = minimal ? { records: [], values: [], diagnostics: [] } : await subjectProvenance(model, provenance);
  const libraryRefs = minimal ? [] : libraryReferences(model, provenance);
  const modelHashValue = await canonicalSha256Hex(model);
  const runRecordHash = run.hashes.find((item) => item.payload_scope === "analysis_run_record");
  const receivedResultScope = analysisResultHashScope(analysisRun.schema_version);
  if (receivedResultScope === null) throw new Error("REPORT-ANALYSIS-VERSION-UNSUPPORTED");
  const resultEnvelopeHash = run.hashes.find((item) => item.payload_scope === receivedResultScope);

  const analysisStatus = Array.from(
    new Set(
      [...run.analysis_status, "HUMAN_REVIEW_REQUIRED"].filter((status) =>
        KNOWN_ANALYSIS_STATUSES.has(status)
      )
    )
  ).sort();

  const reportSections = {
    report_section_id: `sections:${result.run_id}`,
    model_ref: { ref_type: "model", ref_id: result.model_ref },
    run_ref: { ref_type: "analysis_run", ref_id: result.run_id },
	    diagnostics: [
	      ...diagnosticsForSections(result, provenance),
	      ...(minimal ? [] : componentProvenanceDiagnostics(model, provenance)),
	      ...(minimal ? [] : springHangerDiagnostics(model, provenance)),
	      ...subjects.diagnostics,
	      ...(minimal ? [] : libraryFindingDiagnostics(libraryRefFindings, provenance))
	    ],
    analysis_status_disclosures: [
      ...analysisStatus
        .filter((status) => status !== "HUMAN_REVIEW_REQUIRED")
        .map((status) => ({
          status,
          source: { ref_type: "analysis_run", ref_id: result.run_id },
          affected_object: { ref_type: "model", ref_id: result.model_ref },
          explanation: `Automatic analysis status reported by the run record: ${status}.`,
          human_review_required: true,
          human_acceptance_ref: null
        })),
      {
        status: "HUMAN_REVIEW_REQUIRED",
        source: { ref_type: "report_renderer", ref_id: `report:${result.run_id}` },
        affected_object: { ref_type: "report", ref_id: `report:${result.run_id}` },
        explanation: "Human professional review is required before any reliance on this report.",
        human_review_required: true,
        human_acceptance_ref: null
      }
    ],
	    provenance_notes: [
	      provenance,
	      ...(minimal ? [] : model.components.map((component) => componentReportRecord(component, provenance))),
	      ...(minimal ? [] : springHangerSupports(model).map((support) => springHangerProvenanceRecord(support, provenance))),
	      ...subjects.records
	    ],
	    user_supplied_values: [
	      ...(minimal ? [] : componentProvenanceValues(model, provenance)),
	      ...(minimal ? [] : springHangerValues(model, provenance)),
	      ...subjects.values
	    ],
    assumptions: [],
    limitations: [
      {
        limitation_id: `limitation:${result.run_id}:technical-preview`,
        source: { ref_type: "report_renderer", ref_id: `report:${result.run_id}` },
        affected_scope: { ref_type: "model", ref_id: result.model_ref },
        statement:
          "Output over invented or user-local data; not validated engineering output.",
        effect: {
          mechanics_solve_qualified: true,
          user_rule_check_qualified: false,
          report_completeness: "qualified",
          human_review_required: true
        },
        provenance
      }
    ],
    unresolved_tbds: run.reproducibility.unresolved_tbd.map((description, index) => ({
      tbd_id: `tbd:${result.run_id}:${index + 1}`,
      affected_scope: { ref_type: "analysis_run", ref_id: result.run_id },
      description,
      review_needed: true
    })),
    professional_boundary: PROFESSIONAL_BOUNDARY
  };

  // W-11 (§2.4): an exact source's limitation entries, one per published producer
  // string with the closed table's statement, then the profile reference, after the
  // technical-preview entry (which stays first), sharing its effect and provenance.
  if (exactInputs) {
    const semanticContract = run.reproducibility.semantic_contract;
    if (!semanticContract) throw new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`);
    const [preview] = reportSections.limitations;
    const entries = await exactLimitationEntries(result, result.run_id, { id: semanticContract.id, sha256: semanticContract.sha256 });
    reportSections.limitations.push(...entries.map((entry) => ({ ...entry, effect: preview.effect, provenance: preview.provenance })) as never[]);
  }
  if (pfTimoshenko) {
    const [preview] = reportSections.limitations;
    reportSections.limitations.push({ ...pfTimoshenkoLimitationEntry(result, result.run_id), effect: preview.effect, provenance: preview.provenance } as never);
  }

  const sectionsChecksumValue = await canonicalSha256Hex(reportSections);

  const checksum = (payloadRef: Ref, value: string | undefined) => ({
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
    payload_ref: payloadRef,
    value: value || "TBD"
  });

  const envelope = (ref: Ref, schemaId: string, checksumValue: string | undefined) => ({
    ref,
    schema_ref: { ref_type: "schema", ref_id: schemaId },
    checksum: checksum(ref, checksumValue),
    privacy_classification: "private_project_data",
    provenance
  });

  const calculationReport = {
    report_id: `report:${result.run_id}`,
    model_input_summary: {
      project_ref: { ref_type: "project", ref_id: model.project.id },
      model_ref: { ref_type: "model", ref_id: result.model_ref },
      persistence_ref: {
        ref_type: "project_persistence",
        ref_id: projectSummary ? `local_sqlite:${projectSummary.project_id}` : "TBD"
      },
      unit_system_ref: DEC018_UNIT_SYSTEM_REF,
      unit_display_summary: buildUnitDisplaySummary(model, result),
      model_hash: checksum({ ref_type: "model", ref_id: model.project.id }, modelHashValue),
      input_manifest_ref: mapObjectRef(run.reproducibility.input_manifest_refs[0], "audit_manifest"),
      provenance
    },
    load_case_summary: model.load_cases.map((loadCase) => ({
      load_ref: { ref_type: "load_case", ref_id: loadCase.id },
      label: loadCase.label,
      basis: loadCase.kind,
      source: { ref_type: "model", ref_id: model.project.id },
      provenance
    })),
    result_export_refs: run.result_refs.map((item) =>
      envelope(
        { ref_type: item.result_ref.object_type, ref_id: item.result_ref.ref },
        "schemas/results.schema.yaml",
        item.hash_refs[0]?.value
      )
    ),
    audit_manifest_refs: [
      envelope(
        { ref_type: "analysis_run", ref_id: result.run_id },
        "schemas/analysis_run.schema.yaml",
        runRecordHash?.value
      ),
      envelope(
        { ref_type: "result_envelope", ref_id: `result-envelope:${result.run_id}` },
        "schemas/results.schema.yaml",
        resultEnvelopeHash?.value
      )
    ],
    report_section_refs: [
      envelope(
        { ref_type: "report_sections", ref_id: reportSections.report_section_id },
        "schemas/report_sections.schema.yaml",
        sectionsChecksumValue
      )
    ],
    rule_pack_refs: ruleCheckRun ? [await rulePackReference(ruleCheckRun)] : [],
    diagnostics: [],
    template_slots: SECTION_KINDS.map((kind, index) => ({
      slot_id: `slot-${kind}`,
      required: true,
      section_kind: kind,
      source_contract: { ref_type: "contract", ref_id: "DEL-08-01" },
      ordering_index: index + 1
    })),
    rendered_sections: SECTION_KINDS.map((kind) => ({
      section_id: `section-${kind}`,
      slot_id: `slot-${kind}`,
      section_kind: kind,
      title: kind.replaceAll("_", " "),
      source_refs: [{ ref_type: "model", ref_id: model.project.id }],
      content_status: "rendered"
    })),
    analysis_status: analysisStatus,
    professional_boundary: PROFESSIONAL_BOUNDARY,
    provenance,
    privacy_classification: "private_project_data",
    unresolved_runtime_tbds: run.reproducibility.unresolved_tbd.map((description, index) => ({
      tbd_id: `tbd:${result.run_id}:${index + 1}`,
      topic: "analysis run reproducibility",
      affected_scope: { ref_type: "analysis_run", ref_id: result.run_id },
      description,
      review_needed: true
    }))
  };

  const reportContext = minimal
    ? { ...(await buildReportContext(result, analysisRun, { exportContext: USER_RELEASED_REPORT_CONTEXT, generatedAt: null }, inputManifest)), user_release: userRelease }
    : await buildReportContext(result, analysisRun, renderContext, inputManifest);
  // W-11 (§3.2-§3.3): the exact-route result block: the identity, the admission binding
  // (the renderer composes the standing line from it), the load/reference line and the
  // headlines, nominal then reduced.
  const exactRoute = exactInputs
    ? await (async () => {
        const core = await exactRouteCore(result);
        return {
          semantic_contract_id: core.semantic_contract_id,
          admission: { ...exactAdmission!.binding },
          load_reference_states: core.load_reference_states,
          headlines: core.headlines
        };
      })()
    : null;
  // After the last await: the admission still holds for these live objects.
  if (exactInputs) checkExactReportAdmission(exactAdmission, exactInputs);
  if (pfInputs) checkExactReportAdmission(exactAdmission, pfInputs);

  return {
    report_title: `${model.project.name} — Calculation Report`,
    calculation_report: calculationReport,
    report_sections: reportSections,
    // W-11 (DEL-08-01 Design `exact-route-report-sections.md` §6.1): an exact-route
    // report carries no numerical-health line until the native render can re-derive it
    // from the delivered result; its rows are exactly the source rows' projection.
    result_rows: reportResultRows(result).concat((numericalHealth?.route === "generic" && !exactInputs ? numericalHealth.cases : []).map((item) => ({
      row_id: `numerical-health:${item.caseId}`,
      label: reportLineText(item.state),
      case_ref: `load_case:${item.caseId}`,
      quantity_display: "",
      source_ref: item.integrityDiagnosticId ?? "numerical_quality"
    }))),
    report_context: reportContext,
    // The minimal input carries no user-rule outcome member at all (§2.6).
    ...(minimal ? {} : { user_rule_checks: ruleCheckRun ? userRuleChecks(ruleCheckRun) : null }),
    ...(exactRoute ? { exact_route: exactRoute } : {}),
    // DEL-05-03 `pf-timoshenko.md` §4: absent on every other source, so its input is unchanged.
    ...(pfInputs ? { pf_timoshenko: pfTimoshenkoSection(exactAdmission!.binding) } : {}),
    // W-11 slice (c) (§4.2): absent without references, so such an input is unchanged.
    ...(libraryRefs.length > 0 ? { library_references: libraryRefs } : {})
  };
}
