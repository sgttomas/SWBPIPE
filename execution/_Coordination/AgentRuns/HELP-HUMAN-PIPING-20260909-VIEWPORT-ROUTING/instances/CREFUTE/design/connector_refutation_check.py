#!/usr/bin/env python3
"""Deterministic checks for the proposed linear two-frame connector."""

import json
import math


def add(a, b):
    return [x + y for x, y in zip(a, b)]


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def scale(s, a):
    return [s * x for x in a]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    return scale(1.0 / norm(a), a)


def transpose(a):
    return [list(row) for row in zip(*a)]


def matmul(a, b):
    bt = transpose(b)
    return [[dot(row, col) for col in bt] for row in a]


def matvec(a, x):
    return [dot(row, x) for row in a]


def identity(n):
    return [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]


def skew(r):
    x, y, z = r
    return [[0.0, -z, y], [z, 0.0, -x], [-y, x, 0.0]]


def block_diag(blocks):
    rows = sum(len(b) for b in blocks)
    cols = sum(len(b[0]) for b in blocks)
    out = [[0.0 for _ in range(cols)] for _ in range(rows)]
    row0 = col0 = 0
    for block in blocks:
        for i, row in enumerate(block):
            for j, value in enumerate(row):
                out[row0 + i][col0 + j] = value
        row0 += len(block)
        col0 += len(block[0])
    return out


def solve(a, b):
    work = [row[:] + [rhs] for row, rhs in zip(a, b)]
    n = len(b)
    for col in range(n):
        pivot = max(range(col, n), key=lambda row: abs(work[row][col]))
        work[col], work[pivot] = work[pivot], work[col]
        divisor = work[col][col]
        if abs(divisor) < 1e-14:
            raise ValueError("singular system")
        work[col] = [value / divisor for value in work[col]]
        for row in range(n):
            if row == col:
                continue
            factor = work[row][col]
            work[row] = [x - factor * y for x, y in zip(work[row], work[col])]
    return [work[i][-1] for i in range(n)]


def max_abs(values):
    return max((abs(value) for value in values), default=0.0)


def flatten(matrix):
    return [value for row in matrix for value in row]


def matrix_sub(a, b):
    return [[x - y for x, y in zip(arow, brow)] for arow, brow in zip(a, b)]


def matrix_scale(s, a):
    return [[s * value for value in row] for row in a]


def connector_b(ai, aj, r0, q0_axes):
    qt = transpose(q0_axes)
    i3 = identity(3)
    sai = skew(ai)
    saj = skew(aj)
    sr = skew(r0)
    global_bt = []
    for row in range(3):
        global_bt.append(
            [-i3[row][col] for col in range(3)]
            + [sai[row][col] + 0.5 * sr[row][col] for col in range(3)]
            + [i3[row][col] for col in range(3)]
            + [-saj[row][col] + 0.5 * sr[row][col] for col in range(3)]
        )
    global_br = []
    for row in range(3):
        global_br.append(
            [0.0] * 3
            + [-i3[row][col] for col in range(3)]
            + [0.0] * 3
            + [i3[row][col] for col in range(3)]
        )
    return matmul(block_diag([qt, qt]), global_bt + global_br)


def raw_relative_b(q0_axes):
    qt = transpose(q0_axes)
    i3 = identity(3)
    global_bt = []
    global_br = []
    for row in range(3):
        global_bt.append(
            [-i3[row][col] for col in range(3)]
            + [0.0] * 3
            + [i3[row][col] for col in range(3)]
            + [0.0] * 3
        )
        global_br.append(
            [0.0] * 3
            + [-i3[row][col] for col in range(3)]
            + [0.0] * 3
            + [i3[row][col] for col in range(3)]
        )
    return matmul(block_diag([qt, qt]), global_bt + global_br)


def axes_from_x_and_y_reference(x_axis, y_reference):
    ex = unit(x_axis)
    ey = unit(sub(y_reference, scale(dot(y_reference, ex), ex)))
    ez = cross(ex, ey)
    return [[ex[row], ey[row], ez[row]] for row in range(3)]


def rodrigues(axis, angle):
    axis = unit(axis)
    s = skew(axis)
    aa = [[axis[i] * axis[j] for j in range(3)] for i in range(3)]
    c = math.cos(angle)
    sn = math.sin(angle)
    return [
        [c * identity(3)[i][j] + sn * s[i][j] + (1.0 - c) * aa[i][j] for j in range(3)]
        for i in range(3)
    ]


def energy(b, k, q0, d):
    dq = sub(matvec(b, d), q0)
    return 0.5 * dot(dq, matvec(k, dq))


def force(b, k, q0, d):
    return matvec(transpose(b), matvec(k, sub(matvec(b, d), q0)))


def jacobi_eigenvalues(a):
    a = [row[:] for row in a]
    n = len(a)
    for _ in range(100 * n * n):
        p, q = max(
            ((i, j) for i in range(n) for j in range(i + 1, n)),
            key=lambda pair: abs(a[pair[0]][pair[1]]),
        )
        if abs(a[p][q]) < 1e-11:
            break
        phi = 0.5 * math.atan2(2.0 * a[p][q], a[q][q] - a[p][p])
        c, s = math.cos(phi), math.sin(phi)
        app, aqq, apq = a[p][p], a[q][q], a[p][q]
        for r in range(n):
            if r in (p, q):
                continue
            arp, arq = a[r][p], a[r][q]
            a[r][p] = a[p][r] = c * arp - s * arq
            a[r][q] = a[q][r] = s * arp + c * arq
        a[p][p] = c * c * app - 2.0 * s * c * apq + s * s * aqq
        a[q][q] = s * s * app + 2.0 * s * c * apq + c * c * aqq
        a[p][q] = a[q][p] = 0.0
    return sorted(a[i][i] for i in range(n))


def main():
    xi = [2.0, -1.0, 0.5]
    xj = [5.0, 1.0, 2.0]
    ai = [0.2, -0.1, 0.3]
    aj = [-0.15, 0.25, -0.05]
    pi0 = add(xi, ai)
    pj0 = add(xj, aj)
    r0 = sub(pj0, pi0)
    q_axes = axes_from_x_and_y_reference(r0, [-0.4, 0.8, 0.3])
    b = connector_b(ai, aj, r0, q_axes)

    # Positive-definite coupled Kc=A^T A in a fixed SI numerical basis.
    a = [
        [3.0, 0.0, 0.0, 0.40, 0.0, 0.0],
        [0.2, 2.4, 0.0, 0.0, -0.25, 0.0],
        [0.0, -0.1, 2.1, 0.0, 0.0, 0.35],
        [0.0, 0.0, 0.0, 1.7, 0.15, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.4, -0.2],
        [0.0, 0.0, 0.0, 0.0, 0.0, 1.2],
    ]
    k = matmul(transpose(a), a)
    q0_zero = [0.0] * 6
    q0_nonzero = [0.03, -0.02, 0.01, 0.004, -0.003, 0.002]

    rigid = []
    for axis in identity(3):
        rigid.append(axis + [0.0] * 3 + axis + [0.0] * 3)
    origin = [-1.2, 0.7, 2.4]
    for omega in identity(3):
        rigid.append(
            cross(omega, sub(xi, origin))
            + omega
            + cross(omega, sub(xj, origin))
            + omega
        )
    rigid_q_error = max(max_abs(matvec(b, mode)) for mode in rigid)
    rigid_f_error = max(max_abs(force(b, k, q0_zero, mode)) for mode in rigid)
    rigid_energy_error = max(abs(energy(b, k, q0_zero, mode)) for mode in rigid)

    raw = raw_relative_b(q_axes)
    raw_rotation_norms = [norm(matvec(raw, mode)[:3]) for mode in rigid[3:]]

    # Exact B derivative via centered finite differences.
    d = [0.02, -0.03, 0.01, 0.004, -0.002, 0.003, -0.01, 0.025, -0.015, -0.001, 0.005, -0.004]
    eps = 1e-7
    fd_b = [[0.0] * 12 for _ in range(6)]
    for col in range(12):
        plus, minus = d[:], d[:]
        plus[col] += eps
        minus[col] -= eps
        derivative = scale(0.5 / eps, sub(matvec(b, plus), matvec(b, minus)))
        for row in range(6):
            fd_b[row][col] = derivative[row]
    b_fd_error = max_abs(flatten(matrix_sub(fd_b, b)))

    f = force(b, k, q0_nonzero, d)
    fd_f = []
    for col in range(12):
        plus, minus = d[:], d[:]
        plus[col] += eps
        minus[col] -= eps
        fd_f.append((energy(b, k, q0_nonzero, plus) - energy(b, k, q0_nonzero, minus)) / (2.0 * eps))
    virtual_work_error = max_abs(sub(fd_f, f))

    ke = matmul(transpose(b), matmul(k, b))
    ke_symmetry_error = max_abs(flatten(matrix_sub(ke, transpose(ke))))
    ke_eigenvalues = jacobi_eigenvalues(ke)

    # One displacement solution for each unit generalized connector component.
    bbt = matmul(b, transpose(b))
    component_errors = []
    for component in range(6):
        target = [1.0 if i == component else 0.0 for i in range(6)]
        minimum_norm_d = matvec(transpose(b), solve(bbt, target))
        component_errors.append(max_abs(sub(matvec(b, minimum_norm_d), target)))

    # Internal resultant balance about multiple arbitrary origins.
    nodal_force_i, nodal_moment_i = f[0:3], f[3:6]
    nodal_force_j, nodal_moment_j = f[6:9], f[9:12]
    force_balance = norm(add(nodal_force_i, nodal_force_j))
    moment_balance = []
    for check_origin in ([0.0, 0.0, 0.0], [7.2, -3.1, 4.4], [-20.0, 8.0, 11.0]):
        resultant = add(
            add(cross(sub(xi, check_origin), nodal_force_i), nodal_moment_i),
            add(cross(sub(xj, check_origin), nodal_force_j), nodal_moment_j),
        )
        moment_balance.append(norm(resultant))

    # Global translation/rotation of the whole installation preserves local q.
    rotation = rodrigues([0.3, -0.5, 0.8], 0.73)
    translation = [4.0, -2.0, 7.5]
    xip = add(matvec(rotation, xi), translation)
    xjp = add(matvec(rotation, xj), translation)
    aip, ajp = matvec(rotation, ai), matvec(rotation, aj)
    r0p = sub(add(xjp, ajp), add(xip, aip))
    q_axes_p = matmul(rotation, q_axes)
    bp = connector_b(aip, ajp, r0p, q_axes_p)
    dof_rotation = block_diag([rotation, rotation, rotation, rotation])
    installation_covariance_error = max_abs(flatten(matrix_sub(matmul(bp, dof_rotation), b)))

    # Endpoint reversal: Q'=Q D, D=diag(-1,+1,-1), T=diag(-D,-D).
    d_axis = [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]
    q_axes_rev = matmul(q_axes, d_axis)
    b_rev = connector_b(aj, ai, scale(-1.0, r0), q_axes_rev)
    p_dof = [[0.0] * 12 for _ in range(12)]
    for row in range(6):
        p_dof[row][row + 6] = 1.0
        p_dof[row + 6][row] = 1.0
    minus_d = matrix_scale(-1.0, d_axis)
    t_q = block_diag([minus_d, minus_d])
    endpoint_b_error = max_abs(flatten(matrix_sub(matmul(b_rev, p_dof), matmul(t_q, b))))
    k_rev = matmul(t_q, matmul(k, transpose(t_q)))
    q0_rev = matvec(t_q, q0_nonzero)
    d_rev = matvec(p_dof, d)
    endpoint_energy_error = abs(energy(b_rev, k_rev, q0_rev, d_rev) - energy(b, k, q0_nonzero, d))
    endpoint_force_error = max_abs(sub(force(b_rev, k_rev, q0_rev, d_rev), matvec(p_dof, f)))

    # q0 semantics: rigid motion leaves q invariant at zero, but not f or Pi.
    nonzero_q0_rigid_force_norm = norm(force(b, k, q0_nonzero, rigid[3]))
    nonzero_q0_rigid_energy = energy(b, k, q0_nonzero, rigid[3])
    deformed_plus_rigid = add(d, rigid[5])
    rigid_superposition_q_error = max_abs(sub(matvec(b, deformed_plus_rigid), matvec(b, d)))
    rigid_superposition_energy_error = abs(
        energy(b, k, q0_nonzero, deformed_plus_rigid) - energy(b, k, q0_nonzero, d)
    )
    rigid_superposition_force_error = max_abs(
        sub(force(b, k, q0_nonzero, deformed_plus_rigid), force(b, k, q0_nonzero, d))
    )

    output = {
        "basis": "fixed SI numerical basis; radians treated dimensionless",
        "geometry": {"xi": xi, "xj": xj, "ai": ai, "aj": aj, "r0": r0},
        "checks": {
            "six_rigid_modes_stress_free_max_q": rigid_q_error,
            "six_rigid_modes_stress_free_max_force": rigid_f_error,
            "six_rigid_modes_stress_free_max_energy": rigid_energy_error,
            "raw_relative_translation_rigid_rotation_qt_norms": raw_rotation_norms,
            "finite_difference_B_max_error": b_fd_error,
            "finite_difference_energy_gradient_max_error": virtual_work_error,
            "Ke_symmetry_max_error": ke_symmetry_error,
            "Ke_eigenvalues": ke_eigenvalues,
            "single_generalized_component_max_errors": component_errors,
            "force_balance_norm": force_balance,
            "moment_balance_norms_three_origins": moment_balance,
            "translated_rotated_installation_covariance_max_error": installation_covariance_error,
            "endpoint_reversal_B_max_error": endpoint_b_error,
            "endpoint_reversal_energy_error": endpoint_energy_error,
            "endpoint_reversal_force_max_error": endpoint_force_error,
            "nonzero_q0_rigid_force_norm": nonzero_q0_rigid_force_norm,
            "nonzero_q0_rigid_energy": nonzero_q0_rigid_energy,
            "nonzero_q0_rigid_superposition_q_max_error": rigid_superposition_q_error,
            "nonzero_q0_rigid_superposition_energy_error": rigid_superposition_energy_error,
            "nonzero_q0_rigid_superposition_force_max_error": rigid_superposition_force_error,
        },
        "pass_thresholds": {
            "algebraic_max_abs": 1e-8,
            "finite_difference_energy_gradient_max_abs": 1e-7,
            "PSD_min_eigenvalue": -1e-9,
            "raw_mutation_each_rotation_min_norm": 1e-6,
            "nonzero_q0_preload_min_force_norm": 1e-6,
        },
    }
    checks = output["checks"]
    output["status"] = "PASS" if (
        checks["six_rigid_modes_stress_free_max_q"] <= 1e-8
        and checks["six_rigid_modes_stress_free_max_force"] <= 1e-8
        and checks["six_rigid_modes_stress_free_max_energy"] <= 1e-8
        and min(checks["raw_relative_translation_rigid_rotation_qt_norms"]) >= 1e-6
        and checks["finite_difference_B_max_error"] <= 1e-8
        and checks["finite_difference_energy_gradient_max_error"] <= 1e-7
        and checks["Ke_symmetry_max_error"] <= 1e-8
        and min(checks["Ke_eigenvalues"]) >= -1e-9
        and max(checks["single_generalized_component_max_errors"]) <= 1e-8
        and checks["force_balance_norm"] <= 1e-8
        and max(checks["moment_balance_norms_three_origins"]) <= 1e-8
        and checks["translated_rotated_installation_covariance_max_error"] <= 1e-8
        and checks["endpoint_reversal_B_max_error"] <= 1e-8
        and checks["endpoint_reversal_energy_error"] <= 1e-8
        and checks["endpoint_reversal_force_max_error"] <= 1e-8
        and checks["nonzero_q0_rigid_force_norm"] >= 1e-6
        and checks["nonzero_q0_rigid_superposition_q_max_error"] <= 1e-8
        and checks["nonzero_q0_rigid_superposition_energy_error"] <= 1e-8
        and checks["nonzero_q0_rigid_superposition_force_max_error"] <= 1e-8
    ) else "FAIL"
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
