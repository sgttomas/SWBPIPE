//! T4-U4b (DEL-04-07 Design `u4b-u5b-implementation.md` §5.1): the reader
//! checks of the new kinds on the two ordinary routes, shared by the
//! pressure-2 reader (`physics_evidence`) and the preview-physics-2 reader
//! (`preview_physics_evidence`).
//!
//! - `element_local_normal_stress_extreme` (σ⁺/σ⁻): recomputed bit for bit
//!   from the same station's and basis's published stress rows, by §2.3's
//!   fixed sequence (the membrane row /1e6 on a pressurized pressure-2
//!   member, the shared correctly rounded `norm2`); present exactly where its
//!   operand rows are.
//! - `element_local_transverse_shear_stress_maximum` (τ): custody, coverage
//!   (every member, station and basis; on preview-physics-2 but the members
//!   named by `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`, which pressure-2
//!   refuses), non-negativity and the unit; bit for bit from the station's
//!   shear rows when the caller supplies c_b (the exact reader from its
//!   evidence, the preview reader from the model, which also binds the
//!   warning to the member's ν).
//! - directional intensified rows: bound to their `intensified_measures`
//!   records by key set, recomputed bit for bit from the station's bending
//!   rows and the record's factors; the along-arc maximum's record follows
//!   the arc maximum's checks, covered by the station intensified rows.
//! - toroidal hoop and radial rows (pressure-2): present exactly on
//!   pressurized arcs with R > r_o, recomputed bit-equal by the normative
//!   sequence from p, the basis bore, wall, r_o and R.
//! - the arc maximum's closed record, accepted only on realized arcs, with
//!   the station cover.
//!
//! These are internal-consistency checks, not accuracy claims. The readers
//! attest the arc maxima's custody and do not replay the branch and bound.
use open_pipe_stress_correct_norm::norm2;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const EXTREME: &str = "element_local_normal_stress_extreme";
pub(crate) const SHEAR_MAX: &str = "element_local_transverse_shear_stress_maximum";
pub(crate) const HOOP: &str = "pipe_toroidal_membrane_hoop_stress_v1";
pub(crate) const RADIAL: &str = "pipe_toroidal_radial_surface_stress_v1";
pub(crate) const DIRECTIONAL: &str = "curved_bend_directional_intensified_bending_stress_v1";
pub(crate) const DIRECTIONAL_MAX: &str = "curved_bend_directional_intensified_bending_maximum_v1";
pub(crate) const KINDS: [&str; 6] = [EXTREME, SHEAR_MAX, HOOP, RADIAL, DIRECTIONAL, DIRECTIONAL_MAX];

pub(crate) const EXTREMES_BASIS: &str = "section_extremes_from_published_stress_rows_v1";
pub(crate) const SHEAR_STRAIGHT_BASIS: &str = "saint_venant_annulus_flexure_v1";
pub(crate) const SHEAR_ARC_BASIS: &str = "local_straight_annulus_on_arc";
pub(crate) const HOOP_BASIS: &str = "bore_load_transferred_toroidal_membrane_v1";
pub(crate) const DIRECTIONAL_BASIS: &str = "user_directional_sif_times_tangent_frame_bending_stress_v1";
/// The preview arc marker: every arc component stress row's token.
pub(crate) const PREVIEW_ARC_BASIS: &str = "nominal_straight_beam_formula_on_arc_resultants";

pub(crate) const SIGMA_PLUS_SIGN: &str = "signed section normal-stress extreme sigma+ = N/A + hypot(My,Mz)/Z at the station (N_w/A_s on a pressurized pressure-2 member), formed from the published axial or membrane and bending stress rows of the same station and basis; at the outer-surface fibre r_o(-sigma_z, sigma_y)/hypot(sigma_y, sigma_z) of the section frame, every outer fibre tied when the bending rows are zero; not a code or equivalent stress";
pub(crate) const SIGMA_MINUS_SIGN: &str = "signed section normal-stress extreme sigma- = N/A - hypot(My,Mz)/Z at the station (N_w/A_s on a pressurized pressure-2 member), formed from the published axial or membrane and bending stress rows of the same station and basis; at the outer-surface fibre -r_o(-sigma_z, sigma_y)/hypot(sigma_y, sigma_z) of the section frame, every outer fibre tied when the bending rows are zero; not a code or equivalent stress";
pub(crate) const SHEAR_SIGN_EXACT: &str = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the basis annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu is the member's authored nu; torsional shear separate";
pub(crate) const SHEAR_SIGN_PREVIEW: &str = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the section annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu = E/(2G) - 1 from the moduli the solve uses, assuming an isotropic material; torsional shear separate";
pub(crate) const SHEAR_ARC_SUFFIX: &str = "; on an arc the straight-annulus field is driven by the tangent-frame shears without curvature correction";
pub(crate) const HOOP_SIGN: &str = "tension-positive toroidal membrane hoop: the area average over the wall cut at the named section angle (intrados beta = 0, crown 90 deg, extrados 180 deg) of a complete torus in an axisymmetric membrane state under p on the basis bore; station-independent";
pub(crate) const RADIAL_SIGN: &str = "tension-positive radial stress of the torus section: -p at the basis bore and 0 at the outer surface";
pub(crate) const DIRECTIONAL_SIGN: &str = "nonnegative hypot(i_ip*Mz, i_op*My)/Z in the arc tangent frame with the user-entered directional SIFs of its intensified_measures record (i_ip on the in-plane Mz, i_op on the out-of-plane My); not a code stress; no flexibility factor, axial or torsion term; never combined";
pub(crate) const ARC_MAXIMUM_SIGN: &str = "nonnegative maximum |Nw/As|+hypot(My,Mz)/Z along the arc, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; torsional shear remains separate; no code stress or equivalent stress claim";
pub(crate) const DIRECTIONAL_MAX_SIGN: &str = "nonnegative along-arc maximum of hypot(i_ip*Mz, i_op*My)/Z with the user-entered directional SIFs of its intensified_measures record, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; never combined";

pub(crate) const ARC_APPROXIMATION: &str = "arc_closed_form_section_statics";
pub(crate) const ARC_DIRECTIONAL_APPROXIMATION: &str = "arc_closed_form_directional_intensified";
pub(crate) const ARC_COEFFICIENT_BASIS: &str = "arc_end_j_force_and_uniform_intensities_binary64_enclosures";
pub(crate) const ARC_ENCLOSURE_SCOPE: &str = "published_binary64_end_j_force_and_uniform_intensity_operand_enclosures; solution error is separate";
pub(crate) const POISSON_OUT_OF_DOMAIN: &str = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN";
pub(crate) const TOROIDAL_DOMAIN: &str = "ARC_TOROIDAL_HOOP_DOMAIN";

/// The arc maximum record's closed thirteen keys (the straight record's; the
/// pressure-2 record adds `section_basis`).
pub(crate) const RECORD_KEYS: [&str; 13] = [
    "pipe_id", "result_id", "station_fraction", "span_index", "local_fraction", "value_lower_pa",
    "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions", "approximation",
    "coefficient_basis", "enclosure_scope",
];
/// The directional station record's keys (pressure-2 adds `section_basis`).
pub(crate) const STATION_RECORD_KEYS: [&str; 5] = ["pipe_id", "result_id", "location", "i_in_plane", "i_out_of_plane"];

pub(crate) const STATIONS: [&str; 5] = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];

/// The route the checks read: its error prefix, whether rows name a section
/// basis, and the route's τ sign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Route {
    Exact,
    Preview,
}
impl Route {
    fn fail(self, code: &str) -> String {
        match self {
            Route::Exact => format!("SOURCE_PHYSICS_{code}"),
            Route::Preview => format!("SOURCE_PREVIEW_PHYSICS_{code}"),
        }
    }
    fn require(self, ok: bool, code: &str) -> Check {
        if ok { Ok(()) } else { Err(self.fail(code)) }
    }
    fn shear_sign(self, arc: bool) -> String {
        format!("{}{}", if self == Route::Exact { SHEAR_SIGN_EXACT } else { SHEAR_SIGN_PREVIEW }, if arc { SHEAR_ARC_SUFFIX } else { "" })
    }
}

/// Pa rounded upward from a published MPa value (the station cover's
/// conversion, as the producer forms it).
pub(crate) fn pa_upward(mpa: f64) -> f64 {
    let pa = mpa * 1e6;
    if pa == 0.0 { 0.0 } else { pa.next_up() }
}

/// c_b = RN(num/den) of the Saint-Venant maximum (§2.4's fixed sequence), or
/// `None` outside -1 < ν < 1/2.
pub(crate) fn shear_factor(nu: f64, ro: f64, ri: f64, inertia: f64) -> Option<f64> {
    if !(nu.is_finite() && nu > -1.0 && nu < 0.5 && ri > 0.0 && ri < ro && inertia > 0.0) {
        return None;
    }
    let num = (3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri);
    let den = (4.0 * (1.0 + nu)) * inertia;
    Some(num / den).filter(|c| c.is_finite() && *c > 0.0)
}

/// c_b on preview-physics-2 from the moduli (§2.4): the ν sequence of
/// [`shear_factor`] with ν = RN(RN(E/RN(2·G)) − 1) while RN(E/RN(2·G)) >= 1/2,
/// where its 1+ν is exact; below, with no 1+ν formed, x = RN(RN(2·G)/E) and
/// c_b = RN(RN(RN(2·RN(RN(r_o·r_o) + RN(r_i·r_i))) + RN(x·RN(RN(r_o − r_i)·RN(r_o + r_i))))/RN(4·I)).
/// `None` outside -1 < ν < 1/2.
pub(crate) fn preview_shear_factor(e: f64, g: f64, ro: f64, ri: f64, inertia: f64) -> Option<f64> {
    let ratio = e / (2.0 * g);
    if !(ratio < 0.5) {
        return shear_factor(ratio - 1.0, ro, ri, inertia);
    }
    if !(ratio - 1.0 > -1.0 && e > 0.0 && g > 0.0 && ri > 0.0 && ri < ro && inertia > 0.0) {
        return None;
    }
    let x = (2.0 * g) / e;
    let num = 2.0 * (ro * ro + ri * ri) + x * ((ro - ri) * (ro + ri));
    Some(num / (4.0 * inertia)).filter(|c| c.is_finite() && *c > 0.0)
}

/// The toroidal hoop (intrados, crown, extrados) by the normative sequence,
/// `None` unless R > r_o.
pub(crate) fn toroidal_hoop(p: f64, ri: f64, wall: f64, ro: f64, r: f64) -> Option<[f64; 3]> {
    if !(r > ro && ri > 0.0 && wall > 0.0 && ro > 0.0) {
        return None;
    }
    let a = p * ri / wall;
    let two_r = 2.0 * r;
    let sum = ri + ro;
    Some([a * ((two_r - ri) / (two_r - sum)), a, a * ((two_r + ri) / (two_r + sum))])
}

/// One case row's slot.
type Slot<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

/// The case rows of one envelope by (case, entity, kind, component, location,
/// basis); the basis is "" for a row without a section basis.
pub(crate) struct Index<'a> {
    rows: HashMap<Slot<'a>, &'a Value>,
}
fn case_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "load_case").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
fn basis_of(row: &Value) -> &str {
    row["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map_or("", |(_, b)| b)
}
fn token_of(row: &Value) -> &str {
    let b = row["metadata"]["basis"].as_str().unwrap_or("");
    b.rsplit_once("; section_basis=").map_or(b, |(t, _)| t)
}
impl<'a> Index<'a> {
    pub(crate) fn new(rows: &'a [Value], route: Route) -> Result<Self, String> {
        let mut map = HashMap::new();
        for row in rows {
            let Some(case) = case_of(row) else { continue };
            let md = &row["metadata"];
            let (Some(entity), Some(kind)) = (row["entity_ref"].as_str(), row["kind"].as_str()) else { continue };
            let slot = (case, entity, kind, md["component"].as_str().unwrap_or(""), md["location"].as_str().unwrap_or(""), basis_of(row));
            if map.insert(slot, row).is_some() && KINDS.contains(&kind) {
                return Err(route.fail("ROW_SLOT_DUPLICATE"));
            }
        }
        Ok(Self { rows: map })
    }
    fn value(&self, case: &str, entity: &str, kind: &str, component: &str, location: &str, basis: &str) -> Option<f64> {
        self.rows.get(&(case, entity, kind, component, location, basis)).and_then(|r| r["value"].as_f64())
    }
    fn any(&self, case: &str, entity: &str, kind: &str, location: &str, basis: &str) -> Option<f64> {
        self.rows.iter().find(|(s, _)| s.0 == case && s.1 == entity && s.2 == kind && s.4 == location && s.5 == basis).and_then(|(_, r)| r["value"].as_f64())
    }
}

/// What a reader states about one member of one case.
pub(crate) struct MemberFacts<'a> {
    pub case: &'a str,
    pub pipe: &'a str,
    /// c_b per basis when the reader can form it (exact: evidence ν and
    /// sections; preview: the captured model's moduli), else `None`
    /// (model-free checks).
    pub shear_factors: Vec<(&'a str, Option<f64>)>,
    /// The member is named by `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`.
    pub poisson_out_of_domain: bool,
    /// (p, ri, wall, ro, R) per basis of a pressurized pressure-2 arc.
    pub toroidal: Vec<(&'a str, (f64, f64, f64, f64, f64))>,
    /// On pressure-2 the bases; [""] on preview.
    pub bases: Vec<&'a str>,
}

/// §5.1's station checks of one member: σ±, τ, hoop and radial. Returns the
/// station cover (Pa, rounded upward) of each basis for the arc maximum.
pub(crate) fn member_stations(index: &Index<'_>, route: Route, m: &MemberFacts<'_>) -> Result<HashMap<String, Vec<f64>>, String> {
    let mut cover: HashMap<String, Vec<f64>> = HashMap::new();
    for basis in &m.bases {
        let shear = m.shear_factors.iter().find(|(b, _)| b == basis).and_then(|(_, f)| *f);
        let toroidal = m.toroidal.iter().find(|(b, _)| b == basis).map(|(_, t)| *t);
        for location in STATIONS {
            let y = index.value(m.case, m.pipe, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis);
            let z = index.value(m.case, m.pipe, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis);
            let n = index.value(m.case, m.pipe, "element_local_axial_normal_stress", "axial_normal_stress", location, basis)
                .or_else(|| index.value(m.case, m.pipe, "pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, basis).map(|v| v / 1e6));
            let plus = index.value(m.case, m.pipe, EXTREME, "sigma_plus", location, basis);
            let minus = index.value(m.case, m.pipe, EXTREME, "sigma_minus", location, basis);
            match (n, y, z) {
                (Some(n), Some(y), Some(z)) => {
                    let h = norm2(y, z);
                    let (p, q) = (plus.ok_or_else(|| route.fail("EXTREME_COVERAGE"))?, minus.ok_or_else(|| route.fail("EXTREME_COVERAGE"))?);
                    route.require(p.to_bits() == (n + h).to_bits() && q.to_bits() == (n - h).to_bits(), "EXTREME_VALUE")?;
                    cover.entry(basis.to_string()).or_default().push(pa_upward(p.abs().max(q.abs())));
                }
                _ => route.require(plus.is_none() && minus.is_none(), "EXTREME_COVERAGE")?,
            }
            // τ: the section shear rows inside, the end-force rows at the ends
            // (no section basis on either).
            let vy = index.any(m.case, m.pipe, "element_local_shear_force_y", location, "");
            let vz = index.any(m.case, m.pipe, "element_local_shear_force_z", location, "");
            let tau = index.value(m.case, m.pipe, SHEAR_MAX, "tau_v_max", location, basis);
            if m.poisson_out_of_domain {
                route.require(tau.is_none(), "SHEAR_COVERAGE")?;
            } else if let (Some(vy), Some(vz)) = (vy, vz) {
                let tau = tau.ok_or_else(|| route.fail("SHEAR_COVERAGE"))?;
                route.require(tau >= 0.0, "SHEAR_NEGATIVE")?;
                if let Some(factor) = shear {
                    route.require(tau.to_bits() == (norm2(vy, vz) * factor / 1e6).to_bits(), "SHEAR_VALUE")?;
                }
            } else {
                route.require(tau.is_none(), "SHEAR_COVERAGE")?;
            }
            // Hoop and radial: exactly on a pressurized arc with R > r_o.
            let hoop = [
                index.value(m.case, m.pipe, HOOP, "toroidal_intrados_hoop_stress", location, basis),
                index.value(m.case, m.pipe, HOOP, "toroidal_crown_hoop_stress", location, basis),
                index.value(m.case, m.pipe, HOOP, "toroidal_extrados_hoop_stress", location, basis),
            ];
            let radial = [
                index.value(m.case, m.pipe, RADIAL, "toroidal_inner_radial_stress", location, basis),
                index.value(m.case, m.pipe, RADIAL, "toroidal_outer_radial_stress", location, basis),
            ];
            match toroidal.and_then(|(p, ri, wall, ro, r)| toroidal_hoop(p, ri, wall, ro, r).map(|h| (h, p))) {
                Some((expected, p)) => {
                    for (got, want) in hoop.iter().zip(expected) {
                        route.require(got.is_some_and(|g| g.to_bits() == want.to_bits()), "TOROIDAL_VALUE")?;
                    }
                    route.require(radial[0].is_some_and(|g| g.to_bits() == (-p).to_bits()) && radial[1].is_some_and(|g| g.to_bits() == 0f64.to_bits()), "TOROIDAL_VALUE")?;
                }
                None => route.require(hoop.iter().chain(&radial).all(Option::is_none), "TOROIDAL_COVERAGE")?,
            }
        }
    }
    Ok(cover)
}

/// The declared semantics of one row of a new kind (case scoped, closed
/// metadata); `arc` whether its entity is a realized arc.
pub(crate) fn row_semantics(row: &Value, route: Route, arc: bool) -> Check {
    let kind = row["kind"].as_str().unwrap_or("");
    let md = &row["metadata"];
    let location = md["location"].as_str().unwrap_or("");
    let station = STATIONS.contains(&location);
    let basis = basis_of(row);
    route.require(case_of(row).is_some() && md["coordinate_system"] == "pipe_section", "ROW_SEMANTICS")?;
    route.require(if route == Route::Exact { basis == "nominal_section" || basis == "reduced_section" } else { md["basis"].as_str() == Some(token_of(row)) && basis.is_empty() }, "ROW_SECTION_BASIS")?;
    let (unit, components, token, sign, at_station, arc_only): (&str, &[&str], &str, String, bool, bool) = match kind {
        EXTREME => ("MPa", &["sigma_plus", "sigma_minus"], EXTREMES_BASIS,
            if md["component"] == "sigma_minus" { SIGMA_MINUS_SIGN } else { SIGMA_PLUS_SIGN }.to_string(), true, false),
        SHEAR_MAX => ("MPa", &["tau_v_max"], if arc { SHEAR_ARC_BASIS } else { SHEAR_STRAIGHT_BASIS }, route.shear_sign(arc), true, false),
        HOOP => ("Pa", &["toroidal_intrados_hoop_stress", "toroidal_crown_hoop_stress", "toroidal_extrados_hoop_stress"], HOOP_BASIS, HOOP_SIGN.to_string(), true, true),
        RADIAL => ("Pa", &["toroidal_inner_radial_stress", "toroidal_outer_radial_stress"], HOOP_BASIS, RADIAL_SIGN.to_string(), true, true),
        DIRECTIONAL => ("Pa", &["intensified_bending"], DIRECTIONAL_BASIS, DIRECTIONAL_SIGN.to_string(), true, true),
        DIRECTIONAL_MAX => ("Pa", &["intensified_bending_maximum"], DIRECTIONAL_BASIS, DIRECTIONAL_MAX_SIGN.to_string(), false, true),
        _ => return Ok(()),
    };
    route.require(!(arc_only && !arc), "ARC_ROW_ON_STRAIGHT")?;
    route.require(route == Route::Exact || !matches!(kind, HOOP | RADIAL), "ROW_KIND_UNSUPPORTED")?;
    route.require(
        row["unit"] == unit
            && md["component"].as_str().is_some_and(|c| components.contains(&c))
            && token_of(row) == token
            && md["sign_convention"].as_str() == Some(sign.as_str())
            && if at_station { station } else { location == "governing_station" }
            && row.get("source_result_refs").is_none_or(|r| r.as_array().is_some_and(|a| a.is_empty())),
        "ROW_SEMANTICS",
    )
}

/// A certified maximum record (arc or directional) bound to its row:
/// shape, tokens, gap rule, midpoint, fraction and the station cover.
pub(crate) fn maximum_record(route: Route, record: &Value, row: &Value, approximation: &str, cover: &[f64]) -> Check {
    let number = |k: &str| record[k].as_f64().filter(|v| v.is_finite()).ok_or_else(|| route.fail("NUMBER_INVALID"));
    let lower = number("value_lower_pa")?;
    let upper = number("value_upper_pa")?;
    let global = number("global_upper_bound_pa")?;
    let gap = number("certified_gap_pa")?;
    let value = row["value"].as_f64().ok_or_else(|| route.fail("NUMBER_INVALID"))?;
    route.require(
        record["approximation"] == approximation
            && record["coefficient_basis"] == ARC_COEFFICIENT_BASIS
            && record["enclosure_scope"] == ARC_ENCLOSURE_SCOPE
            && record["span_index"].as_u64() == Some(0)
            && record["subdivisions"].as_u64().is_some_and(|n| n <= 131072),
        "EXTREMA_BASIS",
    )?;
    let fraction = number("station_fraction")?;
    route.require((0.0..=1.0).contains(&fraction) && number("local_fraction")?.to_bits() == fraction.to_bits(), "EXTREMA_STATION")?;
    route.require(
        lower >= 0.0 && upper >= lower && global == upper && gap >= 0.0
            && gap <= 1e-12 + 1e-12 * lower && gap >= global - lower
            && value == lower + 0.5 * (upper - lower),
        "EXTREMA_BOUNDS",
    )?;
    route.require(cover.len() == STATIONS.len() && cover.iter().all(|c| upper >= *c), "EXTREMA_STATION_COVER")
}

/// The directional records of one case (§2.6), told apart by key set from
/// the marker records (`marker_keys`): each station row bound to its record,
/// recomputed from the bending rows and the record's factors; each maximum
/// bound to its row with the cover of that arc's station rows. `arcs` the
/// case's realized arcs; `with_basis` whether records carry `section_basis`.
/// Returns the bound row ids.
pub(crate) fn directional_records<'a>(
    route: Route,
    case: &'a str,
    records: &'a [Value],
    rows: &HashMap<&'a str, &'a Value>,
    index: &Index<'_>,
    arcs: &HashSet<&str>,
    with_basis: bool,
) -> Result<HashSet<&'a str>, String> {
    let mut bound = HashSet::new();
    let mut stations: BTreeMap<(&str, &str), Vec<f64>> = BTreeMap::new();
    let mut factors: HashMap<&str, (u64, u64)> = HashMap::new();
    let keys = |v: &Value, base: &[&str]| v.as_object().is_some_and(|o| o.len() == base.len() + usize::from(with_basis)
        && base.iter().all(|k| o.contains_key(*k)) && (!with_basis || o.contains_key("section_basis")));
    let mut maxima = Vec::new();
    for record in records {
        let station = keys(record, &STATION_RECORD_KEYS);
        let maximum = !station && record.as_object().is_some_and(|o| o.len() == RECORD_KEYS.len() + 2 + usize::from(with_basis)
            && RECORD_KEYS.iter().chain(["i_in_plane", "i_out_of_plane"].iter()).all(|k| o.contains_key(*k))
            && (!with_basis || o.contains_key("section_basis")));
        if !station && !maximum {
            continue; // a marker record (the caller's own key set)
        }
        let pipe = record["pipe_id"].as_str().ok_or_else(|| route.fail("INTENSIFIED_SHAPE"))?;
        route.require(arcs.contains(pipe), "ARC_ROW_ON_STRAIGHT")?;
        let id = record["result_id"].as_str().ok_or_else(|| route.fail("INTENSIFIED_SHAPE"))?;
        let row = rows.get(id).ok_or_else(|| route.fail("INTENSIFIED_RESULT_MISSING"))?;
        route.require(bound.insert(id), "INTENSIFIED_DUPLICATE")?;
        let basis = if with_basis { record["section_basis"].as_str().unwrap_or("-") } else { "" };
        let i_ip = record["i_in_plane"].as_f64().filter(|v| v.is_finite() && *v > 0.0).ok_or_else(|| route.fail("INTENSIFIED_RANGE"))?;
        let i_op = record["i_out_of_plane"].as_f64().filter(|v| v.is_finite() && *v > 0.0).ok_or_else(|| route.fail("INTENSIFIED_RANGE"))?;
        let pair = (i_ip.to_bits(), i_op.to_bits());
        route.require(*factors.entry(pipe).or_insert(pair) == pair, "INTENSIFIED_FACTORS")?;
        route.require(case_of(row) == Some(case) && row["entity_ref"] == pipe && basis_of(row) == basis, "INTENSIFIED_BINDING")?;
        if station {
            let location = record["location"].as_str().unwrap_or("");
            route.require(row["kind"] == DIRECTIONAL && row["metadata"]["location"] == location, "INTENSIFIED_BINDING")?;
            let y = index.value(case, pipe, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis);
            let z = index.value(case, pipe, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis);
            let (Some(y), Some(z)) = (y, z) else { return Err(route.fail("INTENSIFIED_OPERANDS")) };
            let value = row["value"].as_f64().unwrap_or(f64::NAN);
            route.require(value.to_bits() == (1e6 * norm2(i_ip * z, i_op * y)).to_bits(), "INTENSIFIED_VALUE")?;
            stations.entry((pipe, basis)).or_default().push(value);
        } else {
            route.require(row["kind"] == DIRECTIONAL_MAX, "INTENSIFIED_BINDING")?;
            maxima.push((pipe, basis, record, *row));
        }
    }
    for (pipe, basis, record, row) in maxima {
        let cover = stations.get(&(pipe, basis)).map(Vec::as_slice).unwrap_or(&[]);
        maximum_record(route, record, row, ARC_DIRECTIONAL_APPROXIMATION, cover)?;
    }
    // Each directional arc has all five stations on every basis it states.
    for cover in stations.values() {
        route.require(cover.len() == STATIONS.len(), "INTENSIFIED_COVERAGE")?;
    }
    Ok(bound)
}

/// The named outcome of a directional maximum the producer could not certify.
pub(crate) const DIRECTIONAL_MAX_UNAVAILABLE: &str = "DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE";

/// The captured model's realized arcs with the directional pair (§2.6), by the
/// producer's realized-arc rule (`is_curved_bend_macro_component`): a `bend`
/// or `elbow` consumed as `curved_bend_macro_element`, with a nonempty
/// `geometry.bend_pipe_ref`, and both `modifiers.sif_in_plane_user_value` and
/// `modifiers.sif_out_of_plane_user_value`, each dimensionless (unit `1` or
/// `none`), finite and above zero. A dimensionless quantity normalizes to its
/// authored binary64 value, so the pair is `(i_ip, i_op)` as authored. Any
/// other placement of the pair is refused (the producer blocks such a model).
pub(crate) fn directional_arcs(model: &Value, route: Route) -> Result<HashMap<&str, (f64, f64)>, String> {
    let mut arcs = HashMap::new();
    for component in model["components"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let modifiers = &component["modifiers"];
        let (ip, op) = (&modifiers["sif_in_plane_user_value"], &modifiers["sif_out_of_plane_user_value"]);
        if ip.is_null() && op.is_null() {
            continue;
        }
        let realized = matches!(component["kind"].as_str(), Some("bend" | "elbow"))
            && component["mechanics_interface"]["solver_consumption"] == "curved_bend_macro_element";
        let pipe = component["geometry"]["bend_pipe_ref"].as_str().filter(|p| !p.trim().is_empty());
        let factor = |q: &Value| {
            q["value"].as_f64().filter(|v| v.is_finite() && *v > 0.0 && matches!(q["unit"].as_str(), Some("1" | "none")))
        };
        match (realized, pipe, factor(ip), factor(op)) {
            (true, Some(pipe), Some(i_ip), Some(i_op)) => {
                route.require(arcs.insert(pipe, (i_ip, i_op)).is_none(), "INTENSIFIED_MODEL")?;
            }
            _ => return Err(route.fail("INTENSIFIED_MODEL")),
        }
    }
    Ok(arcs)
}

/// Review item 1 (§2.6, §5.1): the directional rows and records bound to the
/// captured model, on both routes. In every case, every realized arc with the
/// pair in the model has its five station rows on every basis, the factors of
/// each of its records are the model's bit for bit, and it has the along-arc
/// maximum row or the named unavailable outcome
/// (`DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE` for that case, arc and, on
/// pressure-2, basis). Directional rows or records on any other member are
/// refused. The envelope readers have bound every row to one record and
/// recomputed it from the record's factors already.
pub(crate) fn directional_model_checks(source: &Value, model: &Value, route: Route) -> Check {
    // G2's convention: a model check reads only what the model carries; a model
    // without `components` states nothing about the directional pair.
    if model.get("components").is_none_or(Value::is_null) {
        return Ok(());
    }
    let rows = source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let diagnostics = source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let solved = source["status"]["mechanics"] == "MECHANICS_SOLVED";
    let arcs = if solved { directional_arcs(model, route)? } else { HashMap::new() };
    for row in rows {
        if matches!(row["kind"].as_str(), Some(DIRECTIONAL | DIRECTIONAL_MAX)) {
            route.require(row["entity_ref"].as_str().is_some_and(|pipe| arcs.contains_key(pipe)), "INTENSIFIED_MODEL")?;
        }
    }
    let index = Index::new(rows, route)?;
    let (key, bases): (&str, &[&str]) = match route {
        Route::Exact => ("exact_cases", &["nominal_section", "reduced_section"]),
        Route::Preview => ("preview_cases", &[""]),
    };
    for case in source["contract_evidence"][key].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let case_id = case["load_case_id"].as_str().ok_or_else(|| route.fail("INTENSIFIED_MODEL"))?;
        for record in case["intensified_measures"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            if record.get("i_in_plane").is_none() && record.get("i_out_of_plane").is_none() {
                continue; // a marker record (the preview reader's own key set)
            }
            let pair = record["pipe_id"].as_str().and_then(|pipe| arcs.get(pipe)).ok_or_else(|| route.fail("INTENSIFIED_MODEL"))?;
            let bits = |k: &str| record[k].as_f64().map(f64::to_bits);
            route.require(bits("i_in_plane") == Some(pair.0.to_bits()) && bits("i_out_of_plane") == Some(pair.1.to_bits()), "INTENSIFIED_MODEL_FACTORS")?;
        }
        for &pipe in arcs.keys() {
            for &basis in bases {
                for location in STATIONS {
                    route.require(index.value(case_id, pipe, DIRECTIONAL, "intensified_bending", location, basis).is_some(), "INTENSIFIED_MODEL_COVERAGE")?;
                }
                let maximum = index.value(case_id, pipe, DIRECTIONAL_MAX, "intensified_bending_maximum", "governing_station", basis).is_some();
                let suffix = match basis {
                    "nominal_section" => ":maximum:nominal-section",
                    "reduced_section" => ":maximum:reduced-section",
                    _ => ":maximum",
                };
                let unavailable = diagnostics.iter().any(|d| {
                    d["code"] == DIRECTIONAL_MAX_UNAVAILABLE && d["affected_refs"][0] == case_id && d["affected_refs"][1] == pipe
                        && d["id"].as_str().is_some_and(|id| id.ends_with(suffix))
                });
                route.require(maximum || unavailable, "INTENSIFIED_MODEL_COVERAGE")?;
            }
        }
    }
    Ok(())
}

/// The exact route's model-bound U4b checks (§5.1, G2), beside
/// `reader_verdict::physics_model_checks` on the pressure-2 reader only: the
/// directional binding (review item 1).
pub(crate) fn exact_model_checks(source: &Value, model: &Value) -> Check {
    // W-15 slice d (DEL-03-05 Design §5): the bodies the model's eligibility
    // predicate expects, with their records.
    crate::component_body::model_checks(source, model, crate::semantic_contract::pressure_rows_contract())?;
    directional_model_checks(source, model, Route::Exact)
}

/// The members a case's `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` warnings name.
pub(crate) fn poisson_exempt<'a>(diagnostics: &'a [Value], case: &str) -> HashSet<&'a str> {
    diagnostics
        .iter()
        .filter(|d| d["code"] == POISSON_OUT_OF_DOMAIN && d["affected_refs"][0] == case)
        .filter_map(|d| d["affected_refs"][1].as_str())
        .collect()
}

/// §5.1 on preview-physics-2 with the captured model (G2), for every member of
/// every case, including members with no τ row (review items 3 and 4):
/// ν = RN(RN(E/RN(2·G)) − 1) from the moduli the solve uses for that member
/// and case (§2.4), a named point's (`modulus_basis_ref`) or the base
/// material's E and G. In the Saint-Venant domain the member carries no
/// `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` warning and each τ row is
/// recomputed bit for bit; outside it the member is named by the warning (and
/// the envelope reader has required no τ row). A warning naming a member
/// outside the case is refused. The one exception is the design's
/// interpolation: a case with `modulus_basis_temperature` keeps the
/// model-free checks. (No preview case carries `analysis_state`: the
/// producer refuses it without the exact pressure contract.) The section
/// follows the producer's annulus: t = t_n − c_m, inner diameter D − 2t,
/// r_i = (D − 2t)/2, I = π(D⁴ − d⁴)/64.
pub(crate) fn preview_model_checks(source: &Value, model: &Value) -> Check {
    use open_pipe_stress_units::Dimension;
    // W-15 slice d (DEL-03-05 Design §5): the bodies the model's eligibility
    // predicate expects, with their records.
    crate::component_body::model_checks(source, model, crate::semantic_contract::preview_physics_contract())?;
    let route = Route::Preview;
    let length = |q: &Value| crate::physics_source::normalized_quantity(q, Dimension::Length);
    let stress = |q: &Value| crate::physics_source::normalized_quantity(q, Dimension::Stress);
    let rows = source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let diagnostics = source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let index = Index::new(rows, route)?;
    let items = |v: &Value| v.as_array().cloned().unwrap_or_default();
    for case in source["contract_evidence"]["preview_cases"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let Some(case_id) = case["load_case_id"].as_str() else { continue };
        let Some(model_case) = items(&model["load_cases"]).into_iter().find(|c| c["id"] == case_id) else { continue };
        let members: std::collections::BTreeSet<&str> = rows.iter()
            .filter(|r| case_of(r) == Some(case_id) && r["kind"] == "element_local_axial_force")
            .filter_map(|r| r["entity_ref"].as_str())
            .collect();
        let exempt = poisson_exempt(diagnostics, case_id);
        route.require(exempt.iter().all(|pipe| members.contains(pipe)), "SHEAR_POISSON_EXEMPTION")?;
        if model_case.get("modulus_basis_temperature").is_some_and(|t| !t.is_null()) {
            continue; // §2.4: interpolated moduli keep the model-free checks
        }
        // A load-reference-state case never reaches this route: the producer
        // refuses `analysis_state` without the exact pressure contract
        // (LOAD_STATE_CONTRACT_VERSION_MISMATCH, no rows), so every member of a
        // preview case takes the named point's or the base moduli.
        for &pipe_id in &members {
            let Some(pipe) = items(&model["pipe_segments"]).into_iter().find(|p| p["id"] == pipe_id) else { continue };
            let material = items(&model["materials"]).into_iter().find(|m| m["id"] == pipe["material"]).ok_or_else(|| route.fail("SHEAR_MODEL"))?;
            let (e, g) = match model_case["modulus_basis_ref"].as_str() {
                Some(point) => {
                    let p = items(&material["temperature_points"]).into_iter().find(|p| p["id"] == point).ok_or_else(|| route.fail("SHEAR_MODEL"))?;
                    (stress(&p["elastic_modulus"])?, stress(&p["shear_modulus"])?)
                }
                None => (stress(&material["elastic_modulus"])?, stress(&material["shear_modulus"])?),
            };
            let section = &pipe["section"];
            let od = length(&section["outside_diameter"])?;
            let wall = length(&section["wall_thickness"])?;
            let t = if section["mill_tolerance"].is_null() { wall } else { wall - length(&section["mill_tolerance"])? };
            let id = od - 2.0 * t;
            let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
            let factor = preview_shear_factor(e, g, od / 2.0, id / 2.0, inertia);
            route.require(factor.is_none() == exempt.contains(pipe_id), "SHEAR_POISSON_EXEMPTION")?;
            let Some(factor) = factor else { continue };
            for row in rows.iter().filter(|r| r["kind"] == SHEAR_MAX && case_of(r) == Some(case_id) && r["entity_ref"] == pipe_id) {
                let location = row["metadata"]["location"].as_str().unwrap_or("");
                let vy = index.any(case_id, pipe_id, "element_local_shear_force_y", location, "").ok_or_else(|| route.fail("SHEAR_COVERAGE"))?;
                let vz = index.any(case_id, pipe_id, "element_local_shear_force_z", location, "").ok_or_else(|| route.fail("SHEAR_COVERAGE"))?;
                route.require(row["value"].as_f64().map(f64::to_bits) == Some((norm2(vy, vz) * factor / 1e6).to_bits()), "SHEAR_VALUE")?;
            }
        }
    }
    // Directional rows and records bound to the model (review item 1).
    directional_model_checks(source, model, route)
}
