// RT-A4 / RT-A9 on the desktop (DEL-05-02 Work `absolute-sum-srss-combinations.md` §5.1,
// §5.2; owned by DEL-07-03): the three-axis static seismic template. §5.1 reads
// "(base + EQx) − base": a subtraction of two load-case states, each with κ = 1, so the
// difference has κ = 0. It is not a mechanics combination "base case + EQ-only case", which
// would carry the fixed load twice. The person chooses a base case and, for X, Y and Z, an
// explicit and complete axis-state case (the base's actions plus that axis's excitation).
// The template then writes, through `create_combination`, one `result_state_subtraction`
// per axis (axis state minus base, no `difference_basis`: its gate G-1 is a thermal
// comparison) and one unsigned combination, `srss` or `absolute_sum` as the person
// chooses, over the three differences as typed combination operands. It composes no load,
// creates no load case, supplies no gravity or excitation value and makes no code,
// spectrum or modal claim: only a three-axis static combination is offered.
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { createCombinationIntent, parseFiniteFactor } from "./combinationAuthoring";

export const SEISMIC_AXES = ["x", "y", "z"] as const;
export type SeismicAxis = (typeof SEISMIC_AXES)[number];
export type SeismicUnsignedBasis = "srss" | "absolute_sum";

export type SeismicTemplateDraft = {
  baseId: string;
  axisIds: Record<SeismicAxis, string>;
  /** No default: the person chooses. */
  basis: "" | SeismicUnsignedBasis;
  /** Each difference's factor in the unsigned combination, entered by the person; no default. */
  factors: Record<SeismicAxis, string>;
  differenceIds: Record<SeismicAxis, string>;
  combinationId: string;
};

export const SEISMIC_TEMPLATE_LABEL = "Three-axis static seismic";
export const SEISMIC_TEMPLATE_PROVENANCE = "user_entered_three_axis_static_seismic_template";
const AXIS_LABEL: Record<SeismicAxis, string> = { x: "X", y: "Y", z: "Z" };

/** Free ids for the template's four records, proposed only; the person may change them. */
export function proposeSeismicIds(model: PreviewModel, avoid: readonly string[] = []): Pick<SeismicTemplateDraft, "differenceIds" | "combinationId"> {
  const used = new Set([...(model.combinations ?? []).map((combination) => combination.id), ...model.load_cases.map((loadCase) => loadCase.id), ...avoid]);
  for (let index = 300; index < 1000; index += 1) {
    const ids = {
      differenceIds: { x: `combination:SEISMIC-X-${index}`, y: `combination:SEISMIC-Y-${index}`, z: `combination:SEISMIC-Z-${index}` },
      combinationId: `combination:SEISMIC-${index}`
    };
    if (![...Object.values(ids.differenceIds), ids.combinationId].some((id) => used.has(id))) return ids;
  }
  return { differenceIds: { x: "", y: "", z: "" }, combinationId: "" };
}

export function emptySeismicTemplateDraft(model: PreviewModel, avoid: readonly string[] = []): SeismicTemplateDraft {
  return { baseId: "", axisIds: { x: "", y: "", z: "" }, basis: "", factors: { x: "", y: "", z: "" }, ...proposeSeismicIds(model, avoid) };
}

/** Why the template cannot be queued, or null when it can. */
export function seismicTemplateProblem(model: PreviewModel, draft: SeismicTemplateDraft): string | null {
  const isLoadCase = (id: string) => model.load_cases.some((loadCase) => loadCase.id === id);
  if (!draft.baseId) return "choose the base case";
  if (!isLoadCase(draft.baseId)) return `${draft.baseId} is not a load case of this model`;
  const chosen = [draft.baseId];
  for (const axis of SEISMIC_AXES) {
    const id = draft.axisIds[axis];
    if (!id) return `choose the ${AXIS_LABEL[axis]} axis-state case`;
    if (!isLoadCase(id)) return `${id} is not a load case of this model`;
    if (chosen.includes(id)) return `the ${AXIS_LABEL[axis]} axis-state case ${id} is already chosen; the base and the three axis states are four distinct cases`;
    chosen.push(id);
  }
  if (!draft.basis) return "choose SRSS or absolute sum";
  for (const axis of SEISMIC_AXES) {
    if (!draft.factors[axis].trim()) return `not entered: the factor of the ${AXIS_LABEL[axis]} difference`;
    if (parseFiniteFactor(draft.factors[axis]) === null) return `enter a finite factor for the ${AXIS_LABEL[axis]} difference`;
  }
  const ids = [...SEISMIC_AXES.map((axis) => draft.differenceIds[axis].trim()), draft.combinationId.trim()];
  for (const id of ids) {
    if (!id) return "enter an id for each of the four combinations";
    if ((model.combinations ?? []).some((combination) => combination.id === id)) return `id=${id} already exists; choose a new combination id`;
    if (isLoadCase(id)) return `id=${id} is also a load case id; a bare reference to it would name the load case (§2)`;
  }
  if (new Set(ids).size !== ids.length) return "the four combination ids must be distinct";
  return null;
}

/** The four `create_combination` payloads, in the order they apply: the three differences, then the unsigned combination. */
export function seismicTemplatePayloads(draft: SeismicTemplateDraft): Array<Record<string, unknown> & { id: string }> {
  const differences = SEISMIC_AXES.map((axis) => ({
    id: draft.differenceIds[axis].trim(),
    label: `${AXIS_LABEL[axis]} axis state minus base (static seismic difference)`,
    basis: "result_state_subtraction",
    provenance: SEISMIC_TEMPLATE_PROVENANCE,
    minuend_id: draft.axisIds[axis],
    subtrahend_id: draft.baseId,
    terms: []
  }));
  const unsigned = {
    id: draft.combinationId.trim(),
    label: `${SEISMIC_TEMPLATE_LABEL} ${draft.basis === "srss" ? "SRSS" : "absolute sum"}`,
    basis: draft.basis,
    provenance: SEISMIC_TEMPLATE_PROVENANCE,
    terms: differences.map((difference, index) => ({ combination: difference.id, factor: parseFiniteFactor(draft.factors[SEISMIC_AXES[index]]) ?? 0 }))
  };
  return [...differences, unsigned];
}

export function seismicTemplateIntents(model: PreviewModel, draft: SeismicTemplateDraft): EditorOperationIntent[] {
  return seismicTemplatePayloads(draft).map((payload, index) =>
    createCombinationIntent({
      model,
      payload,
      rationale: index < 3 ? "user_entered_three_axis_static_seismic_difference" : "user_entered_three_axis_static_seismic_combination",
      sourceNote:
        index < 3
          ? "explicit user-chosen axis-state case minus explicit user-chosen base case (three-axis static seismic template); no load composed, no difference_basis"
          : `explicit user-chosen ${draft.basis} over the template's three differences; no code, spectrum or modal claim`
    })
  );
}

export const SEISMIC_TEMPLATE_EXPLANATION = [
  "Base case: the actions every state shares (as authored; the template does not read or change them). Axis-state case: an explicit, complete case holding the base's actions plus one axis's static excitation, entered by you.",
  "Each difference is the axis state minus the base, a result-state subtraction of two load cases; each case has fixed-load count κ = 1, so each difference has κ = 0 (§5.1). It is the static seismic response about the base state, not \"base case + EQ-only case\", which would carry the fixed load twice.",
  "The unsigned combination takes the three differences as typed combination operands, each with the factor you enter; its rows are unsigned bounds and claim no equilibrium state."
] as const;

export const SEISMIC_TEMPLATE_APPLY_TEXT =
  "Apply them together with 'Review all queued changes as one batch' to apply all four or none (it moves every change you have queued); applied singly, each applies on its own, and one already applied stays applied if a later one is refused.";

export const SEISMIC_TEMPLATE_BOUNDARY =
  "Only a three-axis static combination is offered: no load composition, no gravity or excitation value, no code, response-spectrum or modal-response claim. The excitation stays in your axis-state cases.";
