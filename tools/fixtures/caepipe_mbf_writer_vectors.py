#!/usr/bin/env python3
"""Parity vectors between the SI MBF writer and its Rust port (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §6
slice 2).

The Rust engine crate `core/handoff/caepipe_mbf_engine` ports `core/handoff/caepipe_mbf/si_writer.py` byte for byte.
This writer records what the Python does, and the two checks hold each side to the record:

- `harvest` runs the test suites that call the writer (SI writer, round trip, importer, private regression,
  benchmark, correlation) with every `_Writer.run` recorded: its inputs (the model and the keyword arguments, or the
  regeneration's derived straights and reference temperature). It adds the committed corpus the import vectors write
  back (the invented import fixtures and the authored cases, each imported and written back), the correlation
  cases, which the writer renders, and coverage inputs the generator forms from the first correlation case's model
  for the codes, exceptions and argument types no suite reaches (`coverage_inputs`, `chain_model`). Each recorded input is then replayed through the unpatched writer, and the
  replay's outputs are the expectation: the status, the MBF text, the `to_dict()` canonical bytes and the sidecar's
  canonical bytes (or the exception the writer raises), as a digest, with the full text for corpus items. A test that
  bypasses the writer's preflight (`survives_reimport`) also gives the input with the preflight bypassed. Numerical
  vectors add `_num` (`format(x, ".10g")`) at 10-significant-digit rounding boundaries for floats and exact
  fractions, `math.degrees`/`math.radians`, and every unit table's exact normalization factor.
- `--check` replays every recorded input through the Python and fails at the first output that differs: the Python
  side cannot change alone. The Rust test `tests/writer_parity.rs` replays the same file: the Rust side cannot either.

Every input is invented (the test files' own models, the invented fixtures, the authored and correlation cases);
none is CAEPIPE output. Usage, from the project root:

    python3 tools/fixtures/caepipe_mbf_writer_vectors.py harvest   # rewrite the vectors
    python3 tools/fixtures/caepipe_mbf_writer_vectors.py --check   # replay, write nothing
    python3 tools/fixtures/caepipe_mbf_writer_vectors.py show <id> # print one vector's Python outputs
"""
from __future__ import annotations

import contextlib
import copy
import hashlib
import importlib.util
import json
import math
import os
from fractions import Fraction
from pathlib import Path
import random
import re
import struct
import sys
from typing import Any

sys.dont_write_bytecode = True
PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_mbf import importer  # noqa: E402
from core.handoff.caepipe_mbf import si_writer  # noqa: E402


def _load_base():
    spec = importlib.util.spec_from_file_location("caepipe_mbf_import_vectors_base",
                                                  Path(__file__).with_name("caepipe_mbf_import_vectors.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BASE = _load_base()  # the import vectors' lossless JSON forms, value store and corpus harvest
Unrepresentable = BASE.Unrepresentable

# A writer vector is a row [src, kind, model, settings, expect]: the index of the test or corpus item that made the
# call in `sources`; "render" or "regenerate"; the model and the settings (value keys); and the expectation,
# {"digest": 128 bits of SHA-256 over the outputs} (with "status", "text", "to_dict" and "sidecar" text, or
# "sidecar"/"refused" text for a regeneration, for corpus items) or {"raises": [type, message]}. A render's settings
# are {title, reference_temperature_c, max_layout_rows, reference_temperature_f, piping_code, preflight} (and
# `chain`, a chain_model of that many pipe segments in place of the model's own); a
# regeneration's {derived_y_reference, reference} (reference {} or {reference_temperature_c|_f: float}), with the
# digest REGENERATION_SHA256 as its mbf_sha256. A row's id is its index.
ROW = {name: index for index, name in enumerate(("src", "kind", "model", "settings", "expect"))}
VECTORS = PROJECT / "fixtures/caepipe_mbf/invented/parity/writer_vectors.json"
SUITES = ["tests/test_caepipe_mbf_si_writer.py", "tests/test_caepipe_mbf_round_trip.py",
          "tests/test_caepipe_mbf_importer.py", "tests/test_caepipe_private_mbf_import.py",
          "tests/test_caepipe_benchmark.py", "tests/test_caepipe_correlation.py"]
DOCUMENT_KIND = "openpipestress.caepipe_mbf_engine.writer_parity_vectors"
REGENERATION_SHA256 = hashlib.sha256(b"invented writer parity regeneration").hexdigest()
WRITER_SOURCE = PROJECT / "core/handoff/caepipe_mbf/si_writer.py"
ORIGINAL_RUN = si_writer._Writer.run
ORIGINAL_PREFLIGHT = si_writer._Writer.survives_reimport
# The writer's unit tables by the names the Rust port gives them (`ports::writer_unit_factors`).
TABLES = {"length": "_LENGTH_TO_MM", "stress": "_STRESS_TO_MPA", "force": "_FORCE_TO_N", "moment": "_MOMENT_TO_NM",
          "force_per_length": "_FORCE_PER_LENGTH_TO_N_PER_M", "expansion": "_EXPANSION_TO_PER_C",
          "temperature_interval": "_TEMPERATURE_INTERVAL_TO_C", "rotation": "_ROTATION_TO_DEG",
          "dimensionless": "_DIMENSIONLESS", "stiffness": "_STIFFNESS_TO_N_PER_MM", "pressure": "_PRESSURE_TO_BAR"}


PROVENANCE = {
    "description": ("Parity vectors of the SI MBF writer and its Rust port: the inputs the writer's test suites, the "
                    "invented corpus and the generator's coverage inputs give it, and the digests or text of its "
                    "outputs"),
    "license": "project-original",
    "origin": ("invented for the DEL-17-04 MBF engine parity check; written by "
               "tools/fixtures/caepipe_mbf_writer_vectors.py"),
    "produced_by_caepipe": False,
}


def writer_codes() -> list[str]:
    """Every diagnostic code si_writer.py can emit: its `MBF-SI-*` literals and the retired-version code."""
    text = WRITER_SOURCE.read_text(encoding="utf-8")
    return sorted(set(re.findall(r'"(MBF-SI-[A-Z0-9-]+)"', text)) | {"MODEL_SCHEMA_VERSION_RETIRED"})


# ---------------------------------------------------------------------------
# recording
# ---------------------------------------------------------------------------
class Recorder:
    """Wraps `_Writer.run` while the suites and the corpus run, keeping each distinct input once."""

    def __init__(self, store) -> None:
        self.store = store
        self.rows: list[list] = []
        self.seen: dict[str, int] = {}
        self.skipped: dict[str, int] = {}
        self.source = "?"
        self.sources: dict[str, int] = {}
        self.full = False

    def skip(self, reason: str) -> None:
        self.skipped[reason] = self.skipped.get(reason, 0) + 1

    @contextlib.contextmanager
    def installed(self):
        recorder = self

        def run(writer):
            try:
                recorder.capture(writer)
            except Unrepresentable as error:
                recorder.skip(f"input not representable in JSON: {error}")
            return ORIGINAL_RUN(writer)

        si_writer._Writer.run = run
        try:
            yield
        finally:
            si_writer._Writer.run = ORIGINAL_RUN

    def capture(self, writer) -> None:
        model = BASE.lossless(writer.model)
        if writer.regenerating is not None:
            reference = writer.t_ref_f if writer.t_ref_f is not None else writer.t_ref
            if type(reference) is not float:
                raise Unrepresentable("a regeneration reference temperature that is not a float")
            name = "reference_temperature_f" if writer.t_ref_f is not None else "reference_temperature_c"
            settings = {"derived_y_reference": sorted(BASE.lossless(list(writer.regenerating))),
                        "reference": {name: reference}}
            self.add("regenerate", model, settings)
            return
        if writer.max_layout_rows is not None and type(writer.max_layout_rows) is not int:
            raise Unrepresentable("a max_layout_rows that is not an int or None")
        settings = {"title": BASE.lossless(writer.title), "reference_temperature_c": BASE.lossless(writer.t_ref),
                    "max_layout_rows": writer.max_layout_rows,
                    "reference_temperature_f": BASE.lossless(writer.t_ref_f),
                    "piping_code": BASE.lossless(writer.piping_code), "preflight": True}
        self.add("render", model, settings)
        if type(writer).survives_reimport is not ORIGINAL_PREFLIGHT:  # the test bypasses or wraps the preflight
            self.add("render", model, {**settings, "preflight": False})

    def add(self, kind: str, model: Any, settings: dict) -> None:
        model_key, settings_key = self.store.value(model), self.store.value(settings)
        identity = f"{kind}|{model_key}|{settings_key}"
        if self.source not in self.sources:
            self.sources[self.source] = len(self.sources)
        row = [self.sources[self.source], kind, model_key, settings_key, {"full": self.full}]
        if identity in self.seen:
            if self.full and not self.rows[self.seen[identity]][ROW["expect"]]["full"]:
                self.rows[self.seen[identity]] = row  # a corpus call a suite made first: the corpus source, full text
            return
        self.seen[identity] = len(self.rows)
        self.rows.append(row)


class SourcePlugin:
    """Names each recorded call by the test that made it."""

    def __init__(self, recorder: Recorder) -> None:
        self.recorder = recorder

    def pytest_runtest_setup(self, item):
        self.recorder.source = item.nodeid

    def pytest_runtest_call(self, item):
        self.recorder.source = item.nodeid


@contextlib.contextmanager
def without_cargo():
    """The suites run without Cargo (OPENPIPESTRESS_NO_CARGO_BUILD=1 and no supplied binaries), so a harvest records
    the same calls on every host; a test that needs a binary is skipped."""
    saved = {key: value for key, value in os.environ.items()
             if key == "OPENPIPESTRESS_NO_CARGO_BUILD" or (key.startswith("OPENPIPESTRESS_") and key.endswith("_BIN"))}
    for key in saved:
        del os.environ[key]
    os.environ["OPENPIPESTRESS_NO_CARGO_BUILD"] = "1"
    try:
        yield
    finally:
        del os.environ["OPENPIPESTRESS_NO_CARGO_BUILD"]
        os.environ.update(saved)


def harvest_suites(recorder: Recorder) -> None:
    import pytest
    with without_cargo():
        code = pytest.main(["-q", "-p", "no:cacheprovider", "-p", "no:randomly", "-p", "no:xdist", *SUITES],
                           plugins=[SourcePlugin(recorder)])
    if code not in (0, pytest.ExitCode.OK):
        raise SystemExit(f"the writer suites failed under recording (exit {code}); the vectors are not written")


def harvest_corpus(recorder: Recorder) -> None:
    """The import vectors' corpus (each invented fixture and authored case imported and written back, the
    correlation cases written), with only the writer recorded here."""
    BASE.harvest_corpus(recorder)


CASES = PROJECT / "validation/caepipe_correlation/cases"
CHAIN = 10000  # pipe segments: 10001 connected nodes, beyond the 9999 node numbers


def chain_model(model: dict, count: int) -> dict:
    """`model` with its nodes and pipe segments replaced by a straight chain along X of `count` copies of its first
    pipe segment's material and section, unsupported and unloaded: `count + 1` connected nodes. The Rust replay forms
    the same document from the same settings (`chain`), so the vector stores the base model only."""
    out = copy.deepcopy(model)
    pipe = out["pipe_segments"][0]
    out["nodes"] = [{"id": f"node:chain-{index}", "position": {"x": float(index), "y": 0.0, "z": 0.0}}
                    for index in range(count + 1)]
    out["pipe_segments"] = [{"id": f"pipe:chain-{index}", "from": f"node:chain-{index}", "to": f"node:chain-{index + 1}",
                             "material": pipe["material"], "section": pipe["section"]} for index in range(count)]
    out["supports"], out["components"], out["load_cases"] = [], [], []
    return out


def coverage_inputs() -> list[tuple[str, dict, dict]]:
    """Inputs for what no suite test reaches, each the first correlation case's model (or the first with a thermal
    primitive) changed by code: the writer refusals and omissions the suites never emit, the Python exceptions a
    model value can raise, and keyword arguments of other types. `(name, model, keyword arguments)`."""
    models = [json.loads(path.read_text(encoding="utf-8"))["model"] for path in sorted(CASES.glob("*.json"))]
    base = models[0]
    thermal = next(model for model in models if any(load.get("category") == "thermal" for case in model["load_cases"]
                                                     for load in case["primitive_loads"]))
    out: list[tuple[str, dict, dict]] = []

    def variant(name: str, change, source: dict = base, **kwargs) -> None:
        model = copy.deepcopy(source)
        change(model)
        out.append((name, model, kwargs))

    def first_load(model: dict) -> dict:
        return model["load_cases"][0]["primitive_loads"][0]

    def free_node(model: dict, node_id: str = "node:coverage-free") -> str:
        model["nodes"].append({"id": node_id, "position": {"x": 0.0, "y": 5.0, "z": 0.0}})
        return node_id

    def weight(model: dict, value: float, unit: str) -> None:
        model["load_cases"][0]["primitive_loads"].append({
            "id": "load:coverage-weight", "category": "weight", "dimension": "force_per_length", "direction": "global_y",
            "target": {"type": "element", "pipe": model["pipe_segments"][0]["id"]}, "magnitude": {"value": value, "unit": unit}})

    variant("load-case-feature", lambda m: m["load_cases"][0].update(equivalent_static={"invented": "coverage"}))
    variant("material-expansion-missing",
            lambda m: [material.pop("thermal_expansion_coefficient", None) for material in m["materials"]], thermal)
    variant("material-poisson-missing",
            lambda m: [(material.pop("poisson_ratio", None), material.pop("shear_modulus", None)) for material in m["materials"]])
    variant("model-settings", lambda m: m.update(model_settings={"invented": "coverage"}))
    # Review r1 of #32: a Poisson ratio of -1 beside a shear modulus divides by zero (ZeroDivisionError); a load with no
    # category key is subscripted (KeyError), while an explicit null category is refused by name.
    variant("material-poisson-minus-one",
            lambda m: m["materials"][0].update(poisson_ratio={"value": -1.0, "unit": "1"},
                                               shear_modulus=m["materials"][0].get("shear_modulus")
                                               or {"value": 80.0, "unit": "GPa"}))
    variant("load-category-absent", lambda m: first_load(m).pop("category"))
    variant("load-category-null", lambda m: first_load(m).update(category=None))
    variant("node-not-exported", lambda m: free_node(m))
    variant("reference-configurations", lambda m: m.update(reference_configurations=[{"id": "invented"}]))
    variant("section-density", lambda m: m["pipe_segments"][0]["section"].update(material_density={"value": 7850.0, "unit": "kg/m3"}))
    variant("section-mill-tolerance", lambda m: m["pipe_segments"][0]["section"].update(mill_tolerance={"value": 0.5, "unit": "mm"}))
    variant("shear-model", lambda m: m.update(shear_model="euler_bernoulli"))
    variant("support-empty", lambda m: m["supports"].append({"id": "support:coverage-empty", "node": m["nodes"][-1]["id"],
                                                             "restraints": []}))
    variant("topology-support-without-pipe", lambda m: m["supports"].append({
        "id": "support:coverage-free", "node": free_node(m), "family": "anchor", "restraints": list(si_writer.DOFS)}))
    variant("topology-no-pipes", lambda m: m.update(pipe_segments=[]))

    def loop(model: dict) -> None:
        pipe = model["pipe_segments"][0]
        corner = free_node(model, "node:coverage-corner")
        model["pipe_segments"] += [{**copy.deepcopy(pipe), "id": "pipe:coverage-a", "from": pipe["to"], "to": corner},
                                   {**copy.deepcopy(pipe), "id": "pipe:coverage-b", "from": corner, "to": pipe["from"]}]
    variant("topology-loop", loop)
    variant("weight-positive-us", lambda m: weight(m, 0.1, "lbf/in"))
    variant("weight-negative-us", lambda m: weight(m, -0.7, "lbf/in"))
    # Values the Python raises on (the port raises alike): unhashable model values where the writer hashes them, an
    # integer beyond binary64, and the spring at a node no pipe segment reaches.
    variant("raises-unhashable-direction", lambda m: first_load(m).update(direction=["global_y"]))
    variant("raises-unhashable-category", lambda m: first_load(m).update(category=["weight"]))
    variant("raises-unhashable-family", lambda m: m["supports"][0].update(family=["anchor"]))
    variant("raises-unhashable-restraint", lambda m: m["supports"][0].update(restraints=[["UX"]]))
    variant("raises-huge-int-position", lambda m: m["nodes"][-1]["position"].update(x=10 ** 400))
    variant("raises-huge-int-y-reference", lambda m: m["pipe_segments"][0].update(y_reference={"x": 10 ** 400, "y": 1, "z": 0}))
    variant("raises-spring-unconnected", lambda m: m["supports"].append({
        "id": "support:coverage-spring", "node": free_node(m), "family": "spring", "restraints": ["UY"],
        "stiffness": {"dof": "UY", "value": {"value": 1000.0, "unit": "N/mm"}}}))
    variant("int-position-beyond-2-53", lambda m: m["nodes"][-1]["position"].update(x=2 ** 53 + 1))
    # The readers' other refusals and invalid shapes, one change each.
    bent = next(model for model in models if any(item.get("kind") == "bend" for item in model["components"]))
    bend = lambda m: next(item for item in m["components"] if item.get("kind") == "bend")
    bend_pipe = lambda m: next(item for item in m["pipe_segments"] if item["id"] == bend(m)["geometry"]["bend_pipe_ref"])
    node = lambda m, node_id: next(item for item in m["nodes"] if item["id"] == node_id)
    pipe0 = lambda m: m["pipe_segments"][0]

    def displacement(model: dict, target: dict, direction: str, dimension: str, magnitude: Any) -> None:
        model["load_cases"].append({"id": "case:coverage-displacement", "primitive_loads": [{
            "id": "load:coverage-displacement", "category": "imposed_displacement", "target": target,
            "direction": direction, "dimension": dimension, "magnitude": magnitude}]})

    def anchor_target(model: dict, dof: str) -> dict:
        support = next(item for item in model["supports"] if item.get("family") == "anchor")
        return {"type": "support", "support": support["id"], "dof": dof}

    def thermal_direction(value: Any):
        return lambda m: first_load(m).update(direction=value)

    def pressurized(model: dict, pressure: Any) -> None:
        model["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
        for item in model["pipe_segments"]:
            item["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "mm"}
        ends = [pipe0(model)["from"], pipe0(model)["to"]]
        model["load_cases"][0]["pressure_regions"] = [{
            "id": "region:coverage", "member_pipe_ids": [pipe0(model)["id"]], "pressure_basis": si_writer._PRESSURE_BASIS,
            "provenance": "invented coverage region", "pressure": pressure,
            "terminals": [{"node_ref": end, "closure_transfer": "transfers_to_wall", "provenance": "invented coverage"}
                          for end in ends]}]

    def parallel(model: dict) -> None:
        """The outgoing straight turned parallel to the incoming one: no tangent intersection."""
        near, far = bend_pipe(model)["from"], bend_pipe(model)["to"]
        incoming = next(item for item in model["pipe_segments"] if item["to"] == near)
        outgoing = next(item for item in model["pipe_segments"] if item["from"] == far)
        start, end = node(model, incoming["from"])["position"], node(model, near)["position"]
        node(model, outgoing["to"])["position"] = {axis: node(model, far)["position"][axis] + end[axis] - start[axis]
                                                   for axis in "xyz"}

    for name, change, source in (
            ("project-not-object", lambda m: m.update(project="invented"), base),
            ("nodes-not-list", lambda m: m.update(nodes="invented"), base),
            ("node-without-id", lambda m: m["nodes"].append({"position": {"x": 1.0, "y": 1.0, "z": 1.0}}), base),
            ("node-repeated", lambda m: m["nodes"].append(copy.deepcopy(m["nodes"][0])), base),
            ("node-position-not-object", lambda m: m["nodes"][0].update(position=[0, 0, 0]), base),
            ("node-not-a-record", lambda m: m["nodes"].append(5), base),
            ("material-without-id", lambda m: m["materials"].append({"elastic_modulus": 1}), base),
            ("material-repeated", lambda m: m["materials"].append(copy.deepcopy(m["materials"][0])), base),
            ("material-expansion-laws", lambda m: m["materials"][0].update(expansion_laws=[]), base),
            ("material-temperature-points", lambda m: m["materials"][0].update(temperature_points=[{"invented": 1}]), base),
            ("material-modulus-negative", lambda m: m["materials"][0]["elastic_modulus"].update(value=-1.0), base),
            ("material-shear-negative", lambda m: m["materials"][0]["shear_modulus"].update(value=-1.0), base),
            ("pipe-without-id", lambda m: m["pipe_segments"].append({"from": "x"}), base),
            ("pipe-repeated", lambda m: m["pipe_segments"].append(copy.deepcopy(pipe0(m))), base),
            ("pipe-self-loop", lambda m: pipe0(m).update(to=pipe0(m)["from"]), base),
            ("pipe-material-unknown", lambda m: pipe0(m).update(material="material:coverage-none"), base),
            ("pipe-section-not-object", lambda m: pipe0(m).update(section="invented"), base),
            ("pipe-section-thick", lambda m: pipe0(m)["section"]["wall_thickness"].update(
                value=pipe0(m)["section"]["outside_diameter"]["value"]), base),
            ("pipe-corrosion-invalid", lambda m: pipe0(m)["section"].update(corrosion_allowance={"value": "x", "unit": "mm"}), base),
            ("support-without-id", lambda m: m["supports"].append({"node": m["nodes"][0]["id"]}), base),
            ("support-repeated", lambda m: m["supports"].append(copy.deepcopy(m["supports"][0])), base),
            ("support-node-unknown", lambda m: m["supports"][0].update(node="node:coverage-none"), base),
            ("support-restraints-not-list", lambda m: m["supports"][0].update(restraints="UX"), base),
            ("support-restraint-unknown", lambda m: m["supports"][0].update(restraints=["UQ"]), base),
            ("support-anchor-partial", lambda m: m["supports"][0].update(restraints=["UX", "UY"]), base),
            ("case-without-id", lambda m: m["load_cases"].append({"primitive_loads": []}), base),
            ("case-repeated", lambda m: m["load_cases"].append(copy.deepcopy(m["load_cases"][0])), base),
            ("case-loads-not-list", lambda m: m["load_cases"][0].update(primitive_loads="invented"), base),
            ("case-regions-not-list", lambda m: m["load_cases"][0].update(pressure_regions="invented"), base),
            ("case-only-unsupported", lambda m: first_load(m).update(category="wind"), base),
            ("load-target-not-object", lambda m: first_load(m).update(target="invented"), base),
            ("load-magnitude-not-quantity", lambda m: first_load(m).update(magnitude=5), base),
            ("load-magnitude-not-number", lambda m: first_load(m)["magnitude"].update(value="abc"), base),
            ("force-direction-unknown", lambda m: first_load(m).update(direction="global_q"), base),
            ("weight-target-node", lambda m: (weight(m, -1.0, "N/m"), m["load_cases"][0]["primitive_loads"][-1].update(
                target={"type": "node", "node": m["nodes"][0]["id"]})), base),
            ("weight-dimension-force", lambda m: (weight(m, -1.0, "N/m"),
                                                  m["load_cases"][0]["primitive_loads"][-1].update(dimension="force")), base),
            ("weight-magnitude-invalid", lambda m: weight(m, None, "N/m"), base),
            ("thermal-dimension-length", lambda m: first_load(m).update(dimension="length"), thermal),
            ("thermal-magnitude-invalid", lambda m: first_load(m)["magnitude"].update(value=None), thermal),
            ("displacement-target-node", lambda m: displacement(m, {"type": "node", "node": m["nodes"][0]["id"]}, "UX",
                                                                "displacement", {"value": 1.0, "unit": "mm"}), base),
            ("displacement-dof-mismatch", lambda m: displacement(m, anchor_target(m, "UX"), "UY", "displacement",
                                                                 {"value": 1.0, "unit": "mm"}), base),
            ("displacement-dimension-wrong", lambda m: displacement(m, anchor_target(m, "UX"), "UX", "rotation",
                                                                    {"value": 1.0, "unit": "mm"}), base),
            ("displacement-magnitude-invalid", lambda m: displacement(m, anchor_target(m, "UX"), "UX", "displacement",
                                                                      {"value": "x", "unit": "mm"}), base),
            ("rotation-radians", lambda m: displacement(m, anchor_target(m, "RX"), "RX", "rotation",
                                                        {"value": 0.01, "unit": "rad"}), base),
            ("rotation-radians-overflow", lambda m: displacement(m, anchor_target(m, "RX"), "RX", "rotation",
                                                                 {"value": 1e308, "unit": "rad"}), base),
            ("sidecar-direction-nan", thermal_direction(math.nan), thermal),
            ("sidecar-direction-huge-int", thermal_direction(2 ** 1100), thermal),
            ("sidecar-direction-small", thermal_direction(1.5e-7), thermal),
            ("sidecar-direction-decimal", thermal_direction(123.456), thermal),
            ("sidecar-direction-large", thermal_direction(1.5e22), thermal),
            ("pressure-region", lambda m: pressurized(m, {"value": 10.0, "unit": "bar"}), base),
            ("pressure-region-psi", lambda m: pressurized(m, {"value": 145.0, "unit": "psi"}), base),
            ("pressure-region-invalid", lambda m: pressurized(m, {"value": "x", "unit": "bar"}), base),
            ("bend-geometry-not-object", lambda m: bend(m).update(geometry="invented"), bent),
            ("bend-pipe-unknown", lambda m: bend(m)["geometry"].update(bend_pipe_ref="pipe:coverage-none"), bent),
            ("bend-twice", lambda m: m["components"].append({**copy.deepcopy(bend(m)), "id": "component:coverage-twice"}), bent),
            ("bend-node-not-endpoint", lambda m: bend(m).update(node=m["nodes"][0]["id"]), bent),
            ("bend-angle-unit", lambda m: bend(m)["geometry"].update(bend_angle={"value": 90.0, "unit": "grad"}), bent),
            ("bend-angle-degrees", lambda m: bend(m)["geometry"].update(bend_angle={"value": 90.0, "unit": "deg"}), bent),
            ("bend-factor-negative", lambda m: bend(m)["modifiers"]["flexibility_factor_user_value"].update(value=-1.0), bent),
            ("bend-y-reference-missing", lambda m: bend_pipe(m).pop("y_reference"), bent),
            ("bend-y-reference-along-chord", lambda m: bend_pipe(m).update(y_reference={
                axis: node(m, bend_pipe(m)["to"])["position"][axis] - node(m, bend_pipe(m)["from"])["position"][axis]
                for axis in "xyz"}), bent),
            ("bend-branch-at-tangent", lambda m: (free_node(m, "node:coverage-branch"), m["pipe_segments"].append({
                **copy.deepcopy(bend_pipe(m)), "id": "pipe:coverage-branch", "from": bend_pipe(m)["from"],
                "to": "node:coverage-branch"})), bent),
            ("bend-parallel-straights", parallel, bent),
            ("bend-zero-straight", lambda m: node(m, next(item for item in m["pipe_segments"] if item["to"] == bend_pipe(m)["from"])["from"])
             .update(position=copy.deepcopy(node(m, bend_pipe(m)["from"])["position"])), bent)):
        variant(name, change, source)
    out.append(("model-not-object", [copy.deepcopy(base)], {}))
    # Keyword arguments of other types and the piping codes.
    for name, kwargs in (("reference-bool", {"reference_temperature_c": True}),
                         ("reference-string", {"reference_temperature_c": "20"}),
                         ("reference-nan", {"reference_temperature_c": math.nan}),
                         ("reference-f-int", {"reference_temperature_f": 212}),
                         ("reference-f-huge-int", {"reference_temperature_f": 10 ** 400}),
                         ("reference-f-nan", {"reference_temperature_f": math.nan}),
                         ("reference-c-int", {"reference_temperature_c": 2 ** 60 + 1}),
                         ("title-int", {"title": 12345}), ("title-none", {"title": None}),
                         ("title-unprintable", {"title": " \u00e9,\tINVENTED,  TITLE \u2603 "}),
                         ("piping-code-invalid", {"piping_code": 5}), ("piping-code-b311", {"piping_code": "B311"}),
                         ("piping-code-asme", {"piping_code": "ASME"}), ("rows-unlimited", {"max_layout_rows": None}),
                         ("rows-one", {"max_layout_rows": 1})):
        variant(name, lambda m: None, **kwargs)
    return out


def harvest_coverage(recorder: Recorder) -> None:
    recorder.full = True
    for name, model, kwargs in coverage_inputs():
        recorder.source = f"coverage:{name}"
        with contextlib.suppress(Exception):
            si_writer.render_caepipe_mbf_si(model, **{"title": "INVENTED COVERAGE", **kwargs})
    recorder.source = "coverage:node-number-limit"
    base = json.loads(sorted(CASES.glob("*.json"))[0].read_text(encoding="utf-8"))["model"]
    recorder.add("render", BASE.lossless(base), {"title": "INVENTED COVERAGE", "reference_temperature_c": 20.0,
                                                 "max_layout_rows": 20, "reference_temperature_f": None,
                                                 "piping_code": None, "preflight": True, "chain": CHAIN})
    recorder.full = False


# ---------------------------------------------------------------------------
# replay and expectations
# ---------------------------------------------------------------------------
def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def failed(error: BaseException) -> str:
    return f"!{type(error).__name__}: {error}"


@contextlib.contextmanager
def preflight(enabled: bool):
    if enabled:
        yield
        return
    si_writer._Writer.survives_reimport = lambda self, result: True
    try:
        yield
    finally:
        si_writer._Writer.survives_reimport = ORIGINAL_PREFLIGHT


def replay(kind: str, model: Any, settings: dict):
    if kind == "regenerate":
        return si_writer.regenerate_caepipe_sidecar(model, mbf_sha256=REGENERATION_SHA256,
                                                    derived_y_reference=settings["derived_y_reference"],
                                                    **settings["reference"])
    if settings.get("chain"):
        model = chain_model(model, settings["chain"])
    with preflight(settings["preflight"]):
        return si_writer.render_caepipe_mbf_si(model, title=settings["title"],
                                               reference_temperature_c=settings["reference_temperature_c"],
                                               max_layout_rows=settings["max_layout_rows"],
                                               reference_temperature_f=settings["reference_temperature_f"],
                                               piping_code=settings["piping_code"])


def outputs(kind: str, result) -> dict[str, str]:
    """The texts the digest covers: a render's status, MBF text, to_dict() and sidecar bytes (canonical_bytes; the
    sidecar of the text's own SHA-256), or a regeneration's sidecar or refusals; a failure as "!Type: message"."""
    if kind == "regenerate":
        sidecar, refusals = result
        if sidecar is None:
            return {"refused": importer.canonical_bytes(refusals).decode("ascii")}
        return {"sidecar": importer.canonical_bytes(sidecar).decode("ascii")}
    out = {"status": result.status, "text": result.text if result.text is not None else "-"}
    try:
        out["to_dict"] = importer.canonical_bytes(result.to_dict()).decode("ascii")
    except Exception as error:  # noqa: BLE001 - recorded as the expectation
        out["to_dict"] = failed(error)
    if result.status == "written":
        try:
            out["sidecar"] = importer.canonical_bytes(result.sidecar(sha(result.text.encode("ascii")))).decode("ascii")
        except Exception as error:  # noqa: BLE001
            out["sidecar"] = failed(error)
    else:
        out["sidecar"] = "-"
    return out


def digest(texts: dict[str, str]) -> str:
    parts = [f"{key}={sha(value.encode('utf-8'))}" for key, value in sorted(texts.items())]
    return sha("|".join(parts).encode("ascii"))[:32]


def expectation(kind: str, model: Any, settings: dict, *, full: bool) -> tuple[dict, set[str]]:
    try:
        result = replay(kind, model, settings)
    except Exception as error:  # noqa: BLE001 - the Python writer raises: the port must raise alike
        return {"raises": [type(error).__name__, str(error)]}, set()
    texts = outputs(kind, result)
    if kind == "regenerate":
        codes = {item["code"] for item in result[1]}
    else:
        codes = {item["code"] for item in result.diagnostics}
    out = {"digest": digest(texts)}
    if full:
        out.update(texts)
    return out, codes


# ---------------------------------------------------------------------------
# numerical vectors
# ---------------------------------------------------------------------------
def bits(value: float) -> str:
    return struct.pack("<d", value).hex()


def ten_digit_boundaries() -> list[float]:
    """Binary64 values at and beside 10-significant-digit rounding midpoints, across the decimal exponent range."""
    rng = random.Random(1017)
    values = []
    for exponent in range(-330, 310, 3):
        for _ in range(4):
            digits = rng.randrange(10 ** 9, 10 ** 10)
            midpoint = f"{digits}5e{exponent - 10}"
            with contextlib.suppress(OverflowError):
                value = float(midpoint)
                if math.isfinite(value) and value != 0:
                    values += [value, math.nextafter(value, math.inf), math.nextafter(value, 0.0), -value]
    for text in ("0.12345678905", "1.0000000005", "9.9999999995", "99999999995", "0.00099999999995", "1e-05",
                 "0.0001", "12345678950", "1234567895", "4.4482216152605", "6894.757293168361", "25.4"):
        value = float(text)
        values += [value, math.nextafter(value, math.inf), math.nextafter(value, 0.0)]
    return values


def fraction_cases() -> list[Fraction]:
    """Exact fractions `_num` formats: ties at the tenth significant digit, and the products the writer forms (a
    binary64 value times each exact unit factor)."""
    rng = random.Random(2029)
    cases = [Fraction(10000000005, 10 ** 10), Fraction(10000000015, 10 ** 10), Fraction(99999999995, 10 ** 10),
             Fraction(-10000000005, 10 ** 10), Fraction(1, 3), Fraction(-2, 3), Fraction(10 ** 30, 7),
             Fraction(-1, 7 * 10 ** 8), Fraction(5, 9), Fraction(9, 5)]
    for _ in range(300):
        digits = rng.randrange(10 ** 9, 10 ** 10)
        cases.append(Fraction(digits * 10 + 5, 10) * Fraction(10) ** rng.randint(-40, 40))
    factors = [factor for _, _, factor in table_factors() if factor is not None]
    floats = BASE.boundary_floats()[::97]
    for index, value in enumerate(floats):
        cases.append(Fraction(value) * factors[index % len(factors)])
    return [case for case in cases if case != 0]


def table_factors() -> list[tuple[str, str, Fraction | None]]:
    out = []
    for name, attribute in TABLES.items():
        for unit, scaling in getattr(si_writer, attribute).items():
            factor = None if scaling == "rad" else (scaling if isinstance(scaling, Fraction) else Fraction(10) ** scaling)
            out.append((name, unit, factor))
    for unit, (factor, _) in si_writer.NORMALIZATION.items():
        out.append(("normalization", unit, factor))
    return out


def numeric_vectors() -> dict:
    floats = ten_digit_boundaries() + BASE.boundary_floats()[::11]
    rng = random.Random(4099)
    angles = [rng.uniform(-1, 1) * 10.0 ** rng.randint(-20, 20) for _ in range(500)] + [math.pi, -math.pi, 1e308, 5e-324]
    return {
        "num": [[bits(value), si_writer._num(value)] for value in floats],
        "num_fraction": [[str(case.numerator), str(case.denominator), si_writer._num(case)] for case in fraction_cases()],
        "factors": [[name, unit, None if factor is None else [str(factor.numerator), str(factor.denominator)]]
                    for name, unit, factor in table_factors()],
        "degrees": [[bits(value), bits(math.degrees(value)), bits(math.radians(value))] for value in angles],
    }


# ---------------------------------------------------------------------------
# document, check, show
# ---------------------------------------------------------------------------
def harvest() -> dict:
    store = BASE.Store()
    recorder = Recorder(store)
    with recorder.installed():
        harvest_suites(recorder)
        harvest_corpus(recorder)
        harvest_coverage(recorder)
    sources = sorted(recorder.sources, key=recorder.sources.__getitem__)
    document = {"blobs": store.blobs, "values": store.values}
    caches: tuple[dict, dict] = ({}, {})
    rows, codes = [], set()
    for row in recorder.rows:
        model = BASE.value_ref(document, row[ROW["model"]], caches[1])
        settings = BASE.value_ref(document, row[ROW["settings"]], caches[1])
        expect, found = expectation(row[ROW["kind"]], model, settings, full=row[ROW["expect"]]["full"])
        codes |= found
        rows.append([*row[:ROW["expect"]], expect])
    corpus = sorted({row[ROW["model"]] for row in rows if sources[row[ROW["src"]]].startswith("corpus:")})
    return {
        "document_kind": DOCUMENT_KIND,
        "description": ("SI MBF writer parity vectors, written by tools/fixtures/caepipe_mbf_writer_vectors.py from "
                        "core/handoff/caepipe_mbf/si_writer.py over the test suites that call it and the committed "
                        "invented corpus. tests/test_caepipe_mbf_writer_parity.py replays them through the Python; "
                        "core/handoff/caepipe_mbf_engine/tests/writer_parity.rs through the Rust port."),
        "python": ".".join(map(str, sys.version_info[:3])),
        "regeneration_sha256": REGENERATION_SHA256,
        "counts": {"vectors": len(rows), "renders": sum(row[ROW["kind"]] == "render" for row in rows),
                   "regenerations": sum(row[ROW["kind"]] == "regenerate" for row in rows),
                   "corpus_models": len(corpus), "skipped": dict(sorted(recorder.skipped.items()))},
        "codes": sorted(codes),
        "sources": sources,
        "blobs": store.blobs,
        "values": store.values,
        "vectors": rows,
        "numeric": numeric_vectors(),
    }


def render(document: dict) -> str:
    return json.dumps(document, indent=None, separators=(",", ":"), ensure_ascii=True, allow_nan=False) + "\n"


def load() -> dict:
    return json.loads(VECTORS.read_text(encoding="utf-8"))


def inputs(document: dict, row: list, caches: tuple[dict, dict]) -> tuple[str, Any, dict]:
    return (row[ROW["kind"]], BASE.value_ref(document, row[ROW["model"]], caches[1]),
            BASE.value_ref(document, row[ROW["settings"]], caches[1]))


def check(document: dict, part: int | None = None, parts: int = 1) -> tuple[list[str], set[str]]:
    """Replays the vectors (every `parts`-th from `part`); returns the failures and the codes the replays emitted."""
    failures, codes = [], set()
    caches: tuple[dict, dict] = ({}, {})
    for index, row in enumerate(document["vectors"]):
        if part is not None and index % parts != part:
            continue
        kind, model, settings = inputs(document, row, caches)
        want = row[ROW["expect"]]
        got, found = expectation(kind, model, settings, full="digest" in want and len(want) > 1)
        codes |= found
        if got != want:
            failures.append(f"writer vector {index} ({document['sources'][row[ROW['src']]]}): "
                            + (f"raises {got['raises']}" if "raises" in got else f"outputs differ ({kind})"))
            if len(failures) >= 20:
                break
    if part in (None, 0) and document["numeric"] != numeric_vectors():
        recorded, computed = document["numeric"], numeric_vectors()
        failures.append("numeric vectors differ: " + BASE.numeric_difference(recorded, computed))
    return failures, codes


def main(argv: list[str]) -> int:
    os.chdir(PROJECT)
    if argv[:1] == ["harvest"]:
        document = harvest()
        missing = sorted(set(writer_codes()) - set(document["codes"]))
        if missing:
            raise SystemExit(f"no vector emits {', '.join(missing)}; the vectors are not written")
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(render(document), encoding="utf-8")
        VECTORS.with_name("writer_vectors.provenance.json").write_text(json.dumps(PROVENANCE, indent=2) + "\n",
                                                                       encoding="utf-8")
        print(f"wrote {VECTORS.relative_to(PROJECT)}: {json.dumps(document['counts'])}")
        return 0
    if argv[:1] == ["show"] and len(argv) == 2:
        document = load()
        row = document["vectors"][int(argv[1])]
        kind, model, settings = inputs(document, row, ({}, {}))
        sys.stdout.write(json.dumps({"source": document["sources"][row[ROW["src"]]], "kind": kind,
                                     "settings": settings}, indent=2) + "\n")
        try:
            texts = outputs(kind, replay(kind, model, settings))
        except Exception as error:  # noqa: BLE001
            texts = {"raises": failed(error)}
        for key, value in texts.items():
            sys.stdout.write(f"--- {key}\n{value}\n")
        return 0
    if argv[:1] == ["--check"]:
        document = load()
        failures, codes = check(document)
        if codes != set(document["codes"]):
            failures.append("the replayed codes are not the recorded ones")
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1 if failures else 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
