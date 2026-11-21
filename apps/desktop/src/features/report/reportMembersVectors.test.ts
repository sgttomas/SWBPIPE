/** DEL-08-01: report inputs the desktop's builder writes for the branches of the native
 * producers of a report input's row-scale members (`core/reporting/report_package/src/report_members.rs`)
 * that no other committed builder input reaches. Each vector is an invented edit of a
 * committed result (or of its model), solved under the native-capture test seams
 * (`runPreviewMechanics`, `buildAnalysisRunPreview`) and built by the rendered report panel's
 * build step (`assembleRenderedReport`, local private, no numerical-health line), the step
 * that takes a whole result to a report input. The vectors are committed in
 * `fixtures/reports/invented/report_members_vectors/` (stale fails;
 * `OPENPIPESTRESS_WRITE_REPORT_MEMBERS_VECTORS=<dir>` writes them there instead), and
 * `core/reporting/report_package/tests/report_members_parity.rs` holds each native member,
 * and the input completed natively and sealed, equal to the builder's.
 *
 * Unit transport replay of committed producer bytes, edited as each vector states, through
 * mocked IPC; NOT a native UI qualification witness and not engineering output. Job ids are
 * invented. Standard claim fence applies (claims taxonomy per DEC-081). */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { PreviewModel } from "../../types";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { assembleRenderedReport } from "./renderedReportAssembly";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const DIR = "fixtures/reports/invented/report_members_vectors";
const GENERATOR = "apps/desktop/src/features/report/reportMembersVectors.test.ts";
const WRITE = "OPENPIPESTRESS_WRITE_REPORT_MEMBERS_VECTORS";
const MODE = "sparse_interactive" as const;

/** The committed pressure-free pair the vectors start from: the connected preview-physics-2
 * sparse solve and its model. */
const PREVIEW_SOURCE = "fixtures/results/preview_physics_connected_sparse.json";
const PREVIEW_MODEL = "fixtures/model_operations/precision_connected_ui_model.json";

type Vector = {
  name: string;
  reason: string;
  source_fixture: string;
  model_fixture: string;
  editSource?: (source: Json) => void;
  editModel?: (model: Json) => void;
  /** What the builder's input shows of the branch. */
  shows: (report: Json, source: Json) => void;
};

/** The `index`th source row of `kind`. */
const rowOf = (source: Json, kind: string, index = 0) => source.results.filter((row: Json) => row.kind === kind)[index];
/** The solver-mode row, whose basis and coordinate system the preview reader reads as text. */
const SOLVER_MODE_ROW = "linear_solver_mode_basis";

const VECTORS: readonly Vector[] = [
  {
    name: "coordinate_systems_order_and_trim",
    reason: "the coordinate systems are listed once each in UTF-16 code unit order and a name is kept as written unless it is blank after JavaScript's trim or exactly TBD: U+1F600 (a surrogate pair) sorts before U+FF2E, U+FEFF and a no-break space are blank, U+0085 is not, and padded names are kept",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editSource: (source) => {
      const names: [string, number, string][] = [
        [SOLVER_MODE_ROW, 0, "\u{1F600}"],
        ["global_nodal_displacement_x", 0, "\u{FF2E}"],
        ["global_nodal_displacement_x", 1, "\u{A0} "],
        ["global_nodal_displacement_y", 0, " global "],
        ["global_nodal_displacement_z", 0, "\u{FEFF}"],
        ["global_nodal_rotation_x", 0, "\u{85}"],
        ["global_nodal_rotation_y", 0, " TBD "],
        ["global_nodal_rotation_z", 0, "TBD"]
      ];
      for (const [kind, index, name] of names) rowOf(source, kind, index).metadata.coordinate_system = name;
    },
    shows: (report) => expect(report.report_context.result_coordinate_systems).toEqual([" TBD ", " global ", "element_local", "global", "pipe_section", "\u{85}", "\u{1F600}", "\u{FF2E}"])
  },
  {
    name: "section_basis_trim",
    reason: "a section-basis token is trimmed as JavaScript trims: on the solver-mode row a token wrapped in U+FEFF names the nominal section and a token led by U+0085 names none, so the row's label carries [nominal section]",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editSource: (source) => {
      rowOf(source, SOLVER_MODE_ROW).metadata.basis += ";\u{FEFF}section_basis=nominal_section\u{FEFF};\u{85}section_basis=reduced_section";
    },
    shows: (report) => expect(report.result_rows[0].label).toBe(`${SOLVER_MODE_ROW} (linear_solver_mode) [nominal section]`)
  },
  {
    name: "session_diagnostics_after_results",
    reason: "the session's Diagnostics entries follow the result's, in their own order: the model's material has no provenance note and the model has one component without provenance, so the builder adds a component entry and then a material entry after the result's",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editModel: (model) => {
      delete model.materials[0].provenance;
      model.components = [{ id: "component:invented-bend", label: "Invented bend", kind: "bend" }];
    },
    shows: (report, source) => expect(report.report_sections.diagnostics.slice(source.diagnostics.length).map((entry: Json) => entry.code)).toEqual(["COMPONENT_PROVENANCE_MISSING", "MATERIAL_PROVENANCE_MISSING"])
  }
];

/** One vector's model, delivered result bytes and the builder's report input. */
async function build(vector: Vector) {
  const source = JSON.parse(text(vector.source_fixture));
  vector.editSource?.(source);
  const sourceText = solveBytes(source);
  const model = JSON.parse(text(vector.model_fixture)) as PreviewModel;
  vector.editModel?.(model);
  (window as Json).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport(() => sourceText, { jobIdPrefix: `invented-members-${vector.name}-job` });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, MODE);
  const producer = result.producer!;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: producer.component_name, solver_version: producer.component_version, solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: MODE, settings: {} },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  const assembly = await assembleRenderedReport({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null, libraryRefFindings: [] });
  return { model, source, sourceText, report: assembly.input as Json };
}

function vectorFile(vector: Vector, built: Awaited<ReturnType<typeof build>>) {
  return `${JSON.stringify({
    schema_version: "1.0.0",
    generator: GENERATOR,
    description: "Invented data. A report input the desktop's builder (assembleRenderedReport, local private, no numerical-health line) wrote for an invented edit of a committed result or its model, solved by unit transport replay of the edited bytes: the model it was built for, the delivered result bytes (source_text) and the builder's whole report input. core/reporting/report_package/tests/report_members_parity.rs holds the native row-scale members equal to it. Not a native UI qualification witness and not engineering output.",
    name: vector.name,
    reason: vector.reason,
    source_fixture: vector.source_fixture,
    model_fixture: vector.model_fixture,
    solver_mode: MODE,
    model: built.model,
    source_text: built.sourceText,
    report: built.report
  })}\n`;
}

describe("the report members vectors (DEL-08-01)", () => {
  it("writes each vector the builder writes; stale fails", async () => {
    const outDir = process.env[WRITE];
    const names: string[] = [];
    for (const vector of VECTORS) {
      const built = await build(vector);
      vector.shows(built.report, built.source);
      const content = vectorFile(vector, built);
      const name = `${vector.name}.json`;
      names.push(name);
      if (outDir) {
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, name), content);
        continue;
      }
      expect(existsSync(resolve(root, `${DIR}/${name}`)) && text(`${DIR}/${name}`) === content, `${DIR}/${name} is stale; regenerate it with ${WRITE}=<dir>`).toBe(true);
    }
    expect(new Set(names).size).toBe(VECTORS.length);
  }, 120_000);
});
