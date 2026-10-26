// Export length conversion (DEL-02-02 Design `project-unit-selection.md` §7, C2): the
// .mbf and PCF exports convert each length by its own stored unit with the exact
// factors (1 in = 25.4 mm, 1 ft = 304.8 mm), and refuse by name a unit with no routine
// exact conversion. Expected millimetres are hand-computed from the invented inputs.
import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import demoModelFixture from "../../../../fixtures/product_preview/invented_demo_model.json";
import type { PreviewModel } from "../types";
import { lengthToMillimeters } from "./exportLengthConversion";
import { buildCaePipeMbfExportPacket, CaepipeMbfExportPanel, MBF_LENGTH_UNIT_UNSUPPORTED } from "./caepipe-mbf/CaepipeMbfExportPanel";
import { buildPcfExportPacket, PcfExportPanel, PCF_LENGTH_UNIT_UNSUPPORTED } from "./pcf-export/PcfExportPanel";

type Position = { x: number; y: number; z: number };
type Quantity = { value: number; unit: string };

// Invented positions per demo node (sorted ids N-100..N-140 become MBF N001..N005).
const INCH_POSITIONS: Record<string, Position> = {
  "node:N-100": { x: 0, y: 0, z: 0 },
  "node:N-110": { x: 120, y: 0, z: 0 },
  "node:N-120": { x: 120, y: 7.5, z: 0 },
  "node:N-130": { x: 2.375, y: 7.5, z: -3.25 },
  "node:N-140": { x: 2.375, y: 7.5, z: 0.1 }
};
// Hand-computed: 120 in = 3048 mm; 7.5 in = 190.5 mm; 2.375 in = 60.325 mm;
// -3.25 in = -82.55 mm; 0.1 in = 2.54 mm.
const INCH_MM: Record<string, Position> = {
  "node:N-100": { x: 0, y: 0, z: 0 },
  "node:N-110": { x: 3048, y: 0, z: 0 },
  "node:N-120": { x: 3048, y: 190.5, z: 0 },
  "node:N-130": { x: 60.325, y: 190.5, z: -82.55 },
  "node:N-140": { x: 60.325, y: 190.5, z: 2.54 }
};
const FOOT_POSITIONS: Record<string, Position> = {
  "node:N-100": { x: 0, y: 0, z: 0 },
  "node:N-110": { x: 10, y: 0, z: 0 },
  "node:N-120": { x: 10, y: 2.5, z: 0 },
  "node:N-130": { x: 0.1, y: 2.5, z: -1.75 },
  "node:N-140": { x: 0.1, y: 2.5, z: 0.3 }
};
// Hand-computed: 10 ft = 3048 mm; 2.5 ft = 762 mm; 0.1 ft = 30.48 mm;
// -1.75 ft = -533.4 mm; 0.3 ft = 91.44 mm.
const FOOT_MM: Record<string, Position> = {
  "node:N-100": { x: 0, y: 0, z: 0 },
  "node:N-110": { x: 3048, y: 0, z: 0 },
  "node:N-120": { x: 3048, y: 762, z: 0 },
  "node:N-130": { x: 30.48, y: 762, z: -533.4 },
  "node:N-140": { x: 30.48, y: 762, z: 91.44 }
};

function model({
  length,
  positions,
  section
}: {
  length?: string | null;
  positions?: Record<string, Position>;
  section?: (segmentId: string) => { outside_diameter?: Quantity; wall_thickness?: Quantity } | null;
}): PreviewModel {
  const copy = structuredClone(demoModelFixture) as PreviewModel;
  if (length === null) delete (copy.project.units as Record<string, string | undefined>).length;
  else if (length !== undefined) copy.project.units.length = length;
  if (positions) for (const node of copy.nodes) node.position = { ...positions[node.id] };
  if (section) {
    for (const segment of copy.pipe_segments) {
      const replacement = section(segment.id);
      if (replacement) segment.section = { ...segment.section, ...replacement };
    }
  }
  return copy;
}

function pcf(input: PreviewModel) {
  return buildPcfExportPacket({ model: input, result: null, analysisRun: null });
}

function mbf(input: PreviewModel) {
  return buildCaePipeMbfExportPacket({ model: input, result: null, analysisRun: null });
}

function witness(packet: { conversion_witnesses: Array<{ witness_id: string }> }, id: string) {
  const found = packet.conversion_witnesses.find((item) => item.witness_id === id);
  if (!found) throw new Error(`missing witness ${id}`);
  return found as (typeof packet.conversion_witnesses)[number] & {
    target_quantity: { value: number | null };
    conversion_factor_to_target: number | null;
    conversion_status: string;
  };
}

function downloadedPackage(panel: HTMLElement, testId: string) {
  const href = within(panel).getByTestId(testId).getAttribute("href");
  expect(href).toBeTruthy();
  return JSON.parse(decodeURIComponent(String(href).split(",", 2)[1]));
}

const SIX_INCH_PIPE = () => ({
  outside_diameter: { value: 6, unit: "in" },
  wall_thickness: { value: 0.25, unit: "in" }
});

describe("export length conversion helper", () => {
  it("uses the exact defined factors and refuses other units by name", () => {
    expect(lengthToMillimeters(1, "m")).toEqual({ status: "converted", value: 1000, factor: 1000, sourceMatchesTarget: false });
    expect(lengthToMillimeters(1, "mm")).toEqual({ status: "converted", value: 1, factor: 1, sourceMatchesTarget: true });
    expect(lengthToMillimeters(1, "in")).toEqual({ status: "converted", value: 25.4, factor: 25.4, sourceMatchesTarget: false });
    expect(lengthToMillimeters(1, "ft")).toEqual({ status: "converted", value: 304.8, factor: 304.8, sourceMatchesTarget: false });
    // 6 x 25.4 is 152.39999999999998 in binary64; the six-decimal normalisation gives 152.4.
    expect(lengthToMillimeters(6, "in")).toMatchObject({ value: 152.4 });
    // 0.3 x 304.8 = 91.44 (0.3 x 0.3048 x 1000 would give 91.44000000000001).
    expect(lengthToMillimeters(0.3, "ft")).toMatchObject({ value: 91.44 });
    expect(lengthToMillimeters(1, "cm")).toEqual({ status: "refused", unitLabel: '"cm"' });
    expect(lengthToMillimeters(1, "MM")).toEqual({ status: "refused", unitLabel: '"MM"' });
    expect(lengthToMillimeters(1, "TBD")).toEqual({ status: "refused", unitLabel: '"TBD"' });
    expect(lengthToMillimeters(1, "")).toEqual({ status: "refused", unitLabel: "(missing)" });
    expect(lengthToMillimeters(1, undefined)).toEqual({ status: "refused", unitLabel: "(missing)" });
    expect(lengthToMillimeters(1, "toString")).toEqual({ status: "refused", unitLabel: '"toString"' });
  });
});

describe.each([
  { unit: "in", positions: INCH_POSITIONS, expected: INCH_MM, factor: 25.4 },
  { unit: "ft", positions: FOOT_POSITIONS, expected: FOOT_MM, factor: 304.8 }
])("node coordinates in $unit", ({ unit, positions, expected, factor }) => {
  const input = () => model({ length: unit, positions });

  it(".mbf converts to hand-computed millimetres with true witnesses", () => {
    const packet = mbf(input());
    for (const node of packet.model_payload.nodes) {
      expect({ x: node.x, y: node.y, z: node.z }).toEqual(expected[node.node_id]);
    }
    const n4 = expected["node:N-130"];
    expect(packet.mbf_text).toContain(`NODE,N002,3048,0,0\n`);
    expect(packet.mbf_text).toContain(`NODE,N004,${n4.x},${n4.y},${n4.z}\n`);
    const x = witness(packet, "caepipe-mbf-conversion:node-n-130:position:x");
    expect(x.target_quantity.value).toBe(n4.x);
    expect(x.conversion_factor_to_target).toBe(factor);
    expect(x.conversion_status).toBe("converted_to_target_unit");
    expect(packet.diagnostics.map((item) => item.code)).not.toContain(MBF_LENGTH_UNIT_UNSUPPORTED);
    expect(packet.validation_report.validation_status).toBe("boundary_checked");
  });

  it("PCF converts to hand-computed millimetres with true witnesses", () => {
    const packet = pcf(input());
    for (const node of packet.pcf_payload.nodes) {
      expect({ x: node.x, y: node.y, z: node.z }).toEqual(expected[node.node_id]);
    }
    const n3 = expected["node:N-120"];
    const n4 = expected["node:N-130"];
    expect(packet.pcf_text).toContain(`    END-POINT ${n3.x} ${n3.y} ${n3.z} TBD_SOURCE_REQUIRED\n    END-POINT ${n4.x} ${n4.y} ${n4.z} TBD_SOURCE_REQUIRED\n`);
    const z = witness(packet, "pcf-conversion:node-n-130:position:z");
    expect(z.target_quantity.value).toBe(n4.z);
    expect(z.conversion_factor_to_target).toBe(factor);
    expect(z.conversion_status).toBe("converted_to_target_unit");
    expect(packet.diagnostics.map((item) => item.code)).not.toContain(PCF_LENGTH_UNIT_UNSUPPORTED);
  });

  it("the downloadable packages carry the true factor through the real export controls", () => {
    render(
      <>
        <PcfExportPanel model={input()} result={null} analysisRun={null} />
        <CaepipeMbfExportPanel model={input()} result={null} analysisRun={null} />
      </>
    );
    const pcfPackage = downloadedPackage(screen.getByTestId("pcf-export-panel"), "pcf-export-link");
    const mbfPackage = downloadedPackage(screen.getByTestId("caepipe-mbf-panel"), "caepipe-mbf-export-link");
    expect(witness(pcfPackage, "pcf-conversion:node-n-110:position:x").conversion_factor_to_target).toBe(factor);
    expect(witness(mbfPackage, "caepipe-mbf-conversion:node-n-110:position:x").conversion_factor_to_target).toBe(factor);
  });
});

describe("PCF section quantities convert by their own units", () => {
  it.each([
    // Hand-computed: 6 in = 152.4 mm; 0.25 in = 6.35 mm.
    { od: { value: 6, unit: "in" }, wall: { value: 0.25, unit: "in" }, odMm: "152.4", wallMm: "6.35", factor: 25.4 },
    // Hand-computed: 0.5 ft = 152.4 mm; 0.03125 ft = 9.525 mm.
    { od: { value: 0.5, unit: "ft" }, wall: { value: 0.03125, unit: "ft" }, odMm: "152.4", wallMm: "9.525", factor: 304.8 }
  ])("OD and wall in $od.unit with project length m", ({ od, wall, odMm, wallMm, factor }) => {
    const input = model({ length: "m", section: () => ({ outside_diameter: od, wall_thickness: wall }) });
    const packet = pcf(input);
    for (const segment of packet.pcf_payload.pipe_segments) {
      expect(segment.outside_diameter).toBe(odMm);
      expect(segment.wall_thickness).toBe(wallMm);
    }
    expect(packet.pcf_text).toContain(`    OUTSIDE-DIAMETER ${odMm}\n    WALL-THICKNESS ${wallMm}\n`);
    const odWitness = witness(packet, "pcf-conversion:pipe-p-120:outside_diameter");
    expect(odWitness.target_quantity.value).toBe(Number(odMm));
    expect(odWitness.conversion_factor_to_target).toBe(factor);
    expect(odWitness.conversion_status).toBe("converted_to_target_unit");
    const wallWitness = witness(packet, "pcf-conversion:pipe-p-120:wall_thickness");
    expect(wallWitness.target_quantity.value).toBe(Number(wallMm));
    expect(wallWitness.conversion_factor_to_target).toBe(factor);
    expect(wallWitness.conversion_status).toBe("converted_to_target_unit");
    // Coordinates stay metres converted by 1000 (demo N-110 x = 3.2 m = 3200 mm).
    const x = witness(packet, "pcf-conversion:node-n-110:position:x");
    expect(x.conversion_factor_to_target).toBe(1000);
    expect(x.target_quantity.value).toBe(3200);
    expect(packet.diagnostics.map((item) => item.code)).not.toContain(PCF_LENGTH_UNIT_UNSUPPORTED);

    render(<PcfExportPanel model={input} result={null} analysisRun={null} />);
    const downloaded = downloadedPackage(screen.getByTestId("pcf-export-panel"), "pcf-export-link");
    expect(witness(downloaded, "pcf-conversion:pipe-p-120:outside_diameter").conversion_factor_to_target).toBe(factor);
    expect(witness(downloaded, "pcf-conversion:pipe-p-120:wall_thickness").conversion_factor_to_target).toBe(factor);
  });

  it("metre and millimetre witnesses are unchanged: m converts by 1000, mm matches the target", () => {
    const metres = pcf(model({ length: "m" }));
    const od = witness(metres, "pcf-conversion:pipe-p-120:outside_diameter");
    expect(od.conversion_factor_to_target).toBe(1000);
    expect(od.conversion_status).toBe("converted_to_target_unit");
    expect(metres.pcf_payload.pipe_segments[0].outside_diameter).toBe("168");
    const millimetres = pcf(
      model({ length: "mm", section: () => ({ outside_diameter: { value: 168, unit: "mm" }, wall_thickness: { value: 7, unit: "mm" } }) })
    );
    const mmOd = witness(millimetres, "pcf-conversion:pipe-p-120:outside_diameter");
    expect(mmOd.conversion_factor_to_target).toBe(1);
    expect(mmOd.conversion_status).toBe("source_unit_matches_target_unit");
    expect(millimetres.pcf_payload.pipe_segments[0].outside_diameter).toBe("168");
  });
});

describe("a length with no routine exact conversion is refused by name", () => {
  function expectPcfRefused(input: PreviewModel, named: string[]) {
    const packet = pcf(input);
    const refusals = packet.diagnostics.filter((item) => item.code === PCF_LENGTH_UNIT_UNSUPPORTED);
    expect(refusals.length).toBe(named.length);
    named.forEach((text, index) => expect(refusals[index].message).toContain(text));
    expect(refusals.every((item) => item.severity === "blocking")).toBe(true);
    render(<PcfExportPanel model={input} result={null} analysisRun={null} />);
    const panel = screen.getByTestId("pcf-export-panel");
    expect(within(panel).queryByTestId("pcf-export-link")).toBeNull();
    expect(within(panel).queryByTestId("pcf-text-link")).toBeNull();
    const status = within(panel).getByTestId("pcf-export-length-unit-refused");
    expect(status).toHaveTextContent(PCF_LENGTH_UNIT_UNSUPPORTED);
    named.forEach((text) => expect(status).toHaveTextContent(text));
    expect(within(panel).getByTestId("pcf-export-summary")).toHaveTextContent(/^refused;/);
    return packet;
  }

  it("PCF: unsupported OD only, with metre coordinates", () => {
    const packet = expectPcfRefused(
      model({ length: "m", section: (id) => (id === "pipe:P-100" ? { outside_diameter: { value: 16.8, unit: "cm" } } : null) }),
      ['pipe:P-100 outside_diameter is stored in "cm"']
    );
    const od = witness(packet, "pcf-conversion:pipe-p-100:outside_diameter");
    expect(od.target_quantity.value).toBeNull();
    expect(od.conversion_factor_to_target).toBeNull();
    expect(od.conversion_status).toBe("refused_no_routine_exact_conversion");
    expect(packet.pcf_payload.pipe_segments[0].outside_diameter).toBe("TBD_OD_UNIT_UNSUPPORTED");
  });

  it("PCF: unsupported wall only, with inch coordinates", () => {
    expectPcfRefused(
      model({
        length: "in",
        positions: INCH_POSITIONS,
        section: (id) => (id === "pipe:P-120" ? { ...SIX_INCH_PIPE(), wall_thickness: { value: 0.25, unit: "TBD" } } : SIX_INCH_PIPE())
      }),
      ['pipe:P-120 wall_thickness is stored in "TBD"']
    );
  });

  it.each([
    { length: "cm", named: '"cm"' },
    { length: "", named: "(missing)" },
    { length: null, named: "(missing)" }
  ])("PCF and .mbf: coordinates in $named", ({ length, named }) => {
    const input = model({ length });
    const packet = expectPcfRefused(input, [`project length unit ${named}`]);
    expect(packet.pcf_payload.nodes[1].x).toBeNull();
    expect(witness(packet, "pcf-conversion:node-n-110:position:x").conversion_factor_to_target).toBeNull();

    const mbfPacket = mbf(input);
    const refusals = mbfPacket.diagnostics.filter((item) => item.code === MBF_LENGTH_UNIT_UNSUPPORTED);
    expect(refusals).toHaveLength(1);
    expect(refusals[0].message).toContain(`project length unit ${named}`);
    expect(mbfPacket.model_payload.nodes[1].x).toBeNull();
    const x = witness(mbfPacket, "caepipe-mbf-conversion:node-n-110:position:x");
    expect(x.conversion_factor_to_target).toBeNull();
    expect(x.conversion_status).toBe("refused_no_routine_exact_conversion");
    render(<CaepipeMbfExportPanel model={input} result={null} analysisRun={null} />);
    const panel = screen.getByTestId("caepipe-mbf-panel");
    expect(within(panel).queryByTestId("caepipe-mbf-export-link")).toBeNull();
    expect(within(panel).queryByTestId("caepipe-mbf-text-link")).toBeNull();
    expect(within(panel).getByTestId("caepipe-mbf-length-unit-refused")).toHaveTextContent(named);
  });
});

describe("a length that is not finite, or not finite once converted, is refused as out of range", () => {
  // 1e308 in x 25.4 overflows binary64 to Infinity; NaN and +/-Infinity are not finite sources.
  const SOURCES = [
    { label: "1e308 in (overflow after the factor)", value: 1e308, named: "1e+308" },
    { label: "NaN", value: Number.NaN, named: "NaN" },
    { label: "+Infinity", value: Number.POSITIVE_INFINITY, named: "Infinity" },
    { label: "-Infinity", value: Number.NEGATIVE_INFINITY, named: "-Infinity" }
  ];

  it("the helper refuses each", () => {
    for (const { value } of SOURCES) {
      expect(lengthToMillimeters(value, "in")).toEqual({ status: "out_of_range", unitLabel: '"in"' });
    }
    // A large value whose product stays finite still converts: 1e305 m x 1000 is finite.
    const large = lengthToMillimeters(1e305, "m");
    expect(large.status).toBe("converted");
    expect(large.status === "converted" && Number.isFinite(large.value)).toBe(true);
  });

  it.each(SOURCES)("node coordinate $label: both exports refuse by name with links withheld", ({ value, named }) => {
    const positions = structuredClone(INCH_POSITIONS);
    positions["node:N-110"].x = value;
    const input = model({ length: "in", positions });

    const pcfPacket = pcf(input);
    const pcfRefusals = pcfPacket.diagnostics.filter((item) => item.code === "PCF-LENGTH-OUT-OF-RANGE");
    expect(pcfRefusals).toHaveLength(1);
    expect(pcfRefusals[0].severity).toBe("blocking");
    expect(pcfRefusals[0].affected_object).toEqual({ object_type: "Node", ref: "node:N-110" });
    expect(pcfRefusals[0].message).toContain(`Node node:N-110 position x (${named} "in")`);
    const pcfNode = pcfPacket.pcf_payload.nodes.find((node) => node.node_id === "node:N-110");
    expect(pcfNode?.x).toBeNull();
    expect(pcfNode?.y).toBe(0);
    const pcfWitness = witness(pcfPacket, "pcf-conversion:node-n-110:position:x");
    expect(pcfWitness.target_quantity.value).toBeNull();
    expect(pcfWitness.conversion_factor_to_target).toBeNull();
    expect(pcfWitness.conversion_status).toBe("refused_out_of_range");

    const mbfPacket = mbf(input);
    const mbfRefusals = mbfPacket.diagnostics.filter((item) => item.code === "MBF-LENGTH-OUT-OF-RANGE");
    expect(mbfRefusals).toHaveLength(1);
    expect(mbfRefusals[0].message).toContain(`Node node:N-110 position x (${named} "in")`);
    expect(mbfPacket.validation_report.validation_status).toBe("blocked");
    expect(mbfPacket.model_payload.nodes[1].x).toBeNull();
    expect(mbfPacket.mbf_text).toContain("NODE,N002,TBD_LENGTH_REFUSED,0,0\n");
    const mbfWitness = witness(mbfPacket, "caepipe-mbf-conversion:node-n-110:position:x");
    expect(mbfWitness.target_quantity.value).toBeNull();
    expect(mbfWitness.conversion_factor_to_target).toBeNull();
    expect(mbfWitness.conversion_status).toBe("refused_out_of_range");

    render(
      <>
        <PcfExportPanel model={input} result={null} analysisRun={null} />
        <CaepipeMbfExportPanel model={input} result={null} analysisRun={null} />
      </>
    );
    const pcfPanel = screen.getByTestId("pcf-export-panel");
    expect(within(pcfPanel).queryByTestId("pcf-export-link")).toBeNull();
    expect(within(pcfPanel).queryByTestId("pcf-text-link")).toBeNull();
    expect(within(pcfPanel).getByTestId("pcf-export-length-unit-refused")).toHaveTextContent(`PCF-LENGTH-OUT-OF-RANGE: Node node:N-110 position x (${named} "in")`);
    const mbfPanel = screen.getByTestId("caepipe-mbf-panel");
    expect(within(mbfPanel).queryByTestId("caepipe-mbf-export-link")).toBeNull();
    expect(within(mbfPanel).queryByTestId("caepipe-mbf-text-link")).toBeNull();
    expect(within(mbfPanel).getByTestId("caepipe-mbf-length-unit-refused")).toHaveTextContent(`MBF-LENGTH-OUT-OF-RANGE: Node node:N-110 position x (${named} "in")`);
    expect(within(mbfPanel).getByTestId("caepipe-mbf-summary")).toHaveTextContent(/^refused;/);
  });

  it.each([
    { field: "outside_diameter" as const, quantity: { value: 1e308, unit: "ft" }, named: '1e+308 "ft"', placeholder: "TBD_OD_OUT_OF_RANGE" },
    { field: "wall_thickness" as const, quantity: { value: 1e308, unit: "in" }, named: '1e+308 "in"', placeholder: "TBD_WALL_OUT_OF_RANGE" },
    { field: "wall_thickness" as const, quantity: { value: Number.NaN, unit: "in" }, named: 'NaN "in"', placeholder: "TBD_WALL_OUT_OF_RANGE" }
  ])("PCF $field $named with metre coordinates is refused, links withheld", ({ field, quantity, named, placeholder }) => {
    const input = model({ length: "m", section: (id) => (id === "pipe:P-120" ? { ...SIX_INCH_PIPE(), [field]: quantity } : null) });
    const packet = pcf(input);
    const refusals = packet.diagnostics.filter((item) => item.code === "PCF-LENGTH-OUT-OF-RANGE");
    expect(refusals).toHaveLength(1);
    expect(refusals[0].message).toContain(`Pipe segment pipe:P-120 ${field} (${named})`);
    const segment = packet.pcf_payload.pipe_segments.find((item) => item.element_id === "pipe:P-120");
    expect(segment?.[field]).toBe(placeholder);
    const refusedWitness = witness(packet, `pcf-conversion:pipe-p-120:${field}`);
    expect(refusedWitness.target_quantity.value).toBeNull();
    expect(refusedWitness.conversion_factor_to_target).toBeNull();
    expect(refusedWitness.conversion_status).toBe("refused_out_of_range");
    render(<PcfExportPanel model={input} result={null} analysisRun={null} />);
    const panel = screen.getByTestId("pcf-export-panel");
    expect(within(panel).queryByTestId("pcf-export-link")).toBeNull();
    expect(within(panel).queryByTestId("pcf-text-link")).toBeNull();
    expect(within(panel).getByTestId("pcf-export-length-unit-refused")).toHaveTextContent("PCF-LENGTH-OUT-OF-RANGE");
  });
});
