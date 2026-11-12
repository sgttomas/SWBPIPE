"""The table of diagnostic codes the solve crates construct agrees with the scanner's observation of the sources,
and each code is classified.

Exit criterion 3 (`docs/RELEASE_TARGET_v1.0.md`) needs every remaining refusal of a required capability
listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. `tools/validation/diagnostic_codes.py` reads the codes from
the production Rust sources; `fixtures/diagnostics/exact_route_codes.json` holds them with a hand-made
classification. The table's unit is the code: the value of the code field of a diagnostic, finding, refusal
or reason record that production code of the scanned crates constructs (its `scope` says what that excludes).
The table is a superset of what a solve publishes: a code in it is not thereby a refusal a person can meet,
and a limitation drafted for a refusal needs its own trace to the output a person consumes. The scope's
`constructed_not_published` list is kept by hand; this test does not check it.

What this test establishes: the table agrees with the scanner's observation of the sources over the producer
shapes the scanner lists (`PRODUCER_SHAPES`, the table's `scope.shapes_read`), each shape with a witness here
that adding or removing a code at it fails the test. It does not establish that the table is complete. The
shapes the scanner does not read are named in `scope.shapes_not_read`. A code position whose value the
scanner cannot resolve fails the test unless it is pinned by hand (`pinned_sites`) with its values and the
source text the reason rests on, which is checked, so that a change there fails the test too. The test also
fails when a record lacks a classification or a reason, when a value at a position known only by shape or
name appears or goes, and when the number of unsure records or of absent capabilities not named rises above
its pin.

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
from tools.validation.diagnostic_codes import (  # noqa: E402
    ARC_REASON_TOKENS, CLASSES, PRODUCER_SHAPES, TABLE, Scan, tokenize)

# The scan reads every production Rust file of these crates through their module trees.
CI_READS = ('core/product_physics/src/**', 'core/solver/*/src/**', 'core/loads/*/src/**',
            'core/analysis/hanger_sizing/src/**', 'core/numerics/*/src/**',
            'core/model_operations/operation_applier/src/**', 'fixtures/diagnostics/exact_route_codes.json',
            # each scanned crate's manifest: its path dependencies bound what a call or constant can reach
            'core/product_physics/Cargo.toml', 'core/solver/*/Cargo.toml', 'core/loads/*/Cargo.toml',
            'core/analysis/hanger_sizing/Cargo.toml', 'core/numerics/*/Cargo.toml',
            'core/model_operations/operation_applier/Cargo.toml',
            'docs/RELEASE_LIMITATIONS_v1.0.md')

# Pins: no count may grow unnoticed. When one falls, lower its pin to the new count. The last falls to zero
# only when every absent capability is named wholly by a limitation entry or by text the product publishes.
# 30 -> 36 (review of d776be92e): four of the newly found codes (NodeOutOfRange, NonFiniteAxialEffect,
# UnsupportedTargetForCategory, HANGER_SIZING_INPUT_TOO_LARGE) and the two reclassified two-use codes
# (EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED, JOINT_STIFFNESS_BASIS_UNSUPPORTED) are unsure.
# 36 -> 37 (completeness review): kernel_refused, newly inventoried when every JSON code key became a code
# position and its run-time value was pinned, is unsure (a witnessed mechanism is a model a user could correct).
UNSURE_PIN = 37
# 52 -> 56 (review of d776be92e): LIM-003 and LIM-004 name their codes only partially, so those two count as
# unnamed (finding F6); EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED is reclassified an absent capability (F4);
# HANGER_SIZING_INPUT_TOO_LARGE is newly found and classed one (F2). No refusal changed; the count was low.
# 56 -> 61 (completeness review): five source-recovery failure codes newly inventoried from the source
# receipt's failure records (support_attribution_ambiguous, unsupported_source_closure, unsupported_family,
# unsupported_block, unsupported_derived_quantity) are absent capabilities no entry names. No refusal changed.
UNNAMED_ABSENT_CAPABILITY_PIN = 61
# 47 -> 52 (review of d776be92e): the same four, and CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE, whose product
# text describes its gate only partially (F7). 52 -> 57 (completeness review): the same five.
NAMED_NOWHERE_TODAY_PIN = 57

REFRESH = 'run `python3 tools/validation/diagnostic_codes.py --write` and classify the new records by hand'
COVERAGE = ('whole', 'partial')

_SCAN = None


def scanned():
    """The scan of the repository, with the committed table's pins applied."""
    global _SCAN
    if _SCAN is None:
        _SCAN = Scan(PROJECT).run().apply_pins(table().get('pinned_sites'))
    return _SCAN


def scan():
    return scanned().table()


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


def test_no_code_position_is_left_unresolved_unless_pinned():
    """Nothing is dropped: a code position whose value the scanner cannot resolve is an unresolved site, and
    each fails here unless a pin in the table settles it. A pin that no longer holds (its site moved or
    resolved, or the source text its reason rests on changed) fails here too."""
    problems = list(scanned().pin_problems)
    problems += [f'{s["file"]}:{s["line"]} ({s["at"]}): {s["why"]}' for s in scan()['unresolved_sites']]
    assert not problems, f'unresolved code positions and pins that do not hold: {problems}; resolve each, ' \
        'or pin it by hand in pinned_sites with its values, reason and anchors'
    assert table()['unresolved_sites'] == [], REFRESH


def test_pinned_values_are_table_codes():
    codes = {c['code']: c for c in table()['codes']}
    problems = []
    for pin in table().get('pinned_sites', []):
        for value in pin['values']:
            sites = (codes.get(value) or {}).get('sites', [])
            if not any(s.get('pinned') and s['file'] == pin['file'] for s in sites):
                problems.append(f'{value}: not a table code with a pinned site in {pin["file"]}')
    assert not problems, problems


def test_unclaimed_literals_are_current():
    """A new value the scanner does not find raised (a code-shaped literal, or any value at a position known only
    by its shape or name) may be a code reaching a diagnostic by a route the scanner does not know: each is
    looked at and the table refreshed."""
    def literals(t):
        return sorted(x['literal'] for x in t['unclaimed_literals'])

    assert literals(scan()) == literals(table()), REFRESH


def test_the_scope_names_what_the_table_reads_and_leaves_out():
    """The scope lists the shapes the witnesses below exercise, and each reason token it places outside the unit
    stands as a text in the file it names and is printed by codes the table holds."""
    scope = table()['scope']
    assert scope['shapes_read'] == [shape for shape, _c, _v, _s in PRODUCER_SHAPES], REFRESH
    assert scope['reason_tokens_outside_the_unit'] == ARC_REASON_TOKENS, REFRESH
    codes = {c['code'] for c in table()['codes']}
    problems = []
    for t in ARC_REASON_TOKENS:
        path = PROJECT / t['produced_at'].split(':', 1)[0]
        if t['token'] not in [x.val for x in tokenize(path.read_text(encoding='utf-8')) if x.kind == 'str']:
            problems.append(f'{t["token"]}: not a text in {path}')
        problems += [f'{t["token"]}: {code} is not a table code' for code in t['printed_in'] if code not in codes]
    assert not problems, problems


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
fn untraced(code: &str) -> usize { code.len() }
pub struct Tagged { pub code: &'static str, pub n: u32 }

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
pub fn reason() -> String { serde_json::json!({"code": "reason_code", "phase": "p"}).to_string() }
pub fn by_name(code: &str) -> String { serde_json::json!({"code": code, "phase": "p"}).to_string() }
pub fn name_only() -> String { by_name("lower_by_name") }
pub fn shaped_by_name() -> String { by_name("SHAPED_BY_NAME") }
pub fn untraced_lower() -> usize { untraced("lower_untraced") }
pub fn plain_lower() -> Tagged { Tagged { code: "plain_lower", n: 2 } }
EXTRA

#[cfg(test)]
mod tests { fn t() { let _ = super::emit("TestOnly", "m"); } }
'''

FOUND = {'MixedCaseField', 'MixedCaseHelper', 'lower_case_code', 'MissingThing', 'other_thing', 'InputTooLarge',
         'jsonCode', 'reason_code', 'lower_by_name', 'SHAPED_BY_NAME'}
INSIDE_TEST_ONLY = {'TestOnly', 'ShapeInsideTestIf', 'ShapeInsideTestMatch', 'ShapeInsideTestFor'}


def witness_scan(tmp_path, extra='', name=None, pins=None):
    root = tmp_path / (name or ('with' if extra else 'base'))
    src = root / 'core/product_physics/src'
    src.mkdir(parents=True)
    (src / 'lib.rs').write_text(WITNESS.replace('EXTRA', extra), encoding='utf-8')
    scanner = Scan(root).run().apply_pins(pins)
    return scanner, scanner.table()


def test_witness_codes_are_found_in_every_kind_of_position_whatever_their_spelling(tmp_path):
    """A diagnostic record's code field, a helper's code argument passed on through another helper whatever the
    parameters are called, an enum-to-string producer read through a record's code, a typed error's code
    method and a JSON object's code key: each is found, mixed-case and lower-case alike; a test-only call is
    not."""
    _s, found = witness_scan(tmp_path)
    codes = {c['code'] for c in found['codes']}
    assert codes == FOUND, sorted(codes ^ FOUND)
    vias = {c['code']: {s['via'] for s in c['sites']} for c in found['codes']}
    assert any('typed producer' in v for v in vias['InputTooLarge'])
    assert any('enum code producer' in v for v in vias['MissingThing'])


def test_witness_values_at_unverified_positions_are_listed_not_dropped(tmp_path):
    """A non-code-shaped value at a position known only by a parameter's name, or in the code field of a struct
    not known as a diagnostic record, is in the unclaimed list; a code-shaped one there is a code."""
    _s, found = witness_scan(tmp_path)
    assert {u['literal'] for u in found['unclaimed_literals']} == {'lower_untraced', 'plain_lower'}


def test_witness_each_producer_shape_sees_a_code_added_and_removed(tmp_path):
    """For each shape the scanner reads, the witness source raises one code there: with every shape present, and
    with that one shape taken out, the comparison sees exactly that code added (the sources raise a code the
    table lacks) and removed (the table holds a code the sources no longer raise), at a site of that shape. A
    call inside a test-only `if`, `match` or `for` is not seen; the production statement after it is."""
    _s, full = witness_scan(tmp_path, '\n'.join(source for _shape, _c, _v, source in PRODUCER_SHAPES), 'all')
    codes = {c['code']: c for c in full['codes']}
    assert not INSIDE_TEST_ONLY & set(codes), sorted(INSIDE_TEST_ONLY & set(codes))
    assert full['unresolved_sites'] == []
    assert {u['literal'] for u in full['unclaimed_literals']} == {'lower_untraced', 'plain_lower'}
    problems = []
    for i, (shape, code, via, _source) in enumerate(PRODUCER_SHAPES):
        _s, without = witness_scan(tmp_path, '\n'.join(x[3] for j, x in enumerate(PRODUCER_SHAPES) if j != i), f'w{i}')
        held = {c['code'] for c in without['codes']}
        if differences(set(codes), held) != ([code], []):
            problems.append(f'{shape}: adding {code} shows {differences(set(codes), held)}')
        if differences(held, set(codes)) != ([], [code]):
            problems.append(f'{shape}: removing {code} shows {differences(held, set(codes))}')
        if via not in {s['via'] for s in codes.get(code, {'sites': []})['sites']}:
            problems.append(f'{shape}: {code} is not found at a site of this shape ({via})')
    assert not problems, problems


RUNTIME = 'pub fn s_runtime(k: &str) -> Diagnostic { emit(&format!("Built_{k}"), "m") }'


def runtime_pin(**change):
    pin = {'file': 'core/product_physics/src/lib.rs', 'at': 'emit()', 'why': 'built at run time (format!)',
           'site_text': 'emit(&format!("Built_{k}"), "m")', 'values': ['Built_one'],
           'reason': 'the witness says so', 'anchors': [{'file': 'core/product_physics/src/lib.rs',
                                                         'text': 'pub fn s_runtime(k: &str)'}]}
    pin.update(change)
    return [pin]


def test_witness_an_unresolved_code_position_fails_unless_its_pin_holds(tmp_path):
    """A value built at run time at a code position is kept as an unresolved site, never dropped. A pin whose
    site text and anchors stand turns it into the pinned values; a pin whose anchor or site has changed does
    not hold, and the site stays unresolved."""
    scanner, found = witness_scan(tmp_path, RUNTIME, 'unpinned')
    assert [(s['at'], s['why']) for s in found['unresolved_sites']] == [('emit()', 'built at run time (format!)')]
    assert not scanner.pin_problems

    scanner, found = witness_scan(tmp_path, RUNTIME, 'pinned', runtime_pin())
    assert found['unresolved_sites'] == [] and not scanner.pin_problems
    assert [s.get('pinned') for c in found['codes'] if c['code'] == 'Built_one' for s in c['sites']] == [True]

    moved = runtime_pin(anchors=[{'file': 'core/product_physics/src/lib.rs', 'text': 'pub fn s_runtime(key: &str)'}])
    scanner, found = witness_scan(tmp_path, RUNTIME, 'anchor_changed', moved)
    assert len(found['unresolved_sites']) == 1 and len(scanner.pin_problems) == 1
    assert 'Built_one' not in {c['code'] for c in found['codes']}

    scanner, found = witness_scan(tmp_path, RUNTIME.replace('Built_{k}', 'Built_{k}_2'), 'site_changed', runtime_pin())
    assert len(found['unresolved_sites']) == 1 and len(scanner.pin_problems) == 1
