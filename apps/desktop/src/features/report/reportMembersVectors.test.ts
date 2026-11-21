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
 * invented. Each vector's model is asserted to be one File > Open Model Document… admits (the
 * desktop's shape check and document validator) and its result numerically eligible. That
 * each result also passes the session's Current requirements was established by the
 * independent reader's replay probes at the head where the vectors were made; this test does
 * not assert it, because that conjunction lives in a session state module only the session
 * may import. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { PreviewModel } from "../../types";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { numericalResultStanding } from "../results/resultStanding";
import { checkNativeOutcome } from "../../services/modelDocumentFile";
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
/** The committed exact-route pair the desktop's native replay uses: the exact-pressure
 * authoring model and its sparse solve. A vector names the model (`model_pointer` "") rather
 * than copying it. */
const EXACT_SOURCE = "fixtures/results/pressure_connected_ui_mechanics_sparse.json";
const EXACT_MODEL = "fixtures/model_operations/exact_pressure_authoring_model.json";

type Vector = {
  name: string;
  reason: string;
  source_fixture: string;
  model_fixture: string;
  /** Where the model sits in `model_fixture` (a JSON pointer, "" for the whole fixture); absent,
   * the fixture is the model and the vector carries the model it was built for. */
  model_pointer?: string;
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
    name: "section_basis_nel_alone",
    reason: "a lone section-basis token led by U+0085, which JavaScript's trim keeps, names no section basis, so the solver-mode row's label carries none",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editSource: (source) => {
      rowOf(source, SOLVER_MODE_ROW).metadata.basis += ";\u{85}section_basis=nominal_section";
    },
    shows: (report) => expect(report.result_rows[0].label).toBe(`${SOLVER_MODE_ROW} (linear_solver_mode)`)
  },
  {
    name: "coordinate_system_not_text",
    reason: "a coordinate system that is not text is not one: the solver-mode row names the number 7, and the list leaves it out",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editSource: (source) => {
      rowOf(source, SOLVER_MODE_ROW).metadata.coordinate_system = 7;
    },
    shows: (report) => expect(report.report_context.result_coordinate_systems).toEqual(["element_local", "global", "pipe_section"])
  },
  {
    name: "empty_rows",
    reason: "a solved result with no rows: the connected preview with its rows, both headlines, the case's stress extrema and its attributed supports cleared, for which the builder writes empty row-scale members",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editSource: (source) => {
      source.results = [];
      source.summary.max_displacement = null;
      source.summary.max_open_formula_stress = null;
      source.contract_evidence.preview_cases[0].pipe_stress_extrema = [];
      source.contract_evidence.preview_cases[0].support_attribution.attributed_support_ids = [];
    },
    shows: (report) => {
      expect(report.result_rows).toEqual([]);
      expect(report.calculation_report.result_export_refs).toEqual([]);
      expect(report.calculation_report.model_input_summary.unit_display_summary.result_units).toEqual([]);
      expect(report.report_context.result_coordinate_systems).toEqual([]);
    }
  },
  {
    name: "exact_unknown_kind_units",
    reason: "the stored units are listed once each in UTF-16 code unit order: on the exact pressure-2 result of the authoring model, two case-scoped rows of a kind the reader does not know carry U+1F600 (a surrogate pair) and U+FF2E, and U+1F600 sorts first though its code point is greater",
    source_fixture: EXACT_SOURCE,
    model_fixture: EXACT_MODEL,
    model_pointer: "",
    editSource: (source) => {
      const template = source.results[3];
      for (const [index, unit] of ["\u{1F600}", "\u{FF2E}"].entries()) source.results.push({ ...structuredClone(template), id: `result:invented-quantity:${index}`, kind: "invented_quantity", unit });
    },
    shows: (report) => expect(report.calculation_report.model_input_summary.unit_display_summary.result_units).toEqual(["MPa", "N", "N*m", "Pa", "mm", "mode_code", "rad", "\u{1F600}", "\u{FF2E}"])
  },
  {
    name: "session_diagnostics_after_results",
    reason: "the session's Diagnostics entries follow the result's, in their own order: the model's material has a blank provenance note and its one component (at the tip node) has a blank one too, so the builder adds a component entry and then a material entry after the result's",
    source_fixture: PREVIEW_SOURCE,
    model_fixture: PREVIEW_MODEL,
    editModel: (model) => {
      // A blank note: a model document requires the member, and the builder reads blank as none.
      model.materials[0].provenance = "";
      model.components = [{ id: "component:invented-bend", label: "Invented bend", kind: "bend", node: "node:fixture-tip", provenance: "" }];
    },
    shows: (report, source) => expect(report.report_sections.diagnostics.slice(source.diagnostics.length).map((entry: Json) => entry.code)).toEqual(["COMPONENT_PROVENANCE_MISSING", "MATERIAL_PROVENANCE_MISSING"])
  }
];

/** The value at a JSON pointer. */
const pointer = (value: Json, at: string) => at.split("/").slice(1).reduce((item, key) => item[key.replaceAll("~1", "/").replaceAll("~0", "~")], value);

/** One vector's model, delivered result bytes and the builder's report input. */
async function build(vector: Vector) {
  const source = JSON.parse(text(vector.source_fixture));
  vector.editSource?.(source);
  const sourceText = solveBytes(source);
  const fixture = JSON.parse(text(vector.model_fixture));
  const model = (vector.model_pointer === undefined ? fixture : pointer(fixture, vector.model_pointer)) as PreviewModel;
  vector.editModel?.(model);
  // A model document File > Open Model Document… admits: the desktop's shape check and document validator.
  const opened = checkNativeOutcome({ outcome: "opened", file_name: `${vector.name}.json`, byte_count: 0, schema_status: "invented", document: structuredClone(model) });
  expect(opened.outcome === "refused" ? opened.diagnostics : opened.outcome, vector.name).toBe("opened");
  (window as Json).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport(() => sourceText, { jobIdPrefix: `invented-members-${vector.name}-job` });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, MODE);
  const producer = result.producer!;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: producer.component_name, solver_version: producer.component_version, solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: MODE,
      // The settings the session's solve records (workspaceSession.ts).
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: MODE === "sparse_interactive" } },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  expect(numericalResultStanding(result, model).eligible, vector.name).toBe(true);
  const assembly = await assembleRenderedReport({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null, libraryRefFindings: [] });
  return { model, source, sourceText, report: assembly.input as Json };
}

function vectorFile(vector: Vector, built: Awaited<ReturnType<typeof build>>) {
  return `${JSON.stringify({
    schema_version: "1.0.0",
    generator: GENERATOR,
    description: "Invented data. A report input the desktop's builder (assembleRenderedReport, local private, no numerical-health line) wrote for an invented edit of a committed result or its model, solved by unit transport replay of the edited bytes: the model it was built for (or, with model_pointer, where that model sits in model_fixture), the delivered result bytes (source_text) and the builder's whole report input. core/reporting/report_package/tests/report_members_parity.rs holds the native row-scale members equal to it. Not a native UI qualification witness and not engineering output.",
    name: vector.name,
    reason: vector.reason,
    source_fixture: vector.source_fixture,
    model_fixture: vector.model_fixture,
    solver_mode: MODE,
    ...(vector.model_pointer === undefined ? { model: built.model } : { model_pointer: vector.model_pointer }),
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
