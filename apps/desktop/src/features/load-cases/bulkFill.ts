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

/** Members the editors keep "as authored" but do not show or edit: a reason or
 * reference that belongs to its own row, never copied to another (review N1). */
export const ELEMENT_STATE_UNEDITED = ["analysis_basis_override", "mass_state_ref"] as const;
export const SUPPORT_STATE_UNEDITED = ["base_motion", "device_reference"] as const;

/** Why a support state cannot be copied: its locked participation carries
 * components the editor does not show. */
export function supportStateCopyRefusal(row: DraftRecord): string | undefined {
  const participation = row.participation && typeof row.participation === "object" ? row.participation as DraftRecord : {};
  return participation.kind === "locked_equivalent_support" ? "a locked equivalent support carries components this editor does not show" : undefined;
}

const without = (row: DraftRecord, omit: readonly string[]) => Object.fromEntries(Object.entries(row).filter(([key]) => !omit.includes(key)));

/** Appends a copy of `template`, with `key` set to the target, for each target the
 * list does not already name. Existing rows are kept unchanged and in place; the
 * targets they name are skipped and reported. Members in `omit` are not copied,
 * and those the template held are reported. */
export function fillFromTemplate(list: readonly DraftRecord[], template: DraftRecord, key: string, targets: readonly string[], omit: readonly string[] = []): { rows: DraftRecord[]; added: string[]; skipped: string[]; omitted: string[] } {
  const named = new Set(list.map(row => row[key]).filter((id): id is string => typeof id === "string"));
  const added = targets.filter(id => !named.has(id));
  const skipped = targets.filter(id => named.has(id));
  const copy = without(template, omit);
  const omitted = omit.filter(member => template[member] !== undefined);
  return { rows: [...list, ...added.map(id => ({ ...clone(copy), [key]: id }))], added, skipped, omitted };
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
 * reported, never redirected. Members the editor does not show are left out of
 * the element and support states, and a support state it cannot copy is left
 * out; both are reported (review N1). */
export function copyAnalysisState(source: unknown, primitives: readonly string[]): { draft: DraftRecord; dropped: string[]; omitted: string[]; supportsLeftOut: string[] } {
  const draft = clone(source && typeof source === "object" ? source as DraftRecord : {});
  const omitted = new Set<string>();
  const strip = (rows: unknown, omit: readonly string[]) => Array.isArray(rows) ? (rows as DraftRecord[]).map(row => {
    for (const member of omit) if (row[member] !== undefined) omitted.add(member);
    return without(row, omit);
  }) : rows;
  if (Array.isArray(draft.element_states)) draft.element_states = strip(draft.element_states, ELEMENT_STATE_UNEDITED);
  const supportsLeftOut: string[] = [];
  if (Array.isArray(draft.support_states)) {
    const kept = (draft.support_states as DraftRecord[]).filter(row => {
      if (!supportStateCopyRefusal(row)) return true;
      supportsLeftOut.push(String(row.support_ref));
      return false;
    });
    draft.support_states = strip(kept, SUPPORT_STATE_UNEDITED);
  }
  const sources = Array.isArray(draft.load_sources) ? draft.load_sources as DraftRecord[] : undefined;
  const kept = sources?.filter(row => primitives.includes(String(row.source_ref)));
  const dropped = sources ? sources.filter(row => !kept!.includes(row)).map(row => String(row.source_ref)) : [];
  return { draft: kept ? { ...draft, load_sources: kept } : draft, dropped, omitted: [...omitted], supportsLeftOut };
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

/** A copy of another case's pressure regions, with ids re-derived for this case.
 * A source without regions, or ids that would coincide after re-derivation, is
 * refused, so the draft is never replaced by an empty or colliding copy (review N2, N4). */
export function copyPressureRegions(regions: unknown, sourceCaseId: string, targetCaseId: string): DraftRecord[] {
  if (!Array.isArray(regions) || regions.length === 0) throw new Error(`${sourceCaseId} has no pressure regions to copy.`);
  const rows = (clone(regions) as DraftRecord[]).map(row => typeof row.id === "string" ? { ...row, id: rederiveRegionId(row.id, sourceCaseId, targetCaseId) } : row);
  const ids = rows.map(row => row.id);
  const collisions = [...new Set(ids.filter((id, i) => ids.indexOf(id) !== i))];
  if (collisions.length) throw new Error(`Copying would give two regions the id ${collisions.join(", ")}; rename the regions in ${sourceCaseId} first. The draft is unchanged.`);
  return rows;
}
