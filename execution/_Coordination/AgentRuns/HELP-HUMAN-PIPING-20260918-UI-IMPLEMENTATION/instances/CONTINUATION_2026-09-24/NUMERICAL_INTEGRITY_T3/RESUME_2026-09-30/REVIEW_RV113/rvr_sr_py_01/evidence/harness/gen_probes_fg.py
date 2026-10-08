"""RV113 (RV-R): probes for the three-reader alignment items (f) and (g), and their neighbours.

(f) self-position and receipt-internal binding of `material_bases` and `sources` (and the case lists
    and source owners), each read bound and unbound;
(g) the invocation model's scope members (`reference_configurations`, `pressure_contract`, `combinations`,
    `components`) with present-but-empty or non-canonical values, through `invocation_edits` (06C's
    `format_change` rebinds the receipt's invocation digest).
Every probe's `want` is {"observe": true}: the three readers are compared with each other.
Usage: python gen_probes_fg.py <out.json>
"""
import json
import sys

B = ["retained_precision", "body"]
M = ["request", "model"]


def probe(pid, item, base, edits=(), invocation_edits=(), note=""):
    return {"id": pid, "item": item, "base": base, "edits": list(edits), "invocation_edits": list(invocation_edits),
            "rehash": "all", "want": {"observe": True}, "want_i1": {"observe": True}, "note": note}


def s(path, value):
    return {"path": path, "op": "set", "value": value}


O, TG, TC = "ordinary_prepared_synthetic", "two_case_two_groups_synthetic", "two_case_synthetic"
P = [
    probe("f_mb_index_1", "(f)", O, [s(B + ["material_bases", 0, "index"], 1)], note="the one basis labelled 1"),
    probe("f_mb_index_swapped", "(f)", TG, [s(B + ["material_bases", 0, "index"], 1), s(B + ["material_bases", 1, "index"], 0)], note="two bases, labels swapped"),
    probe("f_src_index_1", "(f) neighbour", O, [s(B + ["sources", 0, "index"], 1)], note="the one CaseSource labelled 1"),
    probe("f_src_index_swapped", "(f) neighbour", TC, [s(B + ["sources", 0, "index"], 1), s(B + ["sources", 1, "index"], 0)], note="two CaseSources, labels swapped"),
    probe("f_mb_case_indices_duplicate", "(f) neighbour", TC, [s(B + ["material_bases", 0, "case_indices"], [0, 1, 1])]),
    probe("f_mb_case_indices_out_of_range", "(f) neighbour", TC, [s(B + ["material_bases", 0, "case_indices"], [0, 1, 2])]),
    probe("f_src_owner_case_id_other", "(f) neighbour", TC, [s(B + ["sources", 1, "owner", "case_id"], "case:six-component-load")],
          note="source 1's owner names case 0's id at case_index 1"),
    probe("g_reference_configurations_null", "(g)", O, invocation_edits=[s(M + ["reference_configurations"], None)]),
    probe("g_reference_configurations_empty", "(g)", O, invocation_edits=[s(M + ["reference_configurations"], [])]),
    probe("g_pressure_contract_null", "(g) neighbour", O, invocation_edits=[s(M + ["pressure_contract"], None)], note="control: null"),
    probe("g_pressure_contract_empty_object", "(g) neighbour", O, invocation_edits=[s(M + ["pressure_contract"], {})]),
    probe("g_pressure_contract_false", "(g) neighbour", O, invocation_edits=[s(M + ["pressure_contract"], False)]),
    probe("g_combinations_null", "(g) neighbour", O, invocation_edits=[s(M + ["combinations"], None)], note="control: null"),
    probe("g_combinations_object", "(g) neighbour", O, invocation_edits=[s(M + ["combinations"], {"x": 1})]),
    probe("g_components_string", "(g) neighbour", O, invocation_edits=[s(M + ["components"], "x")]),
]
json.dump(P, open(sys.argv[1], "w"), indent=1)
print(len(P), "probes ->", sys.argv[1])
