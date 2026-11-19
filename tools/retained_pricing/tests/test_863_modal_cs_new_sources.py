"""§8.63, MODAL-CS, continued: a new source stops (a storage request, a mention, a path-declared module, an include).

test_863_modal_completed_set.py holds the rules' summary and the shared helpers and constants. The tests of
§8.63 are in six files so that a hosted run, which keeps one file on one worker, spreads them over its workers.
"""
import pytest

from test_863_modal_completed_set import MODAL, OUTCOME, copy_core, decisions


@pytest.mark.parametrize("files, message", [
    # MODAL-CS-ST: an added storage request.
    ({MODAL + "new_source.rs": "fn extra(s: &mut impl ShapeStorage) {\n    let mut v: Vec<u8> = Vec::new();\n"
      "    let _ = s.modal_shape_reserve(&mut v, 1, MODAL_SET_MODES);\n}\n"}, "MODAL-CS-ST: the reviewed inventory changed"),
    # MODAL-CS-RR: the reader's aliased-constructor caller, and a second production call passing Some.
    ({MODAL + "new_source.rs": "use super::resource::ModalCompletedSetLimits as Limits;\nfn limits() -> Limits {\n"
      "    Limits { computed_modes: 1, completed_modes: 1, normalized_entries: 1, published_entries: 1, response_rows: vec![] }\n}\n"},
     "MODAL-CS-RR: the reviewed inventory changed"),
    ({MODAL + "new_source.rs": "fn again(r: &ModalPreviewRequest, p: &ModalIntegrityParameters, l: &L) {\n"
      "    let _ = super::outcome::run_modal_dense_within(r, p, None, Some(l));\n}\n"}, "MODAL-CS-RR: the reviewed inventory changed"),
    # MODAL-CS-LI: an added caller of modal_lift_within; a lift_into mention in the driver's added correct_norm crate.
    ({MODAL + "new_source.rs": "fn lift(p: &SkewPencil, s: &mut impl ShapeStorage) {\n    let _ = p.modal_lift_within(&[], &[], s);\n}\n"},
     "MODAL-CS-LI: the reviewed inventory changed"),
    ({"core/numerics/correct_norm/src/new_source.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n"
      "    let _ = b.lift_into(v, o);\n}\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
    # The reader's two-declaration module: one test declaration and one #[path] declaration of the same file. The
    # census reads the file wherever it is declared, so its mention stops.
    ({"core/solver/frame_kernel/src/shared.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n    let _ = b.lift_into(v, o);\n}\n",
      "core/solver/frame_kernel/src/lib.rs": "\n#[cfg(test)]\nmod shared;\n#[path = \"shared.rs\"]\nmod production_shared;\n"},
     "MODAL-CS-LI: the reviewed inventory changed"),
    # MODAL-CS-TM: a reviewed test file compiled as production through #[path], from its own crate and from another
    # crate by a relative route; a module path set through cfg_attr; a target outside the files read.
    ({MODAL + "new_source.rs": '\n#[path = "tests_resource.rs"]\nmod production_resource;\n'}, "MODAL-CS-TM: .* names a reviewed test file"),
    ({"core/solver/frame_kernel/src/lib.rs": '\n#[path = "../../../product_physics/src/modal/tests_document.rs"]\nmod copied;\n'},
     "MODAL-CS-TM: .* names a reviewed test file"),
    ({MODAL + "new_source.rs": '\n#[cfg_attr(test, path = "other.rs")]\nmod switched;\n'}, "MODAL-CS-TM: .* sets a module path through cfg_attr"),
    ({"core/solver/frame_kernel/src/lib.rs": '\n#[path = "../../../../tools/elsewhere.rs"]\nmod elsewhere;\n'},
     "MODAL-CS-TM: the reviewed inventory changed"),
    # A reviewed outside target (a tests/ helper a #[path] brings in) is read by the censuses: a call added there stops.
    ({"core/solver/frame_kernel/tests/retained_k4/support.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n"
      "    let _ = b.lift_into(v, o);\n}\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
    # ...and a test file named there through a nested #[path] stops as from the walked crates.
    ({"core/solver/frame_kernel/tests/retained_k4/support.rs": '\n#[path = "../../../../product_physics/src/modal/tests_resource.rs"]\n'
      "mod copied;\n"}, "MODAL-CS-TM: .* names a reviewed test file"),
    # The second reader's module: `# [path]` with a space, to a target the walk does not read (not .rs).
    ({OUTCOME: '\n# [path = "limits.inc"]\nmod extra;\n', MODAL + "limits.inc": "pub(crate) fn extra() {}\n"},
     "MODAL-CS-TM: the reviewed inventory changed"),
    # A path literal that is not a plain string.
    ({MODAL + "new_source.rs": '\n#[path = r"skew_pencil.rs"]\nmod again;\n'}, "MODAL-CS-TM: .* is not a plain literal"),
    # An added include!, of a walked file: not in the reviewed include inventory.
    ({MODAL + "new_source.rs": 'include!("skew_pencil.rs");\n'}, "MODAL-CS-TM: the reviewed inventory changed"),
    # A production file gains an ungated #[path] to a target outside the walked set: no test ground, so a hash entry
    # the inventory does not hold.
    ({MODAL + "new_source.rs": '\n#[path = "../../tests/common/current_model.rs"]\nmod current_model;\n'},
     "MODAL-CS-TM: the reviewed inventory changed"),
    # The lexer's bracket matching: an unbalanced bracket in a file the scans read stops rather than mis-spanning.
    ({MODAL + "new_source.rs": "fn probe() {\n    let _ = modal_shape_reserve;\n    let a = (1;\n}\n"}, "MODAL-CS: an unbalanced bracket"),
    # MODAL-CS-ST's constant lookup: a second declaration of a constant a production request names.
    ({MODAL + "new_source.rs": 'const SKEW_LIFT_SCATTER: &str = "x";\n'}, "MODAL-CS-ST: .* is not a named literal constant"),
    # The same file under its test declaration alone: no exclusion, so the census still sees its mention.
    ({"core/solver/frame_kernel/src/shared.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n    let _ = b.lift_into(v, o);\n}\n",
      "core/solver/frame_kernel/src/lib.rs": "\n#[cfg(test)]\nmod shared;\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
])
def test_a_new_source_stops(tmp_path, monkeypatch, files, message):
    snap = copy_core(tmp_path)
    for path, text in files.items():
        target = snap / path
        target.write_text((target.read_text(encoding="utf-8") if target.exists() else "") + text, encoding="utf-8")
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)
