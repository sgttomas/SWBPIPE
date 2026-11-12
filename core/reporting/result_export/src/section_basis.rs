//! T4-U6/U8 (D-5, D-6): the section bases and the beam theory of exact-contract
//! evidence and stress rows (DEL-05-03 `section-bases-and-timoshenko.md` §1,
//! §2.1-§2.5 and §3.1), shared by the pressure-2, pressure-source-1 and
//! physics-retained-1 readers.
//!
//! What the readers rederive from a `pipe_sections` entry's captured authored
//! values (outside diameter D, wall t, mill tolerance c_m, corrosion allowance
//! c_a) in the fixed binary64 order: the nominal wall t and inner radius
//! D/2 - t, the reduced wall (t - c_m) - c_a and inner radius D/2 - t_r, the
//! pressure bore on the reduced inner radius, and, for
//! `timoshenko_annulus_energy_v1`, the energy-matched shear factor kappa_E and
//! the shear area kappa_E * A_n from the nominal radii and the member's nu (this
//! module's own implementation of §3.1's order, without fused multiply-add).
//! The section leaves A, I, J, Z and Ai stay leaves: the retained exact
//! successor overlays them with its prepared bits (DEF-E), so no reader
//! recomputes them. Coincident bases (equal walls) carry bit-equal leaves.
//!
//! A realized arc's entry, and only an arc's, carries `member_kind =
//! "realized_arc"` (§2.5, [`marked_section`]), so a reader without the model
//! knows every arc, including one outside every pressure region: a marked
//! entry is Euler-Bernoulli and the unmarked entries share one straight shear
//! model. The beam theory each entry states is bound to the captured model
//! where a reader has it (§5): the marked entries are the model's realized
//! arcs, a straight member states the contract's
//! `pressure_contract.shear_model`, a realized arc Euler-Bernoulli
//! ([`bind_shear_model`], [`selected_model`]).
//!
//! Stress rows name their basis (`<recovery token>; section_basis=<basis>` and
//! an id ending in `:<basis>` or its hyphenated spelling); every stress row has
//! its other-basis twin with the same declared semantics, and each twin pair is
//! checked against the two sections: bit-equal values where the bases coincide,
//! else the same section resultant (stress times its own A, Z or J) within the
//! 64-epsilon representation guard. The Lame rows are checked against their own
//! basis's annulus. These are internal-consistency checks, not accuracy claims.
use serde_json::Value;
use std::collections::HashMap;

type Check = Result<(), String>;

pub const NOMINAL: &str = "nominal_section";
pub const REDUCED: &str = "reduced_section";
pub const BASES: [&str; 2] = [NOMINAL, REDUCED];
/// The D-5 `pipe_sections` basis (§2.3).
pub const GEOMETRY_BASIS: &str = "authored_od_wall_d5_sections_v1";
/// §2.5: the key and the one value of a realized arc's marker. A straight
/// member's entry has no `member_kind`.
pub const MEMBER_KIND: &str = "member_kind";
pub const REALIZED_ARC: &str = "realized_arc";
/// §2.5: a `pipe_sections` entry's keys (a realized arc's adds [`MEMBER_KIND`]).
pub const SECTION_KEYS: [&str; 11] = [
    "pipe_id",
    "geometry_basis",
    "outside_diameter_m",
    "ro_m",
    "wall_thickness_m",
    "mill_tolerance_m",
    "corrosion_allowance_m",
    "nominal_section",
    "reduced_section",
    "pressure_bore",
    "beam_theory",
];
/// §2.5: a section object's keys.
pub const ANNULUS_KEYS: [&str; 6] = ["wall_m", "ri_m", "As_m2", "I_m4", "J_m4", "Z_m3"];
/// §2.5: the pressure bore's keys.
pub const BORE_KEYS: [&str; 2] = ["ri_m", "Ai_m2"];
pub const EULER_BERNOULLI: &str = "euler_bernoulli";
pub const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
/// §2.1: the kinds whose pressure-2 table family is `stress` and which the
/// exact route publishes, once per section basis.
pub const STRESS_KINDS: [&str; 15] = [
    // T4-U4b (§2.2): the new per-basis kinds.
    "element_local_normal_stress_extreme",
    "element_local_transverse_shear_stress_maximum",
    "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1",
    "curved_bend_directional_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_maximum_v1",
    // RT-A3 increment 1: a branch connection's intensified rows.
    "branch_intensified_bending_stress_v1",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_axial_membrane_stress_v2",
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_elastic_normal_stress_maximum_v2",
];
/// §2.4: the inherited thin-wall kinds (supported-source-019/059). The exact
/// producer never publishes them; a row of them names no section basis and is
/// refused on an exact envelope.
pub const THIN_WALL_KINDS: [&str; 2] = [
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
];
/// §2.5: the separator between a recovery token and its section basis.
pub const TOKEN_SEPARATOR: &str = "; section_basis=";

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PHYSICS_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn number(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "SOURCE_PHYSICS_NUMBER_INVALID".into())
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
/// T4-U6 (§2.5, A2): structural equality whose numbers compare by their
/// binary64 bits, the one equality of twins and links in the Rust readers and
/// the Python verifiers (`section_basis.bit_equal`): -0.0 is not 0.0, and the
/// same binary64 number in another JSON form (`1e6` or `1000000`, `0` or
/// `0.0`) is the same number. A non-finite or non-number leaf is unequal to a
/// number.
pub fn bit_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(_), Value::Number(_)) => {
            matches!((number(a), number(b)), (Ok(x), Ok(y)) if x.to_bits() == y.to_bits())
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| bit_equal(p, q)),
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| bit_equal(v, w)))
        }
        _ => a == b,
    }
}

pub fn is_stress_kind(kind: &str) -> bool {
    STRESS_KINDS.contains(&kind)
}
pub fn is_thin_wall_kind(kind: &str) -> bool {
    THIN_WALL_KINDS.contains(&kind)
}
fn basis_name(text: &str) -> Option<&'static str> {
    BASES.into_iter().find(|b| *b == text)
}
fn hyphenated(basis: &str) -> &'static str {
    if basis == NOMINAL {
        "nominal-section"
    } else {
        "reduced-section"
    }
}
/// The other basis.
pub fn other(basis: &str) -> &'static str {
    if basis == NOMINAL {
        REDUCED
    } else {
        NOMINAL
    }
}
/// A basis-named token: the recovery token and the section basis, or `None`
/// when the token names no section basis.
pub fn split_token(token: &str) -> Option<(&str, &'static str)> {
    let (recovery, basis) = token.rsplit_once(TOKEN_SEPARATOR)?;
    Some((recovery, basis_name(basis)?)).filter(|(recovery, _)| !recovery.is_empty() && !recovery.contains(TOKEN_SEPARATOR))
}
/// The section basis a result id ends in (`:nominal_section`, `:nominal-section`, ...).
pub fn id_basis(id: &str) -> Option<&'static str> {
    BASES
        .into_iter()
        .find(|b| id.ends_with(&format!(":{b}")) || id.ends_with(&format!(":{}", hyphenated(b))))
}
/// The twin's id: the same id with the other basis's final segment, in the same spelling.
pub fn twin_id(id: &str) -> Option<String> {
    for b in BASES {
        for (from, to) in [
            (format!(":{b}"), format!(":{}", other(b))),
            (format!(":{}", hyphenated(b)), format!(":{}", hyphenated(other(b)))),
        ] {
            if let Some(stem) = id.strip_suffix(from.as_str()) {
                return Some(format!("{stem}{to}"));
            }
        }
    }
    None
}
/// One stress row's declared basis: its recovery token and section basis, with
/// the id naming the same basis. A stress row without a basis is refused.
pub fn row_basis(row: &Value) -> Result<(&str, &'static str), String> {
    let token = row["metadata"]["basis"].as_str().unwrap_or_default();
    let (recovery, basis) = split_token(token).ok_or("SOURCE_PHYSICS_ROW_SECTION_BASIS")?;
    require(
        row["id"].as_str().and_then(id_basis) == Some(basis),
        "ROW_SECTION_BASIS",
    )?;
    Ok((recovery, basis))
}

/// One basis's annulus leaves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Annulus {
    pub wall: f64,
    pub ri: f64,
    pub area: f64,
    pub second_moment: f64,
    pub polar_moment: f64,
    pub modulus: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Beam {
    EulerBernoulli,
    Timoshenko { shear_factor: f64, shear_area: f64 },
}
/// A checked D-5 `pipe_sections` entry.
#[derive(Clone, Copy, Debug)]
pub struct Sections {
    pub outside_diameter: f64,
    pub ro: f64,
    pub wall: f64,
    pub mill_tolerance: f64,
    pub corrosion_allowance: f64,
    pub nominal: Annulus,
    pub reduced: Annulus,
    pub bore_ri: f64,
    pub bore_area: f64,
    pub beam: Beam,
}
impl Sections {
    /// The two bases coincide: equal walls and bit-equal leaves.
    pub fn coincide(&self) -> bool {
        self.nominal == self.reduced
    }
    pub fn basis(&self, basis: &str) -> &Annulus {
        if basis == NOMINAL {
            &self.nominal
        } else {
            &self.reduced
        }
    }
}

/// §1: the reduced wall in its fixed binary64 order, t_r = (t - c_m) - c_a.
pub fn reduced_wall(wall: f64, mill_tolerance: f64, corrosion_allowance: f64) -> f64 {
    (wall - mill_tolerance) - corrosion_allowance
}
/// §3.1: kappa_E, the energy-matched annular shear factor, in its fixed
/// binary64 order (no fused multiply-add) from the nominal binary64 radii and
/// the member's nu. Not Cowper's factor and not 5/6.
pub fn timoshenko_shear_factor(ri: f64, ro: f64, nu: f64) -> f64 {
    let m = ri / ro;
    let m2 = m * m;
    let a = 1.0 + m2;
    let b = 1.0 - m2;
    let n1 = 1.0 + nu;
    let c = (8.0 * nu + 14.0) * nu + 7.0;
    let d = c * (b * b) + 48.0 * (m2 * (n1 * n1));
    (6.0 * (n1 * n1)) * (a * a) / d
}

fn annulus(v: &Value, od: f64, wall: f64) -> Result<Annulus, String> {
    require(keys(v, &ANNULUS_KEYS), "GEOMETRY_SHAPE")?;
    let mut leaves = [0.0; 6];
    for (slot, key) in leaves.iter_mut().zip(ANNULUS_KEYS) {
        *slot = number(&v[key])?;
        require(*slot > 0.0, "GEOMETRY_RANGE")?;
    }
    let [w, ri, area, second_moment, polar_moment, modulus] = leaves;
    require(
        w == wall && w < od * 0.5 && ri == od * 0.5 - wall && ri < od * 0.5,
        "GEOMETRY_SOURCE_BINDING",
    )?;
    Ok(Annulus { wall: w, ri, area, second_moment, polar_moment, modulus })
}

/// §2.5: an entry's realized-arc marker: `true` for `member_kind =
/// "realized_arc"`, `false` without `member_kind`. Any other `member_kind` is
/// refused by name.
pub fn arc_marker(v: &Value) -> Result<bool, String> {
    match v.get(MEMBER_KIND) {
        None => Ok(false),
        Some(kind) if kind == REALIZED_ARC => Ok(true),
        Some(_) => Err("SOURCE_PHYSICS_MEMBER_KIND".into()),
    }
}
/// One `pipe_sections` entry (§2.5) with exactly [`SECTION_KEYS`], checked
/// against its own captured authored values. `beam_theory` is parsed here and
/// bound by [`beam_theory`]. A marked entry is read by [`marked_section`].
pub fn section(v: &Value) -> Result<Sections, String> {
    section_in(v, false)
}
/// One `pipe_sections` entry (§2.5) and its realized-arc marker ([`arc_marker`]):
/// a marked entry carries [`SECTION_KEYS`] and [`MEMBER_KIND`], an unmarked one
/// exactly [`SECTION_KEYS`].
pub fn marked_section(v: &Value) -> Result<(Sections, bool), String> {
    let arc = arc_marker(v)?;
    Ok((section_in(v, arc)?, arc))
}
fn section_in(v: &Value, marked: bool) -> Result<Sections, String> {
    require(
        v.as_object().is_some_and(|o| {
            o.len() == SECTION_KEYS.len() + usize::from(marked)
                && SECTION_KEYS.iter().all(|k| o.contains_key(*k))
                && (!marked || o.contains_key(MEMBER_KIND))
        }),
        "GEOMETRY_SHAPE",
    )?;
    require(v["pipe_id"].as_str().is_some_and(|s| !s.is_empty()), "GEOMETRY_SHAPE")?;
    require(v["geometry_basis"] == GEOMETRY_BASIS, "GEOMETRY_BASIS")?;
    let od = number(&v["outside_diameter_m"])?;
    let ro = number(&v["ro_m"])?;
    let wall = number(&v["wall_thickness_m"])?;
    let mill_tolerance = number(&v["mill_tolerance_m"])?;
    let corrosion_allowance = number(&v["corrosion_allowance_m"])?;
    require(
        od > 0.0 && ro > 0.0 && wall > 0.0 && mill_tolerance >= 0.0 && corrosion_allowance >= 0.0,
        "GEOMETRY_RANGE",
    )?;
    require(ro == od * 0.5, "GEOMETRY_SOURCE_BINDING")?;
    // c_m + c_a < t (the exact contract's section sum), and a positive reduced wall.
    let reduced = reduced_wall(wall, mill_tolerance, corrosion_allowance);
    require(mill_tolerance + corrosion_allowance < wall && reduced > 0.0, "SECTION_ALLOWANCE")?;
    let nominal = annulus(&v["nominal_section"], od, wall)?;
    let reduced = annulus(&v["reduced_section"], od, reduced)?;
    let bore = &v["pressure_bore"];
    require(keys(bore, &BORE_KEYS), "GEOMETRY_SHAPE")?;
    let bore_ri = number(&bore["ri_m"])?;
    let bore_area = number(&bore["Ai_m2"])?;
    require(bore_area > 0.0 && bore_ri == reduced.ri, "SECTION_BASES")?;
    // Coincident walls are one section: from_od_wall is a function of (D, wall).
    require(nominal.wall != reduced.wall || nominal == reduced, "SECTION_BASES")?;
    let beam = &v["beam_theory"];
    let beam = match beam["model"].as_str() {
        Some(EULER_BERNOULLI) if keys(beam, &["model"]) => Beam::EulerBernoulli,
        Some(TIMOSHENKO) if keys(beam, &["model", "shear_factor", "shear_area_m2"]) => Beam::Timoshenko {
            shear_factor: number(&beam["shear_factor"])?,
            shear_area: number(&beam["shear_area_m2"])?,
        },
        _ => return Err("SOURCE_PHYSICS_BEAM_THEORY".into()),
    };
    Ok(Sections {
        outside_diameter: od,
        ro,
        wall,
        mill_tolerance,
        corrosion_allowance,
        nominal,
        reduced,
        bore_ri,
        bore_area,
        beam,
    })
}

/// §3.1/§3.6: a realized arc states Euler-Bernoulli whatever the document
/// selects; a Timoshenko straight states kappa_E and A_s = kappa_E * A_n bit
/// for bit from its nominal radii and its case's nu.
pub fn beam_theory(sections: &Sections, nu: f64, arc: bool) -> Check {
    match sections.beam {
        Beam::EulerBernoulli => Ok(()),
        Beam::Timoshenko { .. } if arc => Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()),
        Beam::Timoshenko { shear_factor, shear_area } => {
            let kappa = timoshenko_shear_factor(sections.nominal.ri, sections.ro, nu);
            require(
                kappa.is_finite()
                    && kappa > 0.0
                    && shear_factor.to_bits() == kappa.to_bits()
                    && shear_area.to_bits() == (kappa * sections.nominal.area).to_bits(),
                "SHEAR_FACTOR",
            )
        }
    }
}
/// The selected model's name.
pub fn model(sections: &Sections) -> &'static str {
    match sections.beam {
        Beam::EulerBernoulli => EULER_BERNOULLI,
        Beam::Timoshenko { .. } => TIMOSHENKO,
    }
}

/// §3.6: the realized arcs of a captured model document, as the producer
/// realizes them: the pipes named by `geometry.bend_pipe_ref` of its `bend` or
/// `elbow` components whose `mechanics_interface.solver_consumption` is
/// `curved_bend_macro_element`. The evidence marks each arc's `pipe_sections`
/// entry and region geometry (§2.5); the model names which pipes those are.
pub fn realized_arcs(model: &Value) -> std::collections::HashSet<&str> {
    items(&model["components"])
        .iter()
        .filter(|c| {
            matches!(c["kind"].as_str(), Some("bend" | "elbow"))
                && c["mechanics_interface"]["solver_consumption"] == "curved_bend_macro_element"
        })
        .filter_map(|c| c["geometry"]["bend_pipe_ref"].as_str().filter(|r| !r.trim().is_empty()))
        .collect()
}
/// §3.6, §5: the beam theory a captured contract selects for one member: a
/// realized arc is Euler-Bernoulli whatever the document selects; a straight
/// member takes the contract's `pressure_contract.shear_model`.
pub fn selected_model(shear_model: &str, arc: bool) -> &str {
    if arc {
        EULER_BERNOULLI
    } else {
        shear_model
    }
}
/// §5 (pressure-2, against the captured model): each exact case's
/// `pipe_sections` entry and each pressure region's geometry entry states the
/// beam theory the model's contract selects for that member
/// ([`selected_model`]). §2.5: the entries the evidence marks
/// ([`arc_marker`]) are exactly the model's realized arcs: a realized arc's
/// unmarked entry or a straight member's marked entry is refused by
/// `SOURCE_PHYSICS_ARC_MARKER`, another `member_kind` by
/// `SOURCE_PHYSICS_MEMBER_KIND`. Arcs are the model's realized arcs and the
/// arcs the region geometry marks. An arc stating another model is refused by
/// `SOURCE_PHYSICS_ARC_BEAM_THEORY`; a straight member stating a model other
/// than the contract's, or a contract naming no admitted model, by
/// `SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT`. κ and A_s are the evidence reader's
/// ([`beam_theory`]); this binds only the selection.
pub fn bind_shear_model(evidence: &Value, model: &Value) -> Check {
    let shear_model = model["pressure_contract"]["shear_model"]
        .as_str()
        .filter(|m| [EULER_BERNOULLI, TIMOSHENKO].contains(m))
        .ok_or("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")?;
    let realized = realized_arcs(model);
    let mut arcs = realized.clone();
    for region in items(&evidence["pressure"]) {
        for g in items(&region["geometry"]) {
            if g.get(MEMBER_KIND).is_some() {
                arcs.insert(g["pipe_id"].as_str().unwrap_or_default());
            }
        }
    }
    let sections = items(&evidence["exact_cases"]).iter().flat_map(|case| items(&case["pipe_sections"]));
    let geometry = items(&evidence["pressure"]).iter().flat_map(|region| items(&region["geometry"]));
    for entry in sections.chain(geometry) {
        let pipe = entry["pipe_id"].as_str().unwrap_or_default();
        require(arc_marker(entry)? == realized.contains(pipe), "ARC_MARKER")?;
        let arc = arcs.contains(pipe);
        if entry["beam_theory"]["model"].as_str() != Some(selected_model(shear_model, arc)) {
            return Err(if arc { "SOURCE_PHYSICS_ARC_BEAM_THEORY" } else { "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT" }.into());
        }
    }
    Ok(())
}

/// The 64-epsilon representation guard, on the E17 scale (no new tolerance).
pub fn agree(a: f64, b: f64) -> bool {
    a.is_finite()
        && b.is_finite()
        && (a - b).abs() <= 64.0 * f64::EPSILON * a.abs().max(b.abs()).max(f64::MIN_POSITIVE)
}
/// The section property a linear stress kind divides its resultant by.
fn divisor(kind: &str, a: &Annulus) -> Option<f64> {
    match kind {
        "element_local_axial_normal_stress" | "pipe_axial_membrane_stress_v2" => Some(a.area),
        "element_local_bending_normal_stress_y" | "element_local_bending_normal_stress_z" => Some(a.modulus),
        "element_local_torsional_shear_stress" => Some(a.polar_moment),
        _ => None,
    }
}
/// A twin pair against the two sections: bit-equal where the bases coincide
/// (a signed zero is not its twin's zero); else the same resultant for the
/// linear kinds and bit-equal Lame radial values (the maximum and the Lame
/// hoop are checked against their own sections).
pub fn twin_values(kind: &str, nominal: f64, reduced: f64, sections: &Sections) -> Check {
    if sections.coincide() {
        return require(nominal.to_bits() == reduced.to_bits(), "ROW_BASIS_SECTION");
    }
    if let (Some(n), Some(r)) = (divisor(kind, &sections.nominal), divisor(kind, &sections.reduced)) {
        return require(agree(nominal * n, reduced * r), "ROW_BASIS_SECTION");
    }
    if kind == "pipe_lame_radial_stress_v2" {
        return require(nominal.to_bits() == reduced.to_bits(), "ROW_BASIS_SECTION");
    }
    Ok(())
}
/// §2.1: a straight member's Lame rows at one station on one basis: radial -p
/// (inner) and 0 (outer); hoop at the outer surface 2p*pi*ri_b^2/A_b and at the
/// inner surface that plus p, on the basis's own annulus.
pub fn lame_station(values: &HashMap<&str, f64>, annulus: &Annulus, p: f64) -> Check {
    let get = |component: &str| values.get(component).copied().ok_or("SOURCE_PHYSICS_PRESSURE_ROW_COVERAGE");
    let inner_radial = get("lame_inner_radial_stress")?;
    let outer_radial = get("lame_outer_radial_stress")?;
    let inner_hoop = get("lame_inner_hoop_stress")?;
    let outer_hoop = get("lame_outer_hoop_stress")?;
    let bore = 2.0 * (p * ((std::f64::consts::PI * annulus.ri) * annulus.ri));
    require(
        inner_radial == -p
            && outer_radial == 0.0
            && agree(outer_hoop * annulus.area, bore)
            && agree(inner_hoop, outer_hoop + p),
        "ROW_BASIS_SECTION",
    )
}

/// §2.5 per-basis records: a case's `stress_maximum_coverage`, the nominal
/// entry then the reduced entry; returns each basis's unavailable pipes.
pub fn coverage(v: &Value) -> Result<[(Vec<&str>, bool); 2], String> {
    let entries = v.as_array().ok_or("SOURCE_PHYSICS_STRESS_COVERAGE_SHAPE")?;
    require(entries.len() == 2, "STRESS_COVERAGE_SHAPE")?;
    let mut out: [(Vec<&str>, bool); 2] = [(Vec::new(), true), (Vec::new(), true)];
    for ((entry, basis), slot) in entries.iter().zip(BASES).zip(out.iter_mut()) {
        require(
            keys(&crate::component_body::coverage_without_bodies(entry), &["section_basis", "complete", "unavailable_pipe_ids"]) && entry["section_basis"] == basis,
            "STRESS_COVERAGE_SHAPE",
        )?;
        let ids = entry["unavailable_pipe_ids"].as_array().ok_or("SOURCE_PHYSICS_STRESS_COVERAGE_SHAPE")?;
        let mut unavailable = Vec::new();
        for id in ids {
            let id = id.as_str().filter(|s| !s.is_empty()).ok_or("SOURCE_PHYSICS_STRING_INVALID")?;
            require(!unavailable.contains(&id), "DUPLICATE_ID")?;
            unavailable.push(id);
        }
        let complete = entry["complete"].as_bool().ok_or("SOURCE_PHYSICS_STRESS_COVERAGE")?;
        require(complete == unavailable.is_empty(), "STRESS_COVERAGE")?;
        *slot = (unavailable, complete);
    }
    Ok(out)
}
/// An extremum record's basis.
pub fn extremum_basis(v: &Value) -> Result<&'static str, String> {
    v["section_basis"].as_str().and_then(basis_name).ok_or_else(|| "SOURCE_PHYSICS_EXTREMA_SHAPE".into())
}
/// Coincident bases: the reduced extremum is the nominal one but for its row
/// and basis, its numbers bit for bit ([`bit_equal`]). Distinct bases with a
/// reduced section no larger in A and Z: the reduced maximum's upper enclosure
/// is not below the nominal lower enclosure.
pub fn extrema_twins(nominal: &Value, reduced: &Value, sections: &Sections) -> Check {
    if sections.coincide() {
        let strip = |v: &Value| {
            let mut copy = v.clone();
            if let Some(o) = copy.as_object_mut() {
                o.remove("result_id");
                o.remove("section_basis");
            }
            copy
        };
        return require(bit_equal(&strip(nominal), &strip(reduced)), "ROW_BASIS_SECTION");
    }
    if sections.reduced.area <= sections.nominal.area && sections.reduced.modulus <= sections.nominal.modulus {
        return require(
            number(&reduced["value_upper_pa"])? >= number(&nominal["value_lower_pa"])?,
            "ROW_BASIS_SECTION",
        );
    }
    Ok(())
}

/// §2.2: the summary's per-basis headlines. `max_open_formula_stress` is null,
/// both basis keys are present, and each non-null headline needs its basis's
/// complete coverage, binds a maximum row of its own basis (its value bit for
/// bit, [`bit_equal`]) and is not below any maximum of that basis.
pub fn headlines(
    summary: &Value,
    rows: &HashMap<&str, &Value>,
    maxima: &[&Value],
    complete: [bool; 2],
) -> Check {
    require(
        summary.get("max_open_formula_stress").is_some_and(Value::is_null)
            && BASES.iter().all(|b| summary.get(format!("max_open_formula_stress_{b}")).is_some()),
        "HEADLINE_BASES",
    )?;
    for (basis, complete) in BASES.into_iter().zip(complete) {
        let headline = &summary[format!("max_open_formula_stress_{basis}")];
        if headline.is_null() {
            continue;
        }
        require(
            complete && keys(headline, &["value", "unit", "location_ref", "result_ref"]),
            "INCOMPLETE_STRESS_HEADLINE",
        )?;
        let row = rows
            .get(headline["result_ref"].as_str().unwrap_or_default())
            .ok_or("SOURCE_PHYSICS_HEADLINE_RESULT_MISSING")?;
        require(
            row["kind"] == "pipe_elastic_normal_stress_maximum_v2"
                && split_token(row["metadata"]["basis"].as_str().unwrap_or_default()).map(|t| t.1) == Some(basis)
                && bit_equal(&headline["value"], &row["value"])
                && headline["unit"] == row["unit"]
                && headline["location_ref"] == row["entity_ref"],
            "HEADLINE_RESULT_BINDING",
        )?;
        let value = row["value"].as_f64().unwrap_or(f64::NEG_INFINITY);
        require(
            maxima
                .iter()
                .filter(|r| split_token(r["metadata"]["basis"].as_str().unwrap_or_default()).map(|t| t.1) == Some(basis))
                .all(|r| r["value"].as_f64().unwrap_or(f64::INFINITY) <= value),
            "HEADLINE_MAXIMUM",
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn kappa_follows_the_fixed_order_and_its_limits() {
        // m -> 0 (solid circle limit of the formula): 6(1+nu)^2/(7+14nu+8nu^2).
        let nu: f64 = 0.3;
        let solid = (6.0 * ((1.0 + nu) * (1.0 + nu))) / ((8.0 * nu + 14.0) * nu + 7.0);
        assert_eq!(timoshenko_shear_factor(0.0, 1.0, nu).to_bits(), solid.to_bits());
        // nu = 0, m = 0: 6/7, not 5/6 and not 0.9.
        assert_eq!(timoshenko_shear_factor(0.0, 1.0, 0.0), 6.0 / 7.0);
        // m -> 1 (thin wall): 6(1+nu)^2*4/(48(1+nu)^2) = 1/2.
        assert_eq!(timoshenko_shear_factor(1.0, 1.0, nu), 0.5);
        let kappa = timoshenko_shear_factor(0.05, 0.06, 0.3);
        let cowper_thin = 2.0 * (1.0 + 0.3) / (4.0 + 3.0 * 0.3);
        assert!(kappa > 0.5 && kappa < 0.6 && kappa != cowper_thin && kappa != 5.0 / 6.0);
    }
    #[test]
    fn tokens_ids_and_twins_name_one_basis() {
        assert_eq!(split_token("recovered_from_local_element_stiffness; section_basis=reduced_section"),
            Some(("recovered_from_local_element_stiffness", REDUCED)));
        for bad in ["recovered_from_local_element_stiffness", "x; section_basis=other", "; section_basis=nominal_section",
            "a; section_basis=nominal_section; section_basis=reduced_section"] {
            assert_eq!(split_token(bad), None, "{bad}");
        }
        assert_eq!(twin_id("result:pressure-exact:a:axial_membrane_stress:nominal_section").as_deref(),
            Some("result:pressure-exact:a:axial_membrane_stress:reduced_section"));
        assert_eq!(twin_id("result:stress:p:end-i:bending-normal-y:reduced-section").as_deref(),
            Some("result:stress:p:end-i:bending-normal-y:nominal-section"));
        assert_eq!(twin_id("result:stress:p:end-i:bending-normal-y"), None);
        let row = json!({"id":"r:nominal-section","metadata":{"basis":"t; section_basis=reduced_section"}});
        assert_eq!(row_basis(&row), Err("SOURCE_PHYSICS_ROW_SECTION_BASIS".into()));
    }
    #[test]
    fn reduced_wall_keeps_the_ruled_order() {
        // (t - c_m) - c_a, not t - (c_m + c_a); with c_a = 0 it is t - c_m bit for bit.
        let (t, cm, ca) = (0.01_f64, 0.00125_f64, 0.002_f64);
        assert_eq!(reduced_wall(t, cm, ca).to_bits(), ((t - cm) - ca).to_bits());
        assert_eq!(reduced_wall(t, cm, 0.0).to_bits(), (t - cm).to_bits());
        let od = 0.1143_f64;
        assert_eq!((od * 0.5 - reduced_wall(t, cm, ca)).to_bits(), (od * 0.5 - ((t - cm) - ca)).to_bits());
    }
}
