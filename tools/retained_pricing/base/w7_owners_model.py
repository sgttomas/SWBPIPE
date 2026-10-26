"""W-7 D3 (DEL-04-01 Design global-end-actions.md; retained-pricing-rules.md §8.20): the positive owners the member end
actions in global coordinates add outside the chain's TEXT inventory.

W7-NC: prepared_not_covered (freeze_case and, since W-7, freeze_combination: one result set at a time) reserves one
[[Option<usize>; 6]; 2] per member through the adapter (exact), the bound positions of the member's end-action rows
that its global end rows rotate. An array's size is its element's times its length, so the table is exactly 12
Option<usize> per member (the in-build atom s(Option<usize>)). It is live while that result set's not-covered rows
complete, beside U4B-W1's operand table, so it is charged per attempted result set with T14.
W7-QF: qualify_source_case_rows (route E, an exact case with a capture) first collects the case's rows other than the
global end rows as references, `rows.iter().filter(..).collect()`: a filter's size hint has lower bound 0, so the
Vec<&ResultItem> push-grows to pushcap(R0) slots of 8 B with the previous backing of its last reallocation, live
through the qualification (beside "Source row qualification"'s map and cloned rows).

The rows themselves (twelve per member and case, and twelve placeholders per member in the failed manifest) are inside
P_final and R0 (W7-RL, W7-RP); their texts are TEXT sites.
W7-RD: the readers' coverage check (result_export global_end_evidence.rs `coverage`, called by the preview-physics-2 and
pressure-2 base readers on the projected successor, G7) keys four BTreeMaps by (load case, member), K <= cases*m keys of
load-case rows: the element-local and global end values ([[Option<f64>; 6]; 2] each), the wall actions
([Option<f64>; 2]) and the wall-basis flags ([BTreeSet<bool>; 2], each set holding at most both flags in one node); a
HashSet of the K member keys (collected from a filter: grown by insertion, with its previous table); and one transient
one-flag BTreeSet per end comparison. Each map is charged at K keys, also where it is empty on D1 (no wall rows). append_element_global_end_results' working values
(`local`, `wall`, `values`) are stack arrays, and the aggregation's `retain` drops rows in place.

Bound to the exact statements; a snapshot without W-7 adds nothing (earlier registered trees keep their coefficients),
a partly present or changed owner stops the run."""
import os
import re

PP = "core/product_physics/src/"
LIB = PP + "lib.rs"
RP = PP + "retained_product.rs"
END_ACTION_SLOTS = 12                                      # [[Option<usize>; 6]; 2]

# (file, function, statement, count): each statement in its function, line breaks removed.
STATEMENTS = (
    (RP, "prepared_not_covered", "let mut end_actions=self.adapter.reserve::<[[Option<usize>;6];2]>(self.members.len())?;", 1),
    (RP, "prepared_not_covered", "for _ in 0..self.members.len() {self.capture_entry(AdapterEvent::MapWrite)?;end_actions.push([[None;6];2]);}", 1),
    (LIB, "qualify_source_case_rows", "let rows: Vec<&ResultItem> = rows.iter().filter(|row| !is_global_end_kind(&row.kind)).collect();", 1),
    (LIB, "append_element_global_end_results", "let mut local = [[None::<f64>; 6]; 2];", 1),
    (LIB, "append_element_global_end_results", "let mut wall = [false; 2];", 1),
    (LIB, "append_element_global_end_results", "let mut values = [0.0; 6];", 1),
    (LIB, "append_element_global_end_results", "results.push(ResultItem {", 1),
    (LIB, "run_linear_static_preview_observed_inner", "solve.results.retain(|row| !is_global_end_kind(&row.kind));", 1),
    # W7-L1's window: the member's first row position, taken just before its element-local end rows, and the call.
    (LIB, "solve_prepared_load_case", "let member_rows_from = results.len();append_element_force_results(&mut results, &pipe.element_id, &corrected_local_forces);", 1),
    (LIB, "solve_prepared_load_case", "append_element_global_end_results(&mut results, member_rows_from, &pipe.element_id, &axes)", 1),
    (RP, "failed_manifest", "let member_rows_from=rows.len();super::append_element_force_results(&mut rows,&member.id,&[0.0;12]);", 1),
    (RP, "failed_manifest", "super::append_element_global_end_results(&mut rows,member_rows_from,&member.id,&[IDENTITY;2])", 1),
)


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text or "", re.M))
    if len(hits) != 1:
        return ""
    end = text.find("\n" + hits[0].group(1) + "}\n", hits[0].start())
    return text[hits[0].start():end] if end >= 0 else ""


def inspect(snapshot):
    """True when W-7's owners are present as reviewed; False on an earlier revision; anything else stops."""
    texts = {rel: _read(snapshot, rel) or "" for rel in (LIB, RP)}
    present = [re.sub(r"\s*\n\s*", "", _body(texts[rel], fn)).count(needle) for rel, fn, needle, _ in STATEMENTS]
    if not any(present):
        return False
    if present != [count for *_, count in STATEMENTS]:
        raise SystemExit(f"W7 owners: the global end rows' owning statements changed; re-review the owners they price: {present}")
    # The only heap the function owns is its pushed rows and their texts (an empty `Vec::new()` allocates nothing): no
    # collect, to_vec, vec!, cloned or with_capacity.
    body = re.sub(r"\s*\n\s*", "", _body(texts[LIB], "append_element_global_end_results"))
    if body.count("Vec::new()") != 1 or any(token in body for token in (".collect", ".to_vec()", "vec![", ".cloned()", "with_capacity")):
        raise SystemExit("W7 owners: append_element_global_end_results gained an owner; re-review W7-NC/W7-QF")
    return True


# Illustrative only (the chain's own check): g5_profile binds each node to btree_node_upper at the in-build layouts.
ASSUMED_NODES = {"Node((&str,&str),[[Option<f64>;6];2])": 2600, "Node((&str,&str),[Option<f64>;2])": 816,
                 "Node((&str,&str),[BTreeSet<bool>;2])": 980, "Node(bool,())": 120, "s(PairBorrowed)": 32}


def reader_coverage(keys):
    """W7-RD: one coverage call's maps and set over `keys` (load case, member) keys."""
    from g3lib import X, HashMov, tree_nodes
    k = tree_nodes(keys)
    return (X({"Node((&str,&str),[[Option<f64>;6];2])": 2 * k}) + X({"Node((&str,&str),[Option<f64>;2])": k})
            + X({"Node((&str,&str),[BTreeSet<bool>;2])": k}) + X({"Node(bool,())": 2 * keys + 1})
            + HashMov("PairBorrowed", keys))


def end_action_operands(m):
    """W7-NC: prepared_not_covered's end-action table, one [[Option<usize>; 6]; 2] per member (adapter reserve, exact)."""
    from g3lib import Xc   # the pricing chain's algebra; inspect() alone also runs from the route rules
    return Xc("Option<usize>", END_ACTION_SLOTS * m)


def source_filter(r0):
    """W7-QF: qualify_source_case_rows' filtered references, push-grown to pushcap(R0) slots with its previous backing."""
    from g3lib import B, pushcap
    return B(8 * pushcap(r0) + 8 * (pushcap(r0) // 2))
