// On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
// model's existing checks, run when the user asks, with their findings as the
// checks state them. No check is defined here; each comes from its owner:
// - the solve's own checks before stiffness formation (input validation, the
//   restraint pre-checks and the W-8 topology checks), through the wasm
//   engine's `model_checks_json` (core/product_physics/src/model_checks.rs);
// - the W-13 model-data slot checks of the one document validator
//   (`modelDocumentDiagnostics`);
// - the W-13 library-reference findings the store published for this model.
// Running them never changes the model: each reads a copy of its JSON text or
// the model value as it stands.
import type { Diagnostic, EntityRef, PreviewModel } from "../../types";
import { loadedWasmEngine, WASM_ENGINE_BUILD_COMMAND } from "../../services/wasmEngine/loadWasmEngine";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { resolveEntitySelection } from "../results/resultInterpretation";

export const SOLVE_CHECKS_SOURCE = "core/product_physics/src/model_checks.rs";

export type ModelCheckGroupId = "solve" | "slots" | "library";

export const MODEL_CHECK_GROUP_LABELS: Record<ModelCheckGroupId, string> = {
  solve: "Solve input, restraint and topology checks",
  slots: "Model-data slot checks",
  library: "Library references"
};

export type ModelCheckFinding = Readonly<{
  key: string;
  group: ModelCheckGroupId;
  diagnostic: Diagnostic;
  /** The first affected reference that names a model entity; null when none does. */
  entity: EntityRef | null;
}>;

/** Whether the solve's checks ran: they need the wasm engine and a model the solve can read. */
export type SolveChecksOutcome =
  | Readonly<{ kind: "checked" }>
  | Readonly<{ kind: "refused"; refusal: string }>
  | Readonly<{ kind: "unavailable"; reason: string }>;

export type ModelCheckRun = Readonly<{
  /** The model value the checks read; a later model is a different value. */
  model: PreviewModel;
  solve: SolveChecksOutcome;
  findings: readonly ModelCheckFinding[];
}>;

type ModelChecksReport = {
  document_kind?: string;
  checked?: boolean;
  diagnostics?: Diagnostic[];
  refusal?: string;
  error?: { code: string; message: string };
};

/** The solve's pre-formation checks for `model`, as the wasm engine reports them. */
export function solveModelChecks(model: PreviewModel): { outcome: SolveChecksOutcome; diagnostics: Diagnostic[] } {
  const engine = loadedWasmEngine();
  if (!engine) {
    return {
      outcome: { kind: "unavailable", reason: `The solve's checks are in the wasm engine, which is not loaded; build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.` },
      diagnostics: []
    };
  }
  let report: ModelChecksReport;
  try {
    report = JSON.parse(engine.modelChecksJson(JSON.stringify(model))) as ModelChecksReport;
  } catch (error) {
    return { outcome: { kind: "unavailable", reason: `The solve's checks did not return a report: ${String(error)}` }, diagnostics: [] };
  }
  if (report.error) return { outcome: { kind: "unavailable", reason: `${report.error.code}: ${report.error.message}` }, diagnostics: [] };
  if (report.checked === false) return { outcome: { kind: "refused", refusal: report.refusal ?? "the solve could not read the model" }, diagnostics: [] };
  if (report.checked !== true || !Array.isArray(report.diagnostics)) {
    return { outcome: { kind: "unavailable", reason: "The solve's checks returned a report the desktop cannot read." }, diagnostics: [] };
  }
  return { outcome: { kind: "checked" }, diagnostics: report.diagnostics.map((item) => ({ ...item, source: item.source ?? SOLVE_CHECKS_SOURCE })) };
}

function entityFor(model: PreviewModel, diagnostic: Diagnostic): EntityRef | null {
  for (const ref of diagnostic.affected_refs ?? []) {
    const entity = resolveEntitySelection(model, ref);
    if (entity) return entity;
  }
  return null;
}

/** Runs every check on `model` and lists the findings by group, in each check's own order. */
export function runModelChecks(model: PreviewModel, libraryReferenceDiagnostics: readonly Diagnostic[] = []): ModelCheckRun {
  const solve = solveModelChecks(model);
  const groups: Array<[ModelCheckGroupId, readonly Diagnostic[]]> = [
    ["solve", solve.diagnostics],
    ["slots", modelDocumentDiagnostics(model)],
    ["library", libraryReferenceDiagnostics]
  ];
  const findings = groups.flatMap(([group, diagnostics]) =>
    diagnostics.map((diagnostic, index): ModelCheckFinding => ({
      key: `${group}:${index}:${diagnostic.id ?? diagnostic.code}`,
      group,
      diagnostic,
      entity: entityFor(model, diagnostic)
    }))
  );
  return { model, solve: solve.outcome, findings };
}
