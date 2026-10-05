"""I65 U4 G2: cap arithmetic for D1 (stdlib only, integer arithmetic, no product code).

Evaluates (1) the derived counts at the D1 caps, (2) the T02 raw-backing formula,
(3) the T07 deep legacy-exact owner bound, (4) the T22 scalar maxima and
(5) the PushCap/Expansion capacity facts used by T07.

Layout values marked ASSUMED are illustrative 64-bit values used only to show the
order of magnitude. They are NOT qualified facts: G5 evaluates every stride with
size_of/align_of in the actual build, and private layouts through the BUILD.md
allocation-Layout witnesses. Every formula is monotone nondecreasing in the
counts it takes, so evaluating at the caps bounds every D1 input.
"""
import json

CAPS = {
    "nodes": 32, "members": 32, "supports": 32,
    "restraint_entries": 192, "spring_entries": 192, "nodal_loads": 192,
    "model_materials": 4, "request_materials": 4, "temperature_points_per_material": 16,
    "sections": 32, "identifier_bytes": 128,
    "raw_values": 16_384, "raw_depth": 16,
    "raw_string_bytes": 65_536, "raw_key_bytes": 65_536,
    "raw_array_capacity_elements": 32_768,
    "raw_string_capacity_bytes": 131_072, "raw_key_capacity_bytes": 131_072,
    "digest_capacity_bytes": 128,
}
MILESTONE = {"nodes": 2, "members": 1, "supports": 4, "spring_entries": 3,
             "rigid_dofs": 3, "nodal_loads": 3, "N": 12, "F": 9,
             "raw_values": 150, "raw_depth": 7, "raw_string_bytes": 1039,
             "raw_key_bytes": 855, "max_string_bytes": 46}

def pushcap(s, h):
    """Capacity after h pushes from empty under RawVec growth (I54 COEFFICIENTS V3)."""
    if h == 0:
        return 0
    m = 8 if s == 1 else (4 if s <= 1024 else 1)
    c = 0
    for length in range(1, h + 1):
        if length > c:
            c = max(2 * c, length, m)
    return c

def up(x, a):
    return (x + a - 1) // a * a

out = {"caps": CAPS, "milestone": MILESTONE}

# ---- (1) derived counts at the caps -------------------------------------------
n, m, g = CAPS["nodes"], CAPS["members"], CAPS["supports"]
s_sp = min(CAPS["spring_entries"], g)      # one LinearSupport::spring per "spring" support
l = CAPS["nodal_loads"]
N = 6 * n
k_max = N                                   # unique rigid DOFs <= N
derived = {
    "N=6n": N,
    "F<=N": N,
    "springs_effective<=g": s_sp,
    "Q=7n+30m+s+k+2g (k<=N)": 7*n + 30*m + s_sp + k_max + 2*g,
    "P_final<=7n+51m+8g+3": 7*n + 51*m + 8*g + 3,
    "C<=144m+s": 144*m + s_sp,
    "Z<=min(N^2,144m+s)": min(N*N, 144*m + s_sp),
    "E_lower<=78m+s": 78*m + s_sp,
    "H<=F(F+1)/2": N*(N+1)//2,
    "source_count=144m+s+l (prepare_sources)": 144*m + s_sp + l,
    "source_count_limit": 16_384,
    "dense_source_dof_limit": 256,
    "functional_descriptors Fn<=42m+N+s+6g": 42*m + N + s_sp + 6*g,
    "N^2": N*N,
}
assert derived["source_count=144m+s+l (prepare_sources)"] <= 16_384
assert N <= 256
out["derived_at_caps"] = derived

# ---- (2) T02 raw backing R_raw (I54 BOUND:36-44) --------------------------------
ASSUMED = {"s(Value)": 32, "Leaf(String,Value)": 632, "Internal(String,Value)": 728,
           "s(Expansion)": 32, "s(Ratio)": 64, "s(String)": 24, "s(Vec)": 24,
           "s(StiffnessContribution)": 24, "s(ForceContribution)": 40,
           "s(BlockWitness)": 120, "s(FunctionalDescriptor)": 128, "s(AffineTerm)": 32,
           "s(QualifiedFunctionalProjection)": 96, "s(QualifiedProjection)": 96,
           "s(RetainedProjection)": 72, "s(RetainedFunctionalProjection)": 72,
           "s(MemberRecovery)": 696, "s(SpringAction)": 48, "s(SupportActions)": 128,
           "s(Snapshot)": 232}
objects_upper = CAPS["raw_values"]
entries_upper = CAPS["raw_values"]
node_count = objects_upper + entries_upper // 5
r_raw = (ASSUMED["s(Value)"] * CAPS["raw_array_capacity_elements"]
         + CAPS["raw_key_capacity_bytes"] + CAPS["raw_string_capacity_bytes"]
         + max(ASSUMED["Leaf(String,Value)"], ASSUMED["Internal(String,Value)"]) * node_count
         + CAPS["digest_capacity_bytes"])
out["T02_R_raw_at_caps_bytes_ASSUMED_LAYOUT"] = r_raw

# ---- (5) Expansion capacity facts -----------------------------------------------
T = 256                                   # exact_boundary Limits.expansion_terms
cap_push = pushcap(8, T)                  # Expansion::add rebuilds `next` by push
assert cap_push == 256
eB = 8 * cap_push                         # child bytes per Expansion, worst case
out["expansion"] = {"T_terms": T, "PushCap(8,T)": cap_push, "child_bytes_max": eB,
                    "add_transient_extra_bytes": eB}

# ---- (3) T07 deep legacy-exact owner bound --------------------------------------
A = ASSUMED
Fn = derived["functional_descriptors Fn<=42m+N+s+6g"]
C = derived["C<=144m+s"]
Z = derived["Z<=min(N^2,144m+s)"]
ft = l + N                                 # force terms: one per nodal contribution, plus N (conservative)
atoms_limit = 16_384                       # descriptors_charge count limit
id_b = CAPS["identifier_bytes"]

def mat(stride, r, c):
    return A["s(Vec)"] * r + stride * r * c

snapshot = (A["s(Snapshot)"]
            + mat(8, N, N) + 8 * N + 8 * N + 16 * N
            + A["s(StiffnessContribution)"] * C
            + A["s(ForceContribution)"] * ft + ft * id_b
            + mat(8, N, N) + mat(8, N, N) + id_b)          # symmetry roundoff + counts + basis
r_ = CAPS["restraint_entries"]
names = 8 + n + 5 * l + l + 3 * g + r_ + 3 * g + s_sp + 12 * m
bits = 8 + 3 * n + l + 3 * l + 4 * g + r_ + 4 * N + 3 * s_sp + 325 * m
identity_json = names * (6 * id_b + 3) + bits * 21 + 4
identity_builder = A["s(String)"] * pushcap(24, names) + names * id_b + 8 * pushcap(8, bits)
identity_bytes = identity_json + identity_builder        # builder and output coexist in finish()
k_mat = A["s(Vec)"] * N + A["s(Expansion)"] * N * N + 8 * (4 * Z + 2 * C)
f_vec = A["s(Expansion)"] * N + 8 * (4 * N + 2 * ft)
blocks = 24 * pushcap(24, N) + N * 32
witnesses = A["s(BlockWitness)"] * pushcap(A["s(BlockWitness)"], N) + N * (16 + 3 * 8 * T + eB)
descriptors_one = (A["s(FunctionalDescriptor)"] * Fn + Fn * 2 * id_b
                   + atoms_limit * max(A["s(Vec)"], 8, A["s(AffineTerm)"]))
response = 2 * N * A["s(Ratio)"] + 4 * N * eB
values = Fn * (A["s(Ratio)"] + 2 * eB)
evaluate_transient = N * (A["s(Expansion)"] + eB) + 12 * eB
projections = (A["s(QualifiedFunctionalProjection)"] * pushcap(A["s(QualifiedFunctionalProjection)"], Fn)
               + A["s(QualifiedProjection)"] * pushcap(A["s(QualifiedProjection)"], 2 * N))
retained = (snapshot + witnesses + 2 * N * (A["s(Ratio)"] + 2 * 8 * T)
            + 2 * N * A["s(RetainedProjection)"] + descriptors_one
            + Fn * (A["s(Ratio)"] + 2 * 8 * T) + Fn * A["s(RetainedFunctionalProjection)"] + 16)
output = (16 * N + 16 * N + m * (A["s(MemberRecovery)"] + id_b)
          + s_sp * (A["s(SpringAction)"] + id_b) + g * (A["s(SupportActions)"] + id_b))
context_prep_transient = mat(8, N, N) + 2 * N + 8 * N + 2 * eB   # ordered_k, seen/visited, ordered_f, sum temporaries
t07 = {
    "identity_names": names, "identity_bits": bits,
    "Snapshot_one_copy": snapshot,
    "identity_string_and_builder": identity_bytes,
    "Context_k": k_mat, "Context_f": f_vec, "blocks": blocks, "witnesses": witnesses,
    "descriptors_x3_(sources,plan,retained)": 3 * 2 * descriptors_one,
    "Response": response, "FunctionalSet_values": values,
    "evaluate_transient": evaluate_transient, "projections": projections,
    "RetainedFunctionalSet": retained, "selected_output": output,
    "context_prep_transient": context_prep_transient,
}
t07["SUM_conservative_all_live_bytes_ASSUMED_LAYOUT"] = sum(t07.values())
out["T07_deep_legacy_exact_at_caps"] = t07

# ---- (4) T22 scalar maxima at the caps -------------------------------------------
out["T22_scalar_maxima_at_caps"] = {
    "6n (usize)": N, "nodes (u32)": n, "3m (u32)": 3 * m, "supports (u32)": g,
    "n*n pattern (source.rs:454)": N * N, "free*n (source.rs:358)": N * N,
    "6*supports (final_case.rs:1132)": 6 * g,
    "id bytes per string (u32 offsets, source.rs:389-440)": id_b,
    "children per load/support (u32)": max(l, CAPS["restraint_entries"]),
    "u32::MAX": 2**32 - 1, "2^53-1 (safe JSON)": 2**53 - 1,
}

# ---- STACK_PLAN reservation arithmetic --------------------------------------------
R = 64 * 2**20
out["stack"] = {"reservation_R_bytes": R, "witness_fraction_k": 16, "witness_stack_bytes": R // 16,
                "test_thread_default_bytes": 2 * 2**20,
                "Element_bytes(144*s(Wide2)=144*32 ASSUMED)": 144 * 32,
                "M6_bytes(36*32)": 36 * 32}

print(json.dumps(out, indent=2, sort_keys=False))
