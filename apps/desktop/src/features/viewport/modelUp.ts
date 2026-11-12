// A model's up direction for drawing it: the desktop's rule, for the three-dimensional viewport and
// the comparison schematic alike. Up is opposite the axis on which every weight and self-weight
// load acts; with none or with disagreement it stays Y and a view readout says so. One pure
// function, reading only the model's primitive loads; it decides nothing about the model and
// writes nothing.
//
// Which loads are read, in the rule's words: "A load is read as weight if its category is weight,
// whoever authored it, or if it is a self-weight load the generator wrote, known by its provenance
// method; a distributed force not so marked is not read; weight moments are not read; the readout
// says which loads the direction was taken from, so many weight loads and so many generated
// self-weight loads, or why it stayed on Y. A model whose only gravity is generated self-weight is
// the ordinary first model and must open upright."
//
// As the model document stores them:
// - a weight load is a primitive load of category `weight`, hand-entered or written by the
//   self-weight generator for a component (`core/product_physics/src/self_weight.rs`,
//   COMPONENT_WEIGHT_CATEGORY);
// - a generated self-weight load is a member's self-weight the generator wrote as a
//   `distributed_force`, known by the method its provenance evidence names (GENERATED_METHOD there,
//   SELF_WEIGHT_MEMBER_METHOD here);
// - a weight moment (one the generator writes for a component whose centre of gravity is offset;
//   its direction is a rotation axis) acts about an axis, not along it, and is not read.
// A force acts along its global axis with the sign of its magnitude; one with a zero or unreadable
// magnitude acts nowhere and is not read. Loads acting along different signed axes (opposite
// signs on one axis included) disagree. Every load case is read, whatever its status.

export type ModelAxis = "x" | "y" | "z";
/** A signed global axis: the direction drawn upward, or the one a load acts in. */
export type SignedAxis = "+x" | "-x" | "+y" | "-y" | "+z" | "-z";

export type ModelUpBasis =
  /** Every weight and generated self-weight load acts along one signed axis: up is opposite it. */
  | "weight_loads"
  /** The model has no weight load and no generated self-weight load: up stays +Y. */
  | "no_weight_load"
  /** Its weight and generated self-weight loads act along different signed axes, or one acts
   * along no global axis: up stays +Y. */
  | "weight_loads_disagree";

export type ModelUp = Readonly<{
  up: SignedAxis;
  basis: ModelUpBasis;
  /** The loads of category `weight` read (forces only). */
  weightLoads: number;
  /** The generated self-weight loads read. */
  selfWeightLoads: number;
  /** The signed axes the loads read act along, in order; with `offAxis`, whether one acts along no global axis. */
  acting: readonly SignedAxis[];
  offAxis: boolean;
}>;

/** The self-weight generator's evidence method of a member's self-weight (`GENERATED_METHOD` in
 * `core/product_physics/src/self_weight.rs`). */
export const SELF_WEIGHT_MEMBER_METHOD = "pipe_mass_per_length_times_explicit_axis_acceleration/v2";

type PlainObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is PlainObject {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function provenanceMethod(provenance: unknown): string | null {
  let value = provenance;
  if (typeof value === "string") {
    if (!value.trimStart().startsWith("{")) return null;
    try { value = JSON.parse(value); } catch { return null; }
  }
  return isPlainObject(value) && typeof value.method === "string" ? value.method : null;
}

/** How a load is read: as a weight load, as a generated self-weight load, or not at all. */
function readAs(load: PlainObject): "weight" | "self_weight" | null {
  if (load.category === "weight") return "weight";
  return provenanceMethod(load.provenance) === SELF_WEIGHT_MEMBER_METHOD ? "self_weight" : null;
}

/** The global axis a force direction names (`global_z`, `GLOBAL_Z`, `UZ`, `Uz`, `uz`), or null. */
function forceAxis(direction: unknown): ModelAxis | null {
  if (typeof direction !== "string") return null;
  const match = /^(?:global_([xyz])|u([xyz]))$/i.exec(direction);
  return match ? ((match[1] ?? match[2]).toLowerCase() as ModelAxis) : null;
}

function isMoment(load: PlainObject): boolean {
  return load.dimension === "moment" || (typeof load.direction === "string" && /^(?:rotation_[xyz]|r[xyz])$/i.test(load.direction));
}

const AXIS_ORDER: readonly SignedAxis[] = ["+x", "-x", "+y", "-y", "+z", "-z"];

/** The model's up direction by the desktop's rule, with which case holds and the loads it was taken from. */
export function deriveModelUp(model: unknown): ModelUp {
  const acting = new Set<SignedAxis>();
  let weightLoads = 0;
  let selfWeightLoads = 0;
  let offAxis = false;
  const cases = isPlainObject(model) && Array.isArray(model.load_cases) ? model.load_cases : [];
  for (const loadCase of cases) {
    const loads = isPlainObject(loadCase) && Array.isArray(loadCase.primitive_loads) ? loadCase.primitive_loads : [];
    for (const load of loads) {
      if (!isPlainObject(load)) continue;
      const kind = readAs(load);
      if (!kind || isMoment(load)) continue;
      const magnitude = isPlainObject(load.magnitude) ? load.magnitude.value : undefined;
      if (typeof magnitude !== "number" || !Number.isFinite(magnitude) || magnitude === 0) continue;
      if (kind === "weight") weightLoads += 1;
      else selfWeightLoads += 1;
      const axis = forceAxis(load.direction);
      if (!axis) { offAxis = true; continue; }
      acting.add(`${magnitude < 0 ? "-" : "+"}${axis}` as SignedAxis);
    }
  }
  const ordered = AXIS_ORDER.filter((axis) => acting.has(axis));
  const counts = { weightLoads, selfWeightLoads, acting: ordered, offAxis };
  if (weightLoads + selfWeightLoads === 0) return { up: "+y", basis: "no_weight_load", ...counts };
  if (offAxis || ordered.length !== 1) return { up: "+y", basis: "weight_loads_disagree", ...counts };
  // Up is opposite the direction the loads act in.
  const up = `${ordered[0][0] === "-" ? "+" : "-"}${ordered[0][1]}` as SignedAxis;
  return { up, basis: "weight_loads", ...counts };
}

/** The signed axis as shown: `+Z`, `−Y`. */
export function signedAxisText(axis: SignedAxis): string {
  return `${axis[0] === "-" ? "−" : "+"}${axis[1].toUpperCase()}`;
}

function counted(count: number, one: string, many: string): string {
  return `${count} ${count === 1 ? one : many}`;
}

/** The readout of the derived up direction: the loads it was taken from, or why it stayed on Y. */
export function modelUpReadout(derived: ModelUp): string {
  const loads = `${counted(derived.weightLoads, "weight load", "weight loads")} and ${counted(derived.selfWeightLoads, "generated self-weight load", "generated self-weight loads")}`;
  if (derived.basis === "weight_loads") return `Up ${signedAxisText(derived.up)}, opposite ${loads}.`;
  if (derived.basis === "no_weight_load") return "Up +Y: the model has no weight load and no generated self-weight load.";
  const along = derived.acting.map(signedAxisText).join(" and ");
  const parts = [along ? `they act along ${along}` : "", derived.offAxis ? `${along ? "and " : ""}one acts along no global axis` : ""].filter(Boolean).join(" ");
  return `Up +Y: the model's ${loads} disagree (${parts}).`;
}
