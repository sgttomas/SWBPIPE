"""W1 coverage repair D3 (retained-pricing-rules.md §8.11).

W1R-FG: code under the off `census-probe` and `loaded-maximum-hooks` features is compiled out of the registered build (no features), so the scanners
blank it as they blank `#[cfg(test)]` items, and skip the two files the build includes only through compiled-out
module declarations. The inclusion and manifest guards fail closed.

W1R-FS: the certified proof's final scales are priced positively per attempted case; a snapshot without them adds
nothing and a changed body stops."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
SCAN = runpy.run_path(str(PRICING / "base/loopscan.py"))
OWNERS = runpy.run_path(str(PRICING / "base/w1r_owners_model.py"))
PROBE = "core/solver/frame_kernel/src/structural/retained/product_certificate/census_probe.rs"
DRIVER = "core/product_physics/src/w1_census_driver.rs"
FINAL = "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs"
FK_TOML = "core/solver/frame_kernel/Cargo.toml"
PP_TOML = "core/product_physics/Cargo.toml"


def production(raw):
    return SCAN["blank_test_items"](SCAN["blank_rust"](raw), raw)


def test_off_feature_statements_and_items_are_blanked_with_lines_kept():
    raw = ('fn f() -> bool {\n'
           '    #[cfg(feature = "census-probe")]\n'
           '    probe.finish("converged", a);\n'
           '    #[cfg(feature = "census-probe")]\n'
           '    if census_probe::switches().record { rows.push(format!("{x}")); }\n'
           '    #[cfg(not(feature = "census-probe"))]\n'
           '    let shared = false;\n'
           '    // #[cfg(feature = "census-probe")] in a comment gates nothing\n'
           '    keep();\n'
           '    shared\n'
           '}\n'
           '#[cfg(all(test, feature = "census-probe"))]\n'
           'pub(crate) fn census_only() -> Value { format!("{y}") }\n'
           '#[cfg(feature = "other")]\n'
           'fn other_feature() {}\n')
    out = production(raw)
    assert out.count("\n") == raw.count("\n")
    assert "finish" not in out and "census_probe" not in out and "format" not in out and "census_only" not in out
    assert "let shared = false;" in out and "keep();" in out
    assert "fn other_feature" in out                          # only the reviewed off features are blanked
    # Without the raw text the historical behaviour is unchanged.
    assert "finish" in SCAN["blank_test_items"](SCAN["blank_rust"](raw))


def test_the_capacity_hook_feature_is_off_as_census_probe_is():
    """Corner 1 (§8.12): `loaded-maximum-hooks` exposes the kernel's test capacity hook only to the
    genuine-exhaustion control; the registered build leaves it off, so its items are blanked too."""
    raw = ('fn limits() -> (usize, u8, usize) {\n'
           '    #[cfg(any(test, feature = "loaded-maximum-hooks"))] if let Some(l) = hooks::get() { return l; }\n'
           '    (LIVE, DEPTH, SPLITS)\n'
           '}\n'
           '#[cfg(feature = "loaded-maximum-hooks")]\n'
           'pub use loaded_maximum::hooks as loaded_maximum_hooks;\n'
           '#[cfg(all(test, feature = "loaded-maximum-hooks"))]\n'
           'fn hooked() -> String { format!("{z}") }\n'
           'pub const LOADED_MAXIMUM_HOOKS: bool = cfg!(feature = "loaded-maximum-hooks");\n')
    out = production(raw)
    assert out.count("\n") == raw.count("\n")
    assert "hooks::get" not in out and "loaded_maximum_hooks" not in out and "hooked" not in out
    assert "(LIVE, DEPTH, SPLITS)" in out and "pub const LOADED_MAXIMUM_HOOKS" in out
    final = (SOURCE / FINAL).read_text()
    assert "loaded_maximum_hooks" in final and "loaded_maximum_hooks" not in production(final)


def test_the_current_source_excludes_exactly_the_probe_and_its_driver():
    assert SCAN["off_build_files"](str(SOURCE)) == {PROBE, DRIVER}
    final = (SOURCE / FINAL).read_text()
    assert "census_probe" in final and "census_probe" not in production(final)


def copy(tmp_path, edit=None):
    root = SOURCE
    for rel in (PROBE, DRIVER, FINAL, FK_TOML, PP_TOML, "core/product_physics/src/w1_coverage_census_tests.rs",
                "core/product_physics/src/w1e_census_tests.rs"):
        target = tmp_path / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / rel, target)
    if edit:
        rel, old, new = edit
        text = (tmp_path / rel).read_text()
        assert text.count(old) == 1, old
        (tmp_path / rel).write_text(text.replace(old, new))
    return str(tmp_path)


def test_a_snapshot_without_the_probe_excludes_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert SCAN["off_build_files"](str(tmp_path)) == set()


@pytest.mark.parametrize("edit", [
    (FINAL, '#[cfg(feature = "census-probe")]\n#[path = "census_probe.rs"]', '#[path = "census_probe.rs"]'),
    (FK_TOML, 'census-probe = []', 'census-probe = []\ndefault = ["census-probe"]'),
    (PP_TOML, '[dev-dependencies]', '[dev-dependencies]\nopen_pipe_stress_frame_kernel = { path = "../solver/frame_kernel", features = ["census-probe"] }'),
    (FINAL, "fn corner1() -> bool {", 'mod census_probe;\nfn corner1() -> bool {'),
    (FK_TOML, 'loaded-maximum-hooks = []', 'loaded-maximum-hooks = []\ndefault = ["loaded-maximum-hooks"]'),
    (PP_TOML, '[dev-dependencies]', '[dev-dependencies]\nopen_pipe_stress_frame_kernel = { path = "../solver/frame_kernel", features = ["loaded-maximum-hooks"] }'),
    (PP_TOML, 'loaded-maximum-hooks = ["open_pipe_stress_frame_kernel/loaded-maximum-hooks"]',
     'loaded-maximum-hooks = ["open_pipe_stress_frame_kernel/loaded-maximum-hooks"]\ndefault = ["loaded-maximum-hooks"]'),
    (FINAL, "fn corner1() -> bool {", 'mod probe_text { include!("census_probe.rs"); }\nfn corner1() -> bool {'),
    (FINAL, "fn corner1() -> bool {", 'mod probe_text { include! { "census_probe.rs" } }\nfn corner1() -> bool {'),
    (FINAL, "fn corner1() -> bool {", 'mod probe_text { include ! ["census_probe.rs"]; }\nfn corner1() -> bool {'),
    (FINAL, "fn corner1() -> bool {", '#[cfg(feature = "census-probe")]\nmod probe_text { include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural/retained/product_certificate/census_probe.rs")); }\nfn corner1() -> bool {'),
])
def test_an_inclusion_or_manifest_change_stops(tmp_path, edit):
    with pytest.raises(ValueError, match="W1R-FG"):
        SCAN["off_build_files"](copy(tmp_path, edit))


@pytest.mark.parametrize("snippet", [
    'fn f(x: u8) -> u8 {\n    match x {\n        #[cfg(feature = "census-probe")]\n        0 => 1,\n        _ => 2,\n    }\n}\nfn g() -> Vec<u8> { Vec::with_capacity(7) }\n',
    'fn f() -> Vec<u8> {\n    vec![\n        1,\n        #[cfg(feature = "census-probe")]\n        2\n    ]\n}\nfn g() -> Vec<u8> { Vec::with_capacity(7) }\n',
    'struct S { a: u8, b: u8 }\nfn f() -> S {\n    S {\n        #[cfg(feature = "census-probe")]\n        a: 1,\n        b: 2,\n    }\n}\nfn g() -> Vec<u8> { Vec::with_capacity(7) }\n',
    'fn f(x: u8) -> u8 {\n    match x {\n        #[cfg(feature = "loaded-maximum-hooks")]\n        0 => 1,\n        _ => 2,\n    }\n}\nfn g() -> Vec<u8> { Vec::with_capacity(7) }\n',
])
def test_an_off_feature_attribute_in_an_expression_position_stops(snippet):
    with pytest.raises(ValueError, match="W1R-FG"):
        production(snippet)


def test_the_final_scales_owner_is_active_and_positive():
    assert OWNERS["inspect"](str(SOURCE)) is True
    form = OWNERS["proof_scales"](32)
    assert dict(form) == {"s(f64)": 128, "s(Vec)": 13}


def test_a_snapshot_without_final_scales_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert OWNERS["inspect"](str(tmp_path)) is False


def test_a_changed_scales_body_stops(tmp_path):
    for rel in {rel for rel, _ in OWNERS["BODY_PINS"]} | {OWNERS["RP"]}:
        target = tmp_path / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / rel, target)
    text = (tmp_path / FINAL).read_text()
    (tmp_path / FINAL).write_text(text.replace("    Ok(scales)\n}", "    Ok(scales.clone())\n}", 1))
    with pytest.raises(SystemExit, match="check_intervals"):
        OWNERS["inspect"](str(tmp_path))
