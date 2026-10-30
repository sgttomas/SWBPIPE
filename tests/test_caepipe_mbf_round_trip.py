#!/usr/bin/env python3
"""Round trip of the DEL-17-04 SI MBF writer and MBF importer on invented models (Design `caepipe-mbf-import.md`
§18.1 items 3 to 6, §18.2).

For each model A in the reversible subset, W = writer(A) and I = import(W) with the bound sidecar:
(a) canonical equality within the derived per-scalar bounds; (a') unbound imports: under profile decision PD-1
canonical equality by CAEPIPE name, and under the base rule the named refusal MBF-IMPORT-TBD-17-04-007;
(b) the identities and splits the sidecar preserves are restored, and every other identity is reported;
(c) byte reproduction writer(I) == W for the byte-reproducible subset, with every formatting collision reported;
(d) solve equality against a provisional threshold; (e) determinism. Then the English/SI units check, the bend
boundary case and schema validation. Every model and fixture here is invented; none is engineering data.
"""

from __future__ import annotations

from copy import deepcopy
from fractions import Fraction
import hashlib
import json
import math
import os
import random
import subprocess
from pathlib import Path
import sys
from typing import Any

import pytest


ROOT = Path(__file__).resolve().parents[1]
for extra in (ROOT, ROOT / "tests", ROOT / "tools" / "validation"):
    if str(extra) not in sys.path:
        sys.path.insert(0, str(extra))

import caepipe_mbf_canonical as canon  # noqa: E402
from core.handoff.caepipe_mbf import importer as importer_module  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from core.handoff.caepipe_mbf.importer import (  # noqa: E402
    canonical_sidecar, sidecar_difference, summed_entries)
from core.handoff.caepipe_mbf.importer import import_caepipe_mbf as import_as_given  # noqa: E402
from core.handoff.caepipe_mbf.si_writer import (  # noqa: E402
    SIDECAR_FORMAT, regenerate_caepipe_sidecar, sidecar_canonical_bytes, sidecar_self_digest)
from schema_validation import JsonSchemaDependencyMissing, _skip_or_note_missing_jsonschema, validate_instance  # noqa: E402
from test_model_schema import desktop_entity_requirements  # noqa: E402
import test_caepipe_mbf_si_writer as writer_tests  # noqa: E402
import test_caepipe_mbf_importer as importer_tests  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402
from tools.caepipe_correlation.run_pilot import load_cases, solve_swbpipe  # noqa: E402


INVENTED = {"*": {"origin": "project_invented", "source_name": "invented round-trip model",
                  "redistribution_status": "public_permissive"}}


def import_caepipe_mbf(data: bytes, *, sidecar=None, **options):
    """The importer, given `sidecar` as a deliberate editor leaves it (Design §15): each edit in this module has its
    self-digest recomputed (canon.redigested), so it reaches the parser, the agreement rule and the regeneration
    identity behind the self-digest; an unedited writer sidecar is unchanged by it. `import_as_given` imports a
    sidecar exactly as given: the writer's own sidecars, and every edit again with its self-digest left stale."""
    return import_as_given(data, sidecar=canon.redigested(sidecar), **options)
FIXTURES = ROOT / "fixtures" / "caepipe_mbf" / "invented" / "import"
DESKTOP_SCHEMA = {"$schema": "https://json-schema.org/draft/2020-12/schema",
                  "$ref": "https://openpipestress.org/schemas/model.schema.yaml#/$defs/DesktopDocument"}


def both_given_material():
    model = writer_tests.cantilever()
    model["materials"][0] = {"id": "mat:CS", "elastic_modulus": writer_tests.q(2.0e11, "Pa"),
                             "poisson_ratio": writer_tests.q(0.25, "1"), "shear_modulus": writer_tests.q(8.0e10, "Pa"),
                             "thermal_expansion_coefficient": writer_tests.q(1.2e-5, "1/K")}
    return model


def weighted():
    model = writer_tests.cantilever()
    od, wall = 0.1683, 0.00711
    area = math.pi / 4.0 * (od**2 - (od - 2 * wall) ** 2)
    model["nodes"].append(writer_tests.node("N2", 6.0))
    model["pipe_segments"].append(writer_tests.pipe("S2", "N1", "N2", od, wall, "m"))
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "w1", "category": "weight", "target": {"type": "element", "pipe": "S1"}, "direction": "global_y",
         "magnitude": writer_tests.q(-area * 7850.0 * 9.80665, "N/m"), "dimension": "force_per_length"})
    return model


def reversed_bend():
    model = writer_tests.bent_l()
    model["supports"] = [{"id": "SB", "node": "B", "restraints": list(writer_tests.SIX)}]
    return model


def bend_with_sifs():
    model = writer_tests.bent_l_with_sifs()
    model["components"][0]["modifiers"]["source_reference"] = "invented round-trip SIF pair"
    return model


def bend_boundary(residual: bool = True):
    """Invented 90 degree bend, R = 999 mm, tangent lengths 999.0000008991 mm, adjacent straights 0.5 mm (§18.1 item 5)."""
    tangent = 999.0000008991 if residual else 999.0
    model = writer_tests.model_shell("mm")
    ti = 0.5 + tangent
    model["nodes"] = [writer_tests.node("P", 0.0), writer_tests.node("N", 0.5), writer_tests.node("F", ti, 0.0, tangent),
                      writer_tests.node("Q", ti, 0.0, tangent + 0.5)]
    model["pipe_segments"] = [writer_tests.pipe("S1", "P", "N", 168.3, 7.11, "mm"),
                              writer_tests.pipe("BEND", "N", "F", 168.3, 7.11, "mm"),
                              writer_tests.pipe("S2", "F", "Q", 168.3, 7.11, "mm")]
    for item in model["pipe_segments"]:
        item["y_reference"] = {"x": 0.0, "y": 1.0, "z": 0.0}
    model["pipe_segments"][1]["y_reference"] = {"x": 1.0, "y": 0.0, "z": -1.0}
    model["supports"] = [{"id": "AP", "node": "P", "restraints": list(writer_tests.SIX)},
                         {"id": "AQ", "node": "Q", "restraints": list(writer_tests.SIX)}]
    model["components"] = [{"id": "bend-b", "kind": "bend", "node": "N",
                            "geometry": {"bend_pipe_ref": "BEND", "bend_radius": writer_tests.q(999.0, "mm")},
                            "modifiers": {"flexibility_factor_user_value": writer_tests.q(1.0, "none")},
                            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}}]
    model["load_cases"] = [{"id": "case:th", "primitive_loads": [
        {"id": f"t:{pipe_id}", "category": "thermal", "target": {"type": "element", "pipe": pipe_id},
         "direction": "global_x", "magnitude": writer_tests.q(50.0, "degC"), "dimension": "temperature_interval"}
        for pipe_id in ("S1", "BEND", "S2")]}]
    return model


CLOSURE = {"closure_transfer": "transfers_to_wall", "provenance": "invented round-trip declaration"}
# Invented Z-vertical files (Design §5.1) with pressure (§9.4), in SI (bar) and English (psi) units. The SI file's
# skew straight meets the next straight at a corner, so its import has two regions.
Z_VERTICAL_SI = (
    "INVENTED Z-VERTICAL SI", "VERSION", "15.10", "OPTIONS", "HGRA,SI,Z", "OPTEMP", "20", "MATERIAL",
    "M1,7850,0.3,1,1,CS,invented steel", "M1,20,200000,1.2e-05", "PIPE", "P1,168.3,7.11,0,0,0,0,0,0", "LOADS",
    "L1,120,15,0", "LAYOUT", "F10,KA,MM1,PP1,CL=L1", "T20,X1000,Y2000,Z3000,CFOR(10,20,30),FLC=1",
    "T30,KI,B250,X4000,CFF=1.5", "T40,KA,Z3000")
Z_VERTICAL_ENGLISH = (
    "INVENTED Z-VERTICAL ENGLISH", "VERSION", "15.10", "OPTIONS", "HGRA,Z", "OPTEMP", "70", "MATERIAL",
    "M1,0.283,0.3,1,1,CS,invented steel", "M1,70,29000000,6.5e-06", "PIPE", "P1,6.625,0.28,0,0,0,0,0,0", "LOADS",
    "L1,250,150,0", "LAYOUT", "F10,KA,MM1,PP1,CL=L1", "T20,X10'6\",CFOR(0,0,-1000),FLC=1", "T30,KA,Y8")


def z_import(lines: tuple[str, ...]):
    data = "".join(line + "\r\n" for line in lines).encode("ascii")
    result = import_caepipe_mbf(data, material_provenance=INVENTED, pressure_closure=CLOSURE, source_label=lines[0])
    assert result.status == "imported", result.diagnostics
    return result


def models() -> dict[str, tuple[dict, str, float]]:
    found = {case["case_id"]: (case["model"], case["title"], case["reference_temperature_c"]) for case in load_cases()}
    found.update({
        "writer-cantilever-m": (writer_tests.cantilever("m"), "INVENTED CANTILEVER", 20.0),
        "writer-cantilever-mm": (writer_tests.cantilever("mm"), "INVENTED CANTILEVER", 20.0),
        "writer-branched": (writer_tests.branched(), "INVENTED BRANCH", 20.0),
        "writer-weight": (weighted(), "WEIGHT", 20.0),
        "writer-bent-l": (writer_tests.bent_l(), "INVENTED L", 20.0),
        "writer-bent-l-sifs": (bend_with_sifs(), "INVENTED L SIF", 20.0),
        "writer-nu-and-g": (both_given_material(), "NU AND G", 20.0),
        "writer-straight-run": (writer_tests.straight_run(20), "RUN", 20.0),
        "bend-boundary": (bend_boundary(), "BEND BOUNDARY", 20.0),
        "writer-cancelling-forces": (writer_tests.cancelling_cantilever(), "CANCEL", 20.0),
        "pressure-l-pa": (writer_tests.pressurized_l(), "PRESSURE L", 20.0),
        "pressure-tee-bar": (writer_tests.pressurized_tee(), "PRESSURE TEE", 20.0),
        "pressure-us-psi": (writer_tests.pressurized_us_cantilever(), "PRESSURE US", 20.0),
        "writer-springs": (writer_tests.sprung_cantilever(), "SPRING", 20.0),
        "z-vertical-si": (z_import(Z_VERTICAL_SI).model, "Z VERTICAL SI", 20.0),
        "z-vertical-english": (z_import(Z_VERTICAL_ENGLISH).model, "Z VERTICAL ENGLISH", 20.0),
    })
    return found


MODELS = models()
# Reversible subset (§18.2): written, no MBF-SI-SHEAR-MODULUS-NOT-EXPORTED, printed values valid, distinct keys kept.
REVERSIBLE = sorted(MODELS)
# Excluded by name, with the reason: the component names the far tangent node, which the sidecar does not carry
# (the importer reports MBF-IMPORT-ID-UNRECOVERABLE for every bound bend component node).
EXCLUDED = {"writer-bent-l-reversed": (reversed_bend(), "component node is the far tangent node; not in the sidecar")}
# Byte-reproducible subset (§18.2): bound sidecar, identical writer settings, no formatting collision.
BYTE_REPRODUCIBLE = sorted(MODELS)
SOLVE_THRESHOLD_FACTOR = 1e3  # ASSUMPTION (Design §18.1(d)): provisional, not a derived bound
SOLVE_R_IN_LIMIT = 1e-8
# Models whose source A the headless solve itself refuses, so there is no solve to compare (listed by name).
SOLVE_EXCLUDED = {
    "writer-branched": "A authors nu without G or a constitutive basis, which the solve refuses",
    "writer-straight-run": "A has no primitive load",
}
PRESSURE_MODELS = {name for name, (model, _, _) in MODELS.items() if model.get("pressure_contract")}


def unbound_closure(trip) -> dict | None:
    """The closure declaration an unbound import of `trip` needs: its source's, by CAEPIPE node name."""
    if trip.name not in PRESSURE_MODELS:
        return None
    nodes = {trip.writer.node_map[terminal["node_ref"]]: terminal["closure_transfer"]
             for case in trip.model["load_cases"] for region in case.get("pressure_regions") or []
             for terminal in region["terminals"] if terminal["closure_transfer"] != "transfers_to_wall"}
    return {**CLOSURE, "nodes": nodes}


class Trip:
    def __init__(self, name: str, model: dict, title: str, t_ref: float) -> None:
        self.name, self.model, self.title, self.t_ref = name, model, title, t_ref
        self.writer = render_caepipe_mbf_si(model, title=title, reference_temperature_c=t_ref)
        assert self.writer.status == "written", self.writer.diagnostics
        assert "MBF-SI-SHEAR-MODULUS-NOT-EXPORTED" not in {item["code"] for item in self.writer.diagnostics}
        self.text = self.writer.text
        self.data = self.text.encode("ascii")
        self.sidecar = self.writer.sidecar(hashlib.sha256(self.data).hexdigest())
        self.bound = import_as_given(self.data, sidecar=self.sidecar, material_provenance=INVENTED, source_label=name)
        self.printed = canon.read_printed(self.text)

    def kinds(self, model: dict) -> dict[str, str]:
        labels = {"Sustained (W+P)": "sustained", "Expansion (T1)": "expansion"}
        mapping = {}
        for case in model.get("load_cases") or []:
            label = case.get("label") or self.writer.load_case_map.get(case["id"])
            if label in labels:
                mapping[case["id"]] = labels[label]
        return mapping

    def reference(self) -> canon.Canonical:
        nu = {record["swbpipe_material"]: record["poisson_ratio"] for record in self.writer.material_map.values()}
        return canon.canonical(self.model, self.writer.node_map, self.kinds(self.model), oriented=True, writer_nu=nu)

    def imported(self, result, *, oriented: bool) -> canon.Canonical:
        names = {swb_id: name for name, swb_id in result.id_map["node_map"].items()}
        return canon.canonical(result.model, names, self.kinds(result.model), oriented=oriented)

    def limits(self, reference, *, residuals: bool = True):
        return canon.bounds(self.model, self.writer, self.printed, self.writer.node_map, reference, residuals=residuals)


@pytest.fixture(scope="module")
def trips() -> dict[str, Trip]:
    return {name: Trip(name, *MODELS[name]) for name in REVERSIBLE}


# --- (a) canonical equality, bound ----------------------------------------------------------------
@pytest.mark.parametrize("name", REVERSIBLE)
def test_bound_import_equals_the_source_within_the_derived_bounds(trips, name):
    trip = trips[name]
    assert trip.bound.status == "imported", trip.bound.diagnostics
    assert trip.bound.source["binding"] == "bound"
    reference = trip.reference()
    result = canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference))
    assert result.failures == []
    assert result.compared > 0


def test_every_excluded_loss_and_model_is_listed():
    assert len(canon.EXCLUDED_LOSSES) == 8
    assert set(EXCLUDED) == {"writer-bent-l-reversed"}
    trip = Trip("writer-bent-l-reversed", *([EXCLUDED["writer-bent-l-reversed"][0], "REVERSED L", 20.0]))
    reference = trip.reference()
    failures = canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference)).failures
    assert [item for item in failures if "'node')" in item]  # the far-node component is why it is excluded
    assert all("'node')" in item for item in failures)


# --- (a') unbound imports ----------------------------------------------------------------------------
@pytest.mark.parametrize("name", REVERSIBLE)
def test_unbound_import_under_pd1_equals_by_caepipe_name(trips, name):
    trip = trips[name]
    # The shear model is not in the MBF: the caller states the source's (Design §9.4), else Timoshenko applies.
    shear = (trip.model.get("pressure_contract") or {}).get("shear_model")
    result = import_caepipe_mbf(trip.data, material_provenance=INVENTED, pressure_closure=unbound_closure(trip),
                                shear_model=shear, source_label=name)
    assert result.status == "imported", result.diagnostics
    assert any("PD-1" in note for note in result.assumptions)
    reference = trip.reference()
    reference_unoriented, limits = canon.unoriented(reference, trip.limits(reference))
    comparison = canon.compare(reference_unoriented, trip.imported(result, oriented=False), limits)
    assert comparison.failures == []


@pytest.mark.parametrize("name", REVERSIBLE)
def test_unbound_import_under_the_base_rule_gives_the_named_refusal(trips, name, monkeypatch):
    monkeypatch.setattr(importer_module, "PD1_VERSIONS", ())
    trip = trips[name]
    result = import_caepipe_mbf(trip.data, material_provenance=INVENTED, source_label=name)
    assert result.status == "refused" and result.model is None
    refused = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-TBD-17-04-007"]
    assert any("L=" in item["message"] for item in refused)  # the writer always writes L= on the first row
    has_force = any(record["mbf"] in ("FOR", "MOM") for record in trip.writer.load_map.values())
    assert any("without FLC" in item["message"] for item in refused) == has_force
    # With the bound sidecar the writer's conventions apply and the same bytes import.
    assert import_caepipe_mbf(trip.data, sidecar=trip.sidecar, material_provenance=INVENTED,
                              source_label=name).status == "imported"


# --- (b) identities ---------------------------------------------------------------------------------------
@pytest.mark.parametrize("name", REVERSIBLE)
def test_bound_import_restores_the_sidecar_identities_and_reports_the_rest(trips, name):
    trip = trips[name]
    source, imported = trip.model, trip.bound.model
    ids = lambda items: {item["id"] for item in items or []}  # noqa: E731
    exported_nodes = set(trip.writer.node_map)
    assert ids(imported["nodes"]) == exported_nodes
    assert {pipe["id"]: (pipe["from"], pipe["to"]) for pipe in imported["pipe_segments"]} == {
        pipe["id"]: (pipe["from"], pipe["to"]) for pipe in source["pipe_segments"]}
    assert {support["id"]: (support["node"], sorted(support["restraints"]), support.get("family"))
            for support in imported["supports"]} == {
        support["id"]: (support["node"], sorted(canon._DOF_TOKEN[token] for token in support["restraints"]),
                        support.get("family")) for support in source["supports"]}
    assert ids(imported["components"]) == ids(source.get("components"))
    assert ids(imported["materials"]) == {pipe["material"] for pipe in source["pipe_segments"]}
    assert ids(imported["load_cases"]) <= ids(source["load_cases"])
    unrecoverable = " ".join(item["location"] + " " + item["reason"] for item in trip.bound.loss_report
                             if item["code"] == "MBF-IMPORT-ID-UNRECOVERABLE")
    restored = {load["id"] for case in imported["load_cases"] for load in case["primitive_loads"]}
    for case in source["load_cases"]:
        for load in case.get("primitive_loads") or []:
            if load["magnitude"]["value"] == 0:
                continue
            assert load["id"] in restored or "keeps its formed id" in unrecoverable, load["id"]
    for pipe in source["pipe_segments"]:  # straight orientation travels in the sidecar
        if pipe.get("y_reference") is not None and pipe["id"] not in {
                component["geometry"]["bend_pipe_ref"] for component in source.get("components") or []}:
            restored = next(item for item in imported["pipe_segments"] if item["id"] == pipe["id"])
            assert restored["y_reference"] == {axis: float(pipe["y_reference"][axis]) for axis in "xyz"}
    for component in imported["components"]:  # the component's tangent node is not in the sidecar
        assert f"bend {component['label'].split()[-1]}" in unrecoverable
    assert "not export" in unrecoverable


def test_summed_primitives_are_many_to_one_and_reported():
    # REQ-016: two primitives on one node and axis print as one FOR. The bound import keeps the sum under its formed
    # id and reports, by name, the primitive and each sidecar entry it sums (VER-002 r13); the regeneration identity
    # excludes exactly those entries and compares the rest.
    model = writer_tests.cantilever()
    model["load_cases"][0]["primitive_loads"].append(writer_tests.force("tip2", "N1", "GLOBAL_Y", -1000.0))
    trip = Trip("summed", model, "SUMMED", 20.0)
    assert trip.bound.status == "imported", trip.bound.diagnostics
    loads = trip.bound.model["load_cases"][0]["primitive_loads"]
    assert [(load["id"], load["magnitude"]["value"]) for load in loads] == [
        ("load:concentrated_force:N1:global_y", -7000.0)]
    assert groups(trip.bound) == {"case:tip/load:concentrated_force:N1:global_y": ["case:tip/tip", "case:tip/tip2"]}
    assert set_aside(trip.bound) == reported(trip.bound) == differing_entries(trip.sidecar, trip.bound)
    assert regeneration_difference(trip.sidecar, trip.bound) is None
    found = sidecar_difference(trip.sidecar, regenerated_from(trip.bound))  # with no identity set aside
    assert found is not None and found[0] in reported(trip.bound)


def test_a_merged_anchor_split_the_solve_refuses_is_not_restored(monkeypatch):
    # The writer writes two rigid supports at one node, translations T and rotations R, as one KA row. With no
    # family the solve reads R as a guide (fewer than six DOFs), which restrains translations only, so it refuses the
    # source model (SUPPORT_INPUT_INVALID); the bound import refuses the split by name rather than restore it. The
    # writer's total preflight refuses the export for that reason (VER-002 r3); it is bypassed here to reach the import.
    model = writer_tests.cantilever()
    model["supports"] = [{"id": "T", "node": "N0", "restraints": ["UX", "UY", "UZ"], "family": None},
                         {"id": "R", "node": "N0", "restraints": ["RX", "RY", "RZ"]}]
    refused = render_caepipe_mbf_si(model, title="SPLIT")
    assert refused.status == "refused" and {item["code"] for item in refused.diagnostics
                                            if item["severity"] == "error"} == {"MBF-SI-PREFLIGHT-REFUSED"}
    from core.handoff.caepipe_mbf import si_writer
    monkeypatch.setattr(si_writer._Writer, "survives_reimport", lambda self, result: True)
    trip = Trip("split", model, "SPLIT", 20.0)
    assert "KA" in trip.text.split("\r\n")[trip.text.split("\r\n").index("LAYOUT") + 1]
    assert refusals(trip.bound) == {("MBF-IMPORT-SIDECAR-INVALID", "support_map[R]")}
    unbound = import_caepipe_mbf(trip.data, material_provenance=INVENTED, source_label="split")
    assert [(support["id"], support["family"]) for support in unbound.model["supports"]] == [("support:10", "anchor")]
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "split", "model": oriented(model)})
    assert error and "SUPPORT_INPUT_INVALID" in json.dumps(output)


def legacy(result, data: bytes) -> dict:
    """The writer `result`'s sidecar as main's tranche-1 writer recorded it: the seven maps in full, no
    sidecar_format and no pressure_map (it exported no pressure), no y_reference for an unauthored straight, null for
    an unauthored family, no undeclared origin."""
    record = result.to_dict()
    old = {key: deepcopy(record[key]) for key in ("node_map", "element_map", "support_map", "load_case_map",
                                                  "material_map", "section_map", "load_map")}
    for entries in [old["element_map"].values()] + [[entry["caepipe_element"]] for entry in old["load_map"].values()
                                                      if "caepipe_element" in entry]:
        for entry in entries:
            if entry.pop("y_reference_authored", True) is False:
                del entry["y_reference"]
    for entry in old["support_map"].values():
        if entry.pop("family_authored") is False:
            entry["family"] = None
    for entry in old["material_map"].values():
        if entry["origin"] == "undeclared":
            del entry["origin"]
    return {**old, "mbf_sha256": hashlib.sha256(data).hexdigest()}


def test_a_sidecar_without_family_reports_the_family_unrecoverable():
    # Main's earlier writer recorded no family: a legacy sidecar without it restores the support's DOFs and reports
    # the family unrecoverable. A format-2 sidecar records every family, so one without it is refused by path.
    trip = Trip("old-sidecar", writer_tests.cantilever(), "OLD", 20.0)
    sidecar = legacy(trip.writer, trip.data)
    for entry in sidecar["support_map"].values():
        del entry["family"]
    result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label="old")
    assert any(item["code"] == "MBF-IMPORT-ID-UNRECOVERABLE" and "family" in item["reason"]
               for item in result.loss_report)
    current = deepcopy(trip.sidecar)
    support_id = next(iter(current["support_map"]))
    del current["support_map"][support_id]["family"]
    result = import_caepipe_mbf(trip.data, sidecar=current, material_provenance=INVENTED, source_label="current")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", f"support_map[{support_id}].family")}


def regions(model: dict) -> list[tuple]:
    return [(case["id"], region["id"], region["member_pipe_ids"], region["terminals"])
            for case in model["load_cases"] for region in case.get("pressure_regions") or []]


@pytest.mark.parametrize("name", sorted(PRESSURE_MODELS))
def test_bound_import_restores_the_pressure_contract_and_regions(trips, name):
    trip = trips[name]
    imported = trip.bound.model
    assert imported["pressure_contract"] == trip.model["pressure_contract"]
    assert regions(imported) == regions(trip.model)  # ids, member order, terminals, closure and their provenance
    assert all(case.get("pressure_regions") is not None for case in imported["load_cases"])
    assert all(pipe["section"]["corrosion_allowance"]["value"] == 0.0 for pipe in imported["pipe_segments"])


def test_the_shear_model_travels_only_in_the_sidecar():
    # Agent 0's decision under the owner's delegation, 2026-10-27 (Design §9.4): an unbound pressure import takes the
    # contract's default timoshenko_annulus_energy_v1 (owner D-6) or the caller's shear_model, one of the contract's
    # two; a bound import keeps the sidecar's, and a different shear_model with it is refused.
    trip = Trip("euler", writer_tests.pressurized_l(shear="euler_bernoulli"), "EULER", 20.0)
    assert trip.bound.model["pressure_contract"]["shear_model"] == "euler_bernoulli"
    assert "MBF-SI-SHEAR-MODEL-NOT-EXPORTED" in {item["code"] for item in trip.writer.loss_report}
    unbound = lambda **options: import_caepipe_mbf(  # noqa: E731
        trip.data, material_provenance=INVENTED, pressure_closure=CLOSURE, source_label="t", **options)
    default = unbound()
    assert default.model["pressure_contract"]["shear_model"] == "timoshenko_annulus_energy_v1"
    assert any("timoshenko_annulus_energy_v1, the exact contract's default (owner D-6" in note
               for note in default.assumptions)
    assert unbound(shear_model="euler_bernoulli").model["pressure_contract"]["shear_model"] == "euler_bernoulli"
    for bad in ("timoshenko", "", 1, True, ["euler_bernoulli"]):
        result = unbound(shear_model=bad)
        assert refusals(result) == {("MBF-IMPORT-SHEAR-MODEL-INVALID", "shear_model")}, bad
    kept = import_caepipe_mbf(trip.data, sidecar=trip.sidecar, material_provenance=INVENTED, source_label="k",
                              shear_model="euler_bernoulli")
    assert kept.model["pressure_contract"]["shear_model"] == "euler_bernoulli"
    conflict = import_caepipe_mbf(trip.data, sidecar=trip.sidecar, material_provenance=INVENTED, source_label="c",
                                  shear_model="timoshenko_annulus_energy_v1")
    assert refusals(conflict) == {("MBF-IMPORT-SIDECAR-CONFLICT", "shear_model")}


def test_a_pressure_map_that_does_not_match_the_file_is_refused(trips):
    trip = trips["pressure-tee-bar"]
    for change in ("merge", "drop", "terminal"):
        sidecar = deepcopy(trip.sidecar)
        stored = sidecar["pressure_map"]["regions"]
        if change == "merge":  # one region over two legs at a tee: members still pressurized, but owned twice
            stored[1]["members"] = stored[0]["members"] + stored[1]["members"]
        elif change == "drop":  # a pressurized leg in no region
            stored.pop()
        else:
            stored[0]["terminals"][0]["closure_transfer"] = "open"
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label="tamper")
        assert result.status == "refused", change
        expected = (("MBF-IMPORT-SIDECAR-INVALID", "pressure_map.regions[0].terminals[0].closure_transfer")
                    if change == "terminal" else ("MBF-IMPORT-SIDECAR-CONFLICT", "pressure_map"))
        assert expected in refusals(result), change


def test_the_pressure_bound_is_half_a_unit_of_the_printed_bar_value(trips):
    # 150 psi = 150 x 4.4482216152605 / 0.00064516 Pa = 1034213.5939752541... Pa, printed as 10.34213594 bar.
    # delta(10.34213594) = 0.5e-8 bar = 5e-4 Pa, plus 4 eps |p|; the import reads 1034213.594 Pa.
    trip = trips["pressure-us-psi"]
    lines = trip.text.split("\r\n")
    assert lines[lines.index("LOADS") + 1] == "L1,120,10.34213594,0"  # T1 = 20 C + 180 degF x 5/9
    reference = trip.reference()
    key = next(key for key in reference.values if key[0] == "pressure")
    assert reference.values[key] == float(150 * Fraction("4.4482216152605") / Fraction("0.00064516"))
    imported = trip.imported(trip.bound, oriented=True)
    assert imported.values[key] == 1034213.594
    limit = trip.limits(reference)[key]
    assert limit == pytest.approx(5e-4 + 4 * canon.EPS * 10.34213594 * 1e5, rel=1e-12)
    assert abs(imported.values[key] - reference.values[key]) < limit


def test_z_vertical_positions_are_the_exact_rotation_of_the_printed_coordinates():
    # The test's own exact sum of the printed offsets, rotated (X, Y, Z) -> (X, Z, -Y) and rounded once, equals
    # every imported position bit for bit.
    lines = ("INVENTED ROTATION", "VERSION", "15.10", "OPTIONS", "HGRA,SI,Z", "OPTEMP", "20", "MATERIAL",
             "M1,0,0.3,1,1,CS,invented", "M1,20,200000,1.2e-05", "PIPE", "P1,168.3,7.11,0,0,0,0,0,0", "LOADS",
             "L1,20,0,0", "LAYOUT", "F10,KA,MM1,PP1,X0.1,Y-0.2,Z0.3,CL=L1", "T20,X1000.1,Y2000.2,Z-3000.3",
             "T30,X0.7,Y0.1,Z0.2", "T40,KA,Y-1e-3")
    text = "".join(line + "\r\n" for line in lines)
    result = import_caepipe_mbf(text.encode("ascii"), material_provenance=INVENTED, source_label="rotation")
    assert result.status == "imported", result.diagnostics
    expected = {name: tuple(float(value) for value in canon.rotate_z_vertical(position))
                for name, position in canon.printed_positions(text).items()}
    assert {node["label"]: tuple(node["position"].values()) for node in result.model["nodes"]} == expected
    assert expected["20"] == (1000.2, -3000.0, -2000.0)  # (0.1 + 1000.1, 0.3 - 3000.3, -(-0.2 + 2000.2))


# --- (c) byte reproduction ---------------------------------------------------------------------------
def collisions(original: str, rewritten: str | None) -> list[str]:
    """Every field of W that writer(I) prints differently, by line and field."""
    if rewritten is None:
        return ["writer(I) refused"]
    a, b = original.split("\r\n"), rewritten.split("\r\n")
    found = [f"line count {len(a)} != {len(b)}"] if len(a) != len(b) else []
    for number, (left, right) in enumerate(zip(a, b), start=1):
        for index, (x, y) in enumerate(zip(left.split(","), right.split(",")), start=1):
            if x != y:
                found.append(f"line {number} field {index}: {x!r} -> {y!r}")
        if left.count(",") != right.count(","):
            found.append(f"line {number}: field count differs")
    return found


@pytest.mark.parametrize("name", REVERSIBLE)
def test_byte_reproducible_models_are_written_again_byte_for_byte(trips, name):
    trip = trips[name]
    imported = trip.bound.model
    t_ref = float(canon.read_printed(trip.text).t_ref)  # the imported OPTEMP value; the writer takes it as an argument
    again = render_caepipe_mbf_si(imported, title=imported["project"]["name"], reference_temperature_c=t_ref,
                                  max_layout_rows=20)
    found = collisions(trip.text, again.text)
    if name in BYTE_REPRODUCIBLE:
        assert found == [], found
        assert again.text.encode("ascii") == trip.data
    else:
        assert found  # reported, never absorbed


def test_a_formatting_collision_is_reported_by_line_and_field():
    assert collisions("A,1\r\nB,2.5", "A,1\r\nB,2.6") == ["line 2 field 2: '2.5' -> '2.6'"]
    assert collisions("A", None) == ["writer(I) refused"]


# --- (d) solve equality ---------------------------------------------------------------------------------
SI_RESULT = {"m": 1.0, "mm": 1e-3, "rad": 1.0, "deg": math.pi / 180.0, "N": 1.0, "kN": 1e3, "N*m": 1.0, "kN*m": 1e3}
FAMILIES = {"global_nodal_displacement": "translation", "global_nodal_rotation": "rotation"}


def result_table(output) -> dict[str, dict[tuple, float]]:
    families: dict[str, dict[tuple, float]] = {}
    for row in output["payload"]["mechanics_envelope"]["results"]:
        kind = row.get("kind", "")
        if kind.startswith("global_nodal_displacement_") or kind.startswith("global_nodal_rotation_"):
            family = FAMILIES[kind.rsplit("_", 1)[0]]
        elif kind == "support_reaction_component_v2":
            family = "reaction_" + (row.get("metadata") or {}).get("component", "?")[0]
        else:
            continue
        key = (kind, row.get("entity_ref"), (row.get("basis_ref") or {}).get("ref_id"),
               (row.get("metadata") or {}).get("component"))
        families.setdefault(family, {})[key] = row["value"] * SI_RESULT[row["unit"]]
    return families


def runner_binary() -> str:
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner or not Path(runner).is_file():
        pytest.skip("the headless runner (OPENPIPESTRESS_RUNNER_BIN) is not available in this session")
    return runner


def oriented(model: dict) -> dict:
    """The source model A as the solve accepts it: an explicit y_reference on every pipe that lacks one (global Y,
    or global X for a pipe along Y) and a provenance string on every record that lacks one; the writer-test models
    omit both. The import needs no such help: it assigns the documented local y (MBF-IMPORT-ORIENTATION-DEFAULTED)."""
    model = deepcopy(model)
    records = [*model["nodes"], *model["pipe_segments"], *model["supports"], *(model.get("materials") or []),
               *(model.get("components") or []), *(model.get("load_cases") or []),
               *(load for case in model.get("load_cases") or [] for load in case.get("primitive_loads") or [])]
    for record in records:
        record.setdefault("provenance", "invented round-trip test model; not engineering data")
    nodes = {node["id"]: node["position"] for node in model["nodes"]}
    for pipe in model["pipe_segments"]:
        if pipe.get("y_reference") is None:
            a, b = nodes[pipe["from"]], nodes[pipe["to"]]
            along_y = a["x"] == b["x"] and a["z"] == b["z"]
            pipe["y_reference"] = {"x": 1.0, "y": 0.0, "z": 0.0} if along_y else {"x": 0.0, "y": 1.0, "z": 0.0}
    return model


@pytest.mark.parametrize("name", REVERSIBLE)
def test_solves_of_source_and_bound_import_agree_within_the_provisional_threshold(trips, name):
    trip = trips[name]
    reference = trip.reference()
    r_in = canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference)).r_in
    if name in SOLVE_EXCLUDED:
        pytest.skip(f"{name} is excluded from solve equality: {SOLVE_EXCLUDED[name]}")
    if r_in > SOLVE_R_IN_LIMIT:
        pytest.skip(f"{name} is excluded from solve equality: r_in = {r_in:.3g} > {SOLVE_R_IN_LIMIT}")
    runner = runner_binary()
    _, source, error = solve_swbpipe(runner, {"case_id": name, "model": oriented(trip.model)})
    assert error == ""
    _, imported, error = solve_swbpipe(runner, {"case_id": name, "model": trip.bound.model})  # as imported
    assert error == ""
    a, b = result_table(source), result_table(imported)
    assert set(a) == set(b)
    for family, rows in a.items():
        assert set(rows) == set(b[family]), family
        scale = max(abs(value) for value in rows.values())
        threshold = SOLVE_THRESHOLD_FACTOR * r_in * scale
        worst = max(abs(rows[key] - b[family][key]) for key in rows)
        assert worst <= threshold, (family, worst, threshold)


def test_the_bend_boundary_case_is_excluded_from_solve_equality(trips):
    trip = trips["bend-boundary"]
    reference = trip.reference()
    assert canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference)).r_in > SOLVE_R_IN_LIMIT


def test_the_runtime_admits_every_region_the_importer_forms():
    # Judged by the exact route itself (VER-002 r1 and r2): Codex's run whose first two straights fail alone, the
    # zigzag run, and a corner each import and solve; the drifting run the importer refuses is refused by the route
    # too (PRESSURE_REGION_NONCOLLINEAR) when authored as one region.
    runner = runner_binary()
    files = {"nonmonotonic": ("F10,KA,MM1,PP1,CL=L1", *importer_tests.NONMONOTONIC),
             "zigzag": ("F10,KA,MM1,PP1,CL=L1", *importer_tests.ZIGZAG),
             "corner": ("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KA,X1000,Y2"),
             "reversed": importer_tests.REVERSED}  # VER-002 r3: emitted 30 -> 10, the order the route admits
    for name, rows in files.items():
        result = importer_tests.run(importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",)),
                                    pressure_closure=importer_tests.CLOSURE)
        assert result.status == "imported", result.diagnostics
        _, _, error = solve_swbpipe(runner, {"case_id": name, "model": result.model})
        assert error == "", name
    drift = importer_tests.run(importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", *importer_tests.DRIFT)).model
    drift["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
    for pipe in drift["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
    drift["load_cases"] = [{"id": "case:p", "label": "Sustained (W+P)", "kind": "sustained", "status": "draft",
                            "primitive_loads": [], "provenance": "invented", "pressure_regions": [
        {"id": "region:drift", "member_pipe_ids": [pipe["id"] for pipe in drift["pipe_segments"]],
         "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 10.0, "unit": "bar"},
         "provenance": "invented", "terminals": [
             {"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": "invented"}
             for node in ("node:10", "node:170")]}]}]
    _, output, error = solve_swbpipe(runner, {"case_id": "drift-one-region", "model": drift})
    assert error and "PRESSURE_REGION_NONCOLLINEAR" in json.dumps(output)
    # The reversed run as the file orders it (10 -> 30) is refused by the route; and Codex's equal-decimal bores
    # (VER-002 r3), which the importer refuses, are refused by the route as one region too.
    for name, rows, pipes, code in (
            ("reversed-file-order", importer_tests.REVERSED, ("P1,168.3,7.11,0",), "PRESSURE_REGION_NONCOLLINEAR"),
            ("bores", importer_tests.BORE_ROWS, importer_tests.BORE_PIPES, "PRESSURE_REGION_BORE_MISMATCH")):
        model = importer_tests.run(importer_tests.mbf(*rows, pipe=pipes)).model
        model["pressure_contract"] = drift["pressure_contract"]
        for pipe in model["pipe_segments"]:
            pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
        region = deepcopy(drift["load_cases"][0])
        region["pressure_regions"][0]["member_pipe_ids"] = [pipe["id"] for pipe in model["pipe_segments"]]
        for terminal, node in zip(region["pressure_regions"][0]["terminals"], ("node:10", "node:30")):
            terminal["node_ref"] = node
        model["load_cases"] = [region]
        _, output, error = solve_swbpipe(runner, {"case_id": name, "model": model})
        assert error and code in json.dumps(output), name


def test_an_import_whose_allowance_is_deleted_is_not_round_tripped_with_an_authored_zero():
    # Codex VER-002 r2 item 2: import a pressure MBF with P1,168.3,7.11,0, delete the allowance, export.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X3000", pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",))
    imported = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    assert imported.status == "imported", imported.diagnostics
    model = deepcopy(imported.model)
    del model["pipe_segments"][0]["section"]["corrosion_allowance"]
    written = render_caepipe_mbf_si(model, title="NO ALLOWANCE")
    assert written.status == "refused" and written.text is None
    assert {item["code"] for item in written.diagnostics if item["severity"] == "error"} == {
        "MBF-SI-CORROSION-ALLOWANCE-MISSING"}
    assert render_caepipe_mbf_si(imported.model, title="WITH ALLOWANCE").status == "written"


# VER-002 r4: two regions the model's values admit but the writer's 10-digit print would not. (a) Bores: 168.30000004
# - 2 x 7.11000002 = 154.08 = 170.3 - 2 x 8.11, and in m both radii are 0.07704; printed, P1 becomes 168.3, 7.11000002.
# (b) Geometry: offsets (1000.0000004, 2000.0000008) then (1999.9999996, 3999.9999992), collinear as read; printed to
# 10 digits, the run is not.
FORMAT_CASES = {
    "bores": (("P1,168.30000004,7.11000002,0", "P2,170.3,8.11,0"),
              ("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KA,X1000,PP2"), "MBF-IMPORT-PRESSURE-BORE-MISMATCH"),
    "geometry": (("P1,168.3,7.11,0",), ("F10,KA,MM1,PP1,CL=L1", "T20,X1000.0000004,Y2000.0000008",
                                        "T30,KA,X1999.9999996,Y3999.9999992"), "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR"),
}


def format_case(name: str):
    pipes, rows, _ = FORMAT_CASES[name]
    result = importer_tests.run(importer_tests.mbf(*rows, pipe=pipes, loads=("L1,20,10,0",)),
                                pressure_closure=importer_tests.CLOSURE)
    assert result.status == "imported", result.diagnostics
    assert [region["id"] for region in result.model["load_cases"][0]["pressure_regions"]] == ["region:mbf-10-30"]
    return result.model


@pytest.mark.parametrize("name", sorted(FORMAT_CASES))
def test_the_writer_refuses_a_region_its_printed_values_would_break(name, monkeypatch):
    model = format_case(name)
    written = render_caepipe_mbf_si(model, title="FORMAT")
    assert written.status == "refused" and written.text is None
    errors = [item for item in written.diagnostics if item["severity"] == "error"]
    assert {(item["code"], item["entity"]) for item in errors} == {
        ("MBF-SI-PRESSURE-REGION-FORMAT", "region:region:mbf-10-30")}
    assert any(item["code"] == "MBF-SI-PRESSURE-REGION-FORMAT" for item in written.loss_report)
    # Without the preflight, the bound import of the printed file refuses the restored region by name, in its
    # declared order, without cutting it.
    from core.handoff.caepipe_mbf import si_writer
    monkeypatch.setattr(si_writer._Writer, "survives_reimport", lambda self, result: True)
    unchecked = render_caepipe_mbf_si(model, title="FORMAT")
    data = unchecked.text.encode("ascii")
    sidecar = unchecked.sidecar(hashlib.sha256(data).hexdigest())
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=name)
    assert bound.status == "refused" and bound.source["binding"] == "bound"
    assert {(item["code"], item["location"]) for item in bound.diagnostics if item["severity"] == "error"} == {
        (FORMAT_CASES[name][2], "pressure_map region region:mbf-10-30")}


def test_the_runtime_solves_the_format_cases_as_imported():
    runner = runner_binary()
    for name in FORMAT_CASES:
        _, _, error = solve_swbpipe(runner, {"case_id": name, "model": format_case(name)})
        assert error == "", name


# --- VER-002 r5: the closed guard set ----------------------------------------------------------------------------------
TEE_ROWS = ("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KA,X1000", "F20", "T40,KA,Y1000")


def bound_parts(model: dict, monkeypatch=None):
    """The writer's text and byte-bound sidecar for `model`, with the preflight bypassed when `monkeypatch` is given."""
    if monkeypatch is not None:
        from core.handoff.caepipe_mbf import si_writer
        monkeypatch.setattr(si_writer._Writer, "survives_reimport", lambda self, result: True)
    written = render_caepipe_mbf_si(model, title="GUARD")
    assert written.status == "written", written.diagnostics
    data = written.text.encode("ascii")
    return data, written.sidecar(hashlib.sha256(data).hexdigest())


def complete_sidecar(data: bytes, model: dict, id_map: dict, **maps) -> dict:
    """A complete format-2 byte-bound sidecar for `data`, from an import (`model`, `id_map`) of the same topology, as
    the writer would record it, each field with its effective value; `maps` replace whole maps. Every straight's
    y_reference is recorded as authored."""
    names = {swb_id: name for name, swb_id in id_map["node_map"].items()}
    bends = {component["geometry"]["bend_pipe_ref"]: component for component in model.get("components", [])}

    def element(pipe: dict) -> dict:
        entry = {"from": names[pipe["from"]], "to": names[pipe["to"]], "reversed": False}
        if pipe["id"] in bends:
            return {**entry, "bend": {"component": bends[pipe["id"]]["id"]}}
        return {**entry, "y_reference": pipe["y_reference"], "y_reference_authored": True}

    def target(found: dict) -> dict:
        return ({"pipe": found["pipe"]} if "pipe" in found else {"node": found["node"]} if "node" in found
                else {"support": found["support"], "dof": found["dof"]})

    pressure = {"contract": "none", "shear_model": "euler_bernoulli", "regions": []}
    if model.get("pressure_contract"):
        regions = next((case["pressure_regions"] for case in model["load_cases"] if case["pressure_regions"]), [])
        pressure = {"contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
                    "shear_model": model["pressure_contract"]["shear_model"], "regions": [
            {"id": region["id"], "members": region["member_pipe_ids"], "terminals": region["terminals"]}
            for region in regions]}
    sidecar = {
        "sidecar_format": 2,
        "node_map": dict(sorted((swb_id, name) for name, swb_id in id_map["node_map"].items())),
        "element_map": {pipe["id"]: element(pipe) for pipe in model["pipe_segments"]},
        "support_map": {support["id"]: {"node": names[support["node"]], "restrained": support["restraints"],
                                        "family": support.get("family") or (
                                            "anchor" if len(support["restraints"]) == 6 else "guide"),
                                        "family_authored": support.get("family") is not None}
                        for support in model["supports"]},
        "load_case_map": {case["id"]: case["label"] for case in model["load_cases"]},
        "material_map": {name: {"swbpipe_material": swb_id, "origin": "undeclared"}
                         for name, swb_id in id_map["material_map"].items()},
        "load_map": {f"{case['id']}/{item['id']}": {"load_case": case["id"], "category": item["category"],
                                                    "target": target(item["target"]), "direction": item["direction"]}
                     for case in model["load_cases"] for item in case["primitive_loads"]},
        "pressure_map": pressure,
        "mbf_sha256": hashlib.sha256(data).hexdigest(),
    }
    sidecar.update(maps)
    return sidecar


def refusals(result) -> set[tuple[str, str]]:
    return {(item["code"], item["location"]) for item in result.diagnostics if item["severity"] == "error"}


def regenerated_from(result) -> dict:
    """What the writer regenerates from the imported model of `result` (and its record of derived orientations)."""
    regenerated, errors = regenerate_caepipe_sidecar(result.model, mbf_sha256="0" * 64,
                                                     derived_y_reference=result.id_map["y_reference_derived"])
    assert regenerated is not None, errors
    return regenerated


def groups(result) -> dict[str, list[str]]:
    """The primitives `result`'s own import formed from several sidecar entries (REQ-016): regenerated key -> keys."""
    return summed_entries(result.id_map)


def reported(result) -> set[str]:
    """The load_map paths `result`'s import reported by name as many-to-one (MBF-IMPORT-ID-UNRECOVERABLE)."""
    return {item["location"] for item in result.loss_report
            if item["code"] == "MBF-IMPORT-ID-UNRECOVERABLE" and item["location"].startswith("load_map[")}


def set_aside(result) -> set[str]:
    """The entries whose identity only the regeneration identity sets aside: each summed primitive's regenerated
    entry and the sidecar entries it sums."""
    return {f"load_map[{key}]" for formed, keys in groups(result).items() for key in (formed, *keys)}


def regeneration_difference(sidecar: dict, result) -> tuple[str, str, str] | None:
    """The regeneration identity alone (Design §14): `sidecar` against what the writer regenerates from the imported
    model of `result`, the summed primitives' identities aside; None when they are equal."""
    return sidecar_difference(sidecar, regenerated_from(result), summed=groups(result))


def differing_entries(sidecar: dict, result) -> set[str]:
    """Every load_map entry at which `sidecar` and the regeneration from `result` differ, with nothing excluded;
    every other part of the two must be equal."""
    given, again = canonical_sidecar(sidecar), canonical_sidecar(regenerated_from(result))
    assert {key: value for key, value in given.items() if key != "load_map"} == {
        key: value for key, value in again.items() if key != "load_map"}
    return {f"load_map[{key}]" for key in set(given["load_map"]) | set(again["load_map"])
            if given["load_map"].get(key) != again["load_map"].get(key)}


def test_a_restored_region_through_a_tee_is_refused_by_name_and_by_the_route():
    # Codex's case: the tee imports as three regions; a sidecar joining 10-20-30 passes through 20, where 20-40 joins.
    model = importer_tests.run(importer_tests.mbf(*TEE_ROWS, loads=("L1,20,10,0",)),
                               pressure_closure=importer_tests.CLOSURE).model
    assert [region["id"] for region in model["load_cases"][0]["pressure_regions"]] == [
        "region:mbf-10-20", "region:mbf-20-30", "region:mbf-20-40"]
    data, sidecar = bound_parts(model)
    stored = sidecar["pressure_map"]["regions"]
    stored[0]["members"] = stored[0]["members"] + stored[1]["members"]
    stored[0]["terminals"][1] = stored[1]["terminals"][1]
    del stored[1]
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="tee")
    assert refusals(bound) == {("MBF-IMPORT-PRESSURE-INTERNAL-BRANCH", "pressure_map region region:mbf-10-20")}
    joined = deepcopy(model)  # the same region authored in the model: the writer refuses it, and so does the route
    regions = joined["load_cases"][0]["pressure_regions"]
    regions[0]["member_pipe_ids"] += regions[1]["member_pipe_ids"]
    regions[0]["terminals"][1] = regions[1]["terminals"][1]
    del regions[1]
    written = render_caepipe_mbf_si(joined, title="TEE")
    assert written.status == "refused"
    assert {item["code"] for item in written.diagnostics if item["severity"] == "error"} == {"MBF-SI-PREFLIGHT-REFUSED"}
    assert "MBF-IMPORT-PRESSURE-INTERNAL-BRANCH" in " ".join(item["message"] for item in written.diagnostics)
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "tee-joined", "model": joined})
    assert error and "PRESSURE_REGION_INTERNAL_BRANCH" in json.dumps(output)


def test_the_preflight_refuses_any_failed_reimport_with_its_code():
    # Codex's case: OD 100.00000004 and wall 50 import (2t < OD) and solve; printed, the OD is 100 = 2t.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", pipe=("P1,100.00000004,50,0",),
                              loads=("L1,20,10,0",))
    model = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE).model
    _, _, error = solve_swbpipe(runner_binary(), {"case_id": "thin-od", "model": model})
    assert error == ""
    written = render_caepipe_mbf_si(model, title="THIN")
    assert written.status == "refused" and written.text is None
    errors = [item for item in written.diagnostics if item["severity"] == "error"]
    assert {item["code"] for item in errors} == {"MBF-SI-PREFLIGHT-REFUSED"}
    assert any("MBF-IMPORT-VALUE-OUT-OF-RANGE" in item["message"] for item in errors)


def renamed(model: dict, names: dict[str, str]) -> dict:
    text = json.dumps(model)
    for old, new in names.items():
        text = text.replace(f'"{old}"', f'"{new}"')
    return json.loads(text)


def test_ids_equal_once_colons_read_as_hyphens_are_refused(monkeypatch):
    # The route's check_suffixes reads ':' as '-' per kind (EXACT_PRESSURE_RESULT_ID_COLLISION): pipes P:1 and P-1.
    model = renamed(writer_tests.pressurized_l(), {"S1": "P:1", "S2": "P-1"})
    written = render_caepipe_mbf_si(model, title="IDS")
    assert written.status == "refused"
    assert "MBF-IMPORT-PRESSURE-ID-COLLISION" in " ".join(item["message"] for item in written.diagnostics)
    data, sidecar = bound_parts(model, monkeypatch)
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="ids")
    assert refusals(bound) == {("MBF-IMPORT-PRESSURE-ID-COLLISION", "pipe P:1, P-1")}


def test_restored_region_ids_must_be_present_and_unique(trips):
    trip = trips["pressure-tee-bar"]
    for change in ("empty", "duplicate"):
        sidecar = deepcopy(trip.sidecar)
        stored = sidecar["pressure_map"]["regions"]
        stored[1]["id"] = " " if change == "empty" else stored[0]["id"]
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label=change)
        assert result.status == "refused"
        expected = (("MBF-IMPORT-SIDECAR-INVALID", "pressure_map.regions[1].id") if change == "empty"
                    else ("MBF-IMPORT-SIDECAR-CONFLICT", "pressure_map"))
        assert expected in refusals(result), change


def test_a_restored_orientation_the_frame_cannot_use_is_refused():
    # frame_kernel: the y_reference's part perpendicular to the axis must exceed 1e-12; one along the axis has none.
    # Refused by name with the element, nothing substituted.
    trip = Trip("parallel", writer_tests.cantilever(), "PARALLEL", 20.0)
    sidecar = deepcopy(trip.sidecar)
    sidecar["element_map"]["S1"]["y_reference"] = {"x": 2.0, "y": 0.0, "z": 0.0}  # S1 runs along +x
    result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label="parallel")
    assert refusals(result) == {("MBF-IMPORT-ORIENTATION-UNUSABLE", "S1")}
    assert importer_module.frame_y_reference_usable((0, 0, 0), (3, 0, 0), (0.0, 1e-12, 0.0)) is False
    assert importer_module.frame_y_reference_usable((0, 0, 0), (3, 0, 0), (0.0, 2e-12, 0.0)) is True


# --- (e) determinism ----------------------------------------------------------------------------------
@pytest.mark.parametrize("name", REVERSIBLE)
def test_importing_twice_gives_identical_bytes(trips, name):
    trip = trips[name]
    again = import_caepipe_mbf(trip.data, sidecar=deepcopy(trip.sidecar), material_provenance=dict(INVENTED),
                               source_label=name)
    assert again.model_bytes() == trip.bound.model_bytes()
    assert again.report_bytes() == trip.bound.report_bytes()
    assert again.id_map_bytes() == trip.bound.id_map_bytes()


# --- bend boundary case (§18.1 item 5) ---------------------------------------------------------------------
def test_bend_boundary_residuals_are_covered_only_with_the_residual_term(trips):
    trip = trips["bend-boundary"]
    assert trip.writer.diagnostics == [] or all(item["severity"] != "error" for item in trip.writer.diagnostics)
    residuals = canon.bend_residuals(trip.model, "BEND")
    assert residuals["tangent_near"] == pytest.approx(8.991e-10, rel=1e-3)  # 8.991e-7 mm in m
    reference = trip.reference()
    imported = trip.imported(trip.bound, oriented=True)
    near = trip.writer.node_map["N"]
    shift = abs(imported.values[("node", near, 0)] - reference.values[("node", near, 0)])
    assert 8e-10 < shift < 1e-9  # about 9e-7 mm
    assert canon.compare(reference, imported, trip.limits(reference)).failures == []
    without = canon.compare(reference, imported, trip.limits(reference, residuals=False)).failures
    assert any(f"'{near}', 0)" in item for item in without)


def test_bend_plane_compares_by_projection_perpendicular_to_the_chord(trips):
    # The invented y_reference (2, 0, 0) on the L bend reconstructs as (1, 0, -1)/sqrt(2): 45 degrees apart raw,
    # the same projected direction.
    model = writer_tests.bent_l(y_reference=(2.0, 0.0, 0.0))
    trip = Trip("projection", model, "PROJECTION", 20.0)
    plane = trip.bound.model["pipe_segments"][1]["y_reference"]
    assert (plane["x"], plane["y"], plane["z"]) == pytest.approx((math.sqrt(0.5), 0.0, -math.sqrt(0.5)), abs=1e-15)
    reference = trip.reference()
    assert canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference)).failures == []


# --- tolerance regressions with hand-derived values ----------------------------------------------------------
def test_delta_is_half_a_unit_in_the_tenth_significant_digit():
    assert canon.delta("0") == 0.0
    assert canon.delta("20.12345679") == pytest.approx(5e-9, rel=1e-12)  # floor(log10 20.1) = 1: 0.5e-8
    assert canon.delta("3000") == pytest.approx(5e-7, rel=1e-12)
    assert canon.delta("1e-05") == pytest.approx(5e-15, rel=1e-12)
    assert canon.delta("100") == pytest.approx(5e-8, rel=1e-12)  # exactly a power of ten
    assert canon.delta("-0.5729577951") == pytest.approx(5e-11, rel=1e-12)


def test_small_delta_t_has_an_absolute_bound_only():
    # Design §18.2 example: dT 0.123456789 C at 20 C prints T1 = 20.12345679; the import returns about 1e-9 C off.
    model = writer_tests.cantilever()
    model["load_cases"].append({"id": "case:th", "primitive_loads": [
        {"id": "dt", "category": "thermal", "target": {"type": "element", "pipe": "S1"}, "direction": "global_x",
         "magnitude": writer_tests.q(0.123456789, "degC"), "dimension": "temperature_interval"}]})
    trip = Trip("small-dt", model, "SMALL DT", 20.0)
    assert "L1,20.12345679,0,0" in trip.text.split("\r\n")
    reference = trip.reference()
    imported = trip.imported(trip.bound, oriented=True)
    key = ("load", "expansion", "thermal", ("10", "20"))
    error = abs(imported.values[key] - reference.values[key])
    assert error == pytest.approx(1e-9, rel=0.05)
    assert error / 0.123456789 > 5e-9  # about 8e-9 relative: a relative bound would be meaningless here
    # delta(20.12345679) = 5e-9 and delta(20) = 0.5e-8 by the 10-digit formula (§18.2), plus 4 eps |T1|.
    assert trip.limits(reference)[key] == pytest.approx(5e-9 + 5e-9 + 4 * canon.EPS * 20.12345679, rel=1e-12)
    assert canon.compare(reference, imported, trip.limits(reference)).failures == []


def test_weight_per_length_bound_is_the_documented_relative_sum():
    trip = Trip("weight", weighted(), "WEIGHT", 20.0)
    reference = trip.reference()
    key = ("load", "sustained", "weight", ("10", "20"))
    od, t = 168.3, 7.11
    area = math.pi / 4 * (od**2 - (od - 2 * t) ** 2)
    expected = (canon.delta("7850") / 7850 + (math.pi * t * 5e-8 + math.pi * (od - 2 * t) * 5e-10) / area
                + 8 * canon.EPS) * abs(reference.values[key])
    assert trip.limits(reference)[key] == pytest.approx(expected, rel=1e-12)


def test_g_bound_adds_the_writer_agreement_threshold_when_both_were_authored():
    trip = Trip("nu-and-g", both_given_material(), "NU AND G", 20.0)
    reference = trip.reference()
    key = ("pipe", ("10", "20"), "G")
    expected = (1e-9 + canon.delta("200000") / 200000 + canon.delta("0.25") / 1.25 + 8 * canon.EPS) * 8.0e10
    assert trip.limits(reference)[key] == pytest.approx(expected, rel=1e-12)


# --- units check, independent of the writer (§18.1 item 4) -----------------------------------------------------
INCH = Fraction("0.0254")
LBF = Fraction("4.4482216152605")
POUND = Fraction("0.45359237")


def own_length(text: str) -> Fraction:
    """The test's own evaluation of the two feet-inch forms in the English fixture."""
    text = text.replace("\u2019", "'").replace("\u201d", '"').rstrip('"')
    feet, _, rest = text.partition("'")
    inches, _, fraction = rest.partition("-")
    value = Fraction(int(feet) * 12 + int(inches))
    if fraction:
        numerator, denominator = fraction.split("/")
        value += Fraction(int(numerator), int(denominator))
    return value


def fixture(stem: str):
    path = FIXTURES / f"{stem}.mbf"
    declaration = json.loads((FIXTURES / f"{stem}.provenance.json").read_text(encoding="utf-8"))
    return path.read_bytes(), declaration


def test_si_fixture_values_are_the_exact_conversions_of_the_english_fixture():
    english = fixture("units_english")[0].decode("utf-8").splitlines()
    si = fixture("units_si")[0].decode("ascii").splitlines()

    def close(printed: str, exact: Fraction) -> bool:
        decimals = len(printed.partition(".")[2].lower().partition("e")[0])
        exponent = int(printed.lower().partition("e")[2] or 0)
        return abs(Fraction(printed) - exact) <= Fraction(1, 2) * Fraction(10) ** (exponent - decimals)

    e_mat, s_mat = english[english.index("MATERIAL") + 1].split(","), si[si.index("MATERIAL") + 1].split(",")
    assert close(s_mat[1], Fraction(e_mat[1]) * POUND / INCH**3)  # lbf/in3 -> kgf/m3
    e_row, s_row = english[english.index("MATERIAL") + 2].split(","), si[si.index("MATERIAL") + 2].split(",")
    assert close(s_row[2], Fraction(e_row[2]) * LBF / INCH**2 / 10**6)  # psi -> N/mm2
    assert close(s_row[3], Fraction(e_row[3]) * Fraction(9, 5))  # 1/degF -> 1/degC
    assert close(s_row[1], (Fraction(e_row[1]) - 32) * Fraction(5, 9))
    e_pipe, s_pipe = english[english.index("PIPE") + 1].split(","), si[si.index("PIPE") + 1].split(",")
    for index in (1, 2):
        assert close(s_pipe[index], Fraction(e_pipe[index]) * 254 / 10)  # in -> mm
    e_layout, s_layout = english[english.index("LAYOUT") + 3:], si[si.index("LAYOUT") + 3:]
    assert close(s_layout[0].split(",")[1][1:], own_length(e_layout[0].split(",")[1][1:]) * 254 / 10)
    assert close(s_layout[1].split(",")[1][1:], own_length(e_layout[1].split(",")[1][1:]) * 254 / 10)
    assert close("4448.2216152605", 1000 * LBF) and "FOR(0,-4448.2216152605,0)" in s_layout[0]
    assert close("677.9089741657002", 500 * Fraction("0.3048") * LBF) and "MOM(0,0,677.9089741657002)" in s_layout[0]
    assert "D(0,12.7,0)" in s_layout[1] and "D(0,0.5,0)" in e_layout[1]


def test_english_positions_equal_the_tests_own_rational_evaluation():
    data, declaration = fixture("units_english")
    result = import_caepipe_mbf(data, material_provenance=declaration["material_provenance"], source_label="english")
    assert result.status == "imported", result.diagnostics
    x = own_length("10'6\"")
    z = own_length("7\u20193-1/4\u201d")
    assert (x, z) == (Fraction(126), Fraction(349, 4))
    assert [tuple(node["position"].values()) for node in result.model["nodes"]] == [
        (0.0, 0.0, 0.0), (float(x), 0.0, 0.0), (float(x), 0.0, float(z))]
    assert result.model["project"]["units"]["length"] == "in"


def test_english_and_si_imports_of_one_model_agree():
    imports = {}
    for stem in ("units_english", "units_si"):
        data, declaration = fixture(stem)
        imports[stem] = import_caepipe_mbf(data, material_provenance=declaration["material_provenance"], source_label=stem)
        assert imports[stem].status == "imported", imports[stem].diagnostics
    forms = {}
    for stem, result in imports.items():
        names = {swb_id: name for name, swb_id in result.id_map["node_map"].items()}
        kinds = {case["id"]: "sustained" if case["kind"] == "sustained" else "expansion"
                 for case in result.model["load_cases"]}
        forms[stem] = canon.canonical(result.model, names, kinds, oriented=True)
    english, si = forms["units_english"], forms["units_si"]
    assert set(english.values) == set(si.values) and english.sets == si.sets
    for key, value in english.values.items():
        # The SI fixture prints non-terminating conversions to 15 significant digits: 5e-15 relative, plus the
        # binary64 rounding of each side.
        assert si.values[key] == pytest.approx(value, rel=5e-15 + 8 * canon.EPS, abs=1e-300), key
    runner = runner_binary()
    tables = []
    for stem, result in imports.items():
        _, output, error = solve_swbpipe(runner, {"case_id": stem, "model": result.model})
        assert error == ""
        tables.append(result_table(output))
    for family, rows in tables[0].items():
        scale = max(abs(value) for value in rows.values())
        assert set(rows) == set(tables[1][family])
        assert max(abs(rows[key] - tables[1][family][key]) for key in rows) <= 1e-9 * scale, family


def test_an_english_import_round_trips_through_the_si_writer_by_exact_normalization():
    # O-2: the SI writer normalizes the English import by exact definitions; importing its MBF again (bound)
    # reproduces the English import within the Design's tolerances.
    data, declaration = fixture("units_english")
    english = import_caepipe_mbf(data, material_provenance=declaration["material_provenance"], source_label="english")
    assert any("exact normalization" in note for note in english.assumptions)
    trip = Trip("english-round-trip", english.model, "ENGLISH", 20.0)
    normalized = [item for item in trip.writer.diagnostics if item["code"] == "MBF-SI-UNITS-NORMALIZED"]
    assert normalized and all(text in normalized[0]["message"] for text in ("1 in = 25.4 mm", "1 psi", "1 lbf*ft",
                                                                           "1/degF = 9/5 per degC", "1 lbf/in"))
    lines = trip.text.split("\r\n")
    assert "T20,X3200.4,CFOR(0,-4448.221615,0),MOM(0,0,677.9089742)" in lines  # 126 in; 1000 lbf; 500 lbf*ft
    assert trip.bound.status == "imported", trip.bound.diagnostics
    assert trip.bound.model["project"]["units"]["length"] == "mm"
    reference = trip.reference()
    assert canon.compare(reference, trip.imported(trip.bound, oriented=True), trip.limits(reference)).failures == []


# --- schema (§18.1 item 6) -----------------------------------------------------------------------------------------
def all_imports(trips):
    found = [(name, trip.bound.model) for name, trip in trips.items()]
    for stem in ("units_english", "units_si"):
        data, declaration = fixture(stem)
        found.append((stem, import_caepipe_mbf(data, material_provenance=declaration["material_provenance"],
                                               source_label=stem).model))
    found += [("z-vertical-si-file", z_import(Z_VERTICAL_SI).model),
              ("z-vertical-english-file", z_import(Z_VERTICAL_ENGLISH).model)]
    return found


def test_every_imported_document_validates_against_the_desktop_profile(trips):
    requirements = desktop_entity_requirements()
    for name, model in all_imports(trips):
        try:
            validate_instance(DESKTOP_SCHEMA, model, schema_label="schemas/model.schema.yaml", instance_label=name)
        except JsonSchemaDependencyMissing as exc:
            _skip_or_note_missing_jsonschema(exc)
        for collection, (fields, optional) in requirements.items():
            if collection == "diagnostics":
                assert model["diagnostics"] == []
                continue
            items = model.get(collection)
            assert items is not None or optional, (name, collection)
            for item in items or []:
                for member in fields:
                    assert isinstance(item.get(member), str) and item[member].strip(), (name, collection, member)
        assert model["schema_version"] == json.loads((ROOT / "schemas" / "model.schema.yaml").read_text(
            encoding="utf-8"))["properties"]["schema_version"]["enum"][0]


def test_canonical_sums_are_exact_and_independent_of_the_writer(trips):
    # Codex VER-002 r1: three same-node Y forces 1e9, 0.001 and -1e9 lbf. By hand the canonical sum is
    # 0.001 x 4.4482216152605 = 0.0044482216152605 N; a binary64 running sum of the converted forces is 0.0044479370.
    trip = trips["writer-cancelling-forces"]
    reference = trip.reference()
    key = ("load", "sustained", "concentrated_force", trip.writer.node_map["N1"], 1)
    assert reference.values[key] == 0.0044482216152605
    running = 0.0
    for value in (1e9, 0.001, -1e9):
        running += value * 4.4482216152605
    assert abs(running - 0.0044482216152605) > 1e-7  # the error the exact sum avoids
    imported = trip.imported(trip.bound, oriented=True)
    assert imported.values[key] == 0.004448221615  # the writer's printed, correctly rounded sum
    assert canon.compare(reference, imported, trip.limits(reference)).failures == []


def test_canonical_form_of_a_tiny_bend_uses_scaled_norms():
    # Codex VER-002 r2 (P3): an imported bend with R = 1e-200 mm and perpendicular legs of 1e-199 mm. The squares of
    # its chord components (about 1e-400) underflow, so an unscaled norm divided by zero. By hand: TI (1e-199, 0, 0);
    # t = 1e-200 x tan 45 deg; the projected bend plane is unit(TI - chord middle) = (1, 0, -1)/sqrt 2.
    data = ("INVENTED TINY BEND\r\nVERSION\r\n15.10\r\nOPTIONS\r\nHGRA,SI\r\nOPTEMP\r\n20\r\nMATERIAL\r\n"
            "M1,0,0.3\r\nM1,20,200000,1.2e-05\r\nPIPE\r\nP1,168.3,7.11\r\nLOADS\r\nL1,20,0,0\r\nLAYOUT\r\n"
            "F10,KA,MM1,PP1,CL=L1\r\nT20,KI,B1e-200,X1e-199,CFF=1\r\nT30,KA,Z1e-199\r\n").encode("ascii")
    result = import_caepipe_mbf(data, material_provenance=INVENTED, source_label="tiny")
    assert result.status == "imported", result.diagnostics
    names = {swb_id: name for name, swb_id in result.id_map["node_map"].items()}
    form = canon.canonical(result.model, names, {}, oriented=True)
    plane = form.sets[("bend", ("20A", "20B"), "plane")]
    assert plane == pytest.approx([math.sqrt(0.5), 0.0, -math.sqrt(0.5)], abs=1e-12)
    assert form.values[("bend", ("20A", "20B"), "R")] == pytest.approx(1e-203, rel=1e-15)  # 1e-200 mm in m


# --- VER-002 r6: what the solve forms in metres, and restored ids ----------------------------------------------------
MITRE_ROWS = ("F10,KA,MM1,PP1,X1e15,CL=L1", "T20,KI,B50,X1000,CFF=1", "T30,KA,Y1000")


def authored_region(model: dict, terminals: tuple[str, str], pressure: float = 10.0) -> dict:
    """`model` with all its pipes in one 10 bar exact-pressure region between `terminals`, as a user authors it."""
    model = deepcopy(model)
    model["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
    for pipe in model["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
    model["load_cases"] = [{"id": "case:p", "label": "Sustained (W+P)", "kind": "sustained", "status": "draft",
                            "primitive_loads": [], "provenance": "invented", "pressure_regions": [
        {"id": "region:r", "member_pipe_ids": [pipe["id"] for pipe in model["pipe_segments"]],
         "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": pressure, "unit": "bar"},
         "provenance": "invented", "terminals": [
             {"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": "invented"}
             for node in terminals]}]}]
    return model


def test_a_bend_whose_arc_in_metres_is_not_tangent_is_refused_whole_on_both_routes():
    # Codex VER-002 r6 (P2). By hand, in mm: TI 20 at (1e15 + 1000, 0, 0) with legs along +X and +Y, theta = 90 deg,
    # t = 50 tan 45 deg = 49.99999999999999, so 20A = (1e15 + 950, 0, 0) and 20B = (1e15 + 1000, 49.99999999999999, 0)
    # (the binary64 spacing at 1e15 mm is 0.125). Read to m (x 0.001, whose binary64 is 0.001 (1 + 2.08e-17)) the
    # spacing at 1e12 is 2^-13: 20A's x is 1e12 + 7783/8192 (0.9500208 x 8192 = 7782.57) and 20B's 1e12 + 1. The
    # arc's chord is (409/8192, 0.04999999999999999, 0) m with R = 0.05 m, and its start tangent leaves the incoming
    # straight (+X) by atan2(c_y, c_x) - asin(|c|/2R), about 1.4648e-3 rad > alpha_tan = 1e-3: a mitre at 20A.
    chord = (409 / 8192, 49.99999999999999 * 0.001)
    theta = math.atan2(chord[1], chord[0]) - math.asin(math.hypot(*chord) / (2 * 0.05))
    assert 1.4648e-3 < theta < 1.4649e-3
    data = importer_tests.mbf(*MITRE_ROWS, pipe=("P1,50,2,0",), loads=("L1,20,10,0",))
    unbound = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    assert refusals(unbound) == {("MBF-IMPORT-PRESSURE-MITRE", "elements 10-20A, 20A-20B, 20B-30")}
    message = next(item["message"] for item in unbound.diagnostics if item["code"] == "MBF-IMPORT-PRESSURE-MITRE")
    assert "at node 20A" in message and f"{theta:.6g} rad" in message
    terminals = [{"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": "invented"}
                 for node in ("node:10", "node:30")]
    free = importer_tests.run(importer_tests.mbf(*MITRE_ROWS, pipe=("P1,50,2,0",)))
    sidecar = complete_sidecar(data, free.model, free.id_map, load_case_map={"case:p": "Sustained (W+P)"},
                               pressure_map={"contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
                                             "shear_model": "euler_bernoulli", "regions": [{"id": "region:r", "members": [
                                                 "pipe:10-20A", "pipe:20A-20B", "pipe:20B-30"],
                                                 "terminals": terminals}]})
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="mitre")
    assert refusals(bound) == {("MBF-IMPORT-PRESSURE-MITRE", "pressure_map region region:r")}
    # The route refuses the same region authored on the file's pressure-free import.
    model = importer_tests.run(importer_tests.mbf(*MITRE_ROWS, pipe=("P1,50,2,0",))).model
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "mitre",
                                                       "model": authored_region(model, ("node:10", "node:30"))})
    assert error and "PRESSURE_REGION_MITRE_UNSUPPORTED" in json.dumps(output)


def test_a_restored_y_reference_the_frame_kernel_refuses_is_refused_by_name():
    # Codex VER-002 r6 (P2): the straight 10 -> 20 runs along (1, 2, 0) m and the sidecar's y_reference is
    # (1, 2.00000001, 0). Its part perpendicular to the axis is exactly 1e-8 (-2, 1, 0)/5, of norm 4.47e-9, but the
    # binary64 projection y.x carries an error of order 1e-16, which the subtraction leaves in y: the kernel's y is
    # about 4e-8 from orthogonal to x, far above its 1e-12 ("local axes must be mutually orthogonal"). The import is
    # refused by name with the element, nothing substituted. Without a sidecar the documented local y applies:
    # Y - (Y.x)x for x = (1, 2, 0)/sqrt 5 is (-2/5, 1/5, 0), normalized (-2, 1, 0)/sqrt 5.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000,Y2000", loads=("L1,20,10,0",))
    restored = {"x": 1.0, "y": 2.00000001, "z": 0.0}
    plain = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    sidecar = complete_sidecar(data, plain.model, plain.id_map)
    sidecar["element_map"]["pipe:10-20"]["y_reference"] = restored
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="parallel")
    assert refusals(result) == {("MBF-IMPORT-ORIENTATION-UNUSABLE", "pipe:10-20")}
    assert importer_module.frame_y_reference_usable((0, 0, 0), (1.0, 2.0, 0), (1.0, 2.00000001, 0.0)) is False
    unbound = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    assert tuple(unbound.model["pipe_segments"][0]["y_reference"].values()) == pytest.approx(
        (-2 / math.sqrt(5), 1 / math.sqrt(5), 0.0), abs=1e-15)
    runner = runner_binary()
    authored = deepcopy(unbound.model)
    authored["pipe_segments"][0]["y_reference"] = restored
    _, output, error = solve_swbpipe(runner, {"case_id": "parallel-authored", "model": authored})
    assert error and "mutually orthogonal" in json.dumps(output)
    _, _, error = solve_swbpipe(runner, {"case_id": "parallel-documented", "model": unbound.model})
    assert error == ""


def test_the_preflight_refuses_an_orientation_its_printed_positions_would_break(monkeypatch):
    # A y_reference 8.1e-8 off the straight's axis: the kernel forms orthonormal axes from it on the authored end
    # (434.3743180726007, 2006.440216234222, 0) mm, but not on the printed one (434.3743181, 2006.440216, 0), where
    # the rounding left in y exceeds 1e-12. The solve is the judge of both.
    end = "X434.3743180726007,Y2006.440216234222"
    model = importer_tests.run(importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", f"T20,KA,{end}", loads=("L1,20,10,0",)),
                               pressure_closure=importer_tests.CLOSURE).model
    restored = {"x": 434.3743180726007, "y": 2006.440379564788, "z": 0.0}
    model["pipe_segments"][0]["y_reference"] = restored
    runner = runner_binary()
    _, _, error = solve_swbpipe(runner, {"case_id": "orientation-authored", "model": model})
    assert error == ""
    written = render_caepipe_mbf_si(model, title="ORIENTATION")
    assert written.status == "refused"
    errors = [item for item in written.diagnostics if item["severity"] == "error"]
    assert {(item["code"], item["entity"]) for item in errors} == {("MBF-SI-PREFLIGHT-REFUSED", "pipe:10-20")}
    assert "MBF-IMPORT-ORIENTATION-UNUSABLE" in errors[0]["message"]
    data, sidecar = bound_parts(model, monkeypatch)  # printed, the bound import refuses it
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="printed")
    assert refusals(bound) == {("MBF-IMPORT-ORIENTATION-UNUSABLE", "pipe:10-20")}
    printed = deepcopy(importer_tests.run(data, pressure_closure=importer_tests.CLOSURE).model)
    printed["pipe_segments"][0]["y_reference"] = restored
    _, output, error = solve_swbpipe(runner, {"case_id": "orientation-printed", "model": printed})
    assert error and "local axes" in json.dumps(output)


def test_the_documented_y_the_frame_kernel_cannot_use_in_metres_is_refused():
    # Near 8.96e15 mm the file's spacing is 1 mm, but read to m (8.96e12, in [2^43, 2^44)) the spacing is 2^-9 m,
    # about twice the 1e-3 m step: the X step of 1 mm vanishes (both ends read as the same binary64), so the axis in
    # m is (0, 1.08e-7, 3.52e-12) m, nearly +Y. The documented y, perpendicular to the file's axis (nearly +X), is
    # nearly +Y too, about 1.1e-4 rad from that axis, and the rounding left in y after removing its axis part exceeds
    # the kernel's 1e-12. The MBF states no other y, so the import is refused; the solve is the judge.
    rows = ("F10,KA,MM1,PP1,X8960000000167142,CL=L1", "T20,KA,X1,Y0.000108,Z3.52e-09")
    assert 8960000000167142 * 0.001 == 8960000000167143 * 0.001
    data = importer_tests.mbf(*rows)
    importer_tests.refused(importer_tests.run(data), "MBF-IMPORT-ORIENTATION-UNUSABLE",
                           importer_tests.layout_line(data, "T20"))
    model = importer_tests.run(importer_tests.mbf(rows[0], "T20,KA,X1000")).model
    end = (8960000000167143.0, 0.000108, 3.52e-09)
    model["nodes"][1]["position"] = dict(zip("xyz", end))
    model["pipe_segments"][0]["y_reference"] = dict(zip("xyz", importer_module.local_y_reference(
        (8960000000167142.0, 0.0, 0.0), end)))
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "documented-unusable",
                                                       "model": authored_region(model, ("node:10", "node:20"))})
    assert error and "PIPE_ELEMENT_INPUT_INVALID" in json.dumps(output)


def test_blank_restored_ids_are_refused_by_kind():
    # Codex VER-002 r6 (P2): the solve refuses an id that Rust's trim() empties (EMPTY_ID; a region member also
    # PRESSURE_REGION_MEMBERS_INVALID). Its whitespace is Unicode White_Space: U+3000 is, U+001C is not (although
    # Python's str.strip removes it).
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,10,0",))
    sidecar = {"mbf_sha256": hashlib.sha256(data).hexdigest(), "element_map": {"": {"from": "10", "to": "20"}}}
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="empty",
                                pressure_closure=importer_tests.CLOSURE)
    assert ("MBF-IMPORT-SIDECAR-INVALID", "element_map[]") in refusals(result)
    model = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE).model
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "empty-id",
                                                       "model": renamed(model, {"pipe:10-20": ""})})
    assert error and all(code in json.dumps(output) for code in ('"EMPTY_ID"', '"PRESSURE_REGION_MEMBERS_INVALID"'))
    data, base = bound_parts(writer_tests.bent_l())

    def rename(mapping: dict, old: str, new: str) -> None:
        mapping[new] = mapping.pop(old)

    changes = {
        "node": lambda sidecar: rename(sidecar["node_map"], "A", " "),
        "pipe": lambda sidecar: rename(sidecar["element_map"], "S1", "\u3000"),
        "component": lambda sidecar: sidecar["element_map"]["BEND"]["bend"].update(component="\t"),
        "material": lambda sidecar: sidecar["material_map"]["M1"].update(swbpipe_material=" "),
        "support": lambda sidecar: rename(sidecar["support_map"], "SA", "\u00a0"),
        "load case": lambda sidecar: rename(sidecar["load_case_map"], "case:th", " "),
        "primitive load": lambda sidecar: rename(sidecar["load_map"], "case:th/t:S1", "case:th/ "),
    }
    path = {"node": "node_map[ ]", "pipe": "element_map[\u3000]", "component": "element_map[BEND].bend.component",
            "material": "material_map[M1].swbpipe_material", "support": "support_map[\u00a0]",
            "load case": "load_case_map[ ]", "primitive load": "load_map[case:th/ ]"}
    for kind, change in changes.items():
        sidecar = deepcopy(base)
        change(sidecar)
        result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=kind)
        assert result.status == "refused", kind
        assert ("MBF-IMPORT-SIDECAR-INVALID", path[kind]) in refusals(result), (kind, refusals(result))
    assert importer_module.runtime_blank("\x1c") is False and "\x1c".strip() == ""
    kept = deepcopy(base)
    rename(kept["element_map"], "S1", "\x1c")
    kept["load_map"]["case:th/t:S1"]["target"]["pipe"] = "\x1c"
    assert import_caepipe_mbf(data, sidecar=kept, material_provenance=INVENTED, source_label="kept").status == "imported"


def test_sections_are_checked_on_their_binary64_values_in_metres():
    # OD 504 mm, wall 251.99999999999997 mm (the binary64 below 252, 252 - 2^-45): 2t < OD in mm. In m the wall is
    # 251.99999999999997158 x 0.001 (1 + 2.08e-17) = 0.2519999999999999768, above the midpoint 0.2519999999999999740
    # of its neighbours, so it rounds to the binary64 0.2520000000000000018; the OD, 0.50400000000000001049, rounds to
    # 0.5040000000000000036, twice that: 2t = OD in m. And OD 1 with wall 0.49999999999999999: 2t < OD in decimals,
    # but the wall's binary64 is 0.5 (it lies 1e-17 below 0.5, the binary64 spacing there 5.6e-17).
    runner = runner_binary()
    for name, section, wall in (("metres", "P1,504,251.99999999999997,0", 251.99999999999997),
                                ("decimal", "P1,1,0.49999999999999999,0", 0.5)):
        data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", pipe=(section,))
        importer_tests.refused(importer_tests.run(data), "MBF-IMPORT-VALUE-OUT-OF-RANGE",
                               importer_tests.layout_line(data, "P1,"))
        od = float(section.split(",")[1])
        model = authored_region(importer_tests.run(importer_tests.mbf(
            "F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", pipe=(f"P1,{od},{od / 4},0",))).model, ("node:10", "node:20"))
        model["pipe_segments"][0]["section"]["wall_thickness"]["value"] = wall
        _, output, error = solve_swbpipe(runner, {"case_id": f"section-{name}", "model": model})
        assert error and "PIPE_DIMENSION_INVALID" in json.dumps(output), name


def test_a_pressure_beyond_binary64_in_pascals_is_refused():
    # 1e304 bar is finite, but the route reads it as 1e304 x 1e5 = 1e309 Pa, beyond the binary64 maximum (about
    # 1.8e308), and refuses it (PRESSURE_REGION_PRESSURE_INVALID). The headless runner's checked-JSON reader stops
    # such a value earlier (an integral float above 2^53), so the arithmetic, not a solve, is the reference here.
    assert 1e304 * 100000.0 == math.inf
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,1e304,0",))
    importer_tests.refused(importer_tests.run(data, pressure_closure=importer_tests.CLOSURE),
                           "MBF-IMPORT-VALUE-UNREPRESENTABLE", importer_tests.layout_line(data, "L1,"))


# --- VER-002 r7: restored sidecar content stays within what the import admits ---------------------------------------
def test_a_restored_support_family_the_route_does_not_admit_is_refused():
    # Codex VER-002 r7 (P2): a SHA-bound sidecar gives the anchor at 10 the family constant_effort_support. The
    # writer exports only rigid families (anchor, guide, line_stop, vertical_support, or none) and the exact route
    # refuses the others (EXACT_PRESSURE_FAMILY_NOT_ADMITTED), so the import refuses it by name with the support id.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    for options in ("HGRA,SI", "HGRA,SI,Z", "HGRA"):
        data = importer_tests.mbf(*rows, options=options, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",))
        plain = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
        base = complete_sidecar(data, plain.model, plain.id_map)
        for family, admitted in (("constant_effort_support", False), ("anchor", True)):
            sidecar = deepcopy(base)
            sidecar["support_map"]["support:10"]["family"] = family
            result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=options)
            if admitted:
                assert result.status == "imported", (options, result.diagnostics)
                assert result.model["supports"] == plain.model["supports"], options
            else:
                assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "support_map[support:10].family")}
    model = importer_tests.run(importer_tests.mbf(*rows, loads=("L1,20,10,0",)),
                               pressure_closure=importer_tests.CLOSURE).model
    model["supports"][0]["family"] = "constant_effort_support"
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "family", "model": model})
    assert error and "EXACT_PRESSURE_FAMILY_NOT_ADMITTED" in json.dumps(output)


def test_restored_supports_are_held_to_the_solves_family_dofs_and_overlap():
    # Codex VER-002 r8 (P2). The solve infers an absent family as anchor with six DOFs and guide otherwise; a guide or
    # line stop restrains translations only and a vertical support UZ only; a DOF restrained twice at a node is
    # refused (SUPPORT_INPUT_INVALID). Each case is refused by support id, and the solve refuses the same supports.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    data = importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",))
    plain = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    six = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
    cases = {f"six-dof {family}": {"support:10": {"node": "10", "restrained": six,
                                                  "family": family, "family_authored": True}}
             for family in ("guide", "line_stop", "vertical_support")}
    cases["overlap"] = {"support:10": {"node": "10", "restrained": six, "family": "anchor",
                                      "family_authored": False},
                        "s2": {"node": "10", "restrained": ["UX"], "family": "guide",
                               "family_authored": False}}
    runner = runner_binary()
    for name, supports in cases.items():
        sidecar = complete_sidecar(data, plain.model, plain.id_map)
        sidecar["support_map"] = {**supports, "support:20": sidecar["support_map"]["support:20"]}
        result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=name)
        refused_ids = {support_id for support_id, entry in supports.items()
                       if (entry["family_authored"] and entry["family"] != "anchor") or support_id == "s2"}
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", f"support_map[{support_id}]")
                                    for support_id in refused_ids}, name
        model = deepcopy(plain.model)
        model["supports"] = [{"id": support_id, "node": f"node:{entry['node']}", "restraints": entry["restrained"],
                              "provenance": "invented",
                              **({"family": entry["family"]} if entry["family_authored"] else {})}
                             for support_id, entry in supports.items()] + model["supports"][1:]
        _, output, error = solve_swbpipe(runner, {"case_id": name, "model": model})
        assert error and "SUPPORT_INPUT_INVALID" in json.dumps(output), name


def test_unbound_supports_the_solve_refuses_are_refused_by_name():
    # The unbound route meets the same rules: FIXD(1,0,1) with FIXRY and no family is a guide to the solve, which
    # restrains no rotation; KA with FIXD restrains UX twice at one node. Both are refused by support id; the solve
    # refuses each as authored. FIXD with FIXR on all six DOFs is an anchor to the solve and is kept.
    runner = runner_binary()
    for name, row, support_id in (("partial-rotation", "T20,KA,X3000", "support:30-restraint"),
                                  ("anchor-and-fixd", "T20,KA,X3000,CFIXD(1,0,0)", "support:20-restraint")):
        rows = ("F10,KA,MM1,PP1,CL=L1", row) + (("T30,X10,CFIXD(1,0,1),FIXRY",) if name == "partial-rotation"
                                                else ())
        result = importer_tests.run(importer_tests.mbf(*rows, loads=("L1,20,10,0",)),
                                    pressure_closure=importer_tests.CLOSURE)
        assert refusals(result) == {("MBF-IMPORT-SUPPORT-UNSUPPORTED", support_id)}, name
    kept = importer_tests.run(importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000",
                                                 "T30,X10,CFIXD(1,1,1),FIXR(1,1,1)"))
    assert kept.status == "imported"
    model = importer_tests.run(importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,X3000")).model
    for name, supports in (("partial-rotation", [{"id": "g", "node": "node:20", "restraints": ["UX", "UZ", "RY"],
                                                  "provenance": "invented"}]),
                           ("anchor-and-fixd", [{"id": "a", "node": "node:20", "restraints": [
                               "UX", "UY", "UZ", "RX", "RY", "RZ"], "family": "anchor", "provenance": "invented"},
                                                {"id": "g", "node": "node:20", "restraints": ["UX"],
                                                 "provenance": "invented"}])):
        authored = deepcopy(model)
        authored["load_cases"] = [{"id": "case:w", "label": "Sustained (W+P)", "kind": "sustained", "status": "draft",
                                   "provenance": "invented", "primitive_loads": [
            {"id": "f", "category": "concentrated_force", "target": {"type": "node", "node": "node:20"},
             "direction": "global_y", "magnitude": {"value": -1.0, "unit": "N"}, "dimension": "force",
             "provenance": "invented"}]}]
        authored["supports"] = authored["supports"] + supports
        _, output, error = solve_swbpipe(runner, {"case_id": name, "model": authored})
        assert error and "SUPPORT_INPUT_INVALID" in json.dumps(output), name


def test_every_restored_sidecar_field_outside_the_admitted_vocabulary_is_refused(trips):
    # Each field a bound import restores, set to a value the unbound route never produces and the writer never
    # records: refused by name (MBF-IMPORT-SIDECAR-INVALID), never substituted or skipped. Before r7 a malformed
    # restrained list or load target raised, and an unknown shear model was replaced by euler_bernoulli.
    data, base = bound_parts(writer_tests.bent_l())
    changes = {
        "element_map[S1].reversed": lambda sidecar: sidecar["element_map"]["S1"].update(reversed="yes"),
        "element_map[BEND].bend.component": lambda sidecar: sidecar["element_map"]["BEND"]["bend"].update(component=7),
        "support_map[SA].restrained": lambda sidecar: sidecar["support_map"]["SA"].update(restrained=5),
        "support_map[SB].family": lambda sidecar: sidecar["support_map"]["SB"].update(
            family="anchor", restrained=["UX", "UY", "UZ", "RX", "RY"]),
        "material_map[M1].origin": lambda sidecar: sidecar["material_map"]["M1"].update(origin="vendor_library"),
        "material_map[M1].swbpipe_material": lambda sidecar: sidecar["material_map"]["M1"].update(
            swbpipe_material=3),
        "load_map[case:th/t:S1].target": lambda sidecar: sidecar["load_map"]["case:th/t:S1"].update(target="S1"),
        "load_map[case:th/t:S1].direction": lambda sidecar: sidecar["load_map"]["case:th/t:S1"].update(
            direction="sideways"),
    }
    for location, change in changes.items():
        sidecar = deepcopy(base)
        change(sidecar)
        options = {} if location.startswith("material_map[M1].origin") else {"material_provenance": INVENTED}
        result = import_caepipe_mbf(data, sidecar=sidecar, source_label=location, **options)
        assert result.status == "refused", location
        assert ("MBF-IMPORT-SIDECAR-INVALID", location) in refusals(result), (location, refusals(result))
    trip = trips["pressure-tee-bar"]
    for path, change in (("pressure_map.shear_model", lambda state: state.update(shear_model="plastic")),
                         ("pressure_map.contract.version", lambda state: state["contract"].update(version="2.0.0")),
                         ("pressure_map.contract.mode", lambda state: state["contract"].update(mode="legacy"))):
        sidecar = deepcopy(trip.sidecar)
        change(sidecar["pressure_map"])
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label="contract")
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", path)}


# --- VER-002 r8: the sidecar parser under mutation ------------------------------------------------------------------
MUTATION_SEED = 20261027


def sidecar_mutations(sidecar: dict) -> list[tuple[str, dict]]:
    """Every field of `sidecar`, mutated deterministically: each value swapped for every other JSON type (list,
    object, string, number, bool, null), each container emptied, each number made huge or non-finite, an unknown key
    added to and each key removed from every object. The seeded generator picks the stand-in of each type."""
    rng = random.Random(MUTATION_SEED)
    stand_ins = {"list": [["x"], [1], [None]], "object": [{"x": 1}, {"node": ["node:20"]}],
                 "string": ["x", "zz", "[]"], "number": [7, -1.5, 0], "bool": [True, False], "null": [None]}
    kind = lambda value: ("null" if value is None else "bool" if isinstance(value, bool)  # noqa: E731
                          else "number" if isinstance(value, (int, float)) else "string" if isinstance(value, str)
                          else "list" if isinstance(value, list) else "object")
    found: list[tuple[str, dict]] = []

    def at(path: tuple, value: Any, label: str) -> None:
        mutated = deepcopy(sidecar)
        holder = mutated
        for step in path[:-1]:
            holder = holder[step]
        if value is _DELETE:
            del holder[path[-1]]
        else:
            holder[path[-1]] = deepcopy(value)
        found.append((f"{'/'.join(map(str, path))}: {label}", mutated))

    def walk(value: Any, path: tuple) -> None:
        if path:
            for other in sorted(stand_ins):
                if other != kind(value):
                    at(path, rng.choice(stand_ins[other]), f"{other} for {kind(value)}")
            if isinstance(value, (list, dict)) and value:
                at(path, type(value)(), "emptied")
            if kind(value) == "number":
                for number in (10 ** 400, -10 ** 400, math.inf, -math.inf, math.nan):
                    at(path, number, f"{number!r}")
        if isinstance(value, dict):
            at(path + ("zz_unknown",), 1, "unknown key")
            for key in value:
                at(path + (key,), _DELETE, "removed")
                walk(value[key], path + (key,))
        elif isinstance(value, list):
            for index, item in enumerate(value):
                walk(item, path + (index,))

    walk(sidecar, ())
    return found


_DELETE = object()


def sidecar_replacements(sidecar: dict, data: bytes) -> list[tuple[str, dict, str]]:
    """Each format-2 field of `sidecar` set to a different well-formed value, with what the import must do: refuse
    where the MBF or a derivation determines the value, import where the writer copies it from the model unchanged
    (so it regenerates as given; Design §14)."""
    from core.handoff.caepipe_mbf.importer import documented_orientations
    derived = documented_orientations(data)
    found: list[tuple[str, dict, str]] = []

    def at(label: str, expected: str, change) -> None:
        mutated = deepcopy(sidecar)
        change(mutated)
        found.append((label, mutated, expected))

    other_label = {"Sustained (W+P)": "Expansion (T1)", "Expansion (T1)": "Sustained (W+P)"}
    for swb_id in sidecar["node_map"]:
        at(f"node_map[{swb_id}]", "refused", lambda s, k=swb_id: s["node_map"].update({k: "99"}))
    for swb_id, entry in sidecar["element_map"].items():
        for key in ("from", "to"):
            at(f"element_map[{swb_id}].{key}", "refused",
               lambda s, k=swb_id, f=key: s["element_map"][k].update({f: "99"}))
        at(f"element_map[{swb_id}].reversed", "imported",
           lambda s, k=swb_id: s["element_map"][k].update(reversed=not s["element_map"][k]["reversed"]))
        if "bend" in entry:
            at(f"element_map[{swb_id}].bend.component", "imported",
               lambda s, k=swb_id: s["element_map"][k]["bend"].update(component="component:other"))
            continue
        flipped = {axis: -value + 0.0 for axis, value in entry["y_reference"].items()}
        at(f"element_map[{swb_id}].y_reference", "imported" if entry["y_reference_authored"] else "refused",
           lambda s, k=swb_id, v=flipped: s["element_map"][k].update(y_reference=v))
        documented = entry["y_reference"] == dict(zip("xyz", derived[(entry["from"], entry["to"])]))
        at(f"element_map[{swb_id}].y_reference_authored", "imported" if documented else "refused",
           lambda s, k=swb_id: s["element_map"][k].update(y_reference_authored=not s["element_map"][k][
               "y_reference_authored"]))
    for swb_id, entry in sidecar["support_map"].items():
        at(f"support_map[{swb_id}].node", "refused", lambda s, k=swb_id: s["support_map"][k].update(node="99"))
        other = ["UX"] if entry["restrained"] != ["UX"] else ["UZ"]
        at(f"support_map[{swb_id}].restrained", "refused",
           lambda s, k=swb_id, v=other: s["support_map"][k].update(restrained=v))
        # A spring (format 3) and a rigid support each read as the other kind: the MBF's springs and rigid DOFs differ.
        if sidecar.get("sidecar_format") == 3 and len(entry["restrained"]) == 1:
            at(f"support_map[{swb_id}].family", "refused", lambda s, k=swb_id, v=entry["family"]: s["support_map"][
                k].update(family="line_stop" if v == "spring" else "spring", family_authored=True))
        inferred = "anchor" if len(entry["restrained"]) == 6 else "guide"
        if not entry["family_authored"]:
            at(f"support_map[{swb_id}].family", "refused",
               lambda s, k=swb_id, f=inferred: s["support_map"][k].update(
                   family="guide" if f == "anchor" else "anchor"))
        at(f"support_map[{swb_id}].family_authored",
           "imported" if entry["family_authored"] is False or entry["family"] == inferred else "refused",
           lambda s, k=swb_id: s["support_map"][k].update(family_authored=not s["support_map"][k]["family_authored"]))
    for swb_id, label in sidecar["load_case_map"].items():
        at(f"load_case_map[{swb_id}]", "refused", lambda s, k=swb_id, v=other_label[label]: s["load_case_map"].update(
            {k: v}))
    for name, entry in sidecar["material_map"].items():
        at(f"material_map[{name}].swbpipe_material", "imported",
           lambda s, k=name: s["material_map"][k].update(swbpipe_material="mat:other"))
        other = next(origin for origin in ("project_invented", "user_supplied", "undeclared")
                     if origin != entry["origin"])
        at(f"material_map[{name}].origin", "imported", lambda s, k=name, v=other: s["material_map"][k].update(origin=v))
    for key, entry in sidecar["load_map"].items():
        at(f"load_map[{key}].load_case", "refused", lambda s, k=key: s["load_map"][k].update(load_case="case:other"))
        other = next(item for item in ("weight", "thermal", "concentrated_force") if item != entry["category"])
        at(f"load_map[{key}].category", "refused", lambda s, k=key, v=other: s["load_map"][k].update(category=v))
        target = {"node": "zz"} if "node" not in entry["target"] else {"pipe": "zz"}
        at(f"load_map[{key}].target", "refused", lambda s, k=key, v=target: s["load_map"][k].update(target=v))
        if entry["category"] == "thermal":  # the MBF carries no thermal direction: the recorded one is authored
            at(f"load_map[{key}].direction", "imported", lambda s, k=key: s["load_map"][k].update(direction="global_z"))
        else:
            other = "global_z" if entry["direction"] not in ("global_z", "UZ") else "global_x"
            at(f"load_map[{key}].direction", "refused", lambda s, k=key, v=other: s["load_map"][k].update(direction=v))
    state = sidecar["pressure_map"]
    if state["contract"] == "none":
        at("pressure_map.contract", "imported", lambda s: s["pressure_map"].update(
            contract={"version": "3.0.0", "mode": "exact_pressure_v3"}))
        at("pressure_map.shear_model", "refused", lambda s: s["pressure_map"].update(
            shear_model="timoshenko_annulus_energy_v1"))
    else:
        at("pressure_map.contract", "refused", lambda s: s["pressure_map"].update(contract="none"))
        other = next(model for model in ("euler_bernoulli", "timoshenko_annulus_energy_v1")
                     if model != state["shear_model"])
        at("pressure_map.shear_model", "imported", lambda s, v=other: s["pressure_map"].update(shear_model=v))
    for index, region in enumerate(state["regions"]):
        at(f"pressure_map.regions[{index}].id", "imported",
           lambda s, i=index: s["pressure_map"]["regions"][i].update(id="region:other"))
        at(f"pressure_map.regions[{index}].members", "refused",
           lambda s, i=index: s["pressure_map"]["regions"][i].update(members=["zz"]))
        for end in range(2):
            terminal = lambda s, i=index, j=end: s["pressure_map"]["regions"][i]["terminals"][j]  # noqa: E731
            at(f"pressure_map.regions[{index}].terminals[{end}].node_ref", "refused",
               lambda s, t=terminal: t(s).update(node_ref="node:zz"))
            at(f"pressure_map.regions[{index}].terminals[{end}].closure_transfer", "imported",
               lambda s, t=terminal: t(s).update(closure_transfer="separately_supported_or_compensated"
                                                 if t(s)["closure_transfer"] == "transfers_to_wall"
                                                 else "transfers_to_wall"))
            at(f"pressure_map.regions[{index}].terminals[{end}].provenance", "imported",
               lambda s, t=terminal: t(s).update(provenance="another invented provenance"))
    at("mbf_sha256", "unbound", lambda s: s.update(mbf_sha256="0" * 64))
    at("sidecar_format", "refused", lambda s: s.update(sidecar_format=4))
    at("sidecar_format", "refused", lambda s: s.update(sidecar_format=2))  # format 2 has no self-digest field
    return found


SIDECAR_MAPS = ("node_map", "element_map", "support_map", "load_case_map", "material_map", "section_map", "load_map",
                "pressure_map")


def contract_only_model() -> dict:
    """Codex VER-002 r11's fixture: a pressure-free invented file whose model carries a Timoshenko contract and no
    region, with an explicit zero allowance."""
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X100,CFOR(0,-1000,0)")
    model = importer_tests.run(importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,0,0",))).model
    model["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3",
                                  "shear_model": "timoshenko_annulus_energy_v1"}
    for pipe in model["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
    for case in model["load_cases"]:
        case["pressure_regions"] = []
    return model


def authored_orientation_model() -> dict:
    """Codex VER-002 r10, r11's fixture: an invented 10 bar straight along +X authored with y_reference (0, 0, 1),
    not the documented (0, 1, 0)."""
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    model = importer_tests.run(importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",)),
                               pressure_closure=CLOSURE).model
    model["pipe_segments"][0]["y_reference"] = {"x": 0.0, "y": 0.0, "z": 1.0}
    return model


def summed_model() -> dict:
    """The invented cantilever with a second force on its tip along the same axis: a many-to-one load_map (REQ-016)."""
    model = writer_tests.cantilever()
    model["load_cases"][0]["primitive_loads"].append(writer_tests.force("tip2", "N1", "GLOBAL_Y", -1000.0))
    return model


MUTATION_BASES = {"pressurized": lambda: writer_tests.pressurized_l(), "pressure-free": lambda: writer_tests.bent_l(),
                  "summed": summed_model, "springs": lambda: writer_tests.sprung_cantilever(),
                  "contract-only": contract_only_model, "authored-orientation": authored_orientation_model}
# Bases without a pressure contract. Every base's export runs the preflight, which is total (VER-002 r3).
PRESSURE_FREE_BASES = ("pressure-free", "summed", "springs")


@pytest.mark.parametrize("base_model", sorted(MUTATION_BASES))
def test_every_mutated_sidecar_is_refused_by_name_or_restores_the_same_model(base_model):
    # VER-002 r8, r9: the import of each mutated sidecar either refuses with a named MBF-IMPORT code or imports the
    # unmutated model, and never raises; with pressure, the writer's preflight, given the same mutated maps, refuses
    # with a named MBF-SI code exactly where the import refuses, and never raises. Each import passes a closure
    # declaration, so a bound import that fell through to unbound region formation would succeed and be caught. The
    # bases have a bend, two supports and thermal loads, and one has pressure; their straights' recorded y_reference
    # is the documented one and their families are absent, so a removed optional record restores the same model.
    # Besides the field mutations, each of the eight maps is deleted and emptied, present in the base or not. The
    # base is a format-2 sidecar, complete by rule and with no null, so every deletion and every field set to null is
    # refused (r10, r11). The bases add Codex's contract-only file and a non-default authored y_reference. A mutated
    # digest that is still well formed is another file's: the import is the unbound one, as documented.
    from core.handoff.caepipe_mbf import si_writer
    captured: dict[str, Any] = {}
    original = si_writer._Writer.survives_reimport

    def capture(self, result):
        captured.update(writer=self, result=result, diagnostics=list(self.diagnostics), loss=list(self.loss))
        return original(self, result)

    si_writer._Writer.survives_reimport = capture
    try:
        written = render_caepipe_mbf_si(MUTATION_BASES[base_model](), title="MUTATION")
    finally:
        si_writer._Writer.survives_reimport = original
    assert written.status == "written", written.diagnostics
    data = written.text.encode("ascii")
    base = written.sidecar(hashlib.sha256(data).hexdigest())
    run = lambda sidecar, label: import_caepipe_mbf(  # noqa: E731
        data, sidecar=sidecar, material_provenance=INVENTED, pressure_closure=CLOSURE, source_label=label)
    expected = run(base, "base")
    assert expected.status == "imported", expected.diagnostics
    # VER-002 r13: the regeneration identity is the guard of record. The writer's own sidecar regenerates exactly
    # (the positive control), and every mutated sidecar differs from that regeneration, so the identity alone,
    # given the model the file and the unmutated sidecar determine, refuses each one too.
    assert regeneration_difference(base, expected) is None
    regenerated = regenerated_from(expected)
    summed = groups(expected)  # only these identities are set aside; every field of their entries is compared
    assert bool(summed) == (base_model == "summed") and set_aside(expected) == reported(expected)
    assert bool(expected.model.get("pressure_contract")) == (base_model not in PRESSURE_FREE_BASES)
    unbound = run(None, "unbound")  # a digest that is not this file's leaves the sidecar unapplied (§15)
    assert unbound.status == "imported", unbound.diagnostics
    mutations = sidecar_mutations(base)
    for name in SIDECAR_MAPS:  # format 2 leaves section_map to the MBF: adding it (emptied) is refused too
        if name in base:
            mutations.append((f"{name}: map deleted",
                              {key: value for key, value in deepcopy(base).items() if key != name}))
        if base.get(name, None) != {}:  # emptying a map the base already holds empty changes nothing
            mutations.append((f"{name}: map emptied", {**deepcopy(base), name: {}}))
    # VER-002 r14: inside each entry reported as summed, its category, target, direction and case changed, and an
    # entry added under the summed primitive's formed id (a duplicate primitive id). Each is refused.
    for formed, keys in summed.items():
        for key in keys:
            for field, value in (("category", "concentrated_moment"), ("target", {"node": "N0"}),
                                 ("direction", "global_x"), ("load_case", "case:other")):
                mutated = deepcopy(base)
                mutated["load_map"][key][field] = value
                mutations.append((f"load_map/{key}/{field}: reported entry changed", mutated))
        mutated = deepcopy(base)
        mutated["load_map"][formed] = {**deepcopy(base["load_map"][keys[0]]), "direction": "global_x"}
        mutations.append((f"load_map/{formed}: duplicate of the formed id", mutated))
    outcomes = {"refused": 0, "equal": 0, "unbound": 0, "stale": 0}

    def stale(sidecar, label):
        # The same edit with its self-digest left stale (an accidental or partial edit): refused on every route.
        # Where the edited sidecar still claims format 3 with a well-formed digest and canonical content, the
        # self-digest alone refuses it, before the parser; otherwise the parser refuses it by path.
        given = import_as_given(data, sidecar=sidecar, material_provenance=INVENTED, pressure_closure=CLOSURE,
                                source_label="stale")
        assert given.status == "refused", label
        if sidecar.get("sidecar_format") == SIDECAR_FORMAT and canon.redigested(sidecar) != sidecar:
            assert refusals(given) == {("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH", "sidecar_sha256")}, label
            outcomes["stale"] += 1
        else:
            assert {code for code, _ in refusals(given)} == {"MBF-IMPORT-SIDECAR-INVALID"}, label

    # Removing one of the summed entries leaves a sidecar the writer writes for a model with one primitive there: a
    # consistent edit, which imports as that model (§14); it is the only mutation that imports.
    consistent = {f"load_map/{key}: removed" for keys in summed.values() for key in keys}
    for label, sidecar in mutations:
        stale(sidecar, label)
        imported = run(sidecar, "mutated")
        errors = {item["code"] for item in imported.diagnostics if item["severity"] == "error"}
        if label in consistent:
            assert imported.status == "imported" and imported.source["binding"] == "bound", label
            assert regeneration_difference(sidecar, imported) is None and not groups(imported), label
            outcomes["equal"] += 1
            continue
        if label.endswith(": removed") or label.endswith(": map deleted") or ": null for " in label:
            assert imported.status == "refused", label  # format 2 is complete and holds no null (r10, r11)
        if imported.status == "refused":
            assert errors and all(code.startswith("MBF-IMPORT-") for code in errors), label
            # The identity alone refuses it too; a mutated digest is the parser's (it decides the route, and the
            # identity, which compares everything else, excludes it), and a mutated self-digest the self-digest's.
            assert label.startswith(("mbf_sha256", "sidecar_sha256")) or sidecar_difference(
                sidecar, regenerated, summed=summed) is not None, label
            outcomes["refused"] += 1
        elif imported.source["binding"] == "unbound":
            assert label.startswith("mbf_sha256") and imported.model == unbound.model, label
            assert "MBF-IMPORT-SIDECAR-NOT-APPLIED" in {item["code"] for item in imported.diagnostics}, label
            outcomes["unbound"] += 1
        else:  # r13: no mutation on the exact-digest route imports
            outcomes["equal"] += 1
            pytest.fail(f"{label} imported bound")
        if "writer" not in captured or not label.split("/")[0].split(":")[0].endswith("_map"):
            continue  # the preflight binds its own text by its own hash and reads only the maps
        writer, result = captured["writer"], captured["result"]
        writer.diagnostics[:], writer.loss[:] = list(captured["diagnostics"]), list(captured["loss"])
        maps = {key: value for key, value in sidecar.items() if key != "mbf_sha256"}
        object.__setattr__(result, "sidecar", lambda digest, maps=maps: canon.redigested(
            {**maps, "mbf_sha256": digest}))
        survives = original(writer, result)
        refusals_ = {item["code"] for item in writer.diagnostics if item["severity"] == "error"}
        if imported.status == "refused":
            assert not survives and refusals_ and all(code.startswith("MBF-SI-") for code in refusals_), label
        else:
            assert survives, label
    assert "writer" in captured
    assert len(mutations) > 300 and outcomes["refused"] > 0, outcomes
    # VER-002 r12, r13: each field set to a different well-formed value. A value the MBF or a derivation determines
    # must agree with it, and the regeneration identity alone refuses it too. A value the writer copies from the model
    # unchanged (an id, `reversed`, an authored orientation or family, an origin, a thermal direction, the shear
    # model, a closure) regenerates as given: a consistent edit, which imports as stated (Design §14: the guard
    # proves consistency, not authorship).
    replaced = sidecar_replacements(base, data)
    for label, sidecar, expected in replaced:
        stale(sidecar, label)
        imported = run(sidecar, "replaced")
        errors = {item["code"] for item in imported.diagnostics if item["severity"] == "error"}
        if expected == "refused":
            assert imported.status == "refused" and errors, label
            assert all(code.startswith("MBF-IMPORT-") for code in errors), label
            assert sidecar_difference(sidecar, regenerated, summed=summed) is not None, label
        elif expected == "unbound":
            assert imported.source["binding"] == "unbound", label
        else:
            assert imported.status == "imported", (label, errors)
            assert regeneration_difference(sidecar, imported) is None, label
    assert {expected for _, _, expected in replaced} >= {"refused", "imported"}
    assert outcomes["stale"] > (len(mutations) + len(replaced)) // 2, outcomes


LOAD_FIELDS = {"load_case": "case:mbf-sustained", "direction": "global_y"}


def test_nested_sidecar_values_are_refused_by_path_and_the_writer_reads_no_unhashable_value():
    # Codex VER-002 r8 (P2), on a one-straight invented SI MBF with SHA-bound sidecars: a load target naming a node
    # by a list raised TypeError, a terminal node_ref [] crashed the importer and the writer, a y_reference component
    # 10**400 raised OverflowError, and a falsey target passed through `or {}`. Each is refused by JSON path.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,10,0",))
    plain = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    base = complete_sidecar(data, plain.model, plain.id_map)
    assert import_caepipe_mbf(data, sidecar=base, material_provenance=INVENTED, source_label="base").status == "imported"
    cases = {
        "load_map[case:mbf-sustained/l].target.node": lambda sidecar: sidecar["load_map"].update(
            {"case:mbf-sustained/l": {**LOAD_FIELDS, "category": "concentrated_force",
                                      "target": {"node": ["node:20"]}}}),
        "pressure_map.regions[0].terminals[0].node_ref": lambda sidecar: sidecar["pressure_map"]["regions"][0][
            "terminals"][0].update(node_ref=[]),
        "element_map[pipe:10-20].y_reference.x": lambda sidecar: sidecar["element_map"]["pipe:10-20"].update(
            y_reference={"x": 10 ** 400, "y": 1.0, "z": 0.0}),
        "load_map[case:mbf-sustained/l].target": lambda sidecar: sidecar["load_map"].update(
            {"case:mbf-sustained/l": {**LOAD_FIELDS, "category": "weight", "target": {}}}),
    }
    for path, change in cases.items():
        sidecar = deepcopy(base)
        change(sidecar)
        result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=path)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", path)}, (path, refusals(result))
    model = deepcopy(writer_tests.pressurized_l())
    next(case for case in model["load_cases"] if case.get("pressure_regions"))["pressure_regions"][0]["terminals"][0][
        "node_ref"] = []
    written = render_caepipe_mbf_si(model, title="NODE REF")
    assert written.status == "refused" and "MBF-SI-INPUT-INVALID" in {item["code"] for item in written.diagnostics}


# --- §9.4 rulings (Agent 0's decisions under the owner's delegation, 2026-10-27) -------------------------------------
def terminal_transfers(output: Any, node: str) -> list[list[float]]:
    """Every region terminal at `node` in the solve's pressure evidence: its pipe cap transfer, in N."""
    found = []
    if isinstance(output, dict):
        if output.get("node_ref") == node and "pipe_cap_transfer_global_n" in output:
            found.append(output["pipe_cap_transfer_global_n"])
        for value in output.values():
            found.extend(terminal_transfers(value, node))
    elif isinstance(output, list):
        for value in output:
            found.extend(terminal_transfers(value, node))
    return found


def test_a_corner_terminal_carries_the_wall_thrust_whatever_the_global_closure():
    # A sharp corner at 20 (+X in, +Y out) ends two regions there. The pipe wall is continuous through it, so both
    # corner terminals take transfers_to_wall, deduced from geometry, even under a global
    # separately_supported_or_compensated, which governs the real ends 10 and 30. By hand: r_i = 168.3/2 - 7.11 =
    # 77.04 mm, A = pi (0.07704 m)^2, p = 10 bar = 1e6 Pa; the two end forces at 20, pA t_in and -pA t_out, sum to
    # pA(t_in - t_out) = pA (1, -1, 0), with pA = 18645.86 N.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,X1000", "T30,KA,Y1000", loads=("L1,20,10,0",))
    remote = {"closure_transfer": "separately_supported_or_compensated", "provenance": "invented remote declaration"}
    result = importer_tests.run(data, pressure_closure=remote)
    assert importer_tests.regions_of(result) == [
        ("region:mbf-10-20", ["pipe:10-20"], 10.0, "bar",
         [("node:10", "separately_supported_or_compensated"), ("node:20", "transfers_to_wall")]),
        ("region:mbf-20-30", ["pipe:20-30"], 10.0, "bar",
         [("node:20", "transfers_to_wall"), ("node:30", "separately_supported_or_compensated")])]
    corner = [terminal for case in result.model["load_cases"] for region in case["pressure_regions"]
              for terminal in region["terminals"] if terminal["node_ref"] == "node:20"]
    assert all(terminal["provenance"].startswith("closure transfer deduced from geometry at import, not inferred "
                                                 "from the file") for terminal in corner)
    assert any("a corner terminal takes transfers_to_wall, deduced from geometry" in note
               for note in result.assumptions)
    p_a = 1.0e6 * math.pi * 0.07704 ** 2
    assert 18645.86 < p_a < 18645.87
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "corner", "model": result.model})
    assert error == ""
    transfers = terminal_transfers(output, "node:20")
    assert len(transfers) == 2
    total = [sum(force[axis] for force in transfers) for axis in range(3)]
    assert total == pytest.approx([p_a, -p_a, 0.0], rel=1e-12, abs=1e-9)
    # The per-node override names the corner node and is honoured there, with its declared provenance.
    override = importer_tests.run(data, pressure_closure={**remote,
                                                          "nodes": {"20": "separately_supported_or_compensated"}})
    corner = [terminal for case in override.model["load_cases"] for region in case["pressure_regions"]
              for terminal in region["terminals"] if terminal["node_ref"] == "node:20"]
    assert [(terminal["closure_transfer"], terminal["provenance"]) for terminal in corner] == [
        ("separately_supported_or_compensated", "closure transfer declared at import: invented remote declaration")] * 2


def test_a_bound_pressurized_import_without_its_pressure_map_is_refused(monkeypatch):
    # Codex VER-002 r9 (P2): imported with Euler-Bernoulli, written and hash-bound, then with pressure_map deleted or
    # emptied, the reimport was bound yet formed regions and took the Timoshenko default. A bound import with
    # Pressure 1 needs its pressure_map (contract, shear model, regions) and never falls through to the unbound route.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X1000,CFOR(0,-1000,0),FLC=1")
    model = importer_tests.run(importer_tests.mbf(*rows, loads=("L1,20,10,0",)), pressure_closure=CLOSURE,
                               shear_model="euler_bernoulli").model
    data, sidecar = bound_parts(model)
    restored = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="whole",
                                  pressure_closure=CLOSURE)
    assert restored.model["pressure_contract"]["shear_model"] == "euler_bernoulli"
    # Format 2 records the pressure state always: deleted or emptied, it is refused by path. A legacy sidecar (main's
    # writer exported no pressure) cannot bind a file with Pressure 1.
    for change, expected in (("deleted", {("MBF-IMPORT-SIDECAR-INVALID", "pressure_map")}),
                             ("emptied", {("MBF-IMPORT-SIDECAR-INVALID", f"pressure_map.{key}")
                                          for key in ("contract", "regions", "shear_model")}),
                             ("null", {("MBF-IMPORT-SIDECAR-INVALID", "pressure_map")})):
        changed = deepcopy(sidecar)
        if change == "deleted":
            del changed["pressure_map"]
        else:
            changed["pressure_map"] = {} if change == "emptied" else None
        result = import_caepipe_mbf(data, sidecar=changed, material_provenance=INVENTED, source_label=change,
                                    pressure_closure=CLOSURE)
        assert refusals(result) == expected, change
    result = import_caepipe_mbf(data, sidecar=legacy(render_caepipe_mbf_si(model, title="GUARD"), data),
                                material_provenance=INVENTED, source_label="legacy",
                                pressure_closure=CLOSURE)
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", "pressure_map")}


def test_a_restored_pressure_state_on_a_file_without_loads():
    # Codex VER-002 r9 (P2): a file without LOADS and a complete hash-bound sidecar. An empty pressure_map restores
    # the file as pressure-free; the writer's empty pressure state (the contract, no regions) restores the contract
    # with no region; a region the file cannot carry is refused by name. Before r9 the last two raised KeyError.
    data = importer_tests.mbf("F10,KA,MM1,PP1", "T20,X1000", loads=None)
    plain = importer_tests.run(data)
    assert plain.status == "imported" and "pressure_contract" not in plain.model
    contract = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
    exact = {"version": "3.0.0", "mode": "exact_pressure_v3"}
    free = import_caepipe_mbf(data, sidecar=complete_sidecar(data, plain.model, plain.id_map, pressure_map={
        "contract": "none", "shear_model": "euler_bernoulli", "regions": []}), material_provenance=INVENTED,
        source_label="free")
    assert free.status == "imported" and "pressure_contract" not in free.model
    empty = import_caepipe_mbf(data, sidecar=complete_sidecar(data, plain.model, plain.id_map, pressure_map={
        "contract": exact, "shear_model": "euler_bernoulli", "regions": []}), material_provenance=INVENTED,
        source_label="empty")
    assert empty.status == "imported", empty.diagnostics
    assert empty.model["pressure_contract"] == contract and empty.model["load_cases"] == []
    terminals = [{"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": "invented"}
                 for node in ("node:10", "node:20")]
    conflicting = import_caepipe_mbf(data, sidecar=complete_sidecar(data, plain.model, plain.id_map, pressure_map={
        "contract": exact, "shear_model": "euler_bernoulli", "regions": [
            {"id": "region:r", "members": ["pipe:10-20"], "terminals": terminals}]}),
        material_provenance=INVENTED, source_label="conflicting")
    assert refusals(conflicting) == {("MBF-IMPORT-SIDECAR-CONFLICT", "pressure_map")}


# --- VER-002 r10: the sidecar format ------------------------------------------------------------------------------------
# Invented fixture produced by running main's tranche-1 writer (si_writer.py at 42a86df92f) on the invented bent L of
# test_caepipe_mbf_si_writer.bent_l with its far anchor's family set to anchor: the MBF text and the legacy sidecar
# exactly as that writer and run_pilot wrote them (no sidecar_format, no pressure_map). Not engineering data.
LEGACY_MBF_TEXT = (
    'INVENTED LEGACY L\r\n'
    'VERSION\r\n'
    '15.10\r\n'
    'OPTIONS\r\n'
    'HGRA,SI\r\n'
    'OPTEMP\r\n'
    '20\r\n'
    'MATERIAL\r\n'
    'M1,0,0.25,1,1,CS,mat:CS\r\n'
    'M1,20,200000,1.2e-05\r\n'
    'M1,120,200000,1.2e-05\r\n'
    'PIPE\r\n'
    'P1,168.3,7.11,0,0,0,0,0,0\r\n'
    'LOADS\r\n'
    'L1,120,0,0\r\n'
    'LAYOUT\r\n'
    'F10,KA,MM1,PP1,CL=L1\r\n'
    'T20,KI,B250,X4000,CFF=1\r\n'
    'T30,KA,Z3000\r\n'
)
LEGACY_SIDECAR = json.loads(
    '{"element_map":{"BEND":{"bend":{"angle_deg":90.0,"caepipe_row_node":"20","component":"bend-1","flexibility_fac'
    'tor":1.0,"radius_mm":250.0,"tangent_intersection_mm":[4000.0,0.0,0.0]},"from":"20A","layout_order":1,"layout_r'
    'ow":1,"load":"L1","material":"M1","reversed":false,"section":"P1","to":"20B"},"S1":{"from":"10","layout_order"'
    ':0,"layout_row":1,"load":"L1","material":"M1","reversed":false,"section":"P1","to":"20A","y_reference":{"x":0.'
    '0,"y":1.0,"z":0.0}},"S2":{"from":"20B","layout_order":2,"layout_row":2,"load":"L1","material":"M1","reversed":'
    'false,"section":"P1","to":"30","y_reference":{"x":0.0,"y":1.0,"z":0.0}}},"load_case_map":{"case:th":"Expansion'
    ' (T1)"},"load_map":{"case:th/t:BEND":{"caepipe_element":{"bend":{"angle_deg":90.0,"caepipe_row_node":"20","com'
    'ponent":"bend-1","flexibility_factor":1.0,"radius_mm":250.0,"tangent_intersection_mm":[4000.0,0.0,0.0]},"from"'
    ':"20A","layout_order":1,"layout_row":1,"load":"L1","material":"M1","reversed":false,"section":"P1","to":"20B"}'
    ',"caepipe_load_case":"Expansion '
    '(T1)","category":"thermal","direction":"global_x","load_case":"case:th","mbf":"LOADS L1 Temperature 1","target'
    '":{"pipe":"BEND"}},"case:th/t:S1":{"caepipe_element":{"from":"10","layout_order":0,"layout_row":1,"load":"L1",'
    '"material":"M1","reversed":false,"section":"P1","to":"20A","y_reference":{"x":0.0,"y":1.0,"z":0.0}},"caepipe_l'
    'oad_case":"Expansion (T1)","category":"thermal","direction":"global_x","load_case":"case:th","mbf":"LOADS L1 '
    'Temperature 1","target":{"pipe":"S1"}},"case:th/t:S2":{"caepipe_element":{"from":"20B","layout_order":2,"layou'
    't_row":2,"load":"L1","material":"M1","reversed":false,"section":"P1","to":"30","y_reference":{"x":0.0,"y":1.0,'
    '"z":0.0}},"caepipe_load_case":"Expansion '
    '(T1)","category":"thermal","direction":"global_x","load_case":"case:th","mbf":"LOADS L1 Temperature 1","target'
    '":{"pipe":"S2"}}},"material_map":{"M1":{"elastic_modulus_n_per_mm2":200000.0,"elements":["S1","BEND","S2"],"po'
    'isson_ratio":0.25,"swbpipe_material":"mat:CS","temperatures_c":[20.0,120.0],"thermal_expansion_mm_per_mm_per_c'
    '":1.2e-05,"weight_density_kgf_per_m3":0.0}},"mbf_sha256":"954b632cc24318f3f463118e9c459fe19055b215c5009bb79de1'
    'c50ed9c64062","node_map":{"A":"10","B":"30","T1":"20A","T2":"20B"},"section_map":{"P1":{"elements":["S1","BEND'
    '","S2"],"outside_diameter_mm":168.3,"wall_thickness_mm":7.11}},"support_map":{"SA":{"caepipe_kind":"anchor","f'
    'amily":null,"node":"10","restrained":["UX","UY","UZ","RX","RY","RZ"]},"SB":{"caepipe_kind":"anchor","family":"'
    'anchor","node":"30","restrained":["UX","UY","UZ","RX","RY","RZ"]}}}'
)


def test_a_legacy_sidecar_from_mains_writer_still_imports_bound():
    data = LEGACY_MBF_TEXT.encode("ascii")
    assert hashlib.sha256(data).hexdigest() == LEGACY_SIDECAR["mbf_sha256"]
    result = import_caepipe_mbf(data, sidecar=LEGACY_SIDECAR, material_provenance=INVENTED, source_label="legacy")
    assert result.status == "imported", result.diagnostics
    assert result.source["binding"] == "bound" and "pressure_contract" not in result.model
    assert [(support["id"], support.get("family")) for support in result.model["supports"]] == [
        ("SA", None), ("SB", "anchor")]
    assert [pipe["id"] for pipe in result.model["pipe_segments"]] == ["S1", "BEND", "S2"]
    assert result.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 1.0, "z": 0.0}
    # Main's writer recorded no pressure_map and never a null orientation: a legacy sidecar carrying either is
    # refused by path, as is any field main's writer never wrote.
    for path, change in (("pressure_map", lambda sidecar: sidecar.update(pressure_map=None)),
                         ("element_map[S1].y_reference", lambda sidecar: sidecar["element_map"]["S1"].update(
                             y_reference=None)),
                         ("support_map[SA].zz_unknown", lambda sidecar: sidecar["support_map"]["SA"].update(
                             zz_unknown=1)),
                         # Codex VER-002 r11 (P3): fields main's writer always recorded are required
                         ("section_map[P1].outside_diameter_mm", lambda sidecar: sidecar["section_map"].update(
                             P1={"wall_thickness_mm": 7.11, "elements": ["S1", "BEND", "S2"]})),
                         ("element_map[S1].layout_order", lambda sidecar: sidecar["element_map"]["S1"].pop(
                             "layout_order")),
                         ("load_map[case:th/t:S1].mbf", lambda sidecar: sidecar["load_map"]["case:th/t:S1"].pop(
                             "mbf"))):
        sidecar = deepcopy(LEGACY_SIDECAR)
        change(sidecar)
        result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label=path)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", path)}, (path, refusals(result))


def test_a_format_2_sidecar_with_a_deleted_pressure_state_or_orientation_is_refused():
    # Codex VER-002 r10 (P2). (1) A pressure-free file whose model carries a Timoshenko contract and no regions:
    # deleting or emptying pressure_map used to import bound without the contract, under Euler-Bernoulli. (2) A
    # straight authored with y_reference (0, 0, 1): deleting the field used to substitute the documented (0, 1, 0).
    # Format 2 records both always; each deletion is refused by path, nothing is defaulted.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X100,CFOR(0,-1000,0)")
    free = importer_tests.run(importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,0,0",))).model
    free["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3",
                                 "shear_model": "timoshenko_annulus_energy_v1"}
    for pipe in free["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
    for case in free["load_cases"]:
        case["pressure_regions"] = []
    data, sidecar = bound_parts(free)
    restored = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="contract")
    assert restored.model["pressure_contract"]["shear_model"] == "timoshenko_annulus_energy_v1"
    deleted = {key: value for key, value in sidecar.items() if key != "pressure_map"}
    result = import_caepipe_mbf(data, sidecar=deleted, material_provenance=INVENTED, source_label="deleted")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "pressure_map")}
    result = import_caepipe_mbf(data, sidecar={**sidecar, "pressure_map": {}}, material_provenance=INVENTED,
                                source_label="emptied")
    assert {path for _, path in refusals(result)} == {"pressure_map.contract", "pressure_map.regions",
                                                       "pressure_map.shear_model"}
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    model = importer_tests.run(importer_tests.mbf(*rows, pipe=("P1,168.3,7.11,0",), loads=("L1,20,10,0",)),
                               pressure_closure=CLOSURE).model
    model["pipe_segments"][0]["y_reference"] = {"x": 0.0, "y": 0.0, "z": 1.0}
    data, sidecar = bound_parts(model)
    restored = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="authored")
    assert restored.model["pipe_segments"][0]["y_reference"] == {"x": 0.0, "y": 0.0, "z": 1.0}
    del sidecar["element_map"]["pipe:10-20"]["y_reference"]
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="deleted")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "element_map[pipe:10-20].y_reference")}


def test_a_format_2_null_pressure_state_or_orientation_is_refused(monkeypatch):
    # Codex VER-002 r11 (P2). (1) The contract-only file: pressure_map replaced by null used to import bound without
    # the contract, under Euler-Bernoulli. (2) The authored (0, 0, 1): y_reference set to null used to restore the
    # documented (0, 1, 0). Format 2 records the effective value always and holds no null: both are refused by path.
    data, sidecar = bound_parts(contract_only_model())
    assert sidecar["pressure_map"]["shear_model"] == "timoshenko_annulus_energy_v1"
    result = import_caepipe_mbf(data, sidecar={**sidecar, "pressure_map": None}, material_provenance=INVENTED,
                                source_label="null")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "pressure_map")}
    data, sidecar = bound_parts(authored_orientation_model())
    entry = sidecar["element_map"]["pipe:10-20"]
    assert (entry["y_reference"], entry["y_reference_authored"]) == ({"x": 0.0, "y": 0.0, "z": 1.0}, True)
    entry["y_reference"] = None
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="null")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "element_map[pipe:10-20].y_reference")}
    # An unauthored straight records its effective orientation, the documented one, flagged unauthored.
    plain = writer_tests.cantilever()
    written = render_caepipe_mbf_si(plain, title="EFFECTIVE")
    assert [(entry["y_reference"], entry["y_reference_authored"]) for entry in written.element_map.values()] == [
        ({"x": 0.0, "y": 1.0, "z": 0.0}, False)]


# --- VER-002 r12: recorded values agree with what the MBF determines -------------------------------------------------
def test_a_degenerate_pipe_is_refused_by_name_before_its_orientation_is_derived():
    # Codex VER-002 r12 (P2): with no y_reference and coincident ends, the writer derived the documented local y of a
    # zero-length pipe and raised ZeroDivisionError. It refuses the pipe by name first, with and without pressure.
    rows = ("F10,KA,MM1,PP1,CL=L1", "T20,X1000,CFOR(0,-1,0),FLC=1")
    for loads, closure in ((("L1,20,0,0",), None), (("L1,20,10,0",), CLOSURE)):
        model = importer_tests.run(importer_tests.mbf(*rows, loads=loads), pressure_closure=closure).model
        del model["pipe_segments"][0]["y_reference"]
        model["nodes"][1]["position"] = dict(model["nodes"][0]["position"])
        written = render_caepipe_mbf_si(model, title="DEGENERATE")
        assert written.status == "refused", loads
        assert "MBF-SI-GEOMETRY-UNSUPPORTED" in {item["code"] for item in written.diagnostics
                                                 if item["severity"] == "error"}, loads


def test_a_recorded_direction_the_mbf_contradicts_is_refused():
    # Codex VER-002 r12 (P2): a weight's direction is the gravity axis (-Y in model axes: OPMISC admits only earth
    # gravity, and a Z-vertical file is rotated to Y vertical); a displacement's is the DOF the MBF's D comment moves.
    # A sidecar recording another is refused by load id, not replaced by the MBF's.
    weighted_rows = ("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    material = ("M1,7850,0.3,1,1,CS,invented steel", "M1,20,200000,1.2e-05")
    model = importer_tests.run(importer_tests.mbf(*weighted_rows, material=material)).model
    data, sidecar = bound_parts(model)
    key = next(key for key, entry in sidecar["load_map"].items() if entry["category"] == "weight")
    assert sidecar["load_map"][key]["direction"] == "global_y"
    sidecar["load_map"][key]["direction"] = "global_x"
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="weight")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", f"load_map[{key}].direction")}
    moved = importer_tests.run(importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000,CD(1,0,0)",
                                                  loads=("L1,120,0,0",))).model
    data, sidecar = bound_parts(moved)
    key = next(key for key, entry in sidecar["load_map"].items() if entry["category"] == "imposed_displacement")
    assert sidecar["load_map"][key]["direction"] == "UX"
    sidecar["load_map"][key]["direction"] = "UY"
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="displacement")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", f"load_map[{key}].direction")}


def test_an_unauthored_record_must_equal_its_derivation():
    # Codex VER-002 r12 (P3): y_reference_authored false means "derived, and equal to the derivation". On a pipe along
    # +X the documented local y is (0, 1, 0); a record of (0, 0, 1) flagged unauthored is refused. So is an
    # unauthored family that is not the solve's inference (a six-DOF anchor recorded as a guide).
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000")
    model = importer_tests.run(data).model
    del model["pipe_segments"][0]["y_reference"]
    data, sidecar = bound_parts(model)
    entry = sidecar["element_map"]["pipe:10-20"]
    assert (entry["y_reference"], entry["y_reference_authored"]) == ({"x": 0.0, "y": 1.0, "z": 0.0}, False)
    assert import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED,
                              source_label="derived").status == "imported"
    changed = deepcopy(sidecar)
    changed["element_map"]["pipe:10-20"]["y_reference"] = {"x": 0.0, "y": 0.0, "z": 1.0}
    result = import_caepipe_mbf(data, sidecar=changed, material_provenance=INVENTED, source_label="claimed")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", "element_map[pipe:10-20].y_reference")}
    changed = deepcopy(sidecar)
    support = next(iter(changed["support_map"]))
    changed["support_map"][support].update(family="guide", family_authored=False)
    result = import_caepipe_mbf(data, sidecar=changed, material_provenance=INVENTED, source_label="family")
    assert ("MBF-IMPORT-SIDECAR-INVALID", f"support_map[{support}].family") in refusals(result)


# --- VER-002 r13: the regeneration identity and the targeted fixes ---------------------------------------------------
def guarded(model: dict, change, label: str, **options):
    """`model`'s writer sidecar changed by `change`, its bound import, and the regeneration identity alone on it
    against the unchanged import (None when that identity would accept it)."""
    data, sidecar = bound_parts(model)
    base = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="base")
    assert base.status == "imported" and regeneration_difference(sidecar, base) is None, base.diagnostics
    changed = deepcopy(sidecar)
    change(changed)
    result = import_caepipe_mbf(data, sidecar=changed, material_provenance=INVENTED, source_label=label, **options)
    return result, regeneration_difference(changed, base), base


def test_every_comparison_model_regenerates_its_sidecar_identically_or_with_identity_only_exclusions(trips):
    # The writer's sidecar of every model is what the writer regenerates from its bound import. Only the identity of a
    # primitive the MBF sums is set aside (REQ-016): its entries differ only there, each set-aside entry has its
    # named report and nothing else is set aside or differs. Only the summed-load model has any.
    excluded_models = set()
    for name in REVERSIBLE:
        trip = trips[name]
        assert trip.bound.status == "imported", (name, trip.bound.diagnostics)
        assert regeneration_difference(trip.sidecar, trip.bound) is None, name
        assert differing_entries(trip.sidecar, trip.bound) == set_aside(trip.bound) == reported(trip.bound), name
        if groups(trip.bound):
            excluded_models.add(name)
    assert excluded_models == {"writer-cancelling-forces"}
    assert len(REVERSIBLE) == 24  # tranche 3 added writer-springs; the bend SIF pair added writer-bent-l-sifs


def test_a_straight_recorded_as_a_bend_is_refused_on_both_routes():
    # Codex VER-002 r13 (finding 1): a straight's orientation fields replaced by bend {"component": "b"} imported
    # silently with the documented orientation. The entry's variant must be the element's.
    def change(sidecar):
        entry = sidecar["element_map"]["S1"]
        sidecar["element_map"]["S1"] = {"from": entry["from"], "to": entry["to"], "reversed": entry["reversed"],
                                        "bend": {"component": "b"}}
    for options in ({}, {"rebind_by_node_number": True}):
        result, alone, _ = guarded(writer_tests.bent_l(), change, "straight as bend", **options)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", "element_map[S1].bend")}, options
        assert alone is not None and alone[0] == "element_map[S1].bend"


def test_a_load_target_of_another_kind_is_refused_by_path():
    # Codex VER-002 r13 (finding 2): a force targeting {"pipe": <node id>} imported as the node's force.
    def change(sidecar):
        sidecar["load_map"]["case:tip/tip"]["target"] = {"pipe": "N1"}
    result, alone, _ = guarded(writer_tests.cantilever(), change, "force on a pipe")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "load_map[case:tip/tip].target")}
    assert alone is not None and alone[0].startswith("load_map[case:tip/tip].target")


def test_ambiguous_reverse_mappings_are_refused_before_assembly():
    # Codex VER-002 r13 (finding 3): two element entries with one endpoint pair, or two cases with one label,
    # overwrote each other silently. Each is refused by path, on the rebind route too for elements.
    def element(sidecar):
        sidecar["element_map"]["S1-copy"] = deepcopy(sidecar["element_map"]["S1"])

    def label(sidecar):
        case, text = next(iter(sidecar["load_case_map"].items()))
        sidecar["load_case_map"]["case:copy"] = text

    for options in ({}, {"rebind_by_node_number": True}):
        result, alone, _ = guarded(writer_tests.bent_l(), element, "pair", **options)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-AMBIGUOUS", "element_map[S1-copy]")}, options
        assert alone is not None
    result, alone, _ = guarded(writer_tests.bent_l(), label, "label")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-AMBIGUOUS", "load_case_map[case:copy]")}
    assert alone is not None


def test_legacy_retained_duplicates_must_be_what_they_repeat():
    # Codex VER-002 r13 (finding 4): a legacy primitive's caepipe_node and caepipe_load_case were not checked. Each
    # repetition, and each element's MBF record names, must be the value it repeats.
    trip = Trip("legacy-force", writer_tests.cantilever(), "LEGACY", 20.0)
    base = legacy(trip.writer, trip.data)
    assert import_caepipe_mbf(trip.data, sidecar=base, material_provenance=INVENTED,
                              source_label="legacy").status == "imported"
    key = "case:tip/tip"
    for path, code, change in (
            (f"load_map[{key}].caepipe_node", "INVALID", lambda s: s["load_map"][key].update(caepipe_node="10")),
            (f"load_map[{key}].caepipe_load_case", "INVALID",
             lambda s: s["load_map"][key].update(caepipe_load_case="Expansion (T1)")),
            (f"load_map[{key}].mbf", "INVALID", lambda s: s["load_map"][key].update(mbf="MOM")),
            ("element_map[S1].material", "CONFLICT", lambda s: s["element_map"]["S1"].update(material="M9"))):
        sidecar = deepcopy(base)
        change(sidecar)
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label=path)
        assert (f"MBF-IMPORT-SIDECAR-{code}", path) in refusals(result), (path, refusals(result))
    data = LEGACY_MBF_TEXT.encode("ascii")
    sidecar = deepcopy(LEGACY_SIDECAR)
    sidecar["load_map"]["case:th/t:S1"]["caepipe_element"] = deepcopy(sidecar["element_map"]["S2"])
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="element")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "load_map[case:th/t:S1].caepipe_element")}


def test_a_direction_alias_matches_and_regenerates_as_its_axis():
    # Codex VER-002 r13 (finding 5): force direction UY, which the solve reads as global_y, was refused. The match
    # and the regeneration identity read a direction as its axis; a force recorded with a rotation is refused.
    for alias in ("UY", "uy", "GLOBAL_Y"):
        result, alone, base = guarded(writer_tests.cantilever(),
                                      lambda s, a=alias: s["load_map"]["case:tip/tip"].update(direction=a), alias)
        assert result.status == "imported", (alias, result.diagnostics)
        assert result.model == base.model and alone is None, alias
    result, _, _ = guarded(writer_tests.cantilever(),
                           lambda s: s["load_map"]["case:tip/tip"].update(direction="RY"), "rotation")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "load_map[case:tip/tip].direction")}


def test_the_regeneration_identity_refuses_what_no_other_check_reads():
    # A difference only the guard of record sees: the supports' DOFs listed in another order (every DOF check reads
    # them as a set), refused at the first differing path; and a model the writer does not write (a bend without a
    # flexibility factor, which the MBF gives without FF), refused naming the writer's code.
    def permuted(sidecar):
        entry = sidecar["support_map"]["anchor:N0"]
        entry["restrained"] = list(reversed(entry["restrained"]))
    result, alone, _ = guarded(writer_tests.cantilever(), permuted, "order")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", "support_map[anchor:N0].restrained[0]")}
    assert alone == ("support_map[anchor:N0].restrained[0]", '"RZ"', '"UX"')
    message = next(item["message"] for item in result.diagnostics if item["severity"] == "error")
    assert "rebind_by_node_number" in message
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KI,B50,X1000", "T30,KA,Y1000")
    plain = importer_tests.run(data)
    assert plain.status == "imported", plain.diagnostics
    sidecar = complete_sidecar(data, plain.model, plain.id_map)
    bound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="no factor")
    assert refusals(bound) == {("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", "sidecar")}
    assert "MBF-SI-BEND-FLEXIBILITY-MISSING" in bound.diagnostics[-1]["message"]


def test_the_rebind_route_is_taken_on_request_whatever_the_digest():
    # Design §15: rebind_by_node_number applies the sidecar by CAEPIPE name and marks every restored identity
    # unconfirmed, also for a sidecar whose digest is this file's; it is where a sidecar the exact-digest route
    # refuses can be applied. The rebind route does not run the regeneration identity.
    data, sidecar = bound_parts(writer_tests.cantilever())
    entry = sidecar["support_map"]["anchor:N0"]
    entry["restrained"] = list(reversed(entry["restrained"]))
    assert import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED,
                              source_label="bound").status == "refused"
    rebound = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, rebind_by_node_number=True,
                                 source_label="rebound")
    assert rebound.status == "imported", rebound.diagnostics
    assert rebound.source["binding"] == "rebound_unconfirmed" == rebound.id_map["binding"]
    assert {"N0", "N1", "S1", "anchor:N0", "mat:CS"} <= set(rebound.id_map["rebound_unconfirmed"])
    assert not any(item["code"] == "MBF-IMPORT-SIDECAR-NOT-REPRODUCED" for item in rebound.diagnostics)
    # It never claims a bound or confirmed import: every restored id is unconfirmed, and no text says bound.
    restored = {item["id"] for name in ("nodes", "pipe_segments", "supports", "materials")
                for item in rebound.model[name]}
    assert restored <= set(rebound.id_map["rebound_unconfirmed"])
    texts = rebound.model_bytes().decode() + rebound.report_bytes().decode()
    assert "bound to the SWBPIPE writer sidecar" not in texts and "(bound import)" not in texts
    assert '"binding": "bound"' not in texts + rebound.id_map_bytes().decode()


def test_a_declared_origin_other_than_the_recorded_one_is_refused():
    # A bound sidecar's recorded origin is the material's; the writer reproduces it from the certification the import
    # keeps. A caller's declaration of another origin for that material is refused, neither being preferred.
    model = writer_tests.cantilever()
    model["materials"][0]["provenance_record"] = {"source_name": "invented",
                                                 "contributor_certification": "invented non-engineering example",
                                                 "redistribution_status": "public_permissive"}
    data, sidecar = bound_parts(model)
    assert sidecar["material_map"]["M1"]["origin"] == "project_invented"
    for provenance in (None, INVENTED):
        result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=provenance, source_label="origin")
        assert result.status == "imported", result.diagnostics
    own = {"*": {"origin": "user_supplied", "source_name": "my own invented data"}}
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=own, source_label="declared")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", "material_map[M1].origin")}


# --- VER-002 r14: a summed primitive's entries are compared but for its identity ------------------------------------
def test_a_restored_id_that_a_summed_primitive_would_also_take_is_refused(monkeypatch):
    # Codex VER-002 r14 (finding 1): with a second Y force on the tip and a +500 N X force named
    # load:concentrated_force:N1:global_y, the MBF prints FOR(500,-7000,0). The X primitive restores that id and the
    # summed Y primitive forms it too: two primitives under one id, which the solve refuses (DUPLICATE_ID). The
    # import refuses the collision by name before the model is assembled.
    model = writer_tests.cantilever()
    model["load_cases"][0]["primitive_loads"] += [
        writer_tests.force("tip2", "N1", "global_y", -1000.0),
        writer_tests.force("load:concentrated_force:N1:global_y", "N1", "global_x", 500.0)]
    refused = render_caepipe_mbf_si(model, title="GUARD")  # the total preflight refuses it (VER-002 r3)
    assert any(item["code"] == "MBF-SI-PREFLIGHT-REFUSED" and "DUPLICATE_ID" in item["message"]
               for item in refused.diagnostics)
    data, sidecar = bound_parts(model, monkeypatch)  # the preflight bypassed, to reach the import
    assert "CFOR(500,-7000,0)" in data.decode()
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="collision")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT",
                                 "load_map[case:tip/load:concentrated_force:N1:global_y]")}
    assert "DUPLICATE_ID" in result.diagnostics[-1]["message"] and result.model is None


def test_summed_thermal_entries_keep_their_common_direction_or_are_refused():
    # Codex VER-002 r14 (finding 2): two thermal entries on one straight, both global_z, imported as one global_x
    # thermal load. The formed load keeps a direction its entries agree on (an alias of one axis agrees); entries that
    # disagree are refused by name, since a sum across directions is not meaningful.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,120,0,0",))
    data, sidecar = bound_parts(importer_tests.run(data).model)
    (key, entry), = sidecar["load_map"].items()
    case = entry["load_case"]
    for directions, expected in ((("global_z", "global_z"), "global_z"), (("global_z", "UZ"), "global_z"),
                                 (("global_z", "global_x"), None)):
        changed = deepcopy(sidecar)
        changed["load_map"][key]["direction"] = directions[0]
        changed["load_map"][f"{case}/t2"] = {**deepcopy(entry), "direction": directions[1]}
        result = import_caepipe_mbf(data, sidecar=changed, material_provenance=INVENTED, source_label="thermal")
        if expected is None:
            assert refusals(result) == {("MBF-IMPORT-SIDECAR-CONFLICT", f"load_map[{case}/t2].direction")}
            continue
        assert result.status == "imported", (directions, result.diagnostics)
        (load,) = [load for case_ in result.model["load_cases"] for load in case_["primitive_loads"]]
        assert load["direction"] == expected and load["id"] == "load:thermal:pipe:10-20"
        assert groups(result) == {f"{case}/load:thermal:pipe:10-20": [key, f"{case}/t2"]}
        assert regeneration_difference(changed, result) is None
        assert set_aside(result) == reported(result)



# --- VER-002 r15: a legacy sidecar's repetitions of the file agree with it ---------------------------------------
def printed_text(value: float) -> str:
    """The writer's printed form of `value` (si_writer _num: ten significant digits, 0 for 0)."""
    return "0" if value == 0 else format(value, ".10g")


def legacy_replacements(sidecar: dict) -> list[tuple[str, dict, str]]:
    """Each MBF-determined field of a legacy `sidecar` (Design §15's table) set to a different valid value, with what
    the import must do. A printed number goes just outside (1.01) and just inside (0.99) the writer's printing bound,
    half a unit in the tenth significant digit; a derived one (TI, angle) moves 1e-6 relative (outside) and four
    binary64 steps (inside)."""
    from core.handoff.caepipe_mbf.importer import tenth_digit
    found: list[tuple[str, dict, str]] = []

    def at(label: str, expected: str, change) -> None:
        mutated = deepcopy(sidecar)
        change(mutated)
        if label.startswith("element_map"):  # a primitive's caepipe_element repeats its element entry: kept in step
            for entry in mutated["load_map"].values():
                if "caepipe_element" in entry:
                    entry["caepipe_element"] = deepcopy(mutated["element_map"][entry["target"]["pipe"]])
        found.append((label, mutated, expected))

    def number(label: str, holder, key, derived: bool = False) -> None:
        value = holder(sidecar)[key]
        if derived:  # a zero coordinate is placed by no printed offset: its bound is 0, and it has no inside
            inside = value
            for _ in range(4 if value else 0):
                inside = math.nextafter(inside, math.inf)
            outside = value * (1 + 1e-6) if value else 1e-6
        else:
            text = printed_text(value)
            half = tenth_digit(Fraction(text))
            inside = float(Fraction(text) + Fraction(99, 100) * half) if half else value
            outside = float(Fraction(text) + Fraction(101, 100) * half) if half else 1e-9
        at(f"{label}: outside", "refused", lambda s: holder(s).__setitem__(key, outside))
        if inside != value:
            at(f"{label}: inside", "imported", lambda s: holder(s).__setitem__(key, inside))

    labels = {"Sustained (W+P)": "Expansion (T1)", "Expansion (T1)": "Sustained (W+P)"}
    for swb_id in sidecar["node_map"]:
        at(f"node_map[{swb_id}]", "refused", lambda s, k=swb_id: s["node_map"].update({k: "99"}))
    for swb_id, entry in sidecar["element_map"].items():
        where = f"element_map[{swb_id}]"
        for key, value in (("from", "99"), ("to", "99"), ("layout_order", entry["layout_order"] + 1),
                           ("layout_row", entry["layout_row"] + 1), ("material", "M9"), ("section", "P9"),
                           ("load", "L9")):
            at(f"{where}.{key}", "refused", lambda s, k=swb_id, f=key, v=value: s["element_map"][k].update({f: v}))
        if "bend" in entry:
            bend = lambda s, k=swb_id: s["element_map"][k]["bend"]  # noqa: E731
            at(f"{where}.bend.caepipe_row_node", "refused", lambda s, b=bend: b(s).update(caepipe_row_node="99"))
            for key in ("radius_mm", "flexibility_factor"):
                number(f"{where}.bend.{key}", bend, key)
            number(f"{where}.bend.angle_deg", bend, "angle_deg", derived=True)
            for axis in range(3):
                number(f"{where}.bend.tangent_intersection_mm[{axis}]",
                       lambda s, b=bend: b(s)["tangent_intersection_mm"], axis, derived=True)
    for swb_id, entry in sidecar["support_map"].items():
        where = f"support_map[{swb_id}]"
        at(f"{where}.node", "refused", lambda s, k=swb_id: s["support_map"][k].update(node="99"))
        at(f"{where}.restrained", "refused", lambda s, k=swb_id: s["support_map"][k].update(restrained=["UX"]))
        other = "restraint" if entry["caepipe_kind"] == "anchor" else "anchor"
        at(f"{where}.caepipe_kind", "refused", lambda s, k=swb_id, v=other: s["support_map"][k].update(caepipe_kind=v))
    for swb_id, label in sidecar["load_case_map"].items():
        at(f"load_case_map[{swb_id}]", "refused", lambda s, k=swb_id, v=labels[label]: s["load_case_map"].update({k: v}))
    for name, entry in sidecar["material_map"].items():
        material = lambda s, k=name: s["material_map"][k]  # noqa: E731
        for key in ("weight_density_kgf_per_m3", "poisson_ratio", "elastic_modulus_n_per_mm2",
                    "thermal_expansion_mm_per_mm_per_c"):
            number(f"material_map[{name}].{key}", material, key)
        for index in range(len(entry["temperatures_c"])):
            number(f"material_map[{name}].temperatures_c[{index}]", lambda s, m=material: m(s)["temperatures_c"], index)
        at(f"material_map[{name}].temperatures_c (one fewer)", "refused",
           lambda s, m=material: m(s).update(temperatures_c=m(s)["temperatures_c"][:-1] or [20.0, 20.0]))
        at(f"material_map[{name}].elements", "refused", lambda s, m=material: m(s).update(elements=["zz"]))
    for name in sidecar["section_map"]:
        section = lambda s, k=name: s["section_map"][k]  # noqa: E731
        for key in ("outside_diameter_mm", "wall_thickness_mm"):
            number(f"section_map[{name}].{key}", section, key)
        at(f"section_map[{name}].elements", "refused", lambda s, m=section: m(s).update(elements=["zz"]))
    for key, entry in sidecar["load_map"].items():
        where, load = f"load_map[{key}]", (lambda s, k=key: s["load_map"][k])  # noqa: E731
        at(f"{where}.caepipe_load_case", "refused",
           lambda s, l=load: l(s).update(caepipe_load_case=labels[l(s)["caepipe_load_case"]]))
        other = next(item for item in ("weight", "thermal", "concentrated_force") if item != entry["category"])
        at(f"{where}.category", "refused", lambda s, l=load, v=other: l(s).update(category=v))
        target = {"node": "zz"} if "node" in entry["target"] else {"pipe": "zz"}
        at(f"{where}.target", "refused", lambda s, l=load, v=target: l(s).update(target=v))
        at(f"{where}.mbf", "refused", lambda s, l=load: l(s).update(mbf="zz"))
        if entry["category"] != "thermal":  # a thermal direction is authored (the MBF carries none)
            at(f"{where}.direction", "refused", lambda s, l=load: l(s).update(
                direction="global_z" if l(s)["direction"] not in ("global_z", "UZ") else "global_x"))
        if "caepipe_node" in entry:
            at(f"{where}.caepipe_node", "refused", lambda s, l=load: l(s).update(caepipe_node="99"))
        else:
            others = [item for item in sidecar["element_map"] if item != entry["target"]["pipe"]]
            at(f"{where}.caepipe_element", "refused", lambda s, l=load, o=others[0]: l(s).update(
                caepipe_element=deepcopy(s["element_map"][o])))
    return found


def legacy_bases() -> dict[str, tuple[bytes, dict]]:
    """The legacy fixture from main's writer (a bend and thermal loads), and legacy sidecars of the invented weighted
    cantilever (weight density, a force at the tip) as main's writer recorded them."""
    model = weighted()
    model["load_cases"][0]["primitive_loads"].append(writer_tests.force("tip2", "N2", "global_x", 250.0))
    trip = Trip("legacy-weighted", model, "LEGACY", 20.0)
    return {"fixture": (LEGACY_MBF_TEXT.encode("ascii"), deepcopy(LEGACY_SIDECAR)),
            "weighted": (trip.data, legacy(trip.writer, trip.data))}


@pytest.mark.parametrize("base_name", ["fixture", "weighted"])
def test_every_mbf_determined_legacy_field_is_checked_against_the_file(base_name):
    # Codex VER-002 r15 (P2): a legacy sidecar's physical numbers were not checked (a 1 mm wall or a 1000 MPa modulus
    # imported the MBF's values silently). Each value it repeats of the file is refused by path when it disagrees, a
    # printed number beyond half a unit in its last printed digit; within that bound it imports the same model.
    data, base = legacy_bases()[base_name]
    run = lambda sidecar, label: import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED,  # noqa: E731
                                                    source_label=label)
    expected = run(base, "base")  # the positive control
    assert expected.status == "imported" and expected.source["binding"] == "bound", expected.diagnostics
    replaced = legacy_replacements(base)
    for label, sidecar, outcome in replaced:
        result = run(sidecar, label)
        if outcome == "refused":
            assert result.status == "refused", label
            assert all(code.startswith("MBF-IMPORT-") for _, code in [(0, item["code"]) for item in result.diagnostics
                                                                         if item["severity"] == "error"]), label
        else:
            assert result.status == "imported", (label, refusals(result))
            assert result.model == expected.model, label
    outcomes = {outcome for _, _, outcome in replaced}
    assert outcomes == {"refused", "imported"} and len(replaced) > 40


def test_codexs_legacy_section_and_modulus_conflicts_are_refused_by_path():
    trip = Trip("legacy-numbers", writer_tests.cantilever(), "LEGACY", 20.0)
    base = legacy(trip.writer, trip.data)
    for path, change in (("section_map[P1].wall_thickness_mm",
                          lambda s: s["section_map"]["P1"].update(wall_thickness_mm=1)),
                         ("material_map[M1].elastic_modulus_n_per_mm2",
                          lambda s: s["material_map"]["M1"].update(elastic_modulus_n_per_mm2=1000))):
        sidecar = deepcopy(base)
        change(sidecar)
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label=path)
        assert ("MBF-IMPORT-SIDECAR-CONFLICT", path) in refusals(result), refusals(result)


def test_a_mixed_legacy_thermal_set_keeps_the_agreed_direction_and_says_so():
    # Codex VER-002 r15 (P3): two legacy thermal entries on one straight, one global_z and one without a direction.
    # The formed load keeps global_z, and the report on the entry without one says it takes global_z.
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,120,0,0",))
    written = render_caepipe_mbf_si(importer_tests.run(data).model, title="MIXED")
    data = written.text.encode("ascii")
    sidecar = legacy(written, data)
    (key, entry), = sidecar["load_map"].items()
    sidecar["load_map"][key]["direction"] = "global_z"
    other = {name: value for name, value in deepcopy(entry).items() if name != "direction"}
    sidecar["load_map"][f"{entry['load_case']}/t2"] = other
    result = import_caepipe_mbf(data, sidecar=sidecar, material_provenance=INVENTED, source_label="mixed")
    assert result.status == "imported", result.diagnostics
    (load,) = [load for case in result.model["load_cases"] for load in case["primitive_loads"]]
    assert load["direction"] == "global_z"
    (report,) = [item for item in result.loss_report if item["code"] == "MBF-IMPORT-DIRECTION-UNRECOVERABLE"]
    assert report["location"] == f"load_map[{entry['load_case']}/t2].direction"
    assert "takes global_z" in report["reason"] and "global_x" not in report["reason"]


# --- the sidecar's self-digest (Design §15) --------------------------------------------------------------------------
# A tiny invented sidecar-shaped object (it need not pass the parser), with its canonical form and SHA-256 written by
# hand: keys sorted by UTF-16 code units (U+1F600 is D83D DE00 and sorts before U+E000, against code-point order),
# numbers as ECMAScript writes them, and the escapes RFC 8785 requires and no others (U+007F and U+00E9 are literal).
TINY_SIDECAR = {
    "sidecar_format": 3,
    "node_map": {"N2": "20", "N10": "10"},
    "element_map": {"S1": {"from": "10", "to": "20", "reversed": False,
                           "y_reference": {"z": -1e-7, "y": 1.0, "x": 0.000001}, "y_reference_authored": True}},
    "load_case_map": {"\U0001f600": "smile", "\ue000": "private use", "case:\u00e9": "tab\there \"q\" \\ \u001f\u007f"},
    "material_map": {"M1": {"origin": None, "big": 1e21, "neg": -0.0, "int": 2, "frac": 123.456, "e20": 1.5e20}},
    "mbf_sha256": "0" * 64,
    "sidecar_sha256": "f" * 64,  # excluded from what it digests
}
TINY_CANONICAL = (
    '{"element_map":{"S1":{"from":"10","reversed":false,"to":"20","y_reference":{"x":0.000001,"y":1,'
    '"z":-1e-7},"y_reference_authored":true}},"load_case_map":{"case:\u00e9":"tab\\there \\"q\\" \\\\ '
    '\\u001f\u007f","\U0001f600":"smile","\ue000":"private use"},"material_map":{"M1":{"big":1e+21,'
    '"e20":150000000000000000000,"frac":123.456,"int":2,"neg":0,"origin":null}},"mbf_sha256":"' + "0" * 64
    + '","node_map":{"N10":"10","N2":"20"},"sidecar_format":3}').encode("utf-8")
TINY_DIGEST = "763f7c67deb82dc29eb03ae4f4fa2efda20ac4b9ea459d180e22605fe0b0ffd3"


def reordered(value: Any) -> Any:
    """`value` with every object's members in reverse order."""
    if isinstance(value, dict):
        return {key: reordered(value[key]) for key in reversed(list(value))}
    return [reordered(item) for item in value] if isinstance(value, list) else value


def test_the_self_digest_is_the_sha256_of_the_hand_derived_canonical_form():
    assert hashlib.sha256(TINY_CANONICAL).hexdigest() == TINY_DIGEST
    assert sidecar_canonical_bytes(TINY_SIDECAR) == TINY_CANONICAL
    assert sidecar_self_digest(TINY_SIDECAR) == TINY_DIGEST == canon.self_digest(TINY_SIDECAR)
    # Key order is not content: the sidecar reaches the importer as a parsed JSON object, whose members JSON leaves
    # unordered, and the canonical form sorts them. Likewise 1 and 1.0 are one binary64 value.
    assert sidecar_self_digest(reordered(TINY_SIDECAR)) == TINY_DIGEST
    assert sidecar_self_digest({**TINY_SIDECAR, "material_map": {"M1": {
        **TINY_SIDECAR["material_map"]["M1"], "int": 2.0}}}) == TINY_DIGEST
    # A value with no canonical form is named by its path.
    from core.handoff.caepipe_mbf.si_writer import SidecarNotCanonical
    for change, path in ((lambda s: s["node_map"].update(N2="\ud800"), "node_map[N2]"),
                         (lambda s: s["element_map"]["S1"]["y_reference"].update(x=math.inf),
                          "element_map[S1].y_reference.x"),
                         (lambda s: s["material_map"]["M1"].update(int=2 ** 53 + 1), "material_map[M1].int"),
                         (lambda s: s["load_case_map"].update({1: "x"}), "load_case_map")):
        sidecar = deepcopy(TINY_SIDECAR)
        change(sidecar)
        with pytest.raises(SidecarNotCanonical) as raised:
            sidecar_canonical_bytes(sidecar)
        assert raised.value.path == path
        with pytest.raises((ValueError, UnicodeEncodeError)):
            canon.self_digest(sidecar)


def test_every_writer_sidecar_carries_its_self_digest_and_imports_verified(trips):
    # The writer's sidecar of every comparison model is format 3, its self-digest the test's own RFC 8785 digest of
    # it, and it imports as given, verified. Written as JSON in another layout and read back, it is the same content
    # and imports the same model. With its members reordered it has the same self-digest: JSON leaves an object's
    # members unordered, and the canonical form sorts them, so a reorder is not an edit.
    for name in REVERSIBLE:
        trip = trips[name]
        assert trip.sidecar["sidecar_format"] == SIDECAR_FORMAT == 3, name
        assert trip.sidecar["sidecar_sha256"] == canon.self_digest(trip.sidecar), name
        assert sidecar_self_digest(reordered(trip.sidecar)) == trip.sidecar["sidecar_sha256"], name
        assert trip.bound.status == "imported" and trip.bound.source["sidecar_self_digest"] == "verified", name
        assert "MBF-IMPORT-SIDECAR-NO-SELF-DIGEST" not in {item["code"] for item in trip.bound.diagnostics}, name
        text = json.dumps(trip.sidecar, indent=1, ensure_ascii=False)
        again = import_as_given(trip.data, sidecar=json.loads(text), material_provenance=INVENTED, source_label=name)
        assert again.model_bytes() == trip.bound.model_bytes(), name
    data, sidecar = bound_parts(writer_tests.bent_l())
    result = import_as_given(data, sidecar=json.loads(json.dumps(reordered(sidecar))), material_provenance=INVENTED,
                             source_label="reordered")
    assert result.status == "imported", result.diagnostics
    assert (result.source["binding"], result.source["sidecar_self_digest"]) == ("bound", "verified")


def test_an_edited_or_partial_sidecar_is_refused_by_its_self_digest_on_every_route():
    # A one-byte edit of the written JSON (a case label), and a partial sidecar (one map dropped), each refused by
    # name before the parser reads it: on the exact-digest route, on the explicit rebind route, and on the route of
    # another MBF, where the sidecar would otherwise be left unapplied. Nothing is bound or rebound.
    data, sidecar = bound_parts(writer_tests.cantilever())
    text = json.dumps(sidecar)
    assert "Sustained (W+P)" in text
    edited = text.replace("Sustained (W+P)", "Sustained (W+Q)", 1)
    assert len(edited) == len(text) and sum(a != b for a, b in zip(text, edited)) == 1
    partial = {key: value for key, value in sidecar.items() if key != "load_case_map"}
    other = data.replace(b"GUARD", b"OTHER")
    assert other != data
    for label, given in (("one byte", json.loads(edited)), ("partial", partial),
                         ("stale digest", {**sidecar, "sidecar_sha256": "0" * 64})):
        for route, mbf, options in (("bound", data, {}), ("rebind", data, {"rebind_by_node_number": True}),
                                    ("rebind elsewhere", other, {"rebind_by_node_number": True}),
                                    ("not applied", other, {})):
            result = import_as_given(mbf, sidecar=given, material_provenance=INVENTED, source_label=route, **options)
            assert refusals(result) == {("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH", "sidecar_sha256")}, (label, route)
            assert result.source["binding"] == "unbound" and result.source["sidecar_self_digest"] is None
    # The same edits, their digest recomputed (a deliberate editor), reach the guards behind it: the label, which
    # the MBF determines, is refused by the agreement rule; the partial sidecar by the parser.
    codes = {code for code, _ in refusals(import_caepipe_mbf(data, sidecar=json.loads(edited),
                                                             material_provenance=INVENTED, source_label="e"))}
    assert "MBF-IMPORT-SIDECAR-CONFLICT" in codes and "MBF-IMPORT-SIDECAR-DIGEST-MISMATCH" not in codes
    assert refusals(import_caepipe_mbf(data, sidecar=partial, material_provenance=INVENTED, source_label="p")) == {
        ("MBF-IMPORT-SIDECAR-INVALID", "load_case_map")}


def test_a_malformed_or_removed_self_digest_is_refused_and_a_deliberate_downgrade_still_meets_the_guards():
    data, sidecar = bound_parts(writer_tests.cantilever())
    for value in (None, 7, "F" * 64, "f" * 63, ["x"]):
        result = import_as_given(data, sidecar={**sidecar, "sidecar_sha256": value}, material_provenance=INVENTED,
                                 source_label="malformed")
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "sidecar_sha256")}, value
    removed = {key: value for key, value in sidecar.items() if key != "sidecar_sha256"}
    for options in ({}, {"rebind_by_node_number": True}):  # format 3 without its digest is not read as format 2
        result = import_as_given(data, sidecar=removed, material_provenance=INVENTED, source_label="removed",
                                 **options)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "sidecar_sha256")}, options
        assert any("not read as an older format" in item["message"] for item in result.diagnostics)
    # Format 2 has no self-digest field, and a legacy sidecar none either.
    result = import_as_given(data, sidecar={**sidecar, "sidecar_format": 2}, material_provenance=INVENTED,
                             source_label="kept")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "sidecar_sha256")}
    legacy_data = LEGACY_MBF_TEXT.encode("ascii")
    result = import_as_given(legacy_data, sidecar={**LEGACY_SIDECAR, "sidecar_sha256": "0" * 64},
                             material_provenance=INVENTED, source_label="legacy")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "sidecar_sha256")}
    # Content with no canonical form is refused by path before the digest is compared.
    bad = deepcopy(sidecar)
    bad["node_map"]["N0"] = "1\ud800"
    result = import_as_given(data, sidecar=bad, material_provenance=INVENTED, source_label="surrogate")
    assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "node_map[N0]")}
    # A deliberate editor can drop the digest and claim format 2: the self-digest cannot stop that, and the import
    # says the sidecar carries none. The parser, the agreement rule and the regeneration identity still govern
    # meaning: an unchanged downgrade imports the same model, and an MBF-determined edit with it is still refused.
    downgraded = {**removed, "sidecar_format": 2}
    result = import_as_given(data, sidecar=downgraded, material_provenance=INVENTED, source_label="downgraded")
    base = import_as_given(data, sidecar=sidecar, material_provenance=INVENTED, source_label="base")
    assert result.status == "imported" and result.model_bytes() == base.model_bytes()
    assert result.source["sidecar_self_digest"] == "absent"
    assert ("MBF-IMPORT-SIDECAR-NO-SELF-DIGEST", "info") in {(item["code"], item["severity"])
                                                            for item in result.diagnostics}
    edited = deepcopy(downgraded)
    edited["support_map"]["anchor:N0"]["node"] = "99"
    assert import_as_given(data, sidecar=edited, material_provenance=INVENTED, source_label="edited").status == (
        "refused")


def test_older_format_2_and_legacy_sidecars_import_as_before_and_say_they_carry_no_self_digest():
    data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,10,0",))
    plain = importer_tests.run(data, pressure_closure=importer_tests.CLOSURE)
    older = complete_sidecar(data, plain.model, plain.id_map)  # format 2, as written before the self-digest
    assert older["sidecar_format"] == 2 and "sidecar_sha256" not in older
    legacy_data = LEGACY_MBF_TEXT.encode("ascii")
    for label, mbf, sidecar, location, says in (
            ("format 2", data, older, "sidecar_format", "format 2, written before the self-digest"),
            ("legacy", legacy_data, LEGACY_SIDECAR, "sidecar", "legacy, from main's tranche-1 writer")):
        result = import_as_given(mbf, sidecar=sidecar, material_provenance=INVENTED, source_label=label)
        assert result.status == "imported" and result.source["binding"] == "bound", (label, result.diagnostics)
        assert result.source["sidecar_self_digest"] == "absent", label
        report = [item for item in result.diagnostics if item["code"] == "MBF-IMPORT-SIDECAR-NO-SELF-DIGEST"]
        assert [(item["severity"], item["location"]) for item in report] == [("info", location)], label
        assert says in report[0]["message"] and "no self-digest" in report[0]["message"], label
    rebound = import_as_given(data, sidecar=older, material_provenance=INVENTED, rebind_by_node_number=True,
                              pressure_closure=CLOSURE, source_label="rebound")
    assert rebound.status == "imported", rebound.diagnostics
    assert (rebound.source["binding"], rebound.source["sidecar_self_digest"]) == ("rebound_unconfirmed", "absent")


def test_the_writer_preflight_reimports_its_sidecar_with_the_self_digest(monkeypatch):
    # The preflight's bound re-import reads the writer's own sidecar, self-digest included: a writer whose digest
    # were wrong refuses its own export, naming the importer's code.
    from core.handoff.caepipe_mbf import si_writer
    assert render_caepipe_mbf_si(writer_tests.pressurized_l(), title="PREFLIGHT").status == "written"
    monkeypatch.setattr(si_writer, "sidecar_self_digest", lambda sidecar: "0" * 64)
    written = render_caepipe_mbf_si(writer_tests.pressurized_l(), title="PREFLIGHT")
    assert written.status == "refused"
    found = [item for item in written.diagnostics if item["code"] == "MBF-SI-PREFLIGHT-REFUSED"]
    assert any("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH" in item["message"] for item in found), written.diagnostics


def test_a_repeated_member_name_is_refused_by_path_on_every_load_path_and_in_every_format(tmp_path):
    # Codex VER-002 self-digest r1 (P2): json.loads keeps the last of two members with one name, so "N0":"99","N0":"10"
    # in a format-3 sidecar imported as verified, its digest unchanged. Where the importer parses JSON (the file
    # loader, sidecar text or bytes, the command line), a repeated name is refused by path, in every format.
    data, sidecar = bound_parts(writer_tests.cantilever())
    text = json.dumps(sidecar)
    assert text.count('"N0": "10"') == 1
    repeated = text.replace('"N0": "10"', '"N0": "99", "N0": "10"')
    assert import_as_given(data, sidecar=json.loads(repeated), material_provenance=INVENTED,
                           source_label="lenient").source["sidecar_self_digest"] == "verified"  # why it matters
    path = tmp_path / "id_map.json"
    path.write_text(repeated, encoding="utf-8")
    loaded = importer_module.load_sidecar(path)
    assert loaded == repeated.encode("utf-8")  # unchanged: the importer parses it
    for label, given, options in (("file", loaded, {}), ("text", repeated, {}), ("bytes", repeated.encode(), {}),
                                  ("rebind", loaded, {"rebind_by_node_number": True})):
        result = import_as_given(data, sidecar=given, material_provenance=INVENTED, source_label=label, **options)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "node_map[N0]")}, label
        assert result.source["sidecar_self_digest"] is None and result.source["binding"] == "unbound", label
    with pytest.raises(importer_module.SidecarJsonInvalid) as raised:
        importer_module.parse_sidecar_json(repeated)
    assert raised.value.findings[0][0] == "node_map[N0]"
    # Nested, top-level, format 2 and legacy alike.
    nested = text.replace('"y_reference": {"x": ', '"y_reference": {"x": 5.0, "x": ', 1)
    top = text[:-1] + ', "mbf_sha256": "' + "0" * 64 + '"}'
    older_data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,10,0",))
    plain = importer_tests.run(older_data, pressure_closure=importer_tests.CLOSURE)
    older = json.dumps(complete_sidecar(older_data, plain.model, plain.id_map))
    legacy_text = json.dumps(LEGACY_SIDECAR)
    for label, mbf, given, where in (
            ("nested", data, nested, "element_map[S1].y_reference.x"),
            ("top", data, top, "mbf_sha256"),
            ("format 2", older_data, older.replace('"sidecar_format": 2', '"sidecar_format": 2, "sidecar_format": 2'),
             "sidecar_format"),
            ("legacy", LEGACY_MBF_TEXT.encode("ascii"),
             legacy_text.replace('"load_case_map": ', '"load_case_map": {}, "load_case_map": ', 1), "load_case_map")):
        assert given not in (text, older, legacy_text), label
        result = import_as_given(mbf, sidecar=given.encode(), material_provenance=INVENTED, pressure_closure=CLOSURE,
                                 source_label=label)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", where)}, (label, refusals(result))
    # Not JSON: NaN, bytes that are not UTF-8, a truncated text. Each is refused at `sidecar`.
    for label, given in (("nan", older.replace('"reversed": false', '"reversed": NaN', 1).encode()),
                         ("utf-8", older.encode()[:-1] + b"\xff}"), ("truncated", older.encode()[:-1])):
        assert given != older.encode(), label
        result = import_as_given(older_data, sidecar=given, material_provenance=INVENTED, pressure_closure=CLOSURE,
                                 source_label=label)
        assert refusals(result) == {("MBF-IMPORT-SIDECAR-INVALID", "sidecar")}, (label, refusals(result))
    # The command line refuses it in the import report.
    mbf_path, out = tmp_path / "in.mbf", tmp_path / "out"
    mbf_path.write_bytes(data)
    assert importer_module.main([str(mbf_path), "--out", str(out), "--sidecar", str(path)]) == 1
    report = json.loads((out / "import_report.json").read_text(encoding="utf-8"))
    assert ("MBF-IMPORT-SIDECAR-INVALID", "node_map[N0]") in {(item["code"], item["location"])
                                                              for item in report["diagnostics"]}


def test_the_companion_digest_backfills_only_a_legacy_sidecar(tmp_path):
    # Codex VER-002 self-digest r1 (P2): load_sidecar filled a missing mbf_sha256 from mbf_render.json before the
    # self-digest was checked, so a format-3 sidecar with its digest field deleted imported as verified. Only main's
    # earliest pilot layout, a legacy sidecar that kept its digest beside it, takes the companion's digest.
    data, sidecar = bound_parts(writer_tests.cantilever())
    older_data = importer_tests.mbf("F10,KA,MM1,PP1,CL=L1", "T20,KA,X1000", loads=("L1,20,10,0",))
    plain = importer_tests.run(older_data, pressure_closure=importer_tests.CLOSURE)
    older = complete_sidecar(older_data, plain.model, plain.id_map)
    legacy_data = LEGACY_MBF_TEXT.encode("ascii")
    for label, mbf, given, expected in (
            ("format 3", data, sidecar, {("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH", "sidecar_sha256")}),
            ("format 2", older_data, older, {("MBF-IMPORT-SIDECAR-INVALID", "mbf_sha256")}),
            ("legacy", legacy_data, LEGACY_SIDECAR, None)):
        folder = tmp_path / label.replace(" ", "-")
        folder.mkdir()
        path = folder / "id_map.json"
        path.write_text(json.dumps({key: value for key, value in given.items() if key != "mbf_sha256"}),
                        encoding="utf-8")
        (folder / "mbf_render.json").write_text(json.dumps({"mbf_sha256": hashlib.sha256(mbf).hexdigest()}),
                                                encoding="utf-8")
        loaded = importer_module.load_sidecar(path)
        result = import_as_given(mbf, sidecar=loaded, material_provenance=INVENTED, pressure_closure=CLOSURE,
                                 source_label=label)
        if expected is None:
            assert loaded == {**LEGACY_SIDECAR, "mbf_sha256": hashlib.sha256(legacy_data).hexdigest()}
            assert result.status == "imported" and result.source["binding"] == "bound", result.diagnostics
        else:
            assert loaded == path.read_bytes(), label  # supplied unchanged: no backfill, merge or normalisation
            assert refusals(result) == expected, (label, refusals(result))
    # A companion that is not strict JSON is refused, not read.
    (tmp_path / "legacy" / "mbf_render.json").write_text('{"mbf_sha256": "0", "mbf_sha256": "1"}', encoding="utf-8")
    with pytest.raises(importer_module.SidecarJsonInvalid):
        importer_module.load_sidecar(tmp_path / "legacy" / "id_map.json")


# --- tranche 3: imported guides, limit stops and springs solve as hand-derived (Design §12) ---------------------------
def support_force(output, support_id: str, component: str = "Fy") -> float:
    """The solve's one reaction component of `support_id`, in N."""
    values = [row["value"] * SI_RESULT[row["unit"]] for row in output["payload"]["mechanics_envelope"]["results"]
              if row.get("kind") == "support_reaction_component_v2" and row.get("entity_ref") == support_id
              and (row.get("metadata") or {}).get("component") == component]
    assert len(values) == 1, (support_id, component, values)
    return values[0]


def solved(rows: tuple[str, ...]) -> tuple[dict, dict]:
    result = importer_tests.run(importer_tests.mbf(*rows))
    assert result.status == "imported", result.diagnostics
    _, output, error = solve_swbpipe(runner_binary(), {"case_id": "tranche-3", "model": result.model})
    assert error == "", error
    return result.model, output


# The invented section and material of importer_tests.mbf: OD 168.3 mm, wall 7.11 mm, E 200000 MPa, no weight.
SECOND_MOMENT = math.pi / 64 * (0.1683 ** 4 - (0.1683 - 2 * 0.00711) ** 4)  # m^4
MODULUS = 200000e6  # Pa


@pytest.mark.parametrize("prop, support_id", [("G", "support:30-guide"), ("LS(0,0),DV(0,-1,0)", "support:30-stop-y")])
def test_an_imported_rigid_guide_or_limit_stop_props_a_cantilever_as_hand_derived(prop, support_id):
    # Fixed at node 10, propped at 30 (3 m), 6000 N down at midspan node 20: the prop carries 5P/16 = 1875 N
    # (Euler-Bernoulli, which a pressure-free document solves with). The guide's element runs along X, so it
    # restrains UY and UZ; the limit stop restrains UY. Neither restrains a rotation.
    model, output = solved(("F10,KA,MM1,PP1,CL=L1", "T20,X1500,CFOR(0,-6000,0),FLC=1", f"T30,X1500,C{prop}"))
    assert next(item for item in model["supports"] if item["id"] == support_id)
    assert abs(support_force(output, support_id)) == pytest.approx(1875.0, rel=1e-9)


def test_an_imported_spring_shares_a_tip_load_with_the_cantilever_as_hand_derived():
    # A 3 m cantilever with a skewed restraint K = 500 N/mm along Y at its tip and 6000 N down there: the tip
    # deflects F / (3EI/L^3 + k), and the spring carries k / (3EI/L^3 + k) of the load. The flexible anchor's TRAK
    # gives the same spring.
    beam = 3 * MODULUS * SECOND_MOMENT / 3.0 ** 3
    expected = 6000.0 * 5e5 / (beam + 5e5)
    for row, support_id in (("T20,X3000,CK=500,DV(0,1,0),FOR(0,-6000,0),FLC=1", "support:20-spring-uy"),
                            ("T20,X3000,KA,CTRAK(0,500,0),ROTK(0,0,0),FOR(0,-6000,0),FLC=1", "support:20-spring-uy")):
        model, output = solved(("F10,KA,MM1,PP1,CL=L1", row))
        assert next(item for item in model["supports"] if item["id"] == support_id)["stiffness"]["value"] == {
            "value": 500.0, "unit": "N/mm"}
        assert abs(support_force(output, support_id)) == pytest.approx(expected, rel=1e-9)
        assert abs(support_force(output, "support:10")) == pytest.approx(6000.0 - expected, rel=1e-9)


def test_a_format_3_spring_entry_is_held_to_its_vocabulary_and_format_2_admits_none():
    # The parser (Design §15): a spring acts on exactly one translation with an authored family, and only format 3
    # records springs; format 2 sidecars keep their rigid vocabulary unchanged.
    trip = Trip("springs", writer_tests.sprung_cantilever(), "SPRING", 20.0)
    assert trip.bound.status == "imported", trip.bound.diagnostics
    key = "spring:N1:y"
    for change, path in (
            (lambda entry: entry.update(restrained=["UY", "UZ"]), f"support_map[{key}].restrained"),
            (lambda entry: entry.update(restrained=["RY"]), f"support_map[{key}].restrained"),
            (lambda entry: entry.update(family_authored=False), f"support_map[{key}].family_authored")):
        sidecar = deepcopy(trip.sidecar)
        change(sidecar["support_map"][key])
        result = import_caepipe_mbf(trip.data, sidecar=sidecar, material_provenance=INVENTED, source_label="spring")
        assert (("MBF-IMPORT-SIDECAR-INVALID", path) in refusals(result)), result.diagnostics
    format_2 = {key: value for key, value in deepcopy(trip.sidecar).items() if key != "sidecar_sha256"}
    format_2["sidecar_format"] = 2
    result = import_as_given(trip.data, sidecar=format_2, material_provenance=INVENTED, source_label="spring")
    assert ("MBF-IMPORT-SIDECAR-INVALID", f"support_map[{key}].family") in refusals(result)


def runner_text(model: dict) -> str:
    """The runner's whole answer (stdout and stderr) to a solve of `model`, for its named refusal."""
    body = {"request": run_pilot.runner_request("tranche-3"), "solve": {"preview_model": {"model": model,
                                                                                          "materials": []}}}
    completed = subprocess.run([runner_binary(), "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    return completed.stdout + completed.stderr


TIP_SPRING = ("F10,KA,MM1,PP1,CL=L1", "T20,X3000,CK={},DV(0,1,0),FOR(0,-6000,0),FLC=1")


def test_a_negligible_spring_is_imported_and_refused_by_name_by_the_solve():
    # VER-002 r2 (tranche 3), classified with the guard map's solve-time arithmetic row (Design §9.4, §12): a
    # stiffness negligible against the assembled stiffness is the file's, and the solve refuses it by name when its
    # assembly absorbs the spring. The import keeps the spring as written; no bound is applied.
    result = importer_tests.run(importer_tests.mbf(*(row.format("1e-300") for row in TIP_SPRING)))
    assert result.status == "imported", result.diagnostics
    spring = next(item for item in result.model["supports"] if item["id"] == "support:20-spring-uy")
    assert spring["stiffness"]["value"] == {"value": 1e-300, "unit": "N/mm"}
    _, _, error = solve_swbpipe(runner_binary(), {"case_id": "tranche-3", "model": result.model})
    assert error != ""
    assert "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED" in runner_text(result.model)


def test_the_checked_json_guard_is_the_solves_own_refusal(monkeypatch):
    # Without the import's guard, k = 1e20 N/mm imports and the solve refuses the document as
    # CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT; with it, the import refuses it by name first (Design §9.4 guard map).
    data = importer_tests.mbf(*(row.format("1e20") for row in TIP_SPRING))
    assert importer_tests.run(data).status == "refused"
    monkeypatch.setattr(importer_module._Importer, "checked_json_domain", lambda self, model: None)
    unguarded = importer_tests.run(data)
    assert unguarded.status == "imported", unguarded.diagnostics
    assert "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT" in runner_text(unguarded.model)
