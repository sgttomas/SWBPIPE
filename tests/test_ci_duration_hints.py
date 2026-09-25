"""Duration-hint refresh parsing contracts; no network, browser or test execution."""
import importlib.util
from pathlib import Path
import sys
import unittest

TOOLS = Path(__file__).resolve().parents[1] / 'tools/ci'
sys.path.insert(0, str(TOOLS))
spec = importlib.util.spec_from_file_location('refresh_duration_hints', TOOLS / 'refresh_duration_hints.py')
refresh = importlib.util.module_from_spec(spec)
spec.loader.exec_module(refresh)


def row(title, project='chromium-desktop', tags=(), path=()):
    return dict(file='e2e/a.spec.ts', title=title, title_path=[*path, title], project=project, tags=list(tags))


class RefreshTests(unittest.TestCase):
    source = [row('plain'), row('inline @explicit-viewport', tags=['@explicit-viewport']),
              row('detail tagged', tags=['@slow']), row('nested', path=['group']), row('skipped')]
    log = '\n'.join([
        '2026-09-25T08:30:00Z   ✓  1 [chromium-desktop] › e2e/a.spec.ts:3:1 › plain (1.5s)',
        '2026-09-25T08:30:01Z   ✓  2 [chromium-desktop] › e2e/a.spec.ts:4:1 › inline @explicit-viewport (250ms)',
        '2026-09-25T08:30:02Z   ✓  3 [chromium-desktop] › e2e/a.spec.ts:5:1 › detail tagged @slow (0ms)',
        '2026-09-25T08:30:03Z   ✓  4 [chromium-desktop] › e2e/a.spec.ts:6:1 › group › nested (1.1m)',
        '2026-09-25T08:30:04Z   -  5 [chromium-desktop] › e2e/a.spec.ts:7:1 › skipped'])

    def test_labels_resolve_inline_and_detail_tags_and_nesting(self):
        durations, skips = refresh.parse_logs(self.log, self.source)
        key = lambda title, *path: ' › '.join(['chromium-desktop', 'e2e/a.spec.ts', *path, title])
        self.assertEqual(durations, {key('plain'): 1.5, key('inline @explicit-viewport'): .25,
                                     key('detail tagged'): 0.0, key('nested', 'group'): 66.0})
        self.assertEqual(skips, {key('skipped')})

    def test_unknown_or_duplicate_labels_fail(self):
        with self.assertRaises(KeyError):
            refresh.parse_logs(self.log.replace('plain', 'renamed'), self.source)
        with self.assertRaises(ValueError):
            refresh.parse_logs(self.log + '\n' + self.log.splitlines()[0], self.source)

    def test_refresh_keeps_unobserved_prior_and_positive_floors(self):
        old = {'version': 1, 'unknown_seconds': 30, 'basis': {'run': 1},
               'seconds': {'kept': 4.0, 'zero': .001, 'measured': 9.0}}
        new = refresh.refresh(old, {'measured': 2.0, 'zero': 0.0, 'fresh': 0.0}, {'skip'}, {'run': 2})
        self.assertEqual(new['seconds'], {'fresh': refresh.ZERO_WEIGHT, 'kept': 4.0, 'measured': 2.0,
                                          'skip': refresh.SKIP_WEIGHT, 'zero': .001})
        self.assertEqual(new['basis'], {'run': 2, 'prior_duration_basis': {'run': 1}})


if __name__ == '__main__':
    unittest.main()
