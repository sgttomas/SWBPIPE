// On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
// model's existing checks, run when the user asks, with their findings as the
// checks state them. No check is defined here; each comes from its owner:
// - the solve's own checks before stiffness formation (input validation, the
//   restraint pre-checks and the W-8 topology checks;
//   core/product_physics/src/model_settings/model_checks.rs), through the native
//   `check_preview_model` command under the memory pool in the app, or the wasm
//   engine's bounded `model_checks_json` in the browser preview;
// - the W-13 model-data slot checks of the one document validator
//   (`modelDocumentDiagnostics`);
// - the W-13 library-reference findings the store published for this model.
// Running them never changes the model: each reads a copy of its JSON text or
// the model value as it stands.
import type { Diagnostic, EntityRef, PreviewModel } from "../../types";
import { loadedWasmEngine, WASM_ENGINE_BUILD_COMMAND } from "../../services/wasmEngine/loadWasmEngine";
import { invokeWithIngress, nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { resolveEntitySelection } from "../results/resultInterpretation";

export const SOLVE_CHECKS_SOURCE = "core/product_physics/src/model_settings/model_checks.rs";

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
  /** Solve findings past the report's bound, counted but not listed. */
  notShown: number;
}>;

/** The browser engine's input bound, as the engine itself refuses above it
 * (`operation_applier::MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`, 4 MiB). */
export const MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES = 4 * 1024 * 1024;
export const MODEL_CHECKS_INPUT_TOO_LARGE = "MODEL-CHECKS-INPUT-TOO-LARGE";

type ModelChecksReport = {
  document_kind?: string;
  checked?: boolean;
  diagnostics?: Array<Diagnostic & { truncated?: boolean }>;
  diagnostics_not_shown?: number;
  refusal?: string;
  refusal_code?: string;
  error?: { code: string; message: string };
};

type SolveChecks = { outcome: SolveChecksOutcome; diagnostics: Diagnostic[]; notShown: number };

function unavailable(reason: string): SolveChecks {
  return { outcome: { kind: "unavailable", reason }, diagnostics: [], notShown: 0 };
}

/** The solve's pre-formation checks for `model`. In the native app they run in
 * the native host, admitted and leased by its memory pool as every model-sized
 * command is (`check_preview_model`); in the browser preview, in the wasm
 * engine, for a model text of at most `MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`.
 * Either way the report is bounded. */
export async function solveModelChecks(model: PreviewModel): Promise<SolveChecks> {
  let report: ModelChecksReport;
  if (isTauriRuntime()) {
    try {
      report = await invokeWithIngress<ModelChecksReport>("check_preview_model", { model });
    } catch (error) {
      return unavailable(nativeFailureText(error));
    }
  } else {
    const engine = loadedWasmEngine();
    if (!engine) return unavailable(`The solve's checks are in the wasm engine, which is not loaded; build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.`);
    const text = JSON.stringify(model);
    // UTF-16 length never exceeds the UTF-8 length, so a longer text is over the
    // engine's byte bound too; it is refused here without copying it into wasm memory.
    if (text.length > MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES) {
      return unavailable(`${MODEL_CHECKS_INPUT_TOO_LARGE}: the browser preview checks a model text of at most ${MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES} bytes; this one is longer. The desktop app checks larger models under its memory pool.`);
    }
    try {
      report = JSON.parse(engine.modelChecksJson(text)) as ModelChecksReport;
    } catch (error) {
      return unavailable(`The solve's checks did not return a report: ${String(error)}`);
    }
  }
  if (report.error) return unavailable(`${report.error.code}: ${report.error.message}`);
  if (report.checked === false) {
    if (report.refusal_code) return unavailable(report.refusal ?? report.refusal_code);
    return { outcome: { kind: "refused", refusal: report.refusal ?? "the solve could not read the model" }, diagnostics: [], notShown: 0 };
  }
  if (report.checked !== true || !Array.isArray(report.diagnostics)) {
    return unavailable("The solve's checks returned a report the desktop cannot read.");
  }
  return {
    outcome: { kind: "checked" },
    diagnostics: report.diagnostics.map((item) => ({ ...item, source: item.source ?? SOLVE_CHECKS_SOURCE })),
    notShown: typeof report.diagnostics_not_shown === "number" ? report.diagnostics_not_shown : 0
  };
}

function entityFor(model: PreviewModel, diagnostic: Diagnostic): EntityRef | null {
  for (const ref of diagnostic.affected_refs ?? []) {
    const entity = resolveEntitySelection(model, ref);
    if (entity) return entity;
  }
  return null;
}

/** Runs every check on `model` and lists the findings by group, in each check's own order. */
export async function runModelChecks(model: PreviewModel, libraryReferenceDiagnostics: readonly Diagnostic[] = []): Promise<ModelCheckRun> {
  const solve = await solveModelChecks(model);
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
  return { model, solve: solve.outcome, findings, notShown: solve.notShown };
}
