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
    lint_run: {
      run_id: `lint:report-preview:${safeFileToken(result?.run_id ?? "not-run")}`,
      configuration: {
        configuration_id: "desktop_public_preview_report_surfaces",
        scan_scope: "explicit_public_surfaces",
        public_surface_roots: [
          "apps/desktop/src/features/report/ReportPanel.tsx",
          "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
          "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
          "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
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
      "target:desktop-export-review-template",
      "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Export review template records local preview exports, private payload false, protected payload false, and no release or professional claim."
    ),
    target(
      "target:desktop-external-prover-template",
      "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "External prover boundary template records metadata-only names, tags, references, attachment refs, unsupported target flags, private payload false, and no software authority claim."
    ),
    target(
      "target:desktop-review-geometry-template",
      "apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx",
      "public_report_template",
      "public_metadata",
      "Review geometry template records glTF preview geometry, stable IDs, private payload false, protected payload false, visual review only, no solver geometry equivalence claim."
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
      `${model.project.id}; ${model.data_boundary.public_examples_policy}; ${model.data_boundary.protected_source_policy}`
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
