"""Read-only independent Fraction/Decimal backcheck; writes only beside this file."""
from pathlib import Path
from fractions import Fraction as F
from decimal import Decimal, localcontext
import hashlib
import json
import math
import sys

packet = Path(sys.argv[1]).resolve()
raw = packet / "_run_records"
claimed = json.loads((packet / "ANALYSIS.json").read_text())
delivery = json.loads((packet / "DELIVERY_HASHES.json").read_text())
hashes = {}
for name, expected in delivery["sha256"].items():
    actual = hashlib.sha256((packet / name).read_bytes()).hexdigest()
    assert actual == expected, (name, actual, expected)
    hashes[name] = actual
for name, expected in claimed["raw_sha256"].items():
    actual = hashlib.sha256((raw / name).read_bytes()).hexdigest()
    assert actual == expected, (name, actual, expected)
    hashes["_run_records/" + name] = actual
for name in ["product_before.rs", "nonlinear_before.rs"]:
    hashes["_run_records/" + name] = hashlib.sha256((raw / name).read_bytes()).hexdigest()
context = json.loads((raw / "SOURCE_CONTEXT.json").read_text())
assert hashes["_run_records/product_before.rs"] == context["source_files_sha256"]["projects/chirality-piping/core/product_physics/src/lib.rs"]
assert hashes["_run_records/nonlinear_before.rs"] == context["source_files_sha256"]["projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs"]
model = json.loads((raw / "model.json").read_text())
assert model["project"]["units"]["length"] == "m"
assert model["project"]["units"]["force"] == "N"
assert [n["position"] for n in model["nodes"]] == [
    {"x": 0, "y": 0, "z": 0}, {"x": 1, "y": 0, "z": 0}]
section = model["pipe_segments"][0]["section"]
assert section["outside_diameter"] == {"value": 0.168, "unit": "m"}
assert section["wall_thickness"] == {"value": 0.007, "unit": "m"}
assert model["materials"][0]["elastic_modulus"] == {"value": 200000000000, "unit": "Pa"}
loads = model["load_cases"][0]["primitive_loads"]
assert {v["direction"]:v["magnitude"] for v in loads} == {
    "global_x": {"value":100, "unit":"N"},
    "global_y": {"value":100000, "unit":"N"},
    "global_z": {"value":100, "unit":"N"}}
nonlinear = {s["nonlinear"]["behavior"]:s["nonlinear"] for s in model["supports"] if s["nonlinear"]}
assert nonlinear["gap"]["gap"] == {"value":0.05, "unit":"mm"}
assert nonlinear["friction"]["friction_coefficient"] == {"value":0.3, "unit":"none"}
assert nonlinear["friction"]["normal_reaction"] == {"value":10, "unit":"N"}

events = [json.loads(line) for line in (raw / "rows.jsonl").read_text().splitlines()]
solves = [event for event in events if event["event"] == "linearized"]
states = [event for event in events if event["event"] == "iteration"]
last = solves[-1]
selected = states[-1]
assert last["u"] == selected["u"]
assert selected["iteration"] == 2 and selected["active_count_residual"] == 0
assert selected["changed_supports"] == [] and selected["converged"] and not selected["blocked"]
assert "max_iterations: 4" in selected["convergence_debug"]
assert "residual_tolerance: 0.0" in selected["convergence_debug"]
assert "absolute_residual_floor: 0.0" in selected["convergence_debug"]
assert last["free_dofs"] == [6, 8, 9, 10, 11]
assert set(last["prescribed_dofs"]) == {0, 1, 2, 3, 4, 5, 7}
for i, value in zip(last["prescribed_dofs"], last["prescribed_values"]):
    assert last["u"][i] == value
assert last["f"][6] == 100 and last["f"][7] == 100000 and last["f"][8] == 97
assert selected["friction_dof_force"] == [[8.0, -3.0]]

roundoff = F(1, 2**53)
def gamma(count):
    return count * roundoff / (1 - count * roundoff)

def decimal(value):
    with localcontext() as context:
        context.prec = 90
        return str(Decimal(value.numerator) / Decimal(value.denominator))

def extract_rows(displacement, include_claims):
    output = []
    for row_index in last["free_dofs"]:
        coefficients = last["K"][row_index]
        assert len(coefficients) == len(displacement) == len(last["f"]) == 12
        assert all(math.isfinite(v) for v in coefficients + displacement + [last["f"][row_index]])
        exact_products = [F(a) * F(v) for a, v in zip(coefficients, displacement)]
        residual = sum(exact_products, F()) - F(last["f"][row_index])
        denominator = sum((abs(term) for term in exact_products), F()) + abs(F(last["f"][row_index]))
        expanded = 0.0
        reduced = 0.0
        intermediates = []
        for a, v in zip(coefficients, displacement):
            product = a * v
            expanded += product
            intermediates.extend([product, expanded])
            if a != 0:
                reduced += product
        observed = expanded - last["f"][row_index]
        reduced -= last["f"][row_index]
        assert observed == reduced
        intermediates.append(observed)
        assert all(math.isfinite(v) and (v == 0 or abs(v) >= sys.float_info.min) for v in intermediates)
        effective_count = sum(a != 0 for a in coefficients)
        count = 2 * effective_count + 2
        g = gamma(count)
        allowance = gamma(26) * denominator
        effective_allowance = g * denominator
        evaluation_error = abs(F(observed) - residual)
        assert evaluation_error <= allowance and evaluation_error <= effective_allowance
        if denominator:
            true_ratio = abs(residual) / denominator
            observational_guard = ((abs(F(observed)) + g * denominator / (1 - g))
                                   / (denominator / (1 + g))) * (1 + gamma(4))
        else:
            assert residual == observed == 0
            true_ratio = observational_guard = F()
        target = 64 * g
        true_work = abs(F(displacement[row_index]) * residual)
        observed_work = abs(displacement[row_index] * observed)
        work_bound = abs(F(displacement[row_index])) * target * denominator
        if include_claims:
            row = next(r for r in claimed["rows"] if r["global_dof"] == row_index)
            assert F(row["exact_represented_residual"]) == residual
            assert F(row["exact_represented_denominator"]) == denominator
            assert row["selected_policy_operation_count"] == count
            assert row["observed_residual"] == last["reactions"][row_index] == observed
            assert row["independent_selected_screen_pass"] == (observational_guard <= target)
            assert row["observed_work_product"] == observed_work
            assert observational_guard <= target and true_ratio <= target
            assert true_work <= work_bound and F(observed_work) <= work_bound
        output.append({
            "global_dof": row_index,
            "unit": "N" if row_index % 6 < 3 else "N*m",
            "residual_exact_fraction": str(residual),
            "residual_exact_decimal": decimal(residual),
            "denominator_exact_fraction": str(denominator),
            "denominator_approx": float(denominator),
            "observed_residual": observed,
            "evaluation_error": float(evaluation_error),
            "evaluation_bound_gamma26": float(allowance),
            "effective_evaluation_bound": float(effective_allowance),
            "nonzero_coefficient_count": effective_count,
            "effective_operation_count": count,
            "true_ratio": float(true_ratio),
            "guarded_ratio": float(observational_guard),
            "target": float(target),
            "screen_pass": observational_guard <= target,
            "true_work": float(true_work),
            "observed_work": observed_work,
            "target_work_bound": float(work_bound),
            "nonzero_term_products": [
                {"column": j, "coefficient_hex": float(a).hex(),
                 "displacement_hex": float(displacement[j]).hex(),
                 "exact_product": str(exact_products[j])}
                for j, a in enumerate(coefficients) if a != 0
            ],
        })
    return output

good = extract_rows(last["u"], True)
wrong = extract_rows(solves[0]["u"], False)
bad_rows = [r["global_dof"] for r in wrong if not r["screen_pass"]]
assert bad_rows
boundary_bad = [
    {"global_dof": i, "actual": solves[0]["u"][i], "required": value}
    for i, value in zip(last["prescribed_dofs"], last["prescribed_values"])
    if solves[0]["u"][i] != value
]
assert boundary_bad and states[0]["active_count_residual"] == 3
assert not states[0]["converged"]

# Independent pi via Machin's identity, not the supplied Gauss-Legendre routine.
# Alternating-series truncation bounds are carried as exact fractions.
def arctan_inverse(q, count):
    total = sum((F((-1)**k, (2*k + 1) * q**(2*k + 1))
                 for k in range(count)), F())
    remainder = F(1, (2*count + 1) * q**(2*count + 1))
    return total, remainder
a5, b5 = arctan_inverse(5, 100)
a239, b239 = arctan_inverse(239, 30)
pi_value = 16*a5 - 4*a239
pi_error = 16*b5 + 4*b239
assert pi_error < F(1, 10**130)
od, wall, length, modulus, gap = F(21,125), F(7,1000), F(1), F(200000000000), F(1,20000)
inside = od - 2*wall
area = pi_value * (od**2 - inside**2)/4
second_moment = pi_value * (od**4 - inside**4)/64
ei = modulus * second_moment
references = {
    6: F(100)*length/(modulus*area),
    7: gap,
    8: F(97)*length**3/(3*ei),
    10: -F(97)*length**2/(2*ei),
    11: 3*gap/(2*length),
}
reference_rows = []
for i, reference in references.items():
    error = abs((F(last["u"][i]) - reference)/reference)
    assert error < F(1, 10**9)
    reference_rows.append({"global_dof": i, "unit": "m" if i%6<3 else "rad",
                           "expected": decimal(reference), "actual": last["u"][i],
                           "relative_error": float(error), "limit": "1e-9"})
contact_force = 3*ei*gap/length**3 - 100000
root_y = -3*ei*gap/length**3
assert contact_force < 0 and last["u"][8] > 0 and selected["friction_dof_force"][0][1] < 0
assert last["u"][6] > 0
assert root_y + contact_force + 100000 == 0
assert F(1) > 64*gamma(4)
tiny = F.from_float(1e-12)
assert float(tiny)*0 == 0 and round(float(tiny)*1e6)/1e6 == 0

report = {
    "actor": "/root/numerical_policy_review",
    "method": "Independent read-only exact Fraction row arithmetic and bounded Machin pi; no production imports or solver/test execution",
    "python": sys.version,
    "packet_hashes_verified": hashes,
    "linearized_record_count": len(solves),
    "iteration_record_count": len(states),
    "selected_mode": last["mode"],
    "selected_iteration": selected["iteration"],
    "all_prescribed_values_match": True,
    "all_rows_finite_normal_or_zero": True,
    "all_five_original_row_screens_pass": True,
    "rows": good,
    "machin_pi_absolute_error_bound": decimal(pi_error),
    "physical_reference": reference_rows,
    "L_over_OD": decimal(length/od),
    "root_Y_force_N": decimal(root_y),
    "gap_contact_force_N": decimal(contact_force),
    "negative_controls": {
        "stale_first_state": {"failed_rows": bad_rows, "rows": wrong,
                             "boundary_mismatches": boundary_bad,
                             "initial_count": states[0]["active_count_residual"],
                             "final_count": selected["active_count_residual"],
                             "separate_contact_law_mutation": False},
        "tiny_omitted_load": {"load_N": 1e-12, "eta": 1, "old_quantized_value": 0,
                              "work_Nm": 0, "rejects": True},
    },
    "limits": [
        "Confirms captured represented rows and selected analytical element theory only.",
        "The observed residual includes nonzero exact represented residual; it is not solely evaluation error.",
        "Observation guard used exact rational denominator; it does not qualify current runtime bound arithmetic.",
        "Stale-state control changes both equations/state and prescribed values; not an isolated complementarity mutation.",
        "Source predates pending kernel repairs; no current implementation, dense-mode, or complete-candidate pass."
    ],
}
destination = Path(__file__).with_name("ROW_SUCCESSOR_PROOF.json")
destination.write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({"status": "PASS_BOUNDED_REFERENCE_BACKCHECK", "row_count": len(good),
                  "max_true_ratio": max(r["true_ratio"] for r in good),
                  "failed_stale_rows": bad_rows, "pi_bound": decimal(pi_error),
                  "output": str(destination)}, indent=2))
