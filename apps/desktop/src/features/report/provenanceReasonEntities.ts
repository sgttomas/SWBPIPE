// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §6.2; Codex review
// round 3 of #1639): the native render names a record it refuses by its collection position
// only, since ids, labels and source text are user-authored. The panel holds the input it
// sent, so it resolves the position to the record's entity for its own message. The
// resolution is display text in this session only; it is never written to a file.

type Json = Record<string, unknown>;

function object(value: unknown): Json | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Json) : null;
}

function at(list: unknown, index: number): Json | null {
  return Array.isArray(list) ? object(list[index]) : null;
}

function text(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}

/** The entity a refusal reason's first record position names in `input` (the renderer input
 * the panel sent), or null when the reason names none or the input has no such record. */
export function provenanceReasonEntity(reason: string, input: unknown): string | null {
  const match = /\b(user_supplied_values|assumptions|library_references|provenance_notes|limitations|diagnostics)\[(\d+)\]/.exec(reason);
  if (!match) return null;
  const root = object(input);
  const sections = object(root?.report_sections);
  const index = Number(match[2]);
  switch (match[1]) {
    case "user_supplied_values": return text(at(sections?.user_supplied_values, index)?.value_id);
    case "assumptions": return text(at(sections?.assumptions, index)?.assumption_id);
    case "provenance_notes": return text(at(sections?.provenance_notes, index)?.source_name);
    case "limitations": return text(at(sections?.limitations, index)?.limitation_id);
    case "diagnostics": return text(at(sections?.diagnostics, index)?.code);
    case "library_references": {
      const reference = at(root?.library_references, index);
      if (!reference) return null;
      const owners = Array.isArray(reference.entity_refs) ? reference.entity_refs.filter((item): item is string => typeof item === "string") : [];
      return `library ${text(reference.library_id) ?? "not recorded"}${owners.length ? `, referenced by ${owners.join(", ")}` : ""}`;
    }
    default: return null;
  }
}
