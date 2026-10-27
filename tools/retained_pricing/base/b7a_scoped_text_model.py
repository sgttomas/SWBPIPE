"""Closed current-source spellings; no global message/error class changes."""
from copy import deepcopy
import hashlib
from pathlib import Path
import runpy

MARKER = 'b7a_scoped_text'
WHY = 'B7a scoped TEXT: exact current producer and admitted source/type closure'
UNITS = 'core/units/src/lib.rs'
UNITS_SHA = '521717eb4cac8114b5c1bbeef3f8caebd57eb3021c7743860f9be8f1078b710c'
PP = 'core/product_physics/src/'
RE = 'core/reporting/result_export/src/'
# B7b: the snapshot lines of the bound sites (B7a 3e89c23ea1 values in comments).
# The inherited full-file pins bind these files; each line keeps B7a's text.
# T4-U6/U8 (PP head 20c6126a16): every line moved with unchanged text (B7b values in brackets). Epoch addbdc030e:
# lib.rs's suffix and diag lines +36 more (the realized-arc marker's helpers). Final source 588de796a4: lib.rs's
# lines +5 more (PP4's withhold_certified_summaries), physics_source.rs's require +1 (RD3's `bit_equal` import).
# W1-L5: the lib.rs aggregation changes shift these unchanged tail sites by +2.
# T4-S5 (epoch): every line moved with unchanged text: pressure_runtime.rs -3 (ARC_WITHHELD_REASON removed) and -25
# more after exact_member_recovery (the arc maximum policy removed), +3 after assemble_pressure_case's bend radius;
# lib.rs +314/+356; retained_product.rs +27; preview_physics.rs +31.
# O4 S5 (retained-pricing-rules.md §8.9): every line moved with unchanged text: lib.rs +4 before the case solve's
# maximum (0 after it: SOURCE_MAXIMUM_BASES_DISTINCT and its comment, -4, offset the case-solve +3 and module +3 lines);
# physics_source.rs +29 (the contract and allowance readers); retained_product.rs +167 (the O4 sidecars).
# Pre-0.4 failure unification D3 (§8.14): lib.rs's four bound lines move +103 with unchanged text (the republication,
# the fail-closed form, the screen hook and the support-norm cause precede them).
# W-8 topology warnings D3 (§8.16): the four lib.rs lines move +2 with unchanged text (`mod topology;` and its call).
# Silent-zero guard D3 (§8.15): +1 more with unchanged text (refuse_all_reference_states' call precedes them).
# Model document 0.7.0 D3 (§8.17): the four lib.rs lines move +7 with unchanged text (`model_settings`' module, re-export,
# PreviewModel and wire members and their move).
# W-7 D3 (§8.20): with unchanged text, the unit message moves +37 (the case solve's member row position and global end
# block precede it) and the three tail lines +156 (append_element_global_end_results and its constants precede them).
# W-15 D3 (§8.21): with unchanged text, the unit message moves +103 and the three tail lines +111 (W-15's ResultItem
# record and ComponentBody, BuiltModel's scaled members, the formed diagnostics, the eligibility scales and the thermal
# frame reads precede them).
# W-16 slice a D3 (§8.23): with unchanged text, the unit message and the three tail lines move +35 and +39
# (LoadTargetInput's support arm, the imposed-motion validation, the case solve's prescribed map and, for the tail,
# the primitive filter precede them).
LN = {
    'problem': 159,       # pressure_runtime.rs fn problem (160) [148]
    'diag': 165,          # `let mut finding = diag(` (166) [154]
    'format': 166,        # `&format!(` (167) [155]
    'join': 168,          # `stable_suffix(&refs.join(":"))` (169) [157]
    'to_string': 173,     # `refs.iter().map(|value| value.to_string()).collect(),` (174) [162]
    # Nested-record closure D3 (§8.25): the four lib.rs lines move +106 with unchanged text; model document 0.8.0 (§8.26):
    # +36 (unit_message) and +36 (the three helpers), unchanged text.
    'unit_message': 10211, # lib.rs `message.into()` (9162) [9159]
    'suffix': 14989,      # lib.rs `id.replace(':', "-")` (13614) [13619]
    'diag_fn': 14938,     # lib.rs fn diag (13563) [13568]
    'suffix_fn': 14988,   # lib.rs fn stable_suffix (13613) [13618]
    'require': 125,       # physics_source.rs `Err(format!("PHYSICS_SOURCE_{detail}"))` (89; 62 before ccb19ba0bf) [78]
}
PR = PP + 'pressure_runtime.rs:'
PROBLEM = PR + '%d:problem' % LN['problem']
REF_LOOP = 'iter:refs.iter().map'
RANGE_CONTENT = 59
RANGE_ORIGINS = {
    'operand bound is not finite',
    'exact formation on a product term',
    'exact formation scale is not 1 or 3',
    '12-scaled value is out of range',
    'intended component is not finite',
    'rounded-product bound is not finite',
    'rounded-product defect is out of range',
    'formation bound is not finite',
    'self-equilibrated magnitude is out of range',
    'intended net is outside the binary64 range',
}
_REQ = RE + 'physics_source.rs:%d' % LN['require']
_FMT, _JOIN = PR + str(LN['format']), PR + str(LN['join'])
_UNIT, _SUF = PP + 'lib.rs:%d' % LN['unit_message'], PP + 'lib.rs:%d' % LN['suffix']
AUDIT_PREIMAGES = {
    _REQ: {'detail': {'bytes': 8192, 'source': 'STATIC'}},
    _FMT: {
        'code': {'bytes': 2173, 'source': 'STATIC'},
        'stable_suffix(&refs.join(":"))': {'bytes': 148, 'source': 'TPL148'}},
    _JOIN: {'refs': {'bytes': 608, 'source': 'IN128'}},
    _UNIT: {'message': {'bytes': 8192, 'source': 'NOTID'}},
    _SUF: {'id': {'bytes': 128, 'source': 'IN128'}},
    PP + 'formation_guard.rs:239': None,
}
# B7b: physics_source.rs's require calls pass literal details, the longest
# 37 B (ACTUAL_MATERIAL_DUPLICATE_TEMPERATURE), so 40 holds; 033626030c's
# LOAD_REFERENCE_CASE_ORDER (25 B) and LOAD_REFERENCE_REQUESTED_MODE (29 B) too.
AUDIT_CURRENT = {
    _REQ: {'detail': {'bytes': 40, 'source': 'STATIC'}},
    _FMT: {
        'code': {'bytes': 45, 'source': 'STATIC'},
        'stable_suffix(&refs.join(":"))': {'bytes': 644, 'source': 'COMP'}},
    _JOIN: {'refs': {'bytes': 644, 'source': 'COMP'}},
    _UNIT: {'message': {'bytes': 600, 'source': 'NOTID'}},
    _SUF: {'id': {'bytes': 128, 'source': 'IN128'}},
    PP + 'formation_guard.rs:239': {'failure': {'bytes': 43, 'source': 'STATIC'}},
}
LEX_CURRENT = {
    'retained_product.rs:3389': {'class': 46, 'why': WHY + '; requested mode literal'},   # [B7b 3028; T4-S5 3206]
    'retained_product.rs:3546': {'class': 167, 'why': WHY + '; straight endpoint sign literal'},   # [B7b 3185; T4-S5 3363]
    # W-15 D3 (§8.21): preview_physics.rs 632 -> 635 and 598 -> 601 (render_members' `bodies` parameter and render's
    # body set), unchanged text.
    'preview_physics.rs:635': {'class': 47, 'why': WHY + '; arc basis literal'},
    'lib.rs:%d' % LN['unit_message']: {'class': 600, 'why': WHY + '; unit-conversion owned message'},
    'pressure_runtime.rs:%d' % LN['join']: {'class': 644, 'why': WHY + '; complete five-ref joined spelling'},
}
# B7b: problem() keeps its nine callers; B7b's new validate_profile messages are
# at most 327 B and at most two refs, within the unchanged 453/760 bounds.
MESSAGE_CURRENT = {'pressure_runtime.rs:%d' % LN['diag']: 453}
REFS_CURRENT = {'pressure_runtime.rs:%d' % LN['diag']: 760}
EXTRA_CURRENT = {'lib.rs:%d' % LN['suffix']: {'owner': PROBLEM, 'extra_content': 516, 'capacity': 2}}
OWNER_LOOP = {'owner': PROBLEM, 'header': REF_LOOP, 'per_call': 5}
PROBLEM_SITES = [(LN['diag'], 'diag'), (LN['format'], 'format'), (LN['join'], 'join'), (LN['to_string'], 'to_string')]
PROBLEM_PARENTS = [PR + x for x in (   # B7a: 188, 234, 531, 545, 618, 1091, 1332, 1480, 1720
    # T4-U6 [B7b: 176, 222, 507, 521, 594, 1067, 1308, 1456, 1696]; epoch addbdc030e: -3 after
    # assemble_pressure_case's `realized_arc` call
    # W-15 D3 (§8.21): validate_profile's reducer refusal (one more call of `problem` in validate_profile, 0 on D1: P2,
    # D1.5) moves the later parents +6 with unchanged text.
    '187:exact_member_recovery', '211:validate_profile', '500:check_suffixes',
    '514:normalize_region_units', '587:assemble_pressure_case', '1066:add_joint_terms',
    '1316:finish_source_groups', '1464:traverse_region', '1704:collinear_run')]
# U5B-JV (T4-U5b): joint.rs `hardware`'s match binding `Err(Some(problem)) => problems.push(problem)` gives the
# by-name value scan one more graph parent of `problem`, which hardware never calls; o10_rules.py zeroes that edge, and
# the owner's message/ref bounds are unchanged (hardware passes neither).
JOINT_VALUE_PARENT = PP + 'joint.rs:226:hardware'


def adjacent(name):
    return runpy.run_path(str(Path(__file__).parent / name))


def inspect(snapshot):
    proof = adjacent('b7a_formation_model.py')['inspect'](snapshot)
    raw = (Path(snapshot) / UNITS).read_bytes()
    if hashlib.sha256(raw).hexdigest() != UNITS_SHA:
        raise ValueError('B7a scoped TEXT: unit source changed')
    # The inherited full ledger and formation source pins bind the only
    # constructor/write of Option<&static str>. These exact literal origins
    # provide the range bound rather than an arbitrary Error-display class.
    model = adjacent('kd5_text_model.py')
    ledger = (Path(snapshot) / 'core/solver/frame_kernel/src/load_ledger.rs').read_text()
    import re
    bodies = ''.join(model['function_body'](ledger, name) for name in ('add_defect', 'formation_row'))
    origins = set(re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', bodies))
    if origins != RANGE_ORIGINS or any(not x.isascii() for x in origins):
        raise ValueError('B7a scoped TEXT: range origin inventory changed')
    assert max(len(x.encode()) for x in origins) + 16 == RANGE_CONTENT
    proof['scoped_units_sha256'] = UNITS_SHA
    proof['scoped_model_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    return proof


def transform(snapshot, loops, table):
    proof = inspect(snapshot)
    result = deepcopy(table)
    if MARKER in result:
        raise ValueError('B7a scoped TEXT: marker already present')
    for key, expected in AUDIT_PREIMAGES.items():
        if result['id_audit'].get(key) != expected:
            raise ValueError('B7a scoped TEXT: authoritative audit preimage changed: ' + key)
        result['id_audit'][key] = deepcopy(AUDIT_CURRENT[key])
    for key, current in LEX_CURRENT.items():
        if key in result['lex_site_size']:
            raise ValueError('B7a scoped TEXT: lexical preimage already present: ' + key)
        result['lex_site_size'][key] = deepcopy(current)
    # Retain the global four-ref loop. Only this qualified private owner's
    # conversion must traverse its actual five-ref slice.
    expected_loop = {'re': r'^iter:refs\.iter\(\)\.map$', 'bound': '4',
                     'why': 'refs of one diagnostic <= 4 (vec! literals)'}
    hits = [row for row in loops['loops'] if row['re'] == expected_loop['re']]
    if hits != [expected_loop]:
        raise ValueError('B7a scoped TEXT: global ref loop preimage changed')
    for name in ('scoped_messages', 'scoped_refs', 'scoped_extra_requested', 'scoped_owner_loop'):
        if name in result:
            raise ValueError('B7a scoped TEXT: local preimage already present: ' + name)
    result.update(scoped_messages=deepcopy(MESSAGE_CURRENT), scoped_refs=deepcopy(REFS_CURRENT),
                  scoped_extra_requested=deepcopy(EXTRA_CURRENT), scoped_owner_loop=deepcopy(OWNER_LOOP))
    result[MARKER] = 'v1'
    return result, proof


def state(snapshot, table, loops, graph=None, formats=None, lexical=None):
    inspect(snapshot)
    if table.get(MARKER) != 'v1':
        raise ValueError('B7a scoped TEXT: missing/versioned marker')
    expected_loop = {'re': r'^iter:refs\.iter\(\)\.map$', 'bound': '4',
                     'why': 'refs of one diagnostic <= 4 (vec! literals)'}
    if [row for row in loops['loops'] if row['re']==expected_loop['re']] != [expected_loop]:
        raise ValueError('B7a scoped TEXT: consuming global ref loop changed')
    import re
    selected = [row for row in loops['loops'] if re.search(row['re'],REF_LOOP)]
    if selected != [expected_loop]:
        raise ValueError('B7a scoped TEXT: consuming global ref loop priority changed')
    for key, expected in AUDIT_CURRENT.items():
        if table['id_audit'].get(key) != expected:
            raise ValueError('B7a scoped TEXT: authoritative current audit changed: ' + key)
    marked = {key for key, entry in table['lex_site_size'].items() if entry.get('why', '').startswith(WHY)}
    if marked != set(LEX_CURRENT):
        raise ValueError('B7a scoped TEXT: lexical inventory changed')
    for key, expected in LEX_CURRENT.items():
        if table['lex_site_size'].get(key) != expected:
            raise ValueError('B7a scoped TEXT: lexical current rule changed: ' + key)
    for name, expected in [('scoped_messages', MESSAGE_CURRENT), ('scoped_refs', REFS_CURRENT),
                           ('scoped_extra_requested', EXTRA_CURRENT), ('scoped_owner_loop', OWNER_LOOP)]:
        if table.get(name) != expected:
            raise ValueError('B7a scoped TEXT: local current rule changed: ' + name)
    # The ordinary prepared tables have no competing owner cap or zero at
    # these sites. Keep that exact absence, including full/kind aliases.
    targets = [
        (PP + 'pressure_runtime.rs', LN['diag'], 'diag'),
        (PP + 'pressure_runtime.rs', LN['format'], 'format'),
        (PP + 'pressure_runtime.rs', LN['join'], 'join'),
        (PP + 'pressure_runtime.rs', LN['to_string'], 'to_string'),
        (PP + 'lib.rs', LN['unit_message'], 'into_text'),
        (PP + 'lib.rs', LN['suffix'], 'replace'),
        (RE + 'physics_source.rs', LN['require'], 'format'),
        (PP + 'retained_product.rs', 3120, 'to_string'),
        (PP + 'retained_product.rs', 3277, 'to_string'),
        (PP + 'preview_physics.rs', 601, 'to_string'),
        (PP + 'formation_guard.rs', 239, 'format'),
        (PP + 'formation_guard.rs', 322, 'format'),
    ]
    aliases = adjacent('kd5_first_error_model.py')['override_aliases']
    for file, line, kind in targets:
        for family in ('site_zero', 'site_total', 'site_size', 'site_from', 'site_aggregate', 'lex_site_size'):
            for key in aliases({'file': file, 'line': line, 'kind': kind}):
                if family == 'lex_site_size' and key in LEX_CURRENT:
                    continue
                if family == 'site_total' and key in ('formation_guard.rs:239','formation_guard.rs:322'):
                    continue
                if key in table.get(family, {}):
                    raise ValueError('B7a scoped TEXT: competing override: ' + family + ':' + key)
    range_why = 'SQ2 G5: B7a exact formed guard owner: full independent generation; <=cases guards, N DOF rows, q<=12*l aggregate escaped source IDs'
    for key, bound in [('formation_guard.rs:239', '2*cases*N*59'),
                       ('formation_guard.rs:322', '2*cases*(N*(600+2+321)+(2*N+(10*128+4)*(12*l)))')]:
        if table['site_total'].get(key) != {'bytes': bound, 'why': range_why}:
            raise ValueError('B7a scoped TEXT: aggregate range/fired bound changed')
    if graph is not None:
        validate_consumers(graph,formats,lexical)
    return table


def validate_consumers(graph, formats, lexical):
    edges=graph['edges']
    if PROBLEM not in edges:
        raise ValueError('B7a scoped TEXT: missing qualified problem owner')
    parents=[u for u,vs in edges.items() for v in vs if v==PROBLEM]
    expected=PROBLEM_PARENTS+([JOINT_VALUE_PARENT] if JOINT_VALUE_PARENT in edges else [])
    if sorted(parents)!=sorted(expected):
        raise ValueError('B7a scoped TEXT: problem caller inventory changed')
    if edges[PROBLEM] != [PP+'lib.rs:%d:diag' % LN['diag_fn'], PP+'lib.rs:%d:stable_suffix' % LN['suffix_fn']]:
        raise ValueError('B7a scoped TEXT: problem conversion/diag edge changed')
    for line,kind in PROBLEM_SITES:
        rows=[r for r in formats+lexical if (r['file'],r['line'],r['kind'])==(PP+'pressure_runtime.rs',line,kind)]
        if len(rows)!=1 or rows[0].get('fn',PROBLEM)!=PROBLEM:
            raise ValueError('B7a scoped TEXT: missing/duplicate/misowned problem site')


def positive_owner(counts):
    count=counts.get(PROBLEM)
    if not isinstance(count,int) or count<=0:
        raise ValueError('B7a scoped TEXT: missing/zero qualified problem calls')
    return count


def witness(active, file, line, kind, mult, counts):
    if file==PP+'pressure_runtime.rs' and (line,kind) in PROBLEM_SITES:
        count=positive_owner(counts)
        expected=count*5 if line==LN['to_string'] else count
        if mult!=expected:
            raise ValueError('B7a scoped TEXT: private conversion/diag/site witness changed')
    if (file,line,kind)==(PP+'lib.rs',LN['suffix'],'replace'):
        positive_owner(counts)
        if mult<=0:
            raise ValueError('B7a scoped TEXT: original suffix allocation became zero')


def message(active, row):
    if (row['file'], row['line'], row['kind']) != (PP + 'pressure_runtime.rs', LN['diag'], 'diag'):
        return None
    if row.get('message_arg') != 'message':
        raise ValueError('B7a scoped TEXT: private problem message expression changed')
    return active['scoped_messages']['pressure_runtime.rs:%d' % LN['diag']]


def refs(active, file, line):
    if (file, line) == (PP + 'pressure_runtime.rs', LN['diag']):
        return active['scoped_refs']['pressure_runtime.rs:%d' % LN['diag']]
    return None


def owner_total(active, owner, stack, counts):
    if owner == PROBLEM and REF_LOOP in stack:
        if stack != [REF_LOOP]:
            raise ValueError('B7a scoped TEXT: private ref conversion loop nesting changed')
        return positive_owner(counts) * 5
    return None


def extra_requested(active, file, line, kind, counts):
    if (file, line, kind) != (PP + 'lib.rs', LN['suffix'], 'replace'):
        return 0
    # Original 2*128 for every helper call remains. This is only the
    # private problem's additional 644-128 spelling, with its full calls.
    return positive_owner(counts) * 2 * (644 - 128)
