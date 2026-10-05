// RV77 independent enumeration (reviewer-owned; not part of the candidate).
// Included into final_case.rs only in RV77's private archive copy, as
//   #[cfg(test)] #[path = "<scratch>/rv77_enum.rs"] mod rv77_enum;
// The oracle is RV77's own row-level transcription of native
// summary_coverage_data (final_case.rs at c618675e84, lines 1461-1515), written
// from the source, not from I61's tests or rederive_coverage.
use super::*;

#[derive(Clone, Copy)]
struct Row { kind: usize, input: bool, nonzero: bool }

/// RV77 model of native summary_coverage_data for one body.
fn native_model(body: u32, rows: &[Row], extent: f64, e: [f64; 2], floor: Option<[f64; 2]>, p: u32, has_data: bool)
    -> ProductSummaryCoverage {
    let mut positive = [false; 4];
    let mut present = [false; 4];
    for r in rows {
        present[r.kind] = true;
        if !r.input && r.nonzero { positive[r.kind] = true; }
    }
    if extent != 0.0 {
        positive = [positive[0] || positive[1], positive[0] || positive[1], positive[2] || positive[3], positive[2] || positive[3]];
    }
    if let Some(f) = floor { positive[2] |= f[0] > 0.0; positive[3] |= f[1] > 0.0; }
    let mut stop = [false; 4];
    for r in rows { stop[r.kind] |= positive[r.kind] || r.nonzero; }
    let mut hats = [e[0] > 0.0, e[1] > 0.0];
    if extent != 0.0 { hats = [hats[0] || hats[1]; 2]; }
    let estimate = [present[2] && hats[0], present[3] && hats[1]];
    let charge = if p == 512 { [present[2] && positive[2], present[3] && positive[3]] } else { estimate };
    ProductSummaryCoverage { body, stop, estimate, charge, has_data }
}

fn facts_of(rows: &[Row], extent: f64, e: [f64; 2], floor: Option<[f64; 2]>, p: u32) -> CoverageFacts {
    let mut present = [false; 4];
    for r in rows { present[r.kind] = true; }
    CoverageFacts { present, extent_bits: extent.to_bits(), resolution_bits: [e[0].to_bits(), e[1].to_bits()],
        floor_bits: floor.map(|f| [f[0].to_bits(), f[1].to_bits()]), precision: p }
}

/// Row types per kind: bit0 non-input zero, bit1 non-input nonzero, bit2 input zero, bit3 input nonzero.
fn rows_of(masks: [u8; 4]) -> Vec<Row> {
    let mut v = Vec::new();
    for k in 0..4 {
        for t in 0..4 {
            if masks[k] & (1 << t) != 0 { v.push(Row { kind: k, input: t >= 2, nonzero: t % 2 == 1 }); }
        }
    }
    v
}

const EXTENTS: [f64; 3] = [0.0, 2.0, 5e-324];
const ES: [f64; 2] = [0.0, 1.0];
fn modes() -> Vec<(u32, Option<[f64; 2]>)> {
    let mut m = vec![(128, None), (256, None)];
    for f in [0.0, 3.0] { for g in [0.0, 3.0] { m.push((512, Some([f, g]))); } }
    m
}

#[test]
fn rv77_genuine_domain_rederives_bit_for_bit_and_is_never_refused() {
    // Genuine domain: recover::layout marks only displacement rows input-derived,
    // so force/moment masks use only bits 0..1.
    let (mut cases, mut p512, mut differ_est_charge) = (0u64, 0u64, 0u64);
    for m0 in 0..16u8 { for m1 in 0..16u8 { for m2 in 0..4u8 { for m3 in 0..4u8 {
        let rows = rows_of([m0, m1, m2, m3]);
        for &x in &EXTENTS { for &ef in &ES { for &em in &ES { for (p, floor) in modes() { for hd in [false, true] {
            let native = native_model(7, &rows, x, [ef, em], floor, p, hd);
            let facts = facts_of(&rows, x, [ef, em], floor, p);
            let payload = CoverageBody { body: 7, stop: native.stop, has_data: native.has_data };
            let got = rederive_coverage(payload, &facts)
                .unwrap_or_else(|e| panic!("genuine refused {:?} rows={m0},{m1},{m2},{m3} x={x} e={ef},{em} p={p} floor={floor:?}", e.category()));
            assert_eq!(got, native, "rows={m0},{m1},{m2},{m3} x={x} e={ef},{em} p={p} floor={floor:?}");
            // Tampering any derivable bit is detected by the equality the seam uses.
            for bit in 0..4 {
                let mut t = native;
                if bit < 2 { t.estimate[bit] ^= true; } else { t.charge[bit - 2] ^= true; }
                assert_ne!(got, t);
            }
            if p == 512 { p512 += 1; if native.estimate != native.charge { differ_est_charge += 1; } }
            cases += 1;
        }}}}}
    }}}}
    println!("RV77_GENUINE cases={cases} p512={p512} p512_estimate_ne_charge={differ_est_charge} refusals=0 mismatches=0");
    assert_eq!(cases, 16 * 16 * 4 * 4 * 3 * 2 * 2 * 6 * 2);
}

#[test]
fn rv77_full_payload_domain_matches_independent_spec() {
    // Every compact payload (16 stop patterns x has_data) against every public-fact
    // pattern, including invalid precision/floor pairings. RV77's spec of the
    // candidate's contract: refuse iff (p,floor) invalid, or a stop on an absent
    // kind, or a present force/moment kind with positive floor and no stop.
    let mut precisions: Vec<(u32, Option<[f64; 2]>)> = modes();
    for p in [64u32, 1024, 512] { precisions.push((p, None)); }
    for p in [128u32, 256, 1024] { precisions.push((p, Some([0.0, 0.0]))); }
    let (mut ok, mut refused) = (0u64, 0u64);
    for present_mask in 0..16u8 {
        let present = [present_mask & 1 != 0, present_mask & 2 != 0, present_mask & 4 != 0, present_mask & 8 != 0];
        for &x in &EXTENTS { for &ef in &ES { for &em in &ES { for &(p, floor) in &precisions {
            for stop_mask in 0..16u8 { for hd in [false, true] {
                let stop = [stop_mask & 1 != 0, stop_mask & 2 != 0, stop_mask & 4 != 0, stop_mask & 8 != 0];
                let facts = CoverageFacts { present, extent_bits: x.to_bits(), resolution_bits: [ef.to_bits(), em.to_bits()],
                    floor_bits: floor.map(|f| [f[0].to_bits(), f[1].to_bits()]), precision: p };
                let r = rederive_coverage(CoverageBody { body: 1, stop, has_data: hd }, &facts);
                let valid_mode = matches!((p, floor), (128 | 256, None) | (512, Some(_)));
                let absent_stop = (0..4).any(|k| stop[k] && !present[k]);
                let floor_unforced = floor.is_some_and(|f| (0..2).any(|k| present[2 + k] && f[k] > 0.0 && !stop[2 + k]));
                let expect_refuse = !valid_mode || absent_stop || floor_unforced;
                match r {
                    Err(e) => { assert!(expect_refuse, "unexpected refusal"); assert_eq!(e.category(), "association"); refused += 1; }
                    Ok(c) => {
                        assert!(!expect_refuse, "missing refusal p={p} floor={floor:?} present={present:?} stop={stop:?}");
                        let mut hats = [ef > 0.0, em > 0.0];
                        if x != 0.0 { hats = [hats[0] || hats[1]; 2]; }
                        let estimate = [present[2] && hats[0], present[3] && hats[1]];
                        let charge = if p == 512 { [stop[2], stop[3]] } else { estimate };
                        assert_eq!(c, ProductSummaryCoverage { body: 1, stop, estimate, charge, has_data: hd });
                        ok += 1;
                    }
                }
            }}
        }}}}
    }
    println!("RV77_PAYLOAD_SPEC accepted={ok} refused={refused}");
}

#[test]
fn rv77_input_derived_force_moment_would_break_the_p512_identity() {
    // Shows the coverage_facts refusal of input-derived force/moment rows is
    // necessary: if such rows existed, native charge (present && positive) could
    // differ from stop at p512. recover::layout never produces them.
    let mut breaks = 0u64;
    for m2 in 0..16u8 { for m3 in 0..16u8 {
        let rows = rows_of([1, 1, m2, m3]);
        for &x in &EXTENTS { for f in [[0.0, 0.0], [3.0, 0.0]] {
            let native = native_model(0, &rows, x, [1.0, 1.0], Some(f), 512, true);
            if native.charge != [native.stop[2], native.stop[3]] { breaks += 1; }
        }}
    }}
    println!("RV77_INPUT_DERIVED_FM p512_identity_breaks={breaks}");
    assert!(breaks > 0);
}
