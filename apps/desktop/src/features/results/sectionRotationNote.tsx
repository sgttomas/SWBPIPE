/** What a result records of its beam theory, the one sentence the results surfaces say about
 * rotations, and when they say it.
 *
 * A Timoshenko straight member's published nodal rotation is its cross-section's rotation, not the
 * slope of its centreline (DEL-05-03 REQ-011; the results' own formulation basis says so). The
 * sentence shows only when the result on screen was solved with shear deformation in straight
 * members, as the result itself records it, never as the document's present setting states it.
 * `recordedBeamTheory` is the one reading; the sentence and the results headline's theory phrase
 * are both read from it, so they cannot disagree. Used under the Results panel's Rotation group
 * heading (both panels) and appended to the Displacement Extremes boundary text; nowhere else. */
import { useId } from "react";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "./numericalResultQuality";

export const SECTION_ROTATION_NOTE =
  "Rotations are the cross-section's. In a straight member solved with shear deformation, that is not the slope of the centreline.";

export type StraightMemberTheory = "timoshenko_annulus_energy_v1" | "euler_bernoulli";
/** `straightMembers`: the theory the result records for its straight members, or null where it
 * records none (or records more than one, which no route publishes). `realizedArc`: whether the
 * result records a realized arc, or "not_known" where it has no member record to read it from. */
export type RecordedBeamTheory = Readonly<{ straightMembers: StraightMemberTheory | null; realizedArc: boolean | "not_known" }>;

/** What the reading takes of a result: a full result and a native handle's header both carry it. */
export type RecordedTheory = Readonly<{
  producer?: Readonly<{ semantic_contract_id?: string }> | null;
  status?: Readonly<{ mechanics?: string }> | null;
  diagnostics?: readonly unknown[] | null;
  contract_evidence?: unknown;
}>;

/** The pressure-free Timoshenko solve's own statement that the result has realized arcs, which stay
 * Euler–Bernoulli (DEL-05-03 Design `pf-timoshenko.md` §4). */
export const REALIZED_ARCS_DIAGNOSTIC = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";

/** The pressure-free identities and the straight-member theory each one publishes. The retained
 * identity declines a Timoshenko document (`retained_memory.rs`), so it is Euler–Bernoulli. */
const PRESSURE_FREE_THEORY: Readonly<Record<string, StraightMemberTheory>> = Object.freeze({
  [PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]: "timoshenko_annulus_energy_v1",
  [PREVIEW_PHYSICS_CONTRACT_ID]: "euler_bernoulli",
  [PREVIEW_PHYSICS_RETAINED_CONTRACT_ID]: "euler_bernoulli"
});
const NOTHING_RECORDED: RecordedBeamTheory = Object.freeze({ straightMembers: null, realizedArc: "not_known" });

const records = (value: unknown): unknown[] => (Array.isArray(value) ? value : []);
const member = (value: unknown, key: string): unknown =>
  value !== null && typeof value === "object" ? (value as Record<string, unknown>)[key] : undefined;
const isTheory = (value: unknown): value is StraightMemberTheory => value === "timoshenko_annulus_energy_v1" || value === "euler_bernoulli";

/** The result's own record of its beam theory, read from the result alone (never the document):
 * - Pressure-free route: the identity gives the straight members' theory. That route publishes no
 *   per-member record today. A solved Timoshenko result states its realized arcs in the information
 *   diagnostic PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI: the solve appends it, once, exactly when
 *   the document selects Timoshenko and the built model has a realized arc (`pf_timoshenko.rs`
 *   `append_arc_disclosure`), on the path of the one solved envelope (`lib.rs`
 *   `run_linear_static_preview_observed_inner`), so on a solved Timoshenko result its absence means
 *   no realized arc (pinned by `pf_timoshenko_runtime.rs`
 *   `realized_arcs_are_disclosed_once_and_only_on_a_timoshenko_result` and
 *   `a_geometry_only_bend_gives_its_plain_straight_twins_rows`, and `pf_timoshenko_envelopes.rs`).
 *   A result that is not solved, and an Euler–Bernoulli result, has no such statement: there
 *   `realizedArc` is "not_known", never a guess from the document.
 * - Exact routes: each member's record (`contract_evidence.exact_cases[].pipe_sections[]`) gives
 *   its theory (`beam_theory.model`) and marks a realized arc (`member_kind: "realized_arc"`, whose
 *   theory is Euler–Bernoulli under either selection); the straight members' theory is that of the
 *   records not so marked. A result with no straight member record records no straight theory.
 * - Anything else records nothing. */
export function recordedBeamTheory(result: RecordedTheory | null | undefined): RecordedBeamTheory {
  if (!result) return NOTHING_RECORDED;
  const identity = result.producer?.semantic_contract_id;
  if (identity !== undefined && Object.hasOwn(PRESSURE_FREE_THEORY, identity)) {
    const straightMembers = PRESSURE_FREE_THEORY[identity];
    // Replaced by the per-member record once the pressure-free route publishes one.
    const stated = straightMembers === "timoshenko_annulus_energy_v1" && result.status?.mechanics === "MECHANICS_SOLVED";
    return { straightMembers, realizedArc: stated ? records(result.diagnostics).some((item) => member(item, "code") === REALIZED_ARCS_DIAGNOSTIC) : "not_known" };
  }
  const exactCases = member(result.contract_evidence, "exact_cases");
  if (!Array.isArray(exactCases)) return NOTHING_RECORDED;
  const sections = exactCases.flatMap((exactCase) => records(member(exactCase, "pipe_sections")));
  const arc = (section: unknown) => member(section, "member_kind") === "realized_arc";
  const straight = new Set(sections.filter((section) => !arc(section)).map((section) => member(member(section, "beam_theory"), "model")));
  const [only] = straight;
  return { straightMembers: straight.size === 1 && isTheory(only) ? only : null, realizedArc: sections.some(arc) };
}

/** Whether the result was solved with shear deformation in its straight members, by its own record. */
export function showsSectionRotationNote(result: RecordedTheory | null | undefined): boolean {
  return recordedBeamTheory(result).straightMembers === "timoshenko_annulus_energy_v1";
}

/** `text` with the sentence appended when the result shows it. */
export function withSectionRotationNote(text: string, result: RecordedTheory | null | undefined): string {
  return showsSectionRotationNote(result) ? `${text} ${SECTION_ROTATION_NOTE}` : text;
}

/** The line under a result group's heading: rendered for the Rotation group of a result that shows
 * it, and it describes its group (`aria-describedby` on the enclosing section) while it is shown. */
export function RotationGroupNote({ group, result }: { group: string; result: RecordedTheory | null | undefined }) {
  const id = useId();
  if (group !== "Rotation" || !showsSectionRotationNote(result)) return null;
  return (
    <p
      id={id}
      className="muted"
      data-testid="result-group-rotation-note"
      ref={(line) => {
        const section = line?.parentElement;
        if (!section) return;
        section.setAttribute("aria-describedby", id);
        return () => section.removeAttribute("aria-describedby");
      }}
    >
      {SECTION_ROTATION_NOTE}
    </p>
  );
}
