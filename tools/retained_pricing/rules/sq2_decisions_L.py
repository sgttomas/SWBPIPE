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

B7A_STAGE1_ROUTE = "L"
