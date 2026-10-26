"""Five owned error allocations returning immediately from their actual owner.

This table changes allocations per owner call only. It preserves the owner
multiplicity, content/capacity, scans, numeric work and downstream copies.
"""
import ast
import hashlib
import os
from pathlib import Path
import runpy

MARKER = "b7a_owned_first_errors"
VERSION = "v1"
WHY = "B7a owned first error: immediate outer Result return; per1 allocation with full owner calls/content/capacity and all work/copies preserved"
SOURCES = {
    "core/solver/frame_kernel/src/exact_sum.rs": "3a847f8dbaa74768603c4d9693b538f1f63c2d172026ed7ab400aea3b4800f67",
    "core/solver/curved_bend/src/lib.rs": "6166e6c27247955ccabfbb8a51bbd5ebe6589bbc49b5a62b096ef0c0b8798a1c",
}
# T4-U6/U8 (PP head 20c6126a16): the owners, sites and callers below moved with unchanged text (B7b lines: lib.rs
# 10446/10487/10493, 11127/11146, 4427; membrane_publication_range.rs 159/233; retained_product.rs 5453 and
# 2811/2831/2850/6369).
# The caller inventories are unchanged; D-5 calls exact_straight_summary_extrema once per basis from the case solve,
# which the chain's own multiplicity counts. Epoch addbdc030e: the lib.rs owners and sites after
# `straight_section_resultants` move +36 (the realized-arc marker's helpers), text unchanged; at the final source
# 588de796a4 every lib.rs key after withhold_certified_summaries moves +5 more (PP4), text unchanged.
# W1-L5: unchanged error owners/sites/callers after aggregation shift +2.
# T4-S5 (epoch): every owner, site and caller moved with unchanged text (lib.rs +177/+314; retained_product.rs
# +5/+27/+95/+203); no owner gains a caller.
# O4 S5 (retained-pricing-rules.md §8.9): every owner, site and caller moved with unchanged text (lib.rs +3; retained_product.rs
# +61/+167/+303/+412). prepared_maxima calls exact_straight_summary_extrema once more for a distinct-basis member's
# reduced maximum (the same caller node; the chain counts its multiplicity); no owner gains a caller.
# Stack-headroom D3 (§8.10): combination_observables carries +59 through the line map; its text and callers are unchanged.
# Pre-0.4 failure unification D3 (§8.14): the lib.rs owners, sites and callers moved +85/+103 with unchanged text; no
# owner gains a caller. W-8 topology warnings D3 (§8.16): +2 more (`mod topology;` and its call), unchanged text.
# Silent-zero guard D3 (§8.15): +1 more (one load-state call), unchanged text.
# Model document 0.7.0 D3 (§8.17): +7 more (`model_settings`), unchanged text.
# W-7 D3 (§8.20): every owner, site and caller moved with unchanged text (lib.rs +5/+37; retained_product.rs
# +16/+36/+85); no owner gains a caller.
# W-15 D3 (§8.21): every lib.rs owner, site and caller moved with unchanged text (+60 to the case solve, +103/+111
# after it); retained_product.rs's are in place (its rows only name `component_body: None`); no owner gains a caller.
# W-16 slice a D3 (§8.23): every lib.rs owner, site and caller moved with unchanged text (+7 to the case solve, +39 after
# it), and retained_product.rs's after the captured-target match +2; no owner gains a caller.
SITES = [
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 11522,
    "kind": "format",
    "owner": "core/product_physics/src/lib.rs:11480:exact_straight_summary_extrema",
    "short": "lib.rs:11522",
    "from": "lib.rs:11480:exact_straight_summary_extrema",
    "parents": [
      "core/product_physics/src/lib.rs:4917:solve_prepared_load_case",
      "core/product_physics/src/membrane_publication_range.rs:168:original_subnormal_wall_force_direct_extrema_preserves_membrane",
      "core/product_physics/src/membrane_publication_range.rs:243:unclamped_subnormal_wall_force_extrema_midpoint_is_the_membrane",
      'core/product_physics/src/retained_product.rs:6191:prepared_maxima'
    ]
  },
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 11528,
    "kind": "format",
    "owner": "core/product_physics/src/lib.rs:11480:exact_straight_summary_extrema",
    "short": "lib.rs:11528",
    "from": "lib.rs:11480:exact_straight_summary_extrema",
    "parents": [
      "core/product_physics/src/lib.rs:4917:solve_prepared_load_case",
      "core/product_physics/src/membrane_publication_range.rs:168:original_subnormal_wall_force_direct_extrema_preserves_membrane",
      "core/product_physics/src/membrane_publication_range.rs:243:unclamped_subnormal_wall_force_extrema_midpoint_is_the_membrane",
      'core/product_physics/src/retained_product.rs:6191:prepared_maxima'
    ]
  },
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 12191,
    "kind": "to_string",
    "owner": "core/product_physics/src/lib.rs:12172:recover_curved_bend_local_forces",
    "short": "lib.rs:12191",
    "from": "lib.rs:12172:recover_curved_bend_local_forces",
    "parents": [
      "core/product_physics/src/lib.rs:4917:solve_prepared_load_case"
    ]
  },
  {
    "file": "core/product_physics/src/retained_product.rs",
    "line": 1877,
    "kind": "into_text",
    "owner": 'core/product_physics/src/retained_product.rs:1837:check_support_maps',
    "short": 'retained_product.rs:1877',
    "from": 'retained_product.rs:1837:check_support_maps',
    "parents": [
      'core/product_physics/src/retained_product.rs:1834:check_support_source',
      'core/product_physics/src/retained_product.rs:1920:capture_case_structure'
    ]
  },
  {
    "file": "core/product_physics/src/retained_product.rs",
    "line": 3162,
    "kind": "into_text",
    "owner": 'core/product_physics/src/retained_product.rs:3142:support_observables',
    "short": 'retained_product.rs:3162',
    "from": 'retained_product.rs:3142:support_observables',
    "parents": [
      'core/product_physics/src/retained_product.rs:3181:case_observables',
      'core/product_physics/src/retained_product.rs:7516:combination_observables'
    ]
  }
]


ENVELOPE_ZERO = ("core/product_physics/src/source_receipt,core/product_physics/src/retained_wire.rs,"
                 "core/product_physics/src/retained_product.rs,core/product_physics/src/retained_receipt.rs,"
                 "core/reporting/result_export")
MODE_SOURCE = {
    "ENVZERO": {"c0916b98869910d07737f0917f43c3629bb7185c05fbfbb4ddc1ef953aab0df8"},
    "env": {"504db9604b77de9dd1ec933852b3e9820d6f840611135d5b02370a70ed2ec066",
             "b175a455bb7e5e4c0cc5b8d1d3cb9a70baf116b98d71e7903d0c5e97235ac923"},
    "run": {"5cb207a921dd8a22509e5ee6381cfb43d7fedece747041acdd8394f4ce823c5d"},
    # Exact base emitter and its ordinary SQ2 _X cap transport. ENV behavior is identical.
    # B7b: the transport's only change is the carried retained_w1 key (lib.rs:3489 -> 3474).
    # T4-U6/U8 (PP 20c6126a16): again only that key (lib.rs:3474 -> 3510; B7b's 221aa563c6d5e533...).
    # W1-L5: same _X emitter, with retained_w1 carried from3510 to3512.
    "tb": {"c6aad8333e010583195c24b3d8effe125d5386f04209c2cffe651486c2a4eb83",
           "0de99a4ac9ca186b441654d8434053ca626474b769d11aa4a3b6f93180b78471",
           "f8f52f6d35ee56b09dc9f8c554211b4f61d929146863d7e71688bcb671bc98a4",
           # T4-S5: the same _X emitter, with retained_w1 carried to lib.rs:3528 (its only difference).
           "ed2cadd1171a34588c2279ade89c3cf5aa5c5b8b0d424b73dad06291759d1ab1",
           # O4 S5: the same _X emitter, with retained_w1 carried to lib.rs:3531 (its only difference).
           "5addba6352f50241e19ed6004c97cb4c7c6949b2cc3b219d6ffa485449e32e0b",
           # Pre-0.4 failure unification D3 (§8.14): the same _X emitter, with retained_w1 carried to lib.rs:3616.
           "49accb86c11cdfb6a045246c0610226a1f9cb4715e2d48bdf4f58e80e88ef83b",
           # W-8 topology warnings D3 (§8.16): the same _X emitter, with retained_w1 carried to lib.rs:3618.
           "779a5d3019558c983accbb7e19e5c0507be150138d226c5baa0ea829ea9d07ec",
           # Silent-zero guard D3 (§8.15), merged on W-8: the same _X emitter, with retained_w1 carried to lib.rs:3619.
           "d8f2b65c5788b7e4104a6faa2acc4b666f54709fec7de0909422a8dbc0cf1c72",
           # Model document 0.7.0 D3 (§8.17), on the silent-zero guard: the same _X emitter, with retained_w1 carried to lib.rs:3626.
           "b8ad36562dbc2091bf1e83b297aac99a364a245944acad2aeeb68187253d709a",
           # Silent-drop guard D3 (§8.19): the same _X emitter, with retained_w1 carried to lib.rs:3689 (its only
           # difference from b8ad3656…).
           "836d05df66da34b1464bb5c067c6d8f68662e15d67b048f32662ea06901ad6f8",
           # W-7 D3 (§8.20): the same _X emitter, with retained_w1 carried to lib.rs:3694 (its only difference from
           # 836d05df…).
           "0955cc3338597f527e0d849d7dc58161059f2d1ea6af59fa04fae572b79448a8",
           # W-15 D3 (§8.21): the same _X emitter, with retained_w1 carried to lib.rs:3745 (its only difference from
           # 0955cc33…).
           "3f1e9c513fcafda1d2efd9232791644a3a863697d6d5c41d985bf87a72859af7",
           # W-16 slice a D3 (§8.23): the same _X emitter, with retained_w1 carried to lib.rs:3752 (its only difference
           # from 3f1e9c51…).
           "59bffb88632df3012358721e4ca240a5f509d79b750bf00f12e5883843ff2276"},
}


def envelope_projection(extra_zero):
    if not extra_zero:
        return False
    if extra_zero != ENVELOPE_ZERO:
        raise ValueError("B7a owned first error: unknown envelope exclusion")
    source = (Path(__file__).parent / "sens.py").read_bytes().decode("utf-8")
    tree = ast.parse(source)
    for name, hashes in MODE_SOURCE.items():
        nodes = [node for node in tree.body if
                 (isinstance(node, ast.FunctionDef) and node.name == name) or
                 (isinstance(node, ast.Assign) and any(isinstance(target, ast.Name)
                  and target.id == name for target in node.targets))]
        if len(nodes) != 1:
            raise ValueError("B7a owned first error: envelope emitter inventory changed")
        digest = hashlib.sha256(ast.get_source_segment(source, nodes[0]).encode()).hexdigest()
        if digest not in hashes:
            raise ValueError("B7a owned first error: envelope emitter source changed")
    return True


def adjacent(name):
    return runpy.run_path(os.path.join(os.path.dirname(__file__), name))


def inspect(snapshot):
    inherited = adjacent("b7a_formation_model.py")["inspect"](snapshot)
    fingerprints = {}
    for name, expected in SOURCES.items():
        raw = (Path(snapshot) / name).read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        if actual != expected:
            raise ValueError("B7a owned first-error source changed: " + name)
        fingerprints[name] = actual
    return {
        "formation_guard_sources": inherited,
        "owned_error_source_fingerprints": fingerprints,
        "model_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }


def rules(snapshot, resolved):
    inspect(snapshot)
    return [(resolved(site["short"]), {
        "from": [resolved(site["from"])], "per": "1", "why": WHY})
        for site in SITES]


def state(snapshot, table, graph, formats, lexical, extra_zero=""):
    inspect(snapshot)
    if table.get(MARKER) != VERSION:
        raise ValueError("B7a owned first error: missing/versioned guard")
    marked = {key for key, entry in table.get("site_from", {}).items()
              if "B7a owned first error:" in entry.get("why", "")}
    if marked != {spec["short"] for spec in SITES}:
        raise ValueError("B7a owned first error: marked allocation inventory changed")
    c6 = adjacent("kd5_first_error_model.py")
    active = []
    for spec in SITES:
        expected = {"from": [spec["from"]], "per": "1", "why": "SQ2 G5: " + WHY}
        if table.get("site_from", {}).get(spec["short"]) != expected:
            raise ValueError("B7a owned first error: exact rule changed: " + spec["short"])
        site = {"file": spec["file"], "line": spec["line"], "kind": spec["kind"],
                "fn": spec["owner"], "roots": [spec["owner"]]}
        for family in ("site_from", "site_zero", "site_total", "site_aggregate", "site_size", "lex_site_size"):
            for alias in c6["override_aliases"](site):
                if family == "site_from" and alias == spec["short"]:
                    continue
                if alias in table.get(family, {}):
                    raise ValueError("B7a owned first error: conflicting override: " + family + ":" + alias)
        edges = graph["edges"]
        if spec["owner"] not in edges or any(parent not in edges for parent in spec["parents"]):
            raise ValueError("B7a owned first error: missing qualified owner/caller node")
        parents = [parent for parent, children in edges.items()
                   for child in children if child == spec["owner"]]
        if sorted(parents) != sorted(spec["parents"]):
            raise ValueError("B7a owned first error: caller inventory changed")
        matches = [row for row in formats + lexical if
                   (row["file"], row["line"], row["kind"]) ==
                   (spec["file"], spec["line"], spec["kind"])]
        if len(matches) != 1 or matches[0].get("fn", spec["owner"]) != spec["owner"]:
            raise ValueError("B7a owned first error: missing/duplicate/misqualified allocation")
        active.append(site)
    return {"sites": active, "envelope_only": envelope_projection(extra_zero)}


def witness(active, file, line, kind, mult, counts, rebound=False):
    if (file, line, kind) not in {(s["file"], s["line"], s["kind"]) for s in SITES}:
        return
    if active["envelope_only"] and file == "core/product_physics/src/retained_product.rs":
        spec = next(s for s in SITES if (s["file"], s["line"], s["kind"]) == (file, line, kind))
        if mult != 0 or counts.get(spec["owner"], 0) != 0:
            raise ValueError("B7a owned first error: excluded envelope owner became positive")
        return
    c6 = adjacent("kd5_first_error_model.py")
    check = c6["rebound_witness"] if rebound else c6["positive_witness"]
    check(active, file, line, kind, mult, counts)
