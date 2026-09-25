"""Static Python range diagnosis; never invokes Cargo, Rust, Git or the solver.

The Scaled transcription below is diagnostic implementation tracing, not an
independent physics oracle. The independent oracle is the preserved Fraction
reference, re-evaluated from the unchanged represented source inputs.
"""
from fractions import Fraction as F
from pathlib import Path
import hashlib
import json
import math
import struct
import sys

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "projects/chirality-piping/core").is_dir())
OLD = Path("/private/tmp/piping-pressure-stress-20260924")
RUN = Path("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24")
PHYS = RUN / "PHYSICS_MANAGER"
REFERENCE = PHYS / "INDEPENDENT_REVIEW/FREEZE_03_BACKCHECK/independent_arithmetic_probe.json"

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]

def scaled(x):
    return math.frexp(x)

def mul(a, b):
    m, e = math.frexp(a[0] * b[0])
    return m, e + a[1] + b[1]

def add(a, b):
    e = max(a[1], b[1])
    m, k = math.frexp(math.ldexp(a[0], a[1] - e) + math.ldexp(b[0], b[1] - e))
    return m, e + k

def sub(a, b):
    return add(a, (-b[0], b[1]))

def out(a):
    return math.ldexp(*a)

od, wall, length, elastic, nu, pressure = 4e-77, 1e-77, 1.0, 1.0, 0.1, 4.7e-170
reference = json.loads((OLD / REFERENCE).read_text())["membrane_publication_arithmetic"]
expected = float(2 * F(nu) * F(pressure) * (F(od) / 2 - F(wall)) ** 2 / (F(wall) * (F(od) - F(wall))))
assert expected == reference["stable_membrane_Pa"] == 3.1333333333333337e-171
reference_force = float(2 * F(nu) * F(pressure) * F(math.pi) * (F(od) / 2 - F(wall)) ** 2)
assert bits(reference_force) == 1

# SourceAnnulus geometry's actual staged binary64 operations, without dynamic
# product execution. This traces the pre-gate arithmetic, not a new oracle.
area_scaled = mul(mul(scaled(math.pi), scaled(wall)), sub(scaled(od), scaled(wall)))
ro = mul(scaled(od), scaled(0.5))
ri = sub(ro, scaled(wall))
i_scaled = mul(mul(area_scaled, add(mul(ro, ro), mul(ri, ri))), scaled(0.25))
area, inertia, polar = out(area_scaled), out(i_scaled), out(mul(i_scaled, scaled(2.0)))
shear = elastic / (2.0 * (1.0 + nu))
axial = elastic * area / length
torsion = shear * polar / length
b12, b6, b4, b2 = (k * elastic * inertia / length ** power for k, power in [(12.0, 3), (6.0, 2), (4.0, 1), (2.0, 1)])
k = [[0.0] * 12 for _ in range(12)]
for a, b, value in [(0, 6, axial), (3, 9, torsion)]:
    k[a][a] = k[b][b] = value
    k[a][b] = k[b][a] = -value
for indices, terms in [
    ([1, 5, 7, 11], [[b12, b6, -b12, b6], [b6, b4, -b6, b2], [-b12, -b6, b12, -b6], [b6, b2, -b6, b4]]),
    ([2, 4, 8, 10], [[b12, -b6, -b12, -b6], [-b6, b4, b6, b2], [-b12, b6, b12, b6], [-b6, b2, b6, b4]]),
]:
    for a, row in enumerate(terms):
        for b, value in enumerate(row):
            k[indices[a]][indices[b]] = value
assert all(x == 0.0 or abs(x) >= sys.float_info.min for row in k for x in row)
g = (24.0 * sys.float_info.epsilon / 2.0) / (1.0 - 24.0 * sys.float_info.epsilon / 2.0)
# For this exact identity transform, both sums are abs(K_ij), and the
# first-stage product/sum guards admit every matrix entry.
allowances = []
for a, row in enumerate(k):
    for b, value in enumerate(row):
        magnitude = abs(value)
        bound = g * (magnitude / (1.0 - g) + magnitude) * (1.0 + 64.0 * sys.float_info.epsilon)
        if 0.0 < bound < sys.float_info.min:
            allowances.append({"row": a, "col": b, "local_stiffness": value, "bound": bound, "bound_bits": bits(bound)})
assert allowances[0]["row"] == allowances[0]["col"] == 1

current_paths = [
    "AGENTS.md", "projects/chirality-piping/AGENTS.md", "projects/chirality-piping/loop/LOOP_INIT.md",
    ".agents/skills/software-defect-diagnosis/SKILL.md",
    "projects/chirality-piping/core/product_physics/src/lib.rs",
    "projects/chirality-piping/core/product_physics/src/pressure_runtime.rs",
    "projects/chirality-piping/core/product_physics/src/pressure_exact.rs",
    "projects/chirality-piping/core/product_physics/src/pressure_exact/source_geometry.rs",
    "projects/chirality-piping/core/product_physics/src/annulus_geometry.rs",
    "projects/chirality-piping/core/product_physics/tests/pressure_membrane_range.rs",
    "projects/chirality-piping/core/solver/frame_kernel/src/lib.rs",
    "projects/chirality-piping/core/solver/frame_kernel/src/structural.rs",
    "projects/chirality-piping/core/solver/nonlinear_integration/src/structural_adapter.rs",
    "projects/chirality-piping/core/loads/stress_recovery/src/elastic_extrema.rs",
    str(RUN / "ENGINE_INTEGRATION/product-regressions-03.log"),
]
old_paths = [str(REFERENCE)] + [str(PHYS / p) for p in [
    "MEMBRANE_PUBLICATION/REFERENCE_BINDING_02.json", "MEMBRANE_PUBLICATION/RETURN.md",
    "MEMBRANE_PUBLICATION/REPAIR_BINDING.json", "MEMBRANE_PUBLICATION/red-02.log",
    "MEMBRANE_PUBLICATION/green-01.log", "MEMBRANE_BACKCHECK/RETURN.md",
]]
assert sha(OLD / REFERENCE) == "23db606adf7fe61f4df6031ac6b11b8285b88b12d6dfe381a04faaabd13c349c"
assert sha(OLD / PHYS / "MEMBRANE_PUBLICATION/red-02.log") == "618e937089db6aeadbca224dc0dbbb0916cca9e43b0c856bc62e6ea55cfd93c3"
assert sha(OLD / PHYS / "MEMBRANE_PUBLICATION/green-01.log") == "4086f6863bef956dce8916ed7a7ce4f305ff21f1532e152bda0cf19576ca0a8c"

result = {
    "actor": "/root/physics_resume/membrane_range_join", "actual_parent": "/root/physics_resume",
    "mechanism": "delegated-harness-native TASK, no descendants",
    "method": __doc__, "python_version": sys.version,
    "input": {"OD_m": od, "wall_m": wall, "L_m": length, "E_Pa": elastic, "nu": nu, "p_Pa": pressure, "all_dofs_fixed": True},
    "reference_membrane_pa": expected, "reference_projected_force_bits": bits(reference_force),
    "source_area_m2": area, "source_I_m4": inertia, "source_J_m4": polar,
    "source_G_pa": shear, "axial_stiffness": axial, "torsion_stiffness": torsion,
    "bending_12_stiffness": b12, "all_local_nonzero_entries_normal": True,
    "first_normal_range_failure": allowances[0], "subnormal_allowance_count": len(allowances),
    "normal_min": sys.float_info.min, "gamma_24": g,
    "rounded_force_redivision_pa": reference_force / area,
    "rounded_force_redivision_relative_error": (reference_force / area - expected) / expected,
    "current_root": str(ROOT), "historical_root": str(OLD),
    "supplied_full_TASK_role_origin": str(HERE / "BRIEF.md"),
    "brief_sha256": sha(HERE / "BRIEF.md"),
    "current_input_sha256": {p: sha(ROOT / p) for p in current_paths},
    "historical_input_sha256": {p: sha(OLD / p) for p in old_paths},
    "limits": ["No Rust, Cargo, public solver or native execution by this TASK.", "Historical public red/green remain manager executions.", "Current public refusal is inherited observed log; exact location is a static reconstruction pending parent Rust confirmation.", "No source, guard, tolerance or original input mutation."],
}
(HERE / "range_probe.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: result[k] for k in ["reference_membrane_pa", "reference_projected_force_bits", "source_I_m4", "first_normal_range_failure", "rounded_force_redivision_relative_error"]}, indent=2))
