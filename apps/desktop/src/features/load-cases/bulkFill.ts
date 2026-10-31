/* G7/G8 (DEL-07-03 `Work/authoring-speed.md` §3): draft helpers for per-member and
 * per-case lists. Each returns a new draft for the person to review; nothing here
 * queues, applies or prefills a value the person did not enter. The draft is
 * still submitted whole, as one reviewed intent, through the existing editors. */
import { clone } from "./loadStateAuthoring";
import type { DraftRecord } from "../rich-authoring/formSupport";

/** How a fill chooses its targets. */
export type FillScope =
  | { kind: "missing" }
  | { kind: "range"; from: string; to: string }
  | { kind: "chosen"; ids: readonly string[] };

/** The targets of a fill, in model order: every target the list does not yet name,
 * the targets from `from` to `to` inclusive, or the chosen ones. A range whose
 * ends are unknown or reversed selects nothing. */
export function fillTargets(ordered: readonly string[], scope: FillScope): string[] {
  if (scope.kind === "missing") return [...ordered];
  if (scope.kind === "chosen") return ordered.filter(id => scope.ids.includes(id));
  const from = ordered.indexOf(scope.from), to = ordered.indexOf(scope.to);
  return from < 0 || to < from ? [] : ordered.slice(from, to + 1);
}

/** Appends a copy of `template`, with `key` set to the target, for each target the
 * list does not already name. Existing rows are kept unchanged and in place; the
 * targets they name are skipped and reported. */
export function fillFromTemplate(list: readonly DraftRecord[], template: DraftRecord, key: string, targets: readonly string[]): { rows: DraftRecord[]; added: string[]; skipped: string[] } {
  const named = new Set(list.map(row => row[key]).filter((id): id is string => typeof id === "string"));
  const added = targets.filter(id => !named.has(id));
  const skipped = targets.filter(id => named.has(id));
  return { rows: [...list, ...added.map(id => ({ ...clone(template), [key]: id }))], added, skipped };
}

/** Appends `{source_ref, factor}` for each of the case's primitives the list does not
 * already name, with the factor the person entered. */
export function addEverySource(list: readonly DraftRecord[], primitives: readonly string[], factor: string): { rows: DraftRecord[]; added: string[] } {
  if (factor.trim() === "" || !Number.isFinite(Number(factor))) throw new Error("Enter the factor for every primitive as a finite number.");
  const named = new Set(list.map(row => row.source_ref));
  const added = primitives.filter(id => !named.has(id));
  return { rows: [...list, ...added.map(source_ref => ({ source_ref, factor }))], added };
}

/** A copy of another case's analysis state for this case. Load sources name the
 * other case's primitives, so those this case does not hold are left out and
 * reported, never redirected. */
export function copyAnalysisState(source: unknown, primitives: readonly string[]): { draft: DraftRecord; dropped: string[] } {
  const draft = clone(source && typeof source === "object" ? source as DraftRecord : {});
  const sources = Array.isArray(draft.load_sources) ? draft.load_sources as DraftRecord[] : undefined;
  if (!sources) return { draft, dropped: [] };
  const kept = sources.filter(row => primitives.includes(String(row.source_ref)));
  const dropped = sources.filter(row => !kept.includes(row)).map(row => String(row.source_ref));
  return { draft: { ...draft, load_sources: kept }, dropped };
}

/** The case id's token (`load:RS-SUS` gives `RS-SUS`). */
function caseToken(id: string): string {
  return id.includes(":") ? id.slice(id.indexOf(":") + 1) : id;
}

/** A region id for the target case: the source case's token, where it stands as a
 * whole segment of the id, is replaced with the target's. Any other id is kept. */
export function rederiveRegionId(id: string, sourceCaseId: string, targetCaseId: string): string {
  const from = caseToken(sourceCaseId), to = caseToken(targetCaseId);
  if (!from) return id;
  const escaped = from.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return id.replace(new RegExp(`(^|[:_.-])${escaped}(?=$|[:_.-])`, "g"), (_, lead: string) => `${lead}${to}`);
}

/** A copy of another case's pressure regions, with ids re-derived for this case. */
export function copyPressureRegions(regions: unknown, sourceCaseId: string, targetCaseId: string): DraftRecord[] {
  const rows = Array.isArray(regions) ? clone(regions) as DraftRecord[] : [];
  return rows.map(row => typeof row.id === "string" ? { ...row, id: rederiveRegionId(row.id, sourceCaseId, targetCaseId) } : row);
}
