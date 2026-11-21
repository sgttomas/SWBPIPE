import { act, render } from "@testing-library/react";
import { createElement, type ReactNode } from "react";
import { describe, expect, it, vi } from "vitest";
import { NativePackagePanel, buildNativePackageReview, buildUnitPreservationEvidence } from "./NativePackagePanel";
import type { PackageHashEvidence } from "../../types";

// The package hash comes from a computation the test resolves by hand, and the export link
// records the href it is given on every render; the other hash services are the real ones.
const mocked = vi.hoisted(() => ({
  packageHashes: [] as Array<{ packet: unknown; resolve: (hash: PackageHashEvidence) => void }>,
  hrefs: [] as string[]
}));
vi.mock("../../services/hashService", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../../services/hashService")>()),
  computePackageHash: vi.fn(
    (_packageId: string, packet: unknown) =>
      new Promise<PackageHashEvidence | null>((resolve) => mocked.packageHashes.push({ packet, resolve }))
  )
}));
vi.mock("../redaction-controls/ControlledExportLink", () => ({
  ControlledExportLink: ({ href, children, ...anchorProps }: { href: string; children: ReactNode }) => {
    mocked.hrefs.push(href);
    return createElement("a", { ...anchorProps, href }, children);
  }
}));
import { loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";

// Pure received-hash inspection of preserved reference data. This constructs
// an analysis record, never a native invocation or Current/qualified export.
async function referenceAnalysis() {
  // The bundled reference carries its own demo model; the session model is not its basis.
  const reference = await loadBundledMechanicsReference(), model = reference.model;
  const result = reference.source;
  const manifest = {
    model_basis: { model_ref: model.project.id, model_payload: model },
    solver_basis: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:preserved-reference-not-native-invocation" },
  };
  const basis = { manifest, manifest_ref: { object_type: "InputManifest", ref: "test:reference-analysis-inputs" }, manifest_sha256: await canonicalSha256HexCheckedV1(manifest) };
  const analysisRun = await buildAnalysisRunV03(result, basis, undefined, modelLoadBasisRefs(model));
  const receivedHash = analysisRun.analysis_run.hashes.find(hash => hash.payload_scope === "received_result")!.value;
  expect(reference.standing).toBe("reference_only");
  expect(reference.provenance.current_use_eligible).toBe(false);
  expect(hasNativeMechanicsInvocation(result, model)).toBe(false);
  expect(analysisRun.schema_version).toBe("0.3.0");
  return { model, result, analysisRun, receivedHash };
}

describe("native review package received-result hash binding", () => {
  it("uses the preserved reference received hash and discloses unsupported future versions", async () => {
    const session = await referenceAnalysis();
    const input = { ...session, editorIntents: [], modelHash: null, projectSummary: null, proposal: null, selectedReviewTarget: null, storageCapability: null };
    const before = JSON.stringify(input);
    const packet = buildNativePackageReview(input);
    const member = packet.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json");
    expect(member?.hash_status).toBe(`sha256:${session.receivedHash}`);
    expect(JSON.stringify(input)).toBe(before);
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    const unsupported = buildNativePackageReview({ ...input, analysisRun: future });
    expect(unsupported.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json")?.hash_status).toBe("TBD_analysis_record_version_unsupported");
  });
});


it("retains explicit exact-profile quantities without inventing an authored shear modulus", async () => {
  const modelModule = await import("../../../../../fixtures/model_operations/exact_pressure_authoring_model.json");
  const resultModule = await import("../../../../../fixtures/results/pressure_connected_mechanics_sparse.json");
  const model = structuredClone(modelModule.default) as import("../../types").PreviewModel;
  const result = resultModule.default as unknown as import("../../types").MechanicsResult;
  model.pipe_segments[0].section.material_density = { value: 1000.123456789, unit: "kg/m³" };
  const before = JSON.stringify(model);
  const evidence = buildUnitPreservationEvidence({ model, result });
  const rows = evidence.model_quantity_witnesses;
  expect(rows.some(row => row.source_ref.field_path === "shear_modulus")).toBe(false);
  expect(rows.find(row => row.source_ref.field_path === "poisson_ratio")?.source_quantity)
    .toEqual({ value: 0.3, unit: "1", dimension: "ratio" });
  expect(rows.find(row => row.source_ref.field_path === "section.material_density")?.source_quantity)
    .toEqual({ value: 1000.123456789, unit: "kg/m³", dimension: "density" });
  expect(rows.find(row => row.source_ref.field_path === "pressure_regions.region:fixture-pressure.pressure")?.source_quantity)
    .toEqual({ value: 2000, unit: "kPa", dimension: "pressure" });
  expect(JSON.stringify(model)).toBe(before);
});

it("never offers a changed native package with the previous package's hash (issue 497)", async () => {
  const session = await referenceAnalysis();
  const props = {
    analysisRun: session.analysisRun, editorIntents: [], model: session.model, modelHash: null, projectSummary: null,
    proposal: null, result: session.result, selectedReviewTarget: null, storageCapability: null
  };
  const offered = (href: string) => {
    const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1]));
    return { projectName: packet.source_project.project_name as string, hash: (packet.validation_report.package_hash?.value ?? null) as string | null };
  };
  const hashFor = (label: string): PackageHashEvidence => ({
    algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "package_review_payload",
    payload_excludes: "validation_report_package_hash_fields", payload_ref: label, value: `sha256:${label}`,
    hash_status: "computed_local_preview"
  });
  mocked.packageHashes.length = 0;
  mocked.hrefs.length = 0;

  const view = render(createElement(NativePackagePanel, props));
  expect(mocked.packageHashes).toHaveLength(1);
  await act(async () => mocked.packageHashes[0].resolve(hashFor("first")));
  const name = session.model.project.name;
  expect(offered(mocked.hrefs.at(-1)!)).toEqual({ projectName: name, hash: "sha256:first" });

  const renamed = structuredClone(session.model);
  renamed.project.name = `${name} (renamed)`;
  mocked.hrefs.length = 0;
  view.rerender(createElement(NativePackagePanel, { ...props, model: renamed }));
  expect(mocked.hrefs.length).toBeGreaterThan(0);
  expect(mocked.hrefs.map(offered)).toEqual(mocked.hrefs.map(() => ({ projectName: renamed.project.name, hash: null })));

  const second = mocked.packageHashes.at(-1)!;
  expect((second.packet as { source_project: { project_name: string } }).source_project.project_name).toBe(renamed.project.name);
  await act(async () => second.resolve(hashFor("second")));
  expect(offered(mocked.hrefs.at(-1)!)).toEqual({ projectName: renamed.project.name, hash: "sha256:second" });
  expect(mocked.hrefs.map(offered).filter(item => item.hash === "sha256:first")).toEqual([]);
  view.unmount();
});
