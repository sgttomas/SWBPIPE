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
SITES = [
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 10470,
    "kind": "format",
    "owner": "core/product_physics/src/lib.rs:10429:exact_straight_summary_extrema",
    "short": "lib.rs:10470",
    "from": "lib.rs:10429:exact_straight_summary_extrema",
    "parents": [
      "core/product_physics/src/lib.rs:4442:solve_prepared_load_case",
      "core/product_physics/src/membrane_publication_range.rs:158:original_subnormal_wall_force_direct_extrema_preserves_membrane",
      "core/product_physics/src/membrane_publication_range.rs:232:unclamped_subnormal_wall_force_extrema_midpoint_is_the_membrane",
      "core/product_physics/src/retained_product.rs:5453:prepared_maxima"
    ]
  },
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 10476,
    "kind": "format",
    "owner": "core/product_physics/src/lib.rs:10429:exact_straight_summary_extrema",
    "short": "lib.rs:10476",
    "from": "lib.rs:10429:exact_straight_summary_extrema",
    "parents": [
      "core/product_physics/src/lib.rs:4442:solve_prepared_load_case",
      "core/product_physics/src/membrane_publication_range.rs:158:original_subnormal_wall_force_direct_extrema_preserves_membrane",
      "core/product_physics/src/membrane_publication_range.rs:232:unclamped_subnormal_wall_force_extrema_midpoint_is_the_membrane",
      "core/product_physics/src/retained_product.rs:5453:prepared_maxima"
    ]
  },
  {
    "file": "core/product_physics/src/lib.rs",
    "line": 11141,
    "kind": "to_string",
    "owner": "core/product_physics/src/lib.rs:11122:recover_curved_bend_local_forces",
    "short": "lib.rs:11141",
    "from": "lib.rs:11122:recover_curved_bend_local_forces",
    "parents": [
      "core/product_physics/src/lib.rs:4442:solve_prepared_load_case"
    ]
  },
  {
    "file": "core/product_physics/src/retained_product.rs",
    "line": 1745,
    "kind": "into_text",
    "owner": "core/product_physics/src/retained_product.rs:1705:check_support_maps",
    "short": "retained_product.rs:1745",
    "from": "retained_product.rs:1705:check_support_maps",
    "parents": [
      "core/product_physics/src/retained_product.rs:1702:check_support_source",
      "core/product_physics/src/retained_product.rs:1835:capture_case_structure"
    ]
  },
  {
    "file": "core/product_physics/src/retained_product.rs",
    "line": 2831,
    "kind": "into_text",
    "owner": "core/product_physics/src/retained_product.rs:2811:support_observables",
    "short": "retained_product.rs:2831",
    "from": "retained_product.rs:2811:support_observables",
    "parents": [
      "core/product_physics/src/retained_product.rs:2850:case_observables",
      "core/product_physics/src/retained_product.rs:6369:combination_observables"
    ]
  }
]


ENVELOPE_ZERO = ("core/product_physics/src/source_receipt,core/product_physics/src/retained_wire.rs,"
                 "core/product_physics/src/retained_product.rs,core/product_physics/src/retained_receipt.rs,"
                 "core/reporting/result_export")
MODE_SOURCE = {
    "ENVZERO": {"c0916b98869910d07737f0917f43c3629bb7185c05fbfbb4ddc1ef953aab0df8"},
    "env": {"504db9604b77de9dd1ec933852b3e9820d6f840611135d5b02370a70ed2ec066"},
    "run": {"5cb207a921dd8a22509e5ee6381cfb43d7fedece747041acdd8394f4ce823c5d"},
    # Exact base emitter and its ordinary SQ2 _X cap transport. ENV behavior is identical.
    "tb": {"0de99a4ac9ca186b441654d8434053ca626474b769d11aa4a3b6f93180b78471",
           "366f2344cfc38cb99aa40fa93173a9a516bb9a1b409179f4f6152fc3a7fc375c"},
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
