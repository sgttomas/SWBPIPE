"""Generate the load-reference VP-STATIC reference values and criteria candidates.

Every expected value, every named wrong value and every zero-reference scale is
read from the maintained independent analytical references
``core/product_physics/tests/fixtures/load_reference_states/reference_cases.json``
at run time. No constant is copied. The pointer, the declared transform and the
unit conversion for each assertion are stated in the case's selector file
(``selector_origin.reference_origin``); this script only evaluates them.

Evaluation follows the reference README: ``exact`` is authoritative. Rationals
and rational multiples of pi are evaluated in decimal at 120 significant
digits (pi from ``numeric_representation.pi_decimal``); ``symbolic`` exacts of
the forms ``exp(p/q)-1`` and ``1-exp(p/q)`` are re-evaluated with
``Decimal.exp`` at the same precision. Every evaluation is cross-checked
against the file's ``decimal`` (to 1e-80 relative) and ``value`` (bitwise)
fields. Declared transforms (negation, ``1 + x``, the generic-to-annulus area
ratio) and exact unit factors (m to mm) are applied in decimal, and the result
is rounded once to binary64.

No product module is imported. No observed producer value is read.

Usage (WORKING_ROOT is ``projects/chirality-piping``)::

    python validation/qualification/fixtures/load_reference/generate_reference_values.py --write
    python validation/qualification/fixtures/load_reference/generate_reference_values.py --check

``--check`` regenerates every reference and criteria candidate in memory and
requires byte equality with the committed files. It exits non-zero on any
difference.
"""
from __future__ import annotations

import argparse
import decimal
from decimal import Decimal
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import re
import sys

PACKAGE = Path(__file__).resolve().parent
WORKING_ROOT = PACKAGE.parents[3]
REFERENCE_PATH = 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'
SELECTOR_FORMAT = 'openpipestress.first_static_selector_candidate/1'
REFERENCE_FORMAT = 'openpipestress.qualification_reference_values/1'
PRODUCER_CONTRACT = 'openpipestress.result_semantics/0.3.0/load-reference-1'
RELATIVE = Decimal('1e-9')
CONTEXT = decimal.Context(prec=120, rounding=decimal.ROUND_HALF_EVEN)
TRANSFORMS = ('identity', 'negate', 'one_plus', 'generic_area_to_annulus')
# Only exact, definitional unit factors. Anything else is refused.
UNIT_FACTORS = {
    ('m', 'm'): Decimal(1), ('m', 'mm'): Decimal(1000), ('mm', 'mm'): Decimal(1),
    ('N', 'N'): Decimal(1), ('N*m', 'N*m'): Decimal(1), ('rad', 'rad'): Decimal(1),
    ('1', '1'): Decimal(1), ('K', 'K'): Decimal(1), ('Pa', 'Pa'): Decimal(1),
}


class GenerationError(ValueError):
    pass


def require(ok: bool, message: str) -> None:
    if not ok:
        raise GenerationError(message)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_reference() -> tuple[dict, bytes]:
    data = (WORKING_ROOT / REFERENCE_PATH).read_bytes()
    return json.loads(data), data


def resolve(document: dict, pointer: str):
    require(pointer.startswith('/'), f'JSON pointer required: {pointer}')
    node = document
    for raw in pointer[1:].split('/'):
        key = raw.replace('~1', '/').replace('~0', '~')
        if isinstance(node, list):
            require(key.isdigit() and int(key) < len(node), f'pointer index {key} missing in {pointer}')
            node = node[int(key)]
        else:
            require(isinstance(node, dict) and key in node, f'pointer key {key} missing in {pointer}')
            node = node[key]
    return node


def pi_decimal(document: dict) -> Decimal:
    return Decimal(document['numeric_representation']['pi_decimal'])


SYMBOLIC = re.compile(r'^(?:exp\((?P<a>-?\d+/\d+)\)-1|1-exp\((?P<b>-?\d+/\d+)\))$')


def exact_decimal(document: dict, quantity: dict, pointer: str) -> Decimal:
    """Authoritative exact value at 120 digits, cross-checked with the file."""
    require(isinstance(quantity, dict) and isinstance(quantity.get('exact'), dict), f'not a maintained quantity: {pointer}')
    exact = quantity['exact']
    kind = exact.get('kind')
    allowed = {'unit', 'exact', 'decimal', 'value'} | ({'evaluation'} if kind == 'symbolic' else set())
    require(set(quantity) == allowed, f'not a maintained quantity: {pointer}')
    with decimal.localcontext(CONTEXT):
        if kind in ('rational', 'rational_times_pi', 'rational_over_pi'):
            fraction = Fraction(exact['rational'])
            result = Decimal(fraction.numerator) / Decimal(fraction.denominator)
            if kind == 'rational_times_pi':
                result = result * pi_decimal(document)
            elif kind == 'rational_over_pi':
                result = result / pi_decimal(document)
        elif kind == 'symbolic':
            # The file evaluates these with Decimal.exp; re-evaluate the stated
            # expression independently at 120 digits.
            require('Decimal.exp' in quantity.get('evaluation', ''), f'unsupported symbolic evaluation at {pointer}')
            match = SYMBOLIC.match(exact.get('expression', ''))
            require(match is not None, f'unsupported symbolic expression at {pointer}')
            argument = Fraction(match.group('a') or match.group('b'))
            power = (Decimal(argument.numerator) / Decimal(argument.denominator)).exp()
            result = power - 1 if match.group('a') else 1 - power
        else:
            raise GenerationError(f'unknown exact kind {kind!r} at {pointer}')
        rendered = Decimal(quantity['decimal'])
        if result != 0:
            require(abs((result - rendered) / result) <= Decimal('1e-80'),
                    f'exact/decimal disagreement at {pointer}')
        else:
            require(rendered == 0, f'exact zero rendered nonzero at {pointer}')
    require(to_binary64(result) == float(quantity['value']), f'binary64 projection disagreement at {pointer}')
    return result


def to_binary64(value: Decimal) -> float:
    result = float(value)  # correctly rounded
    return 0.0 if result == 0.0 else result


def quantity_value(document: dict, spec: dict, label: str) -> tuple[Decimal, str]:
    """Decimal value and unit named by {pointer, reference_unit[, plain_number]}."""
    pointer = spec['pointer']
    node = resolve(document, pointer)
    unit = spec['reference_unit']
    if spec.get('plain_number'):
        require(type(node) in (int, float) and type(node) is not bool, f'{label}: plain number expected at {pointer}')
        return Decimal(repr(node)) if isinstance(node, float) else Decimal(node), unit
    require(node.get('unit') == unit, f'{label}: unit {node.get("unit")!r} at {pointer}, declared {unit!r}')
    return exact_decimal(document, node, pointer), unit


def convert(value: Decimal, from_unit: str, to_unit: str, label: str) -> Decimal:
    factor = UNIT_FACTORS.get((from_unit, to_unit))
    require(factor is not None, f'{label}: no exact unit factor {from_unit} -> {to_unit}')
    with decimal.localcontext(CONTEXT):
        return value * factor


def origin_value(document: dict, origin: dict, target_unit: str, label: str) -> Decimal:
    kind = origin.get('kind')
    if kind == 'equilibrium_zero':
        require(bool(origin.get('reason')), f'{label}: equilibrium zero needs a stated reason')
        return Decimal(0)
    require(kind == 'analytical', f'{label}: unknown reference origin kind {kind!r}')
    transform = origin.get('transform')
    require(transform in TRANSFORMS, f'{label}: unknown transform {transform!r}')
    value, unit = quantity_value(document, origin, label)
    with decimal.localcontext(CONTEXT):
        if transform == 'negate':
            value = -value
        elif transform == 'one_plus':
            require(unit == '1', f'{label}: one_plus needs a dimensionless quantity')
            value = 1 + value
        elif transform == 'generic_area_to_annulus':
            # Generic-area discriminator scaled to the maintained annulus: * As / area.
            ratio_spec = origin['area_ratio']
            area_as, u1 = quantity_value(document, ratio_spec['numerator'], label)
            area_generic, u2 = quantity_value(document, ratio_spec['denominator'], label)
            require(u1 == u2 == 'm^2', f'{label}: area ratio needs two m^2 quantities')
            value = value * area_as / area_generic
    return convert(value, unit, target_unit, label)


def zero_scale_value(document: dict, scale: dict, target_unit: str, label: str) -> Decimal:
    require(isinstance(scale, dict) and bool(scale.get('tag')), f'{label}: zero scale needs a tag')
    base, base_unit = quantity_value(document, scale['base'], label)
    with decimal.localcontext(CONTEXT):
        magnitude = abs(base)
        multipliers = scale.get('multiply_by_sum_of') or []
        if multipliers:
            total = Decimal(0)
            for item in multipliers:
                value, _unit = quantity_value(document, item, label)
                total += value
            magnitude = magnitude * abs(total)
    require(magnitude > 0, f'{label}: zero scale must be a nonzero magnitude')
    return convert(magnitude, scale['scale_unit'], target_unit, label)


def selector_unit_dimension(assertion: dict) -> tuple[str, str]:
    selector = assertion['selector']
    return selector['unit'], selector['dimension']


def rule_for(assertion: dict, expected: float, document: dict) -> dict:
    """The candidate criterion rule an assertion names, computed from the reference."""
    unit, dimension = selector_unit_dimension(assertion)
    origin = assertion['selector_origin']
    family = origin['family']
    rule_id = assertion['criterion_rule_id']
    prefix = f'criterion:{family}:{dimension}:{unit}:'
    common = {
        'dimension_id': dimension,
        'result_family': family,
        'unit_ref': {'ref_type': 'unit', 'ref': unit},
        'normalization_basis': 'same_unit_required',
        'tolerance_value_status': 'project_specific_review_required',
        'review': (ADMITTED_REVIEW if ADMITTED else
                   'pending the independent WP6 freeze check; candidate only, not admitted'),
    }
    if expected != 0.0:
        require(rule_id == prefix + 'relative_1e-9', f'{assertion["id"]}: nonzero reference needs the relative rule')
        return {'rule_id': rule_id, **common, 'relative_tolerance_value': 1e-09, 'absolute_tolerance_value': 0.0,
                'provenance': ('reference_cases.json README consumer rule: |observed - expected| <= 1e-9*|expected| '
                               '(existing protected relative criterion); no new or relaxed threshold')}
    scale = origin['reference_origin'].get('zero_scale')
    require(scale is not None, f'{assertion["id"]}: zero reference needs a zero scale')
    require(rule_id == prefix + 'zero_scale:' + scale['tag'], f'{assertion["id"]}: zero reference rule id mismatch')
    magnitude = zero_scale_value(document, scale, unit, assertion['id'])
    with decimal.localcontext(CONTEXT):
        budget = RELATIVE * magnitude
    parts = [f'|{scale["base"]["pointer"]}|']
    if scale.get('multiply_by_sum_of'):
        parts.append('|' + ' + '.join(item['pointer'] for item in scale['multiply_by_sum_of']) + '|')
    return {'rule_id': rule_id, **common, 'relative_tolerance_value': 0.0,
            'absolute_tolerance_value': to_binary64(budget),
            'provenance': ('existing dimension-aware zero-reference handling (README consumer rule; '
                           'load_reference_state_runtime.rs close()): absolute floor 1e-9*zero_scale; zero_scale = '
                           + ' * '.join(parts) + f' = {magnitude.normalize():f} {unit} (zero-scale tag {scale["tag"]})')}


ADMITTED: dict | None = None
ADMITTED_REVIEW = ('reviewed in the independent WP6 freeze named in ADMISSION.json (verdict FREEZE, admit as '
                   'changed); admitted as a project-specific development criterion, not a release criterion')
ADMISSION_FORMAT = 'openpipestress.load_reference_admission/1'


def admission() -> dict | None:
    """The manager's admission record, written only after the independent freeze.

    Absent: every file stays a pending candidate. Present: the independent
    review it names must exist with the recorded sha256, and the generated
    files carry that review as their admission. Values and rules never change.
    """
    path = PACKAGE / 'ADMISSION.json'
    if not path.exists():
        return None
    record = json.loads(path.read_bytes())
    require(set(record) == {'format', 'status', 'independent_review', 'applied_changes', 'admitted_by'},
            'ADMISSION.json keys')
    require(record['format'] == ADMISSION_FORMAT and record['status'] == 'admitted', 'ADMISSION.json identity')
    review = record['independent_review']
    require(set(review) == {'path', 'sha256'}, 'ADMISSION.json review keys')
    rel = Path(review['path'])
    require(not rel.is_absolute() and '..' not in rel.parts, 'ADMISSION.json review path must be WORKING_ROOT-relative')
    working_root = PACKAGE.parents[3]
    require(sha256_bytes((working_root / rel).read_bytes()) == review['sha256'], 'ADMISSION.json review hash')
    return record


def build_case(selectors: dict, document: dict, reference_sha256: str) -> tuple[dict, dict]:
    require(selectors.get('format') == SELECTOR_FORMAT, 'selector format')
    require(selectors.get('producer_contract') == PRODUCER_CONTRACT, 'producer contract')
    case_id = selectors['case_id']
    values, wrong_values, rules, seen = [], [], [], {}

    def add_rule(rule: dict) -> None:
        if rule['rule_id'] in seen:
            require(seen[rule['rule_id']] == rule, f'rule {rule["rule_id"]} is defined inconsistently')
            return
        seen[rule['rule_id']] = rule
        rules.append(rule)

    ids = set()
    for assertion in selectors['assertions']:
        require(assertion['id'] not in ids, f'duplicate assertion {assertion["id"]}')
        ids.add(assertion['id'])
        unit, _ = selector_unit_dimension(assertion)
        origin = assertion['selector_origin']['reference_origin']
        expected = to_binary64(origin_value(document, origin, unit, assertion['id']))
        values.append({'assertion_id': assertion['id'], 'value': expected, 'unit': unit})
        add_rule(rule_for(assertion, expected, document))
    positives = {a['id']: a for a in selectors['assertions']}
    expected_by_id = {v['assertion_id']: v['value'] for v in values}
    for negative in selectors.get('negative_assertions', []):
        require(negative['id'] not in ids, f'duplicate assertion {negative["id"]}')
        ids.add(negative['id'])
        unit, _ = selector_unit_dimension(negative)
        origin = negative['selector_origin']['reference_origin']
        wrong = to_binary64(origin_value(document, origin, unit, negative['id']))
        rule = rule_for(negative, wrong, document)
        add_rule(rule)
        # A discriminator must itself be distinct from the correct reference under its rule.
        target = negative['selector_origin']['negates_assertion']
        require(target in positives, f'{negative["id"]}: unknown negated assertion')
        require(positives[target]['selector'] == negative['selector'], f'{negative["id"]}: selector differs from target')
        correct = expected_by_id[target]
        allowed = max(rule['absolute_tolerance_value'], rule['relative_tolerance_value'] * max(abs(correct), abs(wrong)))
        require(abs(correct - wrong) > allowed, f'{negative["id"]}: discriminator not distinct from the reference')
        wrong_values.append({'assertion_id': negative['id'], 'value': wrong, 'unit': unit,
                             'discriminator': origin['pointer']})
    reference = {
        'format': REFERENCE_FORMAT,
        'case_id': case_id,
        'readiness': 'ready' if ADMITTED else 'pending_independent_review',
        'basis': (f'generate_reference_values.py evaluating {REFERENCE_PATH} (sha256 {reference_sha256}) at the '
                  'pointers, transforms and exact unit factors in the selector file; exact-first, rounded once to '
                  'binary64; no observed producer value'),
        'reference_kind': 'analytical',
        'independent_review_ref': (f"{ADMITTED['independent_review']['path']}#sha256={ADMITTED['independent_review']['sha256']}"
                                   if ADMITTED else None),
        'values': values,
        'wrong_values': wrong_values,
    }
    criteria = {
        'schema_version': '0.1.0',
        'tolerance_profile': {
            'profile_id': f'load-reference-vp-static-{case_id}-v1',
            'profile_status': 'reviewed' if ADMITTED else 'draft_pending_independent_review',
            'scope': (('This case only. Comparison rules proposed per assertion family from the reference '
                       'README consumer rule and admitted after the independent freeze named in ADMISSION.json; '
                       'not a release criterion.') if ADMITTED else
                      ('This case only. Candidate comparison rules proposed per assertion family from the '
                       'reference README consumer rule; not admitted, not a release criterion.')),
            'rules': rules,
        },
    }
    return reference, criteria


def dump(value: dict) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False) + '\n').encode('utf-8')


def generate() -> dict[Path, bytes]:
    global ADMITTED
    ADMITTED = admission()
    document, data = load_reference()
    reference_sha256 = sha256_bytes(data)
    outputs = {}
    for path in sorted(PACKAGE.glob('*.selectors.candidate.json')):
        selectors = json.loads(path.read_bytes())
        stem = path.name[: -len('.selectors.candidate.json')]
        require(selectors['case_id'] == stem, f'{path.name}: case_id must equal the file stem')
        reference, criteria = build_case(selectors, document, reference_sha256)
        outputs[PACKAGE / f'{stem}.reference.candidate.json'] = dump(reference)
        outputs[PACKAGE / f'{stem}.criteria.candidate.json'] = dump(criteria)
    require(bool(outputs), 'no selector files found')
    return outputs


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--write', action='store_true')
    group.add_argument('--check', action='store_true')
    args = parser.parse_args(argv)
    outputs = generate()
    failures = 0
    for path, data in outputs.items():
        if args.write:
            path.write_bytes(data)
            print(f'wrote {path.name} {sha256_bytes(data)}')
        else:
            current = path.read_bytes() if path.exists() else None
            status = 'ok' if current == data else 'DIFFERS'
            failures += status != 'ok'
            print(f'{status} {path.name} {sha256_bytes(data)}')
    print(f'{len(outputs)} files, {failures} differences')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
