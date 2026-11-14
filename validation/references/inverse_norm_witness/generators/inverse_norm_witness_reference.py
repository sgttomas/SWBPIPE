#!/usr/bin/env python3
"""Known-limitation witness of the integrity check's inverse-norm premise: models and exact truth.

DEL-04-01's integrity check (REQ-015, REQ-016, AC-013; ScopeOfWork.md, OUT-001 row of REQ-015) takes
1024 times a Hager-Higham estimate of the inverse norm of the scaled free system as an upper bound on
that norm. This package builds invented models, accepted through the product's ordinary model
document, on which that premise fails, and computes their truth in exact rational arithmetic.

The models. One objective connector (an expansion joint replacing the span A-B) between an anchored
node A and a free node B, with a user-entered coupled 6x6 work matrix. Its matrix is chosen so that the
free system's one weak direction is orthogonal to both of the estimator's start vectors (the vector of
ones and the alternating vector), so the estimator never sees it. Variants:
  linear            nodal loads only. Truth: the exact displacement the check's enclosure should contain.
  one_way_negative  plus a one-way stop on B's UZ that bears with a negative reaction, and a tiny load term.
  one_way_positive  the same with a stop that bears with a positive reaction and the tiny term reversed.
Each one-way variant has exactly one admissible state, and it is active.

Independence. Python 3 standard library only (fractions, json, hashlib, argparse, pathlib). It reads,
imports and runs no product code, product output, fixture or schema: it writes the request documents
itself and derives every value from them. What it assumes about how the product forms the system is
stated in ASSUMPTIONS and in the README, each with the source it was checked against.

Usage:  python3 -I -B inverse_norm_witness_reference.py [--check]  # default: rebuild in memory, compare bytes
        python3 -I -B inverse_norm_witness_reference.py --write    # rewrite requests, references.json, SHA256SUMS
"""

import argparse
import hashlib
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
LOADS = [("global_x", "force", "N"), ("global_y", "force", "N"), ("global_z", "force", "N"),
         ("rotation_x", "moment", "N*m"), ("rotation_y", "moment", "N*m"), ("rotation_z", "moment", "N*m")]
STOP = 2  # B's UZ
PROVENANCE = "invented inverse-norm witness (DEL-04-01 OUT-001); not library, catalog, manufacturer or code-rule data"

# ---- the numbers and why ----------------------------------------------------------------------------
# K0: the free system's stiffness scale (N/m, N/rad, N*m/rad alike). 2^20 keeps every scaled diagonal a
# power of two times a short mantissa, so the product's power-of-two equilibration is known exactly.
K0 = F(2 ** 20)
# LAMBDA: the weak eigenvalue of M = K_free / K0 (its other eigenvalues are 1). 2^-20 gives a scaled
# inverse norm of exactly 2^18 = 262144 while 1024 times the estimator's value is 1024 (ratio 1/256),
# and keeps the condition number (2^20 scaled) far inside the check's own contraction test (2^-20).
LAMBDA = F(1, 2 ** 20)
# W: the weak direction, a unit vector on UX, UY, UZ, RX. It is orthogonal to the vector of ones and to
# the six-entry alternating vector (-1)^i (1 + i/5) used by the estimator, and it is zero on RY and RZ,
# so the estimator's one unit-vector step (it moves to the last index, RZ) also misses it.
W = [F(1, 2), F(1, 2), F(-1, 2), F(-1, 2), F(0), F(0)]
# L: the chord A-B along global x (m). The connector's axes are global, its offsets zero and its
# translation scale 1 m, so H = K and every formed entry is a short dyadic rational, exact in binary64.
L = F(1)
# X_MAIN: the displacement the nodal loads produce (m, rad). Its UZ is exactly 0: that is where the stop
# sits (ground 0, no imposed movement), so the main loads leave the stop exactly at its threshold and
# only the tiny term decides its state. The loads are F = K_free X_MAIN, exact in binary64. Among the
# small integer patterns tried, this one is solved inexactly by both of the product's factorizations,
# with loop errors of opposite sign at the stop (dense below, sparse above): that is what selects the
# stop's sense per mode. Some patterns (for example (4, -2, 0, 3, 2, -1)) the sparse factorization
# solves exactly, and no error then arises in sparse mode.
X_MAIN = [F(v, 2 ** 12) for v in (3, 1, 0, -5, 2, 1)]
# TINY: the separate load term on B's UZ (N). Its exact response at the stop is TINY times
# (K_free^-1)[UZ][UZ] = TINY * (2^20 + 3) / 2^22. The stop's state is wrong only if this response is
# smaller than the error the check's twice-corrected solution still carries along the weak direction
# (about (kappa u)^3 times the solution, kappa = 2^20, u = 2^-53), and its sign makes the stop's exact
# state active. Its magnitude was located by observing that error in each mode (the window lies between
# the radius the check claims and the error it carries, a factor of up to 256); the truth below does not
# depend on that observation. Dense mode: a stop bearing with a negative reaction and TINY = +2^-112 N.
# Sparse mode: a stop bearing with a positive reaction and TINY = -2^-121 N.
VARIANTS = {
    "linear": {"stop": None, "tiny": None},
    "one_way_negative": {"stop": "negative_reaction", "tiny": F(1, 2 ** 112)},
    "one_way_positive": {"stop": "positive_reaction", "tiny": F(-1, 2 ** 121)},
}
INITIAL_STATE = "inactive"

ASSUMPTIONS = {
    "connector formation": "Ke = B^T K B with B = Q^T[-I, S(a_i)+S(r)/2, I, -S(a_j)+S(r)/2] on translations and "
                           "Q^T[0, -I, 0, I] on rotations, S(v)w = v x w, r = (x_j - x_i) + (a_j - a_i); K = D^-1 H D^-1, "
                           "D = diag(Ls, Ls, Ls, 1, 1, 1); each entry one exact sum rounded once "
                           "(core/solver/frame_kernel/src/connector.rs module comment and b(), global_stiffness(), "
                           "decode_work_matrix()). Checked here: every entry of B, K and Ke is exact in binary64, so "
                           "the product's formed and intended systems are this exact system.",
    "replaced span": "the replaced pipe A-B contributes no stiffness, self-weight or load (execution/PKG-03/DEL-03-06/"
                     "ScopeOfWork.md, Replaced span; AC-004). No case here has self-weight or thermal load.",
    "free system": "node A anchored on all six DOFs with zero prescribed motion; the free DOFs are B's six in the "
                   "order UX, UY, UZ, RX, RY, RZ; an active stop makes B's UZ a prescribed row at its ground 0.",
    "scaling": "row exponent e_i = -floor(floor(log2 K_ii) / 2), K_s = D K D with D = diag(2^e_i) "
               "(core/solver/frame_kernel/src/structural.rs prepare_bound and binary_exponent); an even force scale "
               "2^b shifts every e_i by -b/2 and leaves K_s unchanged (structural.rs unscale_structural_solution "
               "comment).",
    "estimator": "Hager iteration from x = e/n, at most five passes, stopping on no increase, on |z_j| <= z.x or a "
                 "repeated index (ties: the last maximal index), then the alternating vector (-1)^i (1 + i/(n-1)) "
                 "scaled by 2/(3n); est = the larger (structural.rs ConditionMatrix::estimate_condition). The replay "
                 "below is in exact arithmetic; the product's is binary64.",
    "premise": "Lambda = 1024 est / (1 - 1024 est eps_E), used as the bound on ||K_s^-1||_inf "
               "(core/solver/frame_kernel/src/structural/formation_check.rs, after the second correction).",
    "loads": "each load term enters the intended force exactly: a DOF's force is the exact sum of its terms "
             "(core/solver/frame_kernel/src/load_ledger.rs accumulate_dof).",
    "one-way law": "a stop bearing with a positive (negative) reaction is admissible active when its reaction "
                   "K u - f at its DOF is > 0 (< 0), and admissible inactive when its displacement is > 0 (< 0) "
                   "with ground 0 (core/solver/nonlinear_integration/src/confirmation.rs support_reason; "
                   "execution/PKG-04/DEL-04-04/ScopeOfWork.md one-way law).",
}


class CheckFailure(Exception):
    pass


def check(condition, what):
    if not condition:
        raise CheckFailure(what)


def exact_binary64(value):
    f = float(value)
    return math.isfinite(f) and F(f) == value


def binary_exponent(value):
    check(exact_binary64(value) and value > 0, f"diagonal {value} is a positive binary64")
    return math.frexp(float(value))[1] - 1


def identity(n):
    return [[F(int(i == j)) for j in range(n)] for i in range(n)]


def matmul(a, b):
    return [[sum((a[i][k] * b[k][j] for k in range(len(b))), F(0)) for j in range(len(b[0]))] for i in range(len(a))]


def transpose(a):
    return [list(row) for row in zip(*a)]


def matvec(a, x):
    return [sum((a[i][j] * x[j] for j in range(len(x))), F(0)) for i in range(len(a))]


def inverse(a):
    n = len(a)
    m = [row[:] + identity(n)[i] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        pivot = m[c][c]
        m[c] = [v / pivot for v in m[c]]
        for r in range(n):
            if r != c and m[r][c] != 0:
                factor = m[r][c]
                m[r] = [x - factor * y for x, y in zip(m[r], m[c])]
    return [row[n:] for row in m]


def ldl_pivots(a):
    n = len(a)
    a = [row[:] for row in a]
    pivots = []
    for k in range(n):
        pivots.append(a[k][k])
        for i in range(k + 1, n):
            factor = a[i][k] / a[k][k]
            for j in range(k + 1, n):
                a[i][j] -= factor * a[k][j]
    return pivots


def inf_norm(a):
    return max(sum((abs(v) for v in row), F(0)) for row in a)


def one_norm(a):
    return inf_norm(transpose(a))


def skew(v):
    return [[F(0), -v[2], v[1]], [v[2], F(0), -v[0]], [-v[1], v[0], F(0)]]


def hager_exact(ainv):
    """The documented estimator, with exact solves y = A^-1 x."""
    n = len(ainv)
    trace = []
    x = [F(1, n)] * n
    estimate, previous = F(0), n
    for _ in range(5):
        y = matvec(ainv, x)
        current = sum((abs(v) for v in y), F(0))
        trace.append(current)
        if current <= estimate and estimate != 0:
            break
        estimate = current
        signs = [F(1) if v >= 0 else F(-1) for v in y]
        z = matvec(ainv, signs)
        top = max(abs(v) for v in z)
        j = max(i for i in range(n) if abs(z[i]) == top)  # Rust max_by: the last maximal element
        dot = sum((a * b for a, b in zip(z, x)), F(0))
        if abs(z[j]) <= dot or j == previous:
            break
        previous = j
        x = [F(0)] * n
        x[j] = F(1)
    alt = [F(1)] if n == 1 else [(1 if i % 2 == 0 else -1) * (1 + F(i, n - 1)) for i in range(n)]
    alt_estimate = sum((abs(v) for v in matvec(ainv, alt)), F(0)) * 2 / (3 * n)
    return max(estimate, alt_estimate), trace, alt_estimate


def text(value):
    return str(value.numerator) if value.denominator == 1 else f"{value.numerator}/{value.denominator}"


def num(value):
    """The exact value and its nearest binary64."""
    return {"exact": text(value), "binary64": float(value)}


def json_number(value):
    check(exact_binary64(value), f"document value {value} is exact in binary64")
    f = float(value)
    return int(f) if f == int(f) and abs(f) < 2 ** 53 else f


# ---- formation ----------------------------------------------------------------------------------------
def formed():
    out = {}
    # The free-node target M = I - (1 - LAMBDA) W W^T, an exact SPD matrix with eigenvalue LAMBDA along W.
    check(sum((w * w for w in W), F(0)) == 1, "W is a unit vector")
    check(sum(W) == 0, "W is orthogonal to the vector of ones")
    alt = [(1 if i % 2 == 0 else -1) * (1 + F(i, 5)) for i in range(6)]
    check(sum((a * w for a, w in zip(alt, W)), F(0)) == 0, "W is orthogonal to the alternating vector")
    m = [[F(int(i == j)) - (1 - LAMBDA) * W[i] * W[j] for j in range(6)] for i in range(6)]
    k_free = [[K0 * v for v in row] for row in m]
    # B for the documented kinematics: Q = I, a_i = a_j = 0, r = (L, 0, 0).
    r = [L, F(0), F(0)]
    half = skew([v / 2 for v in r])
    b = [[F(0)] * 12 for _ in range(6)]
    for axis in range(3):
        b[axis][axis] = F(-1)
        b[axis][6 + axis] = F(1)
        b[3 + axis][3 + axis] = F(-1)
        b[3 + axis][9 + axis] = F(1)
        for c in range(3):
            b[axis][3 + c] = half[axis][c]
            b[axis][9 + c] = half[axis][c]
    b_j = [row[6:] for row in b]
    # The authored K (= H, since Ls = 1): K = B_j^-T K_free B_j^-1.
    b_j_inv = inverse(b_j)
    k = matmul(matmul(transpose(b_j_inv), k_free), b_j_inv)
    check(k == transpose(k), "K is symmetric")
    check(all(p > 0 for p in ldl_pivots(k)), "K is positive definite (exact LDL^T)")
    ke = matmul(matmul(transpose(b), k), b)
    for name, mat in (("B", b), ("K", k), ("Ke", ke)):
        check(all(exact_binary64(v) for row in mat for v in row), f"every entry of {name} is exact in binary64")
    check([row[6:] for row in ke[6:]] == k_free, "the free block of Ke is K_free")
    # Rigid-body check: a rigid translation and rotation of both ends gives no force.
    translation_x = [F(v) for v in (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)]
    rotation_z = [F(0)] * 5 + [F(1), F(0), L] + [F(0)] * 3 + [F(1)]  # theta_z = 1 about A: u_B = (0, L, 0)
    for d in (translation_x, rotation_z):
        check(all(v == 0 for v in matvec(ke, d)), "rigid-body motion gives no force")
    out.update(m=m, k_free=k_free, k=k, ke=ke, b=b, k_free_inv_zz=inverse(k_free)[STOP][STOP])
    # Scaling of the free system (unscaled K_free; an even force scale changes nothing).
    exps = [-(binary_exponent(k_free[i][i]) // 2) for i in range(6)]
    d = [F(2) ** e for e in exps]
    k_s = [[d[i] * k_free[i][j] * d[j] for j in range(6)] for i in range(6)]
    check(all(1 <= k_s[i][i] < 4 for i in range(6)), "scaled diagonals lie in [1, 4)")
    for b2 in (-40, 2, 40):
        shifted = [-(binary_exponent(F(2) ** b2 * k_free[i][i]) // 2) for i in range(6)]
        check(shifted == [e - b2 // 2 for e in exps], "an even force scale shifts every exponent by -b/2")
    k_s_inv = inverse(k_s)
    est, trace, alt_est = hager_exact(k_s_inv)
    true_norm = inf_norm(k_s_inv)
    check(true_norm == one_norm(k_s_inv), "the scaled inverse is symmetric, so its 1- and inf-norms agree")
    out.update(exps=exps, k_s=k_s, k_s_inv=k_s_inv, est=est, trace=trace, alt_est=alt_est, true_norm=true_norm)
    return out


def request(variant, fm):
    spec = VARIANTS[variant]
    loads = matvec(fm["k_free"], X_MAIN)
    primitives = []
    for i, value in enumerate(loads):
        if value == 0:
            continue
        direction, dimension, unit = LOADS[i]
        primitives.append({
            "id": f"load:MAIN-{DOFS[i]}", "category": "occasional", "target": {"type": "node", "node": "node:B"},
            "direction": direction, "magnitude": {"value": json_number(value), "unit": unit},
            "dimension": dimension, "provenance": PROVENANCE})
    if spec["tiny"] is not None:
        primitives.append({
            "id": "load:TINY-UZ", "category": "occasional", "target": {"type": "node", "node": "node:B"},
            "direction": "global_z", "magnitude": {"value": json_number(spec["tiny"]), "unit": "N"},
            "dimension": "force", "provenance": PROVENANCE})
    upper = [json_number(fm["k"][i][j]) for i in range(6) for j in range(i, 6)]
    axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    end = lambda node: {"node_ref": node, "initial_node_axes_global": axes,
                        "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}}
    supports = [{"id": "support:ANCHOR", "node": "node:A", "family": "anchor",
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}]
    if spec["stop"]:
        supports.append({"id": "support:STOP", "node": "node:B", "family": "nonlinear", "restraints": [],
                         "nonlinear": {"behavior": "one_way", "dof": "UZ", "initial_state": INITIAL_STATE,
                                       "active_when": spec["stop"]},
                         "provenance": PROVENANCE})
    model = {
        "schema_version": "0.9.0",
        "document_kind": "openpipestress.product_preview.model",
        "data_boundary": {
            "public_examples_policy": "invented_or_cleared_data_only",
            "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
            "private_data_policy": "no_private_project_data",
            "professional_boundary": "technical_preview_requires_human_engineering_review"},
        "project": {"id": f"project:inverse-norm-witness-{variant.replace('_', '-')}",
                    "name": f"Inverse-norm witness, {variant}",
                    "description": "Invented known-limitation witness of the integrity check's inverse-norm premise "
                                   "(DEL-04-01 OUT-001); see validation/references/inverse_norm_witness/README.md.",
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "MPa"}},
        "materials": [{"id": "material:ref", "label": "Invented reference material (the replaced span's; unused)",
                       "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": {"value": 200, "unit": "GPa"},
                       "poisson_ratio": {"value": 0.25, "unit": "1"},
                       "shear_modulus": {"value": 80, "unit": "GPa"},
                       "provenance": PROVENANCE,
                       "temperature_points": [{"id": "point:installed", "temperature": {"value": 20, "unit": "degC"},
                                               "elastic_modulus": {"value": 200, "unit": "GPa"},
                                               "poisson_ratio": {"value": 0.25, "unit": "1"}}]}],
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:A", "label": "A", "position": {"x": 0, "y": 0, "z": 0}, "provenance": PROVENANCE},
                  {"id": "node:B", "label": "B", "position": {"x": json_number(L), "y": 0, "z": 0},
                   "provenance": PROVENANCE}],
        "pipe_segments": [{"id": "pipe:AB", "label": "AB (replaced by the joint)", "from": "node:A", "to": "node:B",
                           "section": {"outside_diameter": {"value": 0.168, "unit": "m"},
                                       "wall_thickness": {"value": 0.007, "unit": "m"},
                                       "corrosion_allowance": {"value": 0, "unit": "m"}},
                           "material": "material:ref", "y_reference": {"x": 0, "y": 0, "z": 1},
                           "provenance": PROVENANCE}],
        "supports": supports,
        "components": [{
            "id": "component:JOINT", "label": "Invented expansion joint (objective connector)",
            "kind": "expansion_joint", "node": "node:B", "provenance": PROVENANCE,
            "objective_connector": {
                "version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
                "end_i": end("node:A"), "end_j": end("node:B"), "connector_axes_global": axes,
                "installed_reference_temperature": {"value": 20, "unit": "degC"},
                "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
                "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"},
                          "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
                "stiffness": {
                    "version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                    "translation_scale": {"value": 1, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"},
                    "coefficient_unit": "N*m", "upper_triangle": upper,
                    "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                    "provenance": {"source_reference": "invented coupled work matrix with one weak direction "
                                                       "(inverse-norm witness); not manufacturer data",
                                   "measurement_restraints": "invented",
                                   "basis_transform_reference": "none (authored directly in the midpoint basis)"}},
                "topology": {"type": "replaces_span", "span_ref": "pipe:AB"},
                "calibration": {"kind": "constant_structural_elasticity_v1",
                                "includes_pressure_dependent_tangent": False,
                                "installed_geometry": "authored model geometry"},
                "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
                "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented",
                               "basis_transform_reference": "none",
                               "validity_statement": "invented analytical witness; no applicability claim"}}}],
        "load_cases": [{"id": "load:WITNESS", "label": "WITNESS", "kind": "primitive_user_load",
                        "status": "preview_only", "primitive_loads": primitives, "provenance": PROVENANCE,
                        "pressure_regions": []}],
        "combinations": [],
        "diagnostics": [],
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
    }
    return {"model": model, "materials": []}, loads


def rendered_request(doc):
    return (json.dumps(doc, indent=1) + "\n").encode("utf-8")


def solve_state(fm, force, active):
    """Exact solution of the free system with the stop free (inactive) or prescribed at 0 (active)."""
    k = fm["k_free"]
    free = [i for i in range(6) if not (active and i == STOP)]
    sub = [[k[i][j] for j in free] for i in free]
    x_free = matvec(inverse(sub), [force[i] for i in free])
    x = [F(0)] * 6
    for i, v in zip(free, x_free):
        x[i] = v
    reaction = matvec(k, x)[STOP] - force[STOP]
    return x, reaction


def build():
    fm = formed()
    check(fm["true_norm"] == F(2 ** 18), "the scaled inverse norm is 2^18")
    check(fm["est"] == 1, "the exact replay of the estimator gives 1")
    out_variants, documents = {}, {}
    for variant, spec in VARIANTS.items():
        doc, main = request(variant, fm)
        documents[variant] = rendered_request(doc)
        force = main[:]
        if spec["tiny"] is not None:
            force[STOP] += spec["tiny"]
        x_in, _ = solve_state(fm, force, active=False)
        check(matvec(fm["k_free"], x_in) == force, "the inactive solution satisfies the free equations")
        # Anchor reaction at A (support-on-pipe, K u - f with no load at A): the A-rows of Ke times B's motion.
        anchor = [sum((fm["ke"][i][6 + j] * x_in[j] for j in range(6)), F(0)) for i in range(6)]
        entry = {
            "request": f"requests/{variant}.request.json",
            "request_sha256": hashlib.sha256(documents[variant]).hexdigest(),
            "loads": {"main": {DOFS[i]: num(v) for i, v in enumerate(main) if v != 0},
                      "tiny_uz": None if spec["tiny"] is None else num(spec["tiny"])},
        }
        if spec["stop"] is None:
            check(x_in == X_MAIN, "the linear displacement is X_MAIN")
            entry["exact_displacement_B"] = {DOFS[i]: num(v) for i, v in enumerate(x_in)}
            entry["exact_anchor_reaction_A"] = {DOFS[i]: num(v) for i, v in enumerate(anchor)}
        else:
            sign = 1 if spec["stop"] == "positive_reaction" else -1
            x_act, reaction = solve_state(fm, force, active=True)
            inactive_ok = sign * x_in[STOP] > 0
            active_ok = sign * reaction > 0
            check(active_ok and not inactive_ok, f"{variant}: exactly one admissible state, active")
            check(x_in[STOP] == spec["tiny"] * fm["k_free_inv_zz"], f"{variant}: the stop moves by the tiny term alone")
            entry.update({
                "stop": {"support": "support:STOP", "node": "node:B", "dof": "UZ", "active_when": spec["stop"],
                         "ground": "0 (no imposed movement)", "initial_state": INITIAL_STATE},
                "exact_state": "active",
                "inactive_state": {"displacement_B": {DOFS[i]: num(v) for i, v in enumerate(x_in)},
                                   "stop_displacement": num(x_in[STOP]),
                                   "admissible": inactive_ok,
                                   "why": "the stop's displacement is on its closed side"},
                "active_state": {"displacement_B": {DOFS[i]: num(v) for i, v in enumerate(x_act)},
                                 "stop_reaction": num(reaction), "admissible": active_ok},
            })
        out_variants[variant] = entry
    return fm, out_variants, documents


def render(fm, variants):
    k_s = fm["k_s"]
    return {
        "schema": "swbpipe.validation_reference/inverse_norm_witness/1",
        "status": "known-limitation witness of an unproved premise; not a requirement; no product conformance, "
                  "acceptance or release is asserted",
        "independence": "Python standard library only; no product code, product output, fixture or schema read, "
                        "imported or run",
        "governing_text": ["execution/PKG-04/DEL-04-01/ScopeOfWork.md: REQ-015, REQ-016, AC-013 and the OUT-001 row "
                           "of REQ-015", "docs/RELEASE_TARGET_v1.0.md exit criterion 18"],
        "assumptions": ASSUMPTIONS,
        "construction": {
            "K0": num(K0), "weak_eigenvalue_of_M": num(LAMBDA), "weak_direction": [text(w) for w in W],
            "chord_m": num(L), "x_main": [text(v) for v in X_MAIN],
            "free_order": DOFS,
            "K_free": [[text(v) for v in row] for row in fm["k_free"]],
            "connector_K_equals_H": [[text(v) for v in row] for row in fm["k"]],
        },
        "scaled_system": {
            "row_exponents": fm["exps"],
            "K_s": [[text(v) for v in row] for row in k_s],
            "inverse_norm_inf": num(fm["true_norm"]),
            "estimator_exact_replay": {"estimate": num(fm["est"]),
                                       "hager_passes": [text(v) for v in fm["trace"]],
                                       "alternating_vector_estimate": num(fm["alt_est"])},
            "premise_1024_est": num(1024 * fm["est"]),
            "ratio_1024_est_to_true": num(1024 * fm["est"] / fm["true_norm"]),
        },
        "variants": variants,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite the requests, references.json and SHA256SUMS")
    mode.add_argument("--check", action="store_true", help="rebuild in memory and compare bytes (the default)")
    args = parser.parse_args()
    try:
        fm, variants, documents = build()
    except CheckFailure as exc:
        print(f"check failed: {exc}", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[1]
    outputs = {f"requests/{v}.request.json": d for v, d in documents.items()}
    outputs["references.json"] = (json.dumps(render(fm, variants), indent=1) + "\n").encode("utf-8")
    files = {"README.md": (root / "README.md").read_bytes(),
             "generators/inverse_norm_witness_reference.py": Path(__file__).read_bytes(), **outputs}
    outputs["SHA256SUMS"] = "".join(f"{hashlib.sha256(d).hexdigest()}  {n}\n"
                                    for n, d in sorted(files.items())).encode("ascii")
    if args.write:
        for name, data in outputs.items():
            (root / name).parent.mkdir(parents=True, exist_ok=True)
            (root / name).write_bytes(data)
    else:
        diff = [n for n, d in outputs.items() if not (root / n).is_file() or (root / n).read_bytes() != d]
        if diff:
            print("reference bytes differ: " + ", ".join(diff), file=sys.stderr)
            return 1
    print(f"{len(variants)} variants; ||K_s^-1||_inf = {fm['true_norm']}, exact estimate {fm['est']}; "
          f"frozen bytes {'written' if args.write else 'match'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
