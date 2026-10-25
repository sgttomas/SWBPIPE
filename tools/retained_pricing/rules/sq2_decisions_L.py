"""SQ2 G5, route L: D1.3's branch L (schema 0.1.0/0.2.0, no pressure contract; the preview-physics route, W1Route::Preview).
The common rules (combinations at z <= 2), plus the mirror of I95's route split: B3b's exact-route-only functions and
inline exact branches cannot run on branch L (W1Route::Preview; the RS reader's `route.exact` is false on project()'s
preview-physics-1 base), so they are zero here. I65/I104's D1.3 zero rules stay as they are on this route."""
import os, runpy
globals().update({k: v for k, v in runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sq2_decisions_common.py")).items() if k.isupper() or k in ("rs", "rs_any")})
EX = "route L (W1Route::Preview): reached only under the exact route"
FN_ZERO = [
    ("retained_product.rs:718:capture_exact_materials", EX + " (retained_product.rs:670 `if self.route == W1Route::Exact`)"),
    ("retained_product.rs:737:exact_material_nu", EX + " (its only caller is capture_exact_materials)"),
    ("retained_product.rs:2473:exact_case_evidence", EX + " (retained_product.rs:2462, the Exact arm of observables_view)"),
    ("retained_product.rs:4819:prepared_sections", EX + " (retained_product.rs:4894, the Exact arm of freeze_case)"),
    ("retained_product.rs:5039:apply_exact_section_overlay", EX + " (retained_product.rs:5027 `if scope.route==W1Route::Exact`)"),
    ("retained_wire.rs:946:derived_e_nu_origin", EX + " (retained_wire.rs:928, the Exact arm of material_basis's shear origin)"),
    (rs("g5b_exact_evidence"), EX + " (RS retained_precision.rs:4766 `if route.exact`; project()'s base on route L is preview-physics-1, route() = PREVIEW)"),
    (rs("exact_material"), EX + " (RS retained_precision.rs:3901 `if route.exact`)"),
    (rs("exact_namespace"), EX + " (RS retained_precision.rs:3773 `if route.exact`)"),
]
NEW.update({
    "for entry in evidence": ("0", "RS retained_precision.rs g8: both loops over contract_evidence.exact_cases sit inside `if route.exact` (retained_precision.rs:3925): " + EX),
    'for pm in list(&entry[" "])': ("0", "RS retained_precision.rs g8: inside `if route.exact` (retained_precision.rs:3925): " + EX),
})
EDGE_ZERO = EDGE_ZERO + [
    (rs("g8"), "physics_source.rs:897:actual_materials", "RS g8 calls it only inside `if route.exact` (retained_precision.rs:3925-3943): " + EX),
]
# U6-LF (T4-U6, D-5): the case solve's two per-basis loops sit in `is_exact(model)` arms (o10_rules.py checks it).
O10_LOOPS_FIRST = list(O10_LOOPS_FIRST) + U6_BOTH_L
O10_LOOPS_LAST = list(O10_LOOPS_LAST) + U6_LOOPS_L

B7A_STAGE1_ROUTE = "L"

# W1-L5: route L shares U6-RB's source-bound production guard.
O10_TYPED_PATCHES = list(globals().get("O10_TYPED_PATCHES", [])) + W1_L5_PATCHES

# U4B-RL (T4-S5): route L's rows per member. preview-physics-2 publishes, on each straight member, the 51 rows of
# preview-physics-1 and the derived σ⁺, σ⁻ and τ_v,max at its five stations (u4b_rows::append_station_quantities,
# one section): 15 rows more, 66. The producer states the same count for the failed-ordinary manifest, which mirrors
# preview-physics-2's per-member rows (retained_product.rs FAILED_MEMBER_ROWS, read here; a member whose ν is out of
# domain publishes 5 fewer, so 66 bounds every member). Route L's P_final and R0 take it in place of 51.
import re as _rl_re
_RL_SRC = open(os.path.join(os.environ["SQ2_SNAP"], "core/product_physics/src/retained_product.rs"), encoding="utf-8").read()
_RL = [int(v) for v in _rl_re.findall(r"^const FAILED_MEMBER_ROWS: usize = (\d+);$", _RL_SRC, _rl_re.M)]
# W7-RL (W-7, DEL-04-01 Design global-end-actions.md; retained-pricing-rules.md §8.20): every straight member and realized
# bend also publishes its twelve global end rows on every ordinary run (the failed manifest carries them as not-covered
# placeholders after the maximum): 66 + 12 = 78 (FAILED_MEMBER_ROWS).
if len(_RL) != 1 or _RL[0] not in (51, 51 + 15, 51 + 15 + 12):
    raise SystemExit(f"U4B-RL: route L's per-member row count is not the reviewed 51, 66 or 78: {_RL}")
if _RL[0] != 51:
    _rpm, _why = _RL[0], ("U4B-RL: route L's per-member rows, 51 + 15 derived (FAILED_MEMBER_ROWS)" if _RL[0] == 66 else
                          "U4B-RL/W7-RL: route L's per-member rows, 51 + 15 derived + 12 global end (FAILED_MEMBER_ROWS)")
    O10_TYPED_PATCHES = list(O10_TYPED_PATCHES) + [
        ("g3lib.py", "P=7 * n + 51 * m + 8 * g + 3, R0=7 * n + 51 * m + g + 3,",
         f"P=7 * n + {_rpm} * m + 8 * g + 3, R0=7 * n + {_rpm} * m + g + 3,", _why),
        ("g4_caps.py", "P = 7 * n + 51 * m + 8 * g + 3                     # P_final",
         f"P = 7 * n + {_rpm} * m + 8 * g + 3                     # P_final", _why),
        ("ordinary_caps.py", "    mats = 8; pts = 16                                # model + request materials; points each\n"
         "    P = 7 * n + 51 * m + 8 * g + 3; R0 = 7 * n + 51 * m + g + 3",
         "    mats = 8; pts = 16                                # model + request materials; points each\n"
         f"    P = 7 * n + {_rpm} * m + 8 * g + 3; R0 = 7 * n + {_rpm} * m + g + 3", _why),
        ("producer_caps.py", "P_final = 7 * n + 51 * m + 8 * g + 3",
         f"P_final = 7 * n + ({_rpm} if which == \"caps\" else 51) * m + 8 * g + 3", _why),
        ("t25_g4.py", "P = 7 * n + 51 * m + 8 * g + 3\nR0 = 7 * n + 51 * m + g + 3",
         f"P = 7 * n + {_rpm} * m + 8 * g + 3\nR0 = 7 * n + {_rpm} * m + g + 3", _why),
    ]
O10_SITE_FROM = list(O10_SITE_FROM) + W1_L5_SITE_FROM
