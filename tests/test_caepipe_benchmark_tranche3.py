#!/usr/bin/env python3
"""Focused tests for tranche 3 of the CAEPIPE benchmark set (DEL-17-05 REQ-017 to REQ-020, AC-007): the series S1,
S2 and S3, the dynamics family D, the declared addition of documented options records, the screening-class
declarations, and part two, the cases entered by hand on a written base. Invented inputs only; nothing here runs the external program or reads its output.

Tests whose name contains `req005_addition` examine the declared addition of whole options records and the declared
runs built on it. Those runs rely on REQ-005 as placed in execution/PKG-17/DEL-17-05/ScopeOfWork.md and remain
capture-only pending the run-record and comparison work that scope's Not met block names."""

from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import re
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import parse_caepipe_print_csv  # noqa: E402
from core.handoff.caepipe_external.benchmark import build_benchmark_record, screening_class  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from core.handoff.caepipe_mbf.importer import import_caepipe_mbf  # noqa: E402
from tools.caepipe_correlation import benchmark_hand_entry as hand, benchmark_models, benchmark_tranche3 as t3  # noqa: E402,E501
from tests.test_caepipe_benchmark import mirror_csv, solve  # noqa: E402

BENCHMARK = ROOT / "validation" / "caepipe_benchmark"
MANIFEST = BENCHMARK / "manifest.json"
# The model files of tranches 1 and 2 (82), as tracked before tranche 3: sha256 over each manifest file path, a NUL
# and the file's bytes, in manifest order. Tranche 3 changes none of them.
EARLIER_TRANCHES_DIGEST = "65cb7c01198966ad4ab65b459bd9b3e98e96f0c0cbc8f8cd3506d363e0fc4a14"


def manifest() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def documents(tranche: str = "3") -> list[dict]:
    return [json.loads((BENCHMARK / entry["file"]).read_text(encoding="utf-8"))
            for entry in manifest()["models"] if entry["tranche"] == tranche]


def by_id() -> dict[str, dict]:
    return {document["model_id"]: document for document in documents()}


def writer(document: dict):
    return render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                 reference_temperature_c=document["reference_temperature_c"],
                                 piping_code=document["piping_code"])


# -- the set -----------------------------------------------------------------------------------------------------------
def test_the_earlier_tranches_keep_their_bytes_and_tranche_3_is_appended():
    entries = manifest()["models"]
    digest = hashlib.sha256()
    for entry in entries:
        if entry["tranche"] in ("1", "2"):
            digest.update(entry["file"].encode() + b"\0" + (BENCHMARK / entry["file"]).read_bytes())
    assert digest.hexdigest() == EARLIER_TRANCHES_DIGEST
    assert [entry["tranche"] for entry in entries] == ["1"] * 60 + ["2"] * 22 + ["3"] * 27
    families: dict[str, list[str]] = {}
    for entry in entries[82:]:
        families.setdefault(entry["family"], []).append(entry["model_id"])
    assert families == {
        "S1": [f"cpb-s1-{i:02d}" for i in range(1, 5)], "S2": [f"cpb-s2-{i:02d}" for i in range(1, 4)],
        "S3": [f"cpb-s3-{i:02d}" for i in range(1, 6)],
        "DYN-C": ["cpb-dyn-c-n08-flat", "cpb-dyn-c-n02-flat", "cpb-dyn-c-n16-flat", "cpb-dyn-c-n08-trunc",
                  "cpb-dyn-c-n08-two", "cpb-dyn-c-n04-flat", "cpb-dyn-c-n01-flat"],
        "DYN-D": ["cpb-dyn-d-n08-flat"], "DYN-M": ["cpb-dyn-m-flat"], "DYN-L": ["cpb-dyn-l-flat"],
        "N": ["cpb-n-01", "cpb-n-02", "cpb-n-03", "cpb-n-04"], "H": ["cpb-h-01"]}
    assert [entry["rank"] for entry in entries[94:104]] == list(range(1, 11))
    assert sorted(manifest()["tranches"]) == ["1", "2", "3"]


def test_every_tranche_3_model_is_invented_and_authors_metres_and_its_own_weight():
    for document in documents():
        model = document["model"]
        assert document["provenance"]["origin"] == "invented" and document["provenance"]["produced_by_caepipe"] is False
        assert document["generator"] == {"version": benchmark_models.GENERATOR_VERSION,
                                         "seed": benchmark_models.DEFAULT_SEED}
        for pipe in model["pipe_segments"]:
            assert pipe["section"]["outside_diameter"]["unit"] == "m" and pipe["section"]["wall_thickness"]["unit"] == "m"
        weights = [load for case in model["load_cases"] for load in case["primitive_loads"] if load["category"] == "weight"]
        for load in weights:  # authored per-length loads, -rho A g from the section
            pipe = next(p for p in model["pipe_segments"] if p["id"] == load["target"]["pipe"])
            outer, wall = pipe["section"]["outside_diameter"]["value"], pipe["section"]["wall_thickness"]["value"]
            # The designed series and cases use the designed material; the dynamics designs state their own density.
            rho = designed.DESIGNED_MATERIAL[4] if pipe["material"] == designed.DESIGNED_MATERIAL[0] else t3.RHO
            expected = -rho * math.pi / 4.0 * (outer ** 2 - (outer - 2 * wall) ** 2) * 9.80665
            assert load["magnitude"]["unit"] == "N/m"
            assert load["magnitude"]["value"] == pytest.approx(expected, rel=1e-14)
        # The written model holds linear supports only; part two's members are added by hand_entry_model.
        assert not [s for s in model["supports"] if s.get("family") not in (None, "anchor", "guide", "spring")]
        assert "model_settings" not in model


def test_every_tranche_3_model_is_written_within_the_row_limit_and_imports_back_bound():
    entries = {entry["model_id"]: entry for entry in manifest()["models"]}
    for document in documents():
        result = writer(document)
        assert result.status == "written", (document["model_id"], result.diagnostics)
        assert result.layout_row_count <= 20 and result.layout_row_count == entries[document["model_id"]]["layout_row_count"]
        data = result.text.encode("ascii")
        assert hashlib.sha256(data).hexdigest() == entries[document["model_id"]]["mbf_sha256"]
        back = import_caepipe_mbf(data, sidecar=result.sidecar(hashlib.sha256(data).hexdigest()), source_label="t3",
                                  material_provenance={"*": {"origin": "project_invented", "source_name": "t3"}})
        assert back.status == "imported" and back.source["binding"] == "bound", document["model_id"]
        for key in ("nodes", "pipe_segments", "supports", "load_cases"):
            assert sorted(item["id"] for item in back.model[key]) == sorted(item["id"] for item in document["model"][key])


def _without(model: dict, drop) -> str:
    """The model's JSON with the swept fields blanked (by `drop`), so that members of one series compare equal."""
    return drop(json.loads(json.dumps(model)))


def test_series_members_differ_only_in_their_swept_parameter():
    docs = by_id()

    def walls(model):
        for pipe in model["pipe_segments"]:
            pipe["section"]["wall_thickness"]["value"] = None
        for case in model["load_cases"]:
            for load in case["primitive_loads"]:
                if load["category"] == "weight":
                    load["magnitude"]["value"] = None  # follows the wall
        model["project"] = None
        return json.dumps(model, sort_keys=True)

    def lengths(model):
        for node in model["nodes"]:
            node["position"]["x"] = None
        model["project"] = None
        return json.dumps(model, sort_keys=True)

    for family, count, drop in (("s1", 4, walls), ("s2", 3, walls), ("s3", 5, lengths)):
        forms = {_without(docs[f"cpb-{family}-{i:02d}"]["model"], drop) for i in range(1, count + 1)}
        assert len(forms) == 1, family
    series = manifest()["series"]
    assert [m["value"] for m in series["shear-thick-wall"]["members"]] == [4.0, 6.0, 10.0, 20.0]
    assert [m["value"] for m in series["bend-shear-axial-wall"]["members"]] == [6.0, 15.0, 40.0]
    assert [m["value"] for m in series["rigid-support-length"]["members"]] == [0.5, 1.0, 2.5, 10.0, 40.0]


# -- closed forms and predictions --------------------------------------------------------------------------------------
def test_cowper_coefficient_and_the_s1_prediction():
    # Cowper's coefficient tends to 6(1+nu)/(7+6nu) for a solid circle and to 2(1+nu)/(4+3nu) for a thin ring.
    assert t3.cowper_kappa(100.0, 50.0, 0.3) == pytest.approx(6 * 1.3 / (7 + 1.8), rel=1e-15)
    assert t3.cowper_kappa(100.0, 1e-9, 0.3) == pytest.approx(2 * 1.3 / (4 + 0.9), rel=1e-9)
    previous = 0.0
    for document in (by_id()[f"cpb-s1-{i:02d}"] for i in range(1, 5)):
        [prediction] = document["predictions"]
        values = prediction["values"]
        params = document["parameters"]
        area, inertia = benchmark_models.section_properties(params["od"], params["wall"])
        e, nu = 200.0e9, 0.30
        g = e / (2 * (1 + nu))
        # The tip-force part of the shear fraction is 3 E I/(kappa G A L^2); weight shifts it slightly.
        fraction = 3 * e * inertia / (prediction["shear_coefficient"] * g * area * params["span"] ** 2)
        assert abs(prediction["relative_difference"]["timoshenko"]) == pytest.approx(fraction, rel=0.02)
        assert values["timoshenko"] < values["euler_bernoulli"] < 0  # shear adds to a downward deflection
        # The predicted shear fraction grows as the wall thins (module text), and is visible on every member.
        assert abs(prediction["relative_difference"]["timoshenko"]) > max(previous, 5.0e-2)
        previous = abs(prediction["relative_difference"]["timoshenko"])


def test_the_s2_castigliano_solution_is_in_equilibrium_and_reduces_correctly():
    for document in (by_id()[f"cpb-s2-{i:02d}"] for i in range(1, 4)):
        p = document["parameters"]
        args = dict(a=p["tangent"], b_len=p["tangent"], radius=p["bend_radius"], e=200.0e9, nu=0.3, alpha=12.0e-6,
                    delta_t=p["delta_t"], od_mm=p["od"], wall_mm=p["wall"], k=1.0)
        for axial, shear in ((False, False), (True, False), (True, True)):
            r = t3.castigliano_l(**args, axial=axial, shear=shear)
            assert r["A_Fx"] == pytest.approx(-r["B_Fx"], rel=1e-15) and r["A_Fz"] == pytest.approx(-r["B_Fz"], rel=1e-15)
            # Moment equilibrium about A: M_A + M_B + (r_B - r_A) x F_B = 0 (global Y component).
            dx, dz = p["tangent"] + p["bend_radius"], p["bend_radius"] + p["tangent"]
            assert r["A_My"] + r["B_My"] + (dz * r["B_Fx"] - dx * r["B_Fz"]) == pytest.approx(0.0, abs=1e-6 * abs(r["A_Fx"]))
            assert r["A_Fx"] > 0 and r["A_Fz"] > 0  # the anchors push the growing pipe back
        # Every added compliance lowers the thermal reactions.
        b, ba, bas = (t3.castigliano_l(**args, axial=x, shear=y)["A_Fx"] for x, y in
                      ((False, False), (True, False), (True, True)))
        springs = t3.castigliano_l(**args, axial=True, shear=False, support=(t3.K_TRANSLATION, t3.K_ROTATION))["A_Fx"]
        assert b > ba > bas and ba > springs
        # A very stiff support is the ideal anchor.
        assert t3.castigliano_l(**args, axial=True, shear=False, support=(1e30, 1e30))["A_Fx"] == pytest.approx(ba, rel=1e-12)


def test_the_s3_prediction_and_the_stated_stiffness():
    assert t3.K_TRANSLATION == pytest.approx(1e12 * 4.4482216152605 / 0.0254, rel=1e-15)
    assert t3.K_ROTATION == pytest.approx(1e12 * 4.4482216152605 * 0.0254, rel=1e-15)
    rigid = manifest()["series"]["rigid-support-length"]["rigid_support"]
    assert "docs.sstcae.com/tech_manual/anchor.htm" in rigid["source"] and "2026-11-12" in rigid["source"]
    reductions = []
    for document in (by_id()[f"cpb-s3-{i:02d}"] for i in range(1, 6)):
        axial = next(p for p in document["predictions"] if p["load_case"] == "case:expansion")
        ratio = axial["ratio_2EA_over_ktL"]
        assert axial["relative_difference"] == pytest.approx(-ratio / (1 + ratio), rel=1e-12)
        reductions.append(-axial["relative_difference"])
    assert reductions == sorted(reductions, reverse=True)
    assert reductions[0] > 2e-4 and reductions[1] > 2e-4 and reductions[-1] < 1e-5  # clearly above 1e-4 down to negligible
    text = manifest()["series"]["rigid-support-length"]["description"]
    assert "DECLARED BEFORE ANY RUN" in text and "conditioning" in text and "nothing about agreement" in text


S_MODELS = [f"cpb-s1-{i:02d}" for i in range(1, 5)] + [f"cpb-s2-{i:02d}" for i in range(1, 4)] + [
    f"cpb-s3-{i:02d}" for i in range(1, 6)]
def published_checks_passed(payload: dict) -> bool:
    """Whether the product publishes every case of this solve as checks passed, read from the solve itself; nothing
    tracked records which cases it does."""
    cases = payload["mechanics_envelope"]["numerical_quality"]["cases"]
    return bool(cases) and all(case["solve_quality"] == "checks_passed" for case in cases)


def test_s_series_solve_and_their_closed_forms_match_swbpipe():
    """The closed forms of SWBPIPE's own formulation (Euler-Bernoulli members with axial strain, ideal anchors) against
    its solve through the headless runner, at the record's closed-form tolerance. A model is checked only where the
    product publishes every case of its solve as checks passed (read at the solve); the test requires all twelve to
    solve and to be checked, and counts the rows, so a solved model that is not eligible fails it by name."""
    docs = by_id()
    solved, not_eligible, checked_rows = [], [], 0
    for model_id in S_MODELS:
        document = docs[model_id]
        payload = solve(document["model"])  # asserts MECHANICS_SOLVED
        solved.append(model_id)
        if not published_checks_passed(payload):
            not_eligible.append(model_id)
            continue
        mbf, text = mirror_csv(document, payload)
        record = build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf.text,
                                        parsed_caepipe=parse_caepipe_print_csv(text))
        rows = [row for row in record["results"] if "closed_form" in row]
        assert len(rows) == len(document["closed_form"]), model_id
        for row in rows:
            assert row["closed_form"]["closed_form_vs_swbpipe"] == "agree", (model_id, row)
        checked_rows += len(rows)
    assert solved == S_MODELS
    assert not_eligible == [], f"solved, not checked (a case not published as checks passed): {not_eligible}"
    assert checked_rows == sum(len(docs[model]["closed_form"]) for model in S_MODELS)


# -- family D ----------------------------------------------------------------------------------------------------------
def test_the_dynamics_cases_keep_their_wrapper_declarations():
    quotes = json.loads(t3.QUOTES.read_text(encoding="utf-8"))["models"]
    for document in documents():
        if not document["family"].startswith("DYN"):
            continue
        assert document["reference_values"] == quotes[document["model_id"]]["reference_values"] if \
            "reference_values" in quotes[document["model_id"]] else "reference_values" not in document
        assert document["inexact_inputs"]["dynamic"]  # as the wrapper states
        assert document["screening_classes"]["modal"] == "modal"
        assert document["screening_classes"]["response_spectrum"] == "response_spectrum"
        assert document["comparison_conventions"][0].startswith("relative deviation = (value - continuum) / |continuum|")
        if document["family"] in ("DYN-L", "DYN-M"):
            assert document["third_value"].startswith("NONE.") and document["mode_count_basis"]
        else:
            assert "pull request 268, not yet on main" in json.dumps(document)
    diagonal = by_id()["cpb-dyn-d-n08-flat"]
    rules = diagonal["discrete_reference_values"]["combined_magnitude_by_modal_rule"]
    assert rules["srss"]["displacement node:B y"] == rules["srss"]["displacement node:B z"]
    assert rules["cqc"]["displacement node:B y"] != rules["cqc"]["displacement node:B z"]
    assert any("member axes are outside the like-for-like set" in item for item in diagonal["comparison_conventions"])


def test_the_continuum_frequencies_are_the_textbook_roots():
    cantilever, propped = t3.eb_frequencies("cantilever"), t3.eb_frequencies("propped")
    ratio = cantilever[1] / cantilever[0]
    assert ratio == pytest.approx((4.694091132974175 / 1.875104068711961) ** 2, rel=1e-14)
    assert propped[0] / cantilever[0] == pytest.approx((3.926602312047919 / 1.875104068711961) ** 2, rel=1e-14)


REFERENCE_PACK = ROOT / "validation" / "references" / "f2_spectrum" / "benchmark_pack" / "pack_summary.json"
DISCRETE_REFERENCE = ROOT / "validation" / "references" / "f2_spectrum" / "discrete" / "c9_pipe_diagonal.json"


def test_quoted_reference_values_equal_the_reference_package_where_present():
    if not REFERENCE_PACK.is_file():
        pytest.skip("validation/references/f2_spectrum/benchmark_pack/pack_summary.json is absent (pull request 268, "
                    "not on main)")
    rows = json.loads(REFERENCE_PACK.read_text(encoding="utf-8"))["rows"]
    for document in documents():
        if "reference_values" not in document:
            continue
        expected = []
        for row in rows:
            if row["case"] == document["model_id"]:
                item = {k: v for k, v in row.items() if k not in ("case", "continuum", "consistent", "lumped")}
                for key in ("continuum", "consistent", "lumped"):
                    item[key] = None if row[key] is None else float(row[key][1])
                expected.append(item)
        assert document["reference_values"] == expected, document["model_id"]


def test_the_quoted_discrete_reference_is_the_reference_file_where_present():
    if not DISCRETE_REFERENCE.is_file():
        pytest.skip("validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json is absent (pull request 268, "
                    "not on main)")
    quoted = by_id()["cpb-dyn-d-n08-flat"]["discrete_reference_values"]
    assert hashlib.sha256(DISCRETE_REFERENCE.read_bytes()).hexdigest() == quoted["source_sha256"]


# -- the declared addition (depends on the proposed REQ-005 sentence) -------------------------------------------------
def dynamics_entries() -> list[dict]:
    return [entry for entry in manifest()["models"] if entry["family"].startswith("DYN")]


def test_req005_addition_runs_per_case():
    counts = {entry["rank"]: len(entry["runs"]) for entry in dynamics_entries()}
    # Each run in two forms, but cpb-dyn-c-n08-two (rank 6), declared in the typed form only.
    assert counts == {1: 16, 2: 16, 3: 16, 4: 2, 5: 8, 6: 1, 7: 2, 8: 2, 9: 2, 10: 2}
    orders = []
    for entry in dynamics_entries():
        runs = {run["run_id"]: run for run in entry["runs"]}
        assert len({run["series"] for run in entry["runs"]}) == len(runs)  # one run per model per series
        for run in entry["runs"]:
            assert re.fullmatch(r"[a-z0-9-]+", run["label"]) and run["kind"] == "option_addition"
            assert run["screened"] is False and run["depends_on"] == t3.REQ005_DEPENDENCE
            assert "as placed" in run["depends_on"] and "capture-only" in run["depends_on"]
            assert "not placed" not in run["depends_on"] and "not admitted" not in run["depends_on"]
            assert run["mbf"]["bound_file"] is False and run["mbf"]["writer_output"] is False
            assert run["mbf"]["writer_sha256"] == entry["mbf_sha256"]
            if run["single_form"]:
                assert entry["model_id"] == "cpb-dyn-c-n08-two" and run["form"] == "table_typed"
                assert run["counterpart_run_id"] is None
            else:
                other = runs[run["counterpart_run_id"]]
                assert other["counterpart_run_id"] == run["run_id"] and {run["form"], other["form"]} == {
                    "table_added", "table_typed"}
                assert other["strike_order"] == run["strike_order"]
                assert other["series"] == (run["series"] + "-table-typed" if run["form"] == "table_added"
                                           else run["series"][:-len("-table-typed")])
            ms, ds = t3.MODE_SUMS[run["spectrum_mode_sum"]][0], t3.DIRECTION_SUMS[run["spectrum_direction_sum"]][0]
            base = "dynamics-comparable" if run["comparable"] else f"dynamics-{ms}-{ds}"
            assert run["series"] == base + ("-table-typed" if run["form"] == "table_typed" else "")
            assert run["comparable"] is ((ms, ds) == ("srss", "srss"))
            if run["spectrum_mode_sum"] == 4:
                assert run["swbpipe_counterpart"] is None  # NRL: no SWBPIPE rule; captured only
            text = run["window_instruction"]["text"]
            assert hashlib.sha256(text.encode("utf-8")).hexdigest() == run["window_instruction"]["sha256"]
            kind = "dynamics" if run["single_form"] else "pair"
            assert f"Import {run['run_id']}.mbf" in text and text.count(hand.skip_rule(kind)) == 1
            # The skip rule is the numbered step after the print steps, and the last one unless the export follows.
            steps, _, _ = _parts(text)
            rule = next(i for i, line in enumerate(steps) if line.endswith(hand.skip_rule(kind)))
            assert _body(steps[rule - 1]) == f"Save the .csv file as {run['run_id']}.print.csv. [{hand.PRINT_PAGE}]"
            assert (rule == len(steps) - 1) is ("export" not in run)
            assert text.count(hand.save_sentence(run["run_id"])) == 1 and "never saved over" not in text
            assert "File > Analyze (F12)" in text and hand.ANALYZE_QUOTE in text
            assert "states the file's cutoff frequency value with no unit" in text
            assert "a value that differs is set to the stated value by hand and noted on the run line" in text
            assert text.index("Cutoff Frequency reads") < text.index("File > Analyze (F12)")
            assert text.index(f"{run['run_id']}.mod") < text.index("File > Analyze (F12)") < text.index(
                f"{run['run_id']}.print.csv")
            # One action per step: the print sequence in four steps; the cutoff and the number of modes apart.
            bodies = [_body(line) for line in steps]
            for action in ("In the Results window, open Print: File > Print (Ctrl+P).", "Click All.", "Click To File.",
                           f"Save the .csv file as {run['run_id']}.print.csv."):
                assert sum(body.startswith(action + " [") for body in bodies) == 1, action
            [cutoff] = [body for body in bodies if "Cutoff Frequency reads" in body]
            [modes] = [body for body in bodies if "Number of Modes reads" in body]
            assert "Number of Modes" not in cutoff and "Cutoff Frequency" not in modes
            assert run["expected_saved_file"] == f"{run['run_id']}.mod"
            assert ("agreed" in text.split("\n")[0]) is (run["form"] == "table_added")
            orders.append((run["strike_order"], entry["rank"], run["comparable"]))
    orders = sorted(set(orders))
    assert [order for order, _, _ in orders] == list(range(1, 35))
    assert [rank for _, rank, comparable in orders[:10]] == list(range(1, 11)) and all(c for *_, c in orders[:10])
    assert {rank for _, rank, _ in orders[10:31]} == {1, 2, 3} and {rank for _, rank, _ in orders[31:]} == {5}
    assert "instruction only" in manifest()["option_addition"]["same_session_alternative"]
    dry_run = manifest()["dry_run"]
    assert [item[:5] for item in dry_run["checks"]] == ["(i) c", "(ii) ", "(iii)", "(iv) ", "(v) o"]
    assert len(dry_run["consequences"]) == 4 and "observed at the dry run" in dry_run["save_command"]


def test_req005_addition_records_are_documented_and_whole_and_change_nothing_else():
    docs = by_id()
    for entry in dynamics_entries():
        document = docs[entry["model_id"]]
        written = writer(document).text
        for run in entry["runs"]:
            records = run["mbf"]["added_records"]
            expected = ["OPMISC", "SPCMS"] if run["form"] == "table_added" else ["OPMISC"]
            assert [record["heading"] for record in records] == expected
            text = t3.added_mbf_text(written, records)
            assert hashlib.sha256(text.encode("utf-8")).hexdigest() == run["mbf"]["sha256"]
            # (c) Removing the added lines gives the writer's file back byte for byte; nothing else moved.
            added = [line for record in records for line in record["lines"]]
            lines = text.split("\r\n")
            kept = list(lines)
            for record in records:
                start = kept.index(record["heading"])
                assert kept[start:start + len(record["lines"])] == record["lines"]
                del kept[start:start + len(record["lines"])]
            assert "\r\n".join(kept) == written and len(lines) == len(written.split("\r\n")) + len(added)
            # Placed where the format page orders them: OPMISC with the options records before MATERIAL, SPCMS after
            # LOADS and before LAYOUT.
            assert lines[lines.index("MATERIAL") - 2] == "OPMISC"
            opmisc = records[0]["lines"][1].split(",")
            settings = document["dynamic_request"]["other_program_settings"]
            assert len(opmisc) == 9 and float(opmisc[0]) == settings["cutoff_frequency_hz"]
            assert int(opmisc[1]) == settings["number_of_modes"] and int(opmisc[2]) == settings["include_missing_mass"]
            assert (int(opmisc[5]), int(opmisc[6])) == (run["spectrum_mode_sum"], run["spectrum_direction_sum"])
            assert float(opmisc[8]) == 9.80665
            [table] = document["dynamic_request"]["spectra"]
            window = run["window_instruction"]["text"]
            if run["form"] == "table_added":
                assert lines.index("LOADS") < lines.index("SPCMS") < lines.index("LAYOUT")
                header, *points = records[1]["lines"][1:]
                assert header.split(",")[1:] == [str(len(table["points"])), "0", "0", "0", "3"]
                assert [(float(f), float(a)) for _, f, a in (p.split(",") for p in points)] == [
                    (f, a * 1000.0) for f, a in table["points"]]
            else:
                assert "SPCMS" not in lines
                # Form B: the person types the table; each row's values, units and the axis settings are stated.
                rows = "; ".join(f"{t3._num(f)}, {t3._num(a * 1000.0)}" for f, a in table["points"])
                assert rows in window and "Options > Spectrum" in window and "Frequency (Hz)" in window
                assert "Acceleration (mm/sec2)" in window and "docs.sstcae.com/users_manual/spectrums.htm" in window
            # (b) The resulting file is not the bound file: with the writer's sidecar it does not import bound.
            sidecar = writer(document).sidecar(hashlib.sha256(written.encode("ascii")).hexdigest())
            back = import_caepipe_mbf(text.encode("ascii"), sidecar=sidecar, source_label="addition",
                                      material_provenance={"*": {"origin": "project_invented", "source_name": "t3"}})
            assert back.source.get("binding") != "bound"
            # (d) What no documented record carries is entered in the window, in words.
            assert "Loads > Spectrum" in window and "Gravitational Acceleration and Direction" in window
            assert "check the box next to Response Spectrum" in window
            assert "docs.sstcae.com/users_manual/load_cases.htm" in window


def test_req005_addition_pairs_differ_in_the_spectrum_table_only():
    """For every pair, the two files differ in the spectrum table's lines and nothing else; form B's file differs from
    the writer's output in the options record and nothing else."""
    docs = by_id()
    pairs = 0
    for entry in dynamics_entries():
        written = writer(docs[entry["model_id"]]).text
        runs = {run["run_id"]: run for run in entry["runs"]}
        for run in entry["runs"]:
            if run["form"] != "table_added":
                continue  # each pair once, from its form A (the single-form case has none)
            typed = runs[run["counterpart_run_id"]]
            file_a = t3.added_mbf_text(written, run["mbf"]["added_records"]).split("\r\n")
            file_b = t3.added_mbf_text(written, typed["mbf"]["added_records"]).split("\r\n")
            spcms = run["mbf"]["added_records"][1]["lines"]
            start = file_a.index("SPCMS")
            assert file_a[start:start + len(spcms)] == spcms
            assert file_a[:start] + file_a[start + len(spcms):] == file_b
            opmisc = typed["mbf"]["added_records"][0]["lines"]
            at = file_b.index("OPMISC")
            assert file_b[at:at + len(opmisc)] == opmisc and file_b[:at] + file_b[at + len(opmisc):] == written.split(
                "\r\n")
            assert run["mbf"]["sha256"] != typed["mbf"]["sha256"]
            pairs += 1
    assert pairs == 33
    assert "never both screened" in manifest()["option_addition"]["pairs"]


def test_req005_addition_refuses_any_other_record_or_form():
    document = by_id()["cpb-dyn-c-n08-flat"]
    written = writer(document).text
    good = [{"heading": "OPMISC", "lines": ["OPMISC", "900,6,0,1,1,2,2,1,9.80665"]}]
    assert t3.added_mbf_text(written, good) != written
    for records in (
        [{"heading": "LAYOUT", "lines": ["LAYOUT", "T999,X1"]}],  # (a) a layout record
        [{"heading": "OPPRES", "lines": ["OPPRES", "0,1,0,1,1"]}],  # not an addable record
        [{"heading": "SPLVL", "lines": ["SPLVL", "L1,0,0,0,3"]}],
        [{"heading": "SEISMIC", "lines": ["SEISMIC", "0,1,0"]}],
        good + good,  # added twice
        [{"heading": "OPMISC", "lines": ["OPMISC"]}],  # no data line
        [{"heading": "OPMISC", "lines": ["OPMISC", "900\r\nLAYOUT"]}],  # a line break inside
        [{"heading": "OPMISC", "lines": ["OPTEMP", "20"]}],  # heading mismatch
    ):
        with pytest.raises(ValueError):
            t3.added_mbf_text(written, records)
    with pytest.raises(ValueError, match="already in the file"):
        t3.added_mbf_text(t3.added_mbf_text(written, good), good)


# -- screening classes -------------------------------------------------------------------------------------------------
def test_every_case_and_analysis_declares_its_screening_class_by_the_rule():
    rule = manifest()["screening_class_rule"]
    assert "response_spectrum" in rule["classes"] and "stands only once" in rule["classes"]["response_spectrum"]
    entries = {entry["model_id"]: entry for entry in manifest()["models"]}
    for tranche in ("1", "2", "3"):
        for document in documents(tranche):
            entry = entries[document["model_id"]]
            if tranche == "3":
                declared = document["screening_classes"]
                assert entry["screening_classes"] == declared
                nonlinear = document["family"] == "N"  # a support whose status can change
                for case, cls in declared["static"].items():
                    expected = "nonlinear" if nonlinear else screening_class(document["inexact_inputs"][case])
                    assert cls == expected, (document["model_id"], case)
                cases = {case["id"] for case in document["model"]["load_cases"]} | {"operating"}
                assert set(declared["static"]) == cases
            else:
                assert entry["screening_classes"] == t3.derived_screening_classes(document)
                assert "screening_classes" not in document  # the files are unchanged


# -- part two: cases entered by hand -----------------------------------------------------------------------------------

HAND_IDS = ["cpb-n-01", "cpb-n-02", "cpb-n-03", "cpb-n-04", "cpb-h-01"]


def hand_documents() -> list[dict]:
    docs = by_id()
    return [docs[model_id] for model_id in HAND_IDS]


def test_hand_entry_cases_are_ranked_and_their_bases_are_written():
    entries = {entry["model_id"]: entry for entry in manifest()["models"]}
    assert [document["hand_entry_rank"] for document in hand_documents()] == [1, 2, 3, 4, 5]
    assert len(HAND_IDS) <= 10
    for document in hand_documents():
        base = writer(document)
        assert base.status == "written" and base.layout_row_count <= 20, document["model_id"]
        assert hashlib.sha256(base.text.encode("utf-8")).hexdigest() == entries[document["model_id"]]["mbf_sha256"]
        [run] = entries[document["model_id"]]["runs"]
        assert run["mbf"]["sha256"] == entries[document["model_id"]]["mbf_sha256"] and run["screened"] is False
        assert run["swbpipe_model_hash"] == benchmark_models.canonical_model_hash(hand.hand_entry_model(document))
        # The model with the member is the base with exactly the member's support records added, and the writer
        # refuses that model (which is why the member is entered by hand).
        full = hand.hand_entry_model(document)
        member = document["hand_entry"]["member"]
        assert {k: v for k, v in full.items() if k != "supports"} == {k: v for k, v in document["model"].items()
                                                                     if k != "supports"}
        assert full["supports"] == document["model"]["supports"] + member and len(member) == 1
        refused = render_caepipe_mbf_si(full, title=document["model_id"], reference_temperature_c=20.0,
                                        piping_code=document["piping_code"])
        assert refused.status != "written"
        assert "MBF-SI-SUPPORT-UNSUPPORTED" in {item["code"] for item in refused.diagnostics}


def test_each_hand_entry_run_is_one_change_with_stated_fields_and_a_save():
    labels = set()
    for document in hand_documents():
        run = document["hand_entry"]["run"]
        assert run["kind"] == "hand_entry" and re.fullmatch(r"[a-z0-9-]+", run["label"]) and run["label"] not in labels
        labels.add(run["label"])
        text = run["window_instruction"]["text"]
        assert hashlib.sha256(text.encode("utf-8")).hexdigest() == run["window_instruction"]["sha256"]
        run_id = run["run_id"]
        assert run_id == f"{document['model_id']}--{run['label']}" and run["expected_saved_file"] == f"{run_id}.mod"
        assert text.startswith(f"1. Import {run_id}.mbf")
        assert text.count(hand.save_sentence(run_id)) == 1 and "never saved over" not in text
        assert "File > Analyze (F12)" in text and hand.ANALYZE_QUOTE in text
        assert text.index(f"{run_id}.mod") < text.index("File > Analyze (F12)") < text.index(f"{run_id}.print.csv")
        steps, closing, _ = _parts(text)
        assert closing is None and text.count(hand.skip_rule("hand")) == 1
        for action in ("In the Results window, open Print: File > Print (Ctrl+P).", "Click All.", "Click To File.",
                       f"Save the .csv file as {run_id}.print.csv."):
            assert sum(_body(line).startswith(action + " [") for line in steps) == 1, action
        assert text.count(" enter a ") == 1  # one member
        for field in run["fields"]:
            assert field["unit"] and field["sign_convention"] and field["short"]
            assert field["page"].startswith("https://docs.sstcae.com/tech_manual/") and "2026-11-12" in field["page"]
            assert f"{field['field']}: {field['value']} ({field['short']})" in text
            # No field carries a value whose unit a saved page does not state: only zero, blank, unitless entries.
            assert field["value"] in ("0", "1", "blank", "pressed", "Rigid (the default)"), field
        assert 1 <= run["fields_entered_estimate"] <= 6
        held = document["hand_entry"]["held"]
        assert set(held) == set(document["screening_classes"]["static"]) and all("HELD" in item for item in held.values())


def test_hand_entry_screening_classes_and_one_way_declarations():
    one_way = ("a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be "
               "compared is read from the product at each solve and is recorded nowhere here.")
    for document in hand_documents():
        classes = document["screening_classes"]["static"]
        assert "gap_case" not in document["hand_entry"]
        if document["family"] == "N":
            assert set(classes.values()) == {"nonlinear"} and document["hand_entry"]["one_way_case"] == one_way
            assert set(document["hand_entry"]["held"].values()) == {hand.HELD}
            assert document["hand_entry"]["expected_status"]
        else:
            assert classes == {case: screening_class(document["inexact_inputs"][case]) for case in classes}
            assert "one_way_case" not in document["hand_entry"]
            # The constant support's load is an input not held equal, named; its class is named inexact.
            assert "constant_support_load" in document["inexact_inputs"]["case:sustained"]
            assert set(classes.values()) == {"named_inexact"}
            assert "never adjusted to what comes back" in document["description"]
            [member] = document["hand_entry"]["member"]
            assert member["restraints"] == ["UY"] and member["hanger"]["constant_load"]["value"] > 0
            # REQ-019: no one-parameter series carries constant_support_load, so every case's hold waits for an
            # independently justified one as well as for the product's own result and the capture's binding.
            for held in document["hand_entry"]["held"].values():
                assert held == hand.HELD_UNTIL_SERIES and "constant_support_load" in held and "REQ-019" in held
                assert "independently justified one-parameter series" in held and "never fitted" in held
                assert "binding to the model actually run" in held and "not compared" in held
            [run] = [entry for entry in manifest()["models"] if entry["model_id"] == document["model_id"]][0]["runs"]
            assert hand.SERIES_WAIT in run["screening_waits_for"]
    assert manifest()["hand_entry"]["one_way_cases"] == one_way
    left_out = manifest()["hand_entry"]["left_out"]
    assert {item["case"].split(" ")[1] for item in left_out} >= {"limit", "variable", "uniform", "second"}


def test_hand_entry_models_on_the_product_today():
    """Each model that holds its member, solved through the headless runner. Whether its result may be compared is
    read from the product at the solve and recorded nowhere: where every case is published as checks passed and the
    member is consumed by the solve, the third values that apply to SWBPIPE's formulation (Euler-Bernoulli, ideal
    supports, the stated status) agree with its solve."""
    for document in hand_documents():
        run = hand.hand_entry_document(document)
        payload = solve(run["model"])
        envelope = payload["mechanics_envelope"]
        assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED", document["model_id"]
        warnings = {item.get("code") for item in envelope["diagnostics"] if item.get("severity") == "warning"}
        if not published_checks_passed(payload) or "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED" in warnings:
            continue
        status = {row["entity_ref"]: row["value"] for row in envelope["results"]
                  if row.get("metadata", {}).get("component") == "active_set_state_code"}
        for support, by_case in document["hand_entry"]["expected_status"].items():
            for case, state in by_case.items():
                if case != "operating":
                    assert status[support] == (1.0 if state == "closed" else 0.0), (document["model_id"], support)
        for form in run["closed_form"]:
            value = _swbpipe_value(envelope["results"], form)
            assert value == pytest.approx(form["value"], rel=1e-6, abs=1e-6), (document["model_id"], form)


_KINDS = {"reaction_force": ("support_reaction_component_v2", "F"), "reaction_moment": ("support_reaction_component_v2", "M")}


def _swbpipe_value(results: list[dict], form: dict) -> float:
    case = form["load_case"]
    if form["quantity"] == "displacement":
        [row] = [r for r in results if r.get("kind") == f"global_nodal_displacement_{form['component']}"
                 and r.get("entity_ref") == form["node"] and r["basis_ref"]["ref_id"] == case]
        return row["value"] / 1000.0 if row["unit"] == "mm" else row["value"]
    kind, letter = _KINDS[form["quantity"]]
    [row] = [r for r in results if r.get("kind") == kind and r.get("entity_ref") == form["support"]
             and r["basis_ref"]["ref_id"] == case and r["metadata"]["component"] == letter + form["component"]]
    return row["value"]


# -- the connecting path: main's loader and the one render -------------------------------------------------------------
from core.handoff.caepipe_external.harness import CASE_ID_PATTERN  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402


def render_benchmark_mbf(document: dict):
    """run_pilot.render_benchmark_mbf where this build has it (the sitting pack's branch adds it); otherwise the same
    writer call, with the same arguments, that it makes."""
    render = getattr(run_pilot, "render_benchmark_mbf", None)
    if render is not None:
        return render(document)
    return render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                 reference_temperature_c=document.get("reference_temperature_c", 20.0),
                                 piping_code=document["piping_code"])


def test_every_declared_run_loads_through_run_pilot_and_renders_its_declared_bytes():
    """Every series the manifest declares loads through run_pilot.load_benchmark, one run per model per series, each
    with the manifest's run id; each run's file, rendered by the one render and run_mbf_text, has the SHA-256 the
    manifest declares; option-addition and hand-entry runs load capture-only; models without runs load with no
    series."""
    entries = manifest()["models"]
    runs = [(entry, run) for entry in entries for run in entry.get("runs", [])]
    run_ids = [run["run_id"] for _, run in runs]
    assert len(run_ids) == len(set(run_ids))
    assert all(CASE_ID_PATTERN.match(run_id) for run_id in run_ids), [r for r in run_ids if not CASE_ID_PATTERN.match(r)]
    by_series: dict[str, dict[str, dict]] = {}
    for entry, run in runs:
        assert entry["model_id"] not in by_series.setdefault(run["series"], {}), (entry["model_id"], run["series"])
        by_series[run["series"]][entry["model_id"]] = run
    loaded = 0
    for series, declared in sorted(by_series.items()):
        documents_ = run_pilot.load_benchmark(MANIFEST, None, series)
        assert sorted(document["model_id"] for document in documents_) == sorted(declared), series
        for document in documents_:
            run = declared[document["model_id"]]
            assert document["run_id"] == run["run_id"] and document["series_run"] == series
            written = render_benchmark_mbf(document)
            assert written.status == "written", run["run_id"]
            text = benchmark_models.run_mbf_text(document, written.text)
            assert hashlib.sha256(text.encode("utf-8")).hexdigest() == run["mbf"]["sha256"], run["run_id"]
            if run.get("kind") in ("option_addition", "hand_entry"):
                assert document["capture_only"] is True
                assert document["closed_form"] == [] and document["inexact_inputs"] == {}
                instruction = document["window_instruction"]
                assert hashlib.sha256(instruction["text"].encode("utf-8")).hexdigest() == instruction["sha256"]
                assert document["pages"]
                if run["kind"] == "hand_entry":
                    assert document["model"] == by_id()[document["model_id"]]["model"]  # the written base
            loaded += 1
    assert loaded == len(runs)
    plain = [entry["model_id"] for entry in entries if "runs" not in entry]
    assert [document["model_id"] for document in run_pilot.load_benchmark(MANIFEST, plain)] == sorted(plain)


def test_no_tracked_benchmark_text_names_which_cases_the_product_publishes_as_sensitive():
    """Whether SWBPIPE's result for a case may be compared is read from the product at each solve; no tracked
    benchmark file records it (the project's Agent 0, 2026-11-12)."""
    texts = [MANIFEST.read_text(encoding="utf-8")] + [(BENCHMARK / entry["file"]).read_text(encoding="utf-8")
                                                      for entry in manifest()["models"] if entry["tranche"] == "3"]
    assert not [text for text in texts if "sensitive" in text.lower()]


# -- repair round 1: designed values, hashes, gravity, the spectrum load ------------------------------------------
from tools.caepipe_correlation import benchmark_designed as designed  # noqa: E402

DESIGNED_MODELS = S_MODELS + HAND_IDS


def test_designed_sections_and_material_follow_the_stated_rule():
    """Every designed outside diameter is a whole multiple of 10 mm, each wall is that diameter over the stated
    diameter-to-thickness ratio, and the material is the stated round values (benchmark_designed)."""
    docs = by_id()
    name, e, nu, alpha, _ = designed.DESIGNED_MATERIAL
    for model_id in DESIGNED_MODELS:
        document = docs[model_id]
        parameters, model = document["parameters"], document["model"]
        dt = parameters.get("dt", hand.DT)
        assert parameters["od"] % 10 == 0 and parameters["wall"] == parameters["od"] / dt, model_id
        for pipe in model["pipe_segments"]:
            od = pipe["section"]["outside_diameter"]["value"] * 1000.0
            wall = pipe["section"]["wall_thickness"]["value"] * 1000.0
            assert designed.designed_od(round(od, 9)) == parameters["od"] and wall == pytest.approx(od / dt,
                                                                                                    rel=1e-14), model_id
        [material] = model["materials"]
        assert material["id"] == name and material["elastic_modulus"]["value"] == e, model_id
        assert material["shear_modulus"]["value"] == pytest.approx(e / (2.0 * (1.0 + nu)), rel=1e-14), model_id
        assert material["thermal_expansion_coefficient"]["value"] == alpha, model_id
    # The dynamics models stay byte-equal to the paused lane's designs, whatever their section.
    assert all(docs[model_id]["model"]["materials"][0]["id"] == "material:dyn-a"
               for model_id in docs if model_id.startswith("cpb-dyn-"))
    for refused in (0.0, -10.0, 325.0, 172.5):
        with pytest.raises(ValueError):
            designed.designed_od(refused)


def test_every_model_declares_the_hash_of_its_written_file():
    for entry in manifest()["models"]:
        document = json.loads((BENCHMARK / entry["file"]).read_text(encoding="utf-8"))
        text = writer(document).text
        assert hashlib.sha256(text.encode("utf-8")).hexdigest() == entry["mbf_sha256"], entry["model_id"]


REVIEW_EIGHTEEN = sorted(
    [(f"cpb-g1-{i:02d}", None) for i in (8, 9, 10)] + [(f"cpb-g7-{i:02d}", None) for i in (1, 2, 3)]
    + [(f"cpb-p1-{i:02d}", run) for i in range(1, 7) for run in ("pressure-default", "pressure-elongation-on")],
    key=lambda item: (item[0], item[1] or ""))


def test_the_rigid_support_override_names_the_input_by_rule_and_changes_exactly_eighteen_classes():
    declaration = manifest()["rigid_support_override"]
    changed = sorted(((item["model_id"], item["run"]) for item in declaration["class_changed"]),
                     key=lambda item: (item[0], item[1] or ""))
    assert changed == REVIEW_EIGHTEEN
    assert all(item["case"] == "case:expansion" and item["class_as_first_screened"] == "exact"
               and item["class_named"] == "named_inexact" for item in declaration["class_changed"])
    assert declaration["text"].startswith("The class was changed on 13 November 2026")
    assert "no agreement count is stated for them under the named class alone" in declaration["text"]
    assert "does not read this override" in declaration["consumed_by"]
    for entry in manifest()["models"]:
        override = entry.get("inexact_input_override")
        if override and any(model == entry["model_id"] for model, _ in REVIEW_EIGHTEEN):
            assert override["both_classes"] == declaration["text"]


def test_every_thermal_case_with_two_or_more_anchors_names_the_rigid_support_input():
    """Semantic: any thermal case of a model with two or more anchors (each restraining the pipe's axial growth) names
    rigid_support_stiffness, in its own document (tranche 3) or by the manifest's override (tranches 1 and 2); a model
    with one anchor is not relabelled."""
    for entry in manifest()["models"]:
        document = json.loads((BENCHMARK / entry["file"]).read_text(encoding="utf-8"))
        model = document["model"]
        anchors = sum(1 for support in model["supports"] if support.get("family") == "anchor")
        thermal = [case["id"] for case in model["load_cases"]
                   if any(load.get("category") == "thermal" for load in case["primitive_loads"])]
        override = entry.get("inexact_input_override")
        if anchors < 2 or not thermal:
            assert override is None, entry["model_id"]
            continue
        assert entry["tranche"] == "3" or override is not None, f"{entry['model_id']} declares no override"
        for case in thermal + ["operating"]:
            if entry["tranche"] == "3":
                assert "rigid_support_stiffness" in document["inexact_inputs"][case], (entry["model_id"], case)
            elif entry["tranche"] == "2":
                for run, by_case in override["runs"].items():
                    assert "rigid_support_stiffness" in by_case[case]["inputs_named"], (entry["model_id"], run)
            else:
                assert "rigid_support_stiffness" in override["cases"][case]["inputs_named"], entry["model_id"]


def test_dynamics_instructions_check_gravity_and_enter_the_spectrum_load_in_its_pages_words():
    docs = by_id()
    for entry in dynamics_entries():
        for run in entry["runs"]:
            text = run["window_instruction"]["text"]
            assert "Direction of Gravity downward along Y (-Y)" in text and "Gravitational Acceleration 9.80665" in text
            assert "change nothing" in text and "Leave Gravitational" not in text
            for quote in t3.SPECTRUM_LOAD_QUOTES.values():
                assert quote in text
            assert "pure multiplier" in text and "docs.sstcae.com/users_manual/spectrum_load.htm" in text
            if run["spectrum_mode_sum"] == 4:  # NRL: not among the window's three choices
                assert run["window_mode_sum"] is None and run["mbf"]["added_records"][0]["lines"][1].split(",")[5] == "4"
                assert "Continue only if the displayed Mode Sum is NRL; otherwise record what it shows and skip." in text
                assert "offer NRL" not in text
            steps, paragraph, session = _parts(text)
            bodies = [_body(line) for line in steps]
            # The Misc tab is selected in the step just before the gravity check.
            misc = [i for i, body in enumerate(bodies) if body.startswith(
                f"Select the Misc tab (\"{t3.MISC_QUOTE}\"; the page gives Vertical Direction and Gravitational "
                f"Acceleration and Direction under it). [{hand.ANALYSIS_PAGE}]")]
            gravity = [i for i, body in enumerate(bodies) if body.startswith("Before the analysis, check the gravity")]
            assert len(misc) == 1 and len(gravity) == 1 and misc[0] == gravity[0] - 1
            # Include Missing Mass Correction in the wrong state is set and noted.
            requested = docs[entry["model_id"]]["dynamic_request"]["other_program_settings"]["include_missing_mass"]
            [mass] = [body for body in bodies if body.startswith("Check that Include Missing Mass Correction is")]
            assert ("If it is unchecked, check it and note that on the run line." if requested else
                    "If it is checked, uncheck it and note that on the run line.") in mass
            if run["comparable"]:
                assert paragraph is None
            elif run["spectrum_mode_sum"] == 4:
                assert paragraph.startswith("A further run of the same model in one session is not open for this run: "
                                            "the public Spectrum Load page lists no NRL among its Mode Sum choices; "
                                            "import this run's file. [") and not session
                assert "the window's Mode Sum lists no NRL" not in text
            else:
                comparable = run["run_id"].split("--")[0] + "--srss-srss" + ("-table-typed" if run["form"] ==
                                                                            "table_typed" else "")
                assert paragraph.startswith("Only if the command you noted at the dry run lets you choose the saved "
                                            "file's name and folder: instead of importing this run's file, you MAY")
                assert comparable in paragraph
                # The one-session alternative in numbered steps, the same words.
                assert [int(re.match(r"(\d+)\. ", line).group(1)) for line in session] == list(range(1, 7))
                own = [_body(line) for line in session]
                assert own[0].startswith(f"Change Mode Sum to {run['window_mode_sum']} in Loads > Spectrum. [")
                assert own[1].startswith("Change Direction Sum to ") and "in Loads > Spectrum. [" in own[1]
                assert own[2] == (f"Save the model as {run['run_id']}.mod in this run's folder by the command you noted "
                                  f"at the dry run.")
                assert own[3].startswith("Analyze it by File > Analyze (F12). [")
                assert own[4].startswith(f"Print the results to file as {run['run_id']}.print.csv. [")
                assert own[5] == "Write on this run's run line that the setting was made by hand."
    texts = manifest()["option_addition"]
    assert "dynamics-nrl-abs" in texts["nrl"] and "cpb-dyn-c-n08-two" in texts["single_form"]
    assert "not a run that was skipped" in texts["single_form"] and "checked, not entered" in " ".join(
        texts["conditions"])
    assert "Continue only if the displayed Mode Sum is NRL; otherwise record what it shows and skip." in texts["nrl"]
    assert "choose the saved file's name and folder" in texts["same_session_alternative"]
    dry_run = manifest()["dry_run"]
    assert "whichever single form" in dry_run["consequences"][1] and "This command saves model data to a .mod file" \
        in dry_run["save_command"]
    # The save observation as the file gives it: the run's model file and its saved model share a folder.
    assert dry_run["checks"][2] == ("(iii) write down the command that saved the imported model; whether the program "
                                    "let you choose the name and the folder; the name and folder the saved file took; "
                                    "whether the program asked anything")
    assert dry_run["consequences"][2] == ("(iii) the model cannot be saved as <run id>.mod in the run's folder: every "
                                          "hand-entry and every dynamics run is skipped and noted")
    assert "<run id>.mbf and its saved model <run id>.mod in the same folder" in dry_run["save_command"]
    assert "documents Save and no command for saving a model under another name" in dry_run["save_command"]


def _window_instructions():
    for entry in manifest()["models"]:
        for run in entry.get("runs", []):
            if isinstance(run, dict) and "window_instruction" in run:
                yield entry, run
    for document in hand_documents():
        yield document, document["hand_entry"]["run"]


def test_every_window_instruction_is_numbered_steps_with_the_citations_bracketed_at_their_end():
    """One text with one hash, laid out as numbered steps (digits, a full stop, plain line breaks, no markup), each
    step's page citations in brackets at its end and nowhere else; a further combination ends with one unnumbered
    closing paragraph laid out the same way."""
    seen = 0
    for _, run in _window_instructions():
        text = run["window_instruction"]["text"]
        assert hashlib.sha256(text.encode("utf-8")).hexdigest() == run["window_instruction"]["sha256"]
        lines = text.split("\n")
        steps, closing, session = _parts(text)
        assert [int(re.match(r"(\d+)\. \S", line).group(1)) for line in steps] == list(range(1, len(steps) + 1))
        assert (closing is not None) is (run.get("kind") == "option_addition" and not run["comparable"])
        assert [int(re.match(r"(\d+)\. \S", line).group(1)) for line in session] == list(range(1, len(session) + 1))
        for line in lines:
            body = line[:line.rindex(" [")] if line.endswith("]") else line
            assert line.strip() == line and line and not re.search(r"[*#`_]{2}|^\s*[-*] ", body), line  # no markup
            assert "https://" not in body, line  # a citation only in the closing brackets
            if line.endswith("]"):
                assert line[line.rindex(" [") + 2:-1].startswith("https://docs.sstcae.com/")
        seen += 1
    assert seen == 72 + 5  # the manifest's 72 runs and the five hand documents' own copies


def test_the_dry_run_sample_is_named_so_in_every_tranche_3_text():
    texts = manifest()["option_addition"]
    assert "dry-run sample" in texts["pairs"] and "dry-run sample" in texts["single_form"]
    assert "dry-run sample" in manifest()["dry_run"]["sample"]
    for _, run in _window_instructions():
        if run.get("kind") == "option_addition":
            assert "dry-run sample" in run["window_instruction"]["text"].split("\n")[0]
    tracked = [json.dumps(entry) for entry in manifest()["models"] if entry["tranche"] == "3"]
    tracked += [json.dumps(manifest()[key]) for key in ("option_addition", "dry_run", "hand_entry")]
    assert not [text for text in tracked if "format sample" in text]


def test_one_factor_or_one_per_direction_is_entered_without_ambiguity():
    for _, run in _window_instructions():
        if run.get("kind") != "option_addition":
            continue
        text = run["window_instruction"]["text"]
        assert ("If the window offers a single Factor and this run states the same factor for every direction it "
                "loads, enter that value; if it states different factors for different directions, skip this run, run "
                "nothing in its place") in text
        assert "enter it as stated" not in text and "rather than one per direction" not in text


# -- the optional export of a run's saved model ----------------------------------------------------------------------
def _export_expected() -> set[str]:
    """The runs that end with the export: every hand-entry run and every dynamics run at strike numbers 1 to 10."""
    return {run["run_id"] for entry in manifest()["models"] for run in entry.get("runs", [])
            if isinstance(run, dict) and "window_instruction" in run
            and (run["kind"] == "hand_entry" or (run["kind"] == "option_addition" and run["strike_order"] <= 10))}


def test_exactly_the_24_runs_end_with_the_optional_export_of_their_saved_model():
    expected = _export_expected()
    assert len(expected) == 24
    stepped = {run["run_id"] for _, run in _window_instructions() if "File > Export" in run["window_instruction"]["text"]}
    declared = {run["run_id"] for _, run in _window_instructions() if "export" in run}
    assert stepped == expected and declared == expected
    for _, run in _window_instructions():
        if run["run_id"] not in expected:
            continue
        run_id, text = run["run_id"], run["window_instruction"]["text"]
        name = f"{run_id}.export.mbf"
        lines, _, _ = _parts(text)
        kind = "hand" if run["kind"] == "hand_entry" else ("dynamics" if run["single_form"] else "pair")
        index = {key: next(i for i, line in enumerate(lines) if marker in line) for key, marker in (
            ("print", f"Save the .csv file as {run_id}.print.csv."), ("skip", hand.skip_rule(kind)),
            ("optional", "Optional: the next four steps"), ("menu", "From the Layout window, choose File > Export"),
            ("type", "In the Export Model dialog"), ("name", f"Set the name {name} in this run's folder."),
            ("guard", "Click Export"))}
        # After the print steps and the skip rule, one action per numbered step, the export closes the steps.
        assert [index[key] for key in ("print", "skip", "optional", "menu", "type", "name", "guard")] == list(
            range(index["print"], index["print"] + 7)) and index["guard"] == len(lines) - 1
        # The guard: the displayed name in this run's folder, nothing written over, Cancel otherwise.
        assert text.count(f"Click Export (\"{hand.EXPORT_QUOTE_BUTTON}\") only if the dialog displays exactly the name "
                          f"{name} in this run's folder and no file of that name is already there. If it does not, or "
                          f"if anything would be written over, click Cancel and write \"export not made\" and why on "
                          f"the run line; the run is complete.") == 1
        # A control not offered as described ends the export, not the run.
        assert ('If any control in these steps is not offered as described, click Cancel if a dialog is open, write '
                '"export not made" and why on the run line; the run is complete.') in _body(lines[index["optional"]])
        # The declared loss is carried to the owner.
        assert f"If you drop it, what is lost is {run['export']['lost_if_dropped']}." in _body(lines[index["optional"]])
        assert ("Take them only if the dry run's check (v) found that the export dialog lets you set the file's name and "
                "folder.") in text and '"export not made" on the run line' in text
        # The binding: this run's saved model, as the program holds it after these steps; not the pack's file.
        assert (f"The file it writes records this run's saved model, {run_id}.mod, as the program holds it after this "
                f"instruction's steps, including what was entered or set by hand; it is not the pack's model file.") in text
        for quote in (hand.EXPORT_QUOTE_MENU, hand.EXPORT_QUOTE_TYPE, hand.EXPORT_QUOTE_BUTTON):
            assert quote in text
        declaration = run["export"]
        assert declaration["file"] == name and declaration["optional"] is True and "export not made" in declaration[
            "if_dropped"] and f"{run_id}.mod" in declaration["records"]
        lost = declaration["lost_if_dropped"]
        if run["kind"] == "hand_entry":
            assert lost.startswith("the program's own writing of the support record entered by hand")
        else:
            assert "spectrum assignment and the analyses chosen" in lost and "documents no record" in lost
    statement = manifest()["export"]["statement"]
    assert "private, like every returned file" in statement and "later interoperability work" in statement
    assert "do not demonstrate exit criterion 6's round trip in the desktop" in statement
    assert "do not make any family the product refuses become supported" in statement


def test_no_text_binds_an_export_to_the_packs_file():
    texts = [json.dumps(entry, ensure_ascii=False) for entry in manifest()["models"] if entry["tranche"] == "3"]
    texts += [json.dumps(manifest()[key], ensure_ascii=False) for key in ("export", "dry_run", "option_addition",
                                                                          "hand_entry")]
    texts += [json.dumps(document["hand_entry"], ensure_ascii=False) for document in hand_documents()]
    for text in texts:
        assert "input bytes" not in text.lower()
        # The one sentence that names the pack's file says the export is NOT it.
        assert text.replace("it is not the pack's model file", "").count("pack's") == 0


def test_dry_run_check_v_uses_the_sacrificial_duplicate_and_leaves_the_window_sample_untouched():
    dry_run = manifest()["dry_run"]
    check = dry_run["checks"][4]
    head, *steps = check.split("\n")
    assert head == ("(v) on a sacrificial duplicate: the folder samples/sample-dyn-export-trial holds "
                    "sample-dyn-export-trial.mbf, a copy of the same run file as the window sample "
                    "(cpb-dyn-c-n01-flat--srss-srss); in that folder only:")
    assert [int(re.match(r"(\d+)\. ", line).group(1)) for line in steps] == list(range(1, len(steps) + 1))
    for sentence in ("Import sample-dyn-export-trial.mbf: File > Import from the Main window",
                     "Before touching the dialog, write down the files the folder now holds, by name.",
                     "Note whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, "
                     "and whether it displays them before anything is written.",
                     "If it does, click Export; if not, click Cancel.",
                     "Note the files the folder then holds, by name.",
                     "Nothing in samples/sample-dyn-window or in any case folder is opened or touched by this check."):
        assert sum(sentence in line for line in steps) == 1, sentence
    assert dry_run["consequences"][3] == (
        "(v) the dialog did not let you set the name and the folder, or a file was written before a destination was "
        "offered: no run exports at this sitting, and the export steps of every run are skipped and noted")
    assert dry_run["export_trial"]["folder"] == "samples/sample-dyn-export-trial"
    assert dry_run["export_trial"]["copy_of_run"] == "cpb-dyn-c-n01-flat--srss-srss"


# -- layout helpers and the pair rule --------------------------------------------------------------------------------
def _parts(text: str) -> tuple[list[str], str | None, list[str]]:
    """An instruction's numbered steps, its unnumbered closing line (or None) and the closing's own numbered steps."""
    lines = text.split("\n")
    cut = next((i for i, line in enumerate(lines) if not re.match(r"\d+\. ", line)), len(lines))
    return lines[:cut], (lines[cut] if cut < len(lines) else None), lines[cut + 1:]


def _body(line: str) -> str:
    return re.sub(r"^\d+\. ", "", line)


def _pairs():
    runs = {run["run_id"]: run for entry in dynamics_entries() for run in entry["runs"]}
    return [(run, runs[run["counterpart_run_id"]]) for run in runs.values() if run["form"] == "table_added"]


def test_form_a_leads_to_form_b_only_at_its_named_initial_checks_and_form_b_admits_it_without_return():
    assert t3.A_SWITCH_STEPS == (4, 5)
    for a, b in _pairs():
        a_steps, a_close, a_session = _parts(a["window_instruction"]["text"])
        b_steps, b_close, b_session = _parts(b["window_instruction"]["text"])
        switch = (f"switch to the counterpart: this run is not run, and any files the import wrote in this run's folder "
                  f"are left as they are; write \"switched to counterpart {b['run_id']}\" on this run line; then run "
                  f"{b['run_id']} from its own folder, from its step 1.")
        for number in t3.A_SWITCH_STEPS:
            assert switch in a_steps[number - 1], (a["run_id"], number)
        # Only A's dry-run selection (step 1) and its named initial checks name the counterpart.
        assert [i + 1 for i, line in enumerate(a_steps + [a_close or ""] + a_session) if b["run_id"] in line] == [
            1, *t3.A_SWITCH_STEPS]
        # B admits entry from A's named checks and never sends back to A after it.
        assert (f"or if its counterpart {a['run_id']} was switched to this run at its step 4 or 5; otherwise run its "
                f"counterpart {a['run_id']} instead. Once switched to this run, do not return to {a['run_id']}.") in b_steps[0]
        a_id = re.compile(re.escape(a["run_id"]) + r"(?![\w-])")  # A's id is a prefix of B's: match it whole
        assert [i + 1 for i, line in enumerate(b_steps + [b_close or ""] + b_session) if a_id.search(line)] == [1]
        # A's note says it switched; it does not claim the counterpart ran.
        notes = re.findall(r'write "([^"]*)" on this run line', a["window_instruction"]["text"])
        assert set(notes) == {f"switched to counterpart {b['run_id']}"}
        assert "counterpart run" not in a["window_instruction"]["text"] and " ran" not in notes[0]


def test_every_other_skip_runs_nothing_in_its_place_and_goes_on_to_the_next_item():
    for _, run in _window_instructions():
        kind = "hand" if run["kind"] == "hand_entry" else ("dynamics" if run["single_form"] else "pair")
        following = f"go on to {hand.NEXT_ITEM[kind]}"
        for line in run["window_instruction"]["text"].split("\n"):
            skips = len(re.findall(r"\bskip\b", line))
            assert line.count(following) >= skips, (run["run_id"], line[:80])
            assert line.count("run nothing in its place") + line.count("Run nothing in its place") >= skips
            if kind == "pair":
                # A skip in either form never invites the other form.
                assert line.count("not its counterpart either") >= skips, (run["run_id"], line[:80])
            assert "counterpart run" not in line


def test_the_pair_rule_says_at_most_one_is_analysed_and_every_pair_records_an_outcome():
    texts = manifest()["option_addition"]
    rule = texts["pairs"]
    assert "At most one of a pair is analysed." in rule and "Exactly one of a pair" not in rule
    assert ("Every pair ends with an explicit outcome on its two run lines: form A analysed, form B analysed, or neither "
            "run and why.") in rule
    assert ("Only form A's named initial checks, of the spectrum table and of the spectrum options, which come before "
            "its explicit Save and Analyze steps, lead to form B") in rule
    assert 'its run line says "switched to counterpart <form B run id>"' in rule and "no return to form A" in rule
    assert texts["run_sheet_pair_rule"].startswith("At most one of a pair is analysed")
    assert "neither run and why" in texts["run_sheet_pair_rule"]


def test_no_tranche_3_prose_says_trial():
    """Check (v) is a check: no prose string of tranche 3 (window instructions, the dry-run text, the export statement,
    the option-addition and hand-entry declarations, tranche 3's manifest entries) uses the word "trial". The folder
    and file names samples/sample-dyn-export-trial and sample-dyn-export-trial.mbf, and the key export_trial, are
    identifiers, not prose, and are left out of the search."""
    texts = [run["window_instruction"]["text"] for _, run in _window_instructions()]
    texts += [json.dumps(manifest()[key], ensure_ascii=False) for key in ("dry_run", "export", "option_addition",
                                                                          "hand_entry")]
    texts += [json.dumps(entry, ensure_ascii=False) for entry in manifest()["models"] if entry["tranche"] == "3"]
    for text in texts:
        prose = text.replace("sample-dyn-export-trial", "").replace("export_trial", "")
        assert not re.search(r"(?i)\btrials?\b", prose), re.search(r"(?i).{0,60}\btrials?\b", prose).group(0)
