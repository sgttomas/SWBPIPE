"""RT-U4 D3 (RTU4-RC): the positive owner the exact reader's one-pass region binding adds.

RTU4-RC: validate_physics_evidence_in (result_export physics_evidence.rs; route E's G7 base read of the projection,
T17 V5) counts the published pipe rows by (load case, member) once, `pipe_rows: HashMap<(&str, &str), usize>`, and
checks each region's result ids against the counts. Its keys are pairs of a counted row's case and entity; a counted row
has a load-case basis and a pipe kind, and the row loop before it resolved every such row's case among the exact cases
(`row_case`) and its entity among that case's members (`ROW_MEMBER_UNRESOLVED`), so there are at most cases*m keys. It
grows by insertion (`entry(..).or_insert(0)`) from `HashMap::new`, so its previous table is live during its last resize
(HashMov). Its entry is a (&str, &str) key, s(PairBorrowed), and a usize count, 8 B: the pair's stride plus 8 B per slot,
no new layout symbol. It is held to the function's end, beside U4B-RD's, W7-RD's and W15-RB's sets, so it is charged
in V5 with them. It replaces the per-region `expected` set, which was built inside the region loop (bound 0 on route E:
the projection's evidence has no pressure region).

Bound to the exact statements; a snapshot without RT-U4's count adds nothing (earlier registered trees keep their
coefficients), a partly present or changed owner stops the run."""
import os
import re

PHYS = "core/reporting/result_export/src/physics_evidence.rs"
FN = "validate_physics_evidence_in"

# Each statement in the function, line breaks and their indentation removed, with its count.
STATEMENTS = (
    "let mut pipe_rows: HashMap<(&str, &str), usize> = HashMap::new();",
    "for row in rows.values() {if let Some(case) = case_basis_ok(row) {if pipe_kind(row[\"kind\"].as_str().unwrap_or(\"\")) {"
    "*pipe_rows.entry((case, row[\"entity_ref\"].as_str().unwrap_or(\"\"))).or_insert(0) += 1;}}}",
    "n.checked_add(pipe_rows.get(&(case, *member)).copied().unwrap_or(0))",
)


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text or "", re.M))
    if len(hits) != 1:
        return ""
    end = text.find("\n" + hits[0].group(1) + "}\n", hits[0].start())
    return text[hits[0].start():end] if end >= 0 else ""


def inspect(snapshot):
    """True when RT-U4's count is present as reviewed; False on an earlier revision; anything else stops."""
    path = os.path.join(snapshot, PHYS)
    text = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
    body = re.sub(r"\s*\n\s*", "", _body(text, FN))
    present = [body.count(statement) for statement in STATEMENTS]
    if not any(present) and "pipe_rows" not in body:
        return False
    if present != [1] * len(STATEMENTS) or body.count("pipe_rows") != 3:
        raise SystemExit(f"RT-U4 owners: the region binding's count changed; re-review RTU4-RC: {present}")
    return True


def reader_count(keys):
    """RTU4-RC: the count map over `keys` (load case, member) keys, push-grown: s(PairBorrowed) and an 8 B count per
    slot of its final table and of the previous table live during its last resize."""
    from g3lib import B, HashMov, buckets
    b = buckets(keys)
    prev = b // 2 if b > 4 else 0
    return HashMov("PairBorrowed", keys) + B(8 * (b + prev))
