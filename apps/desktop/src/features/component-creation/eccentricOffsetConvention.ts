/**
 * W-15 slice c, R-c1 (DEL-03-05 Design `eccentric-reducer-offset.md` §3.6): the one
 * place the desktop states the eccentric reducer offset's convention and the
 * reference-section meaning of the reducer body's stress rows. The convention is
 * the owner's ruling of 2026-10-26, "R-c1 A, 0.8.0 a2": from the centerline at the
 * component's node to the centerline at the body's other end, in global X/Y/Z;
 * product physics reads it only in `rigid_component::eccentric_offset_ends`.
 */
export const ECCENTRIC_OFFSET_CONVENTION =
  "user-entered vector in global X/Y/Z from the centerline at this component's node to the centerline at the other end of its body (x, y, z); the node geometry governs, and the solve checks the offset against it";

/** P11: the reducer body's stress rows, scaled or not. */
export const ECCENTRIC_REDUCER_STRESS_BASIS =
  "the reducer body's stress rows are those of the mapped pipe section along the chord between the two centerlines, a reference section and not a stress of the reducer body, whether or not a stiffness scale is applied";
