"""§8.63, MODAL-CS, continued: the guarded rows, through the emitted rules and at the size-zero test sites.

test_863_modal_completed_set.py holds the rules' summary and the shared helpers and constants. The tests of
§8.63 are in six files so that a hosted run, which keeps one file on one worker, spreads them over its workers.
"""
import pytest

from test_863_modal_completed_set import PP, check, decisions, o10_check_rules, point


def rows_for(keys, counts):
    return [{"file": PP + k.rsplit(":", 1)[0], "line": int(k.rsplit(":", 1)[1]), "mult": 0}
            for k in keys for _ in range(counts.get(k, 1))]


@pytest.mark.parametrize("change", ["exact", "shared-line loss", "size-zero positive", "size-zero missing", "boundary positive"])
def test_o10_rejects_a_changed_guarded_row_through_the_emitted_rules(tmp_path, monkeypatch, change):
    rules = o10_check_rules(decisions(monkeypatch))
    keys, counts = rules["o10_mult0_sites"], rules["o10_mult0_rows"]
    assert counts["modal/resource.rs:1402"] == 2
    size0 = next(str(k) for k in keys if str(k) == "modal/tests_resource.rs:907")
    rows = rows_for(keys, counts)
    if change == "shared-line loss":
        rows.remove(next(r for r in rows if r["file"].endswith("modal/resource.rs") and r["line"] == 1402))
    elif change == "size-zero positive":
        next(r for r in rows if r["file"].endswith(size0.split(":")[0]) and r["line"] == 907)["mult"] = 1
    elif change == "size-zero missing":
        rows = [r for r in rows if not (r["file"].endswith(size0.split(":")[0]) and r["line"] == 907)]
    elif change == "boundary positive":
        next(r for r in rows if r["file"].endswith("modal/outcome.rs") and r["line"] == 446)["mult"] = 1
    code, out = check(tmp_path, {k: v for k, v in rules.items() if k != "o10_unreached_fns"}, [point(tmp_path, "L_30", {}, rows=rows)])
    assert code == (0 if change == "exact" else 3), out


@pytest.mark.parametrize("line", [907, 994, 1060, 1104])
@pytest.mark.parametrize("change", ["positive", "missing"])
def test_each_size_zero_test_site_is_guarded(tmp_path, monkeypatch, line, change):
    """Each of J3's four size-zero test sites, through the emitter's own rules: a positive or missing row fails."""
    rules = o10_check_rules(decisions(monkeypatch))
    keys, counts = rules["o10_mult0_sites"], rules["o10_mult0_rows"]
    assert f"modal/tests_resource.rs:{line}" in map(str, keys)
    rows = rows_for(keys, counts)
    hit = [r for r in rows if r["file"].endswith("modal/tests_resource.rs") and r["line"] == line]
    assert len(hit) == 1
    if change == "positive":
        hit[0]["mult"] = 1
    else:
        rows.remove(hit[0])
    code, out = check(tmp_path, {k: v for k, v in rules.items() if k != "o10_unreached_fns"}, [point(tmp_path, "L_30", {}, rows=rows)])
    assert code == 3, out
