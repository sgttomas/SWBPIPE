// A model's up direction for drawing it (the desktop's rule, for the three-dimensional viewport and
// the comparison schematic alike): up is opposite the axis on which every weight and self-weight
// load acts; with none, or with disagreement, it stays +Y and the view says so. One pure function,
// reading only the model's primitive loads; it decides nothing about the model and writes nothing.
//
// What counts as a weight load, as the model document stores them:
// - a primitive load of category `weight` (authored, or a component weight the self-weight
//   generator writes: `core/product_physics/src/self_weight.rs`, COMPONENT_WEIGHT_CATEGORY);
// - a member's generated self-weight, which the generator writes as a `distributed_force` whose
//   provenance evidence names its method (GENERATED_METHOD there, PIPE_WEIGHT_METHOD here).
// Of these, only forces have a direction of action: a weight moment (the generator writes one for a
// component whose centre of gravity is offset; its direction is a rotation axis) is not read. A
// force acts along its global axis with the sign of its magnitude; one with a zero or unreadable
// magnitude acts nowhere and is not read. Every load case is read.

export type ModelAxis = "x" | "y" | "z";
/** A signed global axis: the direction drawn upward. */
export type SignedAxis = "+x" | "-x" | "+y" | "-y" | "+z" | "-z";

export type ModelUpBasis =
  /** Every weight load acts along one signed axis: up is opposite it. */
  | "weight_loads"
  /** The model has no weight load with a direction of action: up stays +Y. */
  | "no_weight_load"
  /** Its weight loads act along different signed axes, or one acts along no global axis: up stays +Y. */
  | "weight_loads_disagree";

export type ModelUp = Readonly<{ up: SignedAxis; basis: ModelUpBasis; weightLoads: number }>;

/** The self-weight generator's evidence method of a member's distributed self-weight
 * (`GENERATED_METHOD` in `core/product_physics/src/self_weight.rs`). */
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

function isWeightLoad(load: PlainObject): boolean {
  if (load.category === "weight") return true;
  return load.category === "distributed_force" && provenanceMethod(load.provenance) === SELF_WEIGHT_MEMBER_METHOD;
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

/** The model's up direction by the desktop's rule, with which case holds and how many weight loads
 * were read. */
export function deriveModelUp(model: unknown): ModelUp {
  const directions = new Set<string>();
  let read = 0;
  let offAxis = false;
  const cases = isPlainObject(model) && Array.isArray(model.load_cases) ? model.load_cases : [];
  for (const loadCase of cases) {
    const loads = isPlainObject(loadCase) && Array.isArray(loadCase.primitive_loads) ? loadCase.primitive_loads : [];
    for (const load of loads) {
      if (!isPlainObject(load) || !isWeightLoad(load) || isMoment(load)) continue;
      const magnitude = isPlainObject(load.magnitude) ? load.magnitude.value : undefined;
      if (typeof magnitude !== "number" || !Number.isFinite(magnitude) || magnitude === 0) continue;
      read += 1;
      const axis = forceAxis(load.direction);
      if (!axis) { offAxis = true; continue; }
      // Up is opposite the direction the weight acts in.
      directions.add(`${magnitude < 0 ? "+" : "-"}${axis}`);
    }
  }
  if (read === 0) return { up: "+y", basis: "no_weight_load", weightLoads: 0 };
  if (offAxis || directions.size !== 1) return { up: "+y", basis: "weight_loads_disagree", weightLoads: read };
  return { up: [...directions][0] as SignedAxis, basis: "weight_loads", weightLoads: read };
}

/** The signed axis as shown: `+Z`, `−Y`. */
export function signedAxisText(up: SignedAxis): string {
  return `${up[0] === "-" ? "−" : "+"}${up[1].toUpperCase()}`;
}

/** The readout of the derived up direction, in words. */
export function modelUpReadout(derived: ModelUp): string {
  if (derived.basis === "weight_loads") {
    return `Up ${signedAxisText(derived.up)}, opposite the model's ${derived.weightLoads} weight load${derived.weightLoads === 1 ? "" : "s"}.`;
  }
  if (derived.basis === "no_weight_load") return "Up +Y: the model has no weight load to take it from.";
  return "Up +Y: the model's weight loads do not act along one direction.";
}
