#!/usr/bin/env python3
"""Tests of `vk_scale_runner.py`'s stop rules and its one named exception.

The exception is ROOT's ruling for V-K's B: at 10,000 members,
`Unresolved(ExactSumSpan)` is the KF3 availability finding. It is recorded and
the tier continues. Standard library only:

    python3 -m unittest runner/test_vk_scale_runner.py   (from VR's directory)
"""
import os
import sys
import unittest

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vk_scale_runner as v  # noqa: E402

ROWS = 103
SPAN = 'Unresolved ExactSumSpan [Restrained]'


def objects(outcome=SPAN, rows=ROWS, published=False, extra_failures=(), rcm_equal=True,
            counts_match=True, not_covered_equal=True, s_full=None):
    case = 'RF-LARGE-CHAIN-n10000-AX'
    selected = outcome.startswith('Selected')
    failures = [] if selected else ['%s: not selected: %s' % (case, outcome)]
    failures += list(extra_failures)
    fail = 0 if selected else rows
    return [
        {'kind': 'w1', 'outcome': outcome, 'counts_match_storage': counts_match},
        {'kind': 'report', 'rows': rows, 'pass': rows if selected else (1 if published else 0),
         'pass_absolute_range': 0, 'not_covered': 0, 'structural_zero': 0, 'expected_unresolved': 0,
         'fail': fail - (1 if published and not selected else 0), 'accounted': True,
         'failures': len(failures), 'failures_head': failures[:50],
         'not_covered_rows': [], 'not_covered_committed': [], 'not_covered_equal': not_covered_equal,
         'not_covered_rows_s_full': s_full, 'class_mismatches': [],
         'controls_discriminated': 3, 'controls_non_discriminating': 0,
         'controls_undiscriminated': [], 'controls_unexpectedly_failing': []},
        {'kind': 'rcm', 'equal': rcm_equal},
    ]


OK = {'classification': 'ok'}


def run(members):
    return {'members': members, 'model': 'RF-LARGE-CHAIN-n%05d-AX' % members}


class Kf3AvailabilityException(unittest.TestCase):
    def test_exact_sum_span_at_10000_members_is_recorded_and_the_tier_continues(self):
        objs = objects()
        self.assertEqual(v.availability_exception(run(10000), objs), v.KF3_AVAILABILITY)
        self.assertEqual(v.stop_reasons(OK, objs, True, run(10000)), [])

    def test_exact_sum_span_below_10000_members_still_stops(self):
        for members in (100, 1000):
            objs = objects()
            self.assertIsNone(v.availability_exception(run(members), objs))
            reasons = v.stop_reasons(OK, objs, True, run(members))
            self.assertTrue(any(r.startswith('not selected') for r in reasons), reasons)

    def test_any_other_unresolved_reason_still_stops(self):
        for outcome in ('Unresolved Ceiling [Restrained]', 'Unresolved ExponentRange [Restrained]',
                        'Unresolved Budget(Case) [Restrained]', 'Refused MechanismWitnessed { body: 0 } []'):
            objs = objects(outcome=outcome)
            self.assertIsNone(v.availability_exception(run(10000), objs))
            self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)), outcome)

    def test_a_failure_on_a_covered_row_still_stops(self):
        objs = objects(extra_failures=['RF-LARGE-CHAIN-n10000-AX: u.C1.UX (1e-3) predicate'])
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)))

    def test_a_published_row_voids_the_exception(self):
        objs = objects(published=True)
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)))

    def test_every_other_check_still_applies_under_the_exception(self):
        for kwargs, expect in (({'rcm_equal': False}, "K4's RCM differs"),
                               ({'counts_match': False}, "V-K's counts differ"),
                               ({'not_covered_equal': False}, 'not-covered set'),
                               ({'s_full': ['u.C1.UX']}, 'C9')):
            objs = objects(**kwargs)
            reasons = v.stop_reasons(OK, objs, True, run(10000))
            self.assertTrue(any(expect in r for r in reasons), (kwargs, reasons))

    def test_a_process_that_did_not_complete_still_stops(self):
        for classification in ('killed_by_rss_watchdog', 'heap_cap_abort', 'timed_out', 'error'):
            record = {'classification': classification, 'classification_detail': None}
            self.assertTrue(v.stop_reasons(record, objects(), True, run(10000)), classification)

    def test_a_selected_case_with_no_failure_does_not_stop(self):
        objs = objects(outcome='Selected at 128')
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertEqual(v.stop_reasons(OK, objs, True, run(10000)), [])


if __name__ == '__main__':
    unittest.main()
