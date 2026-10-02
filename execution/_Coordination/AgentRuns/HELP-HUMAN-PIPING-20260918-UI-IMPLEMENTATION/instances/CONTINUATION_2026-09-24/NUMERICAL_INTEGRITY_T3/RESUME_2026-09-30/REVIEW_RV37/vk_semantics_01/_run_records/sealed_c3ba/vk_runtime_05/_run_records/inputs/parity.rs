//! The binary64 sparse gate's parity on RF-MECH and RF-LARGE (brief Scope 3;
//! T3 D1 §4.8 items 1–3; ROOT's C4 ruling, up to 100 members; plan §5.3):
//! bitwise K and the M03 outcome class in both modes, and the DEC-053 delta
//! asserted where both modes are Passed. RF-MECH's models of up to 100
//! members must be refused in both modes. RF-MECH-DISC-CHAIN100 and -SPRING
//! (103 members) and LINE-IN-CHAIN1000 (1,005) are beyond the ruling's 100
//! members: `vk_records` records them. The binary64 gate's refusal of those
//! takes minutes even in release (A1 finding), so they stay out of CI.
use piping_numerical_robustness::cases::load_family;
use piping_numerical_robustness::parity::parity;

#[test]
fn rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis() {
    let mut n = 0;
    for c in load_family("RF-LARGE").iter().filter(|c| !c.is_large()) {
        let p = parity(c.model.as_ref().unwrap(), true).unwrap();
        println!(
            "{}: sparse {} dense {:?} delta {:?}",
            c.id, p.sparse, p.dense, p.delta
        );
        assert_eq!(p.bitwise_k, Some(None), "{}: bitwise K", c.id);
        assert_eq!(Some(&p.sparse), p.dense.as_ref(), "{}: outcome class", c.id);
        let (within, _) = p.delta.expect("both modes publish");
        if p.both_passed {
            assert!(within, "{}: the DEC-053 basis", c.id);
        }
        n += 1;
    }
    assert_eq!(n, 12);
}

#[test]
fn rf_mech_is_refused_in_both_modes_with_bitwise_k() {
    let mut n = 0;
    for c in load_family("RF-MECH")
        .iter()
        .filter(|c| c.model.as_ref().unwrap().members.len() <= 100)
    {
        let model = c.model.as_ref().unwrap();
        let p = parity(model, true).unwrap();
        println!("{}: sparse {} dense {:?}", c.id, p.sparse, p.dense);
        assert_eq!(p.bitwise_k, Some(None), "{}: bitwise K", c.id);
        assert_eq!(Some(&p.sparse), p.dense.as_ref(), "{}: outcome class", c.id);
        if c.refuse {
            assert!(
                p.sparse != "Passed" && p.sparse != "Sensitive",
                "{}: a mechanism published in the sparse mode",
                c.id
            );
        }
        n += 1;
    }
    assert_eq!(n, 6);
}
