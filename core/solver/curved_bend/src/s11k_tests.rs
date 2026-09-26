//! S11-K test K9 (S11 section 9): the E11 terms API. The section function is
//! odd in each input (every operation is sign-symmetric), so f(-G) = -f(G)
//! exactly and the exact sum of f(G), f(n), f(-G) is f(n): the expected value
//! is the section value of the combined input n, hand-derived, not a value the
//! terms API produced. The precondition asserts today's fold differs.
use super::*;

fn bend() -> CurvedBendMacroElement {
    // Quarter circle, radius 2 m, invented stiffness data.
    CurvedBendMacroElement::new(
        FrameNode::new(0, [2.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
        [0.0, 0.0, 0.0],
        2.0e11,
        7.7e10,
        0.01,
        8.0e-6,
        1.6e-5,
        1.0,
        1.0,
    )
    .unwrap()
}

#[test]
fn k9_terms_sum_exactly_to_the_combined_section_value() {
    let bend = bend();
    for (g, n) in [(1e8, 0.3), (1e80, 1e-8)] {
        for fraction in [0.25, 0.5, 0.8] {
            let combined = bend
                .arc_section_resultants(fraction, [0.0; 6], [0.0, n, 0.0])
                .unwrap();
            let fg = bend
                .arc_section_resultants(fraction, [0.0; 6], [0.0, g, 0.0])
                .unwrap();
            let fn_ = combined;
            let fmg = bend
                .arc_section_resultants(fraction, [0.0; 6], [0.0, -g, 0.0])
                .unwrap();
            // Precondition: some component's producer-order fold is wrong.
            let differs = (0..6).any(|c| {
                let folded = 0.0 + fg[c] + fn_[c] + fmg[c];
                folded.to_bits() != (combined[c] + 0.0).to_bits()
            });
            assert!(differs, "precondition g={g} fraction={fraction}");
            let terms = bend
                .arc_section_resultant_terms(
                    fraction,
                    [0.0; 6],
                    &[[0.0, g, 0.0], [0.0, n, 0.0], [0.0, -g, 0.0]],
                    &[],
                )
                .unwrap();
            for c in 0..6 {
                assert_eq!(terms[c].to_bits(), (combined[c] + 0.0).to_bits(), "{c}");
            }
            // Thrust terms (G, n, -G) likewise sum to the thrust n alone.
            let thrust_only = bend
                .arc_section_resultants_with_radial_pressure(fraction, [0.0; 6], [0.0; 3], n)
                .unwrap();
            let thrust_terms = bend
                .arc_section_resultant_terms(fraction, [0.0; 6], &[], &[g, n, -g])
                .unwrap();
            for c in 0..6 {
                assert_eq!(thrust_terms[c].to_bits(), (thrust_only[c] + 0.0).to_bits());
            }
        }
    }
}

#[test]
fn k9_single_input_controls_are_bit_identical() {
    let bend = bend();
    let force = [12.5, -3.25, 7.75, 1.5, -2.125, 0.875];
    for fraction in [0.0, 0.37, 0.5, 1.0] {
        let today = bend
            .arc_section_resultants(fraction, force, [0.0; 3])
            .unwrap();
        let terms = bend
            .arc_section_resultant_terms(fraction, force, &[], &[])
            .unwrap();
        for c in 0..6 {
            assert_eq!(
                terms[c].to_bits(),
                (today[c] + 0.0).to_bits(),
                "{fraction} {c}"
            );
        }
        let w = [0.4, -190.0, 3.0];
        let today = bend.arc_section_resultants(fraction, [0.0; 6], w).unwrap();
        let terms = bend
            .arc_section_resultant_terms(fraction, [0.0; 6], &[w], &[])
            .unwrap();
        for c in 0..6 {
            assert_eq!(
                terms[c].to_bits(),
                (today[c] + 0.0).to_bits(),
                "{fraction} {c}"
            );
        }
    }
}
