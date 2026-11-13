#!/usr/bin/env python3
"""Focused tests for the CAEPIPE benchmark set, tranches 1 and 2 and the whole set's generation (DEL-17-05 REQ-018 and AC-007; the "Benchmark
method" section of docs/RELEASE_TARGET_v1.0.md): the seeded model generator, the element-row binding, the two-way
comparison and `run_pilot --benchmark` (invented inputs and invented CSV text only; nothing here runs the external
program or reads its output)."""

from __future__ import annotations

import dataclasses
import hashlib
import json
import math
import os
from pathlib import Path
import subprocess
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). Its one project scan is the
# tracked benchmark models; `root.rglob("*")` snapshots the invented exchange folder under tmp_path.
CI_READS = ("validation/caepipe_benchmark/models/*.json",)

from core.handoff.caepipe_external import case_paths, parse_caepipe_print_csv  # noqa: E402
from core.handoff.caepipe_external.benchmark import (  # noqa: E402
    BENCHMARK_RECORD_KIND,
    FINDING,
    OPERATING_LABEL,
    SCREENING_PROFILE,
    STATUS_MISMATCH,
    UNMATCHED,
    WITHIN,
    build_benchmark_record,
    screen,
    screening_class,
)
from core.handoff.caepipe_external.harness import RUN_RECORD_KIND, check_guest_alive  # noqa: E402
from core.handoff.caepipe_external.print_csv import bind_element_rows  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import benchmark_models, run_pilot  # noqa: E402
from tests.test_caepipe_correlation import INVENTED_GUARD, fake_config  # noqa: E402


BENCHMARK = ROOT / "validation" / "caepipe_benchmark"
MANIFEST = BENCHMARK / "manifest.json"


def tracked_models() -> list[dict]:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    return [json.loads((BENCHMARK / entry["file"]).read_text(encoding="utf-8")) for entry in manifest["models"]]


def earlier_models() -> list[dict]:
    """Tranches 1 and 2; tranche 3 is examined in tests/test_caepipe_benchmark_tranche3.py."""
    return [document for document in tracked_models() if document.get("tranche") in (None, "2")]


def pressurised_models() -> list[dict]:
    return [document for document in tracked_models() if document.get("tranche") == "2"]


# -- generator -------------------------------------------------------------------------------------------------------
def test_the_generator_is_deterministic_and_the_tracked_set_is_its_output():
    first, second = benchmark_models.generate(), benchmark_models.generate()
    assert first == second
    tracked = {f"models/{path.name}": path.read_bytes() for path in (BENCHMARK / "models").glob("*.json")}
    tracked["manifest.json"] = MANIFEST.read_bytes()
    assert tracked == {relative: text.encode("ascii") for relative, text in first.items()}
    assert benchmark_models.main(["--check"]) == 0
    other = benchmark_models.generate(benchmark_models.DEFAULT_SEED + 1)
    assert other.keys() == first.keys() and other != first


def test_the_check_reports_a_changed_file(tmp_path, capsys):
    benchmark_models.write(benchmark_models.generate(), tmp_path)
    assert benchmark_models.main(["--check", "--out", str(tmp_path)]) == 0
    (tmp_path / "models" / "cpb-g1-01.json").write_text("{}\n", encoding="ascii")
    (tmp_path / "models" / "cpb-zz-99.json").write_text("{}\n", encoding="ascii")
    assert benchmark_models.main(["--check", "--out", str(tmp_path)]) == 1
    assert capsys.readouterr().out.split() == ["differs:", "models/cpb-g1-01.json", "differs:", "models/cpb-zz-99.json"]


def test_the_manifest_records_seed_family_parameters_and_hashes():
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    assert manifest["document_kind"] == benchmark_models.MANIFEST_KIND
    assert manifest["generator"]["seed"] == benchmark_models.DEFAULT_SEED
    assert manifest["piping_code"] == "B311-80"
    counts: dict[str, int] = {}
    pressure_counts: dict[str, int] = {}
    for entry in manifest["models"]:
        if entry["tranche"] == "3":
            continue  # tests/test_caepipe_benchmark_tranche3.py
        tally = pressure_counts if entry["tranche"] == "2" else counts
        tally[entry["family"]] = tally.get(entry["family"], 0) + 1
        text = (BENCHMARK / entry["file"]).read_bytes()
        document = json.loads(text)
        assert hashlib.sha256(text).hexdigest() == entry["file_sha256"]
        assert run_pilot.canonical_model_hash(document["model"]) == entry["model_hash"]
        assert document["model_id"] == entry["model_id"] and document["parameters"] == entry["parameters"]
        assert document["generator"] == {"version": benchmark_models.GENERATOR_VERSION,
                                         "seed": benchmark_models.DEFAULT_SEED}
        assert document["provenance"]["origin"] == "invented" and document["provenance"]["produced_by_caepipe"] is False
    assert counts == {"G1": 10, "G2": 16, "G3": 12, "G4": 6, "G5": 6, "G6": 3, "G7": 3, "G8": 4}
    assert sum(counts.values()) == 60
    assert pressure_counts == {"P1": 6, "P2": 4, "P3": 4, "P4": 4, "P5": 2, "P6": 1, "P7": 1}
    # Tranche 2 is appended after tranche 1, and tranche 3 after tranche 2.
    assert [entry["tranche"] for entry in manifest["models"]] == ["1"] * 60 + ["2"] * 22 + ["3"] * 27
    assert sorted(manifest["tranches"]) == ["1", "2", "3"]
    series = manifest["series"]
    assert [member["value"] for member in series["shear-dt"]["members"]] == [10.0, 20.0, 40.0, 100.0]
    assert [member["value"] for member in series["tee-ratio"]["members"]] == [0.3, 0.5, 0.75, 1.0]
    bend_k = [name for name in series if name.startswith("bend-k-")]
    assert len(bend_k) == 8 and all(series[name]["members"][0]["value"] == 1.0 for name in bend_k)
    sampling = {(entry["family"], entry["sampling"]) for entry in manifest["models"]}
    assert {("G1", "corner"), ("G2", "corner"), ("G3", "corner"), ("G4", "corner")} <= sampling
    assert {("G1", "lhs"), ("G3", "lhs"), ("G4", "lhs"), ("G5", "lhs"), ("G6", "lhs"), ("G7", "lhs"),
            ("G8", "lhs")} <= sampling


def test_sections_materials_and_bends_span_the_proposed_ranges():
    ods, ratios, materials, radii = [], [], set(), set()
    for document in earlier_models():
        model = document["model"]
        materials.add(model["materials"][0]["id"])
        for pipe in model["pipe_segments"]:
            od = pipe["section"]["outside_diameter"]["value"] * 1000.0
            wall = pipe["section"]["wall_thickness"]["value"] * 1000.0
            ods.append(od)
            ratios.append(od / wall)
        for component in model["components"]:
            od = next(pipe for pipe in model["pipe_segments"] if pipe["id"] == component["geometry"]["bend_pipe_ref"])
            radii.add(round(component["geometry"]["bend_radius"]["value"] / od["section"]["outside_diameter"]["value"], 6))
            assert component["modifiers"]["flexibility_factor_user_value"]["value"] >= 1.0
        assert not model["materials"][0]["temperature_points"]
    assert min(ods) >= 18.0 and max(ods) == 610.0 and min(ods) <= 60.0 + 1e-9
    assert min(ratios) == pytest.approx(10.0, rel=1e-3) and max(ratios) == pytest.approx(100.0, rel=1e-3)
    assert materials == {name for name, *_ in benchmark_models.MATERIALS}
    assert {1.0, 1.5, 3.0, 5.0} <= radii and max(radii) <= 5.0


def test_every_model_is_written_with_its_explicit_piping_code_within_the_row_limit():
    rows = {entry["model_id"]: entry.get("layout_row_count")
            for entry in json.loads(MANIFEST.read_text(encoding="utf-8"))["models"]}
    for document in tracked_models():
        result = render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                       reference_temperature_c=document["reference_temperature_c"],
                                       piping_code=document["piping_code"])
        assert result.status == "written", (document["model_id"], result.diagnostics)
        assert "\r\nOPTIONS\r\nHGRA,B311-80,SI\r\nOPCODE\r\n0,0,0\r\n" in result.text
        assert result.layout_row_count <= 20
        assert not {item["code"] for item in result.diagnostics} & {"MBF-SI-PIPING-CODE-NOT-WRITTEN",
                                                                   "MBF-SI-LAYOUT-ROW-LIMIT"}
        assert sorted(result.load_case_map) == sorted(case["id"] for case in document["model"]["load_cases"])
        if document.get("tranche") == "2":
            # Written with the Bourdon option and bend pressure correction off, and its row count recorded.
            assert "\r\nOPPRES\r\n0,0,0,1,0\r\n" in result.text
            assert rows[document["model_id"]] == result.layout_row_count
            assert result.load_case_map == {"case:sustained": "Sustained (W+P)", "case:expansion": "Expansion (T1)"}
            assert result.pressure_map["contract"] == {"version": "3.0.0", "mode": "exact_pressure_v3"}
        elif document.get("tranche") == "3":
            assert "OPPRES" not in result.text and rows[document["model_id"]] == result.layout_row_count
        else:
            assert "OPPRES" not in result.text and rows[document["model_id"]] is None


def solve(model: dict) -> dict:
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner:
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    body = {"request": run_pilot.runner_request("benchmark-test"), "solve": {"preview_model": {"model": model,
                                                                                                "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    output = json.loads(completed.stdout)
    assert output["payload"]["mechanics_envelope"]["status"]["mechanics"] == "MECHANICS_SOLVED"
    return output["payload"]


# -- invented CSV text in the documented layout ---------------------------------------------------------------------
def csv_text(cases: dict[str, dict]) -> str:
    """Invented Print > To File text: per label, anchors, restraints, global element rows and displacements."""
    lines = ["Invented Prover,INVENTED-BENCHMARK,", "Version 0.0,Invented benchmark fixture,Invented timestamp,", ""]
    for label, tables in cases.items():
        lines += [f"Loads on Anchors in Global Coordinates: {label},",
                  "Node,Tag,FX (N),FY (N),FZ (N),MX (Nm),MY (Nm),MZ (Nm),,"]
        lines += [f"{node},,{','.join(values)},," for node, values in tables.get("anchors", {}).items()]
        lines += ["", f"{label}: Loads on restraints,", "Node,Tag,FX (N),FY (N),FZ (N),,"]
        lines += [f"{node},,{','.join(values)},," for node, values in tables.get("restraints", {}).items()]
        lines += ["", f"Element forces in global coordinates: {label},",
                  "Node,FX (N),FY (N),FZ (N),MX(Nm),MY(Nm),MZ(Nm),"]
        lines += [f"{node},{','.join(values)}," for node, values in tables.get("elements", [])]
        lines += ["", f"{label}: Displacements,", ",,,Displacements (global),,,,", ",X,Y,Z,XX,YY,ZZ,",
                  "Node,(mm),(mm),(mm),(deg),(deg),(deg),,"]
        lines += [f"{node},{','.join(values)},," for node, values in tables.get("displacements", {}).items()]
        lines.append("")
    return "\r\n".join(lines) + "\r\n"


def mirror_tables(payload: dict, mbf, case_ids: list[str]) -> dict:
    """Invented printed tables formed from SWBPIPE's own results, rounded to the printed resolution: what a program
    that agreed exactly with SWBPIPE would print. The support rows are the pipe's load on the support."""
    from core.handoff.caepipe_external.benchmark import swbpipe_values

    record = mbf.to_dict()
    support_nodes = {support: str(entry["node"]) for support, entry in record["support_map"].items()}
    anchors = {str(entry["node"]) for entry in record["support_map"].values() if entry["caepipe_kind"] == "anchor"}
    values, _ = swbpipe_values(payload["mechanics_envelope"]["results"], case_ids, support_nodes, anchors)

    def get(*key, scale=1.0, digits=0):
        return f"{round(values.get(key, 0.0) * scale, digits):.{digits}f}"

    tables: dict = {"anchors": {}, "restraints": {}, "elements": [], "displacements": {}}
    for node, label in record["node_map"].items():
        tables["displacements"][label] = [get("displacement", node, "", axis, scale=1000.0, digits=3) for axis in "xyz"] + [
            get("rotation", node, "", axis, scale=180.0 / math.pi, digits=4) for axis in "xyz"]
    for member, entry in sorted(record["element_map"].items(), key=lambda item: item[1]["layout_order"]):
        ends = ("end_j", "end_i") if entry["reversed"] else ("end_i", "end_j")
        for label, end in zip((entry["from"], entry["to"]), ends):
            tables["elements"].append((label, [get("element_force", member, end, axis) for axis in "xyz"]
                                       + [get("element_moment", member, end, axis) for axis in "xyz"]))
    for node in sorted(set(support_nodes.values())):
        forces = [f"{-float(get('support_force', f'node:{node}', '', axis)):.0f}" for axis in "xyz"]
        if node in anchors:
            moments = [f"{-float(get('support_moment', f'node:{node}', '', axis)):.0f}" for axis in "xyz"]
            tables["anchors"][node] = forces + moments
        else:
            tables["restraints"][node] = forces
    return tables


def mirror_csv(document: dict, payload: dict) -> tuple[object, str]:
    mbf = render_caepipe_mbf_si(document["model"], title=document["model_id"], piping_code=document["piping_code"])
    cases = {label: mirror_tables(payload, mbf, [case]) for case, label in mbf.load_case_map.items()}
    cases[OPERATING_LABEL] = mirror_tables(payload, mbf, sorted(mbf.load_case_map))
    return mbf, csv_text(cases)


def test_every_model_solves_and_its_closed_forms_match_swbpipe_and_a_mirror_screens_within_the_profile():
    """End to end on SWBPIPE's side: each generated model solves; an invented print that mirrors SWBPIPE's own results
    at printed resolution binds every row and screens within the profile; each closed form agrees with SWBPIPE."""
    closed = 0
    for document in earlier_models():
        payload = solve(document["model"])
        mbf, text = mirror_csv(document, payload)
        record = build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf.text,
                                        parsed_caepipe=parse_caepipe_print_csv(text))
        assert record["summary"]["by_classification"] == {WITHIN: record["summary"]["results"]}, document["model_id"]
        assert [case["case"] for case in record["load_cases"]] == ["case:expansion", "case:sustained", "operating"]
        for row in record["results"]:
            if "closed_form" in row:
                closed += 1
                assert row["closed_form"]["closed_form_vs_swbpipe"] == "agree", (document["model_id"], row)
    assert closed == sum(len(document["closed_form"]) for document in earlier_models())


# -- tranche 2: pressure series --------------------------------------------------------------------------------------
def build_has_pressure_elongation_setting() -> bool:
    """Whether this checkout's model document has model_settings.pressure_elongation. Without it (main before the
    setting), a pressurised document solves with pressure elongation on in every case; with it, a document that
    lacks the member solves with elongation off. The runner the tests use is built from the same head."""
    import yaml

    schema = yaml.safe_load((ROOT / "schemas" / "model.schema.yaml").read_text(encoding="utf-8"))
    return "pressure_elongation" in (schema["$defs"]["ModelSettings"].get("properties") or {})


# The elongation-off closed forms, compared only on a build that has the setting. Each is listed so that none is
# left out silently: (model id, number of closed forms valid in the pressure-default state only).
PENDING_ELONGATION_OFF = (
    *((f"cpb-p1-{index:02d}", 2) for index in range(1, 7)),  # both anchors' axial reactions, 0
    *((f"cpb-p2-{index:02d}", 1) for index in range(1, 5)),  # the free end's axial displacement, 0
)
PENDING_REASON = ("the pressure-default closed forms need a build whose model document has "
                  "model_settings.pressure_elongation (solved with the setting absent); this build has none, so a "
                  "pressurised document solves with elongation on and these closed forms are not compared")


def closed_form_rows(document: dict, solved_model: dict) -> list[dict]:
    """The benchmark record's closed-form rows for `document` against SWBPIPE's solve of `solved_model`, with an
    invented print that mirrors that solve."""
    payload = solve(solved_model)
    mbf, text = mirror_csv(document, payload)
    record = build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf.text,
                                    parsed_caepipe=parse_caepipe_print_csv(text))
    return [row for row in record["results"] if "closed_form" in row]


def test_the_elongation_on_closed_forms_match_swbpipe():
    """The pressure-elongation-on run of every pressurised model: on a build with the setting, the document the
    series gives (the setting names the pressure case); without it, the tracked document, which then solves with
    elongation on."""
    has_setting = build_has_pressure_elongation_setting()
    compared = 0
    for document in pressurised_models():
        run = benchmark_models.series_document(document, benchmark_models.SERIES_ELONGATION_ON)
        rows = closed_form_rows(run, run["model"] if has_setting else document["model"])
        assert len(rows) == len(run["closed_form"]), document["model_id"]
        for row in rows:
            assert row["closed_form"]["closed_form_vs_swbpipe"] == "agree", (document["model_id"], row)
        compared += len(rows)
    assert compared == sum(len(document["closed_form"]) + len(document["closed_form_by_series"]
                                                              [benchmark_models.SERIES_ELONGATION_ON])
                           for document in pressurised_models())


def test_the_elongation_off_closed_forms_are_listed_and_compared_only_on_a_build_with_the_setting():
    listed = {document["model_id"]: len(document["closed_form_by_series"][benchmark_models.SERIES_DEFAULT])
              for document in pressurised_models()}
    assert {model: count for model, count in listed.items() if count} == dict(PENDING_ELONGATION_OFF)
    if not build_has_pressure_elongation_setting():
        pytest.skip(f"pending, {sum(listed.values())} closed forms: {PENDING_REASON}")
    for document in pressurised_models():
        run = benchmark_models.series_document(document, benchmark_models.SERIES_DEFAULT)
        assert run["model"] == document["model"]
        rows = closed_form_rows(run, run["model"])
        assert len(rows) == len(run["closed_form"]), document["model_id"]
        for row in rows:
            assert row["closed_form"]["closed_form_vs_swbpipe"] == "agree", (document["model_id"], row)


# The premise of the pressure-bend-opening series, checked only on a build that has the setting, and listed so that
# it cannot drop out silently: (model id, the node whose rotation must not change between the two screened runs).
PENDING_BEND_OPENING = (("cpb-p7-01", "node:B"),)
# Bound on |rotation(on) - rotation(default)| at the free end, in rad, per component. The rotations in this model are
# of order 1e-3 to 1e-2 rad, so double-precision working error is many orders below 1e-12; the result file prints
# rotation to about 1.7e-6 rad (0.0001 deg), so 1e-12 is neither a bitwise demand nor a loose one. The pull request
# 217 author reports bitwise equality out of plane and a residue of order 1e-40 in plane (that author's account).
BEND_OPENING_ROTATION_BOUND = 1.0e-12
BEND_OPENING_REASON = ("the pressure-bend-opening premise (SWBPIPE's free-end rotation unchanged between the "
                       "pressure-default and pressure-elongation-on runs) needs a build whose model document has "
                       "model_settings.pressure_elongation; this build has none, so both runs would solve the same "
                       "state and nothing would be checked")


def test_the_bend_opening_premise_holds_in_swbpipe():
    series = json.loads(MANIFEST.read_text(encoding="utf-8"))["series"]["pressure-bend-opening"]
    assert sorted({member["model_id"] for member in series["members"]}) == [model for model, _ in
                                                                            PENDING_BEND_OPENING]
    assert series["observed"] == "case:sustained rotation about Y of the free end node:B"
    if not build_has_pressure_elongation_setting():
        pytest.skip(f"pending, {len(PENDING_BEND_OPENING)} model: {BEND_OPENING_REASON}")
    from core.handoff.caepipe_external.benchmark import swbpipe_values

    documents = {document["model_id"]: document for document in pressurised_models()}
    for model_id, node in PENDING_BEND_OPENING:
        rotations = []
        for run in RUNS[:2]:
            run_document = benchmark_models.series_document(documents[model_id], run)
            values, invalid = swbpipe_values(solve(run_document["model"])["mechanics_envelope"]["results"],
                                             ["case:sustained"], {})
            rotations.append({axis: values[("rotation", node, "", axis)] for axis in "xyz"})
            assert not {key for key in invalid if key[:2] == ("rotation", node)}
        assert any(abs(value) > 1.0e-6 for value in rotations[0].values())  # the rotation is not trivially zero
        for axis in "xyz":
            assert abs(rotations[1][axis] - rotations[0][axis]) <= BEND_OPENING_ROTATION_BOUND, (model_id, axis,
                                                                                                rotations)


RUNS = (benchmark_models.SERIES_DEFAULT, benchmark_models.SERIES_ELONGATION_ON,
        benchmark_models.SERIES_BOURDON_EXPANSION)


def test_every_pressurised_model_declares_three_runs_of_the_same_file():
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    pressurised = [entry for entry in manifest["models"] if entry["tranche"] == "2"]
    names = [entry["model_id"] for entry in pressurised]
    lines = {benchmark_models.SERIES_DEFAULT: "0,0,0,1,0", benchmark_models.SERIES_ELONGATION_ON: "0,1,0,1,1",
             benchmark_models.SERIES_BOURDON_EXPANSION: "0,1,0,1,0"}
    for name in RUNS:
        declared = manifest["series"][name]
        assert declared["oppres_line"] == lines[name] and declared["screened"] is (name != RUNS[2])
        assert [member["model_id"] for member in declared["members"]] == names
    capture_text = manifest["series"][RUNS[2]]["description"]
    assert manifest["series"][RUNS[2]]["swbpipe"] is None and "collects nothing for it" in capture_text
    assert "capture_path_unavailable" in capture_text and "screened against nothing" not in capture_text
    hypothesis = manifest["pressure_hypothesis"]
    assert hypothesis["statement"].startswith("HYPOTHESIS") and "tests and does not assume" in hypothesis["statement"]
    assert "docs.sstcae.com/users_manual/import_mbf.htm" in hypothesis["format_source"]
    assert "2026-11-12" in hypothesis["format_source"] and hypothesis["refuted_by"]
    documents = {document["model_id"]: document for document in pressurised_models()}
    for entry in pressurised:
        document = documents[entry["model_id"]]
        writer = render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                       reference_temperature_c=document["reference_temperature_c"],
                                       piping_code=document["piping_code"]).text
        assert [run["series"] for run in entry["runs"]] == list(RUNS)
        assert len({run["run_id"] for run in entry["runs"]}) == 3
        default, on, capture = entry["runs"]
        assert default["variant_of"] is None and on["variant_of"] == capture["variant_of"] == RUNS[0]
        assert default["file"] == on["file"] == capture["file"] == entry["file"]
        assert default["mbf"] == {"oppres_line": "0,0,0,1,0", "source": "writer output",
                                  "sha256": hashlib.sha256(writer.encode("utf-8")).hexdigest()}
        for run in (on, capture):
            substituted = benchmark_models.substituted_mbf_text(writer, run["mbf"]["oppres_line"])
            assert substituted != writer and substituted.replace(f"\r\nOPPRES\r\n{run['mbf']['oppres_line']}\r\n",
                                                                 "\r\nOPPRES\r\n0,0,0,1,0\r\n") == writer
            assert run["mbf"] == {"oppres_line": run["mbf"]["oppres_line"], "source": "substituted",
                                  "base_line": "0,0,0,1,0", "writer_output": False,
                                  "sha256": hashlib.sha256(substituted.encode("utf-8")).hexdigest(),
                                  "writer_sha256": default["mbf"]["sha256"]}
        # Each run's file as the runner forms it: the writer's text of that run's own document (for the on run, the
        # document carrying the setting), with that run's line substituted. Its bytes and hash are the manifest's, so a
        # writer change that let model settings reach the MBF would fail here.
        for series, run in zip(RUNS, entry["runs"]):
            run_document = benchmark_models.series_document(document, series)
            own = render_caepipe_mbf_si(run_document["model"], title=run_document["model_id"],
                                        reference_temperature_c=run_document["reference_temperature_c"],
                                        piping_code=run_document["piping_code"]).text
            final = benchmark_models.run_mbf_text(run_document, own).encode("utf-8")
            expected = benchmark_models.substituted_mbf_text(writer, run["mbf"]["oppres_line"]).encode("utf-8")
            assert final == expected and hashlib.sha256(final).hexdigest() == run["mbf"]["sha256"], run["run_id"]
        assert default["swbpipe_model_hash"] == entry["model_hash"] != on["swbpipe_model_hash"]
        assert on["swbpipe_model_settings"] == {"pressure_elongation": {"operating_load_cases": ["case:sustained"]}}
        assert (capture["screened"], capture["swbpipe_model_hash"], capture["closed_forms"]) == (False, None, 0)
    # The thrust series: one target-program input, SWBPIPE's anchor terminal transferring or not.
    for index in (1, 2):
        members = manifest["series"][f"pressure-closure-{index:02d}"]["members"]
        assert [(member["model_id"], member["value"]) for member in members] == [
            (f"cpb-p2-{index:02d}", "transfers_to_wall"), (f"cpb-p3-{index:02d}", "separately_supported_or_compensated")]
    # The bend-opening series: P7's default and on runs.
    assert manifest["series"]["pressure-bend-opening"]["members"] == [
        {"model_id": "cpb-p7-01", "run": RUNS[0]}, {"model_id": "cpb-p7-01", "run": RUNS[1]}]
    assert not [entry for entry in manifest["models"] if entry["tranche"] == "1" and "runs" in entry]


def test_the_substitution_refuses_a_file_without_exactly_one_writer_oppres_line():
    writer = render_caepipe_mbf_si(pressurised_models()[0]["model"], title="T", piping_code="B311-80").text
    for text in (writer.replace("\r\nOPPRES\r\n0,0,0,1,0\r\n", "\r\nOPPRES\r\n0,0,0,1,1\r\n"),
                 writer + "OPPRES\r\n0,0,0,1,0\r\n", writer.replace("OPPRES", "OPTEMP")):
        with pytest.raises(ValueError):
            benchmark_models.substituted_mbf_text(text, "0,1,0,1,1")
    assert benchmark_models.substituted_mbf_text(writer, "0,0,0,1,0") == writer


def test_only_the_data_line_of_a_listed_options_heading_may_be_substituted():
    """DEL-17-05 REQ-005: whole lines under a heading of the maintained short list, which changes no node, element or
    support; for this set, the pressure options record as the writer writes it."""
    assert benchmark_models.SUBSTITUTABLE_OPTION_HEADINGS == ("OPPRES",)
    writer = render_caepipe_mbf_si(pressurised_models()[0]["model"], title="T", piping_code="B311-80").text
    lines = writer.split("\r\n")
    # A layout, material, section or load line is refused, whatever replaces it.
    for heading in ("LAYOUT", "MATERIAL", "PIPE", "LOADS"):
        data = lines[lines.index(heading) + 1]
        with pytest.raises(ValueError, match="not an options heading"):
            benchmark_models.substituted_mbf_text(writer, data, heading=heading)
        with pytest.raises(ValueError, match="not an options heading"):
            benchmark_models.substituted_mbf_text(writer, "0,1,0,1,1", heading=heading)
    # Under OPPRES only a declared run's line, with no carriage return or line feed, replaces the writer's line.
    for line in ("0,1,0,1,1\r\nLAYOUT", "0,1,0,1,1\n", "0,1,1,1,1", "1,0,0,1,0", ""):
        with pytest.raises(ValueError):
            benchmark_models.substituted_mbf_text(writer, line)
    substituted = benchmark_models.substituted_mbf_text(writer, "0,1,0,1,1").split("\r\n")
    assert [index for index, (old, new) in enumerate(zip(lines, substituted)) if old != new] == [
        lines.index("OPPRES") + 1] and len(substituted) == len(lines)


def test_inexact_inputs_follow_the_run_and_the_case():
    bends = {"P4", "P5", "P7"}
    for document in pressurised_models():
        default = benchmark_models.series_document(document, RUNS[0])["inexact_inputs"]
        on = benchmark_models.series_document(document, RUNS[1])["inexact_inputs"]
        assert document["inexact_inputs"] == default
        for case in ("case:sustained", "operating"):
            assert "pressure_thrust_convention" in default[case] and "pressure_thrust_convention" in on[case]
            assert "pressure_elongation_convention" not in default[case] and "pressure_elongation_convention" in on[case]
            assert "bend_bourdon_opening" not in default[case]
            assert ("bend_bourdon_opening" in on[case]) is (document["family"] in bends)
        assert default["case:expansion"] == on["case:expansion"]
        assert not {"pressure_elongation_convention", "bend_bourdon_opening", "pressure_thrust_convention"} & set(
            on["case:expansion"])
        if document["family"] in ("P1", "P2", "P3", "P7"):
            # Axial growth only (P1), or free growth of a cantilever anchored at one end with no pressure or
            # mechanical load (P2, P3, P7): no shear or bend input can change it, so the exact class in both runs.
            assert default["case:expansion"] == [] == on["case:expansion"]
            assert screening_class(default["case:expansion"]) == screening_class(on["case:expansion"]) == "exact"
        else:
            # Anchored at both ends or at three legs: bending is restrained, so the inputs stay named.
            assert "shear_deformation" in default["case:expansion"]
        capture = benchmark_models.series_document(document, RUNS[2])
        assert capture["capture_only"] is True and capture["closed_form"] == [] and capture["inexact_inputs"] == {}


def test_no_tracked_model_carries_model_settings_and_the_series_document_adds_it_without_writing():
    for document in tracked_models():
        assert "model_settings" not in document["model"], document["model_id"]
    document = pressurised_models()[0]
    before = json.dumps(document, sort_keys=True)
    on = benchmark_models.series_document(document, benchmark_models.SERIES_ELONGATION_ON)
    assert on == benchmark_models.series_document(document, benchmark_models.SERIES_ELONGATION_ON)
    assert json.dumps(document, sort_keys=True) == before
    assert on["model"]["model_settings"] == {"pressure_elongation": {"operating_load_cases": ["case:sustained"]}}
    assert {key: value for key, value in on["model"].items() if key != "model_settings"} == document["model"]
    assert on["series_run"] == benchmark_models.SERIES_ELONGATION_ON and "closed_form_by_series" not in on
    with pytest.raises(ValueError):
        benchmark_models.series_document(tracked_models()[0], benchmark_models.SERIES_DEFAULT)
    assert benchmark_models.generate() == benchmark_models.generate()  # nothing the call changed reaches the set


def test_every_pressurised_case_carries_an_unbalanced_thrust_or_one_straight_member_per_region():
    """Each region is one straight member, or holds a bend (whose kink leaves a thrust), or ends at a terminal that
    is separately supported or compensated; so no case is a multi-member closed straight run without one."""
    for document in pressurised_models():
        model = document["model"]
        bends = {component["geometry"]["bend_pipe_ref"] for component in model["components"]}
        regions = [region for case in model["load_cases"] for region in case["pressure_regions"]]
        assert regions and all(case["id"] == "case:sustained" for case in model["load_cases"]
                               if case["pressure_regions"])
        for region in regions:
            members = region["member_pipe_ids"]
            open_end = any(terminal["closure_transfer"] != "transfers_to_wall" for terminal in region["terminals"])
            assert len(members) == 1 or set(members) & bends or open_end, (document["model_id"], region["id"])
        assert "pressure_thrust_convention" in document["inexact_inputs"]["case:sustained"]
        assert model["pressure_contract"] == {"version": "3.0.0", "mode": "exact_pressure_v3",
                                              "shear_model": "timoshenko_annulus_energy_v1"}


# -- element-row binding ---------------------------------------------------------------------------------------------
ELEMENT_MAP = {
    "pipe:A": {"from": "10", "to": "20", "reversed": False, "layout_order": 0},
    "pipe:B": {"from": "20", "to": "30", "reversed": True, "layout_order": 1},
    "pipe:C": {"from": "20", "to": "40", "reversed": False, "layout_order": 2},
}


def elements(*pairs: tuple[str, str]) -> list[dict]:
    text = csv_text({"Invented (W)": {"elements": [
        (node, [str(index * 10 + offset), "0", "0", "0", "0", str(index)])
        for index, pair in enumerate(pairs) for offset, node in enumerate(pair)]}})
    return parse_caepipe_print_csv(text)["load_cases"]["Invented (W)"]["element_forces_global"]


def test_element_rows_bind_to_members_by_their_nodes_through_the_id_map():
    binding = bind_element_rows(elements(("10", "20"), ("20", "30"), ("20", "40")), ELEMENT_MAP)
    assert binding["basis"] == "writer_id_map_node_pairs"
    assert binding["unmatched_rows"] == [] and binding["unmatched_members"] == [] and binding["notes"] == []
    assert binding["bound"]["pipe:A"]["end_i"] == {"caepipe_node": "10", "values": {**zeros(), "fx": 0.0}}
    assert binding["bound"]["pipe:A"]["end_j"]["caepipe_node"] == "20"
    # A reversed member: the layout runs 20 -> 30, SWBPIPE's member runs 30 -> 20, so end_i is the row at 30.
    assert binding["bound"]["pipe:B"]["end_i"] == {"caepipe_node": "30", "values": {**zeros(), "fx": 11.0, "mz": 1.0}}
    assert binding["bound"]["pipe:B"]["end_j"] == {"caepipe_node": "20", "values": {**zeros(), "fx": 10.0, "mz": 1.0}}


def zeros() -> dict:
    return dict.fromkeys(("fx", "fy", "fz", "mx", "my", "mz"), 0.0)


def test_binding_does_not_depend_on_row_position_and_notes_drift():
    binding = bind_element_rows(elements(("40", "20"), ("10", "20"), ("20", "30")), ELEMENT_MAP)
    assert sorted(binding["bound"]) == ["pipe:A", "pipe:B", "pipe:C"] and binding["unmatched_rows"] == []
    assert binding["bound"]["pipe:C"]["end_i"]["caepipe_node"] == "20"
    assert binding["bound"]["pipe:C"]["end_i"]["values"]["fx"] == 1.0  # the second row of the first pair
    assert {(note["member"], note["note"].split(" ")[0]) for note in binding["notes"]} == {
        ("pipe:C", "rows"), ("pipe:C", "at"), ("pipe:A", "at"), ("pipe:B", "at")}


def test_unbound_rows_and_members_are_reported_never_dropped():
    binding = bind_element_rows(elements(("10", "20"), ("10", "20"), ("20", "50")), ELEMENT_MAP)
    assert sorted(binding["bound"]) == ["pipe:A"]
    assert [(row["element_index"], row["reason"]) for row in binding["unmatched_rows"]] == [
        (1, "a second row pair for pipe:A"), (2, "no member of the writer's element_map joins these nodes")]
    assert [item["member"] for item in binding["unmatched_members"]] == ["pipe:B", "pipe:C"]
    twice = {**ELEMENT_MAP, "pipe:D": {"from": "20", "to": "10", "reversed": False, "layout_order": 3}}
    assert bind_element_rows(elements(("10", "20")), twice)["unmatched_rows"][0]["reason"] == \
        "2 members join these nodes: pipe:A, pipe:D"


# -- two-way comparison ----------------------------------------------------------------------------------------------
def result_row(kind, entity, case, component, value, unit, location="node"):
    return {"kind": kind, "entity_ref": entity, "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "metadata": {"component": component, "location": location}, "unit": unit, "value": value}


def invented_cantilever() -> dict:
    """An invented two-element cantilever along X with a restraint at mid-length, in the generator's document."""
    b = benchmark_models.Builder("cpb-test-01", 0)
    a, m, t = b.node("A", (0, 0, 0)), b.node("M", (2, 0, 0)), b.node("B", (4, 0, 0))
    b.run("P", [a, m, t], 168.3, 7.11)
    b.anchor("A", a)
    b.support("R", m, ["UZ"], None)
    b.force("tip", t, "global_y", -1000.0)
    b.add_thermal(100.0)
    return {"model_id": "cpb-test-01", "family": "G1", "piping_code": "B311-80", "reference_temperature_c": 20.0,
            "inexact_inputs": {"case:sustained": ["shear_deformation"], "case:expansion": [],
                               "operating": ["shear_deformation"]},
            "closed_form": [{"quantity": "displacement", "component": "x", "load_case": "case:expansion",
                             "node": "node:B", "value": 0.0048, "unit": "m", "formula": "alpha dT L",
                             "basis": "invented"}],
            "model": b.model()}


def invented_payload() -> dict:
    rows = []
    for case, uy, ux, fy in (("case:sustained", -0.004, 0.0, 1000.0), ("case:expansion", 0.0, 0.0048, 0.0)):
        rows += [result_row(f"global_nodal_displacement_{axis}", node, case, axis, 0.0, "mm")
                 for node in ("node:A", "node:M") for axis in "xyz"]
        rows += [result_row(f"global_nodal_rotation_{axis}", node, case, axis, 0.0, "rad")
                 for node in ("node:A", "node:M", "node:B") for axis in "xyz"]
        rows += [result_row("global_nodal_displacement_x", "node:B", case, "x", ux * 1000.0, "mm"),
                 result_row("global_nodal_displacement_y", "node:B", case, "y", uy * 1000.0, "mm"),
                 result_row("global_nodal_displacement_z", "node:B", case, "z", 0.0, "mm")]
        for support in ("support:A", "support:R"):
            for component in ("Fx", "Fy", "Fz", "Mx", "My", "Mz"):
                value = fy if (support, component) == ("support:A", "Fy") else 0.0
                value = 4000.0 if (support, component) == ("support:A", "Mz") and fy else value
                value = 2.5 if (support, component) == ("support:R", "Mx") else value  # not printed for restraints
                rows.append(result_row("support_reaction_component_v2", support, case, component, value,
                                       "N" if component[0] == "F" else "N*m"))
        for pipe in ("pipe:P1", "pipe:P2"):
            for end in ("end_i", "end_j"):
                for component in ("Fx", "Fy", "Fz"):
                    rows.append(result_row("element_global_end_force_v1", pipe, case, component,
                                           fy if component == "Fy" and end == "end_i" else
                                           -fy if component == "Fy" else 0.0, "N", end))
                for component in ("Mx", "My", "Mz"):
                    rows.append(result_row("element_global_end_moment_v1", pipe, case, component, 0.0, "N*m", end))
    return {"mechanics_envelope": {"results": rows}}


def invented_print(*, sustained_uy="-4", drop_label=None, extra_anchor=False) -> str:
    def tables(uy: str, ux: str, fy: str, mz: str) -> dict:
        element = {"10": ["0", fy, "0", "0", "0", "0"], "20": ["0", f"{-float(fy):g}", "0", "0", "0", "0"]}
        return {
            "anchors": {"10": ["0", f"{-float(fy):g}", "0", "0", "0", mz], **({"90": ["1"] * 6} if extra_anchor else {})},
            "restraints": {"20": ["0", "0", "0"]},
            "elements": [("10", element["10"]), ("20", element["20"]), ("20", element["10"]), ("30", element["20"])],
            "displacements": {"10": ["0"] * 6, "20": ["0"] * 6, "30": [ux, uy, "0", "0", "0", "0"]},
        }
    cases = {"Sustained (W+P)": tables(sustained_uy, "0", "1000", "-4000"),
             "Expansion (T1)": tables("0", "4.8", "0", "0"),
             OPERATING_LABEL: tables(sustained_uy, "4.8", "1000", "-4000")}
    if drop_label:
        cases.pop(drop_label)
    return csv_text(cases)


def compare(text: str, document: dict | None = None) -> dict:
    document = document or invented_cantilever()
    mbf = render_caepipe_mbf_si(document["model"], title="INVENTED", piping_code=document["piping_code"])
    assert mbf.status == "written", mbf.diagnostics
    return build_benchmark_record(document, swbpipe_result_payload=invented_payload(), mbf_result=mbf,
                                  mbf_text=mbf.text, parsed_caepipe=parse_caepipe_print_csv(text))


def find(record: dict, case: str, kind: str, entity: str, component: str, end: str | None = None) -> dict:
    return next(row for row in record["results"] if row["case"] == case and row["kind"] == kind
                and row["entity"] == entity and row.get("component") == component and row.get("end") == end)


def test_the_two_way_record_screens_every_bound_scalar_without_an_analytic_value():
    record = compare(invented_print())
    assert record["document_kind"] == BENCHMARK_RECORD_KIND
    assert record["summary"]["by_classification"] == {WITHIN: record["summary"]["results"]}
    assert [(case["case"], case["screening_class"]) for case in record["load_cases"]] == [
        ("case:expansion", "exact"), ("case:sustained", "named_inexact"), ("operating", "named_inexact")]
    tip = find(record, "case:sustained", "displacement", "node:B", "y")
    assert tip["values"] == {"swbpipe": pytest.approx(-0.004), "caepipe": pytest.approx(-0.004)}
    # Support loads: the printed pipe-on-support load is sign-flipped; the restraint's moments are not compared.
    assert find(record, "case:sustained", "support_moment", "node:10", "z")["values"]["caepipe"] == 4000.0
    assert not [row for row in record["results"] if row["kind"] == "support_moment" and row["entity"] == "node:20"]
    # The operating case is SWBPIPE's sum of its primitive cases.
    operating = find(record, "operating", "displacement", "node:B", "x")
    assert operating["values"]["swbpipe"] == pytest.approx(0.0048)
    assert record["load_cases"][2]["basis"].startswith("SWBPIPE: the sum of its mapped primitive cases")
    element = find(record, "case:sustained", "element_force", "pipe:P2", "y", "end_j")
    assert element["values"] == {"swbpipe": -1000.0, "caepipe": -1000.0}
    closed = find(record, "case:expansion", "displacement", "node:B", "x")
    assert closed["values"]["closed_form"] == 0.0048
    assert closed["closed_form"] == {"formula": "alpha dT L", "basis": "invented", "closed_form_vs_swbpipe": "agree",
                                     "closed_form_vs_caepipe": "agree"}


def test_a_difference_outside_the_profile_is_a_finding_and_inside_is_not():
    within = compare(invented_print(sustained_uy="-4.019"))  # 19 um on 4 mm: under 5e-3 * 4 mm + 1 um
    assert find(within, "case:sustained", "displacement", "node:B", "y")["classification"] == WITHIN
    record = compare(invented_print(sustained_uy="-4.03"))
    row = find(record, "case:sustained", "displacement", "node:B", "y")
    assert row["classification"] == FINDING
    assert row["screen"]["allowed"] == pytest.approx(1e-6 + 5e-3 * 0.00403)
    assert row["screen"]["difference"] == pytest.approx(-0.00003)


def test_the_exact_class_and_the_floor_and_near_zero_scale():
    assert screen(1.0, 1.0 + 1.5e-4, "m", "exact", 1.0)["within"] is False
    assert screen(1.0, 1.0 + 0.5e-4, "m", "exact", 1.0)["within"] is True
    # Below the printed unit, a scalar agrees whatever its relative difference.
    assert screen(0.0, 0.9, "N", "exact", 0.0)["within"] is True and screen(0.0, 1.1, "N", "exact", 0.0)["within"] is False
    # Near zero against a large case maximum: judged against eta * Q.
    assert screen(0.0, 5.0, "N", "named_inexact", 1.0e6)["within"] is True
    assert SCREENING_PROFILE["relative"] == {"exact": 1e-4, "named_inexact": 5e-3}


def test_a_closed_form_that_disagrees_with_swbpipe_makes_a_finding():
    document = invented_cantilever()
    document["closed_form"][0]["value"] = 0.0049
    row = find(compare(invented_print(), document), "case:expansion", "displacement", "node:B", "x")
    assert row["closed_form"]["closed_form_vs_swbpipe"] == "disagree"
    assert row["classification"] == FINDING and row["reason"] == "SWBPIPE differs from the model's closed form"


def test_unmatched_rows_and_status_mismatches_are_findings_never_dropped():
    record = compare(invented_print(drop_label="Expansion (T1)", extra_anchor=True))
    mismatch = [row for row in record["results"] if row["classification"] == STATUS_MISMATCH]
    assert [(row["case"], row["entity"]) for row in mismatch] == [("case:expansion", "Expansion (T1)")]
    extra = [row for row in record["results"] if row["kind"] == "unbound_row"]
    assert {(row["case"], row["entity"], row["section"]) for row in extra} == {
        ("case:sustained", "90", "anchor_loads_global"), ("operating", "90", "anchor_loads_global")}
    assert all(row["classification"] == UNMATCHED for row in extra)
    missing = compare(invented_print().replace("30,4.8,", "31,4.8,").replace("30,0,-4,", "31,0,-4,"))
    only = find(missing, "case:sustained", "displacement", "node:B", "y")
    assert only["classification"] == UNMATCHED and only["reason"] == "only SWBPIPE has this value"


def edited_payload(edit) -> dict:
    payload = invented_payload()
    payload["mechanics_envelope"]["results"] = [row for row in map(edit, payload["mechanics_envelope"]["results"])
                                                if row is not None]
    return payload


def compare_payload(payload: dict, text: str | None = None) -> dict:
    document = invented_cantilever()
    mbf = render_caepipe_mbf_si(document["model"], title="INVENTED", piping_code=document["piping_code"])
    return build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf.text,
                                  parsed_caepipe=parse_caepipe_print_csv(text or invented_print()))


def tip_x(row: dict) -> bool:
    return row["kind"] == "global_nodal_displacement_x" and row["entity_ref"] == "node:B"


def test_an_operating_sum_that_overflows_is_unmatched_never_within():
    huge = edited_payload(lambda row: {**row, "value": 1.5e308, "unit": "m"} if tip_x(row) else row)
    row = find(compare_payload(huge), "operating", "displacement", "node:B", "x")
    assert row["classification"] == UNMATCHED and row["values"]["swbpipe"] is None
    assert screen(math.inf, math.inf, "m", "exact", math.inf)["within"] is False
    assert screen(0.0, 1.0, "m", "exact", math.nan)["within"] is False


@pytest.mark.parametrize("edit", [
    lambda row: None if tip_x(row) and row["basis_ref"]["ref_id"] == "case:expansion" else row,
    lambda row: {**row, "value": math.nan} if tip_x(row) and row["basis_ref"]["ref_id"] == "case:expansion" else row,
])
def test_an_operating_sum_with_a_missing_or_invalid_contribution_is_unmatched(edit):
    record = compare_payload(edited_payload(edit))
    row = find(record, "operating", "displacement", "node:B", "x")
    assert row["classification"] == UNMATCHED and row["values"]["swbpipe"] is None
    assert "missing from a case or not a finite number" in row["reason"]
    assert find(record, "case:sustained", "displacement", "node:B", "x")["classification"] == WITHIN


def co_located(values: dict) -> list[dict]:
    """Invented reactions of supports S1 and S2, both at CAEPIPE node 10: {(support, case): Fy in N}."""
    return [result_row("support_reaction_component_v2", support, case, "Fy", value, "N")
            for (support, case), value in values.items()]


def test_co_located_supports_are_complete_per_support_before_the_node_sum():
    from core.handoff.caepipe_external.benchmark import swbpipe_values

    key = ("support_force", "node:10", "", "y")
    nodes = {"support:S1": "10", "support:S2": "10"}
    whole = co_located({("support:S1", "c1"): 1.0, ("support:S2", "c1"): 2.0,
                        ("support:S1", "c2"): 4.0, ("support:S2", "c2"): 8.0})
    assert swbpipe_values(whole, ["c1", "c2"], nodes) == ({key: 15.0}, set())
    # S2 is missing from c2: the node total is partial, so it is not screened.
    partial = [row for row in whole if (row["entity_ref"], row["basis_ref"]["ref_id"]) != ("support:S2", "c2")]
    assert swbpipe_values(partial, ["c1", "c2"], nodes) == ({}, {key})
    # Per-case totals that overflow in opposite directions are unmatched, never an exception.
    huge = co_located({("support:S1", "c1"): 1e308, ("support:S2", "c1"): 1e308,
                       ("support:S1", "c2"): -1e308, ("support:S2", "c2"): -1e308})
    assert swbpipe_values(huge, ["c1", "c2"], nodes) == ({}, {key})


def test_a_parser_error_is_counted_unmatched_never_dropped():
    text = invented_print().replace("30,0,-4,0,0,0,0,,", "30,0,-4,0,0,0,0,,\r\n30,0,-5,0,0,0,0,,", 1)
    parsed = parse_caepipe_print_csv(text)
    assert [item for item in parsed["diagnostics"] if item["severity"] == "error"]
    record = compare_payload(invented_payload(), text)
    errors = [row for row in record["results"] if row["kind"] == "parser_error"]
    assert errors and all(row["classification"] == UNMATCHED for row in errors)
    assert record["summary"]["by_classification"].get(UNMATCHED, 0) >= len(errors)


# -- run_pilot --benchmark -------------------------------------------------------------------------------------------
def benchmark_manifest(tmp_path: Path, count: int = 5, ids: tuple[str, ...] = ()) -> Path:
    """A copy of the first `count` tracked models (or of the models `ids`) and a manifest naming them."""
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    manifest["models"] = ([entry for entry in manifest["models"] if entry["model_id"] in ids] if ids
                          else manifest["models"][:count])
    (tmp_path / "models").mkdir(parents=True)
    for entry in manifest["models"]:
        (tmp_path / entry["file"]).write_bytes((BENCHMARK / entry["file"]).read_bytes())
    path = tmp_path / "manifest.json"
    path.write_text(json.dumps(manifest), encoding="utf-8")
    return path


@pytest.fixture
def fake_benchmark(tmp_path, monkeypatch):
    """A configured harness with a fake SWBPIPE solve, a fake guarded round trip that prints an invented CSV, a fake
    guest liveness check and a recorded sleep (nothing waits and no guest is reached)."""
    config = dataclasses.replace(fake_config(tmp_path), settings_guard=INVENTED_GUARD)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: (
        {"invented": case["source_location"]}, {"payload": invented_payload()}, ""))
    state = {"calls": [], "outcomes": [], "session": [], "alive": [], "sleeps": [], "events": [], "mbf": {},
             "clean": {"settings_guard": {"status": "unchanged"}}}

    def round_trip(config_, case_id, mbf_text, **kwargs):
        state["calls"].append((case_id, kwargs))
        state["mbf"][case_id] = mbf_text
        state["events"].append(("round_trip", case_id))
        outcome = state["outcomes"].pop(0) if state["outcomes"] else state["clean"]
        if isinstance(outcome, BaseException):
            raise outcome
        sha = hashlib.sha256(mbf_text.encode("utf-8")).hexdigest()
        out = case_paths(config_, case_id)["caepipe_out"]
        (out / "model.csv").write_text(invented_print(), encoding="utf-8")
        record = {"document_kind": RUN_RECORD_KIND, "route": "csv_from_batch", "mbf": {"sha256": sha},
                  "guest_mbf_sha256": sha, "observed_csv_path": {"host": "caepipe_out/model.csv"}} | outcome
        (out / "run_record.json").write_text(json.dumps(record), encoding="utf-8")
        return record

    def alive(config_):
        state["events"].append(("liveness",))
        return state["alive"].pop(0) if state["alive"] else {"status": "alive"}

    def precondition(config_):
        state["session"].append(1)
        state["events"].append(("precondition",))
        return {"status": state.get("pre", "clear")}

    real = run_pilot.run_benchmark_model
    monkeypatch.setattr(run_pilot, "run_benchmark_model",
                        lambda entry, config_, **kw: real(entry, config_, **{**kw, "round_trip": round_trip}))
    monkeypatch.setattr(run_pilot, "check_no_foreign_instance", precondition)
    monkeypatch.setattr(run_pilot, "check_guest_alive", alive)
    monkeypatch.setattr(run_pilot.time, "sleep", lambda seconds: (state["sleeps"].append(seconds),
                                                                  state["events"].append(("sleep", seconds))))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    stops = tmp_path / "stops"
    stops.mkdir()
    state.update(config=config, runner=str(runner), manifest=benchmark_manifest(tmp_path / "set"),
                 freeze=stops / "FREEZE", exclusive=stops / "exclusive.pending")
    state["argv"] = ["--benchmark", str(state["manifest"]), "--runner", str(runner), "--freeze-file",
                     str(state["freeze"]), "--exclusive-file", str(state["exclusive"])]
    return state


def test_the_benchmark_option_writes_a_private_record_per_model_one_at_a_time(fake_benchmark, capsys):
    state = fake_benchmark
    assert run_pilot.main(state["argv"], environ={}) == 0
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-02", "cpb-g1-03", "cpb-g1-04", "cpb-g1-05"]
    # The invented guard restores the recent-files subkey only, so the GUI print fallback is off.
    assert all(call[1] == {"gui_fallback": False} for call in state["calls"])
    # Before each model: liveness, then the no-foreign-instance precondition; 60 s between models.
    first = [("liveness",), ("precondition",), ("round_trip", "cpb-g1-01")]
    assert state["events"][:3] == first
    assert state["events"][3:7] == [("sleep", 60.0), ("liveness",), ("precondition",), ("round_trip", "cpb-g1-02")]
    assert state["sleeps"] == [60.0] * 4 and len(state["session"]) == 5
    out = capsys.readouterr().out
    assert "batch 1/2: cpb-g1-01, cpb-g1-02, cpb-g1-03, cpb-g1-04" in out and "batch 2/2: cpb-g1-05" in out
    root = state["config"].exchange_root
    record = json.loads((root / "cases" / "cpb-g1-01" / "comparison" / "benchmark_comparison.json").read_text())
    assert record["document_kind"] == BENCHMARK_RECORD_KIND and record["piping_code"] == "B311-80"
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["mode"], manifest["outcome"], manifest["route"], manifest["attempt"]) == (
        "benchmark_live", "recorded", "csv_from_batch", 1)
    rendered = (root / "cases" / "cpb-g1-01" / "caepipe_in" / "model.mbf").read_text()
    assert "HGRA,B311-80,SI" in rendered
    # Operational status only on standard output: no classification or count.
    assert "within_profile" not in out and "finding" not in out
    # One batch alone; then the stored, bound CSVs rebuild the records offline with no round trip and no wait.
    state["calls"].clear()
    assert run_pilot.main(state["argv"] + ["--batch", "2", "--pace-s", "90"], environ={}) == 0
    assert [call[0] for call in state["calls"]] == ["cpb-g1-05"]
    state["calls"].clear()
    state["sleeps"].clear()
    offline = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline"]
    assert run_pilot.main(offline + ["--case", "cpb-g1-02"], environ={}) == 0
    assert state["calls"] == [] and state["sleeps"] == []
    assert "cpb-g1-02: route=csv_from_batch; benchmark record written" in capsys.readouterr().out


def test_a_model_that_declares_runs_is_refused_without_a_series(tmp_path, capsys):
    path = benchmark_manifest(tmp_path / "set", ids=("cpb-g1-01", "cpb-p1-01"))
    with pytest.raises(ValueError, match=r"name one with --series: cpb-p1-01 \(runs: pressure-bourdon-expansion-"
                                         r"capture, pressure-default, pressure-elongation-on\)"):
        run_pilot.load_benchmark(path)
    # A model without runs, alone, loads as before; with a series it has no run of, it is refused by name.
    assert [document["model_id"] for document in run_pilot.load_benchmark(path, ["cpb-g1-01"])] == ["cpb-g1-01"]
    assert "run_id" not in run_pilot.load_benchmark(path, ["cpb-g1-01"])[0]
    with pytest.raises(ValueError, match="no pressure-default run is declared for: cpb-g1-01"):
        run_pilot.load_benchmark(path, ["cpb-g1-01"], benchmark_models.SERIES_DEFAULT)
    with pytest.raises(ValueError, match="no model declares a pressure-x run"):
        run_pilot.load_benchmark(path, None, "pressure-x")
    # With a series and no selection, the models declaring that run are taken.
    taken = run_pilot.load_benchmark(path, None, benchmark_models.SERIES_DEFAULT)
    assert [document["run_id"] for document in taken] == ["cpb-p1-01--pressure-default"]


def test_each_run_takes_its_state_document_closed_forms_and_mbf(tmp_path):
    path = benchmark_manifest(tmp_path / "set", ids=("cpb-p2-01",))
    document = json.loads((BENCHMARK / "models" / "cpb-p2-01.json").read_text(encoding="utf-8"))
    for series in RUNS:
        [run] = run_pilot.load_benchmark(path, ["cpb-p2-01"], series)
        assert run == benchmark_models.series_document(document, series)
        assert run["run_id"] == f"cpb-p2-01--{series}" and run["model_id"] == "cpb-p2-01"
    default, on, capture = (run_pilot.load_benchmark(path, None, series)[0] for series in RUNS)
    assert default["model"] == document["model"] and "model_settings" not in default["model"]
    assert on["model"]["model_settings"] == {"pressure_elongation": {"operating_load_cases": ["case:sustained"]}}
    assert default["closed_form"] == document["closed_form"] + document["closed_form_by_series"][RUNS[0]]
    assert on["closed_form"] == document["closed_form"] + document["closed_form_by_series"][RUNS[1]]
    assert [entry["formula"] for entry in on["closed_form"] if entry["load_case"] == "case:sustained"
            and entry["quantity"] == "displacement"] == ["(1 - 2 nu) p A_i L/(E A_s)"]
    assert [entry["value"] for entry in default["closed_form"] if entry["load_case"] == "case:sustained"
            and entry["quantity"] == "displacement"] == [0.0]
    assert capture["capture_only"] and capture["closed_form"] == []
    writer = render_caepipe_mbf_si(document["model"], title="cpb-p2-01", piping_code="B311-80").text
    for run, line in ((default, "0,0,0,1,0"), (on, "0,1,0,1,1"), (capture, "0,1,0,1,0")):
        assert f"\r\nOPPRES\r\n{line}\r\n" in benchmark_models.run_mbf_text(run, writer)


def test_two_states_of_one_model_leave_two_distinct_records(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    path = benchmark_manifest(state["manifest"].parent.parent / "pressure-set", ids=("cpb-p2-01",))
    solved = {}

    def solve(runner, case):
        solved[case["case_id"]] = case["model"]
        return {"invented": case["source_location"]}, {"payload": invented_payload()}, ""
    monkeypatch.setattr(run_pilot, "solve_swbpipe", solve)
    argv = ["--benchmark", str(path)] + state["argv"][2:]
    assert run_pilot.main(argv, environ={}) == 2
    assert "these models declare runs; name one with --series: cpb-p2-01" in capsys.readouterr().err
    root = state["config"].exchange_root
    for series in RUNS[:2]:
        assert run_pilot.main(argv + ["--series", series], environ={}) == 0
    run_ids = [f"cpb-p2-01--{series}" for series in RUNS[:2]]
    assert [call[0] for call in state["calls"]] == run_ids
    assert [state["mbf"][run_id].split("\r\nOPPRES\r\n")[1].split("\r\n")[0] for run_id in run_ids] == [
        "0,0,0,1,0", "0,1,0,1,1"]
    assert sorted(solved) == run_ids and "model_settings" not in solved[run_ids[0]]
    assert solved[run_ids[1]]["model_settings"]["pressure_elongation"] == {"operating_load_cases": ["case:sustained"]}
    records = [root / "cases" / run_id / "comparison" / "benchmark_comparison.json" for run_id in run_ids]
    assert records[0].is_file() and records[1].is_file()
    first, second = (json.loads(record.read_text()) for record in records)
    assert first["model_id"] == second["model_id"] == "cpb-p2-01"
    assert "pressure_elongation_convention" not in first["load_cases"][1]["inexact_inputs"]
    assert "pressure_elongation_convention" in second["load_cases"][1]["inexact_inputs"]
    manifests = [json.loads((root / "cases" / run_id / "case_manifest.json").read_text()) for run_id in run_ids]
    assert [(item["case_id"], item["series_run"], item["oppres_line"], item["outcome"]) for item in manifests] == [
        (run_ids[0], RUNS[0], "0,0,0,1,0", "recorded"), (run_ids[1], RUNS[1], "0,1,0,1,1", "recorded")]


CAPTURE_REFUSAL = ("cpb-p2-01--pressure-bourdon-expansion-capture: route=capture_path_unavailable; a capture-only run "
                   "needs the capture-only path, which collects its result without reading it; that path is a "
                   "separate change and is not on this build; not run")


def test_a_capture_run_is_refused_by_name_and_nothing_is_read_or_written(fake_benchmark, monkeypatch, capsys):
    """On this build a capture run is declared and refused by name, live or offline, at every entry point, before
    anything is solved, written, collected, hashed or read. An invented stored result is seeded for its case, and every
    spy records its call (and then raises), so a read through any parser alias, a recognition helper that swallows
    the raised error, or a plain open of the seeded files still fails this test."""
    import builtins
    import io
    from core.handoff.caepipe_external import harness, print_csv
    import core.handoff.caepipe_external as external

    state = fake_benchmark
    path = benchmark_manifest(state["manifest"].parent.parent / "pressure-set", ids=("cpb-p2-01", "cpb-g1-01"))
    [capture] = run_pilot.load_benchmark(path, ["cpb-p2-01"], RUNS[2])
    [plain] = run_pilot.load_benchmark(path, ["cpb-g1-01"])
    config = state["config"]
    root = config.exchange_root
    # An invented stored result for the capture run's case: obviously made-up text and a run record naming it.
    out = case_paths(config, capture["run_id"])["caepipe_out"]
    out.mkdir(parents=True, exist_ok=True)
    (out / "model.csv").write_text("INVENTED CAPTURE FIXTURE, NOT A RESULT OF ANY PROGRAM,\r\n", encoding="utf-8")
    (out / "run_record.json").write_text(json.dumps({"document_kind": RUN_RECORD_KIND, "route": "csv_from_batch",
                                                     "invented": True}), encoding="utf-8")
    watched = (root / "cases" / capture["run_id"]).resolve()

    def snapshot() -> dict[str, bytes]:
        return {str(item.relative_to(root)): item.read_bytes() if item.is_file() else b"<dir>"
                for item in sorted(root.rglob("*"))}
    before = snapshot()

    calls: list[str] = []
    active = {"on": True}

    def spy(name):
        def call(*args, **kwargs):
            if active["on"]:
                calls.append(name)
            raise AssertionError(f"a capture run must not reach {name}")
        return call
    for module in (run_pilot, external, print_csv, harness):
        for name in ("read_caepipe_print_csv", "parse_caepipe_print_csv", "_print_layout_problem",
                     "stored_run_record", "_sha256", "bound_csv", "case_paths"):
            if hasattr(module, name):
                monkeypatch.setattr(module, name, spy(f"{module.__name__}.{name}"))
    for name in ("solve_swbpipe", "render_caepipe_mbf_si", "write_private", "_write_json", "check_guest_alive",
                 "check_no_foreign_instance", "resolve_harness_config"):
        monkeypatch.setattr(run_pilot, name, spy(f"run_pilot.{name}"))
    real_open, real_os_open = builtins.open, os.open

    def touched(target) -> None:
        try:
            resolved = Path(os.fspath(target)).resolve()
        except TypeError:
            return  # a file descriptor
        if active["on"] and (resolved == watched or watched in resolved.parents):
            calls.append(f"open {resolved.relative_to(root.resolve())}")

    def spy_open(file, *args, **kwargs):
        touched(file)
        return real_open(file, *args, **kwargs)

    def spy_os_open(file, *args, **kwargs):
        touched(file)
        return real_os_open(file, *args, **kwargs)
    monkeypatch.setattr(builtins, "open", spy_open)
    monkeypatch.setattr(io, "open", spy_open)
    monkeypatch.setattr(os, "open", spy_os_open)

    def round_trip(*args, **kwargs):
        calls.append("round_trip")
        raise AssertionError("a capture run must not be round-tripped")
    lines: list[str] = []
    probes = {
        "command live": lambda: run_pilot.main(["--benchmark", str(path), "--case", "cpb-p2-01", "--series", RUNS[2]]
                                               + state["argv"][2:], environ={}),
        "command offline": lambda: run_pilot.main(["--benchmark", str(path), "--case", "cpb-p2-01", "--runner",
                                                   state["runner"], "--offline", "--series", RUNS[2]], environ={}),
        "run_benchmark live": lambda: run_pilot.run_benchmark(
            [capture], config, runner=state["runner"], offline=False, emit=lines.append, round_trip=round_trip),
        "run_benchmark offline": lambda: run_pilot.run_benchmark([capture], config, runner=state["runner"],
                                                                 offline=True, emit=lines.append),
        "mixed batch": lambda: run_pilot.run_benchmark([plain, capture], config, runner=state["runner"],
                                                       offline=True, emit=lines.append),
        "run_benchmark_model live": lambda: run_pilot.run_benchmark_model(
            capture, config, runner=state["runner"], offline=False, emit=lines.append, round_trip=round_trip)[1],
        "run_benchmark_model offline": lambda: run_pilot.run_benchmark_model(
            capture, config, runner=state["runner"], offline=True, emit=lines.append)[1],
        "internal live": lambda: run_pilot._run_benchmark_model(
            capture, config, runner=state["runner"], offline=False, emit=lines.append, round_trip=round_trip,
            gui_fallback=False, attempt=1)[1],
        "internal offline": lambda: run_pilot._run_benchmark_model(
            capture, config, runner=state["runner"], offline=True, emit=lines.append, round_trip=round_trip,
            gui_fallback=False, attempt=1)[1],
    }
    expected = {name: 2 for name in probes if name.startswith(("command", "run_benchmark ", "mixed"))}
    for name, probe in probes.items():
        lines.clear()
        assert probe() == expected.get(name, "capture_path_unavailable"), name
        assert calls == [], (name, calls)
        if name.startswith("command"):
            assert capsys.readouterr().out.strip() == CAPTURE_REFUSAL, name
        else:
            # Only the refusal's own status line; the mixed batch runs nothing, its other model included.
            assert lines == [CAPTURE_REFUSAL], (name, lines)
    active["on"] = False
    assert state["calls"] == [] and state["sleeps"] == [] and snapshot() == before


def test_the_pace_is_the_callers(fake_benchmark):
    state = fake_benchmark
    assert run_pilot.main(state["argv"] + ["--pace-s", "90", "--batch-size", "2", "--batch", "1"], environ={}) == 0
    assert state["sleeps"] == [90.0]


def test_the_gui_fallback_is_on_when_the_guard_restores_the_whole_key(fake_benchmark, monkeypatch):
    state = fake_benchmark
    whole = dataclasses.replace(state["config"], settings_guard=dataclasses.replace(INVENTED_GUARD, restore_scope="key"))
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: whole)
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert state["calls"] == [("cpb-g1-01", {"gui_fallback": True})]


@pytest.mark.parametrize("outcome", [
    {"settings_guard": {"status": "settings_changed"}},
    {"route": "batch_no_csv", "settings_guard": {"status": "unchanged"}},
    {"route": "timeout", "settings_guard": {"status": "owned_process_running"}},
    RuntimeError("invented"),  # raised before any settings check was recorded
])
def test_the_benchmark_run_stops_at_the_first_unclean_round_trip(fake_benchmark, outcome):
    state = fake_benchmark
    state["outcomes"][:] = [outcome]
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"]


def raised_after_a_clean_check() -> RuntimeError:
    error = RuntimeError("invented transport failure")
    error.settings_guard = {"status": "unchanged"}
    return error


def test_a_failed_model_is_retried_once_after_two_minutes(fake_benchmark, capsys):
    state = fake_benchmark
    state["outcomes"][:] = [raised_after_a_clean_check()]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 0
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-01", "cpb-g1-02"]
    assert state["sleeps"] == [120.0, 60.0]
    assert "cpb-g1-01: route=round_trip_raised; the round trip raised RuntimeError; retrying once in 120 s" in \
        capsys.readouterr().out
    manifest = json.loads((state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["attempt"]) == ("recorded", 2)


def test_a_model_that_leaves_no_csv_twice_is_recorded_and_the_run_goes_on(fake_benchmark, capsys):
    state = fake_benchmark
    no_csv = {"route": "csv_missing", "observed_csv_path": {}, "settings_guard": {"status": "unchanged"}}
    state["outcomes"][:] = [no_csv, no_csv]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-01", "cpb-g1-02"]
    assert "cpb-g1-01: route=csv_missing; the run record names no collected CAEPIPE CSV; no benchmark record after " \
           "one retry" in capsys.readouterr().out
    root = state["config"].exchange_root
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["route"], manifest["attempt"]) == ("no_record", "csv_missing", 2)
    assert not (root / "cases" / "cpb-g1-01" / "comparison" / "benchmark_comparison.json").exists()


def test_a_round_trip_that_raises_twice_stops_the_run(fake_benchmark):
    state = fake_benchmark
    state["outcomes"][:] = [raised_after_a_clean_check(), raised_after_a_clean_check()]
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-01"]


def test_the_benchmark_run_starts_no_model_while_another_instance_runs(fake_benchmark):
    state = fake_benchmark
    state["pre"] = "foreign_instance_running"
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert state["calls"] == []


def test_an_unresponsive_guest_is_checked_again_after_two_minutes_then_stops_the_run(fake_benchmark, capsys):
    state = fake_benchmark
    down = {"status": "unreachable", "reason": "invented"}
    state["alive"][:] = [down]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert state["sleeps"] == [120.0] and [call[0] for call in state["calls"]] == ["cpb-g1-01"]
    state["alive"][:] = [down, down]
    state["calls"].clear()
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 1
    assert state["calls"] == []
    assert "stopped before cpb-g1-01: liveness unreachable; invented" in capsys.readouterr().out


def test_the_freeze_file_stops_the_run_before_the_next_model(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    # The freeze file appears during the pause after the first model.
    monkeypatch.setattr(run_pilot.time, "sleep", lambda seconds: state["freeze"].write_text("", encoding="ascii"))
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"]
    assert "stopped before cpb-g1-02: the freeze file is present" in capsys.readouterr().out


def test_a_pending_exclusive_job_holds_the_run(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    state["exclusive"].write_text(f"{os.getpid()}\n", encoding="ascii")  # a running process: this test's
    polls = []

    def poll(seconds):
        polls.append(seconds)
        if len(polls) == 2:
            state["exclusive"].write_text("not a process id\n", encoding="ascii")
    monkeypatch.setattr(run_pilot.time, "sleep", poll)
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert polls == [30.0, 30.0] and [call[0] for call in state["calls"]] == ["cpb-g1-01"]
    assert "cpb-g1-01: an exclusive job is pending; waiting for it (up to 1800 s)" in capsys.readouterr().out
    state["calls"].clear()
    state["exclusive"].write_text(f"{os.getpid()}\n", encoding="ascii")
    monkeypatch.setattr(run_pilot.time, "sleep", lambda seconds: None)
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 1
    assert state["calls"] == []
    assert "stopped before cpb-g1-01: an exclusive job was still pending after 1800 s" in capsys.readouterr().out


@pytest.mark.parametrize("answer, status", [
    ((0, "Invented Guest [Version 0.0]\r\n", ""), "alive"),
    ((1, "", "invented"), "unreachable"),
    ((0, "  \r\n", ""), "unreachable"),
    (subprocess.TimeoutExpired("prlctl", 1.0), "unreachable"),
])
def test_the_liveness_check_asks_the_guest_for_ver_and_starts_nothing(tmp_path, answer, status):
    calls = []

    def runner(argv, **kwargs):
        calls.append(argv)
        if isinstance(answer, BaseException):
            raise answer
        return subprocess.CompletedProcess(argv, answer[0], stdout=answer[1], stderr=answer[2])
    result = check_guest_alive(fake_config(tmp_path), runner=runner)
    assert result["status"] == status
    assert calls == [["/invented/prlctl", "exec", "invented-vm", "--current-user", "cmd", "/c", "ver"]]
    assert "Invented Guest" not in json.dumps(result)


def test_benchmark_refusals(fake_benchmark, monkeypatch, capsys, tmp_path):
    state = fake_benchmark
    base = state["argv"]
    for argv, message in ((base + ["--authored"], "--benchmark takes only"),
                          (base + ["--export-mbf"], "--benchmark takes only"),
                          (["--batch", "1"], "need --benchmark"),
                          (["--series", "pressure-default"], "need --benchmark"),
                          (["--freeze-file", "x"], "need --benchmark"),
                          (base + ["--batch-size", "0"], "--batch-size must be at least 1"),
                          (base + ["--pace-s", "-1"], "--pace-s must be a non-negative number"),
                          (base[:4], "a live --benchmark run needs --freeze-file and --exclusive-file"),
                          (base[:6], "a live --benchmark run needs --freeze-file and --exclusive-file")):
        with pytest.raises(SystemExit) as raised:
            run_pilot.main(argv, environ={})
        assert raised.value.code == 2 and message in capsys.readouterr().err
    assert run_pilot.main(base + ["--batch", "3"], environ={}) == 2
    assert "--batch 3 is not one of the 2 batches" in capsys.readouterr().out
    assert run_pilot.main(base + ["--case", "cpb-g9-01"], environ={}) == 2
    assert "unknown benchmark model id(s): cpb-g9-01" in capsys.readouterr().err
    # A live run needs the settings guard.
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: dataclasses.replace(
        state["config"], settings_guard=None))
    assert run_pilot.main(base, environ={}) == 2
    assert "--benchmark runs need settings_guard" in capsys.readouterr().err
    # A model file that differs from the manifest's hash is refused before anything runs.
    model = state["manifest"].parent / "models" / "cpb-g1-02.json"
    model.write_bytes(model.read_bytes().replace(b'"B311-80"', b'"B313"'))
    assert run_pilot.main(base, environ={}) == 2
    assert "models/cpb-g1-02.json differs from the manifest's file_sha256" in capsys.readouterr().err
    assert state["calls"] == []
