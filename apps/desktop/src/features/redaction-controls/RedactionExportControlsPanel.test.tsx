// TP-E4-REDACTION-001 (DEL-12-02): user-facing redaction warnings and export
// gating. The panel must (a) default to the private local context, (b) refuse
// to offer any export artifact while a blocking finding exists, (c) surface
// clear warnings BEFORE a private-value export is offered, (d) redact private
// values for public/shared contexts, and (e) checksum the offered manifest.
// All fixture data is invented.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PackageHashEvidence, PreviewModel } from "../../types";
import { RedactionExportControlsPanel } from "./RedactionExportControlsPanel";

// The package hash is the real one unless a test resolves it by hand.
const hashing = vi.hoisted(() => ({
  byHand: false,
  pending: [] as Array<{ packet: unknown; resolve: (hash: PackageHashEvidence) => void }>
}));
vi.mock("../../services/hashService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/hashService")>();
  return {
    ...actual,
    computePackageHash: vi.fn((...args: Parameters<typeof actual.computePackageHash>) =>
      hashing.byHand
        ? new Promise<PackageHashEvidence | null>((resolve) => hashing.pending.push({ packet: args[1], resolve }))
        : actual.computePackageHash(...args)
    )
  };
});

afterEach(cleanup);
afterEach(() => {
  hashing.byHand = false;
  hashing.pending.length = 0;
});

const PRIVATE_MATERIAL_LABEL = "User entered alloy placeholder";

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.9.0",
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

  it("offers a changed context's manifest with its checksum pending, never with the previous context's checksum (issue 497)", async () => {
    hashing.byHand = true;
    const handHash = (label: string): PackageHashEvidence => ({
      algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "package_review_payload",
      payload_excludes: "manifest_and_validation_report_package_hash_carrier_fields", payload_ref: label,
      value: `sha256:${label}`, hash_status: "computed_local_preview"
    });
    const resolveLatest = (context: string, label: string) => {
      const computation = [...hashing.pending].reverse()
        .find((item) => (item.packet as { export_run: { export_context: string } }).export_run.export_context === context);
      expect(computation, `a checksum computation for ${context}`).toBeDefined();
      return act(async () => computation!.resolve(handHash(label)));
    };
    const offered = (href: string | null) => {
      expect(href?.startsWith("data:application/json")).toBe(true);
      const manifest = JSON.parse(decodeURIComponent(href!.split(",", 2)[1])) as {
        export_run: { export_context: string };
        checksum: { hash_status: string; value?: string };
      };
      return { context: manifest.export_run.export_context, checksum: manifest.checksum };
    };
    render(<RedactionExportControlsPanel model={inventedModel()} />);

    // Context A: the local private export with explicit intent, its checksum resolved.
    fireEvent.click(screen.getByTestId("redaction-intent-toggle"));
    await resolveLatest("local_private", "context-a");
    const link = screen.getByTestId("redaction-export-link");
    expect(offered(link.getAttribute("href"))).toEqual({ context: "local_private", checksum: handHash("context-a") });

    // Switch to the public report context, recording the href of every commit: each
    // mutation record holds the value before it, and the attribute holds the last one.
    const observer = new MutationObserver(() => undefined);
    observer.observe(link, { attributes: true, attributeFilter: ["href"], attributeOldValue: true });
    fireEvent.change(screen.getByTestId("redaction-context-select"), { target: { value: "public_report" } });
    const records = observer.takeRecords();
    observer.disconnect();
    expect(screen.getByTestId("redaction-export-link")).toBe(link);
    expect(records.length).toBeGreaterThan(0);
    expect(offered(records[0].oldValue).context).toBe("local_private");
    const committed = [...records.slice(1).map((record) => record.oldValue), link.getAttribute("href")].map(offered);

    // The first commit after the switch already offers B's manifest with its checksum pending.
    expect(committed[0]).toEqual({ context: "public_report", checksum: { hash_status: "pending_local_preview_hash" } });
    expect(committed.every((item) => item.context === "public_report")).toBe(true);
    expect(committed.filter((item) => item.checksum.value === "sha256:context-a")).toEqual([]);

    await resolveLatest("public_report", "context-b");
    expect(offered(link.getAttribute("href"))).toEqual({ context: "public_report", checksum: handHash("context-b") });
  });

  it("keeps the informational boundary language on screen", () => {
    render(<RedactionExportControlsPanel model={inventedModel()} />);
    const panelText = screen.getByTestId("redaction-controls-panel").textContent ?? "";
    expect(panelText).toContain("do not certify redaction sufficiency");
    expect(panelText).toContain("nothing is transmitted");
  });
});
