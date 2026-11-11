//! W1-E's product-side confirmation census (DEL-04-08 Design
//! `w1e-sharper-exact-coverage.md` §6.2): O4's frozen judged set and source-control
//! twins, plain and augmented, in both solver modes, through the actual W1 private
//! driver, with the kernel probe recording every judged row's actual verdict and its
//! counterfactual judgements (§6.2's (a), (d), (d'), (d''), (e), (e') and the
//! any-value and single-readout columns of §3.2), and, for an input with a failing
//! row, the lane counterfactuals (b) and (c) re-run whole.
//!
//! Each judged row also carries the premise check of §2.2: whether the published
//! value is the method's projection of the readouts' tight midpoint, formed here from
//! the frozen references (double-double, as M3-O4 reads them; a midpoint within
//! 1e-6 of a unit of its rounding boundary is "undecided").
//!
//! `census-probe` only; every target is `#[ignore]`. `W1_CENSUS_OUT` receives one
//! JSON line per document, binding and mode. Nothing is written under
//! `validation/references` or the fixtures. Standard claim fence applies (F-PIP-2;
//! claims taxonomy per DEC-081).
use super::*;
#[path = "w1_census_driver.rs"]
mod driver;
use driver::*;
use open_pipe_stress_frame_kernel::structural::retained_api::census_probe as probe;

/// RN64 of a double-double, with whether it is decided (not within 1e-6 ulp of a tie).
fn rn(x: Dd) -> (f64, bool) {
    let y = x.hi + x.lo;
    if x.hi == 0.0 && x.lo == 0.0 {
        return (0.0, true);
    }
    // The gap to y as a fraction of y's unit; a tie sits at one half.
    let up = f64::from_bits(y.abs().to_bits() + 1) - y.abs();
    let gap = Dd::from(y).sub(x).abs().div(Dd::from(up)).f();
    (y, (gap - 0.5).abs() > 1e-6)
}
/// The method's projection of an SI value into the row's published unit (DEF-E
/// `projection.H`): mm = RN64(1000·m), MPa = RN64(RN64(m)/10^6), else RN64(m).
fn project(m: Dd, unit: &str) -> (f64, bool) {
    match unit {
        "mm" => rn(m.mul(Dd::from(1000.0))),
        "MPa" => { let (si, ok) = rn(m); (si / 1.0e6, ok) }
        _ => rn(m),
    }
}
/// The premise of §2.2 for every gate row of the case: "same_cell", "other_cell",
/// "undecided" or "unmapped", from the ordinary envelope's row identities.
fn premises(records: &[Value], ordinary: &Value, reference_case: &Value) -> serde_json::Map<String, Value> {
    let rows: BTreeMap<&str, &Value> = ordinary["results"].as_array().map(|a| a.iter().filter_map(|r| Some((r["id"].as_str()?, r))).collect()).unwrap_or_default();
    let readouts = &reference_case["readouts"];
    let mut out = serde_json::Map::new();
    for r in records.iter().filter(|r| r["kind"] == "gate_row") {
        let id = r["id"].as_str().unwrap();
        let verdict = (|| -> Option<&'static str> {
            let row = rows.get(id)?;
            if row["entity_ref"].as_str().unwrap_or("").starts_with("o4trig:") { return None; }
            let unit = row["unit"].as_str()?;
            let (mut lo, mut hi): (Option<Dd>, Option<Dd>) = (None, None);
            for readout in ["annular", "admitted"] {
                let (a, b) = match reference(row, &readouts[readout])? { Exact::Point(p) => (p, p), Exact::Interval(a, b) => (a, b) };
                lo = Some(match lo { Some(l) if l.lt(a) => l, _ => a });
                hi = Some(match hi { Some(h) if b.lt(h) => h, _ => b });
            }
            let mid = lo?.add(hi?).mul(Dd::from(0.5));
            let (p, decided) = project(mid, unit);
            let y = f64::from_bits(r["value_bits"].as_u64()?);
            Some(if !decided { "undecided" } else if p.to_bits() == y.to_bits() || (p == 0.0 && y == 0.0) { "same_cell" } else { "other_cell" })
        })();
        out.insert(id.to_owned(), json!(verdict.unwrap_or("unmapped")));
    }
    out
}

fn census(doc: &Doc, binding: &str, raw: &Value, mode: PreviewSolverMode) -> Value {
    let sw = |same_section, same_readout| probe::Switches { corner1: true, same_section, same_readout, lane_null: false, record: true };
    let started = std::time::Instant::now();
    let (today, candidate, ordinary) = run_once(raw, mode, sw(false, false), &|| {});
    let records = today["records"].as_array().cloned().unwrap_or_default();
    let failing = records.iter().any(|r| r["kind"] == "gate_row" && r["passed"] == false);
    let mut out = json!({"id": doc.id, "design": doc.design, "theory": doc.theory, "variant": doc.variant, "twin": doc.twin,
        "binding": binding, "mode": mode.as_str(), "today": today, "premise": premises(&records, &ordinary, doc.reference)});
    if candidate.is_none() || !records.iter().any(|r| r["kind"] == "gate_case") {
        out["causes"] = w1_causes(raw, mode);
    }
    if failing {
        out["same_section"] = run_once(raw, mode, sw(true, false), &|| {}).0;
        out["same_readout"] = run_once(raw, mode, sw(false, true), &|| {}).0;
    }
    out["seconds"] = json!(started.elapsed().as_secs_f64());
    out
}

fn census_designs(target: &str, designs: &[&str], twins: bool) {
    let docs = if twins { twin_docs() } else { judged_docs() };
    let mut n = 0;
    for doc in docs.iter().filter(|d| designs.iter().any(|p| d.design.starts_with(p))).take(limit()) {
        let mut bindings = vec![("plain", doc.raw.clone())];
        if let Some(raw) = augmented(doc) { bindings.push(("augmented", raw)); }
        for (binding, raw) in bindings {
            for mode in MODES {
                let line = census(doc, binding, &raw, mode);
                println!("W1E_CENSUS {target} {}|{binding}|{} {:.1}s {}", doc.id, mode.as_str(), line["seconds"].as_f64().unwrap(), line["today"]["fallback"]);
                write_line(target, &line);
                n += 1;
            }
        }
    }
    println!("W1E_CENSUS_DONE {target} runs={n}");
}

macro_rules! w1e_census {
    ($($name:ident: [$($design:literal),+], $twins:expr;)+) => {$(
        #[test]
        #[ignore = "W1-E confirmation census, one design group (run with --features census-probe --ignored)"]
        fn $name() {
            census_designs(stringify!($name), &[$($design),+], $twins);
        }
    )+};
}
w1e_census! {
    w1e_census_cantilevers: ["CANT-"], false;
    w1e_census_phi2: ["PROP-PHI2-"], false;
    w1e_census_fixed_runs: ["FF-"], false;
    w1e_census_lines: ["LLINE-", "ZLINE"], false;
    w1e_census_portal_guided_beam: ["UPORTAL", "GUIDED", "CONT-BEAM"], false;
    w1e_census_skew: ["SKEW345-", "SKEW122-"], false;
    w1e_census_rflike: ["RFLIKE-"], false;
    w1e_census_soft_run: ["SOFT-RUN-"], false;
    w1e_census_soft_torsion: ["SOFT-TORSION-"], false;
    w1e_census_contrast_short: ["CONTRAST-STIFF", "SHORT-MEMBER"], false;
    w1e_census_caps: ["CAPS-LOOP-32"], false;
    w1e_census_twins_j2_physics: ["J2-physics_source-"], true;
    w1e_census_twins_j2_load_reference: ["J2-load_reference_source-"], true;
    w1e_census_twins_rf: ["RF-"], true;
}
