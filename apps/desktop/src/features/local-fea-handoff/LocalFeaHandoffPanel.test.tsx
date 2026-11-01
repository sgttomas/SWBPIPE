import { loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
// Rider coverage for TP-SEAM-CORPUS-001 (operation-seam unification plan §3
// T1 rider; assessment §5.3): when the mechanics result summary does not
// carry location references, the Local FEA handoff package must emit an
// explicit incomplete-state finding and degrade honestly — it must never
// invent fixture entity references (the former `pipe:P-120` / `node:N-140`
// fallbacks). All fixture data below is invented.

import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { LocalFeaHandoffPanel, buildLocalFeaHandoffPacket } from "./LocalFeaHandoffPanel";

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.9.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: {
      id: "project:rider-test",
      name: "Invented rider model",
      description: "Invented panel fixture",
      // W-17 slice (c) §7.4: the handoff writes stress_unit from the key when no stress row is referenced.
      units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" }
    },
    analysis_status: { mechanics: "ready", rule_check: "not_performed", professional_acceptance: "not_provided" },
    materials: [],
    nodes: [
      { id: "node:N-1", label: "Anchor", position: { x: 0, y: 0, z: 0 }, provenance: "invented_example" },
      { id: "node:N-2", label: "Elbow", position: { x: 3.2, y: 0, z: 0 }, provenance: "invented_example" }
    ],
    pipe_segments: [
      {
        id: "pipe:P-1",
        label: "Run",
        from: "node:N-1",
        to: "node:N-2",
        section: {
          outside_diameter: { value: 0.168, unit: "m" },
          wall_thickness: { value: 0.007, unit: "m" }
        },
        material: "material:M-1",
        provenance: "invented_example"
      }
    ],
    supports: [
      { id: "support:S-1", label: "Anchor support", node: "node:N-1", restraints: ["UX", "UY", "UZ"], provenance: "invented_example" }
    ],
    components: [],
    load_cases: [],
    diagnostics: []
  } as unknown as PreviewModel;
}

function inventedResult(
  summary: MechanicsResult["summary"],
  results: MechanicsResult["results"] = []
): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:rider-test-001",
    model_ref: "project:rider-test",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "not_performed", professional_acceptance: "not_provided" },
    summary,
    results,
    diagnostics: []
  } as unknown as MechanicsResult;
}

function inventedAnalysisRun(): AnalysisRunEnvelope {
  return {
    schema_version: "0.1.0",
    analysis_run: {
      run_id: "run:rider-test-001",
      run_name: "Invented rider run",
      run_kind: "preview_linear_static",
      model_state_ref: { object_type: "ModelState", ref: "state:rider-test" },
      load_basis_refs: [],
      result_refs: [],
      hashes: [],
      analysis_status: ["MECHANICS_SOLVED"],
      reproducibility: { input_manifest_refs: [], determinism_notes: [], unresolved_tbd: [] },
      immutability_policy: {
        run_record_is_read_only: true,
        mutation_policy: "new_run_required",
        new_run_required_for_change: true,
        hash_invalidates_external_acceptance: true
      },
      professional_boundary: { human_review_required: true }
    }
  } as unknown as AnalysisRunEnvelope;
}

function exportedPacket(): string {
  const href = screen.getByTestId("local-fea-export-link").getAttribute("href") ?? "";
  return decodeURIComponent(href.replace("data:application/json;charset=utf-8,", ""));
}

describe("LocalFeaHandoffPanel result-summary boundary hygiene", () => {
  it("emits an explicit incomplete-state finding instead of invented entity references when summary location refs are missing", () => {
    render(
      <LocalFeaHandoffPanel
        model={inventedModel()}
        result={inventedResult({})}
        analysisRun={inventedAnalysisRun()}
      />
    );

    const panel = screen.getByTestId("local-fea-panel");
    const region = within(panel).getByTestId("local-fea-region").textContent ?? "";
    expect(region).toContain("local-region:stress-location-ref-missing:displacement-location-ref-missing");
    expect(region).toContain("nodes=0");
    expect(region).toContain("elements=0");
    expect(within(panel).getByTestId("local-fea-summary").textContent).toContain("diagnostics=5");

    const packet = exportedPacket();
    expect(packet).toContain("[REDACTED]");
    expect(packet).not.toContain("LOCAL-FEA-RESULT-SUMMARY-REF-MISSING");
    expect(packet).not.toContain("summary.max_open_formula_stress.location_ref");
    expect(packet).not.toContain("summary.max_displacement.location_ref");
    expect(packet).not.toContain("no substitute entity references are invented");
    // The former fixture fallbacks must not appear anywhere in the package.
    expect(packet).not.toContain("pipe:P-120");
    expect(packet).not.toContain("node:N-140");
  });

  it("keeps the standing four findings and real references when the summary carries location refs", () => {
    render(
      <LocalFeaHandoffPanel
        model={inventedModel()}
        result={inventedResult({
          max_displacement: { value: 0.004, unit: "m", location_ref: "node:N-2", result_ref: "result:disp:node-N-2" },
          max_open_formula_stress: { value: 12500000, unit: "Pa", location_ref: "pipe:P-1", result_ref: "result:stress:pipe-P-1" }
        })}
        analysisRun={inventedAnalysisRun()}
      />
    );

    const panel = screen.getByTestId("local-fea-panel");
    const region = within(panel).getByTestId("local-fea-region").textContent ?? "";
    expect(region).toContain("local-region:pipe:P-1:node:N-2");
    expect(within(panel).getByTestId("local-fea-summary").textContent).toContain("diagnostics=4");

    const packet = exportedPacket();
    expect(packet).not.toContain("LOCAL-FEA-RESULT-SUMMARY-REF-MISSING");
    expect(packet).not.toContain("pipe:P-120");
    expect(packet).not.toContain("node:N-140");
  });

  it("witnesses source value and unit preservation for referenced transfer results", () => {
    render(
      <LocalFeaHandoffPanel
        model={inventedModel()}
        result={inventedResult(
          {
            max_displacement: { value: 4, unit: "mm", location_ref: "node:N-2", result_ref: "result:disp:node-N-2" },
            max_open_formula_stress: { value: 12.5, unit: "MPa", location_ref: "pipe:P-1", result_ref: "result:stress:pipe-P-1" }
          },
          [
            {
              id: "result:disp:node-N-2",
              kind: "displacement_magnitude",
              value: 4,
              unit: "mm",
              entity_ref: "node:N-2"
            },
            {
              id: "result:force:pipe-P-120:axial",
              kind: "element_local_axial_force",
              value: 25,
              unit: "N",
              entity_ref: "pipe:P-1"
            },
            {
              id: "result:moment:pipe-P-120:torsion",
              kind: "element_local_torsional_moment",
              value: 11,
              unit: "N*m",
              entity_ref: "pipe:P-1"
            },
            {
              id: "result:stress:pipe-P-1",
              kind: "open_formula_stress",
              value: 12.5,
              unit: "MPa",
              entity_ref: "pipe:P-1"
            }
          ]
        )}
        analysisRun={inventedAnalysisRun()}
      />
    );

    const panel = screen.getByTestId("local-fea-panel");
    expect(within(panel).getByTestId("local-fea-unit-witnesses").textContent).toContain("count=3");
    expect(within(panel).getByTestId("local-fea-unit-witnesses").textContent).toContain("conversion=false");

    const packet = JSON.parse(exportedPacket());
    expect(packet.handoff_package.unit_witness_policy).toBe("[REDACTED]");
    expect(packet.handoff_package.unit_preservation_witnesses).toHaveLength(3);
    expect(
      packet.handoff_package.unit_preservation_witnesses.map(
        (item: { target_field_path: string }) => item.target_field_path
      )
    ).toEqual([
      "[REDACTED]",
      "[REDACTED]",
      "[REDACTED]"
    ]);

    for (const witness of packet.handoff_package.unit_preservation_witnesses) {
      expect(witness.source_result_ref.locator).toBe("[REDACTED]");
      expect(witness.source_quantity).toEqual({
        value: "[REDACTED]",
        unit: "[REDACTED]",
        dimension: "[REDACTED]"
      });
      expect(witness.target_quantity_policy).toBe("[REDACTED]");
      expect(witness.conversion_performed).toBe("[REDACTED]");
    }
  });
});

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

describe("Local FEA received-result hash binding", () => {
  it("retains the reference received hash and discloses unsupported future versions", async () => {
    const session = await referenceAnalysis();
    const before = JSON.stringify(session);
    const packet = buildLocalFeaHandoffPacket(session);
    expect(packet.handoff_package.source_refs.result_hash.value).toBe(session.receivedHash);
    expect(JSON.stringify(session)).toBe(before);
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    const unsupported = buildLocalFeaHandoffPacket({ ...session, analysisRun: future });
    expect(unsupported.handoff_package.source_refs.result_hash.value).toBe("TBD_analysis_record_version_unsupported");
  });
});

// W-17 slice (c), DEL-02-02 Design §7.7 (Agent 0's B-c): the manifest's stress_unit and
// force_unit are declared from the result rows they label, the governing row first (the
// summary maximum, for stress); only rows in the declared unit are referenced, and a
// candidate in another unit is named in a non-blocking diagnostic. Where no governing
// row decides and the rows disagree, the handoff is refused by name.
describe("Local FEA manifest result units agree with every referenced row", () => {
  const OMITTED = "LOCAL-FEA-RESULT-ROW-UNIT-OMITTED";
  const TORSION = "result:stress:pipe-P-120:end-j:torsional-shear";
  const GOVERNING = { max_open_formula_stress: { value: 12.5e6, unit: "Pa", location_ref: "pipe:P-1", result_ref: "result:stress:pipe-P-1" } };
  const rows = (governingUnit: string, torsionUnit: string): MechanicsResult["results"] => [
    { id: "result:stress:pipe-P-1", kind: "open_formula_stress", value: governingUnit === "Pa" ? 12.5e6 : 12.5, unit: governingUnit, entity_ref: "pipe:P-1" },
    { id: TORSION, kind: "element_local_torsional_shear_stress", value: torsionUnit === "Pa" ? 4.5e5 : 0.45, unit: torsionUnit, entity_ref: "pipe:P-1" },
    { id: "result:force:pipe-P-120:axial", kind: "element_local_axial_force", value: 25, unit: "N", entity_ref: "pipe:P-1" }
  ] as MechanicsResult["results"];
  function withUnits(units: Record<string, string>): PreviewModel {
    const model = inventedModel();
    model.project.units = units as PreviewModel["project"]["units"];
    return model;
  }
  const SI = { length: "m", force: "N", moment: "N*m", pressure: "Pa", stress: "Pa", temperature: "degC", angle: "rad" };
  const US = { length: "in", force: "lbf", moment: "lbf*in", pressure: "psi", stress: "psi", temperature: "degF", angle: "deg" };

  function expectLabelsAgree(packet: ReturnType<typeof buildLocalFeaHandoffPacket>, result: MechanicsResult) {
    const manifest = packet.handoff_package.units_manifest;
    const referenced = result.results.filter((row) => packet.handoff_package.entity_ids.result_ids.includes(row.id));
    for (const row of referenced) {
      if (row.id.includes(":stress:")) expect(row.unit, row.id).toBe(manifest.stress_unit);
      if (row.id.includes(":force:")) expect(row.unit, row.id).toBe(manifest.force_unit);
    }
    return referenced;
  }

  it.each([["SI", SI], ["US", US]] as const)("%s keys: declared from the rows, and every referenced stress and force row carries the declared unit", (_label, units) => {
    const result = inventedResult(GOVERNING, rows("MPa", "MPa"));
    const packet = buildLocalFeaHandoffPacket({ model: withUnits(units), result, analysisRun: inventedAnalysisRun() });
    expect(packet.handoff_package.units_manifest.stress_unit).toBe("MPa");
    expect(packet.handoff_package.units_manifest.force_unit).toBe("N");
    expect(expectLabelsAgree(packet, result).map((row) => row.id)).toContain(TORSION);
    expect(packet.handoff_package.units_manifest.coordinate_unit).toBe(units.length);
  });

  it("a stress row in another unit than the governing maximum is not referenced, and a non-blocking diagnostic names it", () => {
    const result = inventedResult(GOVERNING, rows("Pa", "MPa"));
    const packet = buildLocalFeaHandoffPacket({ model: withUnits(SI), result, analysisRun: inventedAnalysisRun() });
    expect(packet.handoff_package.units_manifest.stress_unit).toBe("Pa");
    expectLabelsAgree(packet, result);
    expect(packet.handoff_package.entity_ids.result_ids).not.toContain(TORSION);
    const omitted = packet.handoff_package.diagnostics.filter((item: { code: string }) => item.code === OMITTED);
    expect(omitted).toHaveLength(1);
    expect(omitted[0].severity).toBe("warning");
    expect(omitted[0].message).toBe(
      `Result row ${TORSION} (MPa) is not referenced: this handoff declares stress_unit Pa, from the governing stress result, and references only stress rows in that unit; nothing is converted.`
    );
    render(<LocalFeaHandoffPanel model={withUnits(SI)} result={result} analysisRun={inventedAnalysisRun()} />);
    expect(screen.getByTestId("local-fea-export-link")).toBeInTheDocument();
  });

  it("with no governing row, stress rows in two units refuse the handoff by name, with no download", () => {
    // The displacement maximum names a stress row here, so two stress rows are candidates and none governs.
    const result = inventedResult(
      { max_displacement: { value: 1, unit: "mm", location_ref: "node:N-2", result_ref: "result:stress:pipe-P-1" } },
      rows("Pa", "MPa")
    );
    render(<LocalFeaHandoffPanel model={withUnits(SI)} result={result} analysisRun={inventedAnalysisRun()} />);
    expect(screen.getByTestId("local-fea-result-unit-refusal")).toHaveTextContent(
      "Local FEA handoff refused. LOCAL-FEA-RESULT-UNIT-INCONSISTENT: the referenced stress results carry different units (MPa, Pa) and none governs; no single stress_unit can be declared, and nothing is converted."
    );
    expect(screen.queryByTestId("local-fea-export-link")).toBeNull();
    expect(() => buildLocalFeaHandoffPacket({ model: withUnits(SI), result, analysisRun: inventedAnalysisRun() })).toThrow("LOCAL-FEA-RESULT-UNIT-INCONSISTENT");
  });

  it("the bundled demo declares Pa from its governing maximum, references no MPa stress row, and names the omitted one", async () => {
    const session = await referenceAnalysis();
    const packet = buildLocalFeaHandoffPacket(session);
    const governing = session.result.summary.max_open_formula_stress!;
    expect(governing.unit).toBe("Pa");
    expect(packet.handoff_package.units_manifest.stress_unit).toBe("Pa");
    expect(packet.handoff_package.units_manifest.force_unit).toBe("N");
    expect(expectLabelsAgree(packet, session.result).map((row) => row.id)).toContain(governing.result_ref);
    expect(packet.handoff_package.entity_ids.result_ids).not.toContain(TORSION);
    const omitted = packet.handoff_package.diagnostics.filter((item: { code: string }) => item.code === OMITTED);
    expect(omitted.map((item: { message: string }) => item.message)).toEqual([
      `Result row ${TORSION} (MPa) is not referenced: this handoff declares stress_unit Pa, from the governing stress result, and references only stress rows in that unit; nothing is converted.`
    ]);
    expect(packet.handoff_package.unit_preservation_witnesses).toHaveLength(3);
  });
});
