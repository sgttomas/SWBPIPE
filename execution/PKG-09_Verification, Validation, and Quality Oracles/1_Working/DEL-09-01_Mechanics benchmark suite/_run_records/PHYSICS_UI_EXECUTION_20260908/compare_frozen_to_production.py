#!/usr/bin/env python3
"""Compare production results to the already-frozen independent reference."""

from __future__ import annotations

import argparse
import csv
import json
import math
from pathlib import Path
from typing import Any


def product_map(path: Path) -> tuple[dict[str, float], dict[str, Any]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    return {item["id"]: float(item["value"]) for item in payload["results"]}, payload


def expected_rows(case: dict[str, Any], include_friction: bool) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    dofs = ("ux", "uy", "uz", "rx", "ry", "rz")
    for node, vector in case["nodal_displacements_global"].items():
        for index, name in enumerate(dofs):
            factor = 1000.0 if index < 3 else 1.0
            rows.append({
                "family": "nodal_displacement",
                "id": f"result:disp:node-{node}:{name}",
                "expected": vector[index] * factor,
                "unit": "mm" if index < 3 else "rad",
            })

    state = case["state"]
    rows.extend([
        {"family":"nonlinear","id":"result:nonlinear-support:support-NL-140:uy-displacement","expected":state["contact"]["displacement_u_m"]*1000.0,"unit":"mm"},
        {"family":"nonlinear","id":"result:nonlinear-support:support-NL-140:uy-reaction","expected":state["contact"]["reaction_R_N"],"unit":"N"},
    ])
    if include_friction:
        rows.extend([
            {"family":"nonlinear","id":"result:nonlinear-support:support-NL-130-FRIC:uz-displacement","expected":state["friction"]["tangential_displacement_m"]*1000.0,"unit":"mm"},
            {"family":"nonlinear","id":"result:nonlinear-support:support-NL-130-FRIC:uz-reaction","expected":state["friction"]["tangential_action_N"],"unit":"N"},
            {"family":"nonlinear","id":"result:nonlinear-support:support-NL-130-FRIC:friction-normal-reaction","expected":state["friction"]["current_normal_magnitude_N"],"unit":"N"},
        ])

    support = case["support_actions"]
    support_names = ["S-100", "S-120", "S-130", "NL-140", "SH-140"]
    if include_friction:
        support_names.append("NL-130-FRIC")
    for name in support_names:
        # Product support-reaction rows are force magnitudes only. S-100 also
        # has rotational restraint actions, which remain in the independent
        # signed vector but must not be mixed dimensionally into this scalar.
        values = support[name][:3] if name == "S-100" else support[name]
        magnitude = math.sqrt(sum(value * value for value in values))
        rows.append({"family":"support_magnitude","id":f"result:reaction:support-{name}","expected":magnitude,"unit":"N"})

    force_names = ("axial", "shear-y", "shear-z")
    moment_names = ("torsion", "bending-y", "bending-z")
    station_names = ((0.25, "quarter-1"), (0.5, "midspan"), (0.75, "quarter-3"))
    for pipe, values in case["element_results"].items():
        end = values["local_end_vector_N_Nm"]
        for index, name in enumerate(force_names):
            rows.append({"family":"element_end","id":f"result:force:pipe-{pipe}:{name}","expected":end[index],"unit":"N"})
            rows.append({"family":"element_end","id":f"result:force:pipe-{pipe}:{name}:end-j","expected":end[index+6],"unit":"N"})
        for index, name in enumerate(moment_names):
            rows.append({"family":"element_end","id":f"result:moment:pipe-{pipe}:{name}","expected":end[index+3],"unit":"N*m"})
            rows.append({"family":"element_end","id":f"result:moment:pipe-{pipe}:{name}:end-j","expected":end[index+9],"unit":"N*m"})
        by_fraction = {station["fraction"]: station["local_cut_vector_N_Nm"] for station in values["stations"]}
        for fraction, label in station_names:
            station = by_fraction[fraction]
            for index, name in enumerate(force_names):
                rows.append({"family":"station_cut","id":f"result:force:pipe-{pipe}:{label}:{name}","expected":station[index],"unit":"N"})
            for index, name in enumerate(moment_names):
                rows.append({"family":"station_cut","id":f"result:moment:pipe-{pipe}:{label}:{name}","expected":station[index+3],"unit":"N*m"})
    return rows


def compare_case(case_name: str, expected_case: dict[str, Any], output_dir: Path, include_friction: bool) -> dict[str, Any]:
    dense, dense_payload = product_map(output_dir / f"{case_name}.dense_scrutiny.json")
    sparse, sparse_payload = product_map(output_dir / f"{case_name}.sparse_interactive.json")
    rows = expected_rows(expected_case, include_friction)
    for row in rows:
        identity = row["id"]
        if identity not in dense or identity not in sparse:
            raise KeyError(identity)
        row["production_dense"] = dense[identity]
        row["production_sparse"] = sparse[identity]
        row["dense_minus_expected"] = dense[identity] - row["expected"]
        row["sparse_minus_expected"] = sparse[identity] - row["expected"]
        row["dense_abs_error"] = abs(row["dense_minus_expected"])
        row["sparse_abs_error"] = abs(row["sparse_minus_expected"])
        row["dense_relative_error"] = None if row["expected"] == 0.0 else row["dense_abs_error"] / abs(row["expected"])
        row["sparse_relative_error"] = None if row["expected"] == 0.0 else row["sparse_abs_error"] / abs(row["expected"])
        row["expected_rounded_6dp"] = round(row["expected"], 6)
        row["dense_minus_expected_rounded_6dp"] = dense[identity] - row["expected_rounded_6dp"]
        row["sparse_minus_expected_rounded_6dp"] = sparse[identity] - row["expected_rounded_6dp"]
        row["dense_sparse_delta"] = dense[identity] - sparse[identity]
        row["comparison_status"] = "EXACT_AT_EMITTED_PRECISION" if dense[identity] == sparse[identity] == row["expected_rounded_6dp"] else "DIFFERS_FROM_FROZEN_REFERENCE"
    differing = [row for row in rows if row["comparison_status"] != "EXACT_AT_EMITTED_PRECISION"]
    return {
        "case": case_name,
        "production_status": {"dense":dense_payload["status"]["mechanics"],"sparse":sparse_payload["status"]["mechanics"]},
        "row_count": len(rows),
        "exact_at_emitted_precision_count": len(rows)-len(differing),
        "differing_count": len(differing),
        "max_abs_error_by_unit": {
            unit: max((row["dense_abs_error"] for row in rows if row["unit"] == unit), default=0.0)
            for unit in sorted({row["unit"] for row in rows})
        },
        "max_relative_error": max((row["dense_relative_error"] or 0.0 for row in rows), default=0.0),
        "dense_sparse_max_abs_delta": max(abs(row["dense_sparse_delta"]) for row in rows),
        "rows": rows,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected", type=Path, required=True)
    parser.add_argument("--production-dir", type=Path, required=True)
    parser.add_argument("--json-output", type=Path, required=True)
    parser.add_argument("--csv-output", type=Path, required=True)
    args = parser.parse_args()
    frozen = json.loads(args.expected.read_text(encoding="utf-8"))
    comparisons = [
        compare_case("physical_frame_frictionless", frozen["physical_frame_frictionless_actual_control"], args.production_dir, False),
        compare_case("literal_adapter_frictionless", frozen["literal_adapter_mapping_diagnostic"]["frictionless"], args.production_dir, False),
        compare_case("physical_frame", frozen["physical_frame_zero_reference_current_normal_static_result"], args.production_dir, True),
        compare_case("literal_adapter", frozen["literal_adapter_mapping_diagnostic"]["zero_reference_current_normal_static"], args.production_dir, True),
    ]
    result = {
        "schema_version": "1.0",
        "status": "POST_FREEZE_PRODUCTION_COMPARISON",
        "interpretation": "raw observed differences only; no new engineering tolerance or acceptance threshold",
        "known_contract_check": "production friction action is compared with mu times the same production-returned normal separately in REPORT.md",
        "comparisons": comparisons,
    }
    args.json_output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")
    columns = ["case","family","id","unit","expected","expected_rounded_6dp","production_dense","production_sparse","dense_minus_expected","sparse_minus_expected","dense_abs_error","sparse_abs_error","dense_relative_error","sparse_relative_error","dense_minus_expected_rounded_6dp","sparse_minus_expected_rounded_6dp","dense_sparse_delta","comparison_status"]
    with args.csv_output.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=columns, lineterminator="\n")
        writer.writeheader()
        for comparison in comparisons:
            for row in comparison["rows"]:
                writer.writerow({"case":comparison["case"], **row})


if __name__ == "__main__":
    main()
