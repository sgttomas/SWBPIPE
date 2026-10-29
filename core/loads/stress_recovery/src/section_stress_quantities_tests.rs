//! T4-U4b: the section quantities against the frozen t4_u4b references
//! (`validation/references/t4_u4b/oracle.json`, read as data), at the README
//! criterion |obs − exp| ≤ 1e-9·max(|exp|, zero_scale), with the field-level
//! discriminators of the U4b design §7.1 shown separated.
use super::fields::{fibre_normal, saint_venant_field, toroidal_hoop_at};
use super::*;
use std::collections::HashMap;

const ORACLE: &str = include_str!("../../../../validation/references/t4_u4b/oracle.json");
const BITS: &str = include_str!("../tests/u4b_section_bits/section_quantity_bits.json");

/// A minimal JSON reader for the frozen reference file (test-only; no new
/// dependency). Numbers are parsed with Rust's correctly rounded `f64` parser.
mod json {
    #[derive(Debug, Clone)]
    pub(super) enum Value {
        Null,
        Bool(bool),
        Number(f64),
        Str(String),
        Array(Vec<Value>),
        Object(Vec<(String, Value)>),
    }

    impl Value {
        pub(super) fn get(&self, key: &str) -> Option<&Value> {
            match self {
                Value::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
                _ => None,
            }
        }
        pub(super) fn at(&self, key: &str) -> &Value {
            self.get(key).unwrap_or_else(|| panic!("missing key {key}"))
        }
        pub(super) fn num(&self) -> f64 {
            match self {
                Value::Number(x) => *x,
                other => panic!("not a number: {other:?}"),
            }
        }
        /// The binary64 member of a `[decimal, binary64]` pair.
        pub(super) fn b64(&self) -> f64 {
            self.items()[1].num()
        }
        pub(super) fn text(&self) -> &str {
            match self {
                Value::Str(s) => s,
                other => panic!("not a string: {other:?}"),
            }
        }
        pub(super) fn items(&self) -> &[Value] {
            match self {
                Value::Array(items) => items,
                other => panic!("not an array: {other:?}"),
            }
        }
        pub(super) fn entries(&self) -> &[(String, Value)] {
            match self {
                Value::Object(entries) => entries,
                other => panic!("not an object: {other:?}"),
            }
        }
    }

    pub(super) fn parse(text: &str) -> Value {
        let mut parser = Parser {
            bytes: text.as_bytes(),
            at: 0,
        };
        let value = parser.value();
        parser.space();
        assert_eq!(parser.at, parser.bytes.len(), "trailing data");
        value
    }

    struct Parser<'a> {
        bytes: &'a [u8],
        at: usize,
    }

    impl Parser<'_> {
        fn space(&mut self) {
            while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
                self.at += 1;
            }
        }
        fn eat(&mut self, byte: u8) {
            self.space();
            assert_eq!(self.bytes[self.at], byte, "at {}", self.at);
            self.at += 1;
        }
        fn value(&mut self) -> Value {
            self.space();
            match self.bytes[self.at] {
                b'{' => {
                    self.at += 1;
                    let mut entries = Vec::new();
                    self.space();
                    if self.bytes[self.at] == b'}' {
                        self.at += 1;
                        return Value::Object(entries);
                    }
                    loop {
                        self.space();
                        let key = self.string();
                        self.eat(b':');
                        let value = self.value();
                        entries.push((key, value));
                        self.space();
                        let next = self.bytes[self.at];
                        self.at += 1;
                        if next == b'}' {
                            return Value::Object(entries);
                        }
                        assert_eq!(next, b',');
                    }
                }
                b'[' => {
                    self.at += 1;
                    let mut items = Vec::new();
                    self.space();
                    if self.bytes[self.at] == b']' {
                        self.at += 1;
                        return Value::Array(items);
                    }
                    loop {
                        items.push(self.value());
                        self.space();
                        let next = self.bytes[self.at];
                        self.at += 1;
                        if next == b']' {
                            return Value::Array(items);
                        }
                        assert_eq!(next, b',');
                    }
                }
                b'"' => Value::Str(self.string()),
                b't' => {
                    self.at += 4;
                    Value::Bool(true)
                }
                b'f' => {
                    self.at += 5;
                    Value::Bool(false)
                }
                b'n' => {
                    self.at += 4;
                    Value::Null
                }
                _ => {
                    let start = self.at;
                    while self.at < self.bytes.len()
                        && matches!(
                            self.bytes[self.at],
                            b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                        )
                    {
                        self.at += 1;
                    }
                    let token = std::str::from_utf8(&self.bytes[start..self.at]).unwrap();
                    Value::Number(
                        token
                            .parse()
                            .unwrap_or_else(|_| panic!("bad number {token}")),
                    )
                }
            }
        }
        fn string(&mut self) -> String {
            assert_eq!(self.bytes[self.at], b'"');
            self.at += 1;
            let mut out = Vec::new();
            loop {
                let byte = self.bytes[self.at];
                self.at += 1;
                match byte {
                    b'"' => return String::from_utf8(out).unwrap(),
                    b'\\' => {
                        let escaped = self.bytes[self.at];
                        self.at += 1;
                        match escaped {
                            b'u' => {
                                let hex =
                                    std::str::from_utf8(&self.bytes[self.at..self.at + 4]).unwrap();
                                self.at += 4;
                                let code = u32::from_str_radix(hex, 16).unwrap();
                                let ch = char::from_u32(code).unwrap_or('\u{fffd}');
                                let mut buffer = [0u8; 4];
                                out.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
                            }
                            b'n' => out.push(b'\n'),
                            b't' => out.push(b'\t'),
                            b'r' => out.push(b'\r'),
                            b'b' => out.push(8),
                            b'f' => out.push(12),
                            other => out.push(other),
                        }
                    }
                    other => out.push(other),
                }
            }
        }
    }
}

use json::Value;

const STATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
const HALF_SQRT2: f64 = std::f64::consts::FRAC_1_SQRT_2;
/// The oracle's six SV and hoop fibres: (name, cos β, sin β).
const SIX_FIBRES: [(&str, f64, f64); 6] = [
    ("y_pos", 1.0, 0.0),
    ("diag_45", HALF_SQRT2, HALF_SQRT2),
    ("z_pos", 0.0, 1.0),
    ("diag_135", -HALF_SQRT2, HALF_SQRT2),
    ("y_neg", -1.0, 0.0),
    ("z_neg", 0.0, -1.0),
];
/// The four named fibres of `fibre_normal`.
const FOUR_FIBRES: [(&str, f64, f64); 4] = [
    ("y_pos", 1.0, 0.0),
    ("z_pos", 0.0, 1.0),
    ("y_neg", -1.0, 0.0),
    ("z_neg", 0.0, -1.0),
];

/// |obs − exp| / (1e-9·max(|exp|, scale)).
fn tolerances(observed: f64, expected: f64, scale: f64) -> f64 {
    (observed - expected).abs() / (1e-9 * expected.abs().max(scale))
}

#[derive(Default)]
struct Worst(HashMap<&'static str, (f64, usize)>);

impl Worst {
    fn record(&mut self, quantity: &'static str, label: &str, fraction: f64) {
        assert!(fraction <= 1.0, "{quantity} {label}: {fraction} tolerances");
        let entry = self.0.entry(quantity).or_insert((0.0, 0));
        entry.0 = entry.0.max(fraction);
        entry.1 += 1;
    }
}

/// One member, station and basis, with the oracle inputs this module consumes.
struct Station<'a> {
    case: &'a str,
    member: &'a str,
    station: &'static str,
    basis: &'a str,
    /// N_eff: N_w on a pressurized exact member, N otherwise.
    axial: f64,
    pressurized: bool,
    v_y: f64,
    v_z: f64,
    m_y: f64,
    m_z: f64,
    nu: f64,
    area: f64,
    inertia: f64,
    z: f64,
    ro: f64,
    ri: f64,
    normal_scale: f64,
    shear_scale: f64,
    quantities: &'a Value,
    derived: &'a Value,
}

impl Station<'_> {
    fn label(&self) -> String {
        format!(
            "{} {} {} {}",
            self.case, self.member, self.station, self.basis
        )
    }
    fn row(&self, quantity: &str) -> &Value {
        self.quantities
            .at(quantity)
            .at(self.member)
            .at(self.station)
            .at(self.basis)
    }
    /// The component stress rows (MPa), formed as the product's rows are.
    fn stress_rows(&self) -> (f64, f64, f64) {
        let sigma_n = if self.pressurized {
            pa_to_mpa(self.axial / self.area)
        } else {
            self.axial / self.area / 1e6
        };
        (sigma_n, self.m_y / self.z / 1e6, self.m_z / self.z / 1e6)
    }
}

fn for_each_station(oracle: &Value, mut visit: impl FnMut(&Station<'_>)) -> usize {
    let mut count = 0;
    for (case_id, case) in oracle.at("cases").entries() {
        let quantities = case.at("quantities");
        for (member, derived) in case.at("derived").entries() {
            for station in STATIONS {
                let r = quantities
                    .at("resultants_tangent_frame_j_side")
                    .at(member)
                    .at(station);
                let (axial, pressurized) = match r.get("N_w") {
                    Some(n_w) => (n_w.b64(), true),
                    None => (r.at("N").b64(), false),
                };
                for (basis, section) in derived.at("section_bases").entries() {
                    let scales = case.at("zero_scales").at(member).at(basis);
                    visit(&Station {
                        case: case_id,
                        member,
                        station,
                        basis,
                        axial,
                        pressurized,
                        v_y: r.at("V_y").b64(),
                        v_z: r.at("V_z").b64(),
                        m_y: r.at("M_y").b64(),
                        m_z: r.at("M_z").b64(),
                        nu: derived.at("nu").b64(),
                        area: section.at("A").b64(),
                        inertia: section.at("I").b64(),
                        z: section.at("Z").b64(),
                        ro: section.at("ro").b64(),
                        ri: section.at("ri").b64(),
                        normal_scale: scales.at("normal_stress").b64(),
                        shear_scale: scales.at("shear_stress").b64(),
                        quantities,
                        derived,
                    });
                    count += 1;
                }
            }
        }
    }
    count
}

#[test]
fn normal_extremes_and_fibres_against_the_oracle() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    let mut mirror_failures = 0;
    let count = for_each_station(&oracle, |s| {
        let (sigma_n, sigma_y, sigma_z) = s.stress_rows();
        let extremes = normal_stress_extremes(sigma_n, sigma_y, sigma_z);
        let row = s.row("normal_extremes");
        let label = s.label();
        // Exact MPa → Pa conversion of the observed value for the comparison.
        for (quantity, observed, key) in [
            ("sigma_plus", extremes.sigma_plus, "sigma_plus"),
            ("sigma_minus", extremes.sigma_minus, "sigma_minus"),
            ("max_abs_normal", extremes.max_abs(), "max_abs_normal"),
        ] {
            worst.record(
                quantity,
                &label,
                tolerances(observed * 1e6, row.at(key).b64(), s.normal_scale),
            );
        }
        match (
            normal_extreme_fibres(s.ro, sigma_y, sigma_z),
            row.get("fibres"),
        ) {
            (None, Some(tied)) => assert!(tied.text().starts_with("tied"), "{label}"),
            (Some(fibres), None) => {
                for (fibre, key) in fibres.iter().zip(["sigma_plus_fibre", "sigma_minus_fibre"]) {
                    let expected = row.at(key);
                    for (i, axis) in ["y", "z"].iter().enumerate() {
                        worst.record(
                            "fibre_yz",
                            &label,
                            tolerances(fibre[i], expected.at(axis).b64(), s.ro),
                        );
                    }
                }
                // The test-side mirror (σ_z, −σ_y) must fail against the oracle.
                let h = open_pipe_stress_correct_norm::norm2(sigma_y, sigma_z);
                let mirror = [s.ro * (sigma_z / h), s.ro * (-sigma_y / h)];
                let expected = row.at("sigma_plus_fibre");
                if tolerances(mirror[0], expected.at("y").b64(), s.ro) > 1.0
                    || tolerances(mirror[1], expected.at("z").b64(), s.ro) > 1.0
                {
                    mirror_failures += 1;
                }
            }
            (ours, theirs) => panic!("{label}: fibre tie disagreement {ours:?} {theirs:?}"),
        }
    });
    assert_eq!(count, 170);
    assert!(mirror_failures > 0);
    println!(
        "normal extremes: {:?}; mirrored fibre fails on {mirror_failures} rows",
        worst.0
    );
}

#[test]
fn transverse_shear_maximum_against_the_oracle() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    for_each_station(&oracle, |s| {
        let factor = saint_venant_shear_factor(s.nu, s.ro, s.ri, s.inertia).unwrap();
        let tau = transverse_shear_maximum_mpa(s.v_y, s.v_z, factor);
        let expected = s.row("sv_transverse_shear_max").at("value").b64();
        worst.record(
            "tau_v_max",
            &s.label(),
            tolerances(tau * 1e6, expected, s.shear_scale),
        );
    });
    println!("transverse shear maximum: {:?}", worst.0);
}

#[test]
fn saint_venant_field_at_six_fibres_and_both_surfaces() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    for_each_station(&oracle, |s| {
        let field = s.row("sv_transverse_shear");
        for (surface, r) in [("outer", s.ro), ("inner", s.ri)] {
            for (name, c, sn) in SIX_FIBRES {
                let (xy, xz) =
                    saint_venant_field(s.v_y, s.v_z, s.nu, s.ro, s.ri, s.inertia, r, c, sn);
                let expected = field.at(surface).at(name);
                let label = format!("{} {surface} {name}", s.label());
                worst.record(
                    "sv_field",
                    &label,
                    tolerances(xy, expected.at("tau_xy").b64(), s.shear_scale),
                );
                worst.record(
                    "sv_field",
                    &label,
                    tolerances(xz, expected.at("tau_xz").b64(), s.shear_scale),
                );
            }
        }
    });
    println!("SV field: {:?}", worst.0);
}

#[test]
fn fibre_normal_at_four_fibres_and_both_surfaces() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    for_each_station(&oracle, |s| {
        let row = s.row("fibre_normal");
        for (surface, r) in [("outer", s.ro), ("inner", s.ri)] {
            for (name, c, sn) in FOUR_FIBRES {
                let value = fibre_normal(s.axial, s.area, s.m_y, s.m_z, s.inertia, r * c, r * sn);
                let label = format!("{} {surface} {name}", s.label());
                worst.record(
                    "fibre_normal",
                    &label,
                    tolerances(value, row.at(surface).at(name).b64(), s.normal_scale),
                );
            }
        }
    });
    println!("fibre normal: {:?}", worst.0);
}

/// One oracle discriminator row: (path parts, expected, wrong, separation).
fn listed_rows<'a>(case: &'a Value, mutant: &str) -> Vec<(Vec<&'a str>, f64, f64)> {
    case.at("discriminators")
        .at(mutant)
        .at("listed")
        .items()
        .iter()
        .map(|row| {
            (
                row.at("row").text().split('/').collect(),
                row.at("expected").b64(),
                row.at("wrong").b64(),
            )
        })
        .collect()
}

#[test]
fn field_function_discriminators_are_separated() {
    // §7.1: sv_vy_vz_swapped, fibre_mz_y_sign_flipped and
    // fibre_frame_rotated_y_away_from_centre are field-function mutants. On
    // every oracle row listed for them, the field function reproduces the
    // expected value at the criterion, the mutant reproduces the oracle's wrong
    // value, and the two lie at least 1e3 tolerances apart.
    let oracle = json::parse(ORACLE);
    let mut separations: HashMap<&str, f64> = HashMap::new();
    let mut rows = 0;
    for (case_id, case) in oracle.at("cases").entries() {
        let quantities = case.at("quantities");
        for mutant in [
            "sv_vy_vz_swapped",
            "fibre_mz_y_sign_flipped",
            "fibre_frame_rotated_y_away_from_centre",
        ] {
            let discriminator = case.at("discriminators").at(mutant);
            assert!(
                matches!(
                    discriminator.get("discriminable_in_this_case"),
                    Some(Value::Bool(true))
                ),
                "{case_id} {mutant}"
            );
            for (path, expected, wrong) in listed_rows(case, mutant) {
                let (member, station, basis, surface, fibre) =
                    (path[1], path[2], path[3], path[4], path[5]);
                let derived = case.at("derived").at(member);
                let section = derived.at("section_bases").at(basis);
                let r = quantities
                    .at("resultants_tangent_frame_j_side")
                    .at(member)
                    .at(station);
                let (ro, ri, inertia, area) = (
                    section.at("ro").b64(),
                    section.at("ri").b64(),
                    section.at("I").b64(),
                    section.at("A").b64(),
                );
                let radius = if surface == "outer" { ro } else { ri };
                let scales = case.at("zero_scales").at(member).at(basis);
                let (good, bad, scale) = if mutant == "sv_vy_vz_swapped" {
                    let (_, c, sn) = SIX_FIBRES.iter().copied().find(|f| f.0 == fibre).unwrap();
                    let (v_y, v_z, nu) =
                        (r.at("V_y").b64(), r.at("V_z").b64(), derived.at("nu").b64());
                    let pick = |t: (f64, f64)| if path[6] == "tau_xy" { t.0 } else { t.1 };
                    (
                        pick(saint_venant_field(
                            v_y, v_z, nu, ro, ri, inertia, radius, c, sn,
                        )),
                        pick(saint_venant_field(
                            v_z, v_y, nu, ro, ri, inertia, radius, c, sn,
                        )),
                        scales.at("shear_stress").b64(),
                    )
                } else {
                    let (_, c, sn) = FOUR_FIBRES.iter().copied().find(|f| f.0 == fibre).unwrap();
                    let axial = r.get("N_w").unwrap_or_else(|| r.at("N")).b64();
                    let (m_y, m_z) = (r.at("M_y").b64(), r.at("M_z").b64());
                    let (y, z) = (radius * c, radius * sn);
                    let bad = if mutant == "fibre_mz_y_sign_flipped" {
                        axial / area + (m_y * z + m_z * y) / inertia
                    } else {
                        axial / area - (m_y * z - m_z * y) / inertia
                    };
                    (
                        fibre_normal(axial, area, m_y, m_z, inertia, y, z),
                        bad,
                        scales.at("normal_stress").b64(),
                    )
                };
                let label = format!("{case_id} {mutant} {}", path.join("/"));
                assert!(
                    tolerances(good, expected, scale) <= 1.0,
                    "{label}: {good} vs {expected}"
                );
                assert!(
                    tolerances(bad, wrong, scale) <= 1.0,
                    "{label}: mutant {bad} vs {wrong}"
                );
                let separation = tolerances(bad, expected, scale);
                assert!(separation >= 1e3, "{label}: {separation}");
                let entry = separations.entry(mutant).or_insert(f64::INFINITY);
                *entry = entry.min(separation);
                rows += 1;
            }
        }
    }
    assert!(rows >= 3 * 8);
    println!("smallest separations (tolerances) over {rows} listed rows: {separations:?}");
}

#[test]
fn toroidal_hoop_and_radial_values_against_the_oracle() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    let mut arcs = 0;
    for (case_id, case) in oracle.at("cases").entries() {
        let quantities = case.at("quantities");
        let Some(hoops) = quantities.get("toroidal_membrane_hoop") else {
            continue;
        };
        for (member, by_basis) in hoops.entries() {
            for (basis, hoop) in by_basis.entries() {
                let p = hoop.at("parameters");
                let (pressure, ri, ro, t, r) = (
                    p.at("p").b64(),
                    p.at("ri").b64(),
                    p.at("ro").b64(),
                    p.at("t").b64(),
                    p.at("R").b64(),
                );
                let scale = case
                    .at("zero_scales")
                    .at(member)
                    .at(basis)
                    .at("pressure_stress")
                    .b64();
                let label = format!("{case_id} {member} {basis}");
                let fibres = hoop.at("fibres");
                // The published three.
                let published = toroidal_membrane_hoop(pressure, ri, t, ro, r).unwrap();
                for (value, name) in [
                    (published.intrados, "y_pos"),
                    (published.crown, "z_pos"),
                    (published.extrados, "y_neg"),
                ] {
                    worst.record(
                        "hoop_published",
                        &label,
                        tolerances(value, fibres.at(name).b64(), scale),
                    );
                }
                // All six oracle fibres through the general-β function.
                for (name, c, _) in SIX_FIBRES {
                    let value = toroidal_hoop_at(pressure, ri, t, ro, r, c);
                    worst.record(
                        "hoop_six_fibres",
                        &label,
                        tolerances(value, fibres.at(name).b64(), scale),
                    );
                }
                let radial = quantities.at("radial_surface").at(member).at(basis);
                let (inner, outer) = toroidal_radial_surface(pressure, ro, r).unwrap();
                worst.record(
                    "radial",
                    &label,
                    tolerances(inner, radial.at("inner").b64(), scale),
                );
                worst.record(
                    "radial",
                    &label,
                    tolerances(outer, radial.at("outer").b64(), scale),
                );
                arcs += 1;
            }
        }
    }
    assert_eq!(arcs, 12, "pressurized arc/basis pairs");
    println!("toroidal hoop and radial: {:?}", worst.0);
}

#[test]
fn directional_intensified_station_values_against_the_oracle() {
    let oracle = json::parse(ORACLE);
    let mut worst = Worst::default();
    let mut rows = 0;
    for_each_station(&oracle, |s| {
        let Some(sif) = s.derived.get("directional_sif") else {
            return;
        };
        let (_, sigma_y, sigma_z) = s.stress_rows();
        let value = directional_intensified_bending_pa(
            sif.at("i_ip").b64(),
            sif.at("i_op").b64(),
            sigma_y,
            sigma_z,
        );
        worst.record(
            "intensified",
            &s.label(),
            tolerances(value, s.row("intensified_bending").b64(), s.normal_scale),
        );
        rows += 1;
    });
    assert_eq!(rows, 75);
    println!("intensified station values: {:?}", worst.0);
}

#[test]
fn torus_domain_guard() {
    // ARC90-TIGHT nominal basis: r_o = 0.1125 m, bore 0.1 m, wall 0.0125 m.
    let (p, ri, t, ro) = (8.0e5, 0.1, 0.0125, 0.1125);
    let refused = |r: f64| {
        assert_eq!(
            toroidal_membrane_hoop(p, ri, t, ro, r),
            Err(SectionQuantityError::ToroidalDomain {
                bend_radius: r,
                outer_radius: ro
            })
        );
        assert_eq!(
            toroidal_radial_surface(p, ro, r),
            Err(SectionQuantityError::ToroidalDomain {
                bend_radius: r,
                outer_radius: ro
            })
        );
    };
    refused(ro.next_down());
    refused(ro);
    // R = (r_i + r_o)/2, where the intrados denominator vanishes, is below r_o.
    refused((ri + ro) / 2.0);
    let hoop = toroidal_membrane_hoop(p, ri, t, ro, ro.next_up()).unwrap();
    assert!(
        hoop.intrados.is_finite() && hoop.intrados > hoop.crown && hoop.crown > hoop.extrados,
        "{hoop:?}"
    );
    assert_eq!(hoop.crown, p * ri / t);
    assert_eq!(toroidal_radial_surface(p, ro, ro.next_up()), Ok((-p, 0.0)));
    // Monotone in cos β: p < 0 reverses the order.
    let hoop = toroidal_membrane_hoop(-p, ri, t, ro, 0.16875).unwrap();
    assert!(hoop.intrados < hoop.crown && hoop.crown < hoop.extrados);
    assert!(matches!(
        toroidal_membrane_hoop(f64::NAN, ri, t, ro, 1.0),
        Err(SectionQuantityError::InvalidInput { .. })
    ));
}

#[test]
fn shear_factor_poisson_domain_and_limits() {
    let (ro, ri) = (0.1_f64, 0.09_f64);
    let inertia = std::f64::consts::PI * (ro.powi(4) - ri.powi(4)) / 4.0;
    for nu in [-1.0, 0.5, 0.6, f64::NAN] {
        assert!(
            matches!(
                saint_venant_shear_factor(nu, ro, ri, inertia),
                Err(SectionQuantityError::PoissonOutOfDomain { .. })
            ),
            "{nu}"
        );
    }
    assert!(saint_venant_shear_factor(0.49, ro, ri, inertia).is_ok());
    assert!(saint_venant_shear_factor(-0.99, ro, ri, inertia).is_ok());
    assert!(matches!(
        saint_venant_shear_factor(0.3, ro, ro, inertia),
        Err(SectionQuantityError::InvalidInput { .. })
    ));
    // ν = 0: |V|(3r_o² + r_i²)/(4I) (README limit), and the fixed sequence.
    let factor = saint_venant_shear_factor(0.0, ro, ri, inertia).unwrap();
    assert_eq!(
        factor,
        (3.0 * (ro * ro) + 1.0 * (ri * ri)) / (4.0 * inertia)
    );
    assert_eq!(
        transverse_shear_maximum_mpa(3.0, -4.0, factor),
        5.0 * factor / 1e6
    );
    // Thin wall: about 2V/A.
    let (ro, ri) = (0.1_f64, 0.1_f64 - 1e-6);
    let area = std::f64::consts::PI * (ro * ro - ri * ri);
    let inertia = std::f64::consts::PI * (ro.powi(4) - ri.powi(4)) / 4.0;
    let factor = saint_venant_shear_factor(0.3, ro, ri, inertia).unwrap();
    assert!(
        (factor * area / 2.0 - 1.0).abs() < 1e-4,
        "{}",
        factor * area / 2.0
    );
}

#[test]
fn extremes_and_intensified_follow_the_fixed_sequences() {
    // STRESS_REFERENCE S1/S2: σ_N = 2, σ_y = 3, σ_z = 4 MPa → 7 and −3 MPa.
    let e = normal_stress_extremes(2.0, 3.0, 4.0);
    assert_eq!((e.sigma_plus, e.sigma_minus, e.max_abs()), (7.0, -3.0, 7.0));
    // The fibre at r_o = 0.05: RN(r_o·RN(∓σ/h)), about (−0.04, 0.03).
    let plus = [0.05 * (-4.0 / 5.0), 0.05 * (3.0 / 5.0)];
    assert_eq!(
        normal_extreme_fibres(0.05, 3.0, 4.0),
        Some([plus, [-plus[0], -plus[1]]])
    );
    assert!((plus[0] + 0.04).abs() <= 1e-17 && (plus[1] - 0.03).abs() <= 1e-17);
    assert_eq!(normal_extreme_fibres(0.05, 0.0, 0.0), None);
    assert_eq!(normal_extreme_fibres(0.05, -0.0, 0.0), None);
    // I1: i_ip = 2 on σ_z = 3, i_op = 3 on σ_y = 4 → 1e6·hypot(6, 12) = 6√5 MPa.
    let value = directional_intensified_bending_pa(2.0, 3.0, 4.0, 3.0);
    assert_eq!(value, 1e6 * open_pipe_stress_correct_norm::norm2(6.0, 12.0));
    assert!((value - 6.0 * 5.0_f64.sqrt() * 1e6).abs() <= 1e-9 * value);
    // Swapping the factors is a different value (sif_factors_swapped).
    assert!((directional_intensified_bending_pa(3.0, 2.0, 4.0, 3.0) - value).abs() > 1.0);
    assert_eq!(pa_to_mpa(4.2e6), 4.2);
}

fn bits_value(token: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(token, 16).expect(token))
}

#[test]
fn every_sequence_matches_the_independent_expected_bits() {
    // tests/u4b_section_bits/section_quantity_bits.json: exact rational
    // evaluation of each normative sequence (gen_section_quantity_bits.py).
    let document = json::parse(BITS);
    assert_eq!(document.at("schema").text(), "u4b.section_quantity_bits/1");
    let mut counts: HashMap<String, usize> = HashMap::new();
    for vector in document.at("vectors").items() {
        let id = vector.at("id").text();
        let quantity = vector.at("quantity").text();
        let input = |key: &str| bits_value(vector.at("inputs").at(key).text());
        let expected = |key: &str| vector.at("expected").at(key).text().to_string();
        let check = |key: &str, got: f64| {
            assert_eq!(
                format!("{:016x}", got.to_bits()),
                expected(key),
                "{id} {key}"
            );
        };
        match quantity {
            "saint_venant_shear_factor" => check(
                "c_b",
                saint_venant_shear_factor(
                    input("nu"),
                    input("outer_radius"),
                    input("inner_radius"),
                    input("second_moment"),
                )
                .unwrap(),
            ),
            "transverse_shear_maximum_mpa" => check(
                "tau_v_max",
                transverse_shear_maximum_mpa(input("shear_y"), input("shear_z"), input("c_b")),
            ),
            "normal_stress_extremes" => {
                let e =
                    normal_stress_extremes(input("sigma_n"), input("sigma_y"), input("sigma_z"));
                check("sigma_plus", e.sigma_plus);
                check("sigma_minus", e.sigma_minus);
            }
            "normal_stress_extremes_membrane" => {
                let sigma_n = pa_to_mpa(input("membrane_pa"));
                check("sigma_n", sigma_n);
                let e = normal_stress_extremes(sigma_n, input("sigma_y"), input("sigma_z"));
                check("sigma_plus", e.sigma_plus);
                check("sigma_minus", e.sigma_minus);
            }
            "directional_intensified_bending_pa" => check(
                "intensified",
                directional_intensified_bending_pa(
                    input("in_plane_factor"),
                    input("out_of_plane_factor"),
                    input("sigma_y"),
                    input("sigma_z"),
                ),
            ),
            "toroidal_membrane_hoop" => {
                let args = (
                    input("pressure"),
                    input("inner_radius"),
                    input("wall"),
                    input("outer_radius"),
                    input("bend_radius"),
                );
                let hoop = toroidal_membrane_hoop(args.0, args.1, args.2, args.3, args.4).unwrap();
                check("intrados", hoop.intrados);
                check("crown", hoop.crown);
                check("extrados", hoop.extrados);
                let (inner, outer) = toroidal_radial_surface(args.0, args.3, args.4).unwrap();
                check("radial_inner", inner);
                check("radial_outer", outer);
            }
            other => panic!("{id}: unknown quantity {other}"),
        }
        *counts.entry(quantity.to_string()).or_default() += 1;
    }
    // Codex's rounding-sensitive case (K2 review).
    let codex = document
        .at("vectors")
        .items()
        .iter()
        .find(|v| v.at("id").text() == "c_b/U4B-L90-P/pipe:S1/reduced_section")
        .unwrap();
    assert_eq!(codex.at("expected").at("c_b").text(), "4080cfbe845b9c73");
    let near_edge = document
        .at("vectors")
        .items()
        .iter()
        .filter(|v| v.at("id").text().starts_with("hoop/near_edge/"))
        .count();
    assert_eq!(near_edge, 27);
    assert_eq!(counts.values().sum::<usize>(), 908);
    println!("bit vectors: {counts:?}");
}

/// The x = 1/(1+ν) sequence (§2.4, the preview route below E/(2G) = 1/2):
/// its fixed operations, its refusals, and the same value as the ν sequence
/// where that sequence is exact (x = 1, 4/5, 1/(1+ν) with ν + 1 exact).
#[test]
fn shear_factor_of_ratio_sequence_and_refusals() {
    let (ro, ri, inertia) = (0.1_f64, 0.09_f64, 2.6e-5_f64);
    for x in [0.7, 1.0, 1.6, 2.0, 2.5, 3.0 * 2f64.powi(52), 1e300] {
        let factor = saint_venant_shear_factor_of_ratio(x, ro, ri, inertia).unwrap();
        let num = 2.0 * (ro * ro + ri * ri) + x * ((ro - ri) * (ro + ri));
        assert_eq!(factor.to_bits(), (num / (4.0 * inertia)).to_bits(), "{x}");
    }
    // ν = 0 and ν = 1/4 (x = 4/5 is inexact, so compare values within 8u).
    for (nu, x) in [(0.0, 1.0), (0.25, 0.8)] {
        let a = saint_venant_shear_factor(nu, ro, ri, inertia).unwrap();
        let b = saint_venant_shear_factor_of_ratio(x, ro, ri, inertia).unwrap();
        assert!((a - b).abs() <= 8.0 * f64::EPSILON * a, "{nu}: {a} vs {b}");
    }
    for x in [0.0, -1.0, 0.5, 2.0 / 3.0, f64::NAN, f64::INFINITY] {
        assert!(
            matches!(
                saint_venant_shear_factor_of_ratio(x, ro, ri, inertia),
                Err(SectionQuantityError::InvalidInput {
                    name: "modulus_ratio"
                })
            ),
            "{x}"
        );
    }
    assert!(matches!(
        saint_venant_shear_factor_of_ratio(1.0, ro, ro, inertia),
        Err(SectionQuantityError::InvalidInput {
            name: "inner_radius"
        })
    ));
    assert!(matches!(
        saint_venant_shear_factor_of_ratio(f64::MAX, ro, ri, inertia),
        Err(SectionQuantityError::InvalidInput {
            name: "saint_venant_shear_factor"
        })
    ));
}
