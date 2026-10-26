// W-17 slice (c), DEL-02-02 Design `project-unit-selection.md` §7.4: the .mbf, PCF and
// local FEA exports refuse by name when a project unit they write is missing (absent,
// empty or whitespace-only). Nothing is assumed: no download, no substitute symbol.
// For a missing length key the project-unit refusal takes precedence over §7.5's
// `*-LENGTH-UNIT-UNSUPPORTED (missing)`. With every key present nothing changes.
// Invented data only.
import { cleanup, render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import demoModelFixture from "../../../../fixtures/product_preview/invented_demo_model.json";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import {
  buildCaePipeMbfExportPacket,
  CaepipeMbfExportPanel,
  MBF_LENGTH_UNIT_UNSUPPORTED,
  MBF_PROJECT_UNIT_MISSING
} from "./caepipe-mbf/CaepipeMbfExportPanel";
import { buildPcfExportPacket, PcfExportPanel, PCF_LENGTH_UNIT_UNSUPPORTED, PCF_PROJECT_UNIT_MISSING } from "./pcf-export/PcfExportPanel";
import { LocalFeaHandoffPanel, localFeaHandoffRefusal } from "./local-fea-handoff/LocalFeaHandoffPanel";

const HARD_LOCK = "Re-author the model with a length unit; no fix is offered.";

function model(units: Record<string, string | null> = {}, withNodes = true): PreviewModel {
  const copy = structuredClone(demoModelFixture) as PreviewModel;
  const merged = { ...copy.project.units } as Record<string, string>;
  for (const [key, value] of Object.entries(units)) {
    if (value === null) delete merged[key];
    else merged[key] = value;
  }
  copy.project.units = merged as PreviewModel["project"]["units"];
  if (!withNodes) {
    copy.nodes = [];
    copy.pipe_segments = [];
    copy.supports = [];
    copy.components = [];
  }
  return copy;
}

const MISSING: Array<[string, string | null]> = [["absent", null], ["empty", ""], ["whitespace-only", "  "]];

describe("a missing length key: the project-unit refusal takes precedence over LENGTH-UNIT-UNSUPPORTED", () => {
  it.each(MISSING)("%s length: .mbf and PCF refuse with *-PROJECT-UNIT-MISSING only, links withheld, the hard-lock text", (_label, value) => {
    const input = model({ length: value });
    for (const [packet, missingCode, lengthCode] of [
      [buildCaePipeMbfExportPacket({ model: input, result: null, analysisRun: null }), MBF_PROJECT_UNIT_MISSING, MBF_LENGTH_UNIT_UNSUPPORTED],
      [buildPcfExportPacket({ model: input, result: null, analysisRun: null }), PCF_PROJECT_UNIT_MISSING, PCF_LENGTH_UNIT_UNSUPPORTED]
    ] as const) {
      const codes = packet.diagnostics.map((item) => item.code);
      expect(codes).toContain(missingCode);
      expect(codes).not.toContain(lengthCode);
      const refusal = packet.diagnostics.find((item) => item.code === missingCode)!;
      expect(refusal.severity).toBe("blocking");
      expect(refusal.message).toContain(HARD_LOCK);
      // The remediation agrees with the hard lock: re-author, never "set length" (Codex review P3).
      const action = missingCode === MBF_PROJECT_UNIT_MISSING ? "exporting .mbf text" : "exporting PCF text";
      expect(refusal.remediation).toBe(`Re-author the model with a length unit before ${action}; the positions have no stated unit, and no fix is offered.`);
      expect(refusal.remediation).not.toContain("Project units on the Project page");
    }

    render(<CaepipeMbfExportPanel model={input} result={null} analysisRun={null} />);
    const mbf = screen.getByTestId("caepipe-mbf-panel");
    expect(within(mbf).getByTestId("caepipe-mbf-project-unit-refusal")).toHaveTextContent(`.mbf export refused. ${MBF_PROJECT_UNIT_MISSING}: `);
    expect(within(mbf).getByTestId("caepipe-mbf-project-unit-refusal")).toHaveTextContent(HARD_LOCK);
    expect(within(mbf).queryByTestId("caepipe-mbf-length-unit-refused")).toBeNull();
    expect(within(mbf).queryByTestId("caepipe-mbf-export-link")).toBeNull();
    expect(within(mbf).queryByTestId("caepipe-mbf-text-link")).toBeNull();
    expect(within(mbf).getByTestId("caepipe-mbf-summary")).toHaveTextContent(/^refused;/);

    render(<PcfExportPanel model={input} result={null} analysisRun={null} />);
    const pcf = screen.getByTestId("pcf-export-panel");
    expect(within(pcf).getByTestId("pcf-export-project-unit-refusal")).toHaveTextContent(`PCF export refused. ${PCF_PROJECT_UNIT_MISSING}: `);
    expect(within(pcf).queryByTestId("pcf-export-length-unit-refused")).toBeNull();
    expect(within(pcf).queryByTestId("pcf-export-link")).toBeNull();
    expect(within(pcf).queryByTestId("pcf-text-link")).toBeNull();
  });

  it("with no nodes, a missing length is offered to be set, not the hard lock", () => {
    const packet = buildPcfExportPacket({ model: model({ length: null }, false), result: null, analysisRun: null });
    const refusal = packet.diagnostics.find((item) => item.code === PCF_PROJECT_UNIT_MISSING)!;
    expect(refusal.message).toBe(
      "Missing project unit: length. This export writes the node coordinates' source unit from project.units.length, and nothing is assumed. Set length in Project units on the Project page."
    );
    expect(refusal.remediation).toBe("Set length in Project units on the Project page before exporting PCF text.");
    const mbf = buildCaePipeMbfExportPacket({ model: model({ length: null }, false), result: null, analysisRun: null });
    expect(mbf.diagnostics.find((item) => item.code === MBF_PROJECT_UNIT_MISSING)!.remediation).toBe("Set length in Project units on the Project page before exporting .mbf text.");
  });

  it("an unsupported length that is present keeps §7.5's refusal", () => {
    const packet = buildPcfExportPacket({ model: model({ length: "cm" }), result: null, analysisRun: null });
    const codes = packet.diagnostics.map((item) => item.code);
    expect(codes).toContain(PCF_LENGTH_UNIT_UNSUPPORTED);
    expect(codes).not.toContain(PCF_PROJECT_UNIT_MISSING);
  });
});

describe(".mbf UNIT record: force and temperature as stored, or refused", () => {
  it.each([["force", "N"], ["temperature", "C"]] as const)("missing %s: refused by name, and the former %s is not substituted", (key, former) => {
    for (const [, value] of MISSING) {
      const input = model({ [key]: value });
      const packet = buildCaePipeMbfExportPacket({ model: input, result: null, analysisRun: null });
      const refusal = packet.diagnostics.find((item) => item.code === MBF_PROJECT_UNIT_MISSING)!;
      expect(refusal.message).toBe(`Missing project unit: ${key}. This export writes the UNIT record's ${key} unit from project.units.${key}, and nothing is assumed. Set ${key} in Project units on the Project page.`);
      expect(packet.model_payload.units[key]).toBe("");
      expect(packet.model_payload.units[key]).not.toBe(former);
      render(<CaepipeMbfExportPanel model={input} result={null} analysisRun={null} />);
      expect(screen.getByTestId("caepipe-mbf-project-unit-refusal")).toHaveTextContent(`Missing project unit: ${key}.`);
      expect(screen.queryByTestId("caepipe-mbf-text-link")).toBeNull();
      cleanup();
    }
  });

  it("a legacy C is written as stored", () => {
    const packet = buildCaePipeMbfExportPacket({ model: model({ temperature: "C" }), result: null, analysisRun: null });
    expect(packet.mbf_text).toContain("UNIT,mm,N,C");
    expect(packet.diagnostics.map((item) => item.code)).not.toContain(MBF_PROJECT_UNIT_MISSING);
  });

  it("with every key present, the demo exports as before", () => {
    const packet = buildCaePipeMbfExportPacket({ model: model(), result: null, analysisRun: null });
    expect(packet.mbf_text).toContain("UNIT,mm,N,degC");
    expect(packet.diagnostics.map((item) => item.code)).not.toContain(MBF_PROJECT_UNIT_MISSING);
    render(<CaepipeMbfExportPanel model={model()} result={null} analysisRun={null} />);
    expect(screen.queryByTestId("caepipe-mbf-project-unit-refusal")).toBeNull();
    expect(screen.getByTestId("caepipe-mbf-text-link")).toBeInTheDocument();
  });
});

describe("local FEA handoff: a missing project unit it writes is refused by name", () => {
  const result = {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:units-test",
    model_ref: "project:invented-demo-loop-01",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "not_performed", professional_acceptance: "not_provided" },
    summary: {},
    results: [],
    diagnostics: []
  } as unknown as MechanicsResult;
  const analysisRun = {
    schema_version: "0.1.0",
    analysis_run: {
      run_id: "run:units-test", run_name: "Invented", run_kind: "preview_linear_static",
      model_state_ref: { object_type: "ModelState", ref: "state:units-test" }, load_basis_refs: [], result_refs: [], hashes: [],
      analysis_status: ["MECHANICS_SOLVED"], reproducibility: { input_manifest_refs: [], determinism_notes: [], unresolved_tbd: [] },
      immutability_policy: { run_record_is_read_only: true, mutation_policy: "new_run_required", new_run_required_for_change: true, hash_invalidates_external_acceptance: true },
      professional_boundary: { human_review_required: true }
    }
  } as unknown as AnalysisRunEnvelope;

  it.each(["length", "force", "stress", "temperature"])("missing %s (no result row of the family referenced): refused, no link", (key) => {
    for (const [, value] of MISSING) {
      const input = model({ [key]: value });
      const refusal = localFeaHandoffRefusal(input, result)!;
      expect(refusal.testId).toBe("local-fea-project-unit-refusal");
      expect(refusal.text).toContain(`LOCAL-FEA-PROJECT-UNIT-MISSING: `);
      expect(refusal.text).toContain(key === "length" ? HARD_LOCK : `Missing project unit: ${key}.`);
      render(<LocalFeaHandoffPanel model={input} result={result} analysisRun={analysisRun} />);
      expect(screen.getByTestId("local-fea-project-unit-refusal")).toHaveTextContent("Local FEA handoff refused.");
      expect(screen.queryByTestId("local-fea-export-link")).toBeNull();
      cleanup();
    }
  });

  it("with every key present, the handoff is offered", () => {
    expect(localFeaHandoffRefusal(model(), result)).toBeNull();
    render(<LocalFeaHandoffPanel model={model()} result={result} analysisRun={analysisRun} />);
    expect(screen.getByTestId("local-fea-export-link")).toBeInTheDocument();
  });
});
