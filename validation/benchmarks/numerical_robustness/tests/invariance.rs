//! RF-INVARIANCE's recorded observations (plan Q7): every row of every small
//! variant maps onto its BASE through the variant's transform. The differences
//! are recorded (`observations/kernel_lane/invariance.json`, written by
//! `vk_records --write`), never asserted against a tolerance: the gate is each
//! variant against its own R1 expectations (`tests/lane.rs`).
use piping_numerical_robustness::cases::load_family;
use piping_numerical_robustness::invariance::observations;
use piping_numerical_robustness::lane::run_case;

#[test]
fn every_small_variants_rows_map_onto_its_base() {
    let cases: Vec<_> = load_family("RF-INVARIANCE")
        .into_iter()
        .filter(|c| !c.id.contains("TREE100"))
        .collect();
    let runs: Vec<_> = cases.iter().map(run_case).collect();
    let obs = observations(&cases, &runs);
    assert_eq!(obs.len(), 18);
    for o in &obs {
        println!("{o}");
        assert_eq!(o["rows_unmatched"], 0, "{o}");
        assert!(o["rows_compared"].as_u64().unwrap() > 0, "{o}");
    }
}
