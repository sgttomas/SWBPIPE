"""RT-U4 D3 (RTU4-RC): the exact reader's region-binding count is read from the function that allocates it. The
current source activates the owner; a snapshot without it adds nothing; a changed or half-present owner stops the
run rather than choosing a price. The charge is the push-grown hash table of (&str, &str) keys and 8 B counts."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/rtu4_owners_model.py"))
from g3lib import HashMov, buckets  # noqa: E402


def snapshot(tmp_path, old=None, new=None):
    target = tmp_path / MODEL["PHYS"]
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(SOURCE / MODEL["PHYS"], target)
    if old is not None:
        text = target.read_text()
        assert text.count(old) == 1, old
        target.write_text(text.replace(old, new))
    return tmp_path


def test_current_source_activates_the_owner():
    assert MODEL["inspect"](SOURCE) is True


def test_a_snapshot_without_the_owner_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert MODEL["inspect"](tmp_path) is False


@pytest.mark.parametrize("old,new", [
    # a pre-sized table, another key or count type, a second insertion, a dropped lookup
    ("let mut pipe_rows: HashMap<(&str, &str), usize> = HashMap::new();",
     "let mut pipe_rows: HashMap<(&str, &str), usize> = HashMap::with_capacity(rows.len());"),
    ("let mut pipe_rows: HashMap<(&str, &str), usize> = HashMap::new();",
     "let mut pipe_rows: HashMap<(&str, &str, &str), usize> = HashMap::new();"),
    ("                *pipe_rows.entry((case, row[\"entity_ref\"].as_str().unwrap_or(\"\"))).or_insert(0) += 1;\n",
     "                *pipe_rows.entry((case, row[\"entity_ref\"].as_str().unwrap_or(\"\"))).or_insert(0) += 1;\n"
     "                pipe_rows.entry((case, \"\")).or_insert(0);\n"),
    ("n.checked_add(pipe_rows.get(&(case, *member)).copied().unwrap_or(0))", "n.checked_add(0)"),
])
def test_a_changed_owner_stops(tmp_path, old, new):
    with pytest.raises(SystemExit, match="RT-U4 owners"):
        MODEL["inspect"](snapshot(tmp_path, old, new))


@pytest.mark.parametrize("keys", [0, 1, 4, 7, 8, 56, 57, 96, 112, 113, 1000])
def test_the_charge_is_the_push_grown_table_of_keys_and_counts(keys):
    slots = buckets(keys) + (buckets(keys) // 2 if buckets(keys) > 4 else 0)
    charge = MODEL["reader_count"](keys)
    assert charge.get("s(PairBorrowed)", 0) == slots
    assert charge.get("1", 0) == HashMov("PairBorrowed", keys).get("1", 0) + 8 * slots
