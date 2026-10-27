// On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
// model's existing checks, run when the user asks, with their findings as the
// checks state them. No check is defined here; each comes from its owner:
// - the solve's own checks before stiffness formation (input validation, the
//   restraint pre-checks and the W-8 topology checks;
//   core/product_physics/src/model_settings/model_checks.rs), through the native
//   `check_preview_model` command under the memory pool in the app, or the wasm
//   engine's bounded `model_checks_json` in the browser preview;
// - the W-13 model-data slot checks of the one document validator, run by the
//   same native command or wasm call, in the same bounded report;
// - the W-13 library-reference findings the store published for this model,
//   within what the report leaves of its bound.
// Running them never changes the model: each reads a copy of its JSON text or
// the model value as it stands.
import type { Diagnostic, EntityRef, PreviewModel } from "../../types";
import { loadedWasmEngine, WASM_ENGINE_BUILD_COMMAND } from "../../services/wasmEngine/loadWasmEngine";
import { invokeWithIngress, nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import { DOCUMENT_VALIDATOR_SOURCE } from "../model-data/documentDiagnostics";
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
  /** Findings past the report's bound in each group, counted but not listed. */
  notShown: Readonly<Record<ModelCheckGroupId, number>>;
}>;

/** The browser engine's input bound, as the engine itself refuses above it
 * (`operation_applier::MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`, 4 MiB), in UTF-8 bytes. */
export const MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES = 4 * 1024 * 1024;
export const MODEL_CHECKS_INPUT_TOO_LARGE = "MODEL-CHECKS-INPUT-TOO-LARGE";
/** The report's bound on serialized findings (`operation_applier::MODEL_CHECKS_MAX_RESPONSE_BYTES`). */
export const MODEL_CHECKS_MAX_RESPONSE_BYTES = 1024 * 1024;

type ValidatorFinding = { code: string; severity: string; path: string; message: string; affected_refs?: string[]; truncated?: boolean };

type ModelChecksReport = {
  document_kind?: string;
  checked?: boolean;
  diagnostics?: Array<Diagnostic & { truncated?: boolean }>;
  diagnostics_not_shown?: number;
  slot_findings?: ValidatorFinding[];
  slot_findings_not_shown?: number;
  refusal?: string;
  refusal_truncated?: boolean;
  refusal_code?: string;
  error?: { code: string; message: string };
};

type EngineChecks = {
  outcome: SolveChecksOutcome;
  solve: Diagnostic[];
  slots: Diagnostic[];
  notShown: { solve: number; slots: number };
  /** The report's serialized findings, counted against the bound. */
  bytes: number;
};

function unavailable(reason: string): EngineChecks {
  return { outcome: { kind: "unavailable", reason }, solve: [], slots: [], notShown: { solve: 0, slots: 0 }, bytes: 0 };
}

/** The UTF-8 length of `text`, counted without encoding it; it stops counting
 * once the length passes `limit` (Codex review r2 of #1665: a UTF-16 count
 * undercounts non-ASCII text). */
export function utf8ByteLength(text: string, limit = Number.MAX_SAFE_INTEGER): number {
  let bytes = 0;
  for (let index = 0; index < text.length && bytes <= limit; index += 1) {
    const unit = text.charCodeAt(index);
    if (unit < 0x80) bytes += 1;
    else if (unit < 0x800) bytes += 2;
    else if (unit >= 0xd800 && unit <= 0xdbff && index + 1 < text.length) {
      const next = text.charCodeAt(index + 1);
      if (next >= 0xdc00 && next <= 0xdfff) {
        bytes += 4;
        index += 1;
      } else bytes += 3;
    } else bytes += 3;
  }
  return bytes;
}

/** The engine's checks for `model`: the solve's checks before stiffness
 * formation and the W-13 slot checks, in one bounded report. In the native app
 * they run in the native host (`check_preview_model`), admitted by its memory
 * pool and run in a capped worker as a solve is; in the browser preview, in the
 * wasm engine, for a model text of at most `MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`
 * UTF-8 bytes, counted before the text is passed to the engine. A refusal leaves
 * both groups not run. */
export async function engineModelChecks(model: PreviewModel): Promise<EngineChecks> {
  let report: ModelChecksReport;
  if (isTauriRuntime()) {
    try {
      report = await invokeWithIngress<ModelChecksReport>("check_preview_model", { model });
    } catch (error) {
      return unavailable(nativeFailureText(error));
    }
  } else {
    const engine = loadedWasmEngine();
    if (!engine) return unavailable(`The model checks are in the wasm engine, which is not loaded; build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.`);
    const text = JSON.stringify(model);
    const bytes = utf8ByteLength(text, MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES);
    if (bytes > MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES) {
      return unavailable(`${MODEL_CHECKS_INPUT_TOO_LARGE}: the browser preview checks a model text of at most ${MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES} bytes; this one is longer. The desktop app checks larger models under its memory pool.`);
    }
    try {
      report = JSON.parse(engine.modelChecksJson(text)) as ModelChecksReport;
    } catch (error) {
      return unavailable(`The model checks did not return a report: ${String(error)}`);
    }
  }
  if (report.error) return unavailable(`${report.error.code}: ${report.error.message}`);
  if (report.checked === false && report.refusal_code) return unavailable(report.refusal ?? report.refusal_code);
  if (typeof report.checked !== "boolean" || !Array.isArray(report.diagnostics) || !Array.isArray(report.slot_findings)) {
    return unavailable("The model checks returned a report the desktop cannot read.");
  }
  const slots = slotDiagnostics(report.slot_findings);
  const solve = report.diagnostics.map((item) => ({ ...item, source: item.source ?? SOLVE_CHECKS_SOURCE }));
  const bytes = [...report.diagnostics, ...report.slot_findings].reduce((total, item) => total + utf8ByteLength(JSON.stringify(item)) + 1, 0);
  const notShown = {
    solve: typeof report.diagnostics_not_shown === "number" ? report.diagnostics_not_shown : 0,
    slots: typeof report.slot_findings_not_shown === "number" ? report.slot_findings_not_shown : 0
  };
  const outcome: SolveChecksOutcome = report.checked
    ? { kind: "checked" }
    : { kind: "refused", refusal: report.refusal ?? "the solve could not read the model" };
  return { outcome, solve, slots, notShown, bytes };
}

/** The validator's findings as the Diagnostics list shows them (`modelDocumentDiagnostics`). */
function slotDiagnostics(findings: readonly ValidatorFinding[]): Diagnostic[] {
  return findings.map((finding, index) => ({
    id: `model-document:${finding.code}:${finding.path || "document"}:${index}`,
    code: finding.code,
    severity: finding.severity === "blocking" ? "blocking" : "warning",
    message: finding.path ? `${finding.message} (at ${finding.path})` : finding.message,
    affected_refs: finding.affected_refs ?? [],
    source: DOCUMENT_VALIDATOR_SOURCE,
    ...(finding.truncated ? { truncated: true } : {})
  }) as Diagnostic);
}

function entityFor(model: PreviewModel, diagnostic: Diagnostic): EntityRef | null {
  for (const ref of diagnostic.affected_refs ?? []) {
    const entity = resolveEntitySelection(model, ref);
    if (entity) return entity;
  }
  return null;
}

/** Runs every check on `model` and lists the findings by group, in each check's
 * own order, within the report's bound: the library-reference findings take
 * what the engine's report leaves of it, and are read only when the engine's
 * checks were admitted. */
export async function runModelChecks(model: PreviewModel, libraryReferenceDiagnostics: () => readonly Diagnostic[] = () => []): Promise<ModelCheckRun> {
  const engine = await engineModelChecks(model);
  let library: Diagnostic[] = [];
  let libraryNotShown = 0;
  if (engine.outcome.kind !== "unavailable") {
    let remaining = MODEL_CHECKS_MAX_RESPONSE_BYTES - engine.bytes;
    for (const diagnostic of libraryReferenceDiagnostics()) {
      const length = utf8ByteLength(JSON.stringify(diagnostic)) + 1;
      if (libraryNotShown > 0 || length > remaining) {
        libraryNotShown += 1;
        continue;
      }
      remaining -= length;
      library.push(diagnostic);
    }
  }
  const groups: Array<[ModelCheckGroupId, readonly Diagnostic[]]> = [
    ["solve", engine.solve],
    ["slots", engine.slots],
    ["library", library]
  ];
  const findings = groups.flatMap(([group, diagnostics]) =>
    diagnostics.map((diagnostic, index): ModelCheckFinding => ({
      key: `${group}:${index}:${diagnostic.id ?? diagnostic.code}`,
      group,
      diagnostic,
      entity: entityFor(model, diagnostic)
    }))
  );
  return { model, solve: engine.outcome, findings, notShown: { solve: engine.notShown.solve, slots: engine.notShown.slots, library: libraryNotShown } };
}
