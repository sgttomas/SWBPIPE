/** W-11 (DEL-08-01 Design `exact-route-report-sections.md` §2.1 and §2.3): the
 * closed statement table of the exact-route reports. Each entry is one producer
 * limitation string, named by the SHA-256 of its UTF-8 bytes, with the report's
 * reviewed statement for it. A statement is the producer string verbatim,
 * except the two source paragraphs (S9, S10), whose last sentence is omitted
 * (DEC-107); nothing replaces it, and numerical standing is stated by its own
 * line. Keying by digest keeps the omitted sentence out of desktop product
 * source; the producer text lives in the producer
 * (`core/product_physics`) and in the coverage fixture
 * (`fixtures/reports/exact_route_limitations.json`).
 *
 * The table fails closed: a producer string it lacks refuses every report
 * surface (`REPORT-EXACT-LIMITATION-UNREGISTERED`). Its test checks it against
 * the coverage fixture and writes it as `fixtures/reports/exact_report_statements.json`,
 * the table's only Rust form (`core/reporting/report_renderer/src/exact_statements.rs`).
 * A producer text change therefore needs a reviewed table change before it
 * reaches a report on either side. Each statement is one string literal, so
 * the claims-language scan (GEN-13) reads it whole. Standard claim fence
 * applies (claims taxonomy per DEC-081). */

export type ExactReportStatement = Readonly<{ producer_sha256: string; statement: string }>;

export const REPORT_EXACT_LIMITATION_UNREGISTERED = "REPORT-EXACT-LIMITATION-UNREGISTERED";

function entry(producer_sha256: string, statement: string): ExactReportStatement {
  return Object.freeze({ producer_sha256, statement });
}

/** The 19 entries, in the producer's composition order (§1.3). */
export const EXACT_REPORT_STATEMENTS: readonly ExactReportStatement[] = Object.freeze([
  // S1 identity, pressure-2
  entry("3e0ebf8f9dcac12991914d2f5676320259d8bd1368ba24e0b36ec03f43158cb8",
    "3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state; valve, flange, rigid and specialty components are admitted as their mapped straight member with the user's DEC-045 stiffness scale, an equivalent pipe of the mapped section whose stress rows are those of that reference section (marked component_body), not a body stress, and reducers only when their mapped member lies outside every pressure region (EXACT_PRESSURE_REDUCER_IN_REGION otherwise); a branch connection (tee) is a record at its node, whose members meet as a rigid joint with no branch flexibility; a tee consumed as mechanics_geometry_only gives its user-entered header and branch SIFs as intensified rows at the ends of its named straight members on each section basis, not a code stress, and a realized arc it names carries none. Pressure regions meeting at a node where every terminal transfers to the wall form a junction, whose caps sum to the fitting's pressure load. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component or support family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."),
  // S2 identity, pressure-source-1
  entry("1c8b1ce68cbc56a4f354ad472445d24975505da078096f73ac70644abe9ee209",
    "3.0.0/exact_pressure_v3 under pressure-source-1 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; an expansion joint authored as an objective connector is admitted only as the connector limitation states. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases. Every other component or support family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."),
  // S3 pressure
  entry("3e406fe7447bc73a133a81ff45404ebd76e0de07d1f0fc2714bcc95882088814",
    "Pressure is a signed internal differential with zero external pressure increment, uniform within each explicit case-scoped collinear equal-bore region with explicit closure-transfer paths. A negative differential is admitted, but external-pressure stability and collapse are not assessed."),
  // S4 not modelled
  entry("0cd182f3d07b99b573b7ba3b68d9a672d232a0f5c365e2072555a49fe5ce9a22",
    "Not modelled: steady-flow momentum and transient pressure loads; bend opening under pressure (Bourdon effect); pressure stiffening of flexibility factors and stress intensification; ovalization."),
  // S5 contribution ledgers
  entry("a61ae24cdd3c5325cf87fb20b02f682a91bbc0f57e8e5aeed0559f8f6bb862d9",
    "Mechanical, pressure-eigen and cap-transfer contributions remain distinct; rounded cap/eigen ledgers are observational and do not replace source-grouped pressure assembly."),
  // S6 line loads
  entry("daa48a4c191547020a5894f475950a2b0bdc34f551d95659aae5cce72ee6871d",
    "Structural line loads act as entered; contents density does not imply hydrostatic pressure head or a coupled static-fluid pressure/weight state."),
  // S7 support attribution
  entry("d47820118782fbf0e16a105f01fa8cdc1506ee99cc3ec7e251c5e936598ee5fc",
    "Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident rigid attribution is refused."),
  // S8 normal-stress maximum enclosure
  entry("f324475fea7267ad3c04b1f0d697d82e81c87f2dadeca853fa4e271e42795821",
    "The circular normal-stress maximum bounds the supplied binary64 polynomial coefficient enclosures; solution error is separate, and incomplete coverage withholds the headline."),
  // S9 source paragraph, pressure-2
  entry("fada4069d170093e9e74ef785926ecb362dbccf94104e6b93fa7f602a31aed72",
    "Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name."),
  // S10 source paragraph, pressure-source-1
  entry("4fc7d74460d35680e520131b2cffaa9f957d05231223b607c387b86a402b21a4",
    "Every member section declares its corrosion allowance and the contract declares its shear model. This pressure-source-1 envelope carries a selected retained-source response. The source family is straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. Other cases keep their ordinary recovery and standing."),
  // S11 load/reference states 1
  entry("c7d91ccf111fc974b8574f9de46442623780eee98c725dea3efcf68900e03204",
    "One resolved case per load case supplies each member's selected E/nu with derived G, actual/selected/installation temperatures and explicit expansion definition; each case is solved independently."),
  // S12 load/reference states 2
  entry("8f837617c5259422681368fdf8c0b62bd9fd88359e3747f766679fb25ffbeb59",
    "Thermal and fit reference strain compose as lambda_fit*lambda_thermal-1 and enter once as an axial eigenstrain; uniform member temperature only, no gradients, finite strain, inelasticity or fit-up joints."),
  // S13 load/reference states 3
  entry("62df2a3b0908cfbe3d067ff0e523fce53713a4f4b31474d41af684113fa7cd05",
    "Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or locked supports are not provided."),
  // S14 load/reference states 4
  entry("5d6b888ac33f7b120a90396e60ba289c22abc801678bc307aac2f8afaf3051a9",
    "Ordinary applied loads are exactly the case's declared source ledger with explicit factors; unreferenced stored primitives are excluded. Hydrostatic head, contents-weight state and per-case mass selection are not provided. Generated equivalent-static (seismic or wind) loads have no ledger class and are refused by name (EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED)."),
  // S15 load/reference states 5
  entry("d67bef81dd0df58e72fe799467f916ff5c1d28c3fabadb64450d799fedc0bfa4",
    "History is independent equilibrium only; no installation, contact, friction or predecessor history is represented."),
  // S16 objective connector
  entry("9a039e682885cdd1e6d14633173960dfd6a466df3abd6722cd3ecf959556ee0d",
    "Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware or explicit bilateral linear elastic tie rods (explicit_axial_rods_v1) between the connector's end nodes with rigid offsets, each with its own stiffness and reference extension. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces and joints at region terminals or beside realized arcs are refused. Ideal inextensible rods, collective constraints, tension-only or gapped rods, nuts and stops are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with each tie-rod force, the joint pressure force pAe and the effective axial force S = g_x + sum T_r (n_r.Q_x) - Pe published separately; they are outside the straight-member recovery coverage."),
  // S17 realized bend
  entry("e3cdc39260cf67ee730a608000cd3b182e65167c197095ff6dbceb49f1da3023",
    "Realized curved bends (curved_bend_macro_element with the user's bend radius and flexibility factor k, one k for both bending planes) are admitted under pressure: each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc) and c_b its own end caps along the end tangents; on an arc N_w = N_el + pAi and S = N_el, and shear and moments are elastic. A region changes direction only at realized bends: a bend-end direction change up to alpha_tan = 1e-3 rad is carried exactly by the remainder pAi(t_in - t_out); a larger change (a mitre) and a straight-straight kink are refused. Geometry-only (chord) bends carry no pressure and are refused. On arcs the hoop is the toroidal membrane cut average (bore_load_transferred_toroidal_membrane_v1) and the radial values are -p at the bore and 0 outside, the transverse shear is the straight-annulus Saint-Venant field driven by the tangent-frame shears (local_straight_annulus_on_arc), and the normal-stress maximum is bounded along the arc; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects and the curvature correction of the shear field are not represented; the excluded effects above apply to every bend result."),
  // S18 certified recovery 1
  entry("47a8910e8e8dcb0f60c3d3fe9ca400c91822859293c8331b221d2e913c586f84",
    "Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the solve's own displacement, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets)."),
  // S19 certified recovery 2
  entry("eb043ebf9b3c8186a16574237e69ec69b977bb17c4546ef434c9dfe0b55ccfce",
    "The recovery is checked against the formation check's re-formed system (stiffness and loads formed at 128-bit precision from the binary64 inputs). Wherever the formation check produced its member bound for a load case, including a case that the member recovery check then reports as sensitive, each published nodal displacement component is the corrected displacement (the solve's displacement plus the check's two correction terms, summed exactly) rounded once to binary64 in SI units and then converted to the published unit as every other row is, and each displacement magnitude is the correctly rounded norm of the node's three published translation components; where there is no such bound, or where any component of the case cannot be formed, every published displacement of the case is the solve's own, and a selected retained source publishes its recovery's nodal values. Support reactions balance the re-formed loads, not the solve's binary64 load vector."),
]);

const BY_DIGEST: ReadonlyMap<string, string> = new Map(
  EXACT_REPORT_STATEMENTS.map((item) => [item.producer_sha256, item.statement])
);

/** The reviewed statement for the producer string with this SHA-256 (64
 * lowercase hex characters), or null when the table has none. */
export function exactReportStatementFor(producerSha256: string): string | null {
  return BY_DIGEST.get(producerSha256) ?? null;
}

/** The SHA-256 of a string's UTF-8 bytes, as 64 lowercase hex characters. */
export async function producerStringSha256(text: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** The report statements for an exact identity's published limitations, one
 * per producer string, in the producer's order. A string the table lacks
 * refuses with `REPORT-EXACT-LIMITATION-UNREGISTERED`; nothing is copied from
 * the producer and no suffix is stripped. */
export async function exactReportStatements(identity: string, limitations: readonly string[]): Promise<string[]> {
  const statements: string[] = [];
  for (const text of limitations) {
    const statement = typeof text === "string" ? exactReportStatementFor(await producerStringSha256(text)) : null;
    if (statement === null) {
      throw new Error(`${REPORT_EXACT_LIMITATION_UNREGISTERED}: ${identity} publishes a limitation this report has no reviewed statement for.`);
    }
    statements.push(statement);
  }
  return statements;
}
