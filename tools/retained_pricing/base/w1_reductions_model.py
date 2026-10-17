"""W1 reductions: source-bound allocation and lifetime premises, before stage 2.

The RF source and the reduced source are explicit cohorts. The full raw pins bind
all producer/consumer paths for L1a/b/c, L2, L3 and L5, including the cfg(test)
shadow of the aggregation guard. A half change or a changed lifetime stops;
marker absence never silently selects historical prices. Pins are refreshed only
after the source diff and its allocation/lifetime consequences are reviewed.
"""
import hashlib
from pathlib import Path

FC = "core/solver/frame_kernel/src/structural/formation_check.rs"
LIB = "core/product_physics/src/lib.rs"
RW = "core/product_physics/src/retained_wire.rs"
NI = "core/solver/nonlinear_integration/src/structural_adapter.rs"
GUARD = "    let combining = preview.is_some() && !model.combinations.is_empty();\n"
SHADOW = ("    #[cfg(test)]\n"
          "    let combining = combining || (l5_tests::former_aggregation() && preview.is_some());\n")
INSERT = ("            if combining {\n                rows_by_base_id\n"
          "                    .entry(base_id)\n                    .or_default()\n"
          "                    .insert(load_case_id.clone(), result.clone());\n            }\n")
CALL = ("    let preview_gates = if combining {\n"
        "        preview_physics::append_combination_results(&model, preview.as_ref().expect(\"combining requires preview\"), &rows_by_base_id, &mut results, &mut diagnostics)\n"
        "    } else {\n        Vec::new()\n    };\n")
RF_PINS = {
    "core/product_physics/src/lib.rs": "6c48546bbea452c7f26a3490892d4b441aff25ed7c1c1a9e2b10ac8eb4d91bb4",
    "core/product_physics/src/retained_wire.rs": "7e4326e5b294f6db2ed0995eeca1a8bae3e4ddd487bb0f70436f82426eaa6f3a",
    "core/product_physics/src/certified_publication.rs": "1c6de84c0f7c4cd0799a5b2c7f4bacf3d5d950bb062a816066aaabdd45b489c5",
    "core/product_physics/src/certified_loads.rs": "56f2a4ef24ccca359290d7b25ab2078487d89da908826dd3f3897e14fdd35c8f",
    "core/product_physics/src/preview_physics.rs": "61b2fb2f3f9bc3aa9803bf6bf5c95cf5e51c841289ef5c415bf86fe00f33b468",
    "core/solver/frame_kernel/src/structural/formation_check.rs": "0d83c8a61b890d757b96529d255e38c42b35793ae5c998871331278c8cbaa384",
    "core/solver/frame_kernel/src/structural/certified_recovery.rs": "1766a0d94cf02b909b4875cae756b6df5d0bb81489c359281109b7cb248999dc",
    "core/solver/frame_kernel/src/load_ledger.rs": "8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f",
    "core/solver/nonlinear_integration/src/structural_adapter.rs": "bf42cfa532f0f36823fb78128ad175d6bb9a5765e8fd7374599da89df4232a8b",
    "core/solver/straight_pipe/src/lib.rs": "a375535794d2700ffb08621aebe6f587a2d226f107056f320ee23fef466d1bb7"
}
REDUCED_PINS = {
    "core/product_physics/src/lib.rs": "a5c5b09f31a996c19ff63ace14bd39a07b7a474d6b03556a19825057e90aed74",
    "core/product_physics/src/retained_wire.rs": "d1f3ce7f03a7614e415d57495de07bfc61fa1cb42e915de0e320340a87ff09a0",
    "core/product_physics/src/certified_publication.rs": "0008eb462f94640386194afb3659fdc3d74fb2e45ae690b7b2646db5feb8be57",
    "core/product_physics/src/certified_loads.rs": "56f2a4ef24ccca359290d7b25ab2078487d89da908826dd3f3897e14fdd35c8f",
    "core/product_physics/src/preview_physics.rs": "61b2fb2f3f9bc3aa9803bf6bf5c95cf5e51c841289ef5c415bf86fe00f33b468",
    "core/solver/frame_kernel/src/structural/formation_check.rs": "4e2b2aed3a155b3893d9bab353aad112248ca555b4016b59a075cb0ec719cdf2",
    "core/solver/frame_kernel/src/structural/certified_recovery.rs": "e9a0bb4220482e9596e0e11ec698ae2291da5d223865d21ce9d7e2725594f7af",
    "core/solver/frame_kernel/src/load_ledger.rs": "8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f",
    "core/solver/nonlinear_integration/src/structural_adapter.rs": "c71d4acf1aa3f2d5c048eefe3d559b192d31c2d962083cf89bd4ef978347d9cd",
    "core/solver/straight_pipe/src/lib.rs": "a375535794d2700ffb08621aebe6f587a2d226f107056f320ee23fef466d1bb7"
}


def inspect(snapshot):
    root = Path(snapshot)
    fc = (root / FC).read_text()
    if "    pub loads: Cow<'a, [LoadPrimitive]>,\n" in fc:
        reduced = True
        expected = REDUCED_PINS
    elif "    pub loads: Vec<LoadPrimitive>,\n" in fc:
        reduced = False
        expected = RF_PINS
    else:
        raise ValueError("W1 reductions L1a: FormationSource.loads changed; re-review")
    for relative, digest in expected.items():
        if hashlib.sha256((root / relative).read_bytes()).hexdigest() != digest:
            raise ValueError("W1 reductions source/lifetime pin changed: " + relative)
    if reduced:
        guard((root / LIB).read_text())
    return reduced


def guard(text):
    """L5 and U6-RB share this fail-closed production/test guard warrant."""
    if (text.count(GUARD) != 1 or text.count(SHADOW) != 1 or text.count(INSERT) != 1
            or text.count(CALL) != 1):
        raise ValueError("U6-RB/L5: guarded combination index/call or test shadow changed; re-review")
    return True
