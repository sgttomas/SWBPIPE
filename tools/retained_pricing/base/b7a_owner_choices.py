"""Closed row-source ownership and mutually exclusive metadata base recipes.

All physical sites retain their visits and independent auxiliary allocations.
The recipe match creates at most one base String per validator invocation;
JSON copies, outer qualifications, signs, errors and suffixes are separate.
"""
from copy import deepcopy
from pathlib import Path
import runpy

MARKER = "b7a_owner_choices"
VERSION = "v1"
PP = "core/product_physics/src/"
FK = "core/solver/frame_kernel/src/"
# T4-U6/U8: every snapshot line below moved with unchanged text (structural.rs +1 from one added re-export;
# retained_product.rs and lib.rs by the D-5 hunks before them). No recipe, error, source-copy or helper site changed.
# B7b values: VALIDATOR 2964, BOUNDS 3028..3202 (+89), ERROR_SITES 2544/3005/3227 (+16/+89/+96), structural.rs
# 1123/1124 (+1), lib.rs 13243/13244/13618/13619 (+251/+255), freeze_combination 6253 (+203).
# RF: metering adds 41 lines before these unchanged source-copy sites.
# W1: one retained_api export shifts the structural sites by one; member-only
# capture shifts metadata sites by59 without changing these recipes/owners.
SOURCE_LINE = 1300
SOURCE_SITE = FK + "structural.rs:%d" % SOURCE_LINE
SOURCE_OWNER = FK + 'structural.rs:1299:row_sources'
SOURCE_RULE = "structural.rs:%d" % SOURCE_LINE
# PP (20c6126a16): retained_product.rs moves +3 from line 1995 on (StraightMember::from_frame), +6 after the
# retained member overlay; lib.rs's helpers move +92, and +36 more at the epoch addbdc030e (the realized-arc
# marker's helpers), and +5 more at the final source 588de796a4 (PP4's withhold_certified_summaries). Text unchanged.
# T4-S5 (epoch): every snapshot line moved with unchanged text (structural.rs +118 after ElementAbsorption,
# sparse.rs +40, retained_product.rs +6..+203, lib.rs +314/+356). U4B-W1 adds one mutually exclusive base recipe, the
# NotCovered arm's `result:stress:{suffix}:{loc}:{tail}` (3398): 16 literal bytes + suffix 128 + location <= 9
# ("quarter-1") + tail <= 20 ("transverse-shear-max") = 173 B, below the group's 338 B upper.
# O4 S5 (retained-pricing-rules.md §8.9): every snapshot line moved with unchanged text (structural.rs +3 from the
# retained_api re-exports; retained_product.rs +137 to +411 from the O4 sidecars and ProductCaseView's second lifetime,
# bind_rows_subject's signature line among them). No recipe, error, source-copy or helper site changed.
# Stack-headroom D3 (§8.10): freeze_combination carries +59 through the line map; its text and callers are unchanged.
# W1 coverage repair D3 (§8.11): structural.rs gains six retained_api lines (the CENSUS_PROBE const and the gated
# census_probe re-export) above every snapshot key here, so load_fidelity, audit_load_fidelity, row_sources (and its
# clone site), unaudited_row and audit_load_row carry +6 with unchanged text and callers.
# W1 coverage repair corner 1 D3 (§8.12): six more retained_api lines (LOADED_MAXIMUM_HOOKS and the gated
# loaded_maximum_hooks re-export) carry the same keys a further +6, with unchanged text and callers.
VALIDATOR = PP + 'retained_product.rs:3319:validate_final_metadata'
# W-7 D3 (§8.20): the rows below move +16/+29 with unchanged text (the global end binding precedes them), and
# validate_final_metadata gains one more mutually exclusive base recipe, the global end NotCovered arm's
# `result:element-global-end:{suffix}:{loc}:{}` (3573): 28 literal bytes + suffix 128 + location <= 5 ("end_i") +
# component 2 ("fx") = 163 B, below the group's 338 B upper.
BOUNDS = {3389:46,3412:37,3418:140,3443:143,3470:158,3489:162,
          3523:188,3524:338,3558:169,3573:163,3594:173,3599:322}
ERROR_SITES = {2803:149,3366:1049,3631:1046}
ROWS_ERROR_LINE = 2803          # the one error site owned by bind_rows_subject (inside its row loop)
HELPER_FNS = (PP + "lib.rs:14275:qualified_load_case_result_id", PP + "lib.rs:14696:stable_suffix")
ERROR_OWNER = PP+'retained_product.rs:2553:bind_rows_subject'
ERROR_WHY = "B7a metadata first error: immediate propagation out of whole bind_rows_subject; full owner M/content/capacity and independent errors/detail copies retained"
SOURCE_WHY = ("B7a row-source ownership: audited success or terminal Range fallback "
              "are exclusive per DOF; partitioned force terms are cloned at most q "
              "per full audit; both caller counts/String128/capacity2 remain")
PREIMAGES = {'source_from': {'from': ['structural.rs:1238:audit_load_fidelity',
                          'structural/sparse.rs:1891:audit_sparse_load_fidelity'],
                 'per': '24*l',
                 'why': 'B7a: both audited callers retain their complete2*q source-copy upper before '
                        'row-local dedup; String128/capacity2 remain. Was: G4 (R-1, newly reached): '
                        'row_sources clones one source id per force term of a row; over one audit call, '
                        'every nodal force term sits on one DOF row, and each row calls at most '
                        'unaudited_row and audit_load_row (structural.rs:1037-1180), so <= 2l clones per '
                        'audit call'},
 'group_rows': [{'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3418,
                 'kind': 'format',
                 'template': 'result:disp:{suffix}',
                 'literal_bytes': 12,
                 'placeholders': [{'spec': '', 'arg': 'suffix'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3443,
                 'kind': 'format',
                 'template': 'result:disp:{suffix}:{}',
                 'literal_bytes': 13,
                 'placeholders': [{'spec': '', 'arg': 'suffix'},
                                  {'spec': '', 'arg': '["ux", "uy", "uz", "rx", "ry", "rz"][i]'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3470,
                 'kind': 'format',
                 'template': 'result:{}:{suffix}:{}{}',
                 'literal_bytes': 9,
                 'placeholders': [{'spec': '', 'arg': 'if i < 3 { "force" } else { "moment" }'},
                                  {'spec': '', 'arg': 'suffix'},
                                  {'spec': '',
                                   'arg': '[\n'
                                          '                    "axial",\n'
                                          '                    "shear-y",\n'
                                          '                    "shear-z",\n'
                                          '                    "torsion",\n'
                                          '                    "bending-y",\n'
                                          '                    "bending-z"\n'
                                          '                ][i]'},
                                  {'spec': '', 'arg': 'if end == k::End::J { ":end-j" } else { "" }'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3489,
                 'kind': 'format',
                 'template': 'result:{}:{suffix}:{}:{}',
                 'literal_bytes': 10,
                 'placeholders': [{'spec': '', 'arg': 'if i < 3 { "force" } else { "moment" }'},
                                  {'spec': '', 'arg': 'suffix'},
                                  {'spec': '', 'arg': 'station_id_location(loc)'},
                                  {'spec': '',
                                   'arg': '[\n'
                                          '                    "axial",\n'
                                          '                    "shear-y",\n'
                                          '                    "shear-z",\n'
                                          '                    "torsion",\n'
                                          '                    "bending-y",\n'
                                          '                    "bending-z"\n'
                                          '                ][i]'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3523,
                 'kind': 'format',
                 'template': 'result:support-action:{}:{}:{}',
                 'literal_bytes': 24,
                 'placeholders': [{'spec': '', 'arg': 'row.entity_ref.len()'},
                                  {'spec': '', 'arg': 'row.entity_ref'},
                                  {'spec': '', 'arg': 'component'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3524,
                 'kind': 'format',
                 'template': 'result:support-action:{}:{}:{}:{}:{}',
                 'literal_bytes': 26,
                 'placeholders': [{'spec': '', 'arg': 'case.len()'},
                                  {'spec': '', 'arg': 'case'},
                                  {'spec': '', 'arg': 'row.entity_ref.len()'},
                                  {'spec': '', 'arg': 'row.entity_ref'},
                                  {'spec': '', 'arg': 'component'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3558,
                 'kind': 'format',
                 'template': 'result:stress:{suffix}:{idloc}:{tail}',
                 'literal_bytes': 16,
                 'placeholders': [{'spec': '', 'arg': 'suffix'},
                                  {'spec': '', 'arg': 'idloc'},
                                  {'spec': '', 'arg': 'tail'}]},
                # W-7 D3 (§8.20): the global end NotCovered arm's recipe.
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3573,
                 'kind': 'format',
                 'template': 'result:element-global-end:{suffix}:{loc}:{}',
                 'literal_bytes': 28,
                 'placeholders': [{'spec': '', 'arg': 'suffix'},
                                  {'spec': '', 'arg': 'loc'},
                                  {'spec': '', 'arg': 'component.to_ascii_lowercase()'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3594,
                 'kind': 'format',
                 'template': 'result:stress:{suffix}:{}:{tail}',
                 'literal_bytes': 16,
                 'placeholders': [{'spec': '', 'arg': 'suffix'},
                                  {'spec': '', 'arg': 'u4b_rows::id_location(loc)'},
                                  {'spec': '', 'arg': 'tail'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3599,
                 'kind': 'format',
                 'template': 'result:elastic-maximum:{}:{}:{}:{}',
                 'literal_bytes': 26,
                 'placeholders': [{'spec': '', 'arg': 'case.len()'},
                                  {'spec': '', 'arg': 'case'},
                                  {'spec': '', 'arg': 'row.entity_ref.len()'},
                                  {'spec': '', 'arg': 'row.entity_ref'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3389,
                 'kind': 'to_string',
                 'fn': 'core/product_physics/src/retained_product.rs:3319:validate_final_metadata',
                 'receiver': 'REQUESTED_MODE_ID',
                 'literal_bytes': None,
                 'arg': ''},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3412,
                 'kind': 'to_string',
                 'fn': 'core/product_physics/src/retained_product.rs:3319:validate_final_metadata',
                 'receiver': '" "',
                 'literal_bytes': 37,
                 'arg': ''}],
 'aux_rows': [{'file': 'core/product_physics/src/retained_product.rs',
               'line': 3432,
               'kind': 'format',
               'template': 'positive value follows the global cartesian {axis} axis displacement of the node',
               'literal_bytes': 74,
               'placeholders': [{'spec': '', 'arg': 'axis'}]},
              {'file': 'core/product_physics/src/retained_product.rs',
               'line': 3434,
               'kind': 'format',
               'template': 'positive value follows the right-hand-rule rotation about the global cartesian '
                           '{axis} axis',
               'literal_bytes': 84,
               'placeholders': [{'spec': '', 'arg': 'axis'}]},
              {'file': 'core/product_physics/src/retained_product.rs',
               'line': 3555,
               'kind': 'format',
               'template': '{prefix}; j-side section action in the element-local frame (x toward end j); '
                           'section equilibrium from stiffness-recovered end actions with consistent '
                           'distributed-load fixed-end correction',
               'literal_bytes': 179,
               'placeholders': [{'spec': '', 'arg': 'prefix'}]},
              {'file': 'core/product_physics/src/retained_product.rs',
               'line': 3621,
               'kind': 'format',
               'template': 'result:combination:{}:{}',
               'literal_bytes': 20,
               'placeholders': [{'spec': '', 'arg': 'stable_suffix(id)'},
                                {'spec': '', 'arg': 'super::result_tail(&expected)'}]},
              {'file': 'core/product_physics/src/retained_product.rs',
               'line': 3546,
               'kind': 'to_string',
               'fn': 'core/product_physics/src/retained_product.rs:3319:validate_final_metadata',
               'receiver': 'STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION',
               'literal_bytes': None,
               'arg': ''}],
 'audit_preimages': {'core/product_physics/src/retained_product.rs:3389': None,
                     'core/product_physics/src/retained_product.rs:3412': None,
                     'core/product_physics/src/retained_product.rs:3418': {'suffix': {'bytes': 128,
                                                                                      'source': 'IN128'}},
                     'core/product_physics/src/retained_product.rs:3443': {'suffix': {'bytes': 128,
                                                                                      'source': 'IN128'}},
                     'core/product_physics/src/retained_product.rs:3470': {'["axial","shear-y","shear-z","torsion","bending-y","bending-z"][i]': {'bytes': 128,
                                                                                                                                                  'source': 'STATIC'},
                                                                           'suffix': {'bytes': 128,
                                                                                      'source': 'IN128'}},
                     'core/product_physics/src/retained_product.rs:3489': {'["axial","shear-y","shear-z","torsion","bending-y","bending-z"][i]': {'bytes': 128,
                                                                                                                                                  'source': 'STATIC'},
                                                                           'station_id_location(loc)': {'bytes': 600,
                                                                                                        'source': 'COMP'},
                                                                           'suffix': {'bytes': 128,
                                                                                      'source': 'IN128'}},
                     'core/product_physics/src/retained_product.rs:3523': {'row.entity_ref': {'bytes': 128,
                                                                                              'source': 'IN128'},
                                                                           'component': {'bytes': 128,
                                                                                         'source': 'STATIC'},
                                                                           'row.entity_ref.len()': {'bytes': 20,
                                                                                                    'source': 'NUM'}},
                     'core/product_physics/src/retained_product.rs:3524': {'case': {'bytes': 2173,
                                                                                    'source': 'IN128'},
                                                                           'component': {'bytes': 128,
                                                                                         'source': 'STATIC'},
                                                                           'row.entity_ref': {'bytes': 128,
                                                                                              'source': 'IN128'},
                                                                           'row.entity_ref.len()': {'bytes': 20,
                                                                                                    'source': 'NUM'}},
                     'core/product_physics/src/retained_product.rs:3558': {'idloc': {'bytes': 128,
                                                                                     'source': 'STATIC'},
                                                                           'suffix': {'bytes': 128,
                                                                                      'source': 'IN128'}},
                     'core/product_physics/src/retained_product.rs:3599': {'case': {'bytes': 2173,
                                                                                    'source': 'IN128'},
                                                                           'row.entity_ref': {'bytes': 128,
                                                                                              'source': 'IN128'},
                                                                           'row.entity_ref.len()': {'bytes': 20,
                                                                                                    'source': 'NUM'}}},
 'lex_preimages': {'retained_product.rs:3389': {'class': 46,
                                                'why': 'B7a scoped TEXT: exact current producer and admitted '
                                                       'source/type closure; requested mode literal'}},
 'parents': {'core/solver/frame_kernel/src/structural.rs:1299:row_sources': ['core/solver/frame_kernel/src/structural.rs:1307:unaudited_row',
                                                                             'core/solver/frame_kernel/src/structural.rs:1336:audit_load_row'],
             'core/solver/frame_kernel/src/structural.rs:1307:unaudited_row': ['core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity',
                                                                               'core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity'],
             'core/solver/frame_kernel/src/structural.rs:1336:audit_load_row': ['core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity',
                                                                                'core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity'],
             'core/product_physics/src/retained_product.rs:3319:validate_final_metadata': ['core/product_physics/src/retained_product.rs:2553:bind_rows_subject'],
             'core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity': ['core/solver/frame_kernel/src/structural.rs:560:load_fidelity'],
             'core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity': ['core/solver/frame_kernel/src/structural/sparse.rs:1218:load_fidelity'],
             'core/product_physics/src/retained_product.rs:2553:bind_rows_subject': ['core/product_physics/src/retained_product.rs:2537:bind_combination_rows_scoped',
                                                                                     'core/product_physics/src/retained_product.rs:2546:bind_rows_view',
                                                                                     'core/product_physics/src/retained_product.rs:7392:freeze_combination']},
 'site_stacks': [['core/product_physics/src/retained_product.rs:2553:bind_rows_subject',
                  'core/product_physics/src/retained_product.rs:3319:validate_final_metadata',
                  [['for observed in view.rows()']]],
                 ['core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity',
                  'core/solver/frame_kernel/src/structural.rs:1336:audit_load_row',
                  [['for i in 0..n']]],
                 ['core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity',
                  'core/solver/frame_kernel/src/structural.rs:1307:unaudited_row',
                  [['for i in 0..n']]],
                 ['core/solver/frame_kernel/src/structural.rs:1307:unaudited_row',
                  'core/solver/frame_kernel/src/structural.rs:1299:row_sources',
                  [[]]],
                 ['core/solver/frame_kernel/src/structural.rs:1336:audit_load_row',
                  'core/solver/frame_kernel/src/structural.rs:1299:row_sources',
                  [[]]],
                 ['core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity',
                  'core/solver/frame_kernel/src/structural.rs:1336:audit_load_row',
                  [['for i in 0..n']]],
                 ['core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity',
                  'core/solver/frame_kernel/src/structural.rs:1307:unaudited_row',
                  [['for i in 0..n']]],
                 ['core/solver/frame_kernel/src/structural.rs:560:load_fidelity',
                  'core/solver/frame_kernel/src/structural.rs:1238:audit_load_fidelity',
                  [[]]],
                 ['core/solver/frame_kernel/src/structural/sparse.rs:1218:load_fidelity',
                  'core/solver/frame_kernel/src/structural/sparse.rs:1891:audit_sparse_load_fidelity',
                  [[]]],
                 ['core/product_physics/src/retained_product.rs:2537:bind_combination_rows_scoped',
                  'core/product_physics/src/retained_product.rs:2553:bind_rows_subject',
                  [[]]],
                 ['core/product_physics/src/retained_product.rs:2546:bind_rows_view',
                  'core/product_physics/src/retained_product.rs:2553:bind_rows_subject',
                  [[]]],
                 ['core/product_physics/src/retained_product.rs:7392:freeze_combination',
                  'core/product_physics/src/retained_product.rs:2553:bind_rows_subject',
                  [[], []]]],
 'source_loop': {'re': '^iter:terms\\.iter\\(\\)\\.map$',
                 'bound': '12*l',
                 'why': 'B7a q<=12*l positive force-term upper; input-count owners retain l. Was: G4 (R-1, '
                        'newly reached): structural.rs:1098 row_sources: the force terms of one row <= l'},
 'error_rows': [{'file': 'core/product_physics/src/retained_product.rs',
                 'line': 2803,
                 'kind': 'format',
                 'template': 'uncovered actual row {}',
                 'literal_bytes': 21,
                 'placeholders': [{'spec': '', 'arg': 'r.kind'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3366,
                 'kind': 'format',
                 'template': 'final metadata mismatch: {}',
                 'literal_bytes': 25,
                 'placeholders': [{'spec': '', 'arg': 'row.id'}]},
                {'file': 'core/product_physics/src/retained_product.rs',
                 'line': 3631,
                 'kind': 'format',
                 'template': 'final id association: {}',
                 'literal_bytes': 22,
                 'placeholders': [{'spec': '', 'arg': 'row.id'}]}],
 # Pre-0.4 failure unification D3 (§8.14): lib.rs 13943/13944/14364/14365 -> 14046/14047/14467/14468, unchanged text.
# W-8 topology warnings D3 (§8.16): -> 14048/14049/14469/14470 (`mod topology;` and its call), unchanged text.
# Silent-zero guard D3 (§8.15): -> 14049/14050/14470/14471 (one load-state call), unchanged text.
# Model document 0.7.0 D3 (§8.17): -> 14056/14057/14477/14478 (`model_settings`), unchanged text.
# W-7 D3 (§8.20): -> 14275/14276/14696/14697 (append_element_global_end_results precedes them), and the
# retained_product.rs rows and validate_final_metadata +16 to +85 (the global end binding and completion), unchanged text.
 # B7b snapshot lib.rs lines (B7a: 13239/13238 and 13614/13613; unchanged text).
 'helper_rows': [{'file': 'core/product_physics/src/lib.rs',
                  'line': 14276,
                  'kind': 'format',
                  'template': 'result:loadcase:{}:{}',
                  'literal_bytes': 17,
                  'placeholders': [{'spec': '', 'arg': 'stable_suffix(load_case_id)'},
                                   {'spec': '', 'arg': 'result_tail(base_id)'}],
                  'fn': 'core/product_physics/src/lib.rs:14275:qualified_load_case_result_id'},
                 {'file': 'core/product_physics/src/lib.rs',
                  'line': 14697,
                  'kind': 'replace',
                  'fn': 'core/product_physics/src/lib.rs:14696:stable_suffix',
                  'receiver': 'id',
                  'literal_bytes': None,
                  'arg': '\' \', " "'}],
 'aux_audits': {'core/product_physics/src/retained_product.rs:3432': None,
                'core/product_physics/src/retained_product.rs:3434': None,
                'core/product_physics/src/retained_product.rs:3555': None,
                'core/product_physics/src/retained_product.rs:3621': {'stable_suffix(id)': {'bytes': 128,
                                                                                            'source': 'IN128'},
                                                                      'super::result_tail(&expected)': {'bytes': 1024,
                                                                                                        'source': 'RES'}},
                'core/product_physics/src/retained_product.rs:3546': None,
                'core/product_physics/src/lib.rs:14276': {'result_tail(base_id)': {'bytes': 600,
                                                                                   'source': 'COMP'},
                                                          'stable_suffix(load_case_id)': {'bytes': 128,
                                                                                          'source': 'IN128'}},
                'core/product_physics/src/lib.rs:14697': {'id': {'bytes': 128, 'source': 'IN128'}},
                'core/product_physics/src/retained_product.rs:2803': {'r.kind': {'bytes': 128,
                                                                                 'source': 'STATIC'}},
                'core/product_physics/src/retained_product.rs:3366': {'row.id': {'bytes': 1024,
                                                                                 'source': 'RES'}},
                'core/product_physics/src/retained_product.rs:3631': {'row.id': {'bytes': 1024,
                                                                                 'source': 'RES'}}},
 'aux_overrides': {'site_from': {},
                   'site_zero': {},
                   'site_total': {},
                   'site_aggregate': {},
                   'site_size': {},
                   'lex_site_size': {'retained_product.rs:3546': {'class': 167,
                                                                  'why': 'B7a scoped TEXT: exact current '
                                                                         'producer and admitted source/type '
                                                                         'closure; straight endpoint sign '
                                                                         'literal'}}}}


def adjacent(name):
    return runpy.run_path(str(Path(__file__).with_name(name)))


def inspect(snapshot):
    # Full structural/sparse/ledger/product/lib and all admission/custody/helper
    # sources are bound by this existing transitive source guard. A new fallible
    # operation after sources, retry, partition, recipe or width change refuses.
    return adjacent("b7a_phase_model.py")["inspect"](snapshot)


def source_current():
    return dict(deepcopy(PREIMAGES["source_from"]), per="12*l", why=SOURCE_WHY)


def transform(snapshot, table):
    proof = inspect(snapshot)
    result = deepcopy(table)
    if MARKER in result:
        raise ValueError("B7a owner choices: duplicate marker")
    if result.get("site_from", {}).get(SOURCE_RULE) != PREIMAGES["source_from"]:
        raise ValueError("B7a owner choices: source-copy preimage changed")
    result["site_from"][SOURCE_RULE] = source_current()
    for line in ERROR_SITES:
        key="retained_product.rs:"+str(line)
        if key in result.get("site_from",{}):
            raise ValueError("B7a owner choices: metadata error preimage changed")
        result["site_from"][key] = {"from":[ERROR_OWNER.split("/src/")[-1]],"per":"1","why":ERROR_WHY}
    result[MARKER] = {"version":VERSION, "base_contents":{str(k):v for k,v in BOUNDS.items()}}
    return result, proof


def inventory_row(row):
    keys = ("file","line","kind","template","literal_bytes","placeholders") if row["kind"] == "format" else (
        "file","line","kind","fn","receiver","literal_bytes","arg")
    return {key:row[key] for key in keys}


def graph_inventory(graph):
    edges = graph["edges"]
    for target, parents in PREIMAGES["parents"].items():
        found = [u for u, children in edges.items() for v in children if v == target]
        if target not in edges or sorted(found) != sorted(parents):
            raise ValueError("B7a owner choices: qualified caller inventory changed")
        found_stacks = [row for row in graph["site_loops"] if row[1] == target]
        wanted = [row for row in PREIMAGES["site_stacks"] if row[1] == target]
        if sorted(found_stacks, key=repr) != sorted(wanted, key=repr):
            raise ValueError("B7a owner choices: complete target-stack inventory changed")


def state(snapshot, table, loops, graph, formats, lexical, extra_zero="", fn_zero=""):
    inspect(snapshot)
    if table.get(MARKER) != {"version":VERSION,"base_contents":{str(k):v for k,v in BOUNDS.items()}}:
        raise ValueError("B7a owner choices: exact marker/recipe contents changed")
    if table.get("site_from", {}).get(SOURCE_RULE) != source_current():
        raise ValueError("B7a owner choices: effective source-copy rule changed")
    if table.get("id_audit", {}).get(SOURCE_SITE) != {"t.source":{"bytes":128,"source":"IN128"}}:
        raise ValueError("B7a owner choices: source-copy width/capacity premise changed")
    graph_inventory(graph)
    import re
    selected = [r for r in loops["loops"] if re.search(r["re"], "iter:terms.iter().map")]
    if selected != [PREIMAGES["source_loop"]]:
        raise ValueError("B7a owner choices: source loop priority changed")
    protected = {SOURCE_OWNER, VALIDATOR, *HELPER_FNS, ERROR_OWNER} | {FK+name for name in PREIMAGES["source_from"]["from"]}
    for family in ("fn_cap","fn_zero"):
        if any(row["fn"] in protected or row["fn"] in {key.split("/src/")[-1] for key in protected}
               for row in loops.get(family, [])):
            raise ValueError("B7a owner choices: protected function cap/zero changed")
    aliases = adjacent("kd5_first_error_model.py")["override_aliases"]
    targets = [dict(file=FK+"structural.rs",line=SOURCE_LINE,kind="clone_text")]
    for row in PREIMAGES["error_rows"]:
        targets.append(row)
        matches = [r for r in formats if (r["file"],r["line"],r["kind"]) == (row["file"],row["line"],row["kind"])]
        key="retained_product.rs:"+str(row["line"])
        if len(matches)!=1 or inventory_row(matches[0])!=row:
            raise ValueError("B7a owner choices: metadata error inventory changed")
        if table["site_from"].get(key)!={"from":[ERROR_OWNER.split("/src/")[-1]],"per":"1","why":ERROR_WHY}:
            raise ValueError("B7a owner choices: effective metadata error rule changed")
    marked={key for key,entry in table.get("site_from",{}).items() if entry.get("why","")==ERROR_WHY}
    if marked!={"retained_product.rs:"+str(line) for line in ERROR_SITES}:
        raise ValueError("B7a owner choices: metadata error marked inventory changed")
    for row in PREIMAGES["group_rows"]:
        targets.append(row)
        matches = [r for r in formats+lexical if (r["file"],r["line"],r["kind"]) ==
                   (row["file"],row["line"],row["kind"])]
        if len(matches) != 1 or inventory_row(matches[0]) != row or matches[0].get("fn",VALIDATOR) != VALIDATOR:
            raise ValueError("B7a owner choices: base allocation inventory changed")
    # Preserve each auxiliary generation: exact constants/templates and their
    # allocation presence remain even though only base recipes are grouped.
    for row in PREIMAGES["aux_rows"]:
        targets.append(row)
        matches = [r for r in formats+lexical if (r["file"],r["line"],r["kind"]) ==
                   (row["file"],row["line"],row["kind"])]
        if len(matches) != 1 or inventory_row(matches[0]) != row:
            raise ValueError("B7a owner choices: auxiliary allocation inventory changed")
    for row in PREIMAGES["helper_rows"]:
        targets.append(row)
        matches = [r for r in formats+lexical if (r["file"],r["line"],r["kind"]) ==
                   (row["file"],row["line"],row["kind"])]
        if len(matches) != 1 or inventory_row(matches[0]) != inventory_row(row):
            raise ValueError("B7a owner choices: independent helper allocation changed")
    source_rows = [r for r in lexical if (r["file"],r["line"],r["kind"]) == (FK+"structural.rs",SOURCE_LINE,"clone_text")]
    if len(source_rows) != 1 or source_rows[0]["fn"] != SOURCE_OWNER or source_rows[0]["receiver"] != "t.source":
        raise ValueError("B7a owner choices: source-copy allocation changed")
    for row in targets:
        for family in ("site_zero","site_from","site_total","site_aggregate","site_size","lex_site_size"):
            for alias in aliases(row):
                if family == "site_from" and alias == SOURCE_RULE:
                    continue
                if family == "site_from" and alias in {"retained_product.rs:"+str(line) for line in ERROR_SITES}:
                    continue
                if alias in PREIMAGES["aux_overrides"].get(family,{}):
                    if table.get(family,{}).get(alias) != PREIMAGES["aux_overrides"][family][alias]:
                        raise ValueError("B7a owner choices: independent helper rule changed")
                    continue
                if family == "lex_site_size" and alias in PREIMAGES["lex_preimages"]:
                    if table.get(family,{}).get(alias) != PREIMAGES["lex_preimages"][alias]:
                        raise ValueError("B7a owner choices: inherited lexical rule changed")
                    continue
                if alias in table.get(family,{}):
                    raise ValueError("B7a owner choices: competing override " + family + ":" + alias)
    for key, expected in PREIMAGES["audit_preimages"].items():
        if table.get("id_audit",{}).get(key) != expected:
            raise ValueError("B7a owner choices: inherited authoritative audit changed")
    for key, expected in PREIMAGES["aux_audits"].items():
        if table.get("id_audit",{}).get(key) != expected:
            raise ValueError("B7a owner choices: independent helper audit changed")
    if fn_zero:
        # Canonical W/X exclusions do not omit these owners. Reject a new
        # function waiver rather than accepting a missing multiplicity.
        names = fn_zero.split(",")
        if any(name in protected or name in {key.split("/src/")[-1] for key in protected}
               or name.rsplit(":",1)[-1] in {key.rsplit(":",1)[-1] for key in protected} for name in names):
            raise ValueError("B7a owner choices: protected function zero")
    projected = adjacent("b7a_returning_errors.py")["envelope_projection"](extra_zero)
    return {"envelope_only":projected}


def positive(counts, owner):
    value = counts.get(owner)
    if type(value) is not int or value <= 0:
        raise ValueError("B7a owner choices: missing/zero qualified owner " + owner)
    return value


def source_witness(active, file, line, kind, mult, counts, loads, rebound=False, owner=None, stack=None):
    if (file,line,kind) != (FK+"structural.rs",SOURCE_LINE,"clone_text"):
        return
    if type(loads) is not int or loads <= 0:
        raise ValueError("B7a owner choices: invalid source input count")
    if owner != SOURCE_OWNER or stack != ["iter:terms.iter().map"]:
        raise ValueError("B7a owner choices: source-copy owner/loop changed")
    parents = PREIMAGES["source_from"]["from"]
    calls = sum(positive(counts,FK+owner) for owner in parents)
    if type(mult) is not int or mult <= 0:
        raise ValueError("B7a owner choices: missing source-copy occurrence")
    if rebound and mult != calls * 12 * loads:
        raise ValueError("B7a owner choices: effective source-copy count changed")


def recipe_witness(active, row, counts):
    if row["file"] != PP+"retained_product.rs" or row["line"] not in BOUNDS:
        return False
    wanted = next(r for r in PREIMAGES["group_rows"] if r["line"] == row["line"])
    if row["kind"] != wanted["kind"] or row["fn"] != VALIDATOR or row["loops"]:
        raise ValueError("B7a owner choices: base site owner/branch changed")
    if active["envelope_only"]:
        if row["mult"] != 0 or counts.get(VALIDATOR,0) != 0:
            raise ValueError("B7a owner choices: excluded recipe owner became positive")
    elif row["mult"] != positive(counts,VALIDATOR):
        raise ValueError("B7a owner choices: effective base occurrence changed")
    return True


def error_witness(active, file, line, kind, mult, counts, owner, stack, rebound=False):
    if (file,line,kind) not in {(PP+"retained_product.rs",line,"format") for line in ERROR_SITES}:
        return
    expected_owner=ERROR_OWNER if line==ROWS_ERROR_LINE else VALIDATOR
    expected_stack=["for observed in view.rows()"] if line==ROWS_ERROR_LINE else []
    if owner != expected_owner or stack!=expected_stack:
        raise ValueError("B7a owner choices: metadata error owner/loop changed")
    if active["envelope_only"]:
        if mult!=0 or counts.get(VALIDATOR,0)!=0 or counts.get(ERROR_OWNER,0)!=0:
            raise ValueError("B7a owner choices: excluded error owner became positive")
        return
    expected=positive(counts,ERROR_OWNER) if rebound else positive(counts,VALIDATOR)
    if mult!=expected:
        raise ValueError("B7a owner choices: effective whole-return error count changed")


def recipe_group(active, rows, counts):
    group = [row for row in rows if recipe_witness(active,row,counts)]
    if sorted(row["line"] for row in group) != sorted(BOUNDS):
        raise ValueError("B7a owner choices: missing/duplicate complete base recipe group")
    calls = 0 if active["envelope_only"] else positive(counts,VALIDATOR)
    for spec in PREIMAGES["aux_rows"]+PREIMAGES["helper_rows"]:
        found = [row for row in rows if (row["file"],row["line"],row["kind"]) ==
                 (spec["file"],spec["line"],spec["kind"])]
        if len(found) != 1:
            raise ValueError("B7a owner choices: missing/duplicate auxiliary generation")
        row=found[0]
        expected_owner=spec.get("fn",VALIDATOR)
        projected=active["envelope_only"] and expected_owner==VALIDATOR
        expected=0 if projected else positive(counts,expected_owner)
        if row["fn"] != expected_owner or row["loops"] or row["mult"] != expected:
            raise ValueError("B7a owner choices: effective auxiliary occurrence changed")
        if not projected and row["req"] <= 0:
            raise ValueError("B7a owner choices: erased independent auxiliary allocation")
    # Every invocation selects at most ONE complete base recipe. All current
    # stand-alone rows stay visible, with their physical M and prospective
    # cost; the nonnegative group replaces their sum in allocation volume.
    # Independent signs/suffixes/errors/outer strings are never in this group.
    upper = max(BOUNDS.values())
    return {"name":"metadata expected base recipe", "owner":VALIDATOR,
            "calls":calls,"members":[row["line"] for row in group],
            "contents":deepcopy(BOUNDS),"bytes":upper,"capacity_factor":2,
            "req":calls*max(8,2*upper),
            "standalone_requested_bytes":sum(row["req"] for row in group)}
