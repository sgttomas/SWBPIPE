import { Download, FileWarning } from "lucide-react";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

type LintTargetDraft = ReturnType<typeof target> & { text: string };

export function ReportLintPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildReportLintPacket({ model, result, analysisRun });

  return (
    <section className="panel report-lint-panel" aria-label="Report content lint" data-testid="report-lint-panel">
      <div className="panel-title">
        <FileWarning size={16} aria-hidden="true" />
        Report Content Lint
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="report-lint-export-link"
          download={`openpipestress-preview-report-lint-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Lint JSON
        </a>
        <span data-testid="report-lint-summary">
          available; targets={packet.lint_run.summary.scanned_target_count}; findings=
          {packet.lint_run.summary.finding_count}; blocking={packet.lint_run.summary.blocking_finding_count}
        </span>
      </div>
      <div className="report-list" data-testid="report-lint-body">
        <LintLine
          label="Scan scope"
          value={`${packet.lint_run.configuration.scan_scope}; private=${packet.lint_run.configuration.private_surface_default}`}
          testId="report-lint-scope"
        />
        <LintLine
          label="Fixture policy"
          value={packet.linter_status.fixture_policy}
          testId="report-lint-fixture-policy"
        />
        <LintLine
          label="Clean scan"
          value={`clearance=${String(packet.lint_run.summary.clean_scan_is_clearance)}; heuristic_only=${String(
            packet.linter_status.heuristic_only
          )}`}
          testId="report-lint-clean-scan"
        />
        <LintLine
          label="Unit policy"
          value={`unit_targets=${packet.unit_policy_evidence.unit_policy_target_count}; conversion_witness_targets=${packet.unit_policy_evidence.conversion_witness_target_count}; lint_conversion=${String(
            packet.unit_policy_evidence.lint_performs_conversion
          )}; source=public_surface_inventory`}
          testId="report-lint-unit-policy"
        />
        <LintLine
          label="Boundary"
          value={`legal=${String(packet.linter_status.legal_clearance)}; professional=${String(
            packet.linter_status.professional_approval
          )}; ci=${packet.linter_status.ci_release_policy}; redaction=${packet.linter_status.redaction_export_controls}`}
          testId="report-lint-boundary"
        />
      </div>
      <small className="report-note">
        Lint output is deterministic technical-preview review evidence for public preview surfaces only; it is not legal
        clearance, redaction certification, or professional acceptance.
      </small>
    </section>
  );
}

function LintLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildReportLintPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const targets = lintTargets({ model, result, analysisRun });
  const findings = lintFindings(targets);
  const unitPolicyEvidence = buildUnitPolicyEvidence(targets);

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-08-05",
    package_id: "PKG-08",
    scope_item: "SOW-043",
    objectives: ["OBJ-002", "OBJ-007"],
    linter_status: {
      baseline_linter: "deterministic_public_surface_heuristic_linter",
      fixture_policy: "invented_synthetic_markers_only",
      heuristic_only: true,
      legal_clearance: false,
      security_sufficiency: false,
      professional_approval: false,
      ci_release_policy: "TBD",
      redaction_export_controls: "TBD",
      educational_example_dependency: "DAG-001-E0621_RETAINED_CANDIDATE_NON_GATING"
    },
    unit_policy_evidence: unitPolicyEvidence,
    lint_run: {
      run_id: `lint:report-preview:${safeFileToken(result?.run_id ?? "not-run")}`,
      configuration: {
        configuration_id: "desktop_public_preview_report_surfaces",
        scan_scope: "explicit_public_surfaces",
        public_surface_roots: [
          "apps/desktop/src/features/report/ReportPanel.tsx",
          "apps/desktop/src/features/report/RenderedReportPanel.tsx",
          "apps/desktop/src/features/results/ResultsPanel.tsx",
          "apps/desktop/src/features/comparison/ComparisonPanel.tsx",
          "apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx",
          "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
          "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx",
          "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
          "apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx",
          "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx",
          "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx",
          "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
          "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
          "apps/desktop/src/features/rule-packs/ExpressionComposer.tsx",
          "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
          "apps/desktop/src/features/operations/OperationApplyPanel.tsx",
          "apps/desktop/src/features/operations/OperationLedgerPanel.tsx",
          "apps/desktop/src/features/viewport/PipeViewport.tsx",
          "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
          "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx",
          "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx",
          "apps/desktop/src/features/knowledge/KnowledgePanel.tsx",
          "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx",
          "apps/desktop/src/features/run-audit/RunAuditPanel.tsx",
          "apps/desktop/src/features/solve/SolvePanel.tsx",
          "apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx",
          "apps/desktop/src/features/result-export/ResultExportPanel.tsx",
          "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx",
          "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx",
          "apps/desktop/src/features/handoff/HandoffPanel.tsx",
          "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
          "apps/desktop/src/features/native-package/NativePackagePanel.tsx",
          "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
          "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
          "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
          "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
          "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx",
          "apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx",
          "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
          "fixtures/product_preview"
        ],
        private_surface_default: "skip_unless_explicitly_authorized",
        finding_order: "path_line_column_code",
        synthetic_marker_policy: "fixtures_must_use_invented_markers_not_protected_examples",
        clean_scan_disclaimer: "heuristic_review_evidence_not_legal_or_professional_clearance",
        candidate_edge_policy: "DAG-001-E0621_non_gating_no_DEL-11-04_dependency"
      },
      targets: targets.map(({ text: _text, ...lintTarget }) => lintTarget),
      findings,
      summary: {
        target_count: targets.length,
        scanned_target_count: targets.length,
        skipped_private_target_count: 0,
        finding_count: findings.length,
        blocking_finding_count: findings.filter((item) => item.severity === "BLOCKING").length,
        clean_scan_is_clearance: false
      },
      provenance: previewProvenance()
    }
  };
}

const UNIT_POLICY_SURFACE_MARKERS = [
  unitPolicySurface(
    "apps/desktop/src/features/report/ReportPanel.tsx",
    "report-unit-system",
    "none",
    "report packet unit-system disclosure"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/report/RenderedReportPanel.tsx",
    "rendered-report-unit-basis",
    "none",
    "rendered calculation report unit-basis disclosure"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/results/ResultsPanel.tsx",
    "result-unit-policy",
    "none",
    "results viewer result-unit policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/comparison/ComparisonPanel.tsx",
    "comparison-unit-policy",
    "none",
    "comparison workspace unit-matching policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx",
    "agent-proposal-unit-policy",
    "none",
    "review-only proposal operation unit-validation policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
    "export-review-unit-policy",
    "none",
    "export safety review unit-policy inventory"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx",
    "project-storage-unit-round-trip",
    "none",
    "project storage audit unit round-trip policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
    "project-validation-unit-policy",
    "none",
    "project validation preflight unit round-trip policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx",
    "secret-private-unit-policy",
    "none",
    "secret/private-library metadata unit policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
    "security-threat-model-unit-policy",
    "none",
    "security threat-model unit-check no-bypass evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
    "editor-contract-unit-contract",
    "none",
    "editor contract unit-bearing operation rule"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx",
    "rule-pack-declarations-unit-policy",
    "none",
    "rule-pack declaration unit-reference policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/rule-packs/ExpressionComposer.tsx",
    "rule-pack-expression-unit-policy",
    "none",
    "rule-pack expression AST unit-reference policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/library/LibraryManagerPanel.tsx",
    "library-unit-helper-surfaces",
    "none",
    "material, section, and component library draft unit-helper surfaces"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx",
    "load-manager-unit-validation-surface",
    "none",
    "load-case and primitive-load operation unit-validation surfaces"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
    "property-inspector-unit-validation-surface",
    "none",
    "property inspector edit, create, and delete operation unit-validation surfaces"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
    "operation-diff-unit-witnesses",
    "none",
    "operation diff preview unit preservation witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/operations/OperationApplyPanel.tsx",
    "operation-apply-unit-policy",
    "none",
    "apply-operations queued and receipt unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/operations/OperationLedgerPanel.tsx",
    "operation-ledger-unit-policy",
    "none",
    "operation review ledger unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/viewport/PipeViewport.tsx",
    "viewport-operation-unit-validation",
    "none",
    "viewport structured-operation unit validation"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
    "missing-data-unit-policy",
    "none",
    "missing-data remediation unit-input policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx",
    "accessibility-baseline-unit-visibility",
    "none",
    "accessibility baseline unit-visibility inventory"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx",
    "design-workspace-units",
    "none",
    "design workspace result/comparison unit disclosure"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/knowledge/KnowledgePanel.tsx",
    "knowledge-unit-context",
    "none",
    "design knowledge computed-unit context"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx",
    "validation-evidence-units",
    "none",
    "validation evidence unit-policy visibility"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/run-audit/RunAuditPanel.tsx",
    "run-audit-units",
    "none",
    "analysis-run audit unit traceability"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/solve/SolvePanel.tsx",
    "solve-job-unit-policy",
    "none",
    "solve job audit unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx",
    "diagnostic-unit-context",
    "none",
    "diagnostic detail linked-result unit context"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx",
    "rule-check-unit-binding-policy",
    "none",
    "run-rule-check binding unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/rule-check/RuleCheckPanel.tsx",
    "rule-completeness-unit-policy",
    "none",
    "rule-completeness review explicit rule-input unit policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/result-export/ResultExportPanel.tsx",
    "result-export-unit-witnesses",
    "none",
    "result export envelope unit preservation witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx",
    "local-fea-unit-witnesses",
    "none",
    "local FEA handoff unit witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx",
    "headless-runner-unit-witnesses",
    "none",
    "headless runner result handoff unit preservation witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/handoff/HandoffPanel.tsx",
    "handoff-unit-witnesses",
    "none",
    "local review handoff package unit preservation witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
    "external-prover-units",
    "none",
    "external-prover metadata unit policy"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/native-package/NativePackagePanel.tsx",
    "native-package-unit-witnesses",
    "none",
    "native JSON package unit preservation witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
    "review-geometry-unit-witnesses",
    "none",
    "review geometry export unit witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
    "pcf-export-units",
    "pcf-export-conversion-witnesses",
    "PCF target-format conversion witness visibility"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
    "caepipe-mbf-units",
    "caepipe-mbf-conversion-witnesses",
    "CAEPIPE MBF target-format conversion witness visibility"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
    "caepipe-external-units",
    "none",
    "CAEPIPE external harness parser unit witnesses"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx",
    "adapter-framework-units",
    "none",
    "format-neutral adapter framework unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx",
    "export-adapter-sdk-units",
    "none",
    "export adapter SDK unit-policy evidence"
  ),
  unitPolicySurface(
    "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
    "stress-neutral-unit-witnesses",
    "none",
    "stress-neutral export unit witnesses"
  ),
  unitPolicySurface(
    "fixtures/product_preview",
    "product-preview-fixture-unit-policy",
    "none",
    "invented product-preview fixtures carry explicit model, rule-pack, and result units"
  )
] as const;

function unitPolicySurface(
  sourcePath: string,
  unitPolicySurfaceId: string,
  conversionWitnessSurfaceId: string,
  evidenceBasis: string
) {
  return {
    source_path: sourcePath,
    unit_policy_surface_id: unitPolicySurfaceId,
    conversion_witness_surface_id: conversionWitnessSurfaceId,
    evidence_basis: evidenceBasis
  };
}

function buildUnitPolicyEvidence(targets: LintTargetDraft[]) {
  const targetByPath = new Map(targets.map((item) => [item.path, item]));
  const targetRefs = UNIT_POLICY_SURFACE_MARKERS.flatMap((marker) => {
    const matchedTarget = targetByPath.get(marker.source_path);
    if (!matchedTarget) return [];
    return [
      {
        target_ref: reference("LintTarget", matchedTarget.target_id),
        source_path: marker.source_path,
        unit_policy_surface_id: marker.unit_policy_surface_id,
        conversion_witness_surface_id: marker.conversion_witness_surface_id,
        evidence_basis: marker.evidence_basis
      }
    ];
  });
  const conversionWitnessTargetCount = targetRefs.filter(
    (item) => item.conversion_witness_surface_id !== "none"
  ).length;

  return {
    evidence_id: "unit-policy-evidence:report-lint-public-surfaces",
    evidence_kind: "public_surface_unit_policy_inventory",
    unit_policy: "lint_targets_include_public_unit_policy_and_conversion_witness_surfaces",
    scanned_target_count: targets.length,
    unit_policy_target_count: targetRefs.length,
    conversion_witness_target_count: conversionWitnessTargetCount,
    lint_performs_conversion: false,
    lint_asserts_target_format_compatibility: false,
    target_conversion_claims_are_linted_only: true,
    source: "ReportLintPanel explicit public-surface inventory",
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-08-05")
    ],
    target_refs: targetRefs
  };
}

function lintTargets({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}): LintTargetDraft[] {
  const targets = [
    target(
      "target:desktop-report-template",
      "apps/desktop/src/features/report/ReportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Report packet template uses metadata, diagnostics, limitations, and no professional approval claim."
    ),
    target(
      "target:desktop-rendered-report-template",
      "apps/desktop/src/features/report/RenderedReportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Rendered report template records renderable-report unit basis, canonical HTML hash route, export-blocked state, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-results-viewer-template",
      "apps/desktop/src/features/results/ResultsPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Results viewer template records solved result groups, result-unit policy, row count, filtering state, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-comparison-workspace-template",
      "apps/desktop/src/features/comparison/ComparisonPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Comparison workspace template records explicit-unit matching policy, tolerance status, mapped result counts, comparison diagnostics, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-agent-proposal-template",
      "apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Agent proposal template records review-only operation proposal unit-validation status, user-acceptance boundary, accepted-state mutation false, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-export-review-template",
      "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Export review template records local preview exports, private payload false, protected payload false, and no release or professional claim."
    ),
    target(
      "target:desktop-project-storage-template",
      "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Project storage audit template records local project persistence, unit round-trip evidence, local-only boundary, external file copy false, private payload false, protected payload false, and no release or professional claim."
    ),
    target(
      "target:desktop-project-validation-template",
      "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Project validation preflight template records schema/version checks, migration evidence, hash verification, unit round-trip evidence, private payload false, protected payload false, and no release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-build-readiness-template",
      "apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Build readiness template records provider-neutral local scripts, Tauri preview shell metadata, private payload false, protected payload false, unresolved CI and release decisions, and no release authorization, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-telemetry-boundary-template",
      "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Telemetry boundary template records disabled default-off configuration, opt-in false, allowlist false, blocked event attempts, payload construction false, network initialization false, endpoint and vendor false, private payload false, protected payload false, and no security certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-secret-private-library-template",
      "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Secret and private-library template records metadata-only private library, private path, and credential references, public fixture blocking, public report reference-only handling, direct SQL false, storage bypass false, network false, private payload false, protected payload false, and no security certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-security-threat-model-template",
      "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Security threat-model template records DEL-12-05 metadata-only threat inventory counts, local-first posture, telemetry default-off, no-bypass controls, open TBD decisions, private payload false, protected payload false, and no security sufficiency, release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-editor-contract-template",
      "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Editor contract template records material/component/rule-pack reference editor metadata, operation-intent routing, private library references only, private payload false, protected payload false, accepted state mutation false, and no approval, authentication, certification, sealing, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-rule-pack-declarations-template",
      "apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx",
      "public_report_template",
      "public_metadata",
      "Rule-pack declarations template records required-input and value-slot unit-reference policy, browser manual-entry route, stored unit refs preserved, conversion false, private payload false, protected payload false, and no parser, release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-rule-pack-expression-template",
      "apps/desktop/src/features/rule-packs/ExpressionComposer.tsx",
      "public_report_template",
      "public_metadata",
      "Rule-pack expression composer template records structured AST unit-reference policy, browser manual-entry route, stored expression unit refs preserved, conversion false, private payload false, protected payload false, read-only text preview only, and no parser, release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-library-manager-template",
      "apps/desktop/src/features/library/LibraryManagerPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Library manager template records material, section, and component draft unit helpers, DEC-018-filtered unit choices, private local-import boundary, conversion false, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-load-case-manager-template",
      "apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Load case manager template records load-case, primitive-load, and combination operation intents with explicit unit-validation status, direct model mutation false, conversion false, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-property-inspector-template",
      "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      "public_report_template",
      "public_metadata",
      "Property inspector template records edit, material, section, support, node-delete, pipe-delete, and support-delete operation intents with explicit unit-validation status, direct model mutation false, conversion false, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-operation-diff-template",
      "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Operation diff preview template records local review-only operation diffs, declared change units and dimensions, unit preservation witnesses, accepted state mutation false, private payload false, protected payload false, and no approval, certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-operation-apply-template",
      "apps/desktop/src/features/operations/OperationApplyPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Apply Operations template records queued structured operations, unit-validation outcomes, session-local applied receipts, receipt unit-serialization policy, private payload false, protected payload false, and no approval, certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-operation-ledger-template",
      "apps/desktop/src/features/operations/OperationLedgerPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Operation review ledger template records local review-only operation records, unit-policy evidence, held user-acceptance decisions, accepted state mutation false, private payload false, protected payload false, and no approval, certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-viewport-editor-template",
      "apps/desktop/src/features/viewport/PipeViewport.tsx",
      "public_report_template",
      "public_metadata",
      "Viewport editor template records create-node, pipe-run, and component-symbol gesture intents as review-only structured operations, pending service validation, direct mutation false, private payload false, protected payload false, and no approval, certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-missing-data-template",
      "apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Missing-data warning template records solve-required versus rule-check-required warning classes, text remediation fields, silent defaults false, auto-fill false, private payload false, protected payload false, accepted state mutation false, and no approval, authentication, certification, sealing, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-accessibility-baseline-template",
      "apps/desktop/src/features/accessibility-baseline/AccessibilityBaselinePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Accessibility baseline template records DEL-07-06 deterministic GUI-contract evidence, target TBD, desktop runtime evaluation not performed, color-only status signaling false, private payload false, protected payload false, and no accessibility conformance, release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-design-workspace-template",
      "apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Design workspace template records DEL-07-08 design knowledge, constraint warning, state/run, comparison, overlay, and operation-diff review metadata, accepted model mutation false, private payload false, protected payload false, and no acceptance, release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-design-knowledge-template",
      "apps/desktop/src/features/knowledge/KnowledgePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Design knowledge template records local invented knowledge records, computed preview result unit context, provenance and review metadata, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-validation-evidence-template",
      "apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Validation evidence template records DEL-09-04 and DEL-09-05 manual sections, evidence areas, local readiness profiles, release-gate families, unresolved TBD decisions, browser tool execution false, private payload false, protected payload false, and no release authorization, certification, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-run-audit-template",
      "apps/desktop/src/features/run-audit/RunAuditPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Run audit template records local analysis-run audit metadata, model-state and result hash references, result-envelope unit traceability, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-solve-job-template",
      "apps/desktop/src/features/solve/SolvePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Solve job template records local solve execution audit metadata, model and result unit-policy evidence, cancellation and progress metadata, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-diagnostics-template",
      "apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Diagnostics template records selected diagnostic source, affected refs, linked result unit context, review explanation, professional boundary, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-rule-check-run-template",
      "apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Run Rule Checks template records private rule-pack binding routes, runtime value unit policy, solver-result and private-library binding counts, private payload false, protected payload false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-rule-completeness-template",
      "apps/desktop/src/features/rule-check/RuleCheckPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Rule-check completeness template records missing user/private rule-check data, explicit rule-input unit policy, unit-mismatch diagnostic policy, mechanics separation, private payload false, protected payload false, and no code-compliance, approval, certification, or professional reliance claim."
    ),
    target(
      "target:desktop-result-export-template",
      "apps/desktop/src/features/result-export/ResultExportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Result export template records schema-first local JSON result envelopes, explicit result units, unit preservation witnesses, private payload false, protected payload false, and no release, target compatibility, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-local-fea-handoff-template",
      "apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Local FEA handoff template records target-neutral selected-region metadata, result-reference transfer basis, mesh not generated, external solver not invoked, target format TBD, private payload false, protected payload false, and no approval, authentication, certification, sealing, or code-compliance claim."
    ),
    target(
      "target:desktop-headless-runner-template",
      "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Headless runner template records schema-first preview request/result handoff metadata, result-reference unit preservation witnesses, final CLI and process policy deferred, network false, private payload false, protected payload false, and no release, target compatibility, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-handoff-package-template",
      "apps/desktop/src/features/handoff/HandoffPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Handoff package template records stable IDs, local review handoff metadata, result-reference unit preservation witnesses, target-specific mapping deferred, private payload false, protected payload false, and no target compatibility, approval, certification, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-external-prover-template",
      "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "External prover boundary template records metadata-only names, tags, references, attachment refs, unsupported target flags, private payload false, and no software authority claim."
    ),
    target(
      "target:desktop-native-package-template",
      "apps/desktop/src/features/native-package/NativePackagePanel.tsx",
      "public_report_template",
      "public_metadata",
      "Native JSON package template records project-owned package members, stable IDs, unit preservation witnesses, private payload false, protected payload false, target compatibility false, and no release, code-compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-review-geometry-template",
      "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Review geometry template records glTF preview geometry, stable IDs, private payload false, protected payload false, visual review only, no solver geometry equivalence claim."
    ),
    target(
      "target:desktop-pcf-export-template",
      "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Conservative PCF export template records ASCII PCF preview text, stable ID sidecar, mandatory loss report, private payload false, protected payload false, target profile TBD, and no target compatibility, solver validation, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-caepipe-mbf-export-template",
      "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "CAEPIPE MBF export template records invented ASCII MBF preview text, sidecar stable IDs, mandatory loss report, carried target-version and record-subset TBDs, private payload false, protected payload false, external invocation false, and no target compatibility, solver validation, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-caepipe-external-template",
      "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
      "public_report_template",
      "public_metadata",
      "CAEPIPE external harness template records parser-only invented CSV rows, absent executable configuration, external invocation false, user-owned live execution TBDs, private payload false, protected payload false, and no CAEPIPE compatibility, solver validation, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-adapter-framework-template",
      "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Adapter framework template records format-neutral adapter declaration metadata, unit-validation policy evidence, no-bypass controls, concrete external formats deferred, private payload false, protected payload false, and no support, compatibility, release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-export-adapter-sdk-template",
      "apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Export adapter SDK template records target-registry preview metadata, additional target source-basis admission TBDs, deny-by-default permissions, private payload false, protected payload false, and no support, compatibility, release, code compliance, or professional reliance claim."
    ),
    target(
      "target:desktop-stress-neutral-template",
      "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Stress-neutral export template records CSV/JSON preview rows, explicit units and dimensions, stable IDs, private payload false, protected payload false, diagnostic-only comparison semantics, no vendor format, solver validation, code compliance, or professional reliance claim."
    ),
    target(
      "target:invented-preview-fixture",
      "fixtures/product_preview",
      "public_fixture",
      "invented_public_example",
      `${model.project.id}; ${model.data_boundary.public_examples_policy}; ${model.data_boundary.protected_source_policy}; unit_system=${model.project.units.length},${model.project.units.force},${model.project.units.stress}; invented fixtures include explicit model quantities, rule-pack unit refs, and mechanics-result units`
    )
  ];

  if (result && analysisRun) {
    targets.push(
      target(
        "target:report-packet-preview-json",
        `openpipestress-preview-report-${safeFileToken(result.run_id)}.json`,
        "public_report_example",
        "public_metadata",
        `${result.model_ref}; ${analysisRun.analysis_run.run_id}; ${result.results.length} result refs; private rule payloads redacted`
      )
    );
  }

  return targets;
}

function lintFindings(targets: LintTargetDraft[]) {
  const findings = [];
  for (const item of targets) {
    const matchedCode = protectedMarkerCode(item.text) ?? prohibitedClaimCode(item.text);
    if (!matchedCode) continue;
    findings.push({
      finding_id: `finding:${safeFileToken(item.target_id)}:${findings.length + 1}`,
      code: matchedCode.code,
      class: matchedCode.class,
      severity: matchedCode.severity,
      target_ref: reference("LintTarget", item.target_id),
      source_location: {
        path: item.path,
        line: 1,
        column: 1
      },
      matched_policy: matchedCode.policy,
      excerpt: matchedCode.excerpt,
      message: matchedCode.message,
      remediation: matchedCode.remediation,
      review_route: matchedCode.reviewRoute,
      disposition: "pending",
      provenance: previewProvenance()
    });
  }
  return findings;
}

function protectedMarkerCode(text: string) {
  const markerPolicies = [
    {
      marker: "PROTECTED_CONTENT_SYNTHETIC_MARKER",
      code: "PROTECTED_CONTENT_SYNTHETIC_MARKER",
      class: "IP_BOUNDARY_WARNING",
      severity: "BLOCKING",
      policy: "Synthetic protected-content marker indicates review-blocking public-surface risk.",
      reviewRoute: "HUMAN_IP_REVIEW"
    },
    {
      marker: "PRIVATE_DATA_SYNTHETIC_MARKER",
      code: "PRIVATE_DATA_SYNTHETIC_MARKER",
      class: "PRIVATE_DATA_WARNING",
      severity: "BLOCKING",
      policy: "Synthetic private-data marker indicates user/private payload risk.",
      reviewRoute: "HUMAN_PRIVACY_REVIEW"
    },
    {
      marker: "PROPRIETARY_SOURCE_SYNTHETIC_MARKER",
      code: "PROPRIETARY_SOURCE_SYNTHETIC_MARKER",
      class: "IP_BOUNDARY_WARNING",
      severity: "BLOCKING",
      policy: "Synthetic proprietary-source marker indicates redistribution-rights risk.",
      reviewRoute: "HUMAN_IP_REVIEW"
    }
  ] as const;
  const match = markerPolicies.find((item) => text.includes(item.marker));
  if (!match) return null;
  return {
    ...match,
    excerpt: match.marker,
    message: "Synthetic marker found in an explicit public report surface.",
    remediation: "Remove the marked content from public artifacts or route it through quarantine and human review."
  };
}

function prohibitedClaimCode(text: string) {
  const normalized = text.toLowerCase();
  const prohibitedClaims = ["software certifies", "software seals", "software approves", "code compliant for reliance"];
  const claim = prohibitedClaims.find((item) => normalized.includes(item));
  if (!claim) return null;
  return {
    code: "PROHIBITED_PROFESSIONAL_CLAIM",
    class: "PROFESSIONAL_BOUNDARY_WARNING",
    severity: "BLOCKING",
    policy: "Public report surfaces must not claim certification, sealing, approval, authentication, or code compliance.",
    excerpt: claim,
    message: "A public report surface appears to overstate software or professional authority.",
    remediation: "Rewrite the text as decision-support evidence with human-review-required boundary language.",
    reviewRoute: "HUMAN_PROFESSIONAL_BOUNDARY_REVIEW"
  };
}

function target(
  targetId: string,
  path: string,
  surface: "public_report_template" | "public_report_example" | "public_fixture",
  privacyClassification: "public_metadata" | "invented_public_example",
  text: string
) {
  return {
    target_id: targetId,
    path,
    surface,
    privacy_classification: privacyClassification,
    provenance: previewProvenance(),
    text
  };
}

function reference(refType: string, refId: string) {
  return {
    ref_type: refType,
    ref_id: refId
  };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/report-lint/ReportLintPanel.tsx",
    source_license: "PolyForm-Noncommercial-1.0.0 project license context",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented preview metadata only; no protected standards or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending",
    privacy_classification: "public_metadata"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
