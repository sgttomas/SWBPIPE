# RV99's 16 mutant patch texts, copied byte for byte from RV99's evidence/tools/mut_rv99.py.
EE, RUNNER, PY = 'EE', 'RUNNER', 'PY'
RV99 = {
    # evaluator: comparisons
    "V1_less_than_strictness_lost": (EE, [(
        "        ComparisonOperator::LessThan => {\n            if a.hi < b.lo {",
        "        ComparisonOperator::LessThan => {\n            if a.hi <= b.lo {")]),
    "V2_ge_true_uses_upper_end": (EE, [(
        "        ComparisonOperator::GreaterThanOrEqual => {\n            if a.lo >= b.hi {",
        "        ComparisonOperator::GreaterThanOrEqual => {\n            if a.hi >= b.hi {")]),
    "V3_equal_true_for_equal_ranges": (EE, [(
        "    if a.is_point() && b.is_point() && a.lo == b.lo {",
        "    if a.lo == b.lo && a.hi == b.hi {")]),
    "V4_kleene_false_or_unknown_is_false": (EE, [(
        "            (Truth::False, Truth::False) => Truth::False,\n            (Truth::False, Truth::Indeterminate)\n            | (Truth::Indeterminate, Truth::False)",
        "            (Truth::False, Truth::False) | (Truth::False, Truth::Indeterminate) => Truth::False,\n            (Truth::Indeterminate, Truth::False)")]),
    "V5_select_unknown_boolean_takes_then": (EE, [(
        "                        Some(IValue::Boolean(if then_truth == else_truth {\n                            then_truth\n                        } else {\n                            Truth::Indeterminate\n                        }))",
        "                        Some(IValue::Boolean(if then_truth == else_truth {\n                            then_truth\n                        } else {\n                            then_truth\n                        }))")]),
    "V6_step_governing_row_strict": (EE, [(
        "        if row.argument <= x {\n            index = candidate;",
        "        if row.argument < x {\n            index = candidate;")]),
    "V7_interpolation_first_segment_only": (EE, [(
        "        joined = Some(match joined {\n            Some(enclosure) => interval_hull(enclosure, segment),\n            None => segment,\n        });\n    }\n    joined",
        "        joined = Some(match joined {\n            Some(enclosure) => interval_hull(enclosure, segment),\n            None => segment,\n        });\n        break;\n    }\n    joined")]),
    "V8_bound_enclosure_not_outward": (EE, [(
        "    outward(value - bound, value + bound)\n}",
        "    Some(Enclosure {\n        lo: value - bound,\n        hi: value + bound,\n    })\n}")]),
    "V9_divisor_zero_end_not_refused": (EE, [(
        "    e.lo <= 0.0 && 0.0 <= e.hi",
        "    e.lo < 0.0 && 0.0 < e.hi")]),
    "V14_table_range_note_not_eager": (EE, [(
        "        value.map(|value| value.into_public(!notes.is_empty()))",
        "        value.map(|value| value.into_public(notes.iter().any(|n| n.code != IntervalNoteCode::TableArgumentRange)))")]),
    # runner
    "V10_runner_all_fail_reported_as_pass": (RUNNER, [(
        "        Truth::False => (\n            RuleCheckStatus::UserRuleFailed,\n            RULE_INTERVAL_ALL_FAIL,",
        "        Truth::False => (\n            RuleCheckStatus::UserRuleChecked,\n            RULE_INTERVAL_ALL_PASS,")]),
    "V11_runner_offset_step_inward": (RUNNER, [(
        "    let lo = down(lo - t.offset)?;",
        "    let lo = up(lo - t.offset)?;")]),
    "V12_runner_subnormal_bound_as_point": (RUNNER, [(
        "            Some(b) if b == 0.0 => (raw_value, raw_unit, note, None),",
        "            Some(b) if b < 1e-300 => (raw_value, raw_unit, note, None),")]),
    "V13_runner_limit_compare_uses_point": (RUNNER, [(
        "            enclosure: formula.enclosure,\n        }],",
        "            enclosure: formula.enclosure.map(|e| Enclosure::point(e.lo)),\n        }],")]),
    # python
    "PV1_python_less_than_strictness_lost": (PY, [(
        "        return TRUE if a[1] < b[0] else FALSE if a[0] >= b[1] else INDETERMINATE",
        "        return TRUE if a[1] <= b[0] else FALSE if a[0] >= b[1] else INDETERMINATE")]),
    "PV2_python_or_false_unknown_is_false": (PY, [(
        "    return FALSE if a == b == FALSE else INDETERMINATE",
        "    return FALSE if FALSE in (a, b) else INDETERMINATE")]),
}

