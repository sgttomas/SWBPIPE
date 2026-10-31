"""RTA3B-PS and RTA3B-MR (retained-pricing-rules.md §8.36): RT-A3 increment 1b's published-row index. Each case's
PublishedScope entity set is charged at a u64 table of the case's rows (P_final) with its previous table, and each
member's MemberRows slots at the largest push capacity of 30 vectors holding its rows per member plus one old backing.
The premises they rest on are guarded by name; a changed premise or index stops the run, and a snapshot without the
index adds nothing."""
from itertools import product
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/u4b_u5b_owners_model.py"))
LIB, U4B = MODEL["LIB"], MODEL["U4B"]
from g3lib import pushcap  # noqa: E402


def snapshot(tmp_path, file=None, old=None, new=None):
    for relative in (LIB, U4B):
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    if file:
        target = tmp_path / file
        text = target.read_text(encoding="utf-8")
        assert text.count(old) == 1, old
        target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_current_source_has_the_reviewed_index():
    assert MODEL["published_index"](SOURCE) is True
    assert MODEL["inspect"](SOURCE)["published_index"] is True


def test_a_snapshot_without_the_index_adds_nothing(tmp_path):
    (tmp_path / "core").mkdir()
    assert MODEL["published_index"](tmp_path) is False


def test_a_half_present_index_stops(tmp_path):
    # The scope in u4b_rows.rs without the case solve's use of it is neither absent nor reviewed.
    snap = snapshot(tmp_path, LIB, "    let mut published_scope = u4b_rows::PublishedScope::default();\n", "")
    with pytest.raises(SystemExit, match="only partly present"):
        MODEL["published_index"](snap)


def test_the_owners_at_the_caps():
    # Route L: P_final 7n + 78m + 8g + 3 = 2,979 rows per case at n = m = g = 32; route E: 114 rows per member, 4,131.
    # 2,979 rows need 4,096 buckets (8 B slot and 1 control byte each, plus 8 + 15) and the previous 2,048 coexist.
    assert MODEL["published_scope_entities"](2979) == {"1": (9 * 4096 + 23) + (9 * 2048 + 23)}
    assert MODEL["published_scope_entities"](4131) == {"1": (9 * 8192 + 23) + (9 * 4096 + 23)}
    assert MODEL["published_scope_entities"](0) == {}
    # 78 entries over 30 slots: 28 hold one (capacity 4 each), one 17 (32) and one 33 (64); + 64 old backing.
    assert MODEL["member_row_slots"](78) == {"1": 8 * (28 * 4 + 32 + 64 + 64)}
    # 114 entries: the largest total capacity over every split is 276; + 64 old backing.
    assert MODEL["member_row_slots"](114) == {"1": 8 * (276 + 64)}
    assert MODEL["member_row_slots"](78)["1"] == 2176 and MODEL["member_row_slots"](114)["1"] == 2720


@pytest.mark.parametrize("rows, slots", [(0, 3), (1, 3), (7, 2), (12, 3), (20, 2)])
def test_the_slot_capacity_is_the_largest_over_every_split(rows, slots):
    brute = max(sum(pushcap(k) for k in split) for split in product(range(rows + 1), repeat=slots) if sum(split) <= rows)
    assert MODEL["member_row_slots"](rows, slots) == ({"1": 8 * (brute + pushcap(rows) // 2)} if rows else {})


def test_both_owners_are_charged_per_case_beside_u4b_st():
    text = (PRICING / "base/ordinary_caps.py").read_text(encoding="utf-8")
    for family in ("Published-row scope of one case (RTA3B-PS)", "Member row index of one member (RTA3B-MR)"):
        assert text.count(f'"{family}"') == 2   # added in OrdinaryRecovery and listed among the per-case families
    assert '_u45_model.published_scope_entities(P)' in text and '_u45_model.member_row_slots(_rpm)' in text


SCOPE = "    let mut published_scope = u4b_rows::PublishedScope::default();\n"
BEGIN = "        let member_start = published_scope.begin(&results, &pipe.element_id);\n"
ROWS = "            let mut member_rows = u4b_rows::MemberRows::new(&pipe.element_id, member_start);\n"


@pytest.mark.parametrize("file, old, new, stop", [
    # A second scope in the case solve.
    (LIB, SCOPE, SCOPE + "    let mut spare_scope = u4b_rows::PublishedScope::default();\n", "PS: one scope per case"),
    # A second begin per member.
    (LIB, BEGIN, BEGIN + "        let _again = published_scope.begin(&results, &pipe.element_id);\n", "PS: one begin per member"),
    # The entity set filled beyond begin's one extend.
    (U4B, "        self.synced = results.len();\n", "        self.synced = results.len();\n        self.entities.insert(0);\n",
     "PS: the entities are row-entity hashes"),
    # The realized arc's row removal reached without a realized arc.
    (LIB, "            macro_bend.is_some(),\n        ) {\n            // T4-U2 (H-2): a realized arc's pressure rows.\n",
     "            true,\n        ) {\n            // T4-U2 (H-2): a realized arc's pressure rows.\n", "PS: a case's rows only grow"),
    # A second MemberRows per member.
    (LIB, ROWS, ROWS + "            let mut spare_rows = u4b_rows::MemberRows::new(&pipe.element_id, member_start);\n",
     "MR: one MemberRows per member"),
    # MemberRows outside U4B-ST's station block (the block closed and another opened before it).
    (LIB, ROWS, "        }\n        {\n" + ROWS, "MR: one MemberRows per member"),
    # Seven kinds of slots, or slots reserved at construction.
    (U4B, "    slots: [[Vec<usize>; 5]; 6],\n", "    slots: [[Vec<usize>; 5]; 7],\n", "MR: 6 x 5 slots"),
    (U4B, "slots: Default::default() }", "slots: std::array::from_fn(|_| std::array::from_fn(|_| Vec::with_capacity(64))) }",
     "MR: 6 x 5 slots"),
    # Rows of other entities recorded, or a rescan from the start.
    (U4B, "        if r.entity_ref != rows.pipe {\n            continue;\n        }\n", "", "MR: one push per row"),
    (U4B, "    rows.scanned = results.len();\n", "", "MR: one push per row"),
    # Any other change to the index's span.
    (U4B, "    /// Called at the start of a member's iteration", "    /// Called at the start of each member's iteration",
     "published-row index changed"),
])
def test_a_changed_premise_stops(tmp_path, file, old, new, stop):
    snap = snapshot(tmp_path, file, old, new)
    with pytest.raises(SystemExit, match=stop):
        MODEL["published_index"](snap)
