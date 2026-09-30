//! Runs the kernel lane over every CI-scale case and writes
//! `observations/kernel_lane/<family>.json` (the committed per-case records,
//! plan §12), `invariance.json` (RF-INVARIANCE's recorded cross-variant
//! observations, plan Q7) and `parity.json` (the binary64 gate's parity on
//! RF-LARGE and RF-MECH, plan §5.3), and prints each family's report. Regenerating the records is an
//! explicit decision (the CI test compares them and never writes them).
//!
//! Usage: cargo run --release --example vk_records -- [--write] [family ...]
use piping_numerical_robustness::cases::{crate_dir, load_family, FAMILY_FILES};
use piping_numerical_robustness::compare::{ControlTally, Tally};
use piping_numerical_robustness::invariance::observations;
use piping_numerical_robustness::lane::run_case;
use piping_numerical_robustness::parity::parity;
use piping_numerical_robustness::records::family_file;
use piping_numerical_robustness::sha256::sha256_hex;
use serde_json::json;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let write = args.iter().any(|a| a == "--write");
    // `--show=<case id>` prints that case's record.
    let show: Vec<&str> = args
        .iter()
        .filter_map(|a| a.strip_prefix("--show="))
        .collect();
    let only: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let dir = crate_dir().join("observations/kernel_lane");
    let mut invariance: Vec<serde_json::Value> = Vec::new();
    let mut parity_rows: Vec<serde_json::Value> = Vec::new();
    for (family, file) in FAMILY_FILES {
        if !only.is_empty() && !only.iter().any(|f| *f == family) {
            continue;
        }
        let t0 = Instant::now();
        let mut tally = Tally::default();
        let mut controls = ControlTally::default();
        let mut records = Vec::new();
        let mut failures = Vec::new();
        let mut outcomes = std::collections::BTreeMap::new();
        let cases: Vec<_> = load_family(family)
            .into_iter()
            .filter(|c| !c.is_large())
            .collect();
        let mut runs = Vec::new();
        for case in &cases {
            let t = Instant::now();
            let run = run_case(case);
            eprintln!(
                "{} {} {:?} {:.2}s",
                run.id,
                run.outcome,
                run.selected_precision,
                t.elapsed().as_secs_f64()
            );
            tally.merge(&run.tally);
            controls.merge(&run.controls);
            failures.extend(run.failures.iter().cloned());
            failures.extend(run.class_mismatches.iter().map(|m| format!("CLASS {m}")));
            if run.not_covered != case.not_covered {
                failures.push(format!(
                    "FLOOR {}: {:?} committed {:?}",
                    run.id, run.not_covered, case.not_covered
                ));
            }
            *outcomes
                .entry(format!("{} {:?}", run.outcome, run.selected_precision))
                .or_insert(0usize) += 1;
            if show.iter().any(|s| *s == &run.id) {
                println!("{}", serde_json::to_string_pretty(&run.record).unwrap());
            }
            records.push(run.record.clone());
            runs.push(run);
        }
        if family == "RF-INVARIANCE" {
            invariance = observations(&cases, &runs);
            for o in &invariance {
                println!("{family}: {o}");
            }
        }
        if family == "RF-LARGE" || family == "RF-MECH" {
            for c in &cases {
                let model = c.model.as_ref().unwrap();
                // Dense to 105 members (RF-MECH-DISC's 103 included); the
                // time is printed, never recorded.
                let t = Instant::now();
                let p = parity(model, model.members.len() <= 105).unwrap();
                eprintln!("{} parity {:.1}s", c.id, t.elapsed().as_secs_f64());
                let row = json!({
                    "case": c.id, "sparse": p.sparse, "dense": p.dense,
                    "bitwise_k_equal": p.bitwise_k.as_ref().map(|m| m.is_none()),
                    "dec053_within": p.delta.map(|d| d.0),
                    "dec053_ratio_in_allowances": p.delta.map(|d| d.1),
                    "both_passed": p.both_passed,
                });
                println!("{family}: parity {row}");
                parity_rows.push(row);
            }
        }
        println!("{family}: {tally}");
        println!("{family}: outcomes {outcomes:?}");
        println!(
            "{family}: controls discriminated {}, non-discriminating {}, undiscriminated {:?}, unexpectedly failing {:?}",
            controls.discriminated, controls.non_discriminating, controls.undiscriminated, controls.unexpectedly_failing
        );
        for f in &failures {
            println!("{family}: FAILURE {f}");
        }
        println!("{family}: {:.1}s", t0.elapsed().as_secs_f64());
        if write {
            std::fs::create_dir_all(&dir).unwrap();
            let name = file.replace(".jsonl", ".json");
            std::fs::write(dir.join(name), family_file(&records)).unwrap();
        }
    }
    if write {
        if !invariance.is_empty() {
            std::fs::write(dir.join("invariance.json"), family_file(&invariance)).unwrap();
        }
        if !parity_rows.is_empty() {
            std::fs::write(dir.join("parity.json"), family_file(&parity_rows)).unwrap();
        }
        // SHA256SUMS over every committed record file.
        let mut names: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".json"))
            .collect();
        names.sort();
        let sums: String = names
            .iter()
            .map(|n| {
                format!(
                    "{}  {n}\n",
                    sha256_hex(&std::fs::read(dir.join(n)).unwrap())
                )
            })
            .collect();
        std::fs::write(dir.join("SHA256SUMS"), sums).unwrap();
    }
}
