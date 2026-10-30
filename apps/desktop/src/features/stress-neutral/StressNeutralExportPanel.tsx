import { LoadReferenceDisclosure, loadReferenceDisclosure } from "../results/loadReferenceDisclosure";
import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, isRetainedRoute, type SourceContract } from "../results/numericalResultQuality";
import { currentSemanticContract, hasCurrentSourceContract, readerRowSemantics, readerTransport, readerValidate, sourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { refuseSurfaceOutput, refuseSurfaceRoute, surfaceOutputRefusal } from "../results/outputPolicy";
import { RETAINED_ABSOLUTE_VERIFIED, retainedClassDisclosure, retainedPrecisionSummaryLine, retainedRowClassesFromReader } from "../results/retainedPrecisionDisclosure";
import { validateRetainedPrecisionTransport } from "../results/retainedPrecisionReader";
import type { RowClassification } from "../results/retainedPrecisionTypes";
import { verifyAnalysisRunRecord, validateAnalysisRunV03, analysisRowSemantics, sourceBasisReference, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { Download, FileJson } from "lucide-react";
import { useEffect, useState } from "react";
import { canonicalSha256HexCheckedV1, canonicalJsonCheckedV1, checkedJsonText } from "../../services/hashService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, ObjectRef } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";
import { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, recordRulePackRefAnnotations, rulePackRefAnnotationsToAdd } from "../../services/rulePackRefDiagnostics";

const STRESS_NEUTRAL_EXPORT_VERSION = "0.3.0";
const STRESS_NEUTRAL_EXPORT_PROFILE = "ops.stress_neutral.v3";
const CSV_COLUMNS = [
  "result_id",
  "canonical_ref",
  "row_kind",
  "result_family",
  "load_case_ref",
  "station_ref",
  "component_ref",
  "value",
  "unit",
  "dimension",
  "correlation_status"
] as const;

type CsvColumn = (typeof CSV_COLUMNS)[number];

type StressNeutralRef = {
  object_type: string;
  ref: string;
};

type StressNeutralRow = {
  result_id: string;
  canonical_ref: StressNeutralRef;
  row_kind: "result_value";
  result_family: string;
  load_case_ref: StressNeutralRef;
  station_ref: StressNeutralRef;
  component_ref: StressNeutralRef;
  value: number;
  unit: string;
  dimension: string;
  correlation_status: "canonical_id_map" | "unit_or_dimension_blocking_review_required";
  source_result_ref: StressNeutralRef;
  provenance: ReturnType<typeof previewProvenance>;
};

// Same small publication binding as the source-recovery reader: private live
// invocation plus immutable checked contents, never a serializable proof flag.
// Exported so that its explicit output gate is tested directly (U7 slice T).
export function liveStressBinding(model: PreviewModel, result: MechanicsResult | null, analysisRun: AnalysisRunEnvelope | null): string | null {
  try {
    if (!result || !analysisRun || !hasNativeMechanicsInvocation(result, model)
      // T6S-3 (RR decision 2): an explicit gate on this surface's entry in the output
      // policy; a retained-precision successor is admitted only at eligible standing.
      || !hasCurrentSourceContract(result) || surfaceOutputRefusal(result, model, "stress-neutral") !== null || !numericalResultStanding(result, model).eligible) return null;
    return checkedJsonText({ model, result, analysisRun });
  } catch { return null; }
}

export function StressNeutralExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const [packet, setPacket] = useState<Awaited<ReturnType<typeof buildStressNeutralExportPacket>> | null>(null);
  const [binding, setBinding] = useState<[PreviewModel, MechanicsResult, AnalysisRunEnvelope] | null>(null);
  const [publicationFingerprint, setPublicationFingerprint] = useState<string | null>(null);
  const currentFingerprint = liveStressBinding(model, result, analysisRun);
  const outputRefusal = surfaceOutputRefusal(result, model, "stress-neutral");
  const retainedSummary = retainedPrecisionSummaryLine(result, model);
  // Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §5.4): shown only while the live
  // binding admits the result.
  const loadReferenceLine = currentFingerprint !== null ? loadReferenceDisclosure(result, "stress-neutral") : null;
  const currentPacket = binding?.[0] === model && binding?.[1] === result && binding?.[2] === analysisRun
    && currentFingerprint !== null && currentFingerprint === publicationFingerprint
    ? packet
    : null;
  useEffect(() => {
    let current = true;
    setPacket(null);
    setBinding(null);
    setPublicationFingerprint(null);
    const capturedFingerprint = liveStressBinding(model, result, analysisRun);
    if (result && analysisRun && capturedFingerprint !== null) buildStressNeutralExportPacket({ model, result, analysisRun }).then((built) => {
      if (current && liveStressBinding(model, result, analysisRun) === capturedFingerprint) {
        setPacket(built);
        setBinding([model, result, analysisRun]);
        setPublicationFingerprint(capturedFingerprint);
      }
    }).catch(() => { if (current) { setPacket(null); setBinding(null); } });
    return () => { current = false; };
  }, [model, result, analysisRun]);
  return (
    <section
      className="panel stress-neutral-export-panel"
      aria-label="Stress-neutral CSV JSON export"
      data-testid="stress-neutral-panel"
    >
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Stress-neutral CSV/JSON
      </div>
      <KnownSemanticNotices result={result} testIdPrefix="stress-neutral" />
      {retainedSummary ? <p className="muted" data-testid="stress-neutral-retained-precision-summary">{retainedSummary}</p> : null}
      <LoadReferenceDisclosure text={loadReferenceLine} surface="stress-neutral" />
      {currentPacket ? (
        <>
          <div className="report-actions" onClickCapture={(event) => {
            // A same-object edit may not render before JSON or CSV activation.
            if (liveStressBinding(model, result, analysisRun) !== publicationFingerprint) {
              event.preventDefault(); event.stopPropagation();
              setPacket(null); setBinding(null); setPublicationFingerprint(null);
            }
          }}>
        <ControlledExportLink
              className="report-export-link"
              data-testid="stress-neutral-export-link"
              nativeCurrentBinding={binding}
              download={`openpipestress-preview-stress-neutral-${safeFileToken(currentPacket.source_result_ref.ref)}.json`}
              href={jsonDataHref(currentPacket)}
              validateDecodedPayload={(payload) => validateDesktopStressNeutralExportPacket(payload, binding?.[1], binding?.[2], binding ? modelLoadBasisRefs(binding[0]) : undefined)}
            >
              <Download size={14} aria-hidden="true" />
              Package JSON
        </ControlledExportLink>
        <ControlledExportLink
              className="report-export-link"
              data-testid="stress-neutral-csv-link"
              download={`openpipestress-preview-stress-neutral-${safeFileToken(currentPacket.source_result_ref.ref)}.csv`}
              href={csvDataHref(currentPacket.csv_text)}
            >
              <Download size={14} aria-hidden="true" />
              CSV
        </ControlledExportLink>
            <span data-testid="stress-neutral-summary">
              available; rows={currentPacket.result_rows.length}; csv_columns={currentPacket.export_profile.csv_columns.length};
              diagnostics={currentPacket.diagnostics.length}
            </span>
          </div>
          <div className="report-list" data-testid="stress-neutral-body">
            <StressNeutralLine
              label="Format"
              value={`${currentPacket.export_profile.target_family}; profile=${currentPacket.export_profile.profile_id}; schema=${currentPacket.schema_version}`}
              testId="stress-neutral-format"
            />
            <StressNeutralLine
              label="State binding"
              value={`${currentPacket.source_model_ref.ref}; ${currentPacket.source_run_ref.ref}; ${currentPacket.source_result_ref.ref}`}
              testId="stress-neutral-state-binding"
            />
            <StressNeutralLine
              label="Units"
              value={`${unitCount(currentPacket)} explicit units; dimensions=${dimensionSummary(
                currentPacket
              )}; ${unitDisclosureSummary(currentPacket.unit_system_disclosure)}`}
              testId="stress-neutral-units"
            />
            <StressNeutralLine
              label="Unit witnesses"
              value={`count=${currentPacket.unit_preservation_witnesses.length}; policy=preserve_source_result_units; conversion=false`}
              testId="stress-neutral-unit-witnesses"
            />
            <StressNeutralLine
              label="Package"
              value={`members=${currentPacket.manifest.package_members.length}; stable_ids=${currentPacket.stable_id_map.length}; loss_entries=${currentPacket.loss_report.length}; validation=${currentPacket.validation_report.validation_status}; package_hash=checked`}
              testId="stress-neutral-package"
            />
            <StressNeutralLine
              label="Boundary"
              value={`vendor_format=${String(packet.professional_boundary.software_makes_external_compatibility_claim)}; solver_validation=${String(
                packet.professional_boundary.software_makes_solver_validation_claim
              )}; code_compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; professional_reliance=${String(
                packet.professional_boundary.software_creates_professional_reliance_record
              )}`}
              testId="stress-neutral-boundary"
            />
          </div>
        </>
      ) : outputRefusal ? (
        <p className="muted" role="status" data-testid="stress-neutral-load-reference-output-unavailable">{outputRefusal}</p>
      ) : (
        <p className="muted" data-testid="stress-neutral-empty">
          Run mechanics with the native backend to assemble a stress-neutral CSV/JSON package. Bundled references and restored history are unavailable for qualified export.
        </p>
      )}
      <small className="report-note">
        Stress-neutral export is a local review package only; it is not a vendor format or solver input deck. Validation
        occurs in the user's accepted professional tools; this package is screening and handoff evidence.
      </small>
    </section>
  );
}

function StressNeutralLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

/** The package's own comparison-semantics diagnostic, the first of its diagnostics. */
function comparisonSemanticsDiagnostic(exportId: string, packetProvenance: ReturnType<typeof previewProvenance>) {
  return { code: "SN-DESKTOP-PREVIEW-COMPARISON-TBD", class: "stress_neutral_export", severity: "warning",
    source: reference("StressNeutralExportPackage", exportId), affected_object: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
    message: "Comparison pass/fail semantics and tolerance profiles are not defined by this preview export.",
    remediation: "Review stress-neutral handoff evidence before use.", provenance: packetProvenance };
}

/** Version-neutral inputs consumed by the current 0.3 packet construction. */
function normalizeStressNeutralInputs({ model, result, analysisRun }: { model: PreviewModel; result: MechanicsResult; analysisRun: AnalysisRunEnvelope }) {
  const run = analysisRun.analysis_run;
  const resultRows = result.results.slice()
    .sort((left, right) => scalarIdCompare(left.id, right.id))
    .map((item) => precisionStressRow(item, result));
  const csvText = renderCsv(resultRows);
  validateStressNeutralCsv(csvText, resultRows);
  const exportId = `stress-neutral:${safeFileToken(result.run_id)}`;
  const stableIdMap = resultRows.map((row, index) => ({
    canonical_ref: row.canonical_ref,
    export_ref: reference("StressNeutralResultRow", `stress-neutral-row:${index}`),
    mapping_status: "mapped", loss_category: "exported", provenance: previewProvenance()
  }));
  // Preserve the existing normalization check directly on its input records.
  // No obsolete witness format is constructed, and a received declaration does
  // not supply the reader's physical interpretation or witness eligibility.
  const declarationsMatchRows = resultRows.filter(row => {
    const original = result.results.find(item => item.id === row.result_id);
    return typeof original?.dimension === "string" && original.dimension === row.dimension;
  }).every(row => {
    const received = resultRows.find(item => item.result_id === row.source_result_ref.ref);
    return Boolean(received && row.source_result_ref.ref === received.source_result_ref.ref
      && `stress-neutral-row:${safeFileToken(row.result_id)}` === `stress-neutral-row:${safeFileToken(received.result_id)}`
      && row.value === received.value && row.unit === received.unit && row.dimension === received.dimension);
  });
  return {
    export_id: exportId,
    source_model_ref: reference("Model", model.project.id), source_run_ref: reference("AnalysisRun", run.run_id),
    source_result_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
    export_profile: {
      target_family: "stress_neutral_csv_json", csv_columns: CSV_COLUMNS,
      identity_policy: "canonical_ref_per_row_plus_stable_id_map", unit_policy: "unit_and_dimension_required_per_row",
      loss_report_policy: "mandatory_for_every_package", comparison_semantics: "diagnostic_export_only_no_pass_fail"
    },
    unit_system_disclosure: buildExportUnitSystemDisclosure({ model, result, targetExportUnits: {},
      conversionPolicy: "result_row_units_preserved_no_export_time_conversion", conversionPerformed: false, conversionScope: [],
      sourceLocation: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx" }),
    result_rows: resultRows, csv_text: csvText,
    stable_id_map: stableIdMap,
    loss_entries: [
      { loss_id: "loss:desktop-preview:stress-neutral-exported", category: "exported", severity: "info",
        affected_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
        downstream_implication: "Local review evidence only; validation/loss findings require review before downstream reliance." },
      { loss_id: "loss:desktop-preview:comparison-semantics-tbd", category: "tbd", severity: "warning",
        affected_ref: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
        reason: "Pass/fail, tolerances, and external target interpretation are outside this preview package.",
        downstream_implication: "Consumers must not infer acceptance, equivalence, or compliance from this export." },
      { loss_id: "loss:desktop-preview:private-rule-payload-redacted", category: "omitted", severity: "warning",
        affected_ref: reference("RulePack", "rule-pack:user-supplied:not-loaded"),
        reason: "Private rule criteria and compliance inputs are not bundled in the public build.",
        downstream_implication: "Completeness and code-rule interpretation require separate governed review." }
    ],
    manifest_id: `manifest:${safeFileToken(result.run_id)}:stress-neutral`,
    validation_checks: [
      check("csv_json_row_sync", true),
      check("canonical_ref_per_row", resultRows.every((row) => Boolean(row.canonical_ref.ref))),
      check("unit_and_dimension_per_row", resultRows.every((row) => Boolean(row.unit && row.dimension && row.dimension !== "TBD"))),
      { check_id: "unit_preservation_witness_per_row" },
      check("unit_preservation_witnesses_match_rows", declarationsMatchRows),
      check("stable_id_map_per_row", stableIdMap.length === resultRows.length),
      check("loss_report_present", true)
    ] as Array<{ check_id: string; status?: string; blocking?: boolean }>,
    diagnostics: [comparisonSemanticsDiagnostic(exportId, previewProvenance())],
    boundary_notes: [
      "The package is not a vendor format, solver input deck, or external target compatibility claim.",
      "Comparison pass/fail and tolerances remain outside this export; validation occurs in the user's accepted professional tools, and this package is screening and handoff evidence."
    ],
    privacy_classification: "public_metadata_and_invented_results"
  };
}

const STRICT_MEMBER_NAMES = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"] as const;

function validUtf8Text(text: string): boolean {
  // TextEncoder alone replaces unpaired surrogates. A strict round trip refuses
  // that loss; code points are never normalized or transliterated.
  return new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(new TextEncoder().encode(text)) === text;
}
function scalarIdCompare(left: string, right: string): number {
  if (!validUtf8Text(left) || !validUtf8Text(right)) throw new Error('SN-CSV-UTF8-INVALID');
  const a = Array.from(left, ch => ch.codePointAt(0)!), b = Array.from(right, ch => ch.codePointAt(0)!);
  for (let i = 0; i < Math.min(a.length, b.length); i++) if (a[i] !== b[i]) return a[i] < b[i] ? -1 : 1;
  return a.length - b.length;
}

async function rawTextSha256(text: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function checksumRef(filename: string) {
  return { object_type: "StressNeutralMember", ref: filename };
}

async function checkedMemberChecksum(filename: string, value: unknown, payloadScope = "member_payload") {
  return { algorithm: "sha256" as const, canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: payloadScope, payload_ref: checksumRef(filename), value: await canonicalSha256HexCheckedV1(value) };
}

function strictManifestSeed(packet: any, checksums: any[]) {
  return {
    manifest_id: packet.manifest.manifest_id,
    source_result_ref: packet.source_result_ref,
    source_run_ref: packet.source_run_ref,
    source_model_ref: packet.source_model_ref,
    received_source_checksums: packet.received_source_checksums,
    unresolved_assumption_refs: packet.unresolved_assumption_refs,
    reproducibility_refs: packet.reproducibility_refs,
    export_profile_ref: packet.manifest.export_profile_ref,
    boundary_notes: packet.manifest.boundary_notes,
    member_checksums: checksums,
    diagnostics: packet.diagnostics,
  };
}

type StrictWitnessDisposition = "eligible" | "diagnostic_work" | "unknown_semantic" | "missing_semantic" | "contradiction"
  | "retained_absolute_verified" | "retained_not_covered";

const WITHHOLDING_CODES: Record<Exclude<StrictWitnessDisposition, "eligible">, string> = {
  diagnostic_work: "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK",
  unknown_semantic: "SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC",
  missing_semantic: "SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC",
  contradiction: "SN-UNIT-WITNESS-WITHHELD-CONTRADICTION",
  // T6S-5 (S-d; RR decision 5; D2 4.9.9; D-U6-2): a retained-precision successor row
  // whose validated class is absolute_verified or not_covered keeps its CSV row and
  // value, and its unit-preservation witness is withheld with one of these codes.
  retained_absolute_verified: "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED",
  retained_not_covered: "SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-NOT-COVERED"
};
/** Withholdings that are information, not a block on the package's rows. */
const INFO_WITHHOLDINGS: ReadonlySet<string> = new Set(["diagnostic_work", "retained_absolute_verified", "retained_not_covered"]);
/** Withholdings that block, and so keep precedence over a retained-precision class. */
const BLOCKING_WITHHOLDINGS: ReadonlySet<string> = new Set(["unknown_semantic", "missing_semantic", "contradiction"]);

type StrictWitnessInterpretation = { disposition: StrictWitnessDisposition; dimension: string | null; message?: string };
/** The row's witness disposition. For a successor, the validated class from the
 * accepted reader (`classes`) withholds the witness of an absolute_verified or
 * not_covered row with D-U6-2's message, exactly as Rust's derivative states it.
 * A blocking disposition keeps precedence (fail safe); the class takes precedence
 * over diagnostic work, as it does in Rust's derivative. Exported for the
 * precedence test only. */
export function strictWitnessDisposition(source: MechanicsResult["results"][number], carrier: MechanicsResult, classes: ReadonlyMap<string, RowClassification> | null = null): StrictWitnessInterpretation {
  const ordinary = ordinaryWitnessDisposition(source, carrier);
  if (classes === null || BLOCKING_WITHHOLDINGS.has(ordinary.disposition)) return ordinary;
  const disclosure = retainedClassDisclosure(source.kind, source.unit, classes.get(source.id));
  if (!disclosure) return ordinary;
  return { disposition: disclosure.code === RETAINED_ABSOLUTE_VERIFIED ? "retained_absolute_verified" : "retained_not_covered", dimension: ordinary.dimension, message: disclosure.message };
}
function ordinaryWitnessDisposition(source: MechanicsResult["results"][number], carrier: MechanicsResult): StrictWitnessInterpretation {
  try {
    const semantic = hasCurrentSourceContract(carrier) ? analysisRowSemantics(source, carrier).semantic : readerRowSemantics(carrier)(source).signature;
    if (!semantic) return { disposition: "unknown_semantic", dimension: null };
    if (semantic.category === "diagnostic_work") return { disposition: "diagnostic_work", dimension: semantic.derivative_target_dimension };
    if (semantic.component !== null && (!source.metadata || typeof source.metadata.component !== "string" || source.metadata.component.length === 0)) {
      return { disposition: "missing_semantic", dimension: semantic.derivative_target_dimension };
    }
    if (!semantic.derivative_target_dimension) return { disposition: "missing_semantic", dimension: null };
    return { disposition: "eligible", dimension: semantic.derivative_target_dimension };
  } catch {
    return { disposition: "contradiction", dimension: null };
  }
}

function strictWithholdingDiagnostic(disposition: Exclude<StrictWitnessDisposition, "eligible">, row: StressNeutralRow, rowIndex: number, packetProvenance = previewProvenance(), classMessage?: string) {
  if (disposition === "retained_absolute_verified" || disposition === "retained_not_covered") {
    if (typeof classMessage !== "string" || !classMessage) throw new Error("SN-RETAINED-PRECISION-CLASS-MESSAGE-MISSING");
    return {
      code: WITHHOLDING_CODES[disposition], class: "unit_preservation_witness", severity: "info",
      source: reference("StressNeutralResultRow", row.result_id), affected_object: reference("StressNeutralUnitWitness", `unit-witness:${rowIndex}`),
      message: classMessage,
      remediation: "Read the retained-precision receipt carried whole by this package before any downstream physical interpretation; this quantity is withheld from rule binding and reliance.",
      provenance: structuredClone(packetProvenance)
    };
  }
  const diagnosticWork = disposition === "diagnostic_work";
  return {
    code: WITHHOLDING_CODES[disposition], class: "unit_preservation_witness", severity: diagnosticWork ? "info" : "blocking",
    source: reference("StressNeutralResultRow", row.result_id), affected_object: reference("StressNeutralUnitWitness", `unit-witness:${rowIndex}`),
    message: diagnosticWork
      ? "Diagnostic work evidence is retained as a row but has no physical unit-preservation witness."
      : `${disposition.replaceAll("_", " ")} prevents a unit-preservation witness; the received row remains retained without a physical interpretation claim.`,
    remediation: diagnosticWork
      ? "Review diagnostic work separately from physical quantity witnesses."
      : "Resolve the source family, unit, dimension and correlation evidence before downstream physical interpretation.",
    provenance: structuredClone(packetProvenance)
  };
}

/** The package diagnostics' class for a run record's rule-pack reference diagnostic. */
const RULE_PACK_REFERENCE_CLASS = "rule_pack_reference";
/** The run record's rule-pack reference diagnostics (a model reference the solve left out of
 * the input manifest's `active_rule_packs`: unresolved, or a checksum mismatch), as package
 * diagnostics after the package's own (`present`): code, severity and message copied from the
 * record; the source the run record, the affected object the excluded reference by identity.
 * One that a diagnostic in `present` already carries is not added again. */
function rulePackRefPackageDiagnostics(analysisRun: AnalysisRunEnvelope, result: MechanicsResult, present: readonly unknown[], packetProvenance = previewProvenance()) {
  const run = analysisRun.analysis_run;
  return rulePackRefAnnotationsToAdd(recordRulePackRefAnnotations(run.diagnostics, result.diagnostics.length), present).map((annotation) => ({
    code: annotation.code, class: RULE_PACK_REFERENCE_CLASS, severity: annotation.severity,
    source: reference("AnalysisRun", run.run_id), affected_object: reference("RulePack", annotation.affected_refs[0]),
    message: annotation.message,
    remediation: "Review the excluded rule-pack reference before downstream use; it is not among this run's rule-pack references.",
    provenance: structuredClone(packetProvenance)
  }));
}

/** Whether a package diagnostic is, or claims to be, a rule-pack exclusion diagnostic: by its
 * reserved code (`MODEL-RULE-PACK-REF-UNRESOLVED`, `MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH`)
 * whatever its class, or by the exclusion class. */
function isRulePackRefPackageDiagnostic(item: unknown): boolean {
  if (!item || typeof item !== "object") return false;
  const { code, class: kind } = item as { code?: unknown; class?: unknown };
  return code === RULE_PACK_REF_UNRESOLVED || code === RULE_PACK_REF_CHECKSUM_MISMATCH || kind === RULE_PACK_REFERENCE_CLASS;
}

/** T6S-5: the successor package's boundary note (D2 4.9.7, 4.9.9; CQ-4, CQ-5). */
const RETAINED_PRECISION_BOUNDARY_NOTE = "The retained-precision receipt travels whole with this package. A row that the accepted reader classes as verified only to the receipt's absolute bound, or as uncovered, keeps its CSV row and value; its unit-preservation witness is withheld with a class finding, and it is withheld from rule binding and reliance. No invocation travels with this package, so numerical eligibility cannot be re-established from the package alone, and the package makes no producer-origin claim.";
/** T6S-5 (S-d): the class findings, counted in the loss report's exported reason. */
function retainedLossCount(absolute: number, uncovered: number): string {
  return ` Of these, ${absolute} rows are verified only to the retained-precision receipt's absolute bound and ${uncovered} rows are uncovered; both are withheld from rule binding and reliance.`;
}

export async function buildStressNeutralExportPacket(args: { model: PreviewModel; result: MechanicsResult; analysisRun: AnalysisRunEnvelope }) {
  const route = sourceContract(args.result);
  if (route === "unsupported") throw new Error("SN-SOURCE-CONTRACT-UNSUPPORTED");
  // T6S-3: this surface's policy entry; a retained-precision successor is admitted
  // only at numerically eligible standing with the live native capture. Epoch 8: the
  // exact routes, load/reference-state results included, add no policy refusal here and
  // take Rust's standing below.
  refuseSurfaceOutput(args.result, args.model, "stress-neutral");
  const retained = isRetainedRoute(route);
  const semantics = currentSemanticContract(args.result);
  const version = STRESS_NEUTRAL_EXPORT_VERSION;
  const profile = STRESS_NEUTRAL_EXPORT_PROFILE;
  // Pure packet projection; Current export stays gated by the panel's live binding,
  // which requires full eligibility. (The retired physics-source-1 arm went with v2.)
  if (numericalResultStanding(args.result, args.model).findings.length > 0) throw new Error("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
  if (args.analysisRun.schema_version !== "0.3.0" || args.analysisRun.analysis_run.run_id !== args.result.run_id || args.model.project.id !== args.result.model_ref
    || args.analysisRun.analysis_run.solver_version?.solver_name !== args.result.producer!.component_name
    || args.analysisRun.analysis_run.solver_version?.solver_version !== args.result.producer!.component_version
    || !args.analysisRun.analysis_run.solver_version?.build_ref.ref
    || args.analysisRun.analysis_run.reproducibility.semantic_contract?.id !== semantics?.id
    || args.analysisRun.analysis_run.reproducibility.semantic_contract?.sha256 !== semantics?.sha256
    || await verifyAnalysisRunRecord(args.analysisRun) !== "match") throw new Error("SN-ANALYSIS-SOURCE-BINDING-MISMATCH");
  const hashes = args.analysisRun.analysis_run.hashes.filter(h => h.payload_scope === "received_result");
  if (hashes.length !== 1 || hashes[0].algorithm !== "sha256" || hashes[0].canonicalization !== "openpipestress_jcs_ijson_v1"
    || hashes[0].payload_ref.object_type !== "ResultEnvelope" || hashes[0].payload_ref.ref !== `result-envelope:${args.result.run_id}`
    || hashes[0].value !== await canonicalSha256HexCheckedV1(args.result)) throw new Error("SN-SOURCE-CARRIER-HASH-MISMATCH");
  await validateAnalysisRunV03(args.analysisRun, args.result, modelLoadBasisRefs(args.model));
  const sourceCarrierChecksum = structuredClone(hashes[0]);
  // T6S-5: classes come only from the accepted reader, run on these bytes without an
  // invocation (Rust `retained_row_classes`); null for every other identity.
  const classes = retained ? await retainedRowClassesFromReader(args.result) : null;
  const inputs = normalizeStressNeutralInputs(args);
  const strictBoundaryNotes = [...inputs.boundary_notes];
  strictBoundaryNotes.push("Result-row dimensions and witness eligibility are interpreted from the accepted semantic contract and bound analysis run; received numerical values, units, rows and source hashes remain unchanged, and no absent raw dimension is claimed as received evidence.");
  strictBoundaryNotes.push("Source annotations carry canonical JSON numeric values; source_value_bits preserves the actual received binary64 value, including negative zero. No live invocation follows from this transport.");
  if (retained) strictBoundaryNotes.push(RETAINED_PRECISION_BOUNDARY_NOTE);
  const dispositions = inputs.result_rows.map((row, rowIndex) => {
    const source = args.result.results.find((candidate) => candidate.id === row.result_id);
    const interpreted: StrictWitnessInterpretation = source ? strictWitnessDisposition(source, args.result, classes) : { disposition: "unknown_semantic", dimension: null };
    return { row, rowIndex, source, ...interpreted };
  });
  const strictWitnesses = dispositions.filter((item) => item.disposition === "eligible").map(({ row, rowIndex }) => ({
    witness_id: `unit-witness:${rowIndex}`,
    source_row_index: rowIndex,
    result_id: row.result_id,
    source_quantity: { value: row.value, unit: row.unit, dimension: row.dimension },
    target_quantity: { value: row.value, unit: row.unit, dimension: row.dimension },
    conversion_performed: false,
    policy: "preserve_received_value_and_unit"
  }));
  const withheld = dispositions.filter((item): item is typeof item & { disposition: Exclude<StrictWitnessDisposition, "eligible"> } => item.disposition !== "eligible");
  const withheldDiagnostics = withheld.map(({ disposition, row, rowIndex, message }) => strictWithholdingDiagnostic(disposition, row, rowIndex, previewProvenance(), message));
  const aggregateWithholdingDiagnostic = withheld.length ? [{
    code: "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", class: "export_blocking", severity: "blocking",
    source: reference("ExportConsumer", "DEL-17-06"), affected_object: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
    message: `${withheld.length} retained rows are explicitly categorized as ineligible for unit-preservation witnesses; ${strictWitnesses.length} rows have accepted semantic-contract interpretations and exact value/unit witnesses.`,
    remediation: "Review every explicit witness-withholding finding before downstream use.", provenance: previewProvenance()
  }] : [];
  const packageDiagnostics = [
    ...inputs.diagnostics,
    ...withheldDiagnostics,
    ...aggregateWithholdingDiagnostic
  ];
  const diagnostics = [...packageDiagnostics, ...rulePackRefPackageDiagnostics(args.analysisRun, args.result, packageDiagnostics)];
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const semanticContractRef = reference("ExternalReference", semanticTablePath(args.result));
  const decisionBasisRefs = [...structuredClone(inputs.unit_system_disclosure.decision_basis_refs), semanticContractRef, inputs.source_run_ref];
  const packet: any = {
    schema_version: version,
    deliverable_id: "DEL-17-06", package_id: "PKG-17", scope_items: ["SOW-046", "SOW-074"], objectives: ["OBJ-007", "OBJ-017", "OBJ-018"],
    export_id: inputs.export_id, package_status: "stress_neutral_export_package", schema_conformant: true,
    validation_ready: blockingCount === 0,
    source_result_ref: inputs.source_result_ref, source_run_ref: inputs.source_run_ref, source_model_ref: inputs.source_model_ref,
    received_source_checksums: structuredClone(args.analysisRun.analysis_run.hashes),
    unresolved_assumption_refs: [], reproducibility_refs: [inputs.source_run_ref], export_profile: { profile_id: profile, ...inputs.export_profile, profile_version: version, boundary_notes: strictBoundaryNotes, source_basis_refs: [inputs.source_run_ref, semanticContractRef] },
    manifest: { manifest_id: inputs.manifest_id, export_profile_ref: reference("StressNeutralExportProfile", profile), boundary_notes: strictBoundaryNotes, package_members: [] as any[], checksums: [] as any[] },
    csv_text: inputs.csv_text, result_rows: inputs.result_rows, unit_system_disclosure: { ...inputs.unit_system_disclosure, decision_basis_refs: decisionBasisRefs },
    unit_preservation_witnesses: strictWitnesses,
    stable_id_map: inputs.stable_id_map,
    loss_report: inputs.loss_entries.map((entry) => ({ loss_id: entry.loss_id, category: entry.category, severity: entry.severity, affected_refs: [entry.affected_ref], target_artifact_ref: reference("StressNeutralExportPackage", inputs.export_id), reason: entry.category === "exported" ? `All ${inputs.result_rows.length} received numerical rows and units are retained unchanged; ${strictWitnesses.length} rows have accepted semantic-contract dimension witnesses and ${withheld.length} rows have explicit witness-withholding findings.${retained ? retainedLossCount(withheld.filter((item) => item.disposition === "retained_absolute_verified").length, withheld.filter((item) => item.disposition === "retained_not_covered").length) : ""}` : entry.reason, source_basis_ref: reference("Deliverable", "DEL-17-06"), downstream_implication: entry.downstream_implication, human_review_required: true, provenance: previewProvenance() })),
    validation_report: { validation_status: blockingCount ? "blocked" : "passed", checks: inputs.validation_checks.map((check) => check.check_id === "unit_preservation_witness_per_row"
      ? ({ check_id: check.check_id, check_status: withheld.length ? "blocking" : "passed", blocking_count: withheld.length, diagnostic_count: withheldDiagnostics.length, provenance: previewProvenance() })
      : ({ check_id: check.check_id, check_status: check.status, blocking_count: check.blocking ? 1 : 0, diagnostic_count: 0, provenance: previewProvenance() })), human_review_required: true, provenance: previewProvenance() },
    diagnostics,
    privacy: { classification: inputs.privacy_classification, commercial_tool_payload_embedded: false, local_only: true, private_payload_embedded: false, protected_payload_embedded: false, redaction_refs: [], telemetry_allowed: false },
    provenance: previewProvenance(),
    professional_boundary: {
      human_review_required: true,
      supports_review: true,
      supports_regression_comparison_input: true,
      supports_downstream_tooling: true,
      software_makes_release_claim: false,
      software_makes_external_compatibility_claim: false,
      software_makes_solver_validation_claim: false,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_creates_professional_reliance_record: false
    },
  };
  // Python's `_method_namespaces`: contract_evidence for preview-physics-2 and both exact
  // identities, and pressure-source-1's receipt (b7-plan MJ8), each copied whole.
  if (route === "preview_physics" || route === "pressure" || route === "pressure_source" || retained) packet.contract_evidence = structuredClone(args.result.contract_evidence);
  if (route === "pressure_source") packet.source_block_recovery = structuredClone(args.result.source_block_recovery);
  // D2 4.9.7 and RR decision 5: the successor's receipt travels with the package, whole.
  if (retained) packet.retained_precision = structuredClone(args.result.retained_precision);
  packet.source_annotations = await retainedSourceAnnotations(args.result);
  Object.assign(packet.export_profile, { csv_encoding: 'utf-8', csv_row_order: 'unicode_scalar_value_result_id' });
  Object.assign(packet, {
    producer: structuredClone(args.result.producer), numerical_quality: structuredClone(args.result.numerical_quality), formulation_basis: structuredClone(args.result.formulation_basis),
    semantic_contract: semantics,
    semantic_contract_ref: { ref_type: "semantic_contract", ref_id: semantics!.id }, source_carrier_checksum: sourceCarrierChecksum
  });
  const payloads: Record<string, unknown> = {
    "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure,
    "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report,
    "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics,
  };
  const checksums = [];
  for (const filename of STRICT_MEMBER_NAMES.slice(1)) {
    const value = payloads[filename];
    checksums.push(filename.endsWith(".csv")
      ? { algorithm: "sha256" as const, canonicalization: "utf8_csv_record_lf_v1", payload_scope: "member_bytes", payload_ref: checksumRef(filename), value: await rawTextSha256(value as string) }
      : await checkedMemberChecksum(filename, value));
  }
  const manifestChecksum = await checkedMemberChecksum("manifest.json", strictManifestSeed(packet, checksums), "manifest_seed");
  packet.manifest.checksums = [...checksums, manifestChecksum];
  packet.manifest.package_members = STRICT_MEMBER_NAMES.map((filename) => ({ filename, checksum: packet.manifest.checksums.find((item: any) => item.payload_ref.ref === filename) }));
  packet.package_checksum = { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "complete_package_excluding_self_checksum", payload_ref: { object_type: "StressNeutralExportPackage", ref: packet.export_id }, value: await canonicalSha256HexCheckedV1(packet) };
  await validateDesktopStressNeutralExportPacket(packet, args.result, args.analysisRun, modelLoadBasisRefs(args.model));
  return packet;
}

/** The desktop publication path: the builder's self-check and the panel's package JSON link,
 * whose browser download and native save are both offered only after this check passes. Before
 * the generic checks, the package's top-level provenance must equal this builder's fixed one
 * exactly (canonical JSON), so a package restamped with other dispositions is refused here; the
 * witness-withholding findings, which the generic validator binds to the top level, follow. The
 * generic validator below still admits another exporter's package with its own provenance. */
export async function validateDesktopStressNeutralExportPacket(packet: any, source?: MechanicsResult, analysisRun?: AnalysisRunEnvelope, expectedBasisRefs?: ObjectRef[]): Promise<void> {
  let bound = false;
  try {
    bound = packet !== null && typeof packet === "object" && Object.hasOwn(packet, "provenance")
      && await canonicalSha256HexCheckedV1(packet.provenance) === await canonicalSha256HexCheckedV1(previewProvenance());
  } catch { bound = false; }
  if (!bound) throw new Error("SN-DESKTOP-PROVENANCE-MISMATCH");
  await validateStressNeutralExportPacket(packet, source, analysisRun, expectedBasisRefs);
}

export async function validateStressNeutralExportPacket(packet: any, source?: MechanicsResult, analysisRun?: AnalysisRunEnvelope, expectedBasisRefs?: ObjectRef[]): Promise<void> {
  // Preserve the named rejection of current metadata on a retired statement;
  // this arm reads no old package and admits no retired format.
  if (packet.schema_version === "0.2.0" && ["producer", "numerical_quality", "formulation_basis", "semantic_contract_ref", "semantic_contract", "source_carrier_checksum", "source_block_recovery", "contract_evidence", "source_annotations", "retained_precision"].some(key => Object.hasOwn(packet, key))) {
    throw new Error("SN-LEGACY-PRECISION-METADATA-FORBIDDEN");
  }
  if (packet.schema_version !== STRESS_NEUTRAL_EXPORT_VERSION) throw new Error("SN-VERSION-UNSUPPORTED");
  if (analysisRun !== undefined && source === undefined) throw new Error("SN-PRECISION-ANALYSIS-SOURCE-REQUIRED");
  // T6S-3: this surface's policy entry, route level (a validator has no model).
  refuseSurfaceRoute(source, "stress-neutral");
  if (source !== undefined) {
    if (!hasCurrentSourceContract(source)) throw new Error("SN-PRECISION-SOURCE-BINDING-REQUIRED");
    for (const value of [source.run_id, source.model_ref, ...source.results.map(r => r.id)]) if (typeof value !== "string" || !value) throw new Error("ANALYSIS_SOURCE_REFERENCE_INVALID");
    if (analysisRun !== undefined) await validateAnalysisRunV03(analysisRun, source, expectedBasisRefs);
    const same = async (a: unknown, b: unknown) => a !== undefined && b !== undefined && await canonicalSha256HexCheckedV1(a) === await canonicalSha256HexCheckedV1(b);
    for (const key of ["producer", "numerical_quality", "formulation_basis"] as const) if (!await same(packet[key], source[key])) throw new Error("SN-PRECISION-SOURCE-METADATA-MISMATCH");
    // Epoch 8 (design §4.3): the receipt is present and equal on pressure-source-1 only.
    if (sourceContract(source) === "pressure_source") {
      if (!Object.hasOwn(packet, "source_block_recovery")) throw new Error("SN-SOURCE-RECOVERY-CONTRADICTION");
      if (!await same(packet.source_block_recovery, source.source_block_recovery)) throw new Error("SN-SOURCE-BLOCK-RECEIPT-MISMATCH");
    } else if (Object.hasOwn(packet, "source_block_recovery")) throw new Error("SN-SOURCE-RECOVERY-CONTRADICTION");
    const sourceRetained = isRetainedRoute(sourceContract(source));
    if (["preview_physics", "pressure", "pressure_source", "retained_preview_physics", "retained_physics"].includes(sourceContract(source))) {
      if (!await same(packet.contract_evidence, source.contract_evidence)) throw new Error("SN-PHYSICAL-EVIDENCE-MISMATCH");
    } else if (Object.hasOwn(packet, "contract_evidence")) throw new Error("SN-PHYSICAL-EVIDENCE-CONTRADICTION");
    // T6S-5 (D2 4.9.7): the copied receipt equals the source's, whole; no other identity's package carries one.
    if (sourceRetained) {
      if (!await same(packet.retained_precision, source.retained_precision)) throw new Error("SN-RETAINED-PRECISION-RECEIPT-MISMATCH");
    } else if (Object.hasOwn(packet, "retained_precision")) throw new Error(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    if (!await same(packet.source_annotations, await retainedSourceAnnotations(source))) throw new Error("SN-SOURCE-ANNOTATION-BINDING-MISMATCH");
    const received = { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_result", payload_ref: reference("ResultEnvelope", `result-envelope:${source.run_id}`), value: await canonicalSha256HexCheckedV1(source) };
    if (!await same(packet.source_model_ref, reference("Model", source.model_ref)) || !await same(packet.source_run_ref, reference("AnalysisRun", source.run_id)) || !await same(packet.source_result_ref, received.payload_ref) || !await same(packet.source_carrier_checksum, received) || !await same(packet.received_source_checksums?.filter((h: any) => h.payload_scope === "received_result"), [received])) throw new Error("SN-PRECISION-SOURCE-IDENTITY-MISMATCH");
    const rawById = new Map(source.results.map(row => [row.id, row]));
    const rows = packet.result_rows;
    if (!Array.isArray(rows) || rows.length !== source.results.length || rawById.size !== source.results.length || new Set(rows.map((r: any) => r?.result_id)).size !== rows.length) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
    // T6S-5: the successor's classes from the accepted reader (no invocation).
    const classes = sourceRetained ? await retainedRowClassesFromReader(source) : null;
    const expectedWitnesses = [], expectedFindings = [];
    let retainedAbsolute = 0, retainedUncovered = 0;
    for (const [index, row] of rows.entries()) {
      const raw = rawById.get(row?.result_id);
      if (!raw) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      // Provenance belongs to the exporting package, not the source mechanics row.
      const { provenance: _provenance, ...expected } = precisionStressRow(raw, source);
      if (Object.keys(expected).some(key => !Object.hasOwn(row, key))) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      const actual = Object.fromEntries(Object.keys(expected).map(key => [key, row[key]]));
      if (!await same(actual, expected)) throw new Error("SN-PRECISION-ROW-SOURCE-MISMATCH");
      const { disposition, message } = strictWitnessDisposition(raw, source, classes);
      const quantity = { value: row.value, unit: row.unit, dimension: row.dimension };
      if (disposition === "retained_absolute_verified") retainedAbsolute++;
      if (disposition === "retained_not_covered") retainedUncovered++;
      if (disposition === "eligible") expectedWitnesses.push({ witness_id: `unit-witness:${index}`, source_row_index: index, result_id: row.result_id, source_quantity: quantity, target_quantity: quantity, conversion_performed: false, policy: "preserve_received_value_and_unit" });
      else expectedFindings.push(strictWithholdingDiagnostic(disposition, row, index, packet.provenance, message));
    }
    if (expectedFindings.length) expectedFindings.push({
      code: "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", class: "export_blocking", severity: "blocking",
      source: reference("ExportConsumer", "DEL-17-06"), affected_object: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
      message: `${expectedFindings.length} retained rows are explicitly categorized as ineligible for unit-preservation witnesses; ${expectedWitnesses.length} rows have accepted semantic-contract interpretations and exact value/unit witnesses.`,
      remediation: "Review every explicit witness-withholding finding before downstream use.", provenance: structuredClone(packet.provenance)
    });
    const codes = [...Object.values(WITHHOLDING_CODES), "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE"];
    if (!await same(packet.unit_preservation_witnesses, expectedWitnesses) || !await same(packet.diagnostics.filter((d: any) => codes.includes(d.code)), expectedFindings)) throw new Error("SN-PRECISION-WITNESS-BINDING-MISMATCH");
    // The run record's rule-pack reference diagnostics, each once, as the builder projects them.
    // The expected list comes from trusted inputs alone: the record, and the package's own
    // diagnostics as the builder makes them from the source (comparison, witness withholding),
    // never from the packet's list. Every packet diagnostic with a reserved exclusion code, or
    // the exclusion class, is held to it in full and in order, whatever its class. Their
    // provenance is the desktop builder's fixed one, never the packet's: only that builder
    // emits them, while another exporter's package may carry its own top-level provenance.
    if (analysisRun !== undefined && Array.isArray(packet.diagnostics)) {
      const trustedOwn = [comparisonSemanticsDiagnostic(`stress-neutral:${safeFileToken(source.run_id)}`, previewProvenance()), ...expectedFindings];
      const expected = [...trustedOwn.filter(isRulePackRefPackageDiagnostic), ...rulePackRefPackageDiagnostics(analysisRun, source, trustedOwn, previewProvenance())];
      if (!await same(packet.diagnostics.filter(isRulePackRefPackageDiagnostic), expected)) throw new Error("SN-RULE-PACK-REF-DIAGNOSTICS-MISMATCH");
    }
    // T6S-5 (S-d): the class findings are counted in the loss report's exported reason.
    if (sourceRetained) {
      const withheldCount = expectedFindings.length ? expectedFindings.length - 1 : 0;
      const exported = Array.isArray(packet.loss_report) ? packet.loss_report.filter((entry: any) => entry?.category === "exported") : [];
      const reason = `All ${rows.length} received numerical rows and units are retained unchanged; ${expectedWitnesses.length} rows have accepted semantic-contract dimension witnesses and ${withheldCount} rows have explicit witness-withholding findings.${retainedLossCount(retainedAbsolute, retainedUncovered)}`;
      if (exported.length !== 1 || exported[0].reason !== reason) throw new Error("SN-RETAINED-PRECISION-LOSS-COUNT-MISMATCH");
    }
  }
  const version = STRESS_NEUTRAL_EXPORT_VERSION;
  const profile = STRESS_NEUTRAL_EXPORT_PROFILE;
  // T6S-5 (I67's F4): the transport header carries the receipt, so a successor
  // package dispatches to its own route and a receipt on any other identity reads
  // unsupported (the downgrade guard).
  const header = { schema_version: "0.2.0", producer: packet.producer, numerical_quality: packet.numerical_quality, formulation_basis: packet.formulation_basis, ...(Object.hasOwn(packet,"source_block_recovery") ? {source_block_recovery:packet.source_block_recovery} : {}), ...(Object.hasOwn(packet,"contract_evidence") ? {contract_evidence:packet.contract_evidence} : {}), ...(Object.hasOwn(packet,"retained_precision") ? {retained_precision:packet.retained_precision} : {}) } as MechanicsResult;
  if (sourceContract(header) === 'unsupported') throw new Error('SN-PRECISION-CONTRACT-MISMATCH');
  const semanticPath = semanticTablePath(header);
  if (packet.export_profile?.csv_encoding !== 'utf-8' || packet.export_profile?.csv_row_order !== 'unicode_scalar_value_result_id') throw new Error('SN-CSV-ENCODING-PROFILE-MISMATCH');
  await validateNeutralTransportEvidence(header, source);
  await validateRetainedSourceAnnotations(packet, header);
  const semantics = hasCurrentSourceContract(header) ? currentSemanticContract(header) : null;
  if (!semantics || packet.semantic_contract?.id !== semantics.id || packet.semantic_contract?.sha256 !== semantics.sha256
    || packet.semantic_contract_ref?.ref_type !== "semantic_contract" || packet.semantic_contract_ref?.ref_id !== semantics.id) throw new Error("SN-PRECISION-CONTRACT-MISMATCH");
  const claim = packet.source_carrier_checksum;
  if (!claim || claim.algorithm !== "sha256" || claim.canonicalization !== "openpipestress_jcs_ijson_v1" || claim.payload_scope !== "received_result"
    || JSON.stringify(claim.payload_ref) !== JSON.stringify(packet.source_result_ref)
    || !packet.received_source_checksums?.some((h: unknown) => JSON.stringify(h) === JSON.stringify(claim))) throw new Error("SN-SOURCE-CARRIER-CHECKSUM-UNBOUND");
  if (!Array.isArray(packet.manifest?.package_members) || !Array.isArray(packet.manifest?.checksums) || packet.manifest.package_members.length !== 9 || packet.manifest.checksums.length !== 9) throw new Error("SN-STRICT-INVENTORY-MISMATCH");
  if (packet.schema_conformant !== true) throw new Error("SN-SCHEMA-CONFORMANCE-CLAIM-INVALID");
  if (packet.export_profile?.profile_id !== profile || packet.export_profile?.profile_version !== version
    || packet.manifest?.export_profile_ref?.object_type !== "StressNeutralExportProfile" || packet.manifest?.export_profile_ref?.ref !== profile) throw new Error("SN-PROFILE-IDENTITY-MISMATCH");
  const exactNames = [...STRICT_MEMBER_NAMES];
  const memberNames = packet.manifest.package_members.map((item: any) => item.filename);
  const checksumNames = packet.manifest.checksums.map((item: any) => item?.payload_ref?.ref);
  if (JSON.stringify(memberNames) !== JSON.stringify(exactNames)) throw new Error("SN-MEMBER-ORDER-MISMATCH");
  if (new Set(memberNames).size !== 9 || new Set(checksumNames).size !== 9 || exactNames.some((name) => !checksumNames.includes(name))) throw new Error("SN-MEMBER-CHECKSUM-BIJECTION-MISMATCH");
  const metadata = (filename: string) => filename === "manifest.json"
    ? ["sha256", "openpipestress_jcs_ijson_v1", "manifest_seed", "StressNeutralMember", filename]
    : filename.endsWith(".csv")
      ? ["sha256", "utf8_csv_record_lf_v1", "member_bytes", "StressNeutralMember", filename]
      : ["sha256", "openpipestress_jcs_ijson_v1", "member_payload", "StressNeutralMember", filename];
  for (const [index, filename] of exactNames.entries()) {
    const claim = packet.manifest.checksums.find((item: any) => item?.payload_ref?.ref === filename);
    if (!claim || JSON.stringify([claim.algorithm, claim.canonicalization, claim.payload_scope, claim.payload_ref?.object_type, claim.payload_ref?.ref]) !== JSON.stringify(metadata(filename))) throw new Error(`SN-CHECKSUM-METADATA-MISMATCH: ${filename}`);
    if (packet.manifest.package_members[index]?.filename !== filename || JSON.stringify(packet.manifest.package_members[index]?.checksum) !== JSON.stringify(claim)) throw new Error(`SN-MEMBER-CHECKSUM-BINDING-MISMATCH: ${filename}`);
  }
  const packageMetadata = [packet.package_checksum?.algorithm, packet.package_checksum?.canonicalization, packet.package_checksum?.payload_scope, packet.package_checksum?.payload_ref?.object_type, packet.package_checksum?.payload_ref?.ref];
  if (JSON.stringify(packageMetadata) !== JSON.stringify(["sha256", "openpipestress_jcs_ijson_v1", "complete_package_excluding_self_checksum", "StressNeutralExportPackage", packet.export_id])) throw new Error("SN-PACKAGE-CHECKSUM-METADATA-MISMATCH");
  const received = new Set(packet.received_source_checksums.map((item: any) => JSON.stringify(item)));
  if (packet.manifest.checksums.some((item: any) => received.has(JSON.stringify(item)))) throw new Error("SN-RECEIVED-CHECKSUM-RELABEL");
  const payloads: Record<string, unknown> = { "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure, "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics };
  const nonManifest = packet.manifest.checksums.filter((item: any) => item.payload_ref.ref !== "manifest.json");
  for (const claim of nonManifest) {
    const expected = claim.payload_ref.ref.endsWith(".csv") ? await rawTextSha256(payloads[claim.payload_ref.ref] as string) : await canonicalSha256HexCheckedV1(payloads[claim.payload_ref.ref]);
    if (claim.value !== expected) throw new Error(`SN-MEMBER-CHECKSUM-MISMATCH: ${claim.payload_ref.ref}`);
  }
  const manifest = packet.manifest.checksums.find((item: any) => item.payload_ref.ref === "manifest.json");
  if (!manifest || manifest.value !== await canonicalSha256HexCheckedV1(strictManifestSeed(packet, nonManifest))) throw new Error("SN-MANIFEST-SEED-CHECKSUM-MISMATCH");
  const rows = packet.result_rows;
  if (!Array.isArray(rows) || new Set(rows.map((row: any) => row?.result_id)).size !== rows.length
    || rows.some((row: any) => typeof row?.result_id !== "string" || row?.canonical_ref?.ref !== row.result_id || row?.source_result_ref?.ref !== row.result_id)) throw new Error("SN-RESULT-ROW-IDENTITY-MISMATCH");
  if (rows.some((row: any) => typeof row.value !== "number" || !Number.isFinite(row.value))) throw new Error("SN-PRECISION-ROW-VALUE-INVALID");
  validateStressNeutralCsv(packet.csv_text, rows);
  if (!Array.isArray(packet.stable_id_map) || packet.stable_id_map.length !== rows.length
    || new Set(packet.stable_id_map.map((item: any) => item?.canonical_ref?.ref)).size !== rows.length
    || new Set(packet.stable_id_map.map((item: any) => JSON.stringify([item?.export_ref?.object_type, item?.export_ref?.ref]))).size !== rows.length
    || packet.stable_id_map.some((item: any) => !rows.some((row: any) => row.result_id === item?.canonical_ref?.ref)
      || item?.mapping_status !== "mapped" || item?.loss_category !== "exported")) throw new Error("SN-STABLE-ID-MAP-BINDING-MISMATCH");
  if (packet.stable_id_map.some((item: any) => item.canonical_ref?.object_type !== "Result" || Object.keys(item.canonical_ref).length !== 2 || !item.export_ref || typeof item.export_ref !== "object" || Array.isArray(item.export_ref) || Object.keys(item.export_ref).length !== 2 || typeof item.export_ref.object_type !== "string" || !item.export_ref.object_type || typeof item.export_ref.ref !== "string" || !item.export_ref.ref)) throw new Error("SN-STABLE-ID-MAP-BINDING-MISMATCH");
  if (!packet.received_source_checksums.some((item: any) => JSON.stringify(item.payload_ref) === JSON.stringify(packet.source_result_ref))) throw new Error("SN-SOURCE-RESULT-REF-UNBOUND");
  if (!packet.reproducibility_refs.some((item: any) => JSON.stringify(item) === JSON.stringify(packet.source_run_ref))) throw new Error("SN-SOURCE-RUN-REF-UNBOUND");
  const seenWitnesses = new Set<string>();
  const seenWitnessIds = new Set<string>();
  for (const witness of packet.unit_preservation_witnesses) {
    if ((!Number.isSafeInteger(witness.source_row_index) || witness.source_row_index < 0 || witness.witness_id !== `unit-witness:${witness.source_row_index}` || seenWitnessIds.has(witness.witness_id))) throw new Error("SN-UNIT-WITNESS-BINDING-MISMATCH");
    seenWitnessIds.add(witness.witness_id);
    const row = rows[witness.source_row_index];
    if (!row || seenWitnesses.has(witness.result_id) || witness.result_id !== row.result_id || witness.conversion_performed !== false || witness.policy !== "preserve_received_value_and_unit") throw new Error("SN-UNIT-WITNESS-BINDING-MISMATCH");
    const expectedQuantity = { value: row.value, unit: row.unit, dimension: row.dimension };
    const quantityMatches = witness.source_quantity !== undefined && witness.target_quantity !== undefined && await canonicalJsonCheckedV1(witness.source_quantity) === await canonicalJsonCheckedV1(expectedQuantity) && await canonicalJsonCheckedV1(witness.target_quantity) === await canonicalJsonCheckedV1(expectedQuantity);
    if (!quantityMatches) throw new Error("SN-UNIT-WITNESS-BINDING-MISMATCH");
    seenWitnesses.add(witness.result_id);
  }
  for (const entry of packet.loss_report) if (entry.target_artifact_ref?.ref !== packet.export_id || entry.human_review_required !== true) throw new Error("SN-LOSS-REPORT-BINDING-MISMATCH");
  const withholdingCodes = new Map(Object.entries(WITHHOLDING_CODES).map(([category, code]) => [code, category]));
  const categorized = new Map<string, string>();
  for (const item of packet.diagnostics) {
    const category = withholdingCodes.get(item.code);
    if (!category) continue;
    const resultId = item.source?.ref;
    const expectedSeverity = INFO_WITHHOLDINGS.has(category) ? "info" : "blocking";
    if (typeof resultId !== "string" || !rows.some((row: any) => row.result_id === resultId)
      || categorized.has(resultId) || item.severity !== expectedSeverity) throw new Error("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH");
    categorized.set(resultId, category);
  }
  if ([...seenWitnesses].some((id) => categorized.has(id)) || seenWitnesses.size + categorized.size !== rows.length
    || rows.some((row: any) => !seenWitnesses.has(row.result_id) && !categorized.has(row.result_id))) throw new Error("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH");
  if (!packet.export_profile?.source_basis_refs?.some((item: any) => item?.ref === semanticPath)
    || !packet.unit_system_disclosure?.decision_basis_refs?.some((item: any) => item?.ref === semanticPath)) throw new Error("SN-SEMANTIC-CONTRACT-BINDING-MISSING");
  const blockingDiagnostics = packet.diagnostics.filter((item: any) => item.severity === "blocking").length;
  const checks = packet.validation_report?.checks;
  const checksConsistent = Array.isArray(checks) && checks.every((check: any) => Number.isSafeInteger(check.blocking_count) && check.blocking_count >= 0
    && ((check.check_status === "blocking" && check.blocking_count > 0) || (check.check_status === "passed" && check.blocking_count === 0)));
  if (!checksConsistent || packet.validation_report?.validation_status !== (blockingDiagnostics > 0 ? "blocked" : "passed")
    || checks.some((check: any) => check.check_status === "blocking") !== (blockingDiagnostics > 0)
    || packet.validation_ready !== (blockingDiagnostics === 0)) throw new Error("SN-VALIDATION-STATUS-BINDING-MISMATCH");
  const privacy = packet.privacy;
  if (!privacy || privacy.local_only !== true || privacy.commercial_tool_payload_embedded !== false || privacy.private_payload_embedded !== false
    || privacy.protected_payload_embedded !== false || privacy.telemetry_allowed !== false) throw new Error("SN-PRIVACY-BOUNDARY-VIOLATION");
  const boundary = packet.professional_boundary;
  if (!boundary || boundary.human_review_required !== true || [
    "software_makes_release_claim",
    "software_makes_external_compatibility_claim",
    "software_makes_solver_validation_claim",
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_creates_professional_reliance_record"
  ].some((key) => boundary[key] !== false)) throw new Error("SN-PROFESSIONAL-BOUNDARY-VIOLATION");
  const projection = structuredClone(packet); delete projection.package_checksum;
  if (packet.package_checksum.value !== await canonicalSha256HexCheckedV1(projection)) throw new Error("SN-PACKAGE-CHECKSUM-MISMATCH");
}

/** Validate received spelling without replacing independently hashed CSV bytes. */
export function validateStressNeutralCsv(text: unknown, rows: StressNeutralRow[]): void {
  const fail = (): never => { throw new Error("SN-CSV-ROW-BINDING-MISMATCH"); };
  if (typeof text !== "string" || text.startsWith('\ufeff') || !validUtf8Text(text)) fail();
  const csv = text as string;
  if (!csv.endsWith('\n') || rows.some((row, i) => i > 0 && scalarIdCompare(rows[i - 1].result_id, row.result_id) >= 0)) fail();
  const records: string[][] = []; let record: string[] = [], field = "", mode: "start" | "plain" | "quoted" | "closed" = "start";
  for (let i = 0; i < csv.length; i++) {
    const char = csv[i];
    if (mode === "quoted") {
      if (char === '"') { if (csv[i + 1] === '"') { field += '"'; i++; } else mode = "closed"; }
      else field += char;
      continue;
    }
    if (char === "," || char === "\n") {
      record.push(field); field = ""; mode = "start";
      if (char === "\n") { records.push(record); record = []; }
    } else if (mode === "closed" || char === '\r' || (mode === 'plain' && char === '"')) fail();
    else if (mode === "start" && char === '"') mode = "quoted";
    else { field += char; mode = "plain"; }
  }
  if (mode === "quoted" || field || record.length || records.length !== rows.length + 1 || JSON.stringify(records[0]) !== JSON.stringify(CSV_COLUMNS) || records.some(r => r.length !== CSV_COLUMNS.length)) fail();
  const ids = new Set<string>();
  for (const [index, row] of rows.entries()) {
    const received = records[index + 1];
    if (!received[0] || ids.has(received[0])) fail(); ids.add(received[0]);
    for (const [columnIndex, column] of CSV_COLUMNS.entries()) {
      const scalar = received[columnIndex];
      if (column === "value") {
        if (!/^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?(?![\s\S])/.test(scalar)) fail();
        const parsed = Number(scalar), significand = scalar.split(/[eE]/)[0];
        if (!Number.isFinite(parsed) || (parsed === 0 && /[1-9]/.test(significand)) || (Number.isInteger(parsed) && !Number.isSafeInteger(parsed)) || typeof row.value !== "number" || !Number.isFinite(row.value) || parsed !== row.value) fail();
      } else {
        const value = row[column as keyof StressNeutralRow];
        const expected = ["canonical_ref", "load_case_ref", "station_ref", "component_ref"].includes(column) ? (value as StressNeutralRef)?.ref ?? "" : value ?? "";
        if (typeof expected !== "string" || scalar !== expected) fail();
      }
    }
  }
}

export function precisionStressRow(item: MechanicsResult["results"][number], source: MechanicsResult): StressNeutralRow {
  if (typeof item.value !== "number" || !Number.isFinite(item.value)) throw new Error("SN-PRECISION-ROW-VALUE-INVALID");
  const { semantic } = analysisRowSemantics(item, source);
  const location = item.metadata?.location;
  if (location !== undefined && location !== null && (typeof location !== "string" || !location)) throw new Error("SN-PRECISION-ROW-LOCATION-INVALID");
  if (typeof item.entity_ref !== "string" || !item.entity_ref) throw new Error("SN-PRECISION-ROW-ENTITY-MISSING");
  const dimension = semantic?.derivative_target_dimension ?? "TBD";
  const component = ['support_reaction_component_v2', 'support_reaction_force_magnitude_v2', 'support_reaction_moment_magnitude_v2'].includes(item.kind)
    ? reference('Support', item.entity_ref) : entityReference(item.entity_ref);
  return { result_id: item.id, canonical_ref: reference("Result", item.id), row_kind: "result_value", result_family: semantic?.category === "physical_quantity" ? semantic.family ?? "other" : "other",
    load_case_ref: sourceBasisReference(item.basis_ref) ?? reference("AnalysisRun", source.run_id), station_ref: reference("Station", location ?? "summary"), component_ref: component, value: item.value, unit: item.unit, dimension,
    correlation_status: item.unit && dimension !== "TBD" ? "canonical_id_map" : "unit_or_dimension_blocking_review_required", source_result_ref: reference("Result", item.id), provenance: previewProvenance() };
}

function check(checkId: string, passed: boolean) {
  return {
    check_id: checkId,
    status: passed ? "passed" : "blocking",
    blocking: !passed
  };
}

function renderCsv(rows: StressNeutralRow[]): string {
  const body = rows.map((row) => CSV_COLUMNS.map((column) => csvValue(csvColumnValue(row, column))).join(","));
  return `${CSV_COLUMNS.join(",")}\n${body.join("\n")}\n`;
}

function csvColumnValue(row: StressNeutralRow, column: CsvColumn): string | number | StressNeutralRef {
  return row[column];
}

function csvValue(value: string | number | StressNeutralRef): string {
  const text = typeof value === "object" ? value.ref : String(value);
  if (/[",\n\r]/.test(text)) return `"${text.replaceAll('"', '""')}"`;
  return text;
}

function entityReference(value: string) {
  if (value.startsWith("pipe:")) return reference("PipeElement", value);
  if (value.startsWith("node:")) return reference("Node", value);
  if (value.startsWith("support:")) return reference("Support", value);
  if (value.startsWith("component:")) return reference("Component", value);
  if (value.startsWith("material:")) return reference("Material", value);
  return reference("CanonicalObject", value);
}


function unitCount(packet: Awaited<ReturnType<typeof buildStressNeutralExportPacket>>): number {
  return new Set(packet.result_rows.map((row: any) => row.unit)).size;
}

function dimensionSummary(packet: Awaited<ReturnType<typeof buildStressNeutralExportPacket>>): string {
  return Array.from(new Set<string>(packet.result_rows.map((row: any) => row.dimension))).sort().join(", ");
}

function reference(objectType: string, ref: string): StressNeutralRef {
  return {
    object_type: objectType,
    ref
  };
}

function previewProvenance() {
  return {
    source_name: "SWBPIPE desktop",
    source_location: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
    source_license: "MIT project license context",
    contributor: "SWBPIPE app integration tranche",
    contributor_certification: "Invented preview metadata only; no protected standards or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function csvDataHref(csvText: string): string {
  return `data:text/csv;charset=utf-8,${encodeURIComponent(csvText)}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";

/** Every route has a deliberate entry (tsc-checked); null names no table here, and
 * such a route refuses. Epoch 8 names the exact routes' tables. */
const SEMANTIC_TABLE_FILES: Readonly<Record<Exclude<SourceContract, "unsupported">, string | null>> = Object.freeze({
  preview_physics: "semantic_contract_v0_3_preview_physics_2.json", retained_preview_physics: "semantic_contract_v0_3_preview_physics_retained_1.json",
  retained_physics: "semantic_contract_v0_3_physics_retained_1.json",
  pressure_source: "semantic_contract_v0_3_pressure_source_1.json", pressure: "semantic_contract_v0_3_pressure_2.json",
});
/** DEL-05-03 `pf-timoshenko.md` §4: the preview route's table by the source's identity. */
const PREVIEW_IDENTITY_TABLE_FILES: Readonly<Record<string, string>> = Object.freeze({
  [PREVIEW_PHYSICS_CONTRACT_ID]: "semantic_contract_v0_3_preview_physics_2.json",
  [PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]: "semantic_contract_v0_3_preview_physics_timoshenko_1.json",
});
function semanticTablePath(source: MechanicsResult): string {
  const route = sourceContract(source);
  if (route === "unsupported") throw new Error("SN-SOURCE-CONTRACT-UNSUPPORTED");
  refuseSurfaceRoute(source, "stress-neutral");
  const identity = source.producer?.semantic_contract_id ?? "";
  const file = route === "preview_physics"
    ? (Object.hasOwn(PREVIEW_IDENTITY_TABLE_FILES, identity) ? PREVIEW_IDENTITY_TABLE_FILES[identity] : null)
    : Object.hasOwn(SEMANTIC_TABLE_FILES, route) ? SEMANTIC_TABLE_FILES[route] : null;
  if (!file) throw new Error("SN-SOURCE-CONTRACT-UNSUPPORTED");
  return `fixtures/results/${file}`;
}
async function validateNeutralTransportEvidence(header: MechanicsResult, source?: MechanicsResult): Promise<void> {
  const route = sourceContract(header);
  if (route === "preview_physics") readerTransport("preview-physics", header);
  else if (route === "pressure_source") {
    readerTransport("pressure-source", header);
    // Design §3: a supplied pressure-source-1 source is read through the full raw reader
    // (no invocation), as Rust's derivative and Python's record builder read it; the
    // shallow source-recovery check omits raw-row validation.
    if (source !== undefined) readerValidate("pressure-source", source);
  }
  // Epoch 8 (design §3, §4.3): pressure-2 has no transport reader, so its package is read
  // through its supplied source, as Python reads it; the source bindings above tie every
  // copied member to it. Without a source, Rust refuses by name
  // (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED).
  else if (route === "pressure") {
    if (source === undefined) readerTransport("pressure", header);
    else if (sourceContract(source) !== "pressure") throw new Error("SN-SOURCE-METHOD-BINDING-MISMATCH");
    else readerValidate("pressure", source);
  }
  // T6S-5: the accepted reader's transport checks (G0-G2 and the base transport
  // metadata on its projection); a refusal carries the reader's code.
  else if (isRetainedRoute(route)) await validateRetainedPrecisionTransport(header);
  else throw new Error("SN-SOURCE-CONTRACT-UNSUPPORTED");
}
async function retainedSourceAnnotations(source: MechanicsResult) {
  return Promise.all(source.results.map(async (row, source_row_index) => {
    const source_value_bits = sourceValueBits(row.value);
    const source_row = structuredClone(row);
    if (source_row.value === 0) source_row.value = 0; // Canonical JSON number; sign retained separately.
    return {source_row_index,source_result_id:row.id,source_row,source_row_sha256:await canonicalSha256HexCheckedV1(row),source_value_bits};
  }));
}
async function validateRetainedSourceAnnotations(packet: any, header: MechanicsResult): Promise<void> {
  if (!Array.isArray(packet.source_annotations) || !Array.isArray(packet.result_rows) || packet.source_annotations.length !== packet.result_rows.length) throw new Error("SN-SOURCE-ANNOTATION-COVERAGE");
  const byId = new Map<string, any>(packet.result_rows.map((row: any) => [row.result_id,row])), seen = new Set<string>();
  if (byId.size !== packet.result_rows.length) throw new Error("SN-SOURCE-ANNOTATION-COVERAGE");
  for (const [index, annotation] of packet.source_annotations.entries()) {
    const expectedKeys = ["source_row_index","source_result_id","source_row","source_row_sha256","source_value_bits"];
    if (!annotation || typeof annotation !== "object" || Object.keys(annotation).length !== expectedKeys.length || expectedKeys.some(k=>!Object.hasOwn(annotation,k))) throw new Error("SN-SOURCE-ANNOTATION-SHAPE");
    const raw = annotation.source_row as MechanicsResult["results"][number], id = annotation.source_result_id;
    if (!raw || typeof id !== "string" || !id || raw.id !== id || seen.has(id) || annotation.source_row_index !== index || annotation.source_row_sha256 !== await canonicalSha256HexCheckedV1(raw)) throw new Error("SN-SOURCE-ANNOTATION-IDENTITY-OR-HASH");
    validateSourceValueBits(annotation);
    seen.add(id);
    // Projection needs the explicit table and fallback run identity only. This
    // metadata view is never passed to a raw-source or invocation validator.
    const {provenance:_provenance,...expected} = precisionStressRow(raw, {...header,run_id:packet.source_run_ref?.ref});
    const row=byId.get(id);
    if (!row || Object.entries(expected).some(([key,value])=>JSON.stringify(row[key])!==JSON.stringify(value))) throw new Error("SN-SOURCE-ANNOTATION-ROW-BINDING");
  }
}

function sourceValueBits(value: number): string {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error("SN-SOURCE-VALUE-BITS-INVALID");
  const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,value,false);
  return bytes.getBigUint64(0,false).toString(16).padStart(16,"0");
}
function validateSourceValueBits(annotation: any): void {
  const bits=annotation.source_value_bits,value=annotation.source_row?.value;
  if (typeof bits!=="string" || !/^[0-9a-f]{16}$/.test(bits) || typeof value!=="number" || !Number.isFinite(value) || Object.is(value,-0)) throw new Error("SN-SOURCE-VALUE-BITS-INVALID");
  const bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,BigInt(`0x${bits}`),false);const observed=bytes.getFloat64(0,false);
  if (!Number.isFinite(observed) || (value===0 ? observed!==0 : bits!==sourceValueBits(value))) throw new Error("SN-SOURCE-VALUE-BITS-INVALID");
}
