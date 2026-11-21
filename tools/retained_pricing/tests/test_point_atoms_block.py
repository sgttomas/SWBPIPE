"""The law tests' GENERATED POINT ATOMS block (retained-pricing-rules.md §8.53 B1): g5_profile.py's `--point-atoms`
binds the points' atoms beyond the merged trees, and the committed block regenerates from its own names and strides.

It reads only the generator and core/product_physics/src/retained_memory_law_tests.rs, not the priced source, so it is
not part of the retained drift check (docs/CI_STRATEGY.md): a change to the law tests selects it in every pull request.
"""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PP = "core/product_physics/src/"


# ---- the point atoms (§8.53 B1) -------------------------------------------------------------------------------------

LAW = SOURCE / PP / "retained_memory_law_tests.rs"


def _tree(atoms, assumed=None):
    return {"forms": {"F": {a: 1 for a in atoms} | {"1": 3}}, "assumed": assumed or {a: 8 for a in atoms}, "text_atoms": {}}


def _point_block(tmp_path, merged, points):
    law = tmp_path / "law.rs"
    law.write_text("before\n// ---- BEGIN GENERATED POINT ATOMS\n// ---- END GENERATED POINT ATOMS ----\nafter\n")
    paths = []
    for i, t in enumerate([merged] + points):
        paths.append(tmp_path / f"t{i}.json")
        paths[-1].write_text(json.dumps(t))
    r = subprocess.run([sys.executable, str(PRICING / "base/g5_profile.py"), "--point-atoms", str(paths[0]), str(law),
                        *map(str, paths[1:])], capture_output=True, text=True)
    return r, law.read_text()


def test_the_point_atoms_are_the_points_atoms_beyond_the_merged_trees(tmp_path):
    r, text = _point_block(tmp_path, _tree(["s(String)", "Text(x)"]),
                           [_tree(["s(String)"]), _tree(["s(String)", "s(C3Derived)", "Text(y)"]), _tree(["s(Target)"])])
    assert r.returncode == 0, r.stderr
    assert text.startswith("before\n// ---- BEGIN GENERATED POINT ATOMS (") and text.endswith("// ---- END GENERATED POINT ATOMS ----\nafter\n")
    assert '        "s(C3Derived)",\n        "s(Target)",\n    ];' in text and '"s(String)"' not in text and "Text(" not in text
    assert "(size_of::<crate::combination_plan::Target<'static>>()) as u64, // s(Target)" in text
    assert "pub(crate) const ATOMS: usize = 2;" in text


def test_an_unbound_point_atom_stops(tmp_path):
    r, text = _point_block(tmp_path, _tree([]), [_tree(["s(NoSuchAtom)"])])
    assert r.returncode != 0 and "unbound point atoms: s(NoSuchAtom)" in r.stderr
    assert "const ATOMS" not in text


def test_the_committed_point_atoms_are_the_generators():
    """The committed block regenerates from its own atom names and strides by the generator's current bindings (the
    run-tree check is `price.py point-atoms <run> <rs> --check`)."""
    text = LAW.read_text()
    block = text[text.index("// ---- BEGIN GENERATED POINT ATOMS"):text.index("// ---- END GENERATED POINT ATOMS ----")]
    names = json.loads("[" + block.split("ATOM_NAMES: [&str; ATOMS] = [", 1)[1].split("];", 1)[0].strip().rstrip(",") + "]")
    assumed = [int(x) for x in block.split("ATOM_ASSUMED: [u64; ATOMS] = [", 1)[1].split("];", 1)[0].replace(",", " ").split()]
    assert names == sorted(names) and len(names) == len(assumed) >= 30
    with tempfile.TemporaryDirectory() as td:
        r, regenerated = _point_block(Path(td), _tree([]), [_tree(names, dict(zip(names, assumed)))])
    assert r.returncode == 0, r.stderr
    new = regenerated[regenerated.index("// ---- BEGIN GENERATED POINT ATOMS"):regenerated.index("// ---- END GENERATED POINT ATOMS ----")]
    assert new == block
    # The law test prints them beside the profile's own, under the `profile_` filter.
    assert "fn profile_point_atoms_in_build() {" in text and 'println!("I65_G5_ATOM {:?}\\t{}\\t{}\\t{}", ATOM_BINDINGS[i], ATOM_NAMES[i]' in text
