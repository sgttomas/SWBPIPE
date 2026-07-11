// TP-E4-REDACTION-001 (DEL-12-02): user-facing redaction warnings and export
// gating. The panel must (a) default to the private local context, (b) refuse
// to offer any export artifact while a blocking finding exists, (c) surface
// clear warnings BEFORE a private-value export is offered, (d) redact private
// values for public/shared contexts, and (e) checksum the offered manifest.
// All fixture data is invented.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { RedactionExportControlsPanel } from "./RedactionExportControlsPanel";

afterEach(cleanup);

const PRIVATE_MATERIAL_LABEL = "User entered alloy placeholder";

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: {
      id: "project:redaction-panel-test",
      name: "Invented Redaction Panel Project",
      description: "Invented panel fixture project description",
      units: { length: "m", force: "N", angle: "rad", pressure: "Pa", temperature: "degC" }
    },
    analysis_status: { mechanics: "ready", rule_check: "not_performed", professional_acceptance: "not_provided" },
    materials: [
      {
        id: "material:M-INVENTED",
        label: "Invented example material",
        elastic_modulus: { value: 199000000000, unit: "Pa" },
        shear_modulus: { value: 76000000000, unit: "Pa" },
        // Live-fixture style invented marker with a no-catalog qualifier
        // suffix; must map to the safe invented classes.
        provenance: "invented_example_no_material_standard"
      },
      {
        id: "material:M-USER",
        label: PRIVATE_MATERIAL_LABEL,
        elastic_modulus: { value: 201000000000, unit: "Pa" },
        shear_modulus: { value: 77500000000, unit: "Pa" },
        provenance: "user_entered"
      }
    ],
    nodes: [],
    pipe_segments: [],
    supports: [],
    components: [
      {
        id: "component:C-USER",
        label: "User entered bend placeholder",
        kind: "bend",
        node: "node:N-1",
        provenance: "user_entered"
      }
    ],
    load_cases: [],
    diagnostics: []
  } as unknown as PreviewModel;
}

function exportedManifest(): Record<string, unknown> {
  const link = screen.getByTestId("redaction-export-link");
  const href = link.getAttribute("href") ?? "";
  expect(href.startsWith("data:application/json")).toBe(true);
  return JSON.parse(decodeURIComponent(href.split(",", 2)[1])) as Record<string, unknown>;
}

describe("RedactionExportControlsPanel", () => {
  it("defaults to the private local context and withholds the export until explicit intent is recorded", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    const select = screen.getByTestId("redaction-context-select") as HTMLSelectElement;
    expect(select.value).toBe("local_private");

    expect(screen.getByTestId("redaction-export-blocked")).toBeTruthy();
    expect(screen.queryByTestId("redaction-export-link")).toBeNull();
    expect(screen.getAllByTestId("redaction-finding-LOCAL_PRIVATE_INTENT_REQUIRED").length).toBeGreaterThan(0);
    expect(screen.getByTestId("redaction-posture").textContent).toContain("private_by_default=true");
  });

  it("offers the local private export with explicit intent, retaining private values behind a visible warning", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    fireEvent.click(screen.getByTestId("redaction-intent-toggle"));

    expect(screen.queryByTestId("redaction-export-blocked")).toBeNull();
    expect(screen.getAllByTestId("redaction-finding-PRIVATE_LOCAL_ALLOWED_WITH_WARNING").length).toBeGreaterThan(0);

    const manifest = exportedManifest();
    const serialized = JSON.stringify(manifest);
    expect(serialized).toContain(PRIVATE_MATERIAL_LABEL);
    const boundary = manifest.boundary as Record<string, unknown>;
    expect(boundary.private_payload_included).toBe(true);
    expect(boundary.release_or_professional_claim).toBe(false);
    const exportRun = manifest.export_run as Record<string, unknown>;
    expect(exportRun.explicit_local_private_intent).toBe(true);
  });

  it("redacts private values for a public report export and keeps invented-example records", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    fireEvent.change(screen.getByTestId("redaction-context-select"), { target: { value: "public_report" } });

    expect(screen.queryByTestId("redaction-export-blocked")).toBeNull();
    expect(screen.getAllByTestId("redaction-finding-PRIVATE_DATA_REDACTED").length).toBeGreaterThan(0);

    const manifest = exportedManifest();
    const serialized = JSON.stringify(manifest.redacted_payload);
    expect(serialized).not.toContain(PRIVATE_MATERIAL_LABEL);
    expect(serialized).not.toContain("Invented Redaction Panel Project");
    expect(serialized).toContain("[REDACTED]");
    expect(serialized).toContain("Invented example material");
    const boundary = manifest.boundary as Record<string, unknown>;
    expect(boundary.private_payload_included).toBe(false);
  });

  it("disables the intent toggle outside the local private context", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    fireEvent.change(screen.getByTestId("redaction-context-select"), { target: { value: "shared_model" } });

    const toggle = screen.getByTestId("redaction-intent-toggle") as HTMLInputElement;
    expect(toggle.disabled).toBe(true);
  });

  it("computes a canonical sha256 checksum for the offered manifest", async () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    fireEvent.click(screen.getByTestId("redaction-intent-toggle"));

    await waitFor(() => {
      expect(screen.getByTestId("redaction-checksum").textContent).toContain("sha256:");
    });
  });

  it("keeps the informational boundary language on screen", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);
    const panelText = screen.getByTestId("redaction-controls-panel").textContent ?? "";
    expect(panelText).toContain("do not certify redaction sufficiency");
    expect(panelText).toContain("nothing is transmitted");
  });
});
