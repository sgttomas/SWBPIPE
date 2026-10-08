//! K4 tests of `retained/verify.rs` at A3a (D1 revision 5a.3, R7 §4.1.6.1
//! item 6a): ê's binary64 coupling and Φ = fl↑(2^-438·ê), decided exactly,
//! with its boundary at ê = 2^-584 (below it the product is subnormal and Φ is
//! rounded up). E itself is tested on the models in `scale_tests.rs` (E-UNIT).
use super::*;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;

#[test]
fn phi_is_exact_where_the_product_is_normal_and_rounded_up_below_2_to_the_minus_584() {
    assert_eq!(f64::from_bits(PHI_SCALE_BITS), support::pow2(-438));
    // ê ≥ 2^-584: Φ = 2^-438·ê exactly.
    for e in [
        1.0f64,
        3.0,
        support::pow2(-584),
        1.5 * support::pow2(-584),
        f64::MAX,
    ] {
        let phi = phi_512(e);
        assert_eq!(phi, e * support::pow2(-438), "{e:e}");
    }
    // Below: the product is subnormal; Φ is the least binary64 not below it.
    for e in [
        support::pow2(-584) * 1.25,
        support::pow2(-600) * 1.75,
        f64::from_bits(1) * 3.0,
    ] {
        let phi = phi_512(e);
        // Exact check with integers: Φ·2^438 ≥ ê, and the next binary64 down fails.
        let scale = support::pow2(438);
        assert!(phi * scale >= e, "{e:e}");
        let below = f64::from_bits(phi.to_bits() - 1);
        assert!(below * scale < e || phi.to_bits() == 1, "{e:e}");
    }
    // ê = 0 gives Φ = 0.
    assert_eq!(phi_512(0.0), 0.0);
}

#[test]
fn e_hat_couples_force_and_moment_through_the_body_extent_in_binary64() {
    assert_eq!(e_hat([1.0, 8.0], 0.0), [1.0, 8.0]);
    assert_eq!(e_hat([1.0, 8.0], 2.0), [4.0, 8.0]);
    assert_eq!(e_hat([5.0, 8.0], 2.0), [5.0, 10.0]);
    // fl(E_mo/L_b) rounds once.
    let [fo, _] = e_hat([0.0, 1.0], 3.0);
    assert_eq!(fo, 1.0 / 3.0);
}

#[test]
fn an_e_hat_that_overflows_while_e_encodes_stops_as_e_does() {
    // ROOT's A3b ruling: fl(L_b·E_fo) or fl(E_mo/L_b) beyond binary64, with E
    // finite, is `ResolutionScale` for that body and kind (F2a:
    // `receipt_encoding`), checked body by body, force before moment.
    let big = f64::MAX / 2.0;
    assert_eq!(
        resolution_hats(&[[1.0, 8.0], [big, 1.0]], &[2.0, 4.0]),
        Err(AttemptStop::ResolutionScale {
            body: 1,
            kind: Kind::Moment
        })
    );
    assert_eq!(
        resolution_hats(&[[1.0, big]], &[0.25]),
        Err(AttemptStop::ResolutionScale {
            body: 0,
            kind: Kind::Force
        })
    );
    // At the edge of the range ê is finite; a body of zero extent keeps E.
    assert_eq!(
        resolution_hats(&[[big, 1.0], [f64::MAX, f64::MAX]], &[2.0, 0.0]),
        Ok(vec![[big, f64::MAX], [f64::MAX, f64::MAX]])
    );
}
