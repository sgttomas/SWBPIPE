"""The table of diagnostic codes the solve crates construct agrees with the scanner's observation of the sources,
and each code is classified.

Exit criterion 3 (`docs/RELEASE_TARGET_v1.0.md`) needs every remaining refusal of a required capability
listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. `tools/validation/diagnostic_codes.py` reads the codes from
the production Rust sources; `fixtures/diagnostics/exact_route_codes.json` holds them with a hand-made
classification. The table's unit is the code: the value of the code field of a diagnostic, finding, refusal
or reason record that production code of the scanned crates constructs (its `scope` says what that excludes).
The table is conservative about publication among the constructions the scanner discovers: a code in it is not
thereby a refusal a person can meet, and the table is not an inventory of every refusal a person can meet. Whether
a code reaches a person is traced by hand for a few codes (the scope's `publication_traced`, with what each trace
read and when to recheck it) and is unresolved for the rest; this test does not check that list. The table
supports the work exit criterion 3 needs (from a required capability to its reachable refusal, its consequence
and a declared limitation) and does not do it.

What this test establishes: the table agrees with the scanner's observation of the sources over the shapes
and forms the scanner lists (`PRODUCER_SHAPES` and `READ_FORMS`, the table's `scope.shapes_read`), each with a
witness here (for a producer shape, that adding or removing a code at it fails the test); and every position
the scanner REPORTS as unresolved is pinned with its values or listed with its reason. It does not establish
that the table is complete, nor that every value at a code position was captured: the scanner's name, type,
binding, call-flow and statement-boundary heuristics can omit or misresolve a value without reporting it, and
`scope.shapes_not_read` names the mechanisms a fourth independent review constructed. The third and the fourth
independent reviews each found no code missing from the table in the present sources (the fourth after searching
them for each of its sixteen mechanisms), the witnesses worked, and the third's blind classification of the
sixteen codes then new agreed 16 of 16; that records what was searched for and not found, not completeness.

The test fails on: an unresolved site that is neither pinned (`pinned_sites`, its values and the whole
functions its reason rests on checked character for character) nor listed (`listed_unresolved_sites`, matched
by file, position, expression and the text of its line); a listed site whose text has changed or that no
longer stands; a statement under a test-only attribute that the stripper does not recognise (named by file and
line); a part of a value yielding no values outside the closed list of exemptions (it becomes an
unresolved site); a record without a classification or a reason; a value at a position known only by shape or
name appearing or going; and the number of unsure records or of absent capabilities not named rising above its
pin. Whether each listed entry's stated relay is true is hand-kept and not checked here.

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
import tools.validation.diagnostic_codes as scanner_module  # noqa: E402
from tools.validation.diagnostic_codes import (  # noqa: E402
    ARC_REASON_TOKENS, CLASSES, PRODUCER_SHAPES, EXEMPT_CASES, READ_FORMS, TABLE, Scan, listed_summary,
    listing_problems, tokenize)

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


def test_every_unresolved_code_position_is_pinned_or_listed():
    """Nothing is dropped silently: a code position whose value the scanner cannot resolve is an unresolved site,
    and each fails here unless a pin in the table settles it or an entry of listed_unresolved_sites lists it by
    the text of its line. A pin that no longer holds, or a listed entry whose site changed or went, fails too."""
    problems = list(scanned().pin_problems)
    problems += listing_problems(scan()['unresolved_sites'], table().get('listed_unresolved_sites', []))
    assert not problems, f'{problems}; resolve each site, pin it, or list it by hand with its reason'
    key = lambda u: (u['file'], u['at'], u['expression'], u['line_text'])
    assert sorted(map(key, scan()['unresolved_sites'])) == sorted(map(key, table()['unresolved_sites'])), REFRESH


def test_no_test_only_statement_is_guessed_at():
    """A test-only attribute on a statement form the stripper does not recognise is a failure by file and line."""
    assert scan()['strip_failures'] == [], scan()['strip_failures']
    assert table()['strip_failures'] == [], REFRESH


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
    assert scope['shapes_read'] == [shape for shape, _c, _v, _s in PRODUCER_SHAPES] + READ_FORMS, REFRESH
    assert scope['exempt_cases'] == EXEMPT_CASES, REFRESH
    assert scope['listed_unresolved'] == listed_summary(table().get('listed_unresolved_sites', [])), REFRESH
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


RUNTIME = """pub fn s_runtime(k: &str) -> Diagnostic {
    let kind = k.trim();
    emit(&format!("Built_{kind}"), "m")
}
pub fn s_vocabulary() -> &'static str { "one" }"""
RUNTIME_LIB = 'core/product_physics/src/lib.rs'


def function_text(name, source=RUNTIME):
    lines = source.split('\n')
    first = next(i for i, line in enumerate(lines) if f'fn {name}(' in line)
    last = next(i for i in range(first, len(lines)) if lines[i].rstrip().endswith('}') and
                (i > first or lines[i].count('{') == lines[i].count('}')))
    return '\n'.join(lines[first:last + 1])


def runtime_pin(**change):
    pin = {'file': RUNTIME_LIB, 'at': 'emit()', 'why': 'built at run time (format!)',
           'site_text': 'emit(&format!("Built_{kind}"), "m")', 'values': ['Built_one'],
           'reason': 'the witness says so',
           'anchors': [{'file': RUNTIME_LIB, 'function': 's_runtime', 'text': function_text('s_runtime')},
                       {'file': RUNTIME_LIB, 'function': 's_vocabulary', 'text': function_text('s_vocabulary')}]}
    pin.update(change)
    return [pin]


def test_witness_an_unresolved_code_position_fails_unless_its_pin_holds(tmp_path):
    """A value built at run time at a code position is kept as an unresolved site, never dropped. A pin whose
    site text stands and whose anchors are, unchanged, the whole function the site stands in and the whole
    function its reason rests on turns it into the pinned values. A pin does not hold, and the site stays
    unresolved, when: a line is inserted anywhere in the function that binds, builds and uses the value (the
    third review's mutation), the other anchored function changes, an anchor names no function of the file,
    no anchor is the function the site stands in, or the site itself changes."""
    scanner, found = witness_scan(tmp_path, RUNTIME, 'unpinned')
    assert [(s['at'], s['why']) for s in found['unresolved_sites']] == [('emit()', 'built at run time (format!)')]
    assert not scanner.pin_problems

    scanner, found = witness_scan(tmp_path, RUNTIME, 'pinned', runtime_pin())
    assert found['unresolved_sites'] == [] and not scanner.pin_problems, scanner.pin_problems
    assert [s.get('pinned') for c in found['codes'] if c['code'] == 'Built_one' for s in c['sites']] == [True]

    inserted = RUNTIME.replace('    let kind = k.trim();\n', '    let kind = k.trim();\n    let kind = kind.to_uppercase();\n')
    vocabulary = RUNTIME.replace('"one"', '"two"')
    unenclosed = runtime_pin(anchors=runtime_pin()[0]['anchors'][1:])
    unnamed = runtime_pin(anchors=[dict(runtime_pin()[0]['anchors'][0], function='s_absent')])
    for name, source, pins in [('inserted', inserted, runtime_pin()), ('vocabulary', vocabulary, runtime_pin()),
                               ('unenclosed', RUNTIME, unenclosed), ('unnamed', RUNTIME, unnamed),
                               ('site_changed', RUNTIME.replace('Built_{kind}', 'Built_{kind}_2'), runtime_pin())]:
        scanner, found = witness_scan(tmp_path, source, name, pins)
        assert len(found['unresolved_sites']) == 1 and len(scanner.pin_problems) == 1, (name, scanner.pin_problems)
        assert 'Built_one' not in {c['code'] for c in found['codes']}, name


NESTED_WRAPPER = """fn result_pair(b: bool) -> Result<(&'static str, u8), (&'static str, u8)> {
    if b { Ok(("ok_only", 0)) } else { Err(("error_only", 0)) }
}
pub fn s_nested(b: bool) { if let Err((code, _n)) = result_pair(b) { let _ = emit(code, "m"); } }"""


def test_witness_a_nested_wrapper_pattern_reads_only_its_arm(tmp_path):
    """`if let Err((code, ..)) = f()` reads the tuples f returns inside Err only: an Ok tuple is not a code
    there (third review, finding 6)."""
    _s, found = witness_scan(tmp_path, NESTED_WRAPPER, 'nested')
    codes = {c['code'] for c in found['codes']}
    assert 'error_only' in codes and 'ok_only' not in codes, sorted(codes - FOUND)


# ---------------------------------------------------------------- part two: nothing is dropped silently

def test_witness_no_values_outside_the_closed_list_is_an_unresolved_site(tmp_path):
    """The closed list of exemptions: a case not on it cannot be returned, every case on it is used by the
    scanner and no other is, and an expression that yields no values outside it is an unresolved site (here the
    resolver and the tuple reader are made to yield nothing, and every code position they read becomes one)."""
    try:
        scanner_module._none('not_on_the_list')
    except AssertionError:
        pass
    else:
        raise AssertionError('an exemption not on the list was accepted')
    source = (PROJECT / 'tools/validation/diagnostic_codes.py').read_text(encoding='utf-8')
    used = set(re.findall(r"_none\('([a-z_]+)'\)", source)) | set(re.findall(r"return None, '([a-z_]+)'", source))
    assert used == set(EXEMPT_CASES), (sorted(used - set(EXEMPT_CASES)), sorted(set(EXEMPT_CASES) - used))
    root = tmp_path / 'empty'
    (root / 'core/product_physics/src').mkdir(parents=True)
    (root / 'core/product_physics/src/lib.rs').write_text(WITNESS.replace('EXTRA', ''), encoding='utf-8')
    silent = Scan(root)
    silent._resolve = lambda *args, **kwargs: []
    silent._tuple_values = lambda *args, **kwargs: []
    found = silent.run().table()
    assert found['codes'] == [] or all(c['code'] in ('InputTooLarge', 'MissingThing', 'other_thing')
                                       for c in found['codes']), [c['code'] for c in found['codes']]
    assert len(found['unresolved_sites']) >= 8, found['unresolved_sites']
    assert {u['why'][:14] for u in found['unresolved_sites']} == {'no values from'}


REVIEWER_SHAPES = {
    'chain': 'pub fn s_chain() { for (code, _n) in vec![("first_code", 0)].into_iter().chain(vec![("second_code", 1)]) '
             '{ let _ = emit(code, "m"); } }',
    'new': 'pub struct Maker; impl Maker { fn new() -> (String, u8) { (format!("built_{}", 1), 0) } }\n'
           'pub fn s_new() -> Diagnostic { let (code, _n) = Maker::new(); emit(&code, "m") }',
    'struct_pattern': 'pub struct Reason { pub tag: String }\n'
                      'pub fn s_reason(k: &str) -> Diagnostic { let Reason { tag } = Reason { tag: format!("tag_{k}") }; '
                      'emit(&tag, "m") }',
}


def test_witness_the_third_reviews_three_shapes_fail_closed(tmp_path):
    """An iterator's .chain() argument, a constructor named new, and a struct pattern binding a field that is not
    a code field: each left an unresolved site neither silently empty nor assumed a relay, without a special case
    for any of them."""
    for name, extra in REVIEWER_SHAPES.items():
        _s, found = witness_scan(tmp_path, extra, f'reviewer_{name}')
        sites = [u for u in found['unresolved_sites'] if u['at'] == 'emit()']
        assert sites, (name, found['unresolved_sites'])


CHEAP_FORM = """pub struct Problem { pub code: &'static str, pub message: String }
fn check(x: u8) -> Result<u8, Problem> { if x > 1 { Ok(x) } else { Err(Problem { code: "PROBLEM_BUILT", message: String::new() }) } }
pub fn s_typed(x: u8) -> Option<Diagnostic> { if let Err(e) = check(x) { return Some(emit(e.code, "m")); } None }
pub fn s_arm(x: u8) -> Option<Diagnostic> { match check(x) { Err(e) => Some(emit(e.code, "m")), Ok(_) => None } }
pub fn s_iter(problems: &[Problem]) -> Vec<Diagnostic> { problems.iter().map(|p| emit(p.code, "m")).collect() }
pub fn s_for(problems: Vec<Problem>) -> Vec<Diagnostic> { let mut out = Vec::new(); for p in problems { out.push(emit(p.code, "m")); } out }"""


def test_witness_a_code_field_read_through_a_pattern_or_iterator_is_a_relay(tmp_path):
    """The cheap form: `e.code` where e is bound by Err(e) over a call declaring Result<_, Problem>, in an if let
    or a match arm, or by a closure parameter or a for variable over a named slice or Vec of Problem, is a relay
    of Problem's own construction (recorded there); the same read through a receiver of unknown type stays an
    unresolved site."""
    _s, found = witness_scan(tmp_path, CHEAP_FORM, 'cheap')
    assert found['unresolved_sites'] == [], found['unresolved_sites']
    assert 'PROBLEM_BUILT' in {c['code'] for c in found['codes']}
    # a crate the scanned one can name, defining a type with a code field the scanner does not read
    unknown = CHEAP_FORM + '\npub fn s_unknown(v: &serde_json::Value) -> Diagnostic { let p = pick(v); emit(p.code, "m") }'
    root = tmp_path / 'cheap_unknown'
    (root / 'core/product_physics/src').mkdir(parents=True)
    (root / 'core/product_physics/src/lib.rs').write_text(WITNESS.replace('EXTRA', unknown), encoding='utf-8')
    (root / 'core/product_physics/Cargo.toml').write_text('[dependencies]\nother = { path = "../other" }\n', encoding='utf-8')
    (root / 'core/other/src').mkdir(parents=True)
    (root / 'core/other/Cargo.toml').write_text('[package]\nname = "other"\n', encoding='utf-8')
    (root / 'core/other/src/lib.rs').write_text('pub struct Outside { pub code: String }\n', encoding='utf-8')
    found = Scan(root).run().table()
    assert [u['expression'] for u in found['unresolved_sites']] == ['p.code'], found['unresolved_sites']


STRIP_FORMS = {
    'labelled loop': "#[cfg(test)]\n    'outer: loop { let _ = emit(\"InsideLabel\", \"m\"); break 'outer; }",
    'brace macro': '#[cfg(test)]\n    debug_only! { let _ = emit("InsideMacro", "m"); }',
    'if with a block condition': '#[cfg(test)]\n    if { let _ = emit("InsideCondition", "m"); true } '
                                 '{ let _ = emit("InsideThen", "m"); }',
    'else if chain': '#[cfg(test)]\n    if a() { let _ = emit("InsideIf", "m"); } else if b() { let _ = emit("InsideElseIf", "m"); } '
                     'else { let _ = emit("InsideElse", "m"); }',
    'if let': '#[cfg(test)]\n    if let Some(x) = Some(1).map(|v| v + 1) { let _ = emit("InsideIfLet", "m"); let _ = x; }',
    'while': '#[cfg(test)]\n    while a() { let _ = emit("InsideWhile", "m"); }',
    'loop': '#[cfg(test)]\n    loop { let _ = emit("InsideLoop", "m"); break; }',
    'match with a method chain': '#[cfg(test)]\n    match 1 { _ => emit("InsideChain", "m") }.code.len();',
    'let': '#[cfg(test)]\n    let _x = emit("InsideLet", "m");',
    'unsafe block': '#[cfg(test)]\n    unsafe { let _ = emit("InsideUnsafe", "m"); }',
}
ITEM_FORMS = {
    'const fn': '#[cfg(test)]\nconst fn note() -> &\'static str { "InsideConstFn" }',
    'thread_local! macro item': '#[cfg(test)]\nthread_local! { static LOCAL: u8 = const { 0 }; }',
    'pub field': 'pub struct Held { #[cfg(test)] pub probe: Option<u8>, pub kept: u8 }',
    'item fn': '#[cfg(test)]\nfn only_in_tests() -> Diagnostic { emit("InsideItemFn", "m") }',
}


def test_witness_each_statement_form_the_stripper_recognises(tmp_path):
    """For each statement form the stripper recognises under a test-only attribute: what is inside it is not read,
    the production statement after it is, and nothing fails to strip. Items likewise, with a production function
    after them."""
    problems = []
    for i, (form, test_only) in enumerate(STRIP_FORMS.items()):
        extra = f'pub fn s_strip_{i}() -> Diagnostic {{\n    {test_only}\n    emit("AfterForm{i}", "m")\n}}'
        _s, found = witness_scan(tmp_path, extra, f'strip_{i}')
        codes = {c['code'] for c in found['codes']}
        inside = set(re.findall(r'"(Inside[A-Za-z]+)"', test_only))
        if f'AfterForm{i}' not in codes or inside & codes or found['strip_failures']:
            problems.append((form, sorted(inside & codes), f'AfterForm{i}' in codes, found['strip_failures']))
    for i, (form, test_only) in enumerate(ITEM_FORMS.items()):
        extra = f'{test_only}\npub fn s_after_item_{i}() -> Diagnostic {{ emit("AfterItem{i}", "m") }}'
        _s, found = witness_scan(tmp_path, extra, f'item_{i}')
        codes = {c['code'] for c in found['codes']}
        inside = set(re.findall(r'"(Inside[A-Za-z]+)"', test_only))
        if f'AfterItem{i}' not in codes or inside & codes or found['strip_failures']:
            problems.append((form, sorted(inside & codes), f'AfterItem{i}' in codes, found['strip_failures']))
    assert not problems, problems


def test_witness_an_unrecognised_test_only_form_fails_by_name(tmp_path):
    """A test-only attribute on a form the stripper does not recognise (a labelled block, an inline const block) is
    not guessed at: it is a strip failure naming the file and line, and the form is read as production."""
    for name, form in [('labelled block', "#[cfg(test)]\n    'blk: { let _ = emit(\"ReadAsProduction\", \"m\"); }"),
                       ('inline const', '#[cfg(test)]\n    const { let _ = emit("ReadAsProduction", "m"); }')]:
        extra = f'pub fn s_unknown_form() -> Diagnostic {{\n    {form}\n    emit("AfterUnknown", "m")\n}}'
        _s, found = witness_scan(tmp_path, extra, f'unknown_{name.replace(" ", "_")}')
        failures = found['strip_failures']
        assert len(failures) == 1 and failures[0]['file'] == 'core/product_physics/src/lib.rs', (name, failures)
        assert {'ReadAsProduction', 'AfterUnknown'} <= {c['code'] for c in found['codes']}, name


def test_witness_listing_fails_on_an_unlisted_or_changed_site(tmp_path):
    """An unresolved site with no entry fails; an entry made from it passes; when the site's line changes, the old
    entry no longer stands and the changed site is unlisted, so both fail; an entry needs its reason."""
    _s, found = witness_scan(tmp_path, RUNTIME, 'listing')
    sites = found['unresolved_sites']
    assert len(sites) == 1
    assert listing_problems(sites, []) and listing_problems(sites, [])[0].startswith('unlisted unresolved site')
    entry = {k: sites[0][k] for k in ('file', 'at', 'expression', 'line_text')}
    entry.update(form='a value built at run time', outside_unit=False, not_known=True, reason='the witness')
    assert listing_problems(sites, [entry]) == []
    _s, changed = witness_scan(tmp_path, RUNTIME.replace('"m")', '"changed")'), 'listing_changed')
    problems = listing_problems(changed['unresolved_sites'], [entry])
    assert any(p.startswith('unlisted') for p in problems) and any(p.startswith('listed site that no longer') for p in problems)
    assert listing_problems(sites, [dict(entry, reason='')]), 'an entry marked not known needs its reason'
