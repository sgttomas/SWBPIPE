import { Download, Wrench } from "lucide-react";
import type { PreviewModel } from "../../types";

const ROOT_SCRIPTS = [
  "dev:desktop",
  "build:desktop",
  "test:desktop",
  "generate:product-preview-mechanics"
] as const;

const DESKTOP_SCRIPTS = ["dev", "build", "test", "tauri"] as const;

const READINESS_PROFILES = ["skeleton", "python", "security", "cargo", "all"] as const;

export function BuildReadinessPanel({ model }: { model: PreviewModel }) {
  const packet = buildReadinessPacket(model);
  return (
    <section
      className="panel build-readiness-panel"
      aria-label="Build package readiness"
      data-testid="build-readiness-panel"
    >
      <div className="panel-title">
        <Wrench size={16} aria-hidden="true" />
        Build/Package Readiness
      </div>
      <div className="report-actions">
        <ControlledExportLink
          className="report-export-link"
          data-testid="build-readiness-export-link"
          download={`openpipestress-preview-build-readiness-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Readiness JSON
        </ControlledExportLink>
        <span data-testid="build-readiness-summary">
          available; scripts={packet.summary.script_count}; tauri={packet.summary.tauri_shell_status};
          bundle_active={String(packet.summary.bundle_active)}; diagnostics={packet.diagnostics.length}
        </span>
      </div>
      <div className="report-list" data-testid="build-readiness-body">
        <BuildReadinessLine
          label="Desktop shell"
          value={`${packet.tauri_shell.product_name}; version=${packet.tauri_shell.version}; identifier=${packet.tauri_shell.identifier}`}
          testId="build-readiness-shell"
        />
        <BuildReadinessLine
          label="Local commands"
          value={`root=${packet.workspace.root_scripts.join(", ")}; desktop=${packet.workspace.desktop_scripts.join(", ")}`}
          testId="build-readiness-commands"
        />
        <BuildReadinessLine
          label="Readiness profiles"
          value={`${packet.release_readiness_tool.profiles.join(", ")}; default=${packet.release_readiness_tool.default_profile}; execute_default=false`}
          testId="build-readiness-profiles"
        />
        <BuildReadinessLine
          label="Release decisions"
          value={`ci=${packet.unresolved_decisions.ci_provider}; matrix=${packet.unresolved_decisions.release_matrix}; signing=${packet.unresolved_decisions.signing_notarization}; publishing=${packet.unresolved_decisions.publishing}`}
          testId="build-readiness-decisions"
        />
        <BuildReadinessLine
          label="Boundary"
          value={`network=${String(packet.boundary.network_required)}; telemetry=${String(
            packet.boundary.telemetry_enabled
          )}; release_authorized=${String(packet.boundary.release_publication_authorized)}; professional_claim=${String(
            packet.boundary.professional_reliance_claim
          )}`}
          testId="build-readiness-boundary"
        />
      </div>
      <small className="report-note">
        Build/package readiness is local evidence only; it does not select CI, publish packages, sign binaries, or
        authorize release. Technical preview — not a released product.
      </small>
    </section>
  );
}

function BuildReadinessLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildReadinessPacket(model: PreviewModel) {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.build_package_readiness",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-10-04",
    package_id: "PKG-10",
    scope_item: "SOW-032",
    objectives: ["OBJ-008", "OBJ-009"],
    project_ref: model.project.id,
    source_basis: {
      governance_docs: [
        "docs/BUILD_AND_RELEASE.md",
        "docs/RELEASE_QUALITY_GATES.md",
        "docs/VALIDATION_STRATEGY.md",
        "docs/IP_AND_DATA_BOUNDARY.md"
      ],
      manifests: ["package.json", "apps/desktop/package.json", "apps/desktop/src-tauri/Cargo.toml"],
      tauri_config: "apps/desktop/src-tauri/tauri.conf.json",
      readiness_tool: "tools/release/check_release_readiness.py"
    },
    summary: {
      readiness_status: "local_preview_build_package_surface_available",
      script_count: ROOT_SCRIPTS.length + DESKTOP_SCRIPTS.length,
      tauri_shell_status: "present",
      tauri_major_version: "2",
      bundle_active: false,
      provider_neutral_readiness_tool_available: true,
      release_authorization_status: "not_authorized",
      ci_provider: "TBD",
      release_matrix: "TBD"
    },
    workspace: {
      root_package_name: "openpipestress-workspace",
      root_package_version: "0.1.0",
      workspaces: ["apps/desktop"],
      root_scripts: ROOT_SCRIPTS,
      desktop_package_name: "@openpipestress/desktop",
      desktop_package_version: "0.1.0",
      desktop_private_package: true,
      desktop_module_type: "module",
      desktop_scripts: DESKTOP_SCRIPTS,
      desktop_dependencies: [
        "@tauri-apps/api",
        "@vitejs/plugin-react",
        "three",
        "vite",
        "react",
        "react-dom",
        "lucide-react"
      ],
      desktop_dev_dependencies: [
        "@tauri-apps/cli",
        "@testing-library/jest-dom",
        "@testing-library/react",
        "@types/node",
        "@types/react",
        "@types/react-dom",
        "@types/three",
        "jsdom",
        "typescript",
        "vitest"
      ]
    },
    tauri_shell: {
      product_name: "OpenPipeStress Technical Preview",
      version: "0.1.0",
      identifier: "org.openpipestress.technical-preview",
      before_dev_command: "npm run dev",
      dev_url: "http://127.0.0.1:5173",
      before_build_command: "npm run build",
      frontend_dist: "../dist",
      window_title: "OpenPipeStress Technical Preview",
      window_width: 1440,
      window_height: 920,
      min_width: 1160,
      min_height: 760,
      csp_status: "TBD_null_in_technical_preview_config",
      bundle_active: false,
      bundle_icon_count: 0,
      installer_format: "TBD",
      signing_status: "TBD",
      notarization_status: "TBD"
    },
    release_readiness_tool: {
      command: "python3 tools/release/check_release_readiness.py --profile skeleton",
      execute_command: "python3 tools/release/check_release_readiness.py --profile skeleton --execute",
      default_profile: "skeleton",
      profiles: READINESS_PROFILES,
      browser_panel_runs_tool: false,
      local_shell_validation_required_for_tranche: true,
      dry_run_default: true,
      external_services_required: false,
      shell_command_evaluation: false
    },
    unresolved_decisions: {
      ci_provider: "TBD",
      hosted_workflow_location: "TBD",
      release_matrix: "TBD",
      installer_formats: "TBD",
      signing_notarization: "TBD",
      checksum_publication: "TBD",
      release_attestation: "TBD",
      publishing: "TBD",
      coverage_thresholds: "TBD",
      performance_thresholds: "TBD",
      maintainer_quorum: "TBD",
      release_authority: "TBD"
    },
    diagnostics: [
      diagnostic(
        "BUILD-READINESS-PROVIDER-NEUTRAL",
        "info",
        "Local package and Tauri shell metadata are visible without selecting a CI provider."
      ),
      diagnostic(
        "BUILD-READINESS-BUNDLE-INACTIVE",
        "warning",
        "Tauri bundle.active is false, so this preview does not claim installer or binary package generation."
      ),
      diagnostic(
        "BUILD-READINESS-RELEASE-DECISIONS-TBD",
        "warning",
        "CI provider, release matrix, signing, notarization, publishing, and thresholds remain TBD."
      ),
      diagnostic(
        "BUILD-READINESS-LOCAL-VALIDATION-REQUIRED",
        "info",
        "Desktop tranches still require local npm test/build and applicable smoke evidence outside this browser panel."
      )
    ],
    boundary: {
      local_only_preview: true,
      network_required: false,
      daemon_required: false,
      telemetry_enabled: false,
      private_payload_included: false,
      protected_content_included: false,
      release_publication_authorized: false,
      installer_or_binary_generated: false,
      signing_secret_required: false,
      source_model_mutated: false,
      release_or_professional_claim: false,
      code_compliance_claim: false,
      professional_reliance_claim: false
    },
    professional_boundary: professionalBoundary(),
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false
  };
}

function diagnostic(code: string, severity: "info" | "warning", message: string) {
  return {
    code,
    severity,
    message,
    source: "apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx"
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
