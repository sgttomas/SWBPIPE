# B1 source, operand and row inventory

Source pin: main 49034a940f3f8cd3f3da4d4cbc839943b808063d. P =
projects/chirality-piping; paths below are P-relative unless marked T3.
These are source inspections, not production executions. RETURN.md names the
accepted premises and open T-A/T-Z/T-G cells. “Derived” describes origin;
“exact K primitive” describes the later admitted mechanics boundary.

## Operand truth and actual operation table

| Operand/boundary | Actual source and operation | Certificate treatment |
|---|---|---|
| Authored raw/model/case | product_physics/src/lib.rs:2221–2299 chooses request-level materials when supplied, validates and normalizes; :8243–8278 calls the units engine; source_receipt.rs is the existing captured-entry custody seam | Keep actual capture and model/case/basis association. A parsed or reserialized reconstruction is not raw custody. Newly normalized bits define a new admitted source if they change. |
| Node coordinates, y_reference | FK retained/source.rs:1–21,90–104,166–177; PP lib.rs:6554–6583 constructs the actual straight element with nodes and y_reference | Exact admitted bits for K. Product node order and kernel canonical member ids require explicit checked maps. No rounded transform or element matrix is a primitive. |
| E/G and basis | PP lib.rs:2388–2523 retains basis-specific material/build/stiffness; :6537–6553 takes actual resolved member pair or material E/G; :9210–9238 interpolates E/G/alpha with binary64 subtraction/division/multiply/add | Exact admitted E/G bits for K, with actual material/selected-basis provenance. No base-G substitution. Same material id does not prove same selected bits. |
| Effective wall and OD | PP lib.rs:9436–9471 uses normalized OD and nominal wall; present mill tolerance gives RN(t_nom−m) | These computed bits exist; they are not proof of an unrounded authored geometric difference. Do not reinterpret the adopted source geometry without its warrant. |
| Legacy A,I,J,Z,c | PP lib.rs:9473–9487: d_i=RN(OD−2t); A=PI·(OD²−d_i²)/4; I=PI·(OD⁴−d_i⁴)/64; J=2I; Z=I/(OD/2); c=OD/2, all through actual binary64 expressions | Final A/I/J become exact admitted K primitives under D1. Z/c remain derived stress operands requiring the T-Z/T-G declaration/enclosure. Product PI constant and powi results are not real-π/source-geometry exactness proofs. |
| Exact-mode A,I,J,Z | PP lib.rs:6507–6525 replaces DerivedSection fields with SourceAnnulus accessors; pressure_exact/source_geometry.rs:28–68 forms area, I,J,Z through Scaled; annulus_geometry.rs:33–45 uses PI*t*(OD−t) and PI*(OD/2−t)^2 | Scaled has f64 mantissa and finite exponent, with rounded +,* and / (pressure_exact.rs:13–141). Final accessor bits are not outward enclosures. Z is formed from the Scaled I before its accessor conversion, so do not assume Z_hat=RN(I_hat/c_hat). |
| Receipt section evidence | PP lib.rs:9576–9580 records geometry_basis=authored_normalized_od_wall_v1 and OD/t/ri/ro/Ai/As/I/J/Z; T3 D1 DESIGN §4.1.6.1 item 7, D2 §4.9.3 | Actual A_hat/Z_hat/L_hat/k_a_hat/k_t_hat are bit-bound operational scale inputs. This is distinct from zero uncertainty about ideal geometric denominators. |
| Physical member length | straight_pipe/src/lib.rs:445–446 calls frame_element.length; FK src/lib.rs:586–590 subtracts binary64 coordinates then norm; intended K frame/length is D1 §4.1.2 exact-coordinate limit | L_hat is derived. Its operational scale bits are pinned, but no L_hat=L* theorem is assumed. B1 uses certified station rows, so does not need to rebuild truth with L_hat. Physical-length/span proof belongs to B2. |
| Body extent | FK retained/adaptive.rs:309–340 gives prescribed binary64 extent and original-operand coupling | Operational scale, same adopted bits in producer/readers. Not a physical-geometry error bound. |
| Kernel x and r_x | FK retained/adaptive.rs:3303–3380 builds prescribed-replaced final publication/certificate; :3420–3455 owns publication and private radius; :3460–3542 validates radius association | q* is the admitted K row; x actual final publication; r_x finite upward A1 radius. Stored absent sentinel remains distinct from exact zero. Tighten only with simultaneous certified bounds of this same row. |
| A/Z stress denominators | loads/stress_recovery/src/lib.rs:493–510 and :877–897; PP lib.rs:11203–11231 supplies actual DerivedSection values | Producer performs numerator/denominator. B1 truth uses the corresponding justified positive A/Z enclosure; do not make a singleton because the producer accepts f64. |
| Torsion c,J | loads/stress_recovery/src/lib.rs:899–926; PP lib.rs:11219–11220 supplies actual J,c | Producer evaluates torque*radius/J in that order. B1 encloses that specified truth with eight corners. No T/(2Z_hat) rewrite. |
| Pressureless membrane caveat | pressure_exact/source_geometry.rs:151–184 uses its Scaled wall area in recover_wall_effective_membrane; PP lib.rs:11053–11191 replaces rows when a pressure pipe state exists | Its denominator is not automatically A_hat. W1a exact route requires an explicitly empty pressure-region list (D1 DESIGN:581); pressure_runtime.rs:449 begins empty and states are inserted for regions (:651,:732). Thus this pressure-state branch has no reach in that admitted exact W1a route. Do not invent its rows. If a later admitted route emits membrane, bind its actual truth/denominator before applying the N/A theorem. |

## Actual row-to-kernel/recipe map to freeze before implementation

| Final product family | Actual present publication seam | Required W1 B1 association and SI truth |
|---|---|---|
| global_nodal_displacement_x/y/z | PP lib.rs:10797–10880, displacement[6node+component]·1000, mm | Displacement(Dof), same actual node/global component, x in m. Restrained/prescribed classification from actual source DOF map; InputDerived handled separately. |
| global_nodal_rotation_x/y/z | Same function, ·1, rad | Same Displacement(Dof) identity with rotation component, x in rad. |
| displacement_magnitude | PP lib.rs:3980–4017, current ordinary/source-specific norm and mm output | W1 must use its own DisplacementMagnitude(node) certificate. Current source-block norm is another route and not W1's certificate. No recomputation from rounded components as W1 truth. |
| element_local axial/shear/end moments | PP lib.rs:4328–4331 appends raw end actions, node-on-element convention | EndAction(member,I/J,component), exact same local frame/source. Raw i-end action has its own sign; do not call it the j-side section cut. |
| same action kinds at quarter/mid/three-quarter stations | PP lib.rs:4373–4391 and :10881–10951; N or N*m | StationAction(station,component) at exactly 1/4,1/2,3/4 on mapped member. FK recover.rs:10–14 defines j-side action; t=0 is minus i-end, t=1 equals j-end. No index-only matching. |
| endpoint stress action operands | PP lib.rs:4392 onward obtains endpoint section cuts; :11203–11231 maps N=r[0], My=r[4], Mz=r[5], T=r[3] | At end i use negative EndAction, at end j positive, or matching endpoint StationAction if actually included. Bind the exact same member/frame/functional, not a rounded statics surrogate. |
| element_local_axial_normal_stress | PP lib.rs:12183–12242 and :12379–12431 emits RN(stress_Pa/1e6), MPa; stress_recovery:493 | Signed section N interval divided by warranted A interval. Raw y is actual MPa and final n=RN(y*1e6). |
| element_local_bending_normal_stress_y/z | Same endpoint/station emission; stress_recovery:500–512 | Signed mapped M_y/M_z interval divided by corresponding Z interval. Equal source Z values do not authorize swapping moments or cases. |
| element_local_torsional_shear_stress | Same emission, actual stress_recovery:899–926 | Signed mapped T interval times c interval divided by J interval. Both producer roundings and output conversion are included by distance to final y/n. |
| support_reaction_component_v2 | PP lib.rs:4033–4130 forms attributed support vector, preview_physics.rs:528–565 later replaces support rows | Reaction(Dof), SpringAction(spring,component), or a justified exact identified support-law sum. Support id/node, restraint/spring list and ambiguity disposition must agree. W1 cannot reuse the ordinary −k*u rounded-displacement calculation as a certificate. |
| reaction_resultant / support force or moment magnitude | PP lib.rs:4118 onward and preview replacement | Kernel SupportForceMagnitude(group) / SupportMomentMagnitude(group), with exactly the same support group. No norm of rounded components. Directional springs are kernel-only and not a W1a product source. |
| pipe_wall_endpoint_action_v2 | PP lib.rs:11128–11135 (pressure-state branch), node-on-element | If reached in an admitted no-pressure source, exact same signed EndAction; actual W1a empty-region route must not be said to emit this branch. |
| pipe_wall_axial_force_v2 / pipe_effective_axial_force_v2 | PP lib.rs:11169–11186, j-side section Nw and S | At genuinely zero pressure and an admitted row path, Nw=S=N_section*; prove actual path/source equality before direct mapping. Zero net from an excluded producer does not make it eligible. |
| pipe_axial_membrane_stress_v2 | PP lib.rs:11188 (pressure-state branch) | Conditional Nw/A theorem only after actual denominator warrant; current Scaled denominator noted above. No actual W1a reach claimed. |
| Other InputDerived rows | D1 closed kind table and actual restraint/hanger rules | Bind to actual source facts; no radius synthesized. Unsupported units/kinds retain adopted NotCovered classification. Failed covered recipes instead refuse. |

FK recover.rs:55–198 enumerates the exact QuantityId/QuantityMeta and canonical
layout; source constructor sorts ids, so product iteration position is not a
binding. PP lib.rs:1048–1066 offers length-delimited identities, while many old
result ids use stable_suffix. A concrete map must check uniqueness/collisions
after final qualification rather than use lossy suffixes as evidence of identity.

PP lib.rs:2544–2559 renders preview and chooses summaries; :2588–2625 qualifies
row ids/basis_refs/source refs; preview_physics.rs:528 onward replaces supports
and maxima, and its combination rendering is later. The B1 verdict applies only
to rows after those changes. A headline is a reference to the final row; B1 does
not independently certify a maximum claim.

## Deferred formulas (inventory only)

- preview_physics.rs:628–655 emits actual i*(hypot(My,Mz)/Z) in Pa with a
  component/member/end association. SIF/hypot are B2, not B1.
- PP lib.rs:9583–9665 creates rounded span length, section resultants and
  Bernstein controls; :4655 onward publishes a midpoint from its maximum
  enclosure, whose metadata limits it to supplied binary64 coefficients.
- PP lib.rs:11233–11255 forms the open MPa summary. Its maximum/sum proof is
  B2. B1 component certificates alone are not a certificate of that summary.

