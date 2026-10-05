"""I65 U4 G3 T08: maximum spellings of the composite text arguments (stdlib only, read-only).

Derived Debug output length is computed structurally (std `#[derive(Debug)]`):
  struct  `Name { f1: v1, f2: v2 }` = len(Name) + 3 + sum(len(f)+2+v) + 2*(k-1) + 2
  Vec     `[a, b]`                   = 2 + sum + 2*(n-1)
  Option  `Some(v)` / `None`         = 6 + v
  unit enum variant                  = len(name)
Primitive maxima (installed core 1.97.1, core/fmt/float.rs and integer formatting):
  usize/u64 20, i32 11, bool 5, f64 Debug/LowerExp 24, f64 Display 327.
  String Debug of b input bytes <= 2 + 10*b (each char is >= 1 byte; the longest escape
  is the 10-char `\\u{10ffff}`); static source literals are measured from source and
  checked to be printable ASCII without quotes/backslashes, so their Debug is 2 + len.
Field lists are transcribed from the cited definitions.
Usage: python3 composite_text.py <P root> <caps|milestone>
"""
import json, os, re, sys

proot, which = sys.argv[1], sys.argv[2]
COUNTS = {
    "caps": dict(n=32, m=32, g=32, s=32, k=192, r=192, l=192, N=192, F=192, Z=4640, C=4640, ident=128),
    "milestone": dict(n=2, m=1, g=4, s=3, k=3, r=6, l=3, N=12, F=9, Z=144, C=147, ident=46),
}[which]
if which == "caps":
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from g3lib import g4_caps as _gc
    _c = _gc()
    COUNTS = dict(n=_c["n"], m=_c["m"], g=_c["g"], s=_c["s"], k=_c["k"], r=_c["r"], l=_c["l"], N=_c["N"], F=_c["F"],
                  Z=_c["Z"], C=_c["C"], ident=_c["ident"])
c = COUNTS
U, I32, B, F64D, F64S = 20, 11, 5, 24, 327

def struct(name, fields):
    return len(name) + 3 + sum(len(f) + 2 + v for f, v in fields) + 2 * (len(fields) - 1) + 2
def vec(elem, n):
    return 2 + n * elem + 2 * max(n - 1, 0)
def option(v):
    return 6 + v
def sdebug(b):
    return 2 + 10 * b

# static literals actually used as &'static str fields: measure the longest literal in the
# source files that define them, and check the Debug premise (printable ASCII, no quote/backslash)
lit_files = ["core/solver/frame_kernel/src/structural.rs", "core/solver/frame_kernel/src/lib.rs",
             "core/solver/frame_kernel/src/structural/exact_boundary.rs", "core/product_physics/src/lib.rs"]
L_static, bad = 0, 0
for f in lit_files:
    t = open(os.path.join(proot, f), encoding="utf-8").read()
    for m in re.finditer(r'"((?:\\.|[^"\\])*)"', t):
        s = m.group(1)
        L_static = max(L_static, len(s.encode()))
        if "\\" in s or any(ord(ch) < 32 or ord(ch) > 126 for ch in s):
            bad += 1
STATIC = L_static
# Report fields (&'static str) take literals defined in FK structural.rs / structural/*.rs; the
# symmetry basis String is a copy of a literal from nonlinear_integration structural_adapter.rs.
rep_files = ["core/solver/frame_kernel/src/structural.rs", "core/solver/frame_kernel/src/structural/sparse.rs",
             "core/solver/frame_kernel/src/structural/formation_check.rs",
             "core/solver/nonlinear_integration/src/structural_adapter.rs"]
L_rep, rep_escaped = 0, 0
for f in rep_files:
    t = open(os.path.join(proot, f), encoding="utf-8").read()
    for m in re.finditer(r'"((?:\\.|[^"\\])*)"', t):
        L_rep = max(L_rep, len(m.group(1).encode()))
        if "\\" in m.group(1): rep_escaped += 1
# Debug of a printable literal is 2 + len; escapes (\n, \", \\) at most double a char
STATIC_DEBUG = 2 + 2 * L_rep

ident = c["ident"]
# FK structural.rs:308-345 PivotEvidence / ResidualRow / ContributionRounding / StructuralReport
pivot = struct("PivotEvidence", [("ordered_index", U), ("global_dof", U), ("pivot", F64D),
                                  ("cancellation_scale", F64D), ("operation_count", U), ("screen", F64D)])
resid = struct("ResidualRow", [("global_dof", U), ("residual", F64D), ("denominator", F64D),
                                ("row_scale_exponent", I32), ("normalized_residual", F64D),
                                ("normalized_denominator", F64D), ("normalized_evaluation_allowance", F64D),
                                ("normalization_basis", STATIC_DEBUG), ("operation_count", U),
                                ("evaluation_allowance", F64D), ("guarded_ratio", F64D), ("target", F64D),
                                ("passed", B)])
cr_fixed = struct("ContributionRounding", [("row", U), ("col", U), ("accumulated_high", F64D),
                                           ("accumulated_low", F64D), ("stored_difference_high", F64D),
                                           ("stored_difference_low", F64D),
                                           ("accumulated_expansion", 2), ("difference_expansion", 2)])
# expansion elements over all entries: accumulated <= c_ij, difference <= c_ij + 1 => total <= 2C + Z
cr_total = c["Z"] * cr_fixed + (2 * c["C"] + c["Z"]) * (F64D + 2)
report = (len("StructuralReport") + 3 + 2 + 2 * 17
          + sum(len(f) + 2 for f in ["policy", "quality", "factorization", "scale_exponents", "pivots",
                                     "condition_estimator", "reciprocal_condition_estimate", "residual_rows",
                                     "refinement_attempts", "contribution_audit_performed", "contribution_rounding",
                                     "assembly_relative_perturbation_estimate", "assembly_amplification_estimate",
                                     "assembly_load_perturbation_estimate", "intended_residual_rows",
                                     "symmetry_projection_performed", "maximum_scaled_skew", "symmetry_basis"])
          + STATIC_DEBUG + len("Sensitive") + STATIC_DEBUG + vec(I32, c["F"]) + vec(pivot, c["F"])
          + STATIC_DEBUG + F64D + vec(resid, c["F"]) + U + B + (2 + cr_total + 2 * max(c["Z"] - 1, 0))
          + 3 * F64D + vec(resid, c["F"]) + B + F64D + option(STATIC_DEBUG))
dof_label = ident + 3                                   # "node:UX" (PP lib.rs:1050-1067)
dof_map = vec(sdebug(dof_label), c["N"])
integrity_literal = 204 + 200                            # template literal + the static non-equilibrium sentence
work_report = struct("WorkReport", [("charged", U), ("rejected", U), ("limit", U)])
summary = struct("RecoverySummary", [(f, U) for f in ["dofs", "stiffness_contributions", "identified_load_contributions",
                 "blocks", "largest_block", "member_end_components", "section_components", "spring_actions",
                 "support_components", "published_nodal_components", "functional_count", "dof_projection_count"]]
                 + [(f, F64D) for f in ["max_absolute_projection_error", "max_relative_projection_error", "relative_limit"]]
                 + [("work", work_report)])
ERR = 8192                                               # error Display bound (TEXT.md §3)
formation_check_line = max(120 + dof_label + 3 * F64D, 70 + 30 + ERR)
range_entry = 2 + max(9 + 600 + 600 + 60, 64 + 600 + 24)
range_line = 80 + 20 + 6 * range_entry + 30              # NAMED = 6 entries + "more=" (PP lib.rs:1754-1768)
F_len = max(ident + 34 + 3 * 24, ident + 41 + ERR)       # formation_guard.rs fired item (I54 RESIDUALS)
J_len = 6 * F_len + 2 * 5
C_len = 11 + 20
S_len = 221 + ident + 20 + J_len + C_len                 # guard sentence (I54 RESIDUALS)
message_int = (integrity_literal + STATIC + ident + report + dof_map
               + 1 + formation_check_line + 1 + range_line + 1 + S_len)
# load-row sources: sum over all rows <= l source ids (each nodal load contributes to one DOF)
sources_all = c["l"] * (sdebug(ident) + 2) + 2 * c["N"]
lf_row_fixed = 46 + 165 + 2 * U + 16 + 16 + 3 * F64D + U + B
lf_join = c["N"] * (lf_row_fixed + 2) + sources_all + 300
joined = {
    "fired_join": J_len,
    "support_contribution_summary": c["g"] * (2 * ident + 6 * 2 + 5 + 3),
    "join_dofs": 6 * 2 + 5,
    "provenance_join": 8 * (ident + 200) + 8 * 3,
    "missing_join": 400,
    "load_fidelity_rows_join": lf_join,
}
out = {
    "which": which, "static_literal_max_bytes": L_static, "report_literal_max_bytes": L_rep, "static_literals_with_escapes": bad,
    "classes": {
        "struct_report_debug": report, "dof_map_debug": dof_map, "message_int": message_int,
        "recovery_summary_debug": summary, "sources_debug": sources_all,
        "evidence_line": max(formation_check_line, range_line, S_len),
        "joined_list": max(joined.values()), "static": max(512, STATIC), "error_display": ERR, "error_debug": ERR,
    },
    "parts": {"pivot": pivot, "residual_row": resid, "contribution_rounding_fixed": cr_fixed,
              "contribution_rounding_total": cr_total, "formation_check_line": formation_check_line,
              "range_line": range_line, "F_len": F_len, "S_len": S_len, "joined": joined},
}
print(json.dumps(out, indent=1))
