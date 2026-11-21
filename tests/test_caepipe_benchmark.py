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
    BENCHMARK_RECORD_VERSION,
    FINDING,
    OPERATING_LABEL,
    PRINTED_RESOLUTION,
    SCREENING_PROFILE,
    STATUS_MISMATCH,
    UNMATCHED,
    WITHIN,
    build_benchmark_record,
    screen,
    screening_class,
)
from core.handoff.caepipe_external.harness import RUN_RECORD_KIND, StartUnresolved, check_guest_alive  # noqa: E402
from core.handoff.caepipe_external.print_csv import bind_element_rows  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import benchmark_models, run_pilot  # noqa: E402
from tests.test_caepipe_correlation import INVENTED_GUARD, fake_config  # noqa: E402


BENCHMARK = ROOT / "validation" / "caepipe_benchmark"
MANIFEST = BENCHMARK / "manifest.json"


@pytest.fixture(autouse=True)
def _live_session_passed_through(monkeypatch):
    """Every live route opens its session first (liveness, then the share's letter); that opening is examined in
    tests/test_caepipe_live_session.py, and here it passes the configuration through unchanged."""
    monkeypatch.setattr(run_pilot, "live_session", lambda config, emit=print: config)


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
    assert completed.returncode == 0, (completed.returncode, completed.stderr)
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


def assert_within_or_unwitnessed(record: dict, label: str = "") -> None:
    """Policy boundary (the section rule's no-witness branch): every quantity is screened within the profile, or is
    unmatched only because its section of the print has no nonzero token of its unit, so it prints zero; there is no
    finding and no other unmatched or status row."""
    for row in record["results"]:
        if row["classification"] == WITHIN:
            continue
        assert row["classification"] == UNMATCHED and row.get("resolution_cause") == "no_witness", (label, row)
        assert row["values"]["caepipe"] == 0.0, (label, row)


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
        assert_within_or_unwitnessed(record, document["model_id"])
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
    assert manifest["series"][RUNS[2]]["swbpipe"] is None and "no SWBPIPE counterpart" in capture_text
    assert "collected, bound and parsed" in capture_text
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


def invented_print(*, sustained_uy="-4.000", drop_label=None, extra_anchor=False, override=None) -> str:
    """Invented print of the invented cantilever (nodes 10, 20, 30 for A, M, B). `override`: {label: {section:
    {node: printed values}}} replaces those rows."""
    return csv_text(invented_cases(sustained_uy=sustained_uy, drop_label=drop_label, extra_anchor=extra_anchor,
                                   override=override))


def invented_cases(*, sustained_uy="-4.000", drop_label=None, extra_anchor=False, override=None) -> dict:
    """The tables of invented_print, before they are written as text."""
    def tables(uy: str, ux: str, fy: str, mz: str) -> dict:
        element = {"10": ["0", fy, "0", "0", "0", "0"], "20": ["0", f"{-float(fy):g}", "0", "0", "0", "0"]}
        return {
            "anchors": {"10": ["0", f"{-float(fy):g}", "0", "0", "0", mz], **({"90": ["1"] * 6} if extra_anchor else {})},
            "restraints": {"20": ["0", "0", "0"]},
            "elements": [("10", element["10"]), ("20", element["20"]), ("20", element["10"]), ("30", element["20"])],
            "displacements": {"10": ["0"] * 6, "20": ["0"] * 6, "30": [ux, uy, "0", "0", "0", "0"]},
        }
    cases = {"Sustained (W+P)": tables(sustained_uy, "0", "1000", "-4000"),
             "Expansion (T1)": tables("0", "4.800", "0", "0"),
             OPERATING_LABEL: tables(sustained_uy, "4.800", "1000", "-4000")}
    for label, sections in (override or {}).items():
        for section, rows in sections.items():
            cases[label][section].update(rows)
    if drop_label:
        cases.pop(drop_label)
    return cases


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
    assert_within_or_unwitnessed(record)
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
    record = compare(invented_print(sustained_uy="-4.030"))
    row = find(record, "case:sustained", "displacement", "node:B", "y")
    assert row["classification"] == FINDING
    assert row["screen"]["allowed"] == pytest.approx(1e-6 + 5e-3 * 0.00403)
    assert row["screen"]["difference"] == pytest.approx(-0.00003)


def test_the_exact_class_and_the_floor_and_the_stated_formula():
    assert screen(1.0, 1.0 + 1.5e-4, 1.0e-6, "exact")["within"] is False
    assert screen(1.0, 1.0 + 0.5e-4, 1.0e-6, "exact")["within"] is True
    # Below the printed unit, a scalar agrees whatever its relative difference.
    assert screen(0.0, 0.9, 1.0, "exact")["within"] is True and screen(0.0, 1.1, 1.0, "exact")["within"] is False
    # Near zero there is no other scale: |S - C| <= a + r * max(|S|, |C|) and nothing else (the profile states no
    # near-zero term; this expected value was the near-zero relaxation's True before the profile was applied as stated).
    check = screen(0.0, 5.0, 1.0, "named_inexact")
    assert check["within"] is False and check["allowed"] == 1.0 + 5e-3 * 5.0
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
    missing = compare(invented_print().replace("30,4.800,", "31,4.800,").replace("30,0,-4.000,", "31,0,-4.000,"))
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
    assert screen(math.inf, math.inf, 1.0e-6, "exact")["within"] is False
    assert screen(0.0, 1.0e-7, math.nan, "exact")["within"] is False
    assert screen(0.0, 0.0, 0.0, "exact")["within"] is False and screen(0.0, 0.0, -1.0, "exact")["within"] is False


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
    text = invented_print().replace("30,0,-4.000,0,0,0,0,,", "30,0,-4.000,0,0,0,0,,\r\n30,0,-5.000,0,0,0,0,,", 1)
    parsed = parse_caepipe_print_csv(text)
    assert [item for item in parsed["diagnostics"] if item["severity"] == "error"]
    record = compare_payload(invented_payload(), text)
    errors = [row for row in record["results"] if row["kind"] == "parser_error"]
    assert errors and all(row["classification"] == UNMATCHED for row in errors)
    assert record["summary"]["by_classification"].get(UNMATCHED, 0) >= len(errors)


# -- the stated formula and the printed-resolution floor -------------------------------------------------------------
def record_of(text: str, payload: dict | None = None, document: dict | None = None) -> dict:
    document = document or invented_cantilever()
    mbf = render_caepipe_mbf_si(document["model"], title="INVENTED", piping_code=document["piping_code"])
    return build_benchmark_record(document, swbpipe_result_payload=payload or invented_payload(), mbf_result=mbf,
                                  mbf_text=mbf.text, parsed_caepipe=parse_caepipe_print_csv(text))


def set_value(kind: str, entity: str, case: str, value: float, component: str | None = None):
    def edit(row: dict) -> dict:
        hit = (row["kind"], row["entity_ref"], row["basis_ref"]["ref_id"]) == (kind, entity, case)
        return {**row, "value": value} if hit and component in (None, row["metadata"]["component"]) else row
    return edit


def payload_with(*edits) -> dict:
    payload = invented_payload()
    for edit in edits:
        payload["mechanics_envelope"]["results"] = [edit(row) for row in payload["mechanics_envelope"]["results"]]
    return payload


def without_closed_forms() -> dict:
    document = invented_cantilever()
    document["closed_form"] = []
    return document


def compared_rows(record: dict) -> list[dict]:
    """The rows that have a value on both sides."""
    return [row for row in record["results"] if isinstance(row.get("values"), dict)
            and row["values"]["swbpipe"] is not None and row["values"]["caepipe"] is not None]


def rows_of(record: dict, case: str, kind: str, entities: set[str]) -> list[dict]:
    return [row for row in record["results"] if row["case"] == case and row["kind"] == kind
            and row["entity"] in entities]


# Node B's x displacement made 1000 m on both sides of the expansion and operating cases (SWBPIPE's value in mm).
LARGE_TIP = set_value("global_nodal_displacement_x", "node:B", "case:expansion", 1.0e6)
LARGE_TIP_PRINT = {"Expansion (T1)": {"displacements": {"30": ["1000000", "0", "0", "0", "0", "0"]}},
                   OPERATING_LABEL: {"displacements": {"30": ["1000000", "-4.000", "0", "0", "0", "0"]}}}
CAUSES = ["no_witness", "shorter_than_format", "unsupported_by_this_screen", "unsupported_units_or_angle",
          "malformed_output", "resolution_absent"]
DISPLACEMENT_NODES = {"node:A", "node:M", "node:B"}


def with_print(base: dict, **sections) -> dict:
    """`base` with the expansion case's rows in `sections` replaced as well."""
    merged = {label: {section: dict(rows) for section, rows in tables.items()} for label, tables in base.items()}
    for section, rows in sections.items():
        merged.setdefault("Expansion (T1)", {}).setdefault(section, {}).update(rows)
    return merged


def test_the_readers_example_is_a_finding_whatever_the_case_maximum():
    """Defect: the near-zero term. S = 0 and C = 5e-5 m in a case whose largest displacement is 1000 m: the stated
    formula allows a + r * 5e-5 m, about 1e-6 m, so the difference is a finding. The near-zero term (r * 1e-3 *
    1000 m) passed it."""
    text = invented_print(override=with_print(LARGE_TIP_PRINT, displacements={"20": ["0", "0.050", "0", "0", "0", "0"]}))
    record = record_of(text, payload_with(LARGE_TIP), without_closed_forms())
    row = find(record, "case:expansion", "displacement", "node:M", "y")
    assert row["values"] == {"swbpipe": 0.0, "caepipe": pytest.approx(5.0e-5, rel=1e-15)}
    assert row["classification"] == FINDING
    assert row["screen"]["allowed"] == pytest.approx(1.0e-6 + 1.0e-4 * 5.0e-5, rel=1e-12)
    # The record's screen is the closed form of the profile, computed by the one function.
    direct = screen(0.0, row["values"]["caepipe"], row["screen"]["floor"], "exact")
    assert row["screen"] == {name: direct[name] for name in ("difference", "allowed", "relative", "floor")}
    assert find(record, "case:expansion", "displacement", "node:B", "x")["classification"] == WITHIN


def test_an_unrelated_large_pair_cannot_change_an_existing_pairs_verdict():
    """Defect: the case maximum. Adding a large, agreeing pair elsewhere in the case leaves a pair's screen as it was."""
    restraint = {"Sustained (W+P)": {"restraints": {"20": ["0", "-3", "0"]}}}  # C = 3 N where SWBPIPE has 0
    base = record_of(invented_print(override=restraint))
    anchor = ["-10000000", "-1000", "0", "0", "0", "-4000"]  # a 1e7 N anchor force, equal on both sides
    large = {"Sustained (W+P)": {**restraint["Sustained (W+P)"], "anchors": {"10": anchor}},
             OPERATING_LABEL: {"anchors": {"10": anchor}}}
    payload = payload_with(set_value("support_reaction_component_v2", "support:A", "case:sustained", 1.0e7, "Fx"))
    record = record_of(invented_print(override=large), payload)
    key = ("case:sustained", "support_force", "node:20", "y")
    assert find(base, *key)["classification"] == FINDING
    assert find(record, *key)["classification"] == FINDING and find(record, *key)["screen"] == find(base, *key)["screen"]
    assert find(record, "case:sustained", "support_force", "node:10", "x")["classification"] == WITHIN


# (section, node label, column, kind, entity, component, decimals printed, floor in SI) for each SI label that has a
# candidate precision: 0.001 mm = 1e-6 m, 0.0001 deg in rad, 1 N, 1 N*m.
SI_FLOORS = [
    ("displacements", "20", 1, "displacement", "node:M", "y", 3, 1.0e-6),
    ("displacements", "20", 5, "rotation", "node:M", "z", 4, math.radians(1.0e-4)),
    ("restraints", "20", 1, "support_force", "node:20", "y", 0, 1.0),
    ("anchors", "10", 3, "support_moment", "node:10", "x", 0, 1.0),
]


@pytest.mark.parametrize("section, node, column, kind, entity, component, places, floor", SI_FLOORS)
def test_the_floor_is_one_printed_unit_of_the_si_print(section, node, column, kind, entity, component, places, floor):
    """Policy boundary: a = one printed unit, for each SI label and its conversion to SI. The printed value is itself
    the section's witness: one unit is within, two are a finding."""
    for units, expected in ((1, WITHIN), (2, FINDING)):  # where SWBPIPE has 0: one printed unit, then two
        row = ["0"] * (3 if section == "restraints" else 6)
        row[column] = f"{units * 10.0 ** -places:.{places}f}"
        record = record_of(invented_print(override={"Expansion (T1)": {section: {node: row}}}))
        found = find(record, "case:expansion", kind, entity, component)
        assert found["screen"]["floor"] == pytest.approx(floor, rel=1e-15), (kind, units)
        assert abs(found["values"]["caepipe"]) == pytest.approx(units * floor, rel=1e-15)
        assert found["classification"] == expected, (kind, units)


@pytest.mark.parametrize("printed, swbpipe_mm", [("0.05", 0.052), ("0", 0.002), ("-0", 0.002), ("", 0.002)])
def test_a_witnessed_section_gives_its_short_zero_and_blank_tokens_its_resolution(printed, swbpipe_mm):
    """Policy boundary (the section rule's established branch): node B's 4.800 witnesses three places, so 0.05, 0, -0
    and a blank in the same section take 0.001 mm; 2 um is then a finding, which a floor read from the token alone
    (0.01 or 1 mm) would pass."""
    payload = payload_with(set_value("global_nodal_displacement_y", "node:M", "case:expansion", swbpipe_mm))
    text = invented_print(override={"Expansion (T1)": {"displacements": {"20": ["0", printed, "0", "0", "0", "0"]}}})
    row = find(record_of(text, payload), "case:expansion", "displacement", "node:M", "y")
    assert row["screen"]["floor"] == pytest.approx(1.0e-6, rel=1e-15)
    assert row["classification"] == FINDING


def test_an_unwitnessed_or_shorter_section_leaves_its_unit_unmatched():
    """Policy boundary (no witness; all shorter): a section with no nonzero token of a unit, or whose nonzero tokens
    all show fewer places than the candidate, screens none of that unit's values."""
    shorter = record_of(invented_print(override={"Expansion (T1)": {"displacements": {"30": ["4.8", "0", "0", "0", "0",
                                                                                              "0"]}}}))
    rows = rows_of(shorter, "case:expansion", "displacement", DISPLACEMENT_NODES)
    assert len(rows) == 9 and all(row["resolution_cause"] == "shorter_than_format" for row in rows)
    assert "print shows at most 1 places, format expects 3" in rows[0]["reason"]
    # The expansion case prints no nonzero rotation: every rotation is unmatched for want of a witness.
    rotations = rows_of(shorter, "case:expansion", "rotation", DISPLACEMENT_NODES)
    assert len(rotations) == 9 and all(row["resolution_cause"] == "no_witness" for row in rotations)


@pytest.mark.parametrize("section, node, column, kind, entities, printed, cause, words", [
    ("displacements", "20", 1, "displacement", DISPLACEMENT_NODES, "0.0010", "unsupported_by_this_screen",
     "print shows 4 places, format expects 3"),
    ("displacements", "20", 1, "displacement", DISPLACEMENT_NODES, "1E+00", "unsupported_by_this_screen",
     "exponent form"),
    ("displacements", "20", 1, "displacement", DISPLACEMENT_NODES, "1.0E-03", "unsupported_by_this_screen",
     "exponent form"),
    ("restraints", "20", 1, "support_force", {"node:20"}, "-0.5", "unsupported_by_this_screen",
     "print shows 1 places, format expects 0"),
    ("restraints", "20", 1, "support_force", {"node:20"}, "-1E+02", "unsupported_by_this_screen", "exponent form"),
])
def test_more_places_or_exponent_form_leaves_the_whole_unit_of_the_section_unmatched(section, node, column, kind,
                                                                                     entities, printed, cause, words):
    """Policy boundary (more places or exponent form): one such token leaves every value of that unit in the section
    unmatched, cause unsupported_by_this_screen, even beside a witness (node B's 4.800); finite values are kept."""
    row = ["0"] * (3 if section == "restraints" else 6)
    row[column] = printed
    record = record_of(invented_print(override={"Expansion (T1)": {section: {node: row}}}))
    rows = rows_of(record, "case:expansion", kind, entities)
    assert len(rows) == 3 * len(entities)
    for found in rows:
        assert found["classification"] == UNMATCHED and found["unmatched_cause"] == PRINTED_RESOLUTION
        assert found["resolution_cause"] == cause and words in found["reason"] and "screen" not in found
        assert found["values"]["caepipe"] is not None  # the value is kept, not screened
    counts = record["summary"]["unmatched_for_resolution"]["by_case"]["case:expansion"]["by_cause"]
    assert counts[cause] == len(rows)
    assert FINDING not in record["summary"]["by_classification"]


def test_a_malformed_token_is_its_own_cause_and_does_not_unseat_the_witness():
    """Policy boundary (malformed output): a number in none of the format's decimal forms is unmatched as malformed
    output, apart from unsupported forms; the section's other values keep the witnessed resolution."""
    record = record_of(invented_print(override={"Expansion (T1)": {"displacements": {"20": ["0", "1_0", "0", "0", "0",
                                                                                             "0"]}}}))
    assert find(record, "case:expansion", "displacement", "node:M", "y")["resolution_cause"] == "malformed_output"
    assert find(record, "case:expansion", "displacement", "node:B", "x")["classification"] == WITHIN


def test_a_row_that_does_not_bind_supplies_no_resolution():
    """Policy boundary (evidence stays local): an element row pair that binds to no member and a displacement row at
    a label no node maps to carry witnesses, but bound values of the section stay unwitnessed."""
    cases = invented_cases()
    cases["Expansion (T1)"]["elements"] += [("10", ["0", "5", "0", "0", "0", "7"]), ("90", ["0", "-5", "0", "0", "0", "7"])]
    cases["Expansion (T1)"]["displacements"]["99"] = ["0", "0", "0", "0", "0", "0.1250"]
    record = record_of(csv_text(cases))
    assert {row["section"] for row in record["results"] if row["kind"] == "unbound_row" and row["case"] ==
            "case:expansion"} == {"element_forces_global", "displacements"}
    for kind in ("element_force", "element_moment"):
        rows = [row for row in record["results"] if row["case"] == "case:expansion" and row["kind"] == kind]
        assert rows and all(row["resolution_cause"] == "no_witness" for row in rows), kind
    rotations = rows_of(record, "case:expansion", "rotation", DISPLACEMENT_NODES)
    assert all(row["resolution_cause"] == "no_witness" for row in rotations)


def test_no_resolution_is_borrowed_across_section_case_or_quantity():
    """Policy boundary (evidence stays local): the sustained anchor section's force witness does not reach its
    restraint section; the expansion case's mm witness does not reach its rotations; the sustained case's witnesses do
    not reach the expansion case's unwitnessed sections."""
    record = record_of(invented_print())
    assert find(record, "case:sustained", "support_force", "node:10", "y")["classification"] == WITHIN
    assert find(record, "case:sustained", "support_force", "node:20", "y")["resolution_cause"] == "no_witness"
    assert find(record, "case:expansion", "displacement", "node:B", "x")["classification"] == WITHIN
    assert find(record, "case:expansion", "rotation", "node:B", "z")["resolution_cause"] == "no_witness"
    assert find(record, "case:sustained", "element_force", "pipe:P1", "y", "end_i")["classification"] == WITHIN
    assert find(record, "case:expansion", "element_force", "pipe:P1", "y", "end_i")["resolution_cause"] == "no_witness"


# The SI labels of the invented print replaced by the English ones, and the degree labels by radians.
ENGLISH = (("(mm)", "(inch)"), ("(N)", "(lb)"), ("(Nm)", "(ft-lb)"))
RADIANS = (("(deg)", "(rad)"),)


def relabel(text: str, pairs) -> str:
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def test_english_units_and_radian_rotations_are_unmatched_never_screened():
    """Policy boundary (unsupported units or angle): no candidate exists for the English labels or rad. Every value
    of an English print, degrees included, and every rotation printed in radians is unmatched with that cause, its
    normalized value kept; the other SI quantities of a print with radian rotations stay screenable."""
    english = record_of(relabel(invented_print(), ENGLISH))
    rows = compared_rows(english)
    assert rows and all(row["resolution_cause"] == "unsupported_units_or_angle" for row in rows)
    assert english["summary"]["unmatched_for_resolution"]["by_cause"]["unsupported_units_or_angle"] == len(rows)
    tip = find(english, "case:expansion", "displacement", "node:B", "x")
    assert tip["values"]["caepipe"] == pytest.approx(4.8 * 0.0254, rel=1e-15)  # 4.800 in, by the exact definition
    # The stated cause names the label and the system.
    assert "'inch'" in tip["reason"] and "English" in tip["reason"]
    assert "'deg'" in find(english, "case:expansion", "rotation", "node:M", "z")["reason"]
    assert "'lb'" in find(english, "case:sustained", "support_force", "node:10", "y")["reason"]
    assert "'ft-lb'" in find(english, "case:sustained", "support_moment", "node:10", "z")["reason"]
    radians = record_of(relabel(invented_print(), RADIANS))
    rotations = [row for row in compared_rows(radians) if row["kind"] == "rotation"]
    assert rotations and all(row["resolution_cause"] == "unsupported_units_or_angle" and "'rad'" in row["reason"]
                             for row in rotations)
    others = [row for row in compared_rows(radians) if row["kind"] != "rotation"]
    assert find(radians, "case:sustained", "displacement", "node:B", "y")["classification"] == WITHIN
    assert all(row["classification"] == WITHIN or row.get("resolution_cause") == "no_witness" for row in others)


@pytest.mark.parametrize("units, expected", [(1, WITHIN), (2, FINDING)])
def test_each_unit_conversion_carries_one_printed_unit_into_si(monkeypatch, units, expected):
    """Policy boundary: the conversion path for a label once a candidate exists for it (here hypothetical entries; the
    shipped table has candidates for the SI labels only). The floor is one printed unit by the unit catalog's exact
    definitions: 1 in = 0.0254 m, 1 lbf = 4.4482216152605 N, 1 ft-lb = 1.3558179483314004 N*m, 1 deg = pi/180 rad."""
    from core.handoff.caepipe_external import print_csv

    for key, places in ((("English", "inch"), 4), (("English", "lb"), 0), (("English", "ft-lb"), 0),
                        (("English", "deg"), 4), (("SI", "rad"), 6)):
        monkeypatch.setitem(print_csv.PRINTED_DECIMALS, key, places)
    rows = {"displacements": {"20": ["0", f"{units * 1e-4:.4f}", "0", "0", "0", f"{units * 1e-4:.4f}"]},
            "restraints": {"20": ["0", f"{-units}", "0"]}, "anchors": {"10": ["0", "0", "0", f"{-units}", "0", "0"]}}
    english = record_of(relabel(invented_print(override={"Expansion (T1)": rows}), ENGLISH))
    for kind, entity, component, floor in (("displacement", "node:M", "y", 1e-4 * 0.0254),
                                           ("rotation", "node:M", "z", math.radians(1e-4)),
                                           ("support_force", "node:20", "y", 4.4482216152605),
                                           ("support_moment", "node:10", "x", 4.4482216152605 * 0.3048)):
        row = find(english, "case:expansion", kind, entity, component)
        assert row["screen"]["floor"] == pytest.approx(floor, rel=1e-15), kind
        assert row["classification"] == expected, (kind, row)
    rad = {"displacements": {"20": ["0", "0", "0", "0", "0", f"{units * 1e-6:.6f}"]}}
    row = find(record_of(relabel(invented_print(override={"Expansion (T1)": rad}), RADIANS)),
               "case:expansion", "rotation", "node:M", "z")
    assert row["screen"]["floor"] == pytest.approx(1e-6, rel=1e-15)
    assert row["classification"] == expected


def test_the_closed_form_against_the_program_takes_the_same_screen():
    """Defect: the near-zero term in the closed-form check against the program. It uses the same screen and floor."""
    document = without_closed_forms()
    document["closed_form"] = [{"quantity": "displacement", "component": "y", "load_case": "case:expansion",
                                "node": "node:M", "value": 0.0, "unit": "m", "formula": "invented", "basis": "invented"}]
    for printed, verdict in (("0.001", "agree"), ("0.050", "disagree")):
        text = invented_print(override=with_print(LARGE_TIP_PRINT,
                                                  displacements={"20": ["0", printed, "0", "0", "0", "0"]}))
        row = find(record_of(text, payload_with(LARGE_TIP), document), "case:expansion", "displacement", "node:M", "y")
        assert row["closed_form"]["closed_form_vs_caepipe"] == verdict, printed
        within = screen(0.0, row["values"]["caepipe"], row["screen"]["floor"], "exact")["within"]
        assert verdict == ("agree" if within else "disagree")
    english = record_of(relabel(invented_print(), ENGLISH), document=document)
    closed = find(english, "case:expansion", "displacement", "node:M", "y")["closed_form"]
    assert closed["closed_form_vs_caepipe"] == UNMATCHED
    assert closed["closed_form_vs_caepipe_cause"] == "unsupported_units_or_angle"


def test_a_parse_without_printed_tokens_is_unmatched_never_screened():
    """Policy boundary (resolution absent): a parse that carries no printed tokens screens nothing."""
    document = invented_cantilever()
    mbf = render_caepipe_mbf_si(document["model"], title="INVENTED", piping_code=document["piping_code"])
    parsed = parse_caepipe_print_csv(invented_print())
    del parsed["printed_resolution"]
    record = build_benchmark_record(document, swbpipe_result_payload=invented_payload(), mbf_result=mbf,
                                    mbf_text=mbf.text, parsed_caepipe=parsed)
    rows = compared_rows(record)
    assert rows and all(row.get("resolution_cause") == "resolution_absent" for row in rows)
    assert record["summary"]["quantities"]["screened"] == 0


def test_the_counts_name_every_cause_and_their_denominator():
    """Policy boundary (counts with their denominator): per case and over the record, expected = screened + unmatched
    (+ not compared), and the unmatched for resolution are counted per cause, all six listed."""
    record = record_of(invented_print())
    resolution = record["summary"]["unmatched_for_resolution"]
    assert list(resolution["by_cause"]) == CAUSES
    assert set(resolution["by_case"]) == {"case:expansion", "case:sustained", "operating"}
    rows = [row for row in record["results"] if row.get("unmatched_cause") == PRINTED_RESOLUTION]
    assert resolution["total"] == len(rows) == sum(resolution["by_cause"].values())
    quantities = record["summary"]["quantities"]
    for case, tally in quantities["by_case"].items():
        case_rows = [row for row in record["results"] if row["case"] == case and "values" in row]
        assert tally["expected"] == len(case_rows) == tally["screened"] + tally["unmatched"] + tally["not_compared"]
        assert tally["screened"] == sum("screen" in row for row in case_rows) > 0
        assert tally["outcome"] == "every_screened_quantity_within", case
    assert quantities["expected"] == sum(tally["expected"] for tally in quantities["by_case"].values())


def test_an_empty_screened_set_is_never_read_as_within_the_profile():
    """Policy boundary: a case, or a record, with no screened quantity says so, and never concludes that everything is
    within the profile; nor does a case that is not compared."""
    english = record_of(relabel(invented_print(), ENGLISH))
    quantities = english["summary"]["quantities"]
    for tally in [*quantities["by_case"].values(), quantities]:
        assert tally["screened"] == 0 and tally["outcome"] == "no_quantity_screened" and tally["note"]
    assert FINDING not in english["summary"]["by_classification"]  # no finding, yet nothing is within
    dropped = record_of(invented_print(drop_label="Expansion (T1)"))
    tally = dropped["summary"]["quantities"]["by_case"]["case:expansion"]
    assert tally["screened"] == 0 and tally["not_compared"] == tally["expected"] > 0
    assert tally["outcome"] == "no_quantity_screened"


def test_the_integer_digit_count_witness_counts_and_changes_no_verdict():
    """Policy boundary: for each shorter nonzero token a witnessed section covers, whether a full-places token's
    integer part has at least as many digits. It counts only: the shorter token is screened either way."""
    def expansion(record: dict) -> dict:
        return record["summary"]["integer_digit_count_witness"]["by_case"]["case:expansion"]

    # Node B prints 4.8 (shorter, one integer digit); node M's witness 0.001 has none, 1.001 has one.
    for witness, counts in (("0.001", (0, 1)), ("1.001", (1, 0))):
        record = record_of(invented_print(override={"Expansion (T1)": {"displacements": {
            "20": ["0", witness, "0", "0", "0", "0"], "30": ["4.8", "0", "0", "0", "0", "0"]}}}))
        assert (expansion(record)["witnessed_by_integer_digit_count"],
                expansion(record)["not_witnessed_by_integer_digit_count"]) == counts, witness
        assert find(record, "case:expansion", "displacement", "node:B", "x")["classification"] == WITHIN
    totals = record["summary"]["integer_digit_count_witness"]
    assert totals["witnessed_by_integer_digit_count"] == sum(
        case["witnessed_by_integer_digit_count"] for case in totals["by_case"].values())


def test_the_profile_states_the_formula_and_the_record_version_moves():
    """Policy boundary: the profile's metadata, its rule as computed, and the record version."""
    assert set(SCREENING_PROFILE) == {"basis", "relative", "floor", "floor_basis", "printed_decimals",
                                      "resolution_judgment", "resolution_causes", "limit", "rule"}
    assert SCREENING_PROFILE["relative"] == {"exact": 1e-4, "named_inexact": 5e-3}
    # Candidates exist for the SI labels only: none for an English label or for rad.
    assert SCREENING_PROFILE["printed_decimals"] == {"SI N": 0, "SI Nm": 0, "SI deg": 4, "SI mm": 3}
    assert SCREENING_PROFILE["resolution_causes"] == CAUSES
    assert set(SCREENING_PROFILE["resolution_judgment"]) == {
        "format_family", "witness_establishes", "witness_does_not_establish", "if_the_print_is_coarser",
        "if_the_print_is_finer", "evidence"}
    # The rule as computed: allowed = a + r * max(|S|, |C|), with no other scale.
    for swb, cae, floor, cls in ((0.0, 5e-5, 1e-6, "exact"), (-3.0, 2.0, 1.0, "named_inexact"),
                                 (1e6, 1e6 + 7.0, 1.0, "exact"), (0.0, 0.0, 1e-6, "named_inexact")):
        check = screen(swb, cae, floor, cls)
        rel = SCREENING_PROFILE["relative"][cls]
        assert check["allowed"] == floor + rel * max(abs(swb), abs(cae))
        assert check["within"] == (abs(cae - swb) <= floor + rel * max(abs(swb), abs(cae)))
    record = compare(invented_print())
    assert BENCHMARK_RECORD_VERSION == "3" and record["record_version"] == "3"
    assert record["screening_profile"] == SCREENING_PROFILE


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


def command_sleep(replacement):
    """A replacement for time.sleep that takes only the benchmark command's own waits. run_pilot.time is the
    process-wide time module, so a library's own short wait (subprocess polling a git child it started with a timeout)
    is passed through to the real sleep instead of being mistaken for the command's pacing."""
    real_sleep = run_pilot.time.sleep

    def sleep(seconds):
        if sys._getframe(1).f_globals.get("__name__") != run_pilot.__name__:
            return real_sleep(seconds)
        return replacement(seconds)
    return sleep


# The start state the harness records with each start route (`harness._round_trip_body`).
FAKE_ROUTE_STARTS = {"start_partial": "partial", "not_started": "not_started", "start_unresolved": "unresolved",
                     "report_reply_contradiction": "contradicted"}


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
        if callable(outcome):  # a real round trip (against a fake guest), run as the pilot's own call
            return outcome(config_, case_id, mbf_text, **kwargs)
        sha = hashlib.sha256(mbf_text.encode("utf-8")).hexdigest()
        out = case_paths(config_, case_id)["caepipe_out"]
        (out / "model.csv").write_text(invented_print(), encoding="utf-8")
        record = {"document_kind": RUN_RECORD_KIND, "route": "csv_from_batch", "mbf": {"sha256": sha},
                  "guest_mbf_sha256": sha, "observed_csv_path": {"host": "caepipe_out/model.csv"}} | outcome
        # As the harness writes it: the start state that goes with the route (a route the program ran on: completed).
        record.setdefault("start", {"state": FAKE_ROUTE_STARTS.get(record["route"], "completed")})
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
    monkeypatch.setattr(run_pilot.time, "sleep", command_sleep(lambda seconds: (
        state["sleeps"].append(seconds), state["events"].append(("sleep", seconds)))))
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
    [record_path] = comparisons(root, "cpb-g1-01")
    record = json.loads(record_path.read_text())
    assert record["document_kind"] == BENCHMARK_RECORD_KIND and record["piping_code"] == "B311-80"
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["mode"], manifest["outcome"], manifest["route"], manifest["attempt"]) == (
        "benchmark_live", "recorded", "csv_from_batch", 1)
    # The comparison names the solve it compared and the print it compared with; the manifest names the same solve.
    assert record["swbpipe_solve"] == manifest["swbpipe_solve"] and record_path.parent.name == \
        record["swbpipe_solve"]["solve_id"]
    assert record["caepipe_print"] == {"csv": "caepipe_out/model.csv", "route": "csv_from_batch", "sha256": hashlib.sha256(
        (root / "cases" / "cpb-g1-01" / "caepipe_out" / "model.csv").read_bytes()).hexdigest()}
    rendered = (root / "cases" / "cpb-g1-01" / "caepipe_in" / "model.mbf").read_text()
    assert "HGRA,B311-80,SI" in rendered
    # Standard output names the record, the solve and the comparison's counts by screening class.
    assert f"cpb-g1-01: route=csv_from_batch; benchmark record written: {record_path.relative_to(root)}; solve " \
           f"{record['swbpipe_solve']['solve_id']}; " in out and "within_profile " in out
    # A case already holding a collected print is not run live again: the print and its run record stay as they are.
    kept = {name: (root / "cases" / "cpb-g1-05" / "caepipe_out" / name).read_bytes()
            for name in ("model.csv", "run_record.json")}
    state["calls"].clear()
    assert run_pilot.main(state["argv"] + ["--batch", "2", "--pace-s", "90"], environ={}) == 1
    assert state["calls"] == []
    out = capsys.readouterr().out
    assert "batch 2/2: cpb-g1-05" in out and "cpb-g1-05: route=print_stored;" in out
    # Offline, SWBPIPE's side is solved again and compared again with the same stored print: a new solve folder and
    # a new comparison beside the first, the first left as it was, and nothing round-tripped.
    first = {path: path.read_bytes() for path in comparisons(root, "cpb-g1-02")}
    state["sleeps"].clear()
    offline = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline"]
    assert run_pilot.main(offline + ["--case", "cpb-g1-02"], environ={}) == 0
    assert state["calls"] == [] and state["sleeps"] == []
    records = comparisons(root, "cpb-g1-02")
    assert len(records) == 2 and all(path.read_bytes() == data for path, data in first.items())
    again = json.loads(next(path for path in records if path not in first).read_text())
    assert again["swbpipe_solve"]["solve_id"] != json.loads(next(iter(first)).read_text())["swbpipe_solve"]["solve_id"]
    assert again["caepipe_print"]["csv"] == "caepipe_out/model.csv"
    assert "cpb-g1-02: route=csv_from_batch; benchmark record written:" in capsys.readouterr().out
    assert {name: (root / "cases" / "cpb-g1-05" / "caepipe_out" / name).read_bytes() for name in kept} == kept


def comparisons(root: Path, case_id: str) -> list[Path]:
    """The comparison records of a case, one per SWBPIPE solve, oldest first."""
    return sorted((root / "cases" / case_id / "comparison").glob("*/benchmark_comparison.json"))


def solve_folder(root: Path, case_id: str, manifest: dict) -> Path:
    return root / "cases" / case_id / "swbpipe" / manifest["swbpipe_solve"]["solve_id"]


def tree_bytes(root: Path) -> dict[str, bytes]:
    return {str(item.relative_to(root)): item.read_bytes() if item.is_file() else b"<dir>"
            for item in sorted(root.rglob("*"))}


def test_the_case_manifest_binds_the_revision_runner_request_and_result(fake_benchmark):
    state = fake_benchmark
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    root = state["config"].exchange_root
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    solve = manifest["swbpipe_solve"]
    assert solve["source_commit"] == checkout_head()
    assert solve["source_tree_clean"] in (True, False, None)
    assert solve["runner_sha256"] == hashlib.sha256(Path(state["runner"]).read_bytes()).hexdigest()
    folder = solve_folder(root, "cpb-g1-01", manifest)
    assert folder.name.endswith(("-" + (checkout_head() or "unknown-commit")[:12],
                                 "-" + (checkout_head() or "unknown-commit")[:12] + "-dirty"))
    for name in ("request", "result"):
        assert solve[name] == f"swbpipe/{folder.name}/{name}.json"
        assert solve[f"{name}_sha256"] == hashlib.sha256((folder / f"{name}.json").read_bytes()).hexdigest()
    assert solve["standing"]["note"] == run_pilot.STANDING_NOTE


def checkout_head() -> str | None:
    """This checkout's commit, asked of Git with the inherited GIT_* variables removed."""
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    head = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "HEAD"], capture_output=True, text=True, env=env)
    return head.stdout.strip() if head.returncode == 0 else None


def test_an_inherited_git_dir_does_not_change_the_recorded_commit(fake_benchmark, monkeypatch, tmp_path):
    """Run from a Git hook, GIT_DIR would name another repository; the solve still records this checkout's commit."""
    state = fake_benchmark
    other = tmp_path / "other"
    subprocess.run(["git", "init", "-q", str(other)], check=True)
    monkeypatch.setenv("GIT_DIR", str(other / ".git"))
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    manifest = state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json"
    assert json.loads(manifest.read_text())["swbpipe_solve"]["source_commit"] == checkout_head()

@pytest.mark.parametrize("manifest", [None, "{ NOT JSON", json.dumps({"outcome": "invented", "custody": {
    "request_sha256": "0" * 64}})], ids=["no manifest", "an unreadable manifest", "an earlier manifest"])
def test_a_case_holding_earlier_files_is_solved_again_and_leaves_them_as_they_are(fake_benchmark, monkeypatch,
                                                                                   capsys, manifest):
    """A case folder already holding a SWBPIPE request or result (of an earlier layout or an interrupted run), with or
    without a case manifest of any content, is not refused: SWBPIPE is solved again in a new solve folder and the
    earlier files are left exactly as they are."""
    state = fake_benchmark
    solves = []
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: solves.append(case) or (
        {"invented": True}, {"payload": invented_payload()}, ""))
    root = state["config"].exchange_root
    folder = root / "cases" / "cpb-g1-01" / "swbpipe"
    folder.mkdir(parents=True)
    for name in ("request.json", "result.json"):
        (folder / name).write_text("INVENTED STORED FILE\n", encoding="utf-8")
    if manifest is not None:
        (folder.parent / "case_manifest.json").write_text(manifest, encoding="utf-8")
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert len(solves) == 1 and [call[0] for call in state["calls"]] == ["cpb-g1-01"]
    assert all((folder / name).read_text(encoding="utf-8") == "INVENTED STORED FILE\n"
               for name in ("request.json", "result.json"))
    recorded = json.loads((folder.parent / "case_manifest.json").read_text())
    assert recorded["outcome"] == "recorded" and (solve_folder(root, "cpb-g1-01", recorded) / "result.json").is_file()


def test_o_excl_does_not_replace_a_stored_file(tmp_path):
    """O_EXCL establishes that a stored file is not replaced (it says nothing about whether a racing process solved)."""
    stored = tmp_path / "request.json"
    stored.write_text("INVENTED STORED FILE\n", encoding="utf-8")
    with pytest.raises(FileExistsError):
        run_pilot._create_private_json(tmp_path, stored, {"other": True})
    assert stored.read_text(encoding="utf-8") == "INVENTED STORED FILE\n"


def test_the_clean_flag_is_what_git_reports(fake_benchmark):
    state = fake_benchmark
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    solve = json.loads((state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())[
        "swbpipe_solve"]
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    status = subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain"], capture_output=True, text=True, env=env)
    assert solve["source_tree_clean"] == ((status.stdout == "") if status.returncode == 0 else None)


def _bench_entry(state, model_id="cpb-g1-01"):
    return run_pilot.load_benchmark(state["manifest"], [model_id])[0]


def _collecting_round_trip(config_, case_id, mbf_text, **kwargs):
    sha = hashlib.sha256(mbf_text.encode("utf-8")).hexdigest()
    out = case_paths(config_, case_id)["caepipe_out"]
    (out / "model.csv").write_text(invented_print(), encoding="utf-8")
    record = {"document_kind": RUN_RECORD_KIND, "route": "csv_from_batch", "mbf": {"sha256": sha},
              "guest_mbf_sha256": sha, "observed_csv_path": {"host": "caepipe_out/model.csv"},
              "settings_guard": {"status": "unchanged"}, "start": {"state": "completed"}}
    (out / "run_record.json").write_text(json.dumps(record), encoding="utf-8")
    return record


def test_the_manifest_names_the_solve_when_the_round_trip_starts(fake_benchmark):
    """At the moment the round trip is invoked, the persisted case manifest is in the round_trip_started state and names
    the solve, whose two files are on disk; an interrupt there leaves it so."""
    state = fake_benchmark
    root = state["config"].exchange_root
    case = root / "cases" / "cpb-g1-01"
    seen = {}

    def round_trip(config_, case_id, mbf_text, **kwargs):
        manifest = json.loads((case / "case_manifest.json").read_text())
        seen["outcome"] = manifest["outcome"]
        for name in ("request", "result"):
            seen[name] = manifest["swbpipe_solve"][f"{name}_sha256"] == hashlib.sha256(
                (solve_folder(root, "cpb-g1-01", manifest) / f"{name}.json").read_bytes()).hexdigest()
        raise KeyboardInterrupt
    with pytest.raises(KeyboardInterrupt):
        run_pilot._run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"], offline=False,
                                       emit=lambda line: None, round_trip=round_trip, gui_fallback=False, attempt=1)
    assert seen == {"outcome": "round_trip_started", "request": True, "result": True}
    assert json.loads((case / "case_manifest.json").read_text())["outcome"] == "round_trip_started"


def test_an_interrupt_after_the_round_trip_returns_leaves_round_trip_started(fake_benchmark, monkeypatch):
    """The round_trip_started state shows only that no outcome of the attempt was written: an interrupt after the
    round trip returns and before the final manifest update leaves it so."""
    state = fake_benchmark
    case = state["config"].exchange_root / "cases" / "cpb-g1-01"
    calls = []

    def interrupt(*args, **kwargs):
        raise KeyboardInterrupt
    monkeypatch.setattr(run_pilot, "bound_csv", interrupt)  # after the round trip has returned
    with pytest.raises(KeyboardInterrupt):
        run_pilot._run_benchmark_model(
            _bench_entry(state), state["config"], runner=state["runner"], offline=False, emit=lambda line: None,
            round_trip=lambda *a, **k: calls.append(1) or _collecting_round_trip(*a, **k), gui_fallback=False,
            attempt=1, collect_only=True)
    assert calls == [1]
    assert json.loads((case / "case_manifest.json").read_text())["outcome"] == "round_trip_started"


def test_an_interrupted_later_manifest_write_leaves_the_earlier_manifest_readable(fake_benchmark, monkeypatch):
    state = fake_benchmark
    case = state["config"].exchange_root / "cases" / "cpb-g1-01"
    first = {}

    def round_trip(config_, case_id, mbf_text, **kwargs):
        first["bytes"] = (case / "case_manifest.json").read_bytes()

        def interrupted(descriptor, data):
            os.write(descriptor, data[: len(data) // 2])
            os.close(descriptor)
            raise KeyboardInterrupt
        monkeypatch.setattr(run_pilot, "_write_fd", interrupted)
        return _collecting_round_trip(config_, case_id, mbf_text, **kwargs)
    with pytest.raises(KeyboardInterrupt):
        run_pilot._run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"], offline=False,
                                       emit=lambda line: None, round_trip=round_trip, gui_fallback=False, attempt=1,
                                       collect_only=True)
    assert (case / "case_manifest.json").read_bytes() == first["bytes"]
    assert json.loads(first["bytes"])["outcome"] == "round_trip_started"
    assert not [path for path in case.iterdir() if path.name.endswith(".tmp")]


def test_a_settings_stop_records_its_outcome_before_it_stops(fake_benchmark):
    """A round trip that concluded with an unclean settings check: the outcome is written before the run stops,
    so the manifest is not left in the round_trip_started state."""
    state = fake_benchmark
    state["outcomes"][:] = [{"route": "timeout", "settings_guard": {"status": "owned_process_running"}}]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 1
    manifest = json.loads((state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["route"]) == ("no_record", "settings_stop")
    assert manifest["swbpipe_solve"]["request_sha256"]


@pytest.mark.parametrize("failure", [OSError, KeyboardInterrupt])
def test_a_failed_settings_stop_write_keeps_the_manifest_and_the_stopping_reason(fake_benchmark, monkeypatch, failure):
    """When writing the settings stop's outcome fails, the manifest already on disk stays readable and the stop
    wins: on an error, RunStopped is raised and its message and an emitted line name the failure (what the callers
    print or record); an interrupt propagates with the RunStopped as its context. The run record keeps the settings
    check either way."""
    state = fake_benchmark
    case = state["config"].exchange_root / "cases" / "cpb-g1-01"
    first = {}

    def round_trip(config_, case_id, mbf_text, **kwargs):
        first["bytes"] = (case / "case_manifest.json").read_bytes()

        def failing(descriptor, data):
            os.close(descriptor)
            raise failure("invented failure while writing the outcome")
        monkeypatch.setattr(run_pilot, "_write_fd", failing)
        record = _collecting_round_trip(config_, case_id, mbf_text, **kwargs)
        record["settings_guard"] = {"status": "owned_process_running"}
        (case / "caepipe_out" / "run_record.json").write_text(json.dumps(record), encoding="utf-8")
        return record
    expected, lines = (run_pilot.RunStopped if failure is OSError else KeyboardInterrupt), []
    with pytest.raises(expected) as raised:
        run_pilot._run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"], offline=False,
                                       emit=lines.append, round_trip=round_trip, gui_fallback=False, attempt=1,
                                       collect_only=True)
    if failure is OSError:
        note = "settings_stop; its outcome could not be recorded (OSError: invented failure while writing the outcome)"
        assert note in str(raised.value) and any(note.split("; ", 1)[1] in line for line in lines)
    else:
        assert isinstance(raised.value.__context__, run_pilot.RunStopped)
    assert (case / "case_manifest.json").read_bytes() == first["bytes"]
    assert json.loads(first["bytes"])["outcome"] == "round_trip_started"
    record = json.loads((case / "caepipe_out" / "run_record.json").read_text())
    assert record["settings_guard"]["status"] == "owned_process_running"
    assert not [path for path in case.iterdir() if path.name.endswith(".tmp")]


@pytest.mark.parametrize("stopping", [{"route": "timeout", "settings_guard": {"status": "owned_process_running"}},
                                      {"route": "batch_no_csv", "settings_guard": {"status": "unchanged"}}])
def test_a_stop_survives_a_containment_error_on_its_write(fake_benchmark, monkeypatch, capsys, stopping):
    """A stop's outcome write (a settings stop, or a batch run that left no CSV) meets a ContainmentError: the manifest
    is replaced by a link after the round_trip_started write. The run still stops at that model, no further model runs, and the
    line the benchmark command prints names the failed write."""
    state = fake_benchmark
    real = run_pilot._guard_stop

    def guard(config, case_id, settings, emit):
        manifest = state["config"].exchange_root / "cases" / case_id / "case_manifest.json"
        if case_id == "cpb-g1-01" and manifest.is_file() and not manifest.is_symlink():
            moved = manifest.with_name("moved_manifest.json")
            os.replace(manifest, moved)
            os.symlink(moved.name, manifest)
        return real(config, case_id, settings, emit)
    monkeypatch.setattr(run_pilot, "_guard_stop", guard)
    state["outcomes"][:] = [stopping]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 1
    out = capsys.readouterr().out
    route = "settings_stop" if stopping["route"] == "timeout" else "batch_no_csv"
    assert f"stopped after cpb-g1-01 ({route}; its outcome could not be recorded (ContainmentError" in out
    assert "route=containment_violation" not in out
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"]


def test_the_batch_callable_raises_the_stop_naming_a_failed_write(fake_benchmark, monkeypatch):
    """Through run_benchmark_model, the per-model step (a caller records str(error)): a failed write of a
    stop's outcome raises RunStopped, never a not-run outcome, and its text names the failure."""
    state = fake_benchmark
    state["outcomes"][:] = [{"route": "batch_no_csv", "settings_guard": {"status": "unchanged"}}]
    real = run_pilot._guard_stop

    def guard(config, case_id, settings, emit):
        def failing(descriptor, data):
            os.close(descriptor)
            raise OSError("invented failure while writing the outcome")
        monkeypatch.setattr(run_pilot, "_write_fd", failing)
        return real(config, case_id, settings, emit)
    monkeypatch.setattr(run_pilot, "_guard_stop", guard)
    with pytest.raises(run_pilot.RunStopped) as raised:
        run_pilot.run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"], offline=False,
                                      emit=lambda line: None, collect_only=True)
    assert f"{type(raised.value).__name__}: {raised.value}" == (
        "RunStopped: cpb-g1-01 (batch_no_csv; its outcome could not be recorded (OSError: invented failure while "
        "writing the outcome))")


def test_a_settings_stop_on_the_raise_path_records_its_outcome(fake_benchmark):
    """The round trip raised with no clean settings check: the outcome settings_stop is recorded, saying so."""
    state = fake_benchmark
    failure = RuntimeError("invented")
    failure.disposition = harness.disposition("not_dispatched")  # the program was never started: the raise path
    state["outcomes"][:] = [failure]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 1
    manifest = json.loads(
        (state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["route"]) == ("no_record", "settings_stop")
    assert "the round trip raised" in manifest["reason"] and manifest["swbpipe_solve"]["request_sha256"]


def test_the_runner_hashed_is_the_runner_run(fake_benchmark, monkeypatch, tmp_path):
    """A bare runner name is resolved on PATH, as subprocess does; that same path is hashed and handed to the solve,
    not a file of the same name in the working directory."""
    state = fake_benchmark
    on_path = tmp_path / "bin"
    on_path.mkdir()
    real = on_path / "invented-runner"
    real.write_text("THE RUNNER ON PATH", encoding="utf-8")
    real.chmod(0o755)
    elsewhere = tmp_path / "cwd"
    elsewhere.mkdir()
    (elsewhere / "invented-runner").write_text("A STALE FILE OF THE SAME NAME", encoding="utf-8")
    monkeypatch.chdir(elsewhere)
    monkeypatch.setenv("PATH", str(on_path))
    given = []
    real_solve = run_pilot.solve_swbpipe
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case_: given.append(runner) or real_solve(runner,
                                                                                                             case_))
    run_pilot._run_benchmark_model(_bench_entry(state), state["config"], runner="invented-runner", offline=False,
                                   emit=lambda line: None, round_trip=_collecting_round_trip, gui_fallback=False,
                                   attempt=1)
    assert given == [str(real.resolve())]
    manifest = json.loads((state["config"].exchange_root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert manifest["swbpipe_solve"]["runner_sha256"] == hashlib.sha256(real.read_bytes()).hexdigest()


def test_collect_only_offline_and_a_later_comparison_through_the_model_callable(fake_benchmark):
    """Through run_benchmark_model: collect-only offline is refused before anything is written; offline, a case with no
    folder is refused creating nothing; a collected case is then compared offline with its print left unchanged."""
    state = fake_benchmark
    root = state["config"].exchange_root
    entry = _bench_entry(state)
    before = tree_bytes(root)
    with pytest.raises(ValueError, match="collect-only run is live"):
        run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                      emit=lambda line: None, collect_only=True)
    assert tree_bytes(root) == before
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[1] == "no_stored_case"
    assert tree_bytes(root) == before
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=False,
                                         emit=lambda line: None, round_trip=_collecting_round_trip,
                                         collect_only=True)[0] == run_pilot.COLLECTED
    out = root / "cases" / "cpb-g1-01" / "caepipe_out"
    kept = {path.name: path.read_bytes() for path in out.iterdir()}
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[0] == run_pilot.RECORDED
    assert {path.name: path.read_bytes() for path in out.iterdir()} == kept
    assert len(comparisons(root, "cpb-g1-01")) == 1


def test_the_retry_reuses_its_own_solve_and_replaces_nothing(fake_benchmark, monkeypatch):
    state = fake_benchmark
    solves = []
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: solves.append(case["case_id"]) or (
        {"invented": case["case_id"]}, {"payload": invented_payload()}, ""))
    failure = RuntimeError("invented transport failure")
    failure.settings_guard = {"status": "unchanged"}  # a clean settings check, so the model is retried
    failure.disposition = harness.disposition("not_dispatched")  # and the program was never started
    state["outcomes"].append(failure)
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-01"] and solves == ["cpb-g1-01"]
    root = state["config"].exchange_root
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["attempt"], manifest["outcome"]) == (2, "recorded")
    assert manifest["swbpipe_solve"]["request_sha256"] == hashlib.sha256(
        (solve_folder(root, "cpb-g1-01", manifest) / "request.json").read_bytes()).hexdigest()
    assert len(list((root / "cases" / "cpb-g1-01" / "swbpipe").iterdir())) == 1


def test_collect_only_collects_and_binds_and_offline_then_compares(fake_benchmark, monkeypatch, capsys):
    """Collect-only: the print is collected and bound, and nothing solves SWBPIPE, parses the print or writes a
    comparison; the GUI fallback is off even when the guard restores the whole key. Offline then compares it."""
    state = fake_benchmark
    config = dataclasses.replace(state["config"], settings_guard=dataclasses.replace(
        state["config"].settings_guard, restore_scope="key"))
    assert config.settings_guard.restores_whole_key
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    case = config.exchange_root / "cases" / "cpb-g1-01"
    calls = []
    real = {name: getattr(run_pilot, name) for name in ("solve_swbpipe", "parse_caepipe_print_bytes",
                                                         "build_benchmark_record")}
    for name, function in real.items():
        monkeypatch.setattr(run_pilot, name, lambda *a, _name=name, **k: calls.append(_name) or real[_name](*a, **k))
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--collect-only"], environ={}) == 0
    assert calls == [] and state["calls"][0][1] == {"gui_fallback": False}
    manifest = json.loads((case / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["route"], manifest["parser_status"], manifest["swbpipe_solve"]) == (
        "collected", "csv_from_batch", "not_parsed", None)
    assert manifest["csv"] == "caepipe_out/model.csv"
    assert not any((case / "parsed").iterdir()) and not any((case / "comparison").iterdir())
    assert "cpb-g1-01: route=csv_from_batch; collected and bound; not parsed or compared" in capsys.readouterr().out
    with pytest.raises(SystemExit):
        run_pilot.main(["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline",
                        "--collect-only"], environ={})
    assert run_pilot.main(["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline", "--case",
                           "cpb-g1-01"], environ={}) == 0
    assert calls == ["solve_swbpipe", "parse_caepipe_print_bytes", "build_benchmark_record"]
    assert len(comparisons(config.exchange_root, "cpb-g1-01")) == 1


def test_offline_leaves_a_case_without_a_bound_print_untouched(fake_benchmark):
    """Offline, a case with no stored folder is refused at admission, before anything is solved, and no folder is
    created."""
    state = fake_benchmark
    root = state["config"].exchange_root
    before = tree_bytes(root)
    offline = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline", "--case", "cpb-g1-03"]
    assert run_pilot.main(offline, environ={}) == 1
    assert tree_bytes(root) == before


def test_offline_a_stored_folder_without_a_bound_print_is_refused_before_any_solve(fake_benchmark):
    """Seeded: offline, a case whose folders exist and hold no stored, bound print is refused before anything is
    solved, so no SWBPIPE request or result is created and nothing changes."""
    state = fake_benchmark
    root = state["config"].exchange_root
    case_paths(state["config"], "cpb-g1-03")
    before = tree_bytes(root)
    offline = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline", "--case", "cpb-g1-03"]
    assert run_pilot.main(offline, environ={}) == 1
    assert tree_bytes(root) == before


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
    records = [path for run_id in run_ids for path in comparisons(root, run_id)]
    assert len(records) == 2
    first, second = (json.loads(record.read_text()) for record in records)
    assert first["model_id"] == second["model_id"] == "cpb-p2-01"
    assert "pressure_elongation_convention" not in first["load_cases"][1]["inexact_inputs"]
    assert "pressure_elongation_convention" in second["load_cases"][1]["inexact_inputs"]
    manifests = [json.loads((root / "cases" / run_id / "case_manifest.json").read_text()) for run_id in run_ids]
    assert [(item["case_id"], item["series_run"], item["oppres_line"], item["outcome"]) for item in manifests] == [
        (run_ids[0], RUNS[0], "0,0,0,1,0", "recorded"), (run_ids[1], RUNS[1], "0,1,0,1,1", "recorded")]


def test_a_capture_run_is_collected_bound_and_parsed_and_never_compared(fake_benchmark, monkeypatch, capsys):
    """A capture-only run (no SWBPIPE state equals it) goes through the batch round trip like any run: its print is
    collected, bound and parsed, no SWBPIPE solve or comparison is made, and offline it is parsed again from the stored
    print. A mixed list runs its other model as usual."""
    state = fake_benchmark
    path = benchmark_manifest(state["manifest"].parent.parent / "pressure-set", ids=("cpb-p2-01", "cpb-g1-01"))
    [capture] = run_pilot.load_benchmark(path, ["cpb-p2-01"], RUNS[2])
    [plain] = run_pilot.load_benchmark(path, ["cpb-g1-01"])
    config = state["config"]
    root = config.exchange_root
    solves = []
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: solves.append(case["case_id"]) or (
        {"invented": True}, {"payload": invented_payload()}, ""))
    lines: list[str] = []
    assert run_pilot.run_benchmark([plain, capture], config, runner=state["runner"], offline=False, emit=lines.append,
                                   stops=run_pilot.BenchmarkStops(state["freeze"], state["exclusive"])) == 0
    run_id = capture["run_id"]
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", run_id] and solves == ["cpb-g1-01"]
    assert "0,1,0,1,0" in state["mbf"][run_id]  # its own substituted OPPRES line
    case = root / "cases" / run_id
    manifest = json.loads((case / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["capture_only"], manifest["swbpipe_solve"], manifest["parser_status"]) == (
        "captured", True, None, "parsed")
    assert json.loads((case / "parsed" / "caepipe_results.json").read_text())["sections_parsed"]
    assert not any((case / "comparison").iterdir()) and not any((case / "swbpipe").iterdir())
    assert any(line.startswith(f"{run_id}: route=csv_from_batch; print captured and parsed: ")
               and line.endswith("no SWBPIPE counterpart, so no comparison") for line in lines)
    kept = {item.name: item.read_bytes() for item in (case / "caepipe_out").iterdir()}
    lines.clear()
    assert run_pilot.run_benchmark_model(capture, config, runner=state["runner"], offline=True,
                                         emit=lines.append)[0] == run_pilot.CAPTURED
    assert {item.name: item.read_bytes() for item in (case / "caepipe_out").iterdir()} == kept
    assert solves == ["cpb-g1-01"] and not any((case / "comparison").iterdir())


TOKEN = "0123456789abcdef0123456789abcdef"


def _run_mbf_sha256(entry: dict) -> str:
    mbf = render_caepipe_mbf_si(entry["model"], title=entry["model_id"],
                                reference_temperature_c=entry.get("reference_temperature_c", 20.0),
                                piping_code=entry["piping_code"])
    return hashlib.sha256(benchmark_models.run_mbf_text(entry, mbf.text).encode("utf-8")).hexdigest()


def _seed_print(state, entry: dict, **fields) -> tuple[Path, bytes]:
    """An invented print stored under an attempt token, with a harness run record binding it (fields override)."""
    paths = case_paths(state["config"], entry["model_id"])
    data = invented_print().encode("utf-8")
    (paths["caepipe_out"] / TOKEN).mkdir()
    (paths["caepipe_out"] / TOKEN / "model.csv").write_bytes(data)
    sha = _run_mbf_sha256(entry)
    record = {"document_kind": RUN_RECORD_KIND, "route": "csv_from_batch", "mbf": {"sha256": sha},
              "guest_mbf_sha256": sha, "input_hash_verified": True, "attempt_token": TOKEN, "exit_status": 0,
              "start": {"state": "completed"}, "observed_csv_path": {"host": f"caepipe_out/{TOKEN}/model.csv"},
              "guest_log": {"status": "csv_from_batch", "csv_path": f"C:\\invented-work\\{TOKEN}\\model.csv"},
              "outputs_collected": ["model.csv"], "outputs_sha256": {"model.csv": hashlib.sha256(data).hexdigest()}}
    record.update(fields)
    (paths["caepipe_out"] / "run_record.json").write_text(json.dumps(record), encoding="utf-8")
    return paths["root"], data


class _FixedClock(run_pilot.datetime):
    @classmethod
    def now(cls, tz=None):
        return run_pilot.datetime(2026, 11, 21, 12, 0, 0, tzinfo=tz)


def test_a_new_solve_never_reuses_an_id_whose_solve_or_comparison_folder_exists(fake_benchmark, monkeypatch):
    """An id whose comparison folder exists, or whose solve folder appears between the check and its creation, is
    passed over for the next suffix, so no earlier comparison can be replaced."""
    monkeypatch.setattr(run_pilot, "datetime", _FixedClock)
    root = fake_benchmark["config"].exchange_root
    paths = case_paths(fake_benchmark["config"], "cpb-g1-01")
    identity = {"source_commit": "a" * 40, "source_tree_clean": True}
    name = "20261121T120000Z-" + "a" * 12
    (paths["comparison"] / name).mkdir()
    assert run_pilot._new_solve_folder(root, paths["swbpipe"], identity)[0] == f"{name}-2"
    # A folder that appears after the existence check: mkdir raises FileExistsError and the next suffix is taken.
    (paths["swbpipe"] / f"{name}-3").mkdir()
    real = os.path.lexists
    monkeypatch.setattr(run_pilot.os.path, "lexists", lambda path: False if Path(path).name == f"{name}-3"
                        else real(path))
    assert run_pilot._new_solve_folder(root, paths["swbpipe"], identity)[0] == f"{name}-4"


def test_a_comparison_record_is_created_and_never_replaced(fake_benchmark, monkeypatch):
    state = fake_benchmark
    entry = _bench_entry(state)
    case, _ = _seed_print(state, entry)
    seeded = case / "comparison" / "seeded" / "benchmark_comparison.json"
    seeded.parent.mkdir()
    seeded.write_text("EARLIER COMPARISON\n", encoding="utf-8")

    def same_id(root, swbpipe_dir, identity):  # an id whose comparison already exists, its solve folder new
        folder = swbpipe_dir / "seeded"
        folder.mkdir()
        return "seeded", folder
    monkeypatch.setattr(run_pilot, "_new_solve_folder", same_id)
    with pytest.raises(FileExistsError):
        run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                      emit=lambda line: None)
    assert seeded.read_text(encoding="utf-8") == "EARLIER COMPARISON\n"


def test_a_comparison_binds_its_print_by_digest_and_token(fake_benchmark):
    state = fake_benchmark
    entry = _bench_entry(state)
    case, data = _seed_print(state, entry)
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[0] == run_pilot.RECORDED
    [path] = comparisons(state["config"].exchange_root, "cpb-g1-01")
    assert json.loads(path.read_text())["caepipe_print"] == {
        "csv": f"caepipe_out/{TOKEN}/model.csv", "route": "csv_from_batch",
        "sha256": hashlib.sha256(data).hexdigest(), "attempt_token": TOKEN}
    # A print whose bytes differ from the run record's digest is not compared.
    (case / "caepipe_out" / TOKEN / "model.csv").write_bytes(data.replace(b"-4.000", b"-4.001"))
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[:2] == (run_pilot.NOT_RUN, "print_digest_mismatch")
    assert len(comparisons(state["config"].exchange_root, "cpb-g1-01")) == 1


def test_a_stored_print_refuses_every_live_attempt_and_an_odd_record_does_not_crash(fake_benchmark):
    state = fake_benchmark
    entry = _bench_entry(state)
    _seed_print(state, entry)
    for attempt in (1, 2):
        assert run_pilot._run_benchmark_model(
            entry, state["config"], runner=state["runner"], offline=False, emit=lambda line: None,
            round_trip=lambda *a, **k: pytest.fail("round-tripped"), gui_fallback=False,
            attempt=attempt)[:2] == (run_pilot.NOT_RUN, "print_stored")
    # A run record whose observed_csv_path is not a mapping names no print: the live run proceeds.
    other = _bench_entry(state, "cpb-g1-02")
    _seed_print(state, other, observed_csv_path="caepipe_out/model.csv")
    calls = []
    run_pilot._run_benchmark_model(other, state["config"], runner=state["runner"], offline=False,
                                   emit=lambda line: None, round_trip=lambda *a, **k: calls.append(1) or {
                                       "route": "not_started", "start": {"state": "not_started"},
                                       "settings_guard": {"status": "unchanged"}},
                                   gui_fallback=False, attempt=1)
    assert calls == [1]


def test_a_print_collected_before_a_harness_error_is_stored_bound_and_compared(fake_benchmark):
    """The batch exited 0 on the verified input and its print was collected under its token, but the run record ended
    in harness_error (a settings write-back failed afterwards), so observed_csv_path.host is null. The print is still
    the case's: a live run is refused as print_stored and --offline binds it by digest and compares it."""
    state = fake_benchmark
    entry = _bench_entry(state)
    _seed_print(state, entry, route="harness_error", observed_csv_path={"guest": None, "host": None},
                harness_error="RuntimeError: invented settings write-back failure")
    lines: list[str] = []
    assert run_pilot._run_benchmark_model(
        entry, state["config"], runner=state["runner"], offline=False, emit=lines.append,
        round_trip=lambda *a, **k: pytest.fail("round-tripped"), gui_fallback=False, attempt=1)[:2] == (
        run_pilot.NOT_RUN, "print_stored")
    assert "recovered from a run record that ended in harness_error" in lines[0]
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[0] == run_pilot.RECORDED
    [path] = comparisons(state["config"].exchange_root, "cpb-g1-01")
    printed = json.loads(path.read_text())["caepipe_print"]
    assert (printed["csv"], printed["route"], printed["run_record_route"]) == (
        f"caepipe_out/{TOKEN}/model.csv", "csv_from_batch", "harness_error")


def test_a_harness_error_record_that_does_not_establish_a_clean_batch_still_protects_its_folder(fake_benchmark):
    state = fake_benchmark
    entry = _bench_entry(state)
    _seed_print(state, entry, route="harness_error", observed_csv_path={"guest": None, "host": None}, exit_status=1)
    lines: list[str] = []
    assert run_pilot._run_benchmark_model(
        entry, state["config"], runner=state["runner"], offline=False, emit=lines.append,
        round_trip=lambda *a, **k: pytest.fail("round-tripped"), gui_fallback=False, attempt=1)[:2] == (
        run_pilot.NOT_RUN, "print_unestablished")
    assert "the batch's exit status is not 0" in lines[0]
    assert run_pilot.run_benchmark_model(entry, state["config"], runner=state["runner"], offline=True,
                                         emit=lambda line: None)[:2] == (run_pilot.NOT_RUN, "harness_error")


def test_a_manifest_that_is_a_link_is_refused_at_admission(fake_benchmark):
    """A case manifest that is a symbolic link is refused before anything is solved, written or round-tripped."""
    state = fake_benchmark
    case = state["config"].exchange_root / "cases" / "cpb-g1-01"
    case.mkdir(parents=True)
    (case / "elsewhere.json").write_text("{}", encoding="utf-8")
    os.symlink("elsewhere.json", case / "case_manifest.json")
    before = tree_bytes(state["config"].exchange_root)
    for offline in (False, True):
        assert run_pilot.run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"],
                                             offline=offline, emit=lambda line: None)[:2] == (
            run_pilot.NOT_RUN, "manifest_link")
    assert state["calls"] == [] and tree_bytes(state["config"].exchange_root) == before


def test_a_window_run_is_refused_by_name_before_anything_runs(fake_benchmark, capsys):
    """A run declared with a window instruction (a hand-entry or an option-addition run) cannot be made by the batch
    round trip: every entry point refuses it by name before anything is solved, written or round-tripped."""
    state = fake_benchmark
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    entry = next(item for item in manifest["models"] if any(run.get("kind") == "hand_entry"
                                                             for run in item.get("runs") or []))
    [run] = [run for run in entry["runs"] if run.get("kind") == "hand_entry"]
    path = benchmark_manifest(state["manifest"].parent.parent / "hand-set", ids=(entry["model_id"],))
    [window] = run_pilot.load_benchmark(path, [entry["model_id"]], run["series"])
    root = state["config"].exchange_root
    before = tree_bytes(root)
    lines: list[str] = []
    expected = f"{window['run_id']}: route=window_run; this run is declared with a window instruction"
    assert run_pilot.main(["--benchmark", str(path), "--case", entry["model_id"], "--series", run["series"]]
                          + state["argv"][2:], environ={}) == 2
    assert capsys.readouterr().out.startswith(expected)
    assert run_pilot.run_benchmark([window], state["config"], runner=state["runner"], offline=True,
                                   emit=lines.append) == 2
    assert run_pilot.run_benchmark_model(window, state["config"], runner=state["runner"], offline=False,
                                         emit=lines.append)[:2] == (run_pilot.NOT_RUN, run_pilot.WINDOW_RUN)
    assert len(lines) == 2 and all(line.startswith(expected) for line in lines)
    assert state["calls"] == [] and tree_bytes(root) == before


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
    error.disposition = harness.disposition("not_dispatched")  # the program was never started: retry is permitted
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
    # Retried only because the program was never started; a completed attempt with no CSV stops (test_p1_5_...).
    no_csv = {"route": "csv_missing", "observed_csv_path": {}, "settings_guard": {"status": "unchanged"},
              "start": {"state": "not_dispatched"}}
    state["outcomes"][:] = [no_csv, no_csv]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-01", "cpb-g1-02"]
    assert "cpb-g1-01: route=csv_missing; the run record names no collected CAEPIPE CSV; no benchmark record after " \
           "one retry" in capsys.readouterr().out
    root = state["config"].exchange_root
    manifest = json.loads((root / "cases" / "cpb-g1-01" / "case_manifest.json").read_text())
    assert (manifest["outcome"], manifest["route"], manifest["attempt"]) == ("no_record", "csv_missing", 2)
    assert comparisons(root, "cpb-g1-01") == []


def test_a_failed_write_of_a_retry_outcome_stops_the_list(fake_benchmark, monkeypatch, capsys):
    """The round trip raised with the program never started (a retry is due), and writing that outcome meets a
    ContainmentError: the list stops there, so the retry is not lost and the next model does not run."""
    state = fake_benchmark
    state["outcomes"][:] = [raised_after_a_clean_check()]
    real = run_pilot._benchmark_manifest

    def failing(*args, **kwargs):
        if kwargs.get("route") == "round_trip_raised":
            raise ContainmentError("invented containment failure writing the outcome")
        return real(*args, **kwargs)
    monkeypatch.setattr(run_pilot, "_benchmark_manifest", failing)
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 1
    out = capsys.readouterr().out
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"], out
    assert "its outcome could not be recorded (ContainmentError: invented containment failure" in out
    assert "route=containment_violation" not in out


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
    monkeypatch.setattr(run_pilot.time, "sleep",
                        command_sleep(lambda seconds: state["freeze"].write_text("", encoding="ascii")))
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
    monkeypatch.setattr(run_pilot.time, "sleep", command_sleep(poll))
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert polls == [30.0, 30.0] and [call[0] for call in state["calls"]] == ["cpb-g1-01"]
    assert "cpb-g1-01: an exclusive job is pending; waiting for it (up to 1800 s)" in capsys.readouterr().out
    state["calls"].clear()
    state["exclusive"].write_text(f"{os.getpid()}\n", encoding="ascii")
    monkeypatch.setattr(run_pilot.time, "sleep", command_sleep(lambda seconds: None))
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01"], environ={}) == 1
    assert state["calls"] == []
    assert "stopped before cpb-g1-01: an exclusive job was still pending after 1800 s" in capsys.readouterr().out


# The answer's form is the Windows version line alone, with nothing on stderr: any other text with exit 0 (an
# invented banner, a sign-in or licence text) is not an answer.
@pytest.mark.parametrize("answer, status", [
    ((0, "\r\nMicrosoft Windows [Version 10.0.19045.5011]\r\n", ""), "alive"),
    ((0, "Invented Guest [Version 0.0]\r\n", ""), "unreachable"),
    ((0, "Please sign in to continue.\r\n", ""), "unreachable"),
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
    assert "Invented Guest" not in json.dumps(result) and "19045" not in json.dumps(result)


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


# The run phase's start dispositions, as the model-list command and its per-model step take them (synthetic only).
def _manifest_of(state, case_id="cpb-g1-01") -> dict:
    return json.loads((state["config"].exchange_root / "cases" / case_id / "case_manifest.json").read_text())


@pytest.mark.parametrize("how", ["StartUnresolved", "raised while unresolved"])
def test_an_unresolved_start_stops_with_the_manifest_left_at_round_trip_started(fake_benchmark, capsys, how):
    state = fake_benchmark
    if how == "StartUnresolved":
        error = StartUnresolved("start unresolved: invented")
    else:
        error = OSError("invented failure writing the record")
        error.start_unresolved = True
    error.settings_guard = {"status": "start_unresolved", "reason": "restoration inhibited: start unresolved"}
    state["outcomes"][:] = [error]
    assert run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={}) == 1
    manifest = _manifest_of(state)
    assert (manifest["outcome"], manifest["route"]) == ("round_trip_started", "round_trip_started")
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"]  # never retried, and no further model ran
    out = capsys.readouterr().out
    assert "cpb-g1-01: route=start_unresolved;" in out and "retrying" not in out
    # Through the per-model step (run_benchmark_model): the stop is raised, and no outcome is written.
    state["calls"].clear()
    state["outcomes"][:] = [error]
    with pytest.raises(run_pilot.RunStopped, match="start_unresolved"):
        run_pilot.run_benchmark_model(_bench_entry(state, "cpb-g1-03"), state["config"], runner=state["runner"],
                                      offline=False, emit=lambda line: None, collect_only=True)
    assert _manifest_of(state, "cpb-g1-03")["outcome"] == "round_trip_started"


def test_a_partial_start_is_recorded_never_retried_and_a_second_stops_the_run(fake_benchmark, capsys):
    state = fake_benchmark
    partial = {"route": "start_partial", "settings_guard": {"status": "unchanged"}, "guest_mbf_sha256": None,
               "observed_csv_path": {"host": None}}
    state["outcomes"][:] = [dict(partial), dict(partial)]
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-02"]  # one round trip each, no retry
    first = _manifest_of(state)
    assert (first["outcome"], first["route"], first["attempt"]) == ("partial", "start_partial", 1)
    out = capsys.readouterr().out
    assert "stopped after cpb-g1-02: a second attempted run that did not agree in this run" in out and \
        "retrying" not in out


def test_two_not_started_attempts_are_not_run_and_never_retried(fake_benchmark):
    state = fake_benchmark
    state["outcomes"][:] = [{"route": "not_started", "settings_guard": {"status": "unchanged"},
                             "guest_mbf_sha256": None, "observed_csv_path": {"host": None}}]
    status = run_pilot.run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"],
                                           offline=False, emit=lambda line: None)
    assert status[:2] == (run_pilot.NOT_RUN, "not_started")
    assert (_manifest_of(state)["outcome"], _manifest_of(state)["route"]) == ("no_record", "not_started")
    assert len(state["calls"]) == 1



# --------------------------------------------------------------------------------------------------------------
# P1 regressions from the first independent reading of the run-phase change, written before their repair. Each
# drives a real round trip against the fake guest, hands what it raised to run_pilot (the outer caller), and observes
# there: the dispatch count, restore writes, whether the next model ran, and the disposition received. Each failed
# on the head that was read (60bb0d182 and before), and passes on the settled repair.
# --------------------------------------------------------------------------------------------------------------
import signal as _signal  # noqa: E402
import sys as _sys  # noqa: E402
from types import SimpleNamespace  # noqa: E402

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import ContainmentError  # noqa: E402

from tests.test_caepipe_start_reconciliation import (  # noqa: E402
    CASE as RECONCILE_CASE, GUARD as RECONCILE_GUARD, Registry as InventedRegistry, config_for, guest_for,
    run as run_attempt)
from tests.caepipe_fake_guest import Program as InventedProgram  # noqa: E402



def _invented_clock(monkeypatch) -> dict:
    state = {"t": 0.0}
    monkeypatch.setattr(harness, "time", SimpleNamespace(monotonic=lambda: state["t"]))
    monkeypatch.setattr(harness, "_backoff", lambda seconds: state.__setitem__("t", state["t"] + seconds))
    return state


def _partial_run_raising(tmp_path, monkeypatch, failing: str):
    """A real round trip whose attempt is established partial (claimed, launched, launcher and program killed,
    quiescent, no terminal report), with a clean guard, whose reporting then fails at `failing`. Returns what it
    raised, the fake guest and its invented registry."""
    config = config_for(tmp_path / "partial", RECONCILE_GUARD)
    with monkeypatch.context() as patch:
        clock_state = _invented_clock(patch)
        guest = guest_for(config, clock_state)
        registry = InventedRegistry(guest, change_on_launch=False)
        guest.plan = {1: {"kill_at": "launched", "reply": "transport"}}
        if failing == "record construction":
            def broken(item):
                raise ValueError("invented record construction failure")
            patch.setattr(harness, "_failed_write_stderr", broken)
        else:
            name = {"guard result write": harness.GUARD_RESULT_FILE, "dispatch stream write": "guest_stdout.txt",
                    "record write": "run_record.json"}[failing]
            real_write = harness.write_private

            def failing_write(root, path, data):
                if Path(path).name == name:
                    raise OSError(f"invented failure writing {name}")
                return real_write(root, path, data)
            patch.setattr(harness, "write_private", failing_write)
        with pytest.raises(Exception) as raised:
            run_attempt(config, guest)
    return raised.value, guest, registry


@pytest.mark.parametrize("failing", ["guard result write", "dispatch stream write", "record construction",
                                     "record write"])
def test_p1_1_a_partial_runs_reporting_error_never_becomes_retry(fake_benchmark, tmp_path, monkeypatch, capsys,
                                                                 failing):
    error, guest, registry = _partial_run_raising(tmp_path, monkeypatch, failing)
    assert guest.kinds.count("attempt") == 1 and len(guest.launches) == 1  # one dispatch, one start
    assert registry.writes == []
    state = fake_benchmark
    state["outcomes"][:] = [error]
    exit_status = run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={})
    out = capsys.readouterr().out
    # Not retried, and with its evidence not saved the run stops: the next model does not run.
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"], out
    assert exit_status == 1
    assert "retrying" not in out and _manifest_of(state)["route"] != "round_trip_raised"
    assert "start_partial" in out  # the disposition the caller received


def test_p1_2_a_failure_to_record_partial_is_never_not_run(fake_benchmark, monkeypatch):
    state = fake_benchmark
    state["outcomes"][:] = [{"route": "start_partial", "settings_guard": {"status": "unchanged"},
                             "guest_mbf_sha256": None, "observed_csv_path": {"host": None}}]
    real = run_pilot._benchmark_manifest

    def failing(*args, **kwargs):
        if kwargs.get("outcome") == "partial":
            raise ContainmentError("invented containment failure saving a partial outcome")
        return real(*args, **kwargs)
    monkeypatch.setattr(run_pilot, "_benchmark_manifest", failing)
    with pytest.raises(run_pilot.RunStopped, match="start_partial"):
        run_pilot.run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"], offline=False,
                                      emit=lambda line: None, collect_only=True)
    assert len(state["calls"]) == 1


def test_p1_2_a_partial_whose_outcome_cannot_be_recorded_stops_the_list(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    partial = {"route": "start_partial", "settings_guard": {"status": "unchanged"}, "guest_mbf_sha256": None,
               "observed_csv_path": {"host": None}}
    state["outcomes"][:] = [dict(partial), dict(partial)]
    real = run_pilot._benchmark_manifest

    def failing(*args, **kwargs):
        if kwargs.get("outcome") == "partial":
            raise ContainmentError("invented containment failure saving a partial outcome")
        return real(*args, **kwargs)
    monkeypatch.setattr(run_pilot, "_benchmark_manifest", failing)
    assert run_pilot.main(state["argv"], environ={}) == 1
    out = capsys.readouterr().out
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"], out  # never NOT_RUN, so the list stops
    assert "start_partial" in out and "route=containment_violation" not in out


# The post-run boundary: after reconciliation left the start unresolved and before the guard checks begin, and
# again where the guard's result is computed but not yet handed to the live state.
BOUNDARIES = {"after reconciliation": ("# other or the result. While the start is unresolved these are read-only "
                                       "diagnosis: no restore.", "    if guarded:"),
              "before the live guard result": (None, '        live["settings"] = settings')}


def _boundary_line(where: str) -> int:
    lines = Path(harness.__file__).read_text(encoding="utf-8").splitlines()
    comment, text = BOUNDARIES[where]
    start = next(i for i, line in enumerate(lines) if comment in line) if comment else 0
    found = [i + 1 for i, line in enumerate(lines) if i > start and line == text]
    return found[0]


def _unresolved_run_terminated(tmp_path, monkeypatch, where: str, signum: int, again_in_recovery: bool):
    """A real round trip whose program lives on (the start unresolved), terminated by a real signal at `where`,
    and, with `again_in_recovery`, a second time as the outer recovery begins."""
    config = config_for(tmp_path / "unresolved", RECONCILE_GUARD)
    expected = harness.TerminationRequested if signum == _signal.SIGTERM else KeyboardInterrupt
    with monkeypatch.context() as patch:
        clock_state = _invented_clock(patch)
        guest = guest_for(config, clock_state)
        registry = InventedRegistry(guest)
        guest.plan = {1: {"run": "launched", "reply": "transport"}}
        guest.programs[1] = InventedProgram(lingers=True)
        body, line = harness._round_trip_body.__code__, _boundary_line(where)
        fired = []
        previous = _sys.gettrace()

        def tracer(frame, event, arg):
            if event == "line" and frame.f_code is body and frame.f_lineno == line and not fired:
                fired.append("boundary")
                _signal.raise_signal(signum)
            return tracer
        if again_in_recovery:
            real_recovery = harness._recover_guard_result

            def recovery(*args, **kwargs):  # a second termination as the outer recovery begins
                if fired == ["boundary"]:
                    fired.append("recovery")
                    _signal.raise_signal(_signal.SIGTERM)
                return real_recovery(*args, **kwargs)
            patch.setattr(harness, "_recover_guard_result", recovery)
        try:
            _sys.settrace(tracer)
            with pytest.raises(BaseException) as raised:
                run_attempt(config, guest)
        finally:
            _sys.settrace(previous)
    assert fired == (["boundary", "recovery"] if again_in_recovery else ["boundary"])
    assert isinstance(raised.value, expected if not again_in_recovery else harness.TerminationRequested)
    path = config.exchange_root / "cases" / RECONCILE_CASE / "caepipe_out" / harness.GUARD_RESULT_FILE
    stored = json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None  # an interrupted write may
    return raised.value, guest, registry, stored                                          # leave none


@pytest.mark.parametrize("where", sorted(BOUNDARIES))
@pytest.mark.parametrize("signum, again", [(_signal.SIGTERM, False), (_signal.SIGINT, False), (_signal.SIGTERM, True)],
                         ids=["SIGTERM", "SIGINT", "SIGTERM then SIGTERM in recovery"])
def test_p1_3_a_termination_at_the_post_run_boundary_keeps_the_start_unresolved(fake_benchmark, tmp_path, monkeypatch,
                                                                                where, signum, again):
    error, guest, registry, stored = _unresolved_run_terminated(tmp_path, monkeypatch, where, signum, again)
    assert guest.kinds.count("attempt") == 1 and registry.writes == []  # one dispatch, zero restore writes
    # The persisted guard result, if any was saved, says start_unresolved: never settings_check_failed.
    assert stored is None or stored["status"] == "start_unresolved", stored
    state = fake_benchmark
    state["outcomes"][:] = [error]
    with pytest.raises(type(error)) as received:  # what run_pilot's caller receives
        run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={})
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"]  # the next model did not run
    assert getattr(received.value, "start_unresolved", False) is True
    assert received.value.settings_guard["status"] == "start_unresolved"
    assert _manifest_of(state)["outcome"] == "round_trip_started"


# --------------------------------------------------------------------------------------------------------------
# Exits that are never success: a reply whose own exit is nonzero (1, 3, 255, or a signal's negative status) never
# becomes a success route, an export reported as exported with a nonzero exit never collects, and a collection copy
# with a nonzero exit never gives an exported or collected route. Each runs to the outer caller.
# --------------------------------------------------------------------------------------------------------------
from tests.caepipe_fake_guest import FakeGuestModel as InventedGuest, classify as classify_command, done as answer  # noqa: E402
from tests.test_caepipe_correlation import (  # noqa: E402
    COPIED, EXPORTED_SHA, MOD_SHA, current_mbf_sha, export_runner, solved_case)

NONZERO_EXITS = (1, 3, 255, -9)


def _pilot_round_trip(state, guest):
    """The pilot's round trip, run for real against `guest`, on the pilot's own exchange folder."""
    def call(config_, case_id, mbf_text, **kwargs):
        return harness.run_caepipe_round_trip(config_, case_id, mbf_text, runner=guest.runner, **kwargs)
    return call


def _pilot_guest(state, monkeypatch) -> tuple[InventedGuest, InventedRegistry]:
    _invented_clock(monkeypatch)
    guest = InventedGuest(state["config"].exchange_root, state["config"].guest_exchange_root)
    return guest, InventedRegistry(guest, change_on_launch=False)


def _collect_one(state, outcome) -> tuple:
    state["outcomes"][:] = [outcome]
    try:
        return run_pilot.run_benchmark_model(_bench_entry(state), state["config"], runner=state["runner"],
                                             offline=False, emit=lambda line: None, collect_only=True)
    except run_pilot.RunStopped as stopped:
        return ("stopped", str(stopped), "")


def test_a_clean_attempt_collects_through_the_pilot(fake_benchmark, monkeypatch):
    state = fake_benchmark
    guest, registry = _pilot_guest(state, monkeypatch)
    status = _collect_one(state, _pilot_round_trip(state, guest))
    assert status[:2] == (run_pilot.COLLECTED, "csv_from_batch") and guest.kinds.count("attempt") == 1
    assert registry.writes == []


@pytest.mark.parametrize("code", NONZERO_EXITS)
@pytest.mark.parametrize("kind", ["attempt", "claim_read", "cim", "job", "finished_read", "chunk"])
def test_a_success_reply_with_a_nonzero_exit_never_becomes_a_success_route(fake_benchmark, monkeypatch, kind, code):
    state = fake_benchmark
    guest, registry = _pilot_guest(state, monkeypatch)
    if kind == "attempt":
        guest.plan = {1: {"kill_at": "launched"}}  # no terminal report: only the reply claims success
    base = guest.runner

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        if classify_command(argv[6]) != kind:
            return result
        if kind == "attempt":  # the reply a successful script prints, with another exit
            report = {"schema": harness.GUEST_REPORT_SCHEMA, "attempt_token": guest.attempts[-1].token,
                      "status": "csv_from_batch", "mbf_sha256": "0" * 64, "csv_path": "model.csv"}
            return answer(argv, code, json.dumps(report), "")
        return answer(argv, code, result.stdout, result.stderr)  # the read's own success form, another exit
    guest.runner = runner
    status = _collect_one(state, _pilot_round_trip(state, guest))
    assert status[0] not in (run_pilot.COLLECTED, run_pilot.RECORDED), status
    assert guest.kinds.count("attempt") == 1 and registry.writes == []
    if kind == "attempt":
        assert status[:2] == (run_pilot.PARTIAL, "start_partial"), status
    elif kind == "chunk":  # completed, nothing collected: never retried, the list stops with the route named
        assert status[0] == "stopped" and _manifest_of(state)["route"] == "collection_failed", status
    else:  # the start stays unresolved: a stop, the manifest left at round_trip_started
        assert status[0] == "stopped" and "start_unresolved" in status[1], status
        assert _manifest_of(state)["outcome"] == "round_trip_started"


def _bound_export_case(tmp_path):
    case = next(item for item in run_pilot.load_cases() if item["case_id"] == "cpc-01-cantilever-force")
    sha = current_mbf_sha(case["case_id"])
    config, out = solved_case(tmp_path, case["case_id"], mbf={"sha256": sha}, guest_mbf_sha256=sha)
    return case, config, out


@pytest.mark.parametrize("code", (1, 255, -15))
def test_an_export_reported_as_exported_with_a_nonzero_exit_never_collects(tmp_path, code):
    case, config, out = _bound_export_case(tmp_path)
    commands: list[str] = []
    base = export_runner(out, commands)

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        return answer(argv, code, result.stdout, "") if argv[6].startswith("powershell") else result
    lines: list[str] = []
    assert run_pilot.export_case(case, config, emit=lines.append, export=lambda config_, case_id, sha: (
        harness.run_caepipe_mbf_export(config_, case_id, sha, runner=runner))) is False
    assert not any(command.startswith("copy") for command in commands)  # never collected
    assert not (out / "exported.mbf").exists()
    assert lines == [f"{case['case_id']}: export route=exported_with_nonzero_exit; the guest export did not complete "
                     f"(cases/{case['case_id']}/caepipe_out/run_record.json); no exported MBF"]
    entry = json.loads((out / "run_record.json").read_text(encoding="utf-8"))["mbf_export"]
    assert entry["exit_status"] == code and entry["exported_mbf"]["sha256"] is None


@pytest.mark.parametrize("code", (1, 255, -15))
def test_a_collection_copy_with_a_nonzero_exit_never_gives_an_exported_route(tmp_path, code):
    case, config, out = _bound_export_case(tmp_path)
    base = export_runner(out, [])

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)  # the copy took place and printed its success line
        return answer(argv, code, COPIED, "") if argv[6].startswith("copy") else result
    lines: list[str] = []
    assert run_pilot.export_case(case, config, emit=lines.append, export=lambda config_, case_id, sha: (
        harness.run_caepipe_mbf_export(config_, case_id, sha, runner=runner))) is False
    assert not (out / "exported.mbf").exists()
    assert "export route=mbf_not_collected" in lines[0]


def test_the_held_exports_real_refusal_reaches_the_pilot_as_held(tmp_path):
    case, config, out = _bound_export_case(tmp_path)
    refusal = {"schema": "openpipestress.caepipe_guest_mbf_export.v1", "status": "export_held",
               "diagnostics": ["the export starts the program only inside a won claim and job, which it has not "
                               "adopted"]}
    commands: list[str] = []
    base = export_runner(out, commands, guest=refusal)

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        return answer(argv, 6, result.stdout, "") if argv[6].startswith("powershell") else result
    lines: list[str] = []
    assert run_pilot.export_case(case, config, emit=lines.append, export=lambda config_, case_id, sha: (
        harness.run_caepipe_mbf_export(config_, case_id, sha, runner=runner))) is False
    assert "export route=export_held" in lines[0] and not any(c.startswith("copy") for c in commands)
    entry = json.loads((out / "run_record.json").read_text(encoding="utf-8"))["mbf_export"]
    assert entry["route"] == "export_held" and entry["guest_mod_sha256"] is None



# A refusal the guest reports, with the nonzero exit its script gives it, keeps its own route name at the caller.
REFUSALS = [("no_results_after_batch", 2), ("batch_no_csv", 3), ("print_file_ambiguous", 5)]


@pytest.mark.parametrize("status, code", REFUSALS)
def test_a_guest_refusal_with_its_nonzero_exit_keeps_its_route_name_at_the_pilot(fake_benchmark, monkeypatch, capsys,
                                                                               status, code):
    state = fake_benchmark
    guest, registry = _pilot_guest(state, monkeypatch)
    guest.programs[1] = InventedProgram(status=status, print_name=None, files={"model.res": b"RES"})
    base = guest.runner

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        return answer(argv, code, result.stdout, "") if classify_command(argv[6]) == "attempt" else result
    guest.runner = runner
    status_seen = _collect_one(state, _pilot_round_trip(state, guest))
    assert guest.kinds.count("attempt") == 1 and registry.writes == []
    # Each is a stop under its own route name: batch_no_csv because the GUI fallback is off for the batch, the
    # others because the program has run (completed), which is never a ground for retry.
    assert status_seen[0] == "stopped" and _manifest_of(state)["route"] == status, status_seen
    stored = json.loads((state["config"].exchange_root / "cases" / "cpb-g1-01" / "caepipe_out" / "run_record.json")
                        .read_text(encoding="utf-8"))
    assert stored["route"] == status and stored["exit_status"] == code


# P1-5, the fifth row, found by the author's exit table: a COMPLETED attempt (the program ran) whose reporting raises,
# or whose route carries no CSV, is never retried, and the list stops. Completed is never a ground for retry.
def _completed_run_raising(tmp_path, monkeypatch) -> tuple[BaseException, InventedGuest]:
    config = config_for(tmp_path / "completed", RECONCILE_GUARD)
    with monkeypatch.context() as patch:
        guest = guest_for(config, _invented_clock(patch))
        InventedRegistry(guest, change_on_launch=False)
        real_write = harness.write_private

        def failing_write(root, path, data):
            if Path(path).name == "run_record.json":
                raise OSError("invented failure writing run_record.json")
            return real_write(root, path, data)
        patch.setattr(harness, "write_private", failing_write)
        with pytest.raises(OSError) as raised:
            run_attempt(config, guest)
    assert guest.kinds.count("attempt") == 1 and len(guest.launches) == 1
    return raised.value, guest


@pytest.mark.parametrize("how", ["its reporting raises", "collection fails", "the guest reports no print"])
def test_p1_5_a_completed_attempt_is_never_retried_and_the_list_stops(fake_benchmark, tmp_path, monkeypatch, capsys,
                                                                      how):
    state = fake_benchmark
    if how == "its reporting raises":
        outcome, guest = _completed_run_raising(tmp_path, monkeypatch)
    else:
        guest, _ = _pilot_guest(state, monkeypatch)
        if how == "collection fails":
            base = guest.runner

            def runner(argv, **kwargs):  # every chunk read answers out of form: nothing is collected
                result = base(argv, **kwargs)
                return answer(argv, 1, result.stdout, "") if classify_command(argv[6]) == "chunk" else result
            guest.runner = runner
        else:
            guest.programs[1] = InventedProgram(status="no_results_after_batch", print_name=None,
                                                files={"model.res": b"RES"})
        outcome = _pilot_round_trip(state, guest)
    state["outcomes"][:] = [outcome]
    exit_status = run_pilot.main(state["argv"] + ["--case", "cpb-g1-01", "--case", "cpb-g1-02"], environ={})
    out = capsys.readouterr().out
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01"], out  # never retried; the next model not run
    assert guest.kinds.count("attempt") == 1 and len(guest.launches) == 1  # one dispatch, one start
    assert exit_status == 1 and "retrying" not in out


# A success terminal report beside a nonzero dispatch reply that is neither a transport line nor the session-unavailable
# line: no success route, never retried or compared; partial's permitted actions under its own route name.
def _reply_replaced(guest, code: int, stdout: str | None = None, stderr: str | None = None):
    base = guest.runner

    def runner(argv, **kwargs):
        result = base(argv, **kwargs)
        if classify_command(argv[6]) != "attempt":
            return result
        return answer(argv, code, result.stdout if stdout is None else stdout, "" if stderr is None else stderr)
    guest.runner = runner


@pytest.mark.parametrize("code", (1, 3))
def test_a_success_report_beside_a_nonzero_reply_is_a_contradiction_at_the_pilot(fake_benchmark, monkeypatch, code):
    state = fake_benchmark
    guest, registry = _pilot_guest(state, monkeypatch)
    _reply_replaced(guest, code)  # the script's own report on stdout, with another exit
    status = _collect_one(state, _pilot_round_trip(state, guest))
    assert status[:2] == (run_pilot.PARTIAL, "report_reply_contradiction"), status
    assert guest.kinds.count("attempt") == 1 and registry.writes == [] and "chunk" not in guest.kinds
    out = state["config"].exchange_root / "cases" / "cpb-g1-01" / "caepipe_out"
    record = json.loads((out / "run_record.json").read_text(encoding="utf-8"))
    assert (record["route"], record["start"]["state"], record["exit_status"]) == (
        "report_reply_contradiction", "contradicted", code)
    assert record["observed_csv_path"]["host"] is None and record["outputs_collected"] == []  # never compared
    token = record["attempt_token"]
    report = json.loads((out / token / "guest_log.json").read_text(encoding="utf-8"))
    assert report["status"] == "csv_from_batch" and report["attempt_token"] == token  # the report, kept whole
    assert (out / token / "guest_stdout.txt").read_text(encoding="utf-8") == guest.attempts[0].stdout  # the reply too
    assert (_manifest_of(state)["outcome"], _manifest_of(state)["route"]) == ("partial", "report_reply_contradiction")


@pytest.mark.parametrize("code, line", [(255, harness.TRANSPORT_LINES[0]), (255, harness.TRANSPORT_LINES[1]),
                                        (255, harness.SESSION_UNAVAILABLE_LINE), (1, harness.SESSION_UNAVAILABLE_LINE)],
                         ids=["GetResult", "GetRetCode", "session unavailable, 255", "session unavailable, 1"])
def test_a_lost_reply_beside_a_success_report_leaves_the_report_to_decide(fake_benchmark, monkeypatch, code, line):
    state = fake_benchmark
    guest, registry = _pilot_guest(state, monkeypatch)
    _reply_replaced(guest, code, stdout="", stderr=line)
    status = _collect_one(state, _pilot_round_trip(state, guest))
    assert status[:2] == (run_pilot.COLLECTED, "csv_from_batch"), status
    assert guest.kinds.count("attempt") == 1


def _contradicting(state, monkeypatch):
    guest, _ = _pilot_guest(state, monkeypatch)
    _reply_replaced(guest, 1)
    return guest, _pilot_round_trip(state, guest)


def test_a_second_contradiction_stops_the_list(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    first, first_trip = _contradicting(state, monkeypatch)
    second, second_trip = _contradicting(state, monkeypatch)
    state["outcomes"][:] = [first_trip, second_trip]
    assert run_pilot.main(state["argv"], environ={}) == 1
    out = capsys.readouterr().out
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-02"], out
    assert first.kinds.count("attempt") == 1 and second.kinds.count("attempt") == 1
    assert "a second attempted run that did not agree" in out


def test_a_partial_and_a_contradiction_count_together_for_the_stop_rule(fake_benchmark, monkeypatch, capsys):
    state = fake_benchmark
    partial = {"route": "start_partial", "settings_guard": {"status": "unchanged"}, "guest_mbf_sha256": None,
               "observed_csv_path": {"host": None}}
    _, contradiction = _contradicting(state, monkeypatch)
    state["outcomes"][:] = [partial, contradiction]
    assert run_pilot.main(state["argv"], environ={}) == 1
    assert [call[0] for call in state["calls"]] == ["cpb-g1-01", "cpb-g1-02"]
