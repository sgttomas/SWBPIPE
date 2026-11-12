"""The table of diagnostic codes a solve can raise stays equal to the sources, and each code is classified.

Exit criterion 3 (`docs/RELEASE_TARGET_v1.0.md`) needs every remaining refusal of a required capability
listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. `tools/validation/diagnostic_codes.py` reads the codes from
the production Rust sources; `fixtures/diagnostics/exact_route_codes.json` holds them with a hand-made
classification. This test fails when the two disagree, when a record lacks a classification or a reason,
when a site the scanner cannot resolve or a code-shaped literal it does not account for appears or goes,
and when the number of unsure records or of absent capabilities no limitation entry names rises above
its pin.

To refresh the table after a source change: `python3 tools/validation/diagnostic_codes.py --write`, then
classify each new record by hand.
"""
from __future__ import annotations

import json
from pathlib import Path
import re
import sys
import warnings

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation.diagnostic_codes import CLASSES, TABLE, Scan, tokenize  # noqa: E402

# The scan reads every production Rust file of these crates through their module trees.
CI_READS = ('core/product_physics/src/**', 'core/solver/*/src/**', 'core/loads/*/src/**',
            'core/analysis/hanger_sizing/src/**', 'core/numerics/*/src/**',
            'core/model_operations/operation_applier/src/**', 'fixtures/diagnostics/exact_route_codes.json',
            'docs/RELEASE_LIMITATIONS_v1.0.md')

# Pins: no count may grow unnoticed. When one falls, lower its pin to the new count. The last falls to zero
# only when every absent capability is named by a limitation entry or by text the product publishes.
UNSURE_PIN = 30
UNNAMED_ABSENT_CAPABILITY_PIN = 54
NAMED_NOWHERE_TODAY_PIN = 49

REFRESH = 'run `python3 tools/validation/diagnostic_codes.py --write` and classify the new records by hand'

_SCAN = None


def scan():
    global _SCAN
    if _SCAN is None:
        _SCAN = Scan(PROJECT).run().table()
    return _SCAN


def table():
    return json.loads((PROJECT / TABLE).read_text(encoding='utf-8'))


def test_every_raised_code_is_in_the_table_and_no_other():
    found = {c['code'] for c in scan()['codes']}
    held = {c['code'] for c in table()['codes']}
    assert not found - held, f'codes the sources raise that the table lacks: {sorted(found - held)}; {REFRESH}'
    assert not held - found, f'codes in the table the sources no longer raise: {sorted(held - found)}; {REFRESH}'


def test_the_table_is_sorted_with_one_record_per_code():
    codes = [c['code'] for c in table()['codes']]
    assert codes == sorted(set(codes))


def test_unresolved_sites_and_unclaimed_literals_are_current():
    """A new site the scanner cannot resolve, or a new code-shaped literal it does not find raised, may be a code
    reaching a diagnostic by a route the scanner does not know: each is looked at and the table refreshed."""
    def sites(t):
        return sorted((s['file'], s['at'], s['why']) for s in t['unresolved_sites'])

    def literals(t):
        return sorted(x['literal'] for x in t['unclaimed_literals'])

    assert sites(scan()) == sites(table()), REFRESH
    assert literals(scan()) == literals(table()), REFRESH


def test_every_record_is_classified_with_a_reason():
    problems = []
    for c in table()['codes']:
        if c.get('classification') not in CLASSES:
            problems.append(f'{c["code"]}: classification {c.get("classification")!r}')
        if not (isinstance(c.get('reason'), str) and c['reason'].strip()):
            problems.append(f'{c["code"]}: no reason')
        if c.get('unsure'):
            candidates = c.get('candidates') or []
            if len(candidates) != 2 or not set(candidates) <= set(CLASSES) or c['classification'] not in candidates:
                problems.append(f'{c["code"]}: unsure needs its two candidate classes, the chosen one among them')
        if 'named_by_statement' not in c:
            problems.append(f'{c["code"]}: no named_by_statement field')
        if c.get('classification') == 'absent_capability':
            if not (c.get('named_by') == 'unnamed' or re.fullmatch(r'LIM-\d{3}', str(c.get('named_by')))):
                problems.append(f'{c["code"]}: an absent capability needs named_by LIM-nnn or "unnamed"')
        elif c.get('named_by') is not None:
            problems.append(f'{c["code"]}: named_by is for absent capabilities only')
    assert not problems, problems


def test_named_by_names_an_entry_that_names_the_code():
    limitations = (PROJECT / 'docs/RELEASE_LIMITATIONS_v1.0.md').read_text(encoding='utf-8')
    entries = dict(re.findall(r'^### (LIM-\d{3})\b.*?$(.*?)(?=^### |\Z)', limitations, re.M | re.S))
    problems = []
    for c in table()['codes']:
        lim = c.get('named_by')
        if lim and lim != 'unnamed':
            if lim not in entries:
                problems.append(f'{c["code"]}: {lim} is not an entry')
            elif f'`{c["code"]}`' not in entries[lim]:
                problems.append(f'{c["code"]}: {lim} does not name it')
    assert not problems, problems


def test_a_statement_said_to_name_a_code_names_it_today():
    """`in_product` is a claim about text the product publishes now: the file it cites holds the code inside a
    longer text literal. A `planned` statement names nothing yet and is counted as named nowhere."""
    problems = []
    for c in table()['codes']:
        s = c.get('named_by_statement')
        if s is None:
            continue
        if c.get('classification') != 'absent_capability':
            problems.append(f'{c["code"]}: named_by_statement is for absent capabilities only')
            continue
        if s.get('status') == 'in_product':
            path = PROJECT / s['where'].rsplit(':', 1)[0]
            texts = [t.val for t in tokenize(path.read_text(encoding='utf-8')) if t.kind == 'str']
            if not any(c['code'] in text and text != c['code'] for text in texts):
                problems.append(f'{c["code"]}: {s["where"]} has no text that names it')
        elif s.get('status') != 'planned':
            problems.append(f'{c["code"]}: named_by_statement status {s.get("status")!r}')
    assert not problems, problems


def named_nowhere_today(c):
    s = c.get('named_by_statement') or {}
    return c.get('classification') == 'absent_capability' and c.get('named_by') == 'unnamed' \
        and s.get('status') != 'in_product'


def test_unsure_and_unnamed_counts_do_not_rise():
    codes = table()['codes']
    counts = {
        'unsure records': (sum(1 for c in codes if c.get('unsure')), UNSURE_PIN, 'UNSURE_PIN'),
        'absent capabilities no limitation entry names': (
            sum(1 for c in codes if c.get('classification') == 'absent_capability' and c.get('named_by') == 'unnamed'),
            UNNAMED_ABSENT_CAPABILITY_PIN, 'UNNAMED_ABSENT_CAPABILITY_PIN'),
        'absent capabilities named nowhere today (no limitation entry, no product text; planned statements count)': (
            sum(1 for c in codes if named_nowhere_today(c)), NAMED_NOWHERE_TODAY_PIN, 'NAMED_NOWHERE_TODAY_PIN'),
    }
    for what, (count, pin, name) in counts.items():
        print(f'{what}: {count} (pin {pin})')
        assert count <= pin, f'{count} {what}, more than the pin {name} = {pin}'
        if count < pin:
            warnings.warn(f'{what} fell to {count}, below its pin: lower {name} in '
                          f'tests/test_diagnostic_code_table.py to {count}')
