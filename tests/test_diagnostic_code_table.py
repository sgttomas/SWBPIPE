"""The table of diagnostic codes a solve can raise stays equal to the sources, and each code is classified.

Exit criterion 3 (`docs/RELEASE_TARGET_v1.0.md`) needs every remaining refusal of a required capability
listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. `tools/validation/diagnostic_codes.py` reads the codes from
the production Rust sources; `fixtures/diagnostics/exact_route_codes.json` holds them with a hand-made
classification. This test fails when the two disagree, when a record lacks a classification or a reason,
when a site the scanner cannot resolve or a value it does not account for appears or goes, and when the
number of unsure records or of absent capabilities not named rises above its pin. Witnesses on a small
synthetic source pin that a code is found in each kind of diagnostic position whatever its spelling.

The test cannot check that a text covers a refusal, only that it mentions a code. Each naming (a LIM
entry, or product text) carries a hand-made coverage, whole or partial, with its reason; only a whole
naming counts as naming the refusal.

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
# only when every absent capability is named wholly by a limitation entry or by text the product publishes.
# 30 -> 36 (review of d776be92e): four of the newly found codes (NodeOutOfRange, NonFiniteAxialEffect,
# UnsupportedTargetForCategory, HANGER_SIZING_INPUT_TOO_LARGE) and the two reclassified two-use codes
# (EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED, JOINT_STIFFNESS_BASIS_UNSUPPORTED) are unsure.
UNSURE_PIN = 36
# 52 -> 56 (review of d776be92e): LIM-003 and LIM-004 name their codes only partially, so those two count as
# unnamed (finding F6); EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED is reclassified an absent capability (F4);
# HANGER_SIZING_INPUT_TOO_LARGE is newly found and classed one (F2). No refusal changed; the count was low.
UNNAMED_ABSENT_CAPABILITY_PIN = 56
# 47 -> 52 (review of d776be92e): the same four, and CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE, whose product
# text describes its gate only partially (F7).
NAMED_NOWHERE_TODAY_PIN = 52

REFRESH = 'run `python3 tools/validation/diagnostic_codes.py --write` and classify the new records by hand'
COVERAGE = ('whole', 'partial')

_SCAN = None


def scan():
    global _SCAN
    if _SCAN is None:
        _SCAN = Scan(PROJECT).run().table()
    return _SCAN


def table():
    return json.loads((PROJECT / TABLE).read_text(encoding='utf-8'))


def differences(found, held):
    """The codes the sources raise that the table lacks, and the codes it holds that they no longer raise."""
    return sorted(set(found) - set(held)), sorted(set(held) - set(found))


def test_every_raised_code_is_in_the_table_and_no_other():
    lacking, stale = differences({c['code'] for c in scan()['codes']}, {c['code'] for c in table()['codes']})
    assert not lacking, f'codes the sources raise that the table lacks: {lacking}; {REFRESH}'
    assert not stale, f'codes in the table the sources no longer raise: {stale}; {REFRESH}'


def test_the_table_is_sorted_with_one_record_per_code():
    codes = [c['code'] for c in table()['codes']]
    assert codes == sorted(set(codes))


def test_unresolved_sites_and_unclaimed_literals_are_current():
    """A new site the scanner cannot resolve, or a new value it does not find raised (a code-shaped literal,
    or any value at a position known only by its shape or name), may be a code reaching a diagnostic by a
    route the scanner does not know: each is looked at and the table refreshed."""
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
            if len(candidates) != 2 or len(set(candidates)) != 2 or not set(candidates) <= set(CLASSES) \
                    or c['classification'] not in candidates:
                problems.append(f'{c["code"]}: unsure needs two different candidate classes, the chosen one among them')
        if 'named_by_statement' not in c:
            problems.append(f'{c["code"]}: no named_by_statement field')
        if c.get('classification') == 'absent_capability':
            if not (c.get('named_by') == 'unnamed' or re.fullmatch(r'LIM-\d{3}', str(c.get('named_by')))):
                problems.append(f'{c["code"]}: an absent capability needs named_by LIM-nnn or "unnamed"')
        elif c.get('named_by') is not None:
            problems.append(f'{c["code"]}: named_by is for absent capabilities only')
        lim = c.get('named_by') not in (None, 'unnamed')
        cov = c.get('named_by_coverage')
        if lim and not (isinstance(cov, dict) and cov.get('coverage') in COVERAGE and str(cov.get('reason', '')).strip()):
            problems.append(f'{c["code"]}: a LIM naming needs named_by_coverage, whole or partial, with a reason')
        if not lim and cov is not None:
            problems.append(f'{c["code"]}: named_by_coverage without a LIM naming')
    assert not problems, problems


def test_named_by_names_an_entry_that_names_the_code():
    """Mentioning only: whether the entry covers the refusal is the hand-made coverage."""
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
    longer text literal (mentioning only; its coverage is hand-made). A `planned` statement names nothing yet
    and is counted as named nowhere."""
    problems = []
    for c in table()['codes']:
        statements = c.get('named_by_statement')
        if statements is None:
            continue
        if not isinstance(statements, list) or not statements:
            problems.append(f'{c["code"]}: named_by_statement is a nonempty list or null')
            continue
        for s in statements:
            if s.get('status') == 'in_product':
                path = PROJECT / s['where'].rsplit(':', 1)[0]
                texts = [t.val for t in tokenize(path.read_text(encoding='utf-8')) if t.kind == 'str']
                if not any(c['code'] in text and text != c['code'] for text in texts):
                    problems.append(f'{c["code"]}: {s["where"]} has no text that names it')
                if s.get('coverage') not in COVERAGE or not str(s.get('coverage_reason', '')).strip():
                    problems.append(f'{c["code"]}: an in-product naming needs its coverage, whole or partial, with a reason')
            elif s.get('status') == 'planned':
                if not s.get('subject') or not s.get('lands_with'):
                    problems.append(f'{c["code"]}: a planned statement needs its subject and when it lands')
            else:
                problems.append(f'{c["code"]}: named_by_statement status {s.get("status")!r}')
    assert not problems, problems


def named_wholly_by_entry(c):
    return c.get('named_by') not in (None, 'unnamed') and (c.get('named_by_coverage') or {}).get('coverage') == 'whole'


def named_nowhere_today(c):
    statements = c.get('named_by_statement') or []
    return c.get('classification') == 'absent_capability' and not named_wholly_by_entry(c) \
        and not any(s.get('status') == 'in_product' and s.get('coverage') == 'whole' for s in statements)


def test_unsure_and_unnamed_counts_do_not_rise():
    codes = table()['codes']
    counts = {
        'unsure records': (sum(1 for c in codes if c.get('unsure')), UNSURE_PIN, 'UNSURE_PIN'),
        'absent capabilities no limitation entry names wholly': (
            sum(1 for c in codes if c.get('classification') == 'absent_capability' and not named_wholly_by_entry(c)),
            UNNAMED_ABSENT_CAPABILITY_PIN, 'UNNAMED_ABSENT_CAPABILITY_PIN'),
        'absent capabilities named wholly nowhere today (planned statements and partial namings count)': (
            sum(1 for c in codes if named_nowhere_today(c)), NAMED_NOWHERE_TODAY_PIN, 'NAMED_NOWHERE_TODAY_PIN'),
    }
    for what, (count, pin, name) in counts.items():
        print(f'{what}: {count} (pin {pin})')
        assert count <= pin, f'{count} {what}, more than the pin {name} = {pin}'
        if count < pin:
            warnings.warn(f'{what} fell to {count}, below its pin: lower {name} in '
                          f'tests/test_diagnostic_code_table.py to {count}')


# ---------------------------------------------------------------- witnesses on a synthetic source

WITNESS = r'''
pub struct Diagnostic { pub code: String, pub severity: String, pub message: String }

fn emit(kind: &str, message: &str) -> Diagnostic {
    Diagnostic { code: kind.to_string(), severity: "blocking".to_string(), message: message.to_string() }
}

fn relay(tag: &str) -> Diagnostic { emit(tag, "relayed") }

pub enum Kind { MissingThing, OtherThing }
impl Kind {
    pub fn as_str(self) -> &'static str {
        match self { Kind::MissingThing => "MissingThing", Kind::OtherThing => "other_thing" }
    }
}
pub struct Finding { pub code: Kind, pub message: String }
impl Finding {
    pub fn record(&self) -> Diagnostic { emit(self.code.as_str(), &self.message) }
}

pub struct SizeError;
impl SizeError { pub fn code(&self) -> &'static str { "InputTooLarge" } }

pub fn field() -> Diagnostic {
    Diagnostic { code: "MixedCaseField".into(), severity: "info".into(), message: "m".into() }
}
pub fn helper() -> Diagnostic { relay("MixedCaseHelper") }
pub fn lower() -> Diagnostic { emit("lower_case_code", "m") }
pub fn json() -> String { serde_json::json!({"code": "jsonCode", "severity": "info", "message": "m"}).to_string() }
pub fn reason() -> String { serde_json::json!({"code": "not_a_diagnostic", "phase": "p"}).to_string() }
pub fn by_name(code: &str) -> String { serde_json::json!({"code": code, "phase": "p"}).to_string() }
pub fn name_only() -> String { by_name("lower_by_name") }
pub fn shaped_by_name() -> String { by_name("SHAPED_BY_NAME") }
EXTRA

#[cfg(test)]
mod tests { fn t() { let _ = super::emit("TestOnly", "m"); } }
'''

FOUND = {'MixedCaseField', 'MixedCaseHelper', 'lower_case_code', 'MissingThing', 'other_thing', 'InputTooLarge',
         'jsonCode', 'SHAPED_BY_NAME'}


def witness_scan(tmp_path, extra=''):
    root = tmp_path / ('with' if extra else 'base')
    src = root / 'core/product_physics/src'
    src.mkdir(parents=True)
    (src / 'lib.rs').write_text(WITNESS.replace('EXTRA', extra), encoding='utf-8')
    return Scan(root).run().table()


def test_witness_codes_are_found_in_every_kind_of_position_whatever_their_spelling(tmp_path):
    """A diagnostic record's code field, a helper's code argument passed on through another helper whatever the
    parameters are called, an enum-to-string producer read through a record's code, a typed error's code
    method and a json diagnostic: each is found, mixed-case and lower-case alike; a test-only call is not."""
    found = witness_scan(tmp_path)
    codes = {c['code'] for c in found['codes']}
    assert codes == FOUND, sorted(codes ^ FOUND)
    vias = {c['code']: {s['via'] for s in c['sites']} for c in found['codes']}
    assert any('typed producer' in v for v in vias['InputTooLarge'])
    assert any('enum code producer' in v for v in vias['MissingThing'])


def test_witness_values_at_unverified_positions_are_listed_not_dropped(tmp_path):
    """A non-code-shaped value at a position known only by a parameter's name, or in a json object that is not
    a diagnostic, is in the unclaimed list; a code-shaped one there is a code."""
    found = witness_scan(tmp_path)
    unclaimed = {u['literal'] for u in found['unclaimed_literals']}
    assert {'lower_by_name', 'not_a_diagnostic'} <= unclaimed
    assert 'SHAPED_BY_NAME' in {c['code'] for c in found['codes']}


def test_witness_an_added_or_removed_code_fails_the_comparison(tmp_path):
    base = {c['code'] for c in witness_scan(tmp_path)['codes']}
    added = {c['code'] for c in witness_scan(tmp_path, 'pub fn more() -> Diagnostic { emit("AddedMixedCase", "m") }')['codes']}
    assert differences(added, base) == (['AddedMixedCase'], [])
    assert differences(base, added) == ([], ['AddedMixedCase'])
