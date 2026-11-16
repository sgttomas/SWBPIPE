//! DEL-04-10 AC-002 ("each interpolation rule"), for unit S2: the product's table evaluation
//! against the frozen closed-form references `spectrum_tables.json` (item 1) and the spectral
//! ordinates of `sdof.json` (item 2), at REQ-008's verification tolerance, 1e-9 relative.
//!
//! Every value compared is a positive ordinate of at least 0.098 m/s², so no absolute floor near
//! zero applies. On a tabulated point the ordinate is compared bit for bit: the product returns
//! the tabulated value, and a g multiple times g is one correctly rounded product, the nearest
//! binary64 to the exact value the reference gives.
//!
//! On a period table the primary reading (interpolation on the tabulated axis, the
//! reference's "interpolated on the tabulated axis" entries and `reading: "tabulated"`) is the
//! product's (decision 1). The alternative reading (periods converted to frequencies) is not
//! asserted as agreement; its interior values are shown to differ from the product's by far more
//! than the tolerance, so the test would notice a product that interpolated on the derived axis.
use open_pipe_stress_response_spectrum::{
    Abscissa, AxisScale, Interpolation, Location, OrdinateKind, QuantityInput, QueryFrequency, RecordUnits,
    RefusalCode, ScaleFactor, SpectrumRecord, SpectrumTable,
};
use serde_json::Value;

const TABLES: &str = include_str!("../../../../validation/references/f2_spectrum/closed_form/spectrum_tables.json");
const SDOF: &str = include_str!("../../../../validation/references/f2_spectrum/closed_form/sdof.json");
const TOLERANCE: f64 = 1e-9;

fn binary64(v: &Value) -> f64 {
    v["binary64"].as_str().expect("binary64 string").parse().expect("a binary64 decimal")
}

/// The 40-digit decimal of a derived real, parsed to its nearest binary64.
fn value(v: &Value) -> f64 {
    v["value"].as_str().expect("value string").parse().expect("a decimal")
}

fn scale(name: &str) -> AxisScale {
    match name {
        "linear" => AxisScale::Linear,
        "log" => AxisScale::Log,
        other => panic!("unknown rule {other}"),
    }
}

fn table_record(table: &Value, interpolation: Interpolation) -> SpectrumRecord {
    let period = table["abscissa"].as_str().unwrap().starts_with("period");
    let g = table.get("g_m_per_s2").map(binary64);
    let points = table["abscissae"].as_array().unwrap().iter().zip(table["ordinates"].as_array().unwrap())
        .map(|(x, y)| [binary64(x), binary64(y)]).collect();
    SpectrumRecord {
        id: table["id"].as_str().unwrap().into(),
        label: None,
        abscissa: if period { Abscissa::Period } else { Abscissa::Frequency },
        ordinate: if g.is_some() { OrdinateKind::GMultiple } else { OrdinateKind::Acceleration },
        units: RecordUnits {
            abscissa: if period { "s" } else { "Hz" }.into(),
            ordinate: if g.is_some() { None } else { Some("m/s^2".into()) },
        },
        g_value: g.map(|value| QuantityInput { value, unit: "m/s^2".into() }),
        damping_ratio: binary64(&table["damping_ratio"]),
        interpolation,
        points,
        tags: None,
        notes: None,
    }
}

fn relative(actual: f64, expected: f64) -> f64 {
    ((actual - expected) / expected).abs()
}

#[derive(Default)]
struct Tally {
    compared: usize,
    largest: f64,
}

impl Tally {
    fn check(&mut self, what: &str, actual: f64, expected: f64) {
        let r = relative(actual, expected);
        assert!(r <= TOLERANCE, "{what}: {actual} against {expected}, relative {r:e}");
        self.compared += 1;
        self.largest = self.largest.max(r);
    }
}

#[test]
fn every_rule_reproduces_spectrum_tables_on_the_authored_axis() {
    let doc: Value = serde_json::from_str(TABLES).unwrap();
    let s = ScaleFactor::new(binary64(&doc["scale_factor_s"])).unwrap();
    let mut tally = Tally::default();
    let (mut primary, mut alternative_interiors, mut refused, mut knots) = (0, 0, 0, 0);
    for table in doc["tables"].as_array().unwrap() {
        for evaluation in table["evaluations"].as_array().unwrap() {
            let interpolation = Interpolation {
                abscissa: scale(evaluation["abscissa_rule"].as_str().unwrap()),
                ordinate: scale(evaluation["ordinate_rule"].as_str().unwrap()),
            };
            let product = SpectrumTable::admit(&table_record(table, interpolation)).expect("the reference table is admitted");
            let reading = evaluation.get("reading").and_then(Value::as_str).unwrap_or("frequency table");
            let alternative = reading.starts_with("tabulated periods converted to frequencies");
            for row in evaluation["rows"].as_array().unwrap() {
                let f = QueryFrequency::new(binary64(&row["query_frequency_hz"]), "Hz").unwrap();
                let at = format!("{} {} ({reading}) f = {} Hz", table["id"], evaluation["rule"], f.hertz());
                let outcome = product.evaluate(f, s);
                match row["where"].as_str().unwrap() {
                    "outside" => {
                        assert_eq!(outcome.unwrap_err().code, RefusalCode::FrequencyOutsideRange, "{at}");
                        refused += 1;
                    }
                    "tabulated" => {
                        let ordinate = outcome.unwrap();
                        assert!(matches!(ordinate.location, Location::Knot { .. }), "{at}: {:?}", ordinate.location);
                        assert_eq!(ordinate.pseudo_acceleration_m_per_s2.to_bits(), binary64(&row["S_m_per_s2"]).to_bits(), "{at}");
                        tally.check(&at, ordinate.scaled_pseudo_acceleration_m_per_s2, value(&row["sS_m_per_s2"]));
                        knots += 1;
                    }
                    "inside" if alternative => {
                        let ordinate = outcome.unwrap();
                        let r = relative(ordinate.pseudo_acceleration_m_per_s2, value(&row["S_m_per_s2"]));
                        assert!(r > 1e-3, "{at}: the derived-axis reading should differ, relative {r:e}");
                        alternative_interiors += 1;
                    }
                    "inside" => {
                        let ordinate = outcome.unwrap();
                        assert!(matches!(ordinate.location, Location::Segment { .. }), "{at}: {:?}", ordinate.location);
                        if let Some(multiple) = row.get("S_g_multiple") {
                            tally.check(&format!("{at} g multiple"), ordinate.interpolated, value(multiple));
                        }
                        tally.check(&format!("{at} S"), ordinate.pseudo_acceleration_m_per_s2, value(&row["S_m_per_s2"]));
                        tally.check(&format!("{at} sS"), ordinate.scaled_pseudo_acceleration_m_per_s2, value(&row["sS_m_per_s2"]));
                        primary += 1;
                    }
                    other => panic!("{at}: unknown row kind {other}"),
                }
            }
        }
    }
    // T1 and T2 each under four rules, plus T2's two alternative readings: 10 evaluations of 11 rows.
    assert_eq!((knots, primary, refused, alternative_interiors), (50, 32, 20, 8));
    eprintln!("spectrum_tables.json: {} values compared, largest relative difference {:e}", tally.compared, tally.largest);
}

#[test]
fn sdof_spectral_ordinates_under_every_rule_and_the_primary_reading() {
    let doc: Value = serde_json::from_str(SDOF).unwrap();
    let tables: Value = serde_json::from_str(TABLES).unwrap();
    let s = ScaleFactor::new(binary64(&doc["inputs"]["scale_factor_s"])).unwrap();
    // The oscillator's frequency is 20/π Hz; the product receives its nearest binary64, as it
    // would from a modal solve. The ordinate's sensitivity to that rounding is far below 1e-9.
    let f = QueryFrequency::new(binary64(&doc["derived"]["frequency_hz"]), "Hz").unwrap();
    let mut tally = Tally::default();
    let mut alternative = 0;
    for case in doc["cases"].as_array().unwrap() {
        let table = tables["tables"].as_array().unwrap().iter().find(|t| t["id"] == case["table"]).unwrap();
        let rule = case["rule"].as_str().unwrap();
        let (a, o) = rule.split_once('/').unwrap();
        let interpolation = Interpolation {
            abscissa: scale(a.trim_end_matches("-abscissa")),
            ordinate: scale(o.trim_end_matches("-ordinate")),
        };
        let ordinate = SpectrumTable::admit(&table_record(table, interpolation)).unwrap().evaluate(f, s).unwrap();
        let at = format!("sdof {} {rule} reading {}", case["table"], case["reading"]);
        if case["reading"] == "frequency" {
            let r = relative(ordinate.pseudo_acceleration_m_per_s2, value(&case["S_m_per_s2"]));
            assert!(r > 1e-3, "{at}: the derived-axis reading should differ, relative {r:e}");
            alternative += 1;
        } else {
            assert_eq!(case["reading"], "tabulated", "{at}");
            tally.check(&format!("{at} S"), ordinate.pseudo_acceleration_m_per_s2, value(&case["S_m_per_s2"]));
            tally.check(&format!("{at} sS"), ordinate.scaled_pseudo_acceleration_m_per_s2, value(&case["sS_m_per_s2"]));
        }
    }
    assert_eq!((tally.compared, alternative), (16, 2));
    eprintln!("sdof.json: {} values compared, largest relative difference {:e}", tally.compared, tally.largest);
}
