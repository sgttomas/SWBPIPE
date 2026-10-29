import type { MechanicsResult } from "../../types";

/** §8.30 correction 3 (RT-S3): the solver's warning that a variable spring hanger's entered
 * installed, cold or hot load is not applied as preload (`core/product_physics/src/validation.rs`,
 * one warning per support and field, `affected_refs` = [support, field]). */
export const HANGER_LOAD_NOT_APPLIED_AS_PRELOAD = "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD";

const FIELD_NAMES: Readonly<Record<string, string>> = {
  "hanger.installed_load": "installed",
  "hanger.cold_load": "cold",
  "hanger.hot_load": "hot"
};

/** The results standing line's sentence for those warnings, or null when there are none. Each
 * support is named once, in the order of its first warning, with its fields in that order. */
export function hangerPreloadStanding(result: Pick<MechanicsResult, "diagnostics"> | null | undefined): string | null {
  const fields = new Map<string, string[]>();
  for (const diagnostic of result?.diagnostics ?? []) {
    if (diagnostic.code !== HANGER_LOAD_NOT_APPLIED_AS_PRELOAD) continue;
    const [support, field] = diagnostic.affected_refs ?? [];
    if (!support) continue;
    const name = FIELD_NAMES[field ?? ""] ?? field ?? "load";
    const named = fields.get(support) ?? [];
    if (!named.includes(name)) named.push(name);
    fields.set(support, named);
  }
  if (fields.size === 0) return null;
  const listed = Array.from(fields, ([support, names]) => `${support} (${joinNames(names)} load)`).join("; ");
  return `Spring hanger loads are not applied as preload: ${listed}; results treat each hanger as a linear spring from zero load.`;
}

function joinNames(names: string[]): string {
  if (names.length <= 1) return names.join("");
  return `${names.slice(0, -1).join(", ")} and ${names[names.length - 1]}`;
}
