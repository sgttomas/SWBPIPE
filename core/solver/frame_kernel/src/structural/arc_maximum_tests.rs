//! T4-U4b: tests of the certified arc maximum (DEL-04-07 Design
//! `u4b-u5b-implementation.md` §2.3, §2.6 and §7.1).
//!
//! - The frozen references in `validation/references/t4_u4b/` (`oracle.json`,
//!   `cases.json`), read as data: every arc's `member_maximum` and
//!   `intensified_bending_maximum` on every basis, from the oracle's own end-j
//!   section action, end-j frame, uniform loads, R, φ, p_Ai, A and Z.
//! - `tests/arc_maximum/` (`gen_arc_maximum_vectors.py`, standard library,
//!   `decimal` at 110 digits; `--check` regenerates byte for byte):
//!   `trig.txt` (sine and cosine constants), `two_peaks.txt` (the location
//!   controls) and `directed.txt` (high-precision evaluations of the same
//!   represented problems).
use super::*;
use std::collections::HashMap;

const ORACLE: &str = include_str!("../../../../../validation/references/t4_u4b/oracle.json");
const CASES: &str = include_str!("../../../../../validation/references/t4_u4b/cases.json");
const TRIG: &str = include_str!("../../tests/arc_maximum/trig.txt");
const TWO_PEAKS: &str = include_str!("../../tests/arc_maximum/two_peaks.txt");
const DIRECTED: &str = include_str!("../../tests/arc_maximum/directed.txt");
const END_CONTROLS: &str = include_str!("../../tests/arc_maximum/end_controls.txt");

/// A minimal JSON reader for the frozen reference files (test-only; no new
/// dependency). Numbers are parsed with Rust's correctly rounded `f64` parser.
mod json {
    #[derive(Debug, Clone)]
    pub(super) enum Value {
        Null,
        Bool,
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
                    Value::Bool
                }
                b'f' => {
                    self.at += 5;
                    Value::Bool
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

fn hex(token: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(token, 16).expect(token))
}

/// `<sign><32 hex digits>p<e>`: value = sign·m·2^(e − 127); `Z+` is +0.
fn wide(token: &str) -> Wide2 {
    if token == "Z+" {
        return Wide2::ZERO;
    }
    let negative = token.starts_with('-');
    let (m, e) = token[1..].split_once('p').unwrap();
    let m = u128::from_str_radix(m, 16).unwrap();
    Wide2::from_parts(negative, e.parse().unwrap(), [m as u64, (m >> 64) as u64]).unwrap()
}

/// |a − b| ≤ r, with the difference formed exactly (the two are close).
fn within_radius(a: &Wide2, b: &Wide2, r: f64) -> bool {
    let mut arith = WideArith::new(128).unwrap();
    let d = arith.sub(a, b).unwrap();
    abs_up(&d).unwrap() <= r
}

// ---------------------------------------------------------------------------
// The oracle arcs
// ---------------------------------------------------------------------------

struct MaximumRecord {
    value: f64,
    kind: String,
    fraction: f64,
    psi: f64,
    band_psi: Option<f64>,
    band_fraction: Option<f64>,
}

fn record(value: &Value) -> MaximumRecord {
    let location = value.at("location");
    let band = location
        .get("location_band")
        .or_else(|| value.get("location_band"));
    MaximumRecord {
        value: value.at("value").b64(),
        kind: location.at("kind").text().to_string(),
        fraction: location.at("fraction").b64(),
        psi: location.at("psi_rad").b64(),
        band_psi: band.map(|b| b.at("psi_rad").b64()),
        band_fraction: band.map(|b| b.at("fraction").b64()),
    }
}

struct OracleArc {
    case: String,
    member: String,
    basis: String,
    radius: f64,
    phi: f64,
    axes: [[f64; 3]; 3],
    force: [f64; 6],
    intensities: Vec<ArcUniformIntensity>,
    pa: Option<f64>,
    area: f64,
    z: f64,
    factors: (f64, f64),
    normal_scale: f64,
    elastic: MaximumRecord,
    intensified: MaximumRecord,
    elastic_stations: Vec<f64>,
    intensified_stations: Vec<f64>,
}

impl OracleArc {
    fn problem(&self) -> ArcStressMaximumProblem<'_> {
        ArcStressMaximumProblem {
            bend_radius: self.radius,
            included_angle: self.phi,
            end_j_axes: self.axes,
            end_j_force: self.force,
            uniform_intensities: &self.intensities,
            pressure_area_product: self.pa,
            area: self.area,
            section_modulus: self.z,
        }
    }

    fn measure(&self, name: &str) -> ArcStressMeasure {
        match name {
            "elastic" => ArcStressMeasure::ElasticNormal,
            "intensified" => ArcStressMeasure::DirectionalIntensified {
                in_plane: self.factors.0,
                out_of_plane: self.factors.1,
            },
            other => panic!("unknown measure {other}"),
        }
    }
}

const STATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];

fn oracle_arcs() -> Vec<OracleArc> {
    let oracle = json::parse(ORACLE);
    let cases = json::parse(CASES);
    let mut loads: HashMap<(String, String), Vec<[f64; 3]>> = HashMap::new();
    for case in cases.at("cases").items() {
        let id = case.at("case_id").text().to_string();
        if let Some(list) = case.at("load_case").get("member_uniform_forces") {
            for load in list.items() {
                let g = load.at("global_components_per_length").items();
                loads
                    .entry((id.clone(), load.at("member").text().to_string()))
                    .or_default()
                    .push([g[0].num(), g[1].num(), g[2].num()]);
            }
        }
    }
    let mut arcs = Vec::new();
    for (case_id, case) in oracle.at("cases").entries() {
        let quantities = case.at("quantities");
        for (member, derived) in case.at("derived").entries() {
            if derived.at("kind").text() != "realized_arc" {
                continue;
            }
            let end_j = quantities
                .at("resultants_tangent_frame_j_side")
                .at(member)
                .at("end_j");
            let axial = end_j.get("S").unwrap_or_else(|| end_j.at("N")).b64();
            let force = [
                axial,
                end_j.at("V_y").b64(),
                end_j.at("V_z").b64(),
                end_j.at("T").b64(),
                end_j.at("M_y").b64(),
                end_j.at("M_z").b64(),
            ];
            let frame = derived.at("tangent_frames_global_components").at("end_j");
            let mut axes = [[0.0; 3]; 3];
            for (a, key) in ["x", "y", "z"].iter().enumerate() {
                for (i, v) in frame.at(key).items().iter().enumerate() {
                    axes[a][i] = v.text().parse().unwrap();
                }
            }
            // Split per global component, as the product's intensity map does.
            let mut intensities = Vec::new();
            for g in loads
                .get(&(case_id.clone(), member.clone()))
                .into_iter()
                .flatten()
            {
                for (i, &value) in g.iter().enumerate() {
                    if value != 0.0 {
                        let mut global = [0.0; 3];
                        global[i] = value;
                        intensities.push(ArcUniformIntensity { global, scale: 1.0 });
                    }
                }
            }
            let pa = derived.get("p_Ai").map(Value::b64);
            let sif = derived.at("directional_sif");
            for (basis, section) in derived.at("section_bases").entries() {
                let station = |key: &str, field: Option<&str>| -> Vec<f64> {
                    STATIONS
                        .iter()
                        .map(|s| {
                            let v = quantities.at(key).at(member).at(s).at(basis);
                            match field {
                                Some(f) => v.at(f).b64(),
                                None => v.b64(),
                            }
                        })
                        .collect()
                };
                arcs.push(OracleArc {
                    case: case_id.clone(),
                    member: member.clone(),
                    basis: basis.clone(),
                    radius: derived.at("arc").at("R").b64(),
                    phi: derived.at("arc").at("included_angle_rad").b64(),
                    axes,
                    force,
                    intensities: intensities.clone(),
                    pa,
                    area: section.at("A").b64(),
                    z: section.at("Z").b64(),
                    factors: (sif.at("i_ip").b64(), sif.at("i_op").b64()),
                    normal_scale: case
                        .at("zero_scales")
                        .at(member)
                        .at(basis)
                        .at("normal_stress")
                        .b64(),
                    elastic: record(quantities.at("member_maximum").at(member).at(basis)),
                    intensified: record(
                        quantities
                            .at("intensified_bending_maximum")
                            .at(member)
                            .at(basis),
                    ),
                    elastic_stations: station("normal_extremes", Some("max_abs_normal")),
                    intensified_stations: station("intensified_bending", None),
                });
            }
        }
    }
    assert_eq!(arcs.len(), 15, "15 arc/basis pairs in t4_u4b");
    arcs
}

/// The directed.txt maximum line for one arc, basis and measure.
fn directed_maximum(case: &str, member: &str, basis: &str, measure: &str) -> (f64, f64) {
    for line in DIRECTED.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first() == Some(&"maximum")
            && f[1] == case
            && f[2] == member
            && f[3] == basis
            && f[4] == measure
        {
            return (hex(f[6]), hex(f[7]));
        }
    }
    panic!("no directed maximum for {case} {member} {basis} {measure}");
}

#[test]
fn oracle_arc_maxima_at_the_criterion() {
    let mut worst_value: HashMap<&str, f64> = HashMap::new();
    let mut worst_location: HashMap<&str, f64> = HashMap::new();
    for arc in oracle_arcs() {
        for (name, expected, stations) in [
            ("elastic", &arc.elastic, &arc.elastic_stations),
            ("intensified", &arc.intensified, &arc.intensified_stations),
        ] {
            // The station cover with the oracle's own station values (Pa).
            let result = certify_arc_stress_maximum(&arc.problem(), arc.measure(name), stations)
                .unwrap_or_else(|e| {
                    panic!("{} {} {} {name}: {e:?}", arc.case, arc.member, arc.basis)
                });
            let label = format!("{} {} {} {name}", arc.case, arc.member, arc.basis);
            // The gap rule and the publication.
            assert!(result.value_lower <= result.upper_bound, "{label}");
            assert!(
                result.certified_gap <= tolerance(result.value_lower),
                "{label}: {result:?}"
            );
            assert_eq!(
                result.midpoint.to_bits(),
                (result.value_lower + 0.5 * (result.upper_bound - result.value_lower)).to_bits()
            );
            // Directedness: the 110-digit maximum of the same represented problem.
            let (floor, ceil) = directed_maximum(&arc.case, &arc.member, &arc.basis, name);
            assert!(
                result.value_lower <= floor && result.upper_bound >= ceil,
                "{label}: {result:?} vs [{floor:e}, {ceil:e}]"
            );
            // The value at the README criterion.
            let scale = expected.value.abs().max(arc.normal_scale);
            let fraction_of_tolerance = (result.midpoint - expected.value).abs() / (1e-9 * scale);
            assert!(
                fraction_of_tolerance <= 1.0,
                "{label}: {} vs {}",
                result.midpoint,
                expected.value
            );
            let worst = worst_value.entry(name).or_insert(0.0);
            *worst = worst.max(fraction_of_tolerance);
            // The location.
            match expected.kind.as_str() {
                "end_i" => {
                    assert_eq!(result.location, ArcMaximumLocation::EndI, "{label}");
                    assert_eq!(result.fraction, 0.0, "{label}");
                }
                "end_j" => {
                    assert_eq!(result.location, ArcMaximumLocation::EndJ, "{label}");
                    assert_eq!(result.fraction, 1.0, "{label}");
                }
                "interior" => {
                    assert_eq!(result.location, ArcMaximumLocation::Bracket, "{label}");
                    let band_psi = expected.band_psi.unwrap();
                    let band_fraction = expected.band_fraction.unwrap();
                    let psi_error = (result.fraction * arc.phi - expected.psi).abs() / band_psi;
                    let fraction_error =
                        (result.fraction - expected.fraction).abs() / band_fraction;
                    assert!(
                        psi_error <= 1.0 && fraction_error <= 1.0,
                        "{label}: {} vs {}",
                        result.fraction,
                        expected.fraction
                    );
                    let worst = worst_location.entry(name).or_insert(0.0);
                    *worst = worst.max(psi_error.max(fraction_error));
                }
                other => panic!("{label}: unknown kind {other}"),
            }
            println!(
                "{label}: value {:.17e} (oracle {:.17e}, {:.3e} tol), fraction {:.12} ({:?}, oracle {}), subdivisions {}, K {:.4e}",
                result.midpoint, expected.value, fraction_of_tolerance, result.fraction, result.location,
                expected.fraction, result.subdivisions, result.curvature_bound
            );
        }
    }
    println!("worst value fraction of tolerance: {worst_value:?}");
    println!("worst interior location fraction of band: {worst_location:?}");
}

#[test]
fn arc106_closed_form() {
    // ψ* = φ − π/2 and f* = |pA_i|/A + F(1/A + R/Z) (t4_u4b item 4). The
    // represented problem has its maximum exactly at Δ = π/2.
    for arc in oracle_arcs()
        .into_iter()
        .filter(|a| a.case == "U4B-ARC106-RADIAL-NEGP")
    {
        let result =
            certify_arc_stress_maximum(&arc.problem(), ArcStressMeasure::ElasticNormal, &[])
                .unwrap();
        let (f, r) = (arc.force[1], arc.radius);
        assert_eq!(f, 7500.0);
        let closed = arc.pa.unwrap().abs() / arc.area + f * (1.0 / arc.area + r / arc.z);
        assert!(
            (result.midpoint - closed).abs() <= 1e-13 * closed,
            "{} vs {closed}",
            result.midpoint
        );
        assert!(
            result.value_lower <= closed * (1.0 + 1e-15)
                && result.upper_bound >= closed * (1.0 - 1e-15)
        );
        let psi = arc.phi - std::f64::consts::FRAC_PI_2;
        assert_eq!(result.location, ArcMaximumLocation::Bracket);
        assert!(
            (result.fraction * arc.phi - psi).abs() <= 1e-11,
            "{} vs {psi}",
            result.fraction * arc.phi
        );
    }
}

#[test]
fn every_evaluation_is_directed() {
    // No lower value above, and no upper value below, the 110-digit value of
    // the same represented problem (value and derivative).
    let arcs = oracle_arcs();
    let peaks = two_peak_variants();
    let mut checked = 0;
    for line in DIRECTED.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first() != Some(&"point") {
            continue;
        }
        let k: u64 = f[5].parse().unwrap();
        let mut evaluator = if f[1] == "TWO-PEAKS" {
            let variant = peaks.iter().find(|v| v.name == f[2]).unwrap();
            Evaluator::new(&variant.problem(), ArcStressMeasure::ElasticNormal).unwrap()
        } else {
            let arc = arcs
                .iter()
                .find(|a| a.case == f[1] && a.member == f[2] && a.basis == f[3])
                .unwrap();
            Evaluator::new(&arc.problem(), arc.measure(f[4])).unwrap()
        };
        let value = evaluator.value(k).unwrap();
        assert!(
            value.lo <= hex(f[6]) && value.hi >= hex(f[7]),
            "{line}: {value:?}"
        );
        // The enclosure is tight: within 1e-12 relative of the value (1e-60 Pa
        // absolute where the value is an exact zero).
        assert!(
            value.hi - value.lo <= 1e-12 * value.hi + 1e-60,
            "{line}: {value:?}"
        );
        if f[8] != "none" {
            let slope = evaluator.slope(k).unwrap();
            assert!(
                slope.lo <= hex(f[8]) && slope.hi >= hex(f[9]),
                "{line}: {slope:?}"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 30 * 9 + 3 * 9);
}

// ---------------------------------------------------------------------------
// Sine and cosine
// ---------------------------------------------------------------------------

#[test]
fn sin_cos_enclose_independent_constants() {
    let mut b = Balls::new().unwrap();
    let trig = Trig::new(&mut b).unwrap();
    let mut rows = HashMap::new();
    let mut count = 0;
    for line in TRIG.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(f.len(), 8, "{line}");
        let x = wide(f[1]);
        let (sin, cos) = trig.sin_cos(&mut b, Ball::exact(x)).unwrap();
        for (ball, lo, hi, decimal) in [(sin, f[4], f[5], f[2]), (cos, f[6], f[7], f[3])] {
            assert!(
                within_radius(&wide(lo), &ball.m, ball.r)
                    && within_radius(&wide(hi), &ball.m, ball.r),
                "{} {ball:?}",
                f[0]
            );
            assert!(
                ball.r <= f64::from_bits((1023 - 120) << 52),
                "{}: radius {}",
                f[0],
                ball.r
            );
            // The 60-digit decimal lies in the directed interval too.
            let interval = bounds(&ball).unwrap();
            let value: f64 = decimal.parse().unwrap();
            assert!(
                interval.lo <= value && value <= interval.hi,
                "{}: {value} {interval:?}",
                f[0]
            );
        }
        rows.insert(
            f[0].to_string(),
            (
                f[4].to_string(),
                f[5].to_string(),
                f[6].to_string(),
                f[7].to_string(),
            ),
        );
        count += 1;
    }
    assert_eq!(count, 23);
    // An argument ball: 1 ± 2⁻⁶⁰ encloses sin and cos at both extreme points.
    let ball = Ball {
        m: Wide2::ONE,
        r: f64::from_bits((1023 - 60) << 52),
    };
    let (sin, cos) = trig.sin_cos(&mut b, ball).unwrap();
    for name in ["one_minus_2^-60", "one", "one_plus_2^-60"] {
        let (slo, shi, clo, chi) = &rows[name];
        for (ball, token) in [(sin, slo), (sin, shi), (cos, clo), (cos, chi)] {
            assert!(within_radius(&wide(token), &ball.m, ball.r), "{name}");
        }
    }
    // The domain is [0, π] with the binary64 π.
    let pi = b.lift(std::f64::consts::PI).unwrap();
    assert!(trig.sin_cos(&mut b, pi).is_ok());
    let above = b.lift(std::f64::consts::PI.next_up()).unwrap();
    assert_eq!(
        trig.sin_cos(&mut b, above).unwrap_err(),
        ArcMaximumFailure::TrigArgumentOutsideDomain
    );
    let below = b.lift(-f64::from_bits(1)).unwrap();
    assert_eq!(
        trig.sin_cos(&mut b, below).unwrap_err(),
        ArcMaximumFailure::TrigArgumentOutsideDomain
    );
    let wide_pi = Ball { m: pi.m, r: 1e-30 };
    assert_eq!(
        trig.sin_cos(&mut b, wide_pi).unwrap_err(),
        ArcMaximumFailure::TrigArgumentOutsideDomain
    );
}

// ---------------------------------------------------------------------------
// Synthetic controls
// ---------------------------------------------------------------------------

const IDENTITY: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

struct Variant {
    name: String,
    radius: f64,
    phi: f64,
    w: Vec<ArcUniformIntensity>,
    force: [f64; 6],
    pa: f64,
    area: f64,
    z: f64,
    /// (fraction, value floor, value ceiling) of the left and right peaks.
    peaks: Vec<(f64, f64, f64)>,
}

impl Variant {
    fn problem(&self) -> ArcStressMaximumProblem<'_> {
        ArcStressMaximumProblem {
            bend_radius: self.radius,
            included_angle: self.phi,
            end_j_axes: IDENTITY,
            end_j_force: self.force,
            uniform_intensities: &self.w,
            pressure_area_product: Some(self.pa),
            area: self.area,
            section_modulus: self.z,
        }
    }
}

fn two_peak_variants() -> Vec<Variant> {
    let mut variants: Vec<Variant> = Vec::new();
    for line in TWO_PEAKS.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f[0] == "variant" {
            variants.push(Variant {
                name: f[1].to_string(),
                radius: 0.0,
                phi: 0.0,
                w: Vec::new(),
                force: [0.0; 6],
                pa: 0.0,
                area: 0.0,
                z: 0.0,
                peaks: Vec::new(),
            });
            continue;
        }
        let v = variants.last_mut().unwrap();
        let values: Vec<f64> = f[2..].iter().map(|t| hex(t)).collect();
        match (f[0], f.get(1).copied()) {
            ("operand", Some("R")) => v.radius = values[0],
            ("operand", Some("phi")) => v.phi = values[0],
            ("operand", Some("w")) => {
                v.w = vec![ArcUniformIntensity {
                    global: [values[0], values[1], values[2]],
                    scale: 1.0,
                }]
            }
            ("operand", Some("force")) => v.force.copy_from_slice(&values),
            ("operand", Some("pa")) => v.pa = values[0],
            ("operand", Some("area")) => v.area = values[0],
            ("operand", Some("z")) => v.z = values[0],
            ("peak", _) => v.peaks.push((hex(f[1]), hex(f[2]), hex(f[3]))),
            _ => panic!("{line}"),
        }
    }
    assert_eq!(variants.len(), 3);
    variants
}

#[test]
fn close_competing_peaks_and_ties() {
    for variant in two_peak_variants() {
        let result =
            certify_arc_stress_maximum(&variant.problem(), ArcStressMeasure::ElasticNormal, &[])
                .unwrap();
        let (left, right) = (variant.peaks[0], variant.peaks[1]);
        let expected = match variant.name.as_str() {
            // Equal peaks (symmetric arc; both on the grid): the smallest ψ.
            "tie" => left,
            // About 2e-9 relative apart, far above the gap: the higher one.
            "left_high" => left,
            "right_high" => right,
            other => panic!("{other}"),
        };
        let higher = left.2.max(right.2);
        assert!(
            result.value_lower <= higher && result.upper_bound >= expected.2,
            "{}: {result:?}",
            variant.name
        );
        assert_eq!(
            result.location,
            ArcMaximumLocation::Bracket,
            "{}",
            variant.name
        );
        let error = (result.fraction - expected.0).abs() * variant.phi;
        assert!(
            error <= 1e-11,
            "{}: fraction {} vs {}",
            variant.name,
            result.fraction,
            expected.0
        );
        println!(
            "{}: fraction {:.15} (expected {:.15}), value {:.17e}",
            variant.name, result.fraction, expected.0, result.midpoint
        );
    }
}

#[test]
fn bisection_stops_at_an_unresolved_sign() {
    // R/Z = 1/A exactly and N, M_z > 0: the large F_y parts of N/A and M_z/Z
    // cancel in the measure, which is (C + w_x sin Δ − w_y (1 − cos Δ))/A,
    // largest at Δ = π/4. Each part of f′ is about 1e7, so the directed
    // binary64 sum leaves f′'s sign unresolved within ~1e-8 rad of the peak,
    // which is wider than 2⁻⁴⁰·φ: the bisection stops there.
    let w = [ArcUniformIntensity {
        global: [1.0, 1.0, 0.0],
        scale: 1.0,
    }];
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 2.0,
        end_j_axes: IDENTITY,
        end_j_force: [0.0, 1.0e7, 0.0, 0.0, 0.0, 1.0e3],
        uniform_intensities: &w,
        pressure_area_product: Some(3.0e7),
        area: 1.0,
        section_modulus: 1.0,
    };
    let result =
        certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).unwrap();
    assert_eq!(
        result.location,
        ArcMaximumLocation::UnresolvedSign,
        "{result:?}"
    );
    let psi = 2.0 - std::f64::consts::FRAC_PI_4;
    assert!(
        (result.fraction * 2.0 - psi).abs() <= 1e-7,
        "{} vs {psi}",
        result.fraction * 2.0
    );
    let closed = 3.0e7 + 1.0e3 + (std::f64::consts::SQRT_2 - 1.0);
    assert!(
        (result.midpoint - closed).abs() <= 1e-12 * closed,
        "{} vs {closed}",
        result.midpoint
    );
}

#[test]
fn end_j_rule() {
    // f = |N|/A + |M_z|/Z with N = s|F|, M_z = M − R s|F| near end j and
    // R/Z > 1/A: f decreases away from j, and f′(φ) > 0.
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 2.0,
        end_j_axes: IDENTITY,
        end_j_force: [0.0, -100.0, 0.0, 0.0, 0.0, 1000.0],
        uniform_intensities: &[],
        pressure_area_product: None,
        area: 1e-3,
        section_modulus: 1e-4,
    };
    let result =
        certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).unwrap();
    assert_eq!(result.location, ArcMaximumLocation::EndJ);
    assert_eq!(result.fraction, 1.0);
    assert!(
        result.value_lower <= 1.0e7 && result.upper_bound >= 1.0e7,
        "{result:?}"
    );
}

fn zero_problem(scale: f64) -> ([f64; 6], Vec<ArcUniformIntensity>) {
    (
        [1.0, -2.0, 0.5, 3.0, -1.0, 2.0].map(|x| x * scale),
        vec![ArcUniformIntensity {
            global: [0.25 * scale, -0.5 * scale, 1.0 * scale],
            scale: 1.0,
        }],
    )
}

#[test]
fn zero_and_near_zero_arcs_close_under_the_absolute_floor() {
    for scale in [0.0, 1e-20] {
        let (force, w) = zero_problem(scale);
        let problem = ArcStressMaximumProblem {
            bend_radius: 0.5,
            included_angle: 1.5,
            end_j_axes: IDENTITY,
            end_j_force: force,
            uniform_intensities: &w,
            pressure_area_product: Some(0.0),
            area: 3e-3,
            section_modulus: 1e-4,
        };
        for measure in [
            ArcStressMeasure::ElasticNormal,
            ArcStressMeasure::DirectionalIntensified {
                in_plane: 2.0,
                out_of_plane: 1.5,
            },
        ] {
            let result = certify_arc_stress_maximum(&problem, measure, &[]).unwrap();
            assert!(result.upper_bound <= 1e-12, "{scale}: {result:?}");
            assert!(result.certified_gap <= 1e-12, "{scale}: {result:?}");
            if scale == 0.0 {
                assert_eq!(
                    (result.value_lower, result.upper_bound, result.midpoint),
                    (0.0, 0.0, 0.0)
                );
                assert_eq!(result.subdivisions, 0);
                assert_eq!(result.fraction, 0.0);
            } else {
                assert!(result.value_lower > 0.0, "{result:?}");
            }
        }
    }
}

fn arc106_nominal() -> OracleArc {
    oracle_arcs()
        .into_iter()
        .find(|a| a.case == "U4B-ARC106-RADIAL-NEGP" && a.basis == "nominal_section")
        .unwrap()
}

#[test]
fn limits_refuse_by_name() {
    assert_eq!(ARC_MAXIMUM_SUBDIVISION_LIMIT, 131_072);
    assert_eq!(ARC_MAXIMUM_DEPTH_LIMIT, 48);
    assert_eq!(ARC_MAXIMUM_INITIAL_CELLS, 4096);
    let arc = arc106_nominal();
    let expected = arc.elastic.value;
    let error = certify_with_limits(
        &arc.problem(),
        ArcStressMeasure::ElasticNormal,
        &[],
        3,
        ARC_MAXIMUM_DEPTH_LIMIT,
    )
    .unwrap_err();
    let ArcMaximumFailure::SubdivisionLimit(diagnostic) = error else {
        panic!("{error:?}")
    };
    assert_eq!(error.reason(), "arc_maximum_subdivision_limit");
    assert_eq!(diagnostic.subdivisions, 3);
    assert!(diagnostic.value_lower <= expected && diagnostic.upper_bound >= expected);
    let error = certify_with_limits(
        &arc.problem(),
        ArcStressMeasure::ElasticNormal,
        &[],
        ARC_MAXIMUM_SUBDIVISION_LIMIT,
        0,
    )
    .unwrap_err();
    assert!(
        matches!(error, ArcMaximumFailure::DepthLimit(_)),
        "{error:?}"
    );
    assert_eq!(error.reason(), "arc_maximum_depth_limit");
}

#[test]
fn station_cover_raises_the_bound_and_rechecks_the_gap() {
    let arc = arc106_nominal();
    let measure = ArcStressMeasure::ElasticNormal;
    let problem = arc.problem();
    let plain = certify_arc_stress_maximum(&problem, measure, &[]).unwrap();
    // The oracle's own station values lie below the bound: nothing changes.
    assert_eq!(
        certify_arc_stress_maximum(&problem, measure, &arc.elastic_stations).unwrap(),
        plain
    );
    // Above the bound but within the gap: raised, still certified.
    let tol = tolerance(plain.value_lower);
    let raised_value = plain.value_lower + 0.5 * tol;
    assert!(raised_value > plain.upper_bound);
    let raised = certify_arc_stress_maximum(&problem, measure, &[raised_value, 1.0]).unwrap();
    assert_eq!(raised.upper_bound, raised_value);
    assert!(raised.certified_gap <= tol);
    // Beyond the gap: unavailable by name.
    let error = certify_arc_stress_maximum(&problem, measure, &[plain.value_lower * (1.0 + 1e-9)])
        .unwrap_err();
    assert!(
        matches!(error, ArcMaximumFailure::StationCoverNotClosed(_)),
        "{error:?}"
    );
    assert_eq!(error.reason(), "arc_maximum_station_cover_not_closed");
    // A value below the station rows is not a refusal: the cover is upward only.
    assert_eq!(
        certify_arc_stress_maximum(&problem, measure, &[0.5 * plain.value_lower]).unwrap(),
        plain
    );
    let error = certify_arc_stress_maximum(&problem, measure, &[f64::NAN]).unwrap_err();
    assert_eq!(
        error,
        ArcMaximumFailure::InvalidOperand {
            name: "station_cover_pa"
        }
    );
}

#[test]
fn pressure_counted_twice_moves_the_maximum_off_the_true_one() {
    // N_w passed as the end-j axial together with pA_i counts the pressure
    // term twice. The kernel cannot see the operand error itself. Where it
    // raises |N_eff| the result lies above the true maximum by far more than
    // the criterion (PP's integration test, binding each arc maximum to its
    // station rows, catches it); where it lowers the maximum below the
    // station rows (ARC90-TIGHT, whose N_w changes sign along the arc), the
    // upward cover no longer closes and the maximum is refused.
    let (mut above, mut refused) = (Vec::new(), Vec::new());
    for arc in oracle_arcs().into_iter().filter(|a| a.pa.is_some()) {
        let mut force = arc.force;
        force[0] += arc.pa.unwrap();
        let doubled = ArcStressMaximumProblem {
            end_j_force: force,
            ..arc.problem()
        };
        let label = format!("{} {}", arc.case, arc.basis);
        match certify_arc_stress_maximum(
            &doubled,
            ArcStressMeasure::ElasticNormal,
            &arc.elastic_stations,
        ) {
            Ok(wrong) => {
                let expected = arc.elastic.value;
                let separation =
                    (wrong.value_lower - expected) / (1e-9 * expected.abs().max(arc.normal_scale));
                assert!(separation > 1e3, "{label}: {separation} tolerances");
                let station_max = arc.elastic_stations.iter().copied().fold(0.0, f64::max);
                assert!(wrong.value_lower > station_max * (1.0 + 1e-6), "{label}");
                above.push(label);
            }
            Err(ArcMaximumFailure::StationCoverNotClosed(_)) => refused.push(label),
            Err(other) => panic!("{label}: {other:?}"),
        }
    }
    assert_eq!(above.len(), 10, "{above:?}");
    assert_eq!(refused.len(), 2, "{refused:?}");
    assert!(
        refused
            .iter()
            .all(|l| l.starts_with("U4B-ARC90-TIGHT-NSIGN-P")),
        "{refused:?}"
    );
}

#[test]
fn codex_rotated_torsion_encloses_the_transposed_chain() {
    // R = A = Z = 1, φ = 1e-5, no loads, end-j action [0, 0, 0, 1e8, 0, 0] in
    // a rounded rotated frame: the maximum of the specified chain (E Eᵀ) is
    // 2.8e-9 below the direct-local one, more than the gap.
    let frame = [
        [0.7816391739070251, -0.4829292842142122, 0.3947397981737998],
        [
            -0.5501172307043584,
            -0.8320301337746346,
            0.07139249941787586,
        ],
        [
            0.29395787843858057,
            -0.2729563388883143,
            -0.9160150668873173,
        ],
    ];
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 1e-5,
        end_j_axes: frame,
        end_j_force: [0.0, 0.0, 0.0, 1.0e8, 0.0, 0.0],
        uniform_intensities: &[],
        pressure_area_product: None,
        area: 1.0,
        section_modulus: 1.0,
    };
    let line = |kind: &str| -> (f64, f64, f64) {
        let f: Vec<&str> = DIRECTED
            .lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .find(|f| f.first() == Some(&"codex") && f[1] == kind)
            .unwrap();
        (hex(f[2]), hex(f[3]), hex(f[4]))
    };
    let (fraction, floor, ceil) = line("transposed_chain");
    let (_, direct_floor, _) = line("direct_local");
    let result =
        certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).unwrap();
    assert!(
        result.value_lower <= floor && result.upper_bound >= ceil,
        "{result:?} vs [{floor}, {ceil}]"
    );
    assert!(
        result.upper_bound < direct_floor,
        "{result:?} must exclude the direct-local {direct_floor}"
    );
    assert_eq!(result.fraction, fraction);
    assert_eq!(result.location, ArcMaximumLocation::EndI);
}

#[test]
fn refuses_invalid_operands_by_name() {
    let arc = arc106_nominal();
    let base = arc.problem();
    let elastic = ArcStressMeasure::ElasticNormal;
    let cases: Vec<(ArcStressMaximumProblem<'_>, ArcStressMeasure, &str)> = vec![
        (
            ArcStressMaximumProblem {
                bend_radius: 0.0,
                ..base
            },
            elastic,
            "bend_radius",
        ),
        (
            ArcStressMaximumProblem {
                bend_radius: f64::NAN,
                ..base
            },
            elastic,
            "bend_radius",
        ),
        (
            ArcStressMaximumProblem {
                included_angle: 0.0,
                ..base
            },
            elastic,
            "included_angle",
        ),
        (
            ArcStressMaximumProblem {
                included_angle: std::f64::consts::PI.next_up(),
                ..base
            },
            elastic,
            "included_angle",
        ),
        (
            ArcStressMaximumProblem { area: 0.0, ..base },
            elastic,
            "area",
        ),
        (
            ArcStressMaximumProblem {
                section_modulus: -1.0,
                ..base
            },
            elastic,
            "section_modulus",
        ),
        (
            ArcStressMaximumProblem {
                end_j_force: [f64::INFINITY, 0.0, 0.0, 0.0, 0.0, 0.0],
                ..base
            },
            elastic,
            "end_j_force",
        ),
        (
            ArcStressMaximumProblem {
                pressure_area_product: Some(f64::NAN),
                ..base
            },
            elastic,
            "pressure_area_product",
        ),
        (
            base,
            ArcStressMeasure::DirectionalIntensified {
                in_plane: 0.0,
                out_of_plane: 1.0,
            },
            "in_plane_factor",
        ),
        (
            base,
            ArcStressMeasure::DirectionalIntensified {
                in_plane: 1.0,
                out_of_plane: f64::NAN,
            },
            "out_of_plane_factor",
        ),
    ];
    for (problem, measure, name) in cases {
        let error = certify_arc_stress_maximum(&problem, measure, &[]).unwrap_err();
        assert_eq!(error, ArcMaximumFailure::InvalidOperand { name });
        assert_eq!(error.reason(), "arc_maximum_invalid_operand");
    }
    // A semicircle (φ = the binary64 π) is admitted.
    let semicircle = ArcStressMaximumProblem {
        included_angle: std::f64::consts::PI,
        ..base
    };
    assert!(certify_arc_stress_maximum(&semicircle, elastic, &[]).is_ok());
}

#[test]
fn curvature_bound_is_the_designed_formula() {
    // K = (2R|w| + F_max)/A + (R²|w| + M_max + R F_max)/Z, and the
    // intensified K = max(i_ip, i_op)(R²|w| + M_max + R F_max)/Z, formed upward.
    let w = [ArcUniformIntensity {
        global: [3.0, 0.0, 4.0],
        scale: 2.0,
    }];
    let problem = ArcStressMaximumProblem {
        bend_radius: 2.0,
        included_angle: 1.0,
        end_j_axes: IDENTITY,
        end_j_force: [0.0, 6.0, 8.0, 0.0, 0.0, 5.0],
        uniform_intensities: &w,
        pressure_area_product: Some(1e6),
        area: 0.5,
        section_modulus: 0.25,
    };
    let (w_norm, f_max, m_max) = (10.0, 10.0 + 10.0 * 2.0, 5.0 + 2.0 * 30.0);
    let moment = 4.0 * w_norm + m_max + 2.0 * f_max;
    let elastic = Evaluator::new(&problem, ArcStressMeasure::ElasticNormal)
        .unwrap()
        .curvature_bound()
        .unwrap();
    let exact = (2.0 * 2.0 * w_norm + f_max) / 0.5 + moment / 0.25;
    assert!(
        elastic >= exact && elastic <= exact * (1.0 + 1e-14),
        "{elastic} vs {exact}"
    );
    let intensified = Evaluator::new(
        &problem,
        ArcStressMeasure::DirectionalIntensified {
            in_plane: 1.5,
            out_of_plane: 3.0,
        },
    )
    .unwrap()
    .curvature_bound()
    .unwrap();
    let exact = 3.0 * moment / 0.25;
    assert!(
        intensified >= exact && intensified <= exact * (1.0 + 1e-14),
        "{intensified} vs {exact}"
    );
}

struct EndControl {
    variant: Variant,
    kind: String,
    fraction: f64,
}

fn end_controls() -> Vec<EndControl> {
    let mut controls: Vec<EndControl> = Vec::new();
    for line in END_CONTROLS.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f[0] == "variant" {
            controls.push(EndControl {
                variant: Variant {
                    name: f[1].to_string(),
                    radius: 0.0,
                    phi: 0.0,
                    w: Vec::new(),
                    force: [0.0; 6],
                    pa: 0.0,
                    area: 0.0,
                    z: 0.0,
                    peaks: Vec::new(),
                },
                kind: String::new(),
                fraction: 0.0,
            });
            continue;
        }
        let c = controls.last_mut().unwrap();
        let v = &mut c.variant;
        if f[0] == "expect" {
            c.kind = f[1].to_string();
            c.fraction = hex(f[2]);
            continue;
        }
        let values: Vec<f64> = f[2..].iter().map(|t| hex(t)).collect();
        match f[1] {
            "R" => v.radius = values[0],
            "phi" => v.phi = values[0],
            "w" => {
                v.w = vec![ArcUniformIntensity {
                    global: [values[0], values[1], values[2]],
                    scale: 1.0,
                }]
            }
            "force" => v.force.copy_from_slice(&values),
            "pa" => v.pa = values[0],
            "area" => v.area = values[0],
            "z" => v.z = values[0],
            other => panic!("{other}"),
        }
    }
    assert_eq!(controls.len(), 2);
    controls
}

#[test]
fn location_follows_the_designed_order() {
    // (a) A governing end j pointing out publishes exactly 1, although an
    //     interior bracketed peak lies within the gap (1e-14 lower).
    // (b) A governing interior peak publishes its bracket, although end i,
    //     pointing out, lies within the gap (1e-14 lower).
    for control in end_controls() {
        let v = &control.variant;
        let result =
            certify_arc_stress_maximum(&v.problem(), ArcStressMeasure::ElasticNormal, &[]).unwrap();
        match control.kind.as_str() {
            "end_j" => {
                assert_eq!(
                    result.location,
                    ArcMaximumLocation::EndJ,
                    "{}: {result:?}",
                    v.name
                );
                assert_eq!(result.fraction, 1.0);
            }
            "interior" => {
                assert!(
                    matches!(
                        result.location,
                        ArcMaximumLocation::Bracket | ArcMaximumLocation::UnresolvedSign
                    ),
                    "{}: {result:?}",
                    v.name
                );
                assert!(
                    (result.fraction - control.fraction).abs() * v.phi <= 1e-11,
                    "{}: {result:?}",
                    v.name
                );
            }
            other => panic!("{other}"),
        }
        println!(
            "{}: {:?} at {:.15}",
            v.name, result.location, result.fraction
        );
    }
}

/// The slope fallbacks at exact zeros of their arguments: the |N_eff| term
/// is enclosed by ±|N′|/A where the N_eff enclosure contains 0, and the
/// bending term by ±|M′|/Z where the hypot enclosure reaches 0. An enclosure
/// that contains the exact real value contains 0 wherever that value is
/// exactly 0, so the fallback is taken there.
fn fallback_control(force: [f64; 6]) -> (ArcStressMaximumProblem<'static>, Evaluator) {
    static W: [ArcUniformIntensity; 1] = [ArcUniformIntensity {
        global: [2.0, 0.0, 0.0],
        scale: 1.0,
    }];
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 1.0,
        end_j_axes: IDENTITY,
        end_j_force: force,
        uniform_intensities: &W,
        pressure_area_product: None,
        area: 1.0,
        section_modulus: 1.0,
    };
    let evaluator = Evaluator::new(&problem, ArcStressMeasure::ElasticNormal).unwrap();
    (problem, evaluator)
}

#[test]
fn slope_fallbacks_at_isolated_interior_zeros() {
    // Codex's controls (K2 round 3): E = I, R = φ = 1, w = [2, 0, 0], no
    // pressure. At fraction 0.5, Δ = 1/2 exactly.
    let half = FULL / 2;
    let c = 0.5_f64.cos();
    // (1) End-j action [−1, 0, 0, 0, 0, 0]: N_eff = (2Δ − 1)cos Δ, an exact
    //     isolated zero at fraction 0.5, where |N′| = 2cos(1/2). The bending
    //     norm is not zero there, so the slope enclosure is the bending slope
    //     ± |N′|/A: width 4cos(1/2), up to rounding.
    let (problem, mut evaluator) = fallback_control([-1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let section = evaluator.section(half).unwrap();
    let n = bounds(&section.n_eff).unwrap();
    assert!(n.lo <= 0.0 && n.hi >= 0.0 && n.hi - n.lo < 1e-30, "{n:?}");
    let slope = evaluator.slope(half).unwrap();
    assert!(
        ((slope.hi - slope.lo) - 4.0 * c).abs() <= 1e-12,
        "{slope:?}"
    );
    assert!(certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).is_ok());
    // (2) End-j action [−1, −2, 0, 0, 0, 1]: M_y ≡ 0 and M_z = (1 − 2Δ)cos Δ,
    //     a bending-norm zero at fraction 0.5, where |M′| = |R V_y| = 2cos(1/2).
    //     N_eff = 2 sin(1/2) ≠ 0 there, so the slope enclosure is the axial
    //     slope ± |M′|/Z: width 4cos(1/2), up to rounding.
    let (problem, mut evaluator) = fallback_control([-1.0, -2.0, 0.0, 0.0, 0.0, 1.0]);
    let section = evaluator.section(half).unwrap();
    assert_eq!(bounds(&section.m_y).unwrap(), Interval { lo: 0.0, hi: 0.0 });
    let m_z = bounds(&section.m_z).unwrap();
    assert!(
        m_z.lo <= 0.0 && m_z.hi >= 0.0 && m_z.hi - m_z.lo < 1e-30,
        "{m_z:?}"
    );
    let n = bounds(&section.n_eff).unwrap();
    assert!((n.lo - 2.0 * 0.5_f64.sin()).abs() < 1e-15, "{n:?}");
    let slope = evaluator.slope(half).unwrap();
    assert!(
        ((slope.hi - slope.lo) - 4.0 * c).abs() <= 1e-12,
        "{slope:?}"
    );
    assert!(certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).is_ok());
}

#[test]
fn slope_fallbacks_on_structural_zeros() {
    // Structural cases besides the isolated zeros above:
    // (1) N_eff ≡ 0 on the whole arc (an out-of-plane end moment only): the
    //     |N_eff| fallback is taken at every slope evaluation, including every
    //     bracket and bisection point, and the bending peak at Δ = π/2 is
    //     still bracketed.
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 2.0,
        end_j_axes: IDENTITY,
        end_j_force: [0.0, 0.0, 0.0, 1000.0, 0.0, 0.0],
        uniform_intensities: &[],
        pressure_area_product: None,
        area: 1e-3,
        section_modulus: 1e-4,
    };
    let result =
        certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).unwrap();
    assert_eq!(result.location, ArcMaximumLocation::Bracket);
    let psi = 2.0 - std::f64::consts::FRAC_PI_2;
    assert!((result.fraction * 2.0 - psi).abs() <= 1e-11, "{result:?}");
    assert!(
        result.value_lower <= 1.0e7 && result.upper_bound >= 1.0e7,
        "{result:?}"
    );
    let mut evaluator = Evaluator::new(&problem, ArcStressMeasure::ElasticNormal).unwrap();
    let k = (result.fraction * (FULL as f64)) as u64;
    let section = evaluator.section(k).unwrap();
    assert_eq!(
        bounds(&section.n_eff).unwrap(),
        Interval { lo: 0.0, hi: 0.0 }
    );
    // (2) The bending norm is exactly zero at end j (M_j = 0); the hypot
    //     fallback ±|M′|/Z is taken there, and the axial slope still resolves
    //     f′(φ) > 0, so the governing end j publishes 1.
    let problem = ArcStressMaximumProblem {
        bend_radius: 1.0,
        included_angle: 1.5,
        end_j_axes: IDENTITY,
        end_j_force: [0.0, 100.0, 0.0, 0.0, 0.0, 0.0],
        uniform_intensities: &[],
        pressure_area_product: Some(1000.0),
        area: 1e-5,
        section_modulus: 1e-4,
    };
    let result =
        certify_arc_stress_maximum(&problem, ArcStressMeasure::ElasticNormal, &[]).unwrap();
    assert_eq!(result.location, ArcMaximumLocation::EndJ);
    assert_eq!(result.fraction, 1.0);
    assert!(
        result.value_lower <= 1.0e8 && result.upper_bound >= 1.0e8,
        "{result:?}"
    );
    let mut evaluator = Evaluator::new(&problem, ArcStressMeasure::ElasticNormal).unwrap();
    let section = evaluator.section(FULL).unwrap();
    assert_eq!(bounds(&section.m_y).unwrap(), Interval { lo: 0.0, hi: 0.0 });
    assert_eq!(bounds(&section.m_z).unwrap(), Interval { lo: 0.0, hi: 0.0 });
    let slope = evaluator.slope(FULL).unwrap();
    // Axial 100/1e-5 = 1e7 with the fallback ±R·100/1e-4 = ±1e6.
    assert!(
        slope.lo > 8.9e6 && slope.lo < 9.1e6 && slope.hi > 1.09e7 && slope.hi < 1.11e7,
        "{slope:?}"
    );
}
