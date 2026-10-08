#!/usr/bin/env python3
"""RV109's mutants of ST (B1 round 1). Each is an exact, single-occurrence text replacement in
PP `src/lib.rs` of the reviewer's candidate copy.

Usage: mutants.py <lib.rs path> apply <ID>   (writes the mutant; refuses unless exactly one hit)
       mutants.py list
"""
import sys

CLASSIFY_OLD = """        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);
        let seed = only_one(seeds.iter().filter(|seed| seed.case == case_id));
        if verdict == Some(NumericalQualityStatus::ChecksPassed) {"""

MUTANTS = {
    # PLAN_v2 §2.1's five.
    "P1_keyed_on_initial": (
        "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {",
        "        if matches!(seed.and_then(|s| s.initial.as_ref()), Some(retained_product::InitialSeed::Report { code, .. }) if code == \"NUMERICAL_INTEGRITY_CHECKS_PASSED\") {",
        "The not_required test reads the seed's initial report code (CHECKS_PASSED), not the published verdict."),
    "P2_verdict_by_position": (
        "    case_ids.iter().map(move |&case_id| {\n        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);",
        "    case_ids.iter().enumerate().map(move |(position, &case_id)| {\n        let verdict = quality.cases.get(position).map(|entry| entry.solve_quality);",
        "The verdict is the quality entry at the case's request position."),
    "P3_no_decision_21": (
        "        } else if seed.is_some_and(dn_trigger_excluded) {",
        "        } else if seed.is_some_and(|_| false) && seed.is_some_and(dn_trigger_excluded) {",
        "Decision 21's exclusion is dropped (every non-Passed case is in A)."),
    "P4_seedless_excluded": (
        "        } else if seed.is_some_and(dn_trigger_excluded) {",
        "        } else if seed.map_or(true, dn_trigger_excluded) {",
        "A case with no seed is excluded."),
    "P5_no_triggered_case_appends_notice": (
        """    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }
    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };""",
        """    let no_trigger = !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted);
    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };
    if no_trigger {
        return notice.publish(ordinary, W1Fallback::NoTriggeredCase);
    }""",
        "NoTriggeredCase reserves and appends the N1 notice."),
    # RV109's own.
    "R6_seed_by_position": (
        "    case_ids.iter().map(move |&case_id| {\n        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);\n        let seed = only_one(seeds.iter().filter(|seed| seed.case == case_id));",
        "    case_ids.iter().enumerate().map(move |(position, &case_id)| {\n        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);\n        let seed = seeds.get(position);",
        "The seed is the one at the case's request position."),
    "R7_first_match_not_unique": (
        "        let verdict = only_one(quality.cases.iter().filter(|entry| entry.basis_ref.ref_id == case_id)).map(|entry| entry.solve_quality);\n        let seed = only_one(seeds.iter().filter(|seed| seed.case == case_id));",
        "        let verdict = quality.cases.iter().find(|entry| entry.basis_ref.ref_id == case_id).map(|entry| entry.solve_quality);\n        let seed = seeds.iter().find(|seed| seed.case == case_id);",
        "R3 ruling 1 dropped: the first matching entry and seed are used, not only a unique one."),
    "R8_exclusion_ignores_w2": (
        "    })) && !matches!(seed.w2, retained_product::W2Seed::Published { .. })",
        "    })) && (true || !matches!(seed.w2, retained_product::W2Seed::Published { .. }))",
        "Decision 21 excludes even when W2 published the case."),
    "R9_w2_failed_counts_as_published": (
        "    })) && !matches!(seed.w2, retained_product::W2Seed::Published { .. })",
        "    })) && matches!(seed.w2, retained_product::W2Seed::NotTriggered)",
        "Decision 21 excludes only when W2 was not triggered (a failed W2 counts as published)."),
    "R10_t4_after_reservation": (
        """    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }
    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };""",
        """    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };
    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        drop(notice);
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }""",
        "T-4 runs after the notice reservation (the slot is reserved, then dropped unpublished)."),
    "R10b_reserve_then_shrink": (
        """    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }
    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };""",
        """    // R-2: the notice's space is reserved before any W1 work starts.
    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {
        return (ordinary, Err(W1Fallback::NoticeReservation));
    };
    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
        drop(notice);
        ordinary.diagnostics.shrink_to_fit();
        return (ordinary, Err(W1Fallback::NoTriggeredCase));
    }""",
        "R10 that also gives the reserved slot back (shrink_to_fit): the capacity check cannot see it, so only the collision variant can kill it."),
    "R11_not_assessed_not_required": (
        "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {",
        "        if matches!(verdict, Some(NumericalQualityStatus::ChecksPassed | NumericalQualityStatus::NotAssessed)) {",
        "A not_assessed verdict is treated as not_required."),
    "R12_negative_energy_excluded": (
        "        error: StructuralError::Mechanism { .. } | StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_), ..",
        "        error: StructuralError::Mechanism { .. } | StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_) | StructuralError::NegativeEnergy { .. }, ..",
        "NegativeEnergy (a DN §4.3 trigger) is excluded too."),
    "R13_mechanism_not_excluded": (
        "        error: StructuralError::Mechanism { .. } | StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_), ..",
        "        error: StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_), ..",
        "Mechanism is dropped from the exclusion."),
    "R14_seeds_not_wired": (
        "    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id])",
        "    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary[..0], &[case_id])",
        "retained_w1 hands the classifier no seeds."),
    "R15_quality_not_wired": (
        "    if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id])",
        "    if !case_triggers(&NumericalQuality { cases: Vec::new(), ..ordinary.numerical_quality.clone() }, &observer.ordinary, &[case_id])",
        "retained_w1 hands the classifier no quality entries."),
    "R16_not_required_needs_seed": (
        "        if verdict == Some(NumericalQualityStatus::ChecksPassed) {",
        "        if verdict == Some(NumericalQualityStatus::ChecksPassed) && seed.is_some() {",
        "not_required also requires a seed (a Passed case with no seed is attempted)."),
    "R17_any_to_all": (
        ".any(|trigger| trigger == CaseTrigger::Attempted) {\n        return (ordinary, Err(W1Fallback::NoTriggeredCase));",
        ".all(|trigger| trigger == CaseTrigger::Attempted) {\n        return (ordinary, Err(W1Fallback::NoTriggeredCase));",
        "A is empty unless every case is attempted (equivalent at c = 1; recorded as such)."),
}


def main():
    if sys.argv[1] == "list":
        for k, (_, _, why) in MUTANTS.items():
            print(f"{k}\t{why}")
        return
    path, action, mid = sys.argv[1], sys.argv[2], sys.argv[3]
    assert action == "apply"
    old, new, _ = MUTANTS[mid]
    text = open(path, encoding="utf-8").read()
    hits = text.count(old)
    if hits != 1:
        sys.exit(f"{mid}: {hits} hits")
    open(path, "w", encoding="utf-8").write(text.replace(old, new))
    print(f"applied {mid}")


if __name__ == "__main__":
    main()
