"""RV116 S-2 illustration (read-only; standard library only).

Applies RV116's proposed S-2 wording to the regenerated DEF-E draft, then rebuilds the
physics-retained-1 table with I96's own `physics_retained_table` (imported from a scratch copy
of b3d_statics.py, unchanged) so that ROOT can see the hash consequences of adopting the
wording verbatim. Illustrative only: the final wording and hashes are I-A's at J1.

Usage: python rv116_s2_illustration.py <P> <regen_dir> <out.json>
"""
import hashlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b3d_statics as b  # I96's generator, scratch copy, unchanged
import rv116_checks as mine  # RV116's own canonicalizer

P, regen, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
defe = json.loads((regen / "retained_precision_prepared_exact_v1.json").read_bytes())
d = json.loads(json.dumps(defe))
d["evidence"] = {
    "pipe_sections": (
        "for the owner case's contract_evidence.exact_cases entry (matched by load_case_id): regenerate "
        "pipe_sections[].As_m2,I_m4,J_m4,Z_m3 as the prepared A,I,J,Z bits; outside_diameter_m, "
        "effective_wall_thickness_m, ro_m, ri_m, Ai_m2, geometry_basis, pipe_id and order unchanged"),
    "pipe_stress_extrema": ("for the owner case's entry: the maximum row family's eight regenerated numeric "
                            "fields; every other key unchanged"),
    "unchanged": (
        "pressure [], connector [], and the owner case's load_case_id, profile_mode, material_basis, "
        "pipe_materials, pressure_rhs_assembly and stress_maximum_coverage (complete) byte-identical to the "
        "ordinary exact envelope; an unselected case's exact_cases entry is byte-identical to the ordinary "
        "exact envelope; no recovery_method member"),
}
d["rows"]["maximum"] = d["rows"]["maximum"].replace(
    "regenerate the eight numeric fields of contract_evidence.exact_cases[].pipe_stress_extrema",
    "regenerate the eight numeric fields of the owner case's contract_evidence.exact_cases[].pipe_stress_extrema")
assert d["rows"]["maximum"] != defe["rows"]["maximum"]
d["scope"]["excludes"] = ["nonempty_pressure_regions" if x == "pressure_regions" else x for x in d["scope"]["excludes"]]
raw = mine.jcs(d).encode("utf-8")
assert raw == b.canonical(d)
h = mine.H("retained_precision_formation_v1", d)
assert h == b.h("retained_precision_formation_v1", d)
p1 = (P / "fixtures/results/semantic_contract_v0_3_physics_1.json").read_bytes()
table = b.physics_retained_table(p1, h)
report = {
    "illustrative_only": True,
    "definition_bytes": len(raw), "definition_raw_sha256": hashlib.sha256(raw).hexdigest(), "definition_H": h,
    "table_bytes": len(table), "table_sha256": hashlib.sha256(table).hexdigest(),
    "changed_paths_vs_I96_draft": ["evidence.pipe_sections", "evidence.pipe_stress_extrema", "evidence.unchanged",
                                   "rows.maximum", "scope.excludes (optional token rename)"],
}
out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
print(json.dumps(report, indent=1, sort_keys=True))
