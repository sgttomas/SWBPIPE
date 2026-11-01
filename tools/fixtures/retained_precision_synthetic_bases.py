#!/usr/bin/env python3
"""Regenerate the synthetic reader-control bases of the shared retained-precision corpus.

The corpus ``fixtures/results/retained_precision_cases.json`` holds three kinds of base.
Producer-solved and hook-produced bases come from their maintained producer writers and
are not touched here. The synthetic reader controls (provenance
``synthetic_reader_control_not_producer_execution_or_native_current``; DEL-04-08
``b7-plan.md`` "Terms", *The corpus*) are retained statements with no producer run. This
writer keeps their T4-U4b content current, deterministically, from each base's own data:

1. the captured request's model ``schema_version`` 0.5.0, 0.6.0, 0.7.0 or 0.8.0 is re-authored to
   0.9.0 (version token only; DEL-02-01 Design ``model-document-0.7.0.md`` section 2,
   ``model-document-0.8.0.md`` section 7 and ``model-document-0.9.0.md`` section 3.6);
2. ``formulation_basis`` takes the current text, read from the producer-written
   successor ``retained_precision_milestone_successor_sparse_interactive.json``;
3. the not-covered derived rows (sigma+, sigma-, tau_v,max at the five stations) are formed
   again from the base's own published rows, by the producer's binary64 sequences
   (``core/loads/stress_recovery/src/section_stress_quantities.rs``; DEL-04-07
   ``u4b-u5b-implementation.md`` 2.3 and 2.4), in the producer's row form, with identity,
   unit and metadata text from the same successor; they follow the member's
   ``pipe_elastic_normal_stress_maximum_v2`` row;
4. each case's ``selection.not_covered`` lists its derived ids in published row order;
5. projection ``row_index`` values, and corpus entry edit paths into ``results``, follow
   their rows by id;
6. the corpus rehash rule: the invocation digest, then the preparation, source,
   combination, publication and receipt hashes, in that order.

Existing derived rows are removed and formed again, so a current corpus regenerates to
itself: ``--check`` writes nothing and reports the first difference, if any, from the
committed bytes. ``expected`` and ``expected_classifications`` are not written; a change
in a base's row set is reported for review against the readers' tests.

The tau factor c_b uses the section the model defines on the preview route
(``derive_pipe_section``: d = D - 2t, r_i = d/2, I = pi(D^4 - d^4)/64, r_o = D/2) and
nu = E/(2G) - 1 from the moduli the case's solve uses: those of the case's retained source
member where the case has a source (the W1-L producer's operand), else the selected named
point's or the base moduli. Lengths in m and moduli in Pa only.

Standard library only. It shares no code with the independent Python verifiers or the
contract tests: the correctly rounded norm, canonical JSON (``openpipestress_jcs_ijson_v1``)
and the hash rule are written here.

    python3 tools/fixtures/retained_precision_synthetic_bases.py           # rewrite in place
    python3 tools/fixtures/retained_precision_synthetic_bases.py --check   # compare only
    python3 tools/fixtures/retained_precision_corpus.py synthetic-bases [--check]
"""
import argparse
import copy
import hashlib
import importlib.util
import json
import math
import struct
import sys
from decimal import Decimal
from fractions import Fraction
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[2]
CORPUS = "fixtures/results/retained_precision_cases.json"
PROBES = "fixtures/results/retained_precision_b2_parity_probes.json"
TEMPLATE = "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"
PREVIEW_TABLE = "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
EXACT_TABLE = "fixtures/results/semantic_contract_v0_3_physics_retained_1.json"
EXACT_CONTRACT = "openpipestress.result_semantics/0.3.0/physics-retained-1"
SYNTHETIC = "synthetic_reader_control_not_producer_execution_or_native_current"
# The current model document version; the previous versions are re-authored by their version token alone.
MODEL_VERSION = "0.9.0"
PREVIOUS_MODEL_VERSIONS = ("0.5.0", "0.6.0", "0.7.0", "0.8.0")

MAXIMUM = "pipe_elastic_normal_stress_maximum_v2"
EXTREME = "element_local_normal_stress_extreme"
SHEAR_MAX = "element_local_transverse_shear_stress_maximum"
DERIVED = (EXTREME, SHEAR_MAX)
STATIONS = (("end_i", "end-i"), ("end_j", "end-j"), ("quarter_1", "quarter-1"), ("midspan", "midspan"), ("quarter_3", "quarter-3"))
ROW_KEYS = ("basis_ref", "entity_ref", "id", "kind", "metadata", "recovery_method", "unit", "value")
SECTION_BASIS_SEPARATOR = "; section_basis="
SAFE_INTEGER = 2 ** 53 - 1


class WriterError(ValueError):
    pass


def _storage():
    spec = importlib.util.spec_from_file_location("retained_precision_corpus_storage", Path(__file__).with_name("retained_precision_corpus.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# --- canonical JSON (openpipestress_jcs_ijson_v1: RFC 8785 text, I-JSON numbers) and the hash rule ---

def _jcs_string(text):
    out = ['"']
    for ch in text:
        code = ord(ch)
        if 0xD800 <= code <= 0xDFFF:
            raise WriterError("canonical JSON: lone surrogate")
        if ch in '"\\':
            out.append("\\" + ch)
        elif code < 0x20:
            out.append({8: "\\b", 9: "\\t", 10: "\\n", 12: "\\f", 13: "\\r"}.get(code, "\\u%04x" % code))
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def _jcs_number(value):
    """ECMAScript Number::toString of a finite binary64 (shortest round-trip digits; -0 is 0)."""
    if not math.isfinite(value) or (value == int(value) and abs(value) > SAFE_INTEGER):
        raise WriterError("canonical JSON: number outside the profile")
    if value == 0:
        return "0"
    _, digit_tuple, exponent = Decimal(repr(abs(value))).normalize().as_tuple()
    digits = "".join(map(str, digit_tuple))
    k, n = len(digits), len(digits) + exponent      # value = 0.d1..dk x 10^n
    sign = "-" if value < 0 else ""
    if k <= n <= 21:
        return sign + digits + "0" * (n - k)
    if 0 < n <= 21:
        return sign + digits[:n] + "." + digits[n:]
    if -6 < n <= 0:
        return sign + "0." + "0" * (-n) + digits
    e = n - 1
    return sign + digits[0] + ("." + digits[1:] if k > 1 else "") + "e" + ("+" if e >= 0 else "-") + str(abs(e))


def canonical_text(value):
    if value is None:
        return "null"
    if value is True or value is False:
        return "true" if value else "false"
    if isinstance(value, str):
        return _jcs_string(value)
    if type(value) is int:
        if abs(value) > SAFE_INTEGER:
            raise WriterError("canonical JSON: integer outside the profile")
        return str(value)
    if type(value) is float:
        return _jcs_number(value)
    if type(value) is list:
        return "[" + ",".join(canonical_text(item) for item in value) + "]"
    if type(value) is dict:
        if not all(isinstance(key, str) for key in value):
            raise WriterError("canonical JSON: non-string key")
        keys = sorted(value, key=lambda key: key.encode("utf-16-be"))
        return "{" + ",".join(_jcs_string(key) + ":" + canonical_text(value[key]) for key in keys) + "}"
    raise WriterError(f"canonical JSON: unsupported {type(value).__name__}")


def domain_hash(domain, payload):
    return hashlib.sha256(canonical_text({"domain": domain, "payload": payload}).encode("utf-8")).hexdigest()


def _definition_hash(table, definition_id):
    data = json.loads((PROJECT / table).read_text(encoding="utf-8"))
    found = [d["sha256"] for d in data["product_formation_definitions"] if d["id"] == definition_id]
    if len(found) != 1:
        raise WriterError(f"{table}: definition {definition_id}")
    return found[0]


def _index(items, ref):
    """A corpus index: a JSON number (never a Boolean), finite, integral, >= 0 and not -0; else none."""
    if type(ref) not in (int, float) or not math.isfinite(ref) or ref != int(ref) or ref < 0:
        return None
    if ref == 0 and math.copysign(1.0, ref) < 0:
        return None
    return items[int(ref)] if int(ref) < len(items) else None


def _preparation_payload(record, definition_hash):
    return {"definition_id": record["definition_id"], "definition_sha256": definition_hash, "owner_ref": record["owner_ref"],
            "ordinary_attempt_ref": record["ordinary_attempt_ref"], "material_basis_ref": record["material_basis_ref"],
            "members": [{"member": m["member"], "old_source": m["old_source"], "old_facts": m["old_facts"], "section": m["result"]["section"]}
                        for m in record["preparation"]["members"]]}


def _source_identity(source):
    return domain_hash("retained_precision_source_mp_v2", {k: v for k, v in source.items() if k != "index"})


def rehash(source, invocation, definitions):
    """The corpus rehash rule ("rehash": "all") of a base, with its invocation digest bound first."""
    receipt = source["retained_precision"]
    body = receipt["body"]
    body["invocation"]["value"] = domain_hash("source_blocks_invocation_v1", invocation)
    route = definitions["exact"] if source.get("producer", {}).get("semantic_contract_id") == EXACT_CONTRACT else definitions["ordinary"]
    records = body.get("operand_preparations") if isinstance(body.get("operand_preparations"), list) else []
    for item in body["sources"]:
        preparation = item.get("preparation")
        if not isinstance(preparation, dict):
            continue
        attempt = _index(body["product_attempts"], preparation.get("attempt_ref"))
        if isinstance(attempt, dict) and isinstance(attempt.get("preparation"), dict) and all(
                m["result"]["kind"] == "prepared" for m in attempt["preparation"]["members"]):
            preparation["sha256"] = domain_hash("retained_precision_preparation_v1", _preparation_payload(attempt, route))
        record = _index(records, preparation.get("operand_preparation_ref"))
        if isinstance(record, dict) and record["result"]["kind"] == "prepared" and all(
                m["result"]["kind"] == "prepared" for m in record["preparation"]["members"]):
            preparation["sha256"] = domain_hash("retained_precision_operand_preparation_v1",
                                                dict(_preparation_payload(record, definitions["ordinary"]), purpose=record["purpose"]))
    for case in body["cases"]:
        item = _index(body["sources"], case.get("source_ref")) if case["status"] == "selected" else None
        if item is not None:
            case["source_identity_sha256"] = _source_identity(item)
    for item in body["sources"]:
        for operand in item.get("operands") if isinstance(item.get("operands"), list) else []:
            target = _index(body["sources"], operand.get("source_ref"))
            if target is not None:
                operand["source_identity_sha256"] = _source_identity(target)
    for entry in body.get("combinations") if isinstance(body.get("combinations"), list) else []:
        item = _index(body["sources"], entry.get("source_ref")) if entry.get("disposition") == "retained_selected" else None
        if item is not None:
            entry["source_identity_sha256"] = _source_identity(item)
    body["publication_sha256"] = domain_hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    receipt["receipt_sha256"] = domain_hash("retained_precision_receipt_mp_v2", body)


# --- the producer's binary64 sequences (section_stress_quantities.rs) ---

def norm2(a, b):
    """RN(sqrt(a^2 + b^2)), ties to even: a binary64 candidate settled against exact midpoints."""
    target = Fraction(a) ** 2 + Fraction(b) ** 2
    if target == 0:
        return 0.0
    candidate = math.hypot(a, b)

    def side(lower, upper):    # the sign of root - midpoint(lower, upper)
        if not math.isfinite(upper):
            raise WriterError("norm2 outside binary64")
        middle = (Fraction(lower) + Fraction(upper)) / 2
        return (target > middle * middle) - (target < middle * middle)

    while True:
        up, down = math.nextafter(candidate, math.inf), math.nextafter(candidate, 0.0)
        above = side(candidate, up)
        if above > 0 or (above == 0 and _odd(candidate)):
            candidate = up
            continue
        below = side(down, candidate) if candidate > 0 else 1
        if below < 0 or (below == 0 and _odd(candidate)):
            candidate = down
            continue
        return candidate


def _odd(value):
    return struct.unpack(">Q", struct.pack(">d", value))[0] & 1 == 1


def extremes(sigma_n, sigma_y, sigma_z):
    h = norm2(sigma_y, sigma_z)
    return sigma_n + h, sigma_n - h


def shear_factor(nu, outer, inner, inertia):
    """c_b = RN(num/den); None outside -1 < nu < 1/2 or for an invalid annulus."""
    if not (math.isfinite(nu) and -1.0 < nu < 0.5):
        return None
    if not all(math.isfinite(v) and v > 0.0 for v in (outer, inner, inertia)) or inner >= outer:
        return None
    num = (3.0 + 2.0 * nu) * (outer * outer) + (1.0 + 2.0 * nu) * (inner * inner)
    den = (4.0 * (1.0 + nu)) * inertia
    factor = num / den
    return factor if math.isfinite(factor) and factor > 0.0 else None


def preview_shear_factor(elastic, shear, outer, inner, inertia):
    """c_b from the moduli (DEL-04-07 §2.4): the nu sequence while RN(E/RN(2G)) >= 1/2, else the x = RN(RN(2G)/E)
    form without 1 + nu; None outside -1 < nu < 1/2 (admission by the rounded nu)."""
    ratio = elastic / (2.0 * shear)
    if not ratio < 0.5:
        return shear_factor(ratio - 1.0, outer, inner, inertia)
    if not (ratio - 1.0 > -1.0 and elastic > 0.0 and shear > 0.0):
        return None
    if not all(math.isfinite(v) and v > 0.0 for v in (outer, inner, inertia)) or inner >= outer:
        return None
    x = (2.0 * shear) / elastic
    if not (math.isfinite(x) and x > 2.0 / 3.0):
        return None
    factor = (2.0 * (outer * outer + inner * inner) + x * ((outer - inner) * (outer + inner))) / (4.0 * inertia)
    return factor if math.isfinite(factor) and factor > 0.0 else None


def shear_maximum(shear_y, shear_z, factor):
    return norm2(shear_y, shear_z) * factor / 1e6


# --- one synthetic base ---

def _canonical(quantity, unit, what):
    if not isinstance(quantity, dict) or quantity.get("unit") != unit or type(quantity.get("value")) not in (int, float):
        raise WriterError(f"{what}: only {unit} is supported")
    return float(quantity["value"])


def _word(word):
    return struct.unpack(">d", bytes.fromhex(word))[0]


def _moduli(model, body, case_id, pipe):
    """(E, G) the case's solve uses for the member, in Pa."""
    for case in body["cases"]:
        if case["basis_ref"]["ref_id"] != case_id:
            continue
        item = _index(body["sources"], case.get("source_ref"))
        if isinstance(item, dict) and isinstance(item.get("id_maps"), dict):
            members = [m for m in item["id_maps"]["members"] if m.get("id") == pipe]
            if len(members) != 1:
                raise WriterError(f"{case_id}/{pipe}: source member")
            return _word(members[0]["E"]), _word(members[0]["G"])
    load_case = next(c for c in model["load_cases"] if c["id"] == case_id)
    pipe_segment = next(p for p in model["pipe_segments"] if p["id"] == pipe)
    material = next(m for m in model["materials"] if m["id"] == pipe_segment["material"])
    if load_case.get("modulus_basis_temperature") is not None:
        raise WriterError(f"{case_id}: interpolated moduli without a retained source are not reproduced")
    point_id = load_case.get("modulus_basis_ref")
    holder = material if point_id is None else next(p for p in material["temperature_points"] if p["id"] == point_id)
    return _canonical(holder["elastic_modulus"], "Pa", "modulus"), _canonical(holder["shear_modulus"], "Pa", "modulus")


def _section(model, pipe):
    section = next(p for p in model["pipe_segments"] if p["id"] == pipe)["section"]
    diameter = _canonical(section["outside_diameter"], "m", "outside diameter")
    wall = _canonical(section["wall_thickness"], "m", "wall thickness")
    if section.get("mill_tolerance") is not None:
        wall = wall - _canonical(section["mill_tolerance"], "m", "mill tolerance")
    bore = diameter - 2.0 * wall
    # Rust f64::powi(4) is (x*x)*(x*x).
    inertia = math.pi * ((diameter * diameter) * (diameter * diameter) - (bore * bore) * (bore * bore)) / 64.0
    return diameter / 2.0, bore / 2.0, inertia


def _case_of(row):
    basis = row.get("basis_ref") or {}
    return basis.get("ref_id") if basis.get("ref_type") == "load_case" else None


def _one(rows, case_id, pipe, kind, location, component=None):
    found = [r for r in rows if _case_of(r) == case_id and r.get("entity_ref") == pipe and r.get("kind") == kind
             and (r.get("metadata") or {}).get("location") == location
             and SECTION_BASIS_SEPARATOR not in ((r.get("metadata") or {}).get("basis") or "")
             and (component is None or (r.get("metadata") or {}).get("component") == component)]
    if len(found) > 1:
        raise WriterError(f"{case_id}/{pipe}: {kind} {location} is not unique")
    return found[0]["value"] if found else None


def row_templates(template):
    """The producer's derived-row text by component (identical apart from location)."""
    out = {}
    for row in template["source"]["results"]:
        if row["kind"] not in DERIVED:
            continue
        form = {"kind": row["kind"], "unit": row["unit"], "metadata": row["metadata"]}
        component = row["metadata"]["component"]
        known = out.setdefault(component, form)
        if {k: v for k, v in known["metadata"].items() if k != "location"} != {k: v for k, v in form["metadata"].items() if k != "location"} \
                or (known["kind"], known["unit"]) != (form["kind"], form["unit"]):
            raise WriterError(f"template: {component} rows differ")
    if set(out) != {"sigma_plus", "sigma_minus", "tau_v_max"}:
        raise WriterError("template: derived rows missing")
    return out


def case_rows(source):
    out = {}
    for row in source["results"]:
        case_id = _case_of(row)
        if case_id is not None:
            out.setdefault(case_id, []).append(row["id"])
    return out


def regenerate_base(base, template, forms, definitions):
    """The base's new (source, invocation)."""
    invocation = copy.deepcopy(base["invocation"])
    model = invocation["request"]["model"]
    if model.get("schema_version") in PREVIOUS_MODEL_VERSIONS:
        model["schema_version"] = MODEL_VERSION
    old = base["source"]
    source = copy.deepcopy(old)
    source["formulation_basis"] = copy.deepcopy(template["source"]["formulation_basis"])
    body = source["retained_precision"]["body"]
    derived_ids = {r["id"] for r in source["results"] if r["kind"] in DERIVED}
    own = [r for r in source["results"] if r["kind"] not in DERIVED]
    for case in body["cases"]:
        selection = case.get("selection")
        if isinstance(selection, dict) and isinstance(selection.get("not_covered"), list):
            selection["not_covered"] = [i for i in selection["not_covered"] if i not in derived_ids]
    results, added = [], {}
    for row in own:
        results.append(row)
        if row["kind"] != MAXIMUM:
            continue
        case_id, pipe = _case_of(row), row["entity_ref"]
        axial_ids = [r["id"] for r in own if r["kind"] == "element_local_axial_normal_stress" and r["entity_ref"] == pipe and _case_of(r) == case_id]
        if not axial_ids:
            raise WriterError(f"{base['id']}: {case_id}/{pipe} has no axial stress rows")
        prefix = axial_ids[0].rsplit(":", 2)[0]
        elastic, shear = _moduli(model, body, case_id, pipe)
        outer, inner, inertia = _section(model, pipe)
        factor = preview_shear_factor(elastic, shear, outer, inner, inertia)
        for location, token in STATIONS:
            n = _one(own, case_id, pipe, "element_local_axial_normal_stress", location, "axial_normal_stress")
            y = _one(own, case_id, pipe, "element_local_bending_normal_stress_y", location, "bending_normal_stress_y")
            z = _one(own, case_id, pipe, "element_local_bending_normal_stress_z", location, "bending_normal_stress_z")
            vy = _one(own, case_id, pipe, "element_local_shear_force_y", location)
            vz = _one(own, case_id, pipe, "element_local_shear_force_z", location)
            values = []
            if None not in (n, y, z):
                plus, minus = extremes(n, y, z)
                values += [("sigma_plus", "sigma-plus", plus), ("sigma_minus", "sigma-minus", minus)]
            if None not in (vy, vz):
                if factor is None:
                    raise WriterError(f"{base['id']}: {case_id}/{pipe}: nu outside the Saint-Venant domain needs the producer's diagnostic")
                values.append(("tau_v_max", "transverse-shear-max", shear_maximum(vy, vz, factor)))
            for component, suffix, value in values:
                form = forms[component]
                new = {"basis_ref": copy.deepcopy(row["basis_ref"]), "entity_ref": pipe, "id": f"{prefix}:{token}:{suffix}", "kind": form["kind"],
                       "metadata": dict(copy.deepcopy(form["metadata"]), location=location), "recovery_method": row.get("recovery_method"),
                       "unit": form["unit"], "value": value}
                if "recovery_method" not in row:
                    del new["recovery_method"]
                new = {k: new[k] for k in ROW_KEYS if k in new}
                results.append(new)
                added.setdefault(case_id, []).append(new["id"])
    source["results"] = results
    order = {r["id"]: i for i, r in enumerate(results)}
    if len(order) != len(results):
        raise WriterError(f"{base['id']}: duplicate row ids")
    for case in body["cases"]:
        selection = case.get("selection")
        case_id = case["basis_ref"]["ref_id"]
        if isinstance(selection, dict) and isinstance(selection.get("not_covered"), list) and case_id in added:
            ids = list(dict.fromkeys(selection["not_covered"] + added[case_id]))
            selection["not_covered"] = sorted(ids, key=lambda i: order.get(i, -1))
    old_rows, new_rows = case_rows(old), case_rows(source)
    for attempt in body["product_attempts"]:
        if not isinstance(attempt, dict) or not isinstance(attempt.get("proof"), dict):
            continue
        owner = attempt.get("owner_ref") or {}
        if owner.get("kind") != "case":
            continue
        case_id = body["cases"][owner["index"]]["basis_ref"]["ref_id"]
        for outcome in attempt["proof"].get("projection_outcomes") or []:
            i = outcome.get("row_index")
            if type(i) is int and 0 <= i < len(old_rows.get(case_id, [])):
                rid = old_rows[case_id][i]
                if rid not in new_rows.get(case_id, []):
                    raise WriterError(f"{base['id']}: projected row {rid} is gone")
                outcome["row_index"] = new_rows[case_id].index(rid)
    rehash(source, invocation, definitions)
    return source, invocation


def follow_rows(entry, old_ids, new_ids):
    """Edit paths ["results", i, ...] follow their rows by id; returns whether any moved."""
    moved = False
    for key in ("edits", "after_rehash"):
        for edit in entry.get(key) or []:
            path = edit["path"]
            if len(path) >= 2 and path[0] == "results" and type(path[1]) is int and path[1] < len(old_ids):
                if old_ids[path[1]] not in new_ids:
                    raise WriterError(f"{entry.get('id', entry.get('name'))}: edited row {old_ids[path[1]]} is gone")
                index = new_ids.index(old_ids[path[1]])
                if index != path[1]:
                    edit["path"] = ["results", index] + path[2:]
                    moved = True
    return moved


def regenerate(corpus_doc, probes_doc):
    """(new corpus, new probes, report) from the expanded corpus and the probe shapes."""
    definitions = {"ordinary": _definition_hash(PREVIEW_TABLE, "RP-PREPARED-ORDINARY-DUAL-v1"),
                   "exact": _definition_hash(EXACT_TABLE, "RP-PREPARED-EXACT-DUAL-v1")}
    template = json.loads((PROJECT / TEMPLATE).read_text(encoding="utf-8"))
    forms = row_templates(template)
    corpus_doc, probes_doc = copy.deepcopy(corpus_doc), copy.deepcopy(probes_doc)
    report = {"bases": 0, "row_sets_changed": [], "entries_moved": []}
    ids = {}
    for base in corpus_doc["cases"]:
        if base["provenance"] != SYNTHETIC:
            continue
        old_ids = [r["id"] for r in base["source"]["results"]]
        base["source"], base["invocation"] = regenerate_base(base, template, forms, definitions)
        new_ids = [r["id"] for r in base["source"]["results"]]
        ids[base["id"]] = (old_ids, new_ids)
        report["bases"] += 1
        if set(old_ids) != set(new_ids):
            report["row_sets_changed"].append(base["id"])
    entries = [e for kind in ("mutations", "must_pass") for e in corpus_doc.get(kind, [])] + list(probes_doc["shapes"])
    for entry in entries:
        if entry["base"] in ids and follow_rows(entry, *ids[entry["base"]]):
            report["entries_moved"].append(entry.get("id", entry.get("name")))
    return corpus_doc, probes_doc, report


def _first_difference(old, new, path=()):
    if type(old) is not type(new):
        return path
    if isinstance(old, dict):
        for key in list(old) + [k for k in new if k not in old]:
            if key not in old or key not in new:
                return path + (key,)
            found = _first_difference(old[key], new[key], path + (key,))
            if found is not None:
                return found
        return None if list(old) == list(new) else path
    if isinstance(old, list):
        for i, (a, b) in enumerate(zip(old, new)):
            found = _first_difference(a, b, path + (i,))
            if found is not None:
                return found
        return None if len(old) == len(new) else path
    return None if repr(old) == repr(new) else path


def _probe_bytes(doc):
    return (json.dumps(doc, indent=1, ensure_ascii=False) + "\n").encode("utf-8")


def run(check=False, project=PROJECT, out=sys.stdout):
    storage = _storage()
    corpus_path, probes_path = project / CORPUS, project / PROBES
    corpus_raw = corpus_path.read_bytes()
    definition = json.loads(corpus_raw)
    probes_raw = probes_path.read_bytes()
    probes = json.loads(probes_raw)
    if _probe_bytes(probes) != probes_raw:
        raise WriterError(f"{PROBES}: not in its indent-1 serialization")
    old = storage.expand(definition)
    new, new_probes, report = regenerate(old, probes)
    packed = storage.compact(storage._bytes(new, definition["layout"]))
    corpus_bytes = (json.dumps(packed, separators=(",", ":"), ensure_ascii=True, allow_nan=False) + "\n").encode()
    probe_bytes = _probe_bytes(new_probes)
    differences = []
    if corpus_bytes != corpus_raw:
        differences.append((CORPUS, _first_difference(old, new)))
    if probe_bytes != probes_raw:
        differences.append((PROBES, _first_difference(probes, new_probes)))
    print(f"synthetic bases regenerated: {report['bases']}; row sets changed: {report['row_sets_changed']}; "
          f"entries whose result paths moved: {len(report['entries_moved'])}", file=out)
    for name, path in differences:
        print(f"{'differs' if check else 'rewritten'}: {name} first at {list(path) if path is not None else 'serialization'}", file=out)
    if check:
        print("check: " + ("0 differences" if not differences else f"{len(differences)} file(s) differ"), file=out)
        return 1 if differences else 0
    if report["row_sets_changed"]:
        print("review: expected_classifications and entries on these bases are not rewritten", file=out)
    if corpus_bytes != corpus_raw:
        corpus_path.write_bytes(corpus_bytes)
    if probe_bytes != probes_raw:
        probes_path.write_bytes(probe_bytes)
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--check", action="store_true", help="compare with the committed bytes; write nothing")
    args = parser.parse_args(argv)
    return run(check=args.check)


if __name__ == "__main__":
    raise SystemExit(main())
