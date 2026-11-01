"""Silent-drop guard D3 (DEL-04-07 Design pressure-contract.md, "Pressure-free input closure"; retained-pricing-rules.md
§8.19): the positive owners the guard adds outside the chain's TEXT inventory.

SD-WH: append_combination_results (the combination callee, route L with z >= 1) keeps, per combination and one
combination at a time, beside the W1-L5 index (sorted base_ids and cloned case_rows):
- `withheld`, a push-grown Vec<String> of at most one name per base id and one per support (<= PA + g), with its
  previous backing; the names' text is TEXT (their format! sites, SD-WT);
- the warning's list: `.take(NAMED).cloned()` clones <= 8 names (<= 1,052 B each, a result base id and a static reason)
  into an exact Vec<String>, which `.join` consumes (the joined text is a TEXT site);
- one support group's `rows`, a push-grown Vec<ResultItem> of <= 8 rows (six components and two magnitudes) with its
  previous backing, moved into `results` (its rows' texts are the unchanged combination-row sites).
SD-HS: validate_mechanics_combination's `seen`, a HashSet<(bool, &str)> of a combination's <= 3 term operands (model
document 0.9.0: was HashSet<&str> of case ids), grown by insertion with its previous table, transient within the call
(one combination at a time).

The scan of the closed records (model_fields.rs) owns no heap beyond its TEXT sites: `findings` stays empty on a permitted
call (SD-NF) and the record field lists are 'static slices read from the serde derives; each probe's boxed error text
("field probe", 11 B) lives only inside its one call (SD-PB, charged once).

Bound to the exact statements; a snapshot without the guard adds nothing (earlier registered trees keep their
coefficients), a partly present or changed owner stops the run."""
import os
import re

from g3lib import B, X, Xc, s, pushcap, HashMov

PP = "core/product_physics/src/"
PREVIEW = PP + "preview_physics.rs"
VAL = PP + "validation.rs"
FIELDS = PP + "model_fields.rs"
NAMED = 8
ENTRY = 1024 + len(" (load-case algebra refused)")          # 1,052: the longest withheld name (SD-WT)

# (file, function, statement): each once in its function (the probe's error twice, in its two methods).
STATEMENTS = (
    (PREVIEW, "append_combination_results", "let mut withheld: Vec<String> = Vec::new();"),
    (PREVIEW, "append_combination_results", "const NAMED: usize = 8;"),
    (PREVIEW, "append_combination_results", 'let mut named = withheld.iter().take(NAMED).cloned().collect::<Vec<_>>().join(", ");'),
    (PREVIEW, "append_combination_results", "let mut rows = Vec::new();"),
    (PREVIEW, "append_combination_results", "Ok(rows) => results.extend(rows),"),
    (VAL, "validate_mechanics_combination", "let mut seen = HashSet::new();"),
    # Model document 0.9.0 (§8.36, S1-HS): a term's operand is `(is_combination, id)`, borrowed by the local `fn operand`
    # (no allocation); the set and the earlier-occurrence count compare those pairs.
    (VAL, "validate_mechanics_combination", "if !seen.insert(operand(term)) {"),
    (VAL, "validate_mechanics_combination",
     "fn operand(term: &crate::PreviewCombinationTerm) -> (bool, &str) {match &term.combination {Some(id) => (true, id.as_str()),None => (false, term.load_case.as_str()),}}"),
    # The earlier-occurrence counts borrow and allocate nothing (SD-DL): each complete statement, line breaks removed.
    (VAL, "validate_mechanics_combination",
     "let earlier = combination.terms[..index].iter().filter(|other| operand(other) == operand(term)).count();"),
    (VAL, "validate_range_envelope_combination",
     "if operand_ids[..index].iter().filter(|other| *other == operand_id).count() != 1 {"),
    (FIELDS, "declared_fields", 'Err(de::Error::custom("field probe"))'),
)


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        return ""
    end = text.find("\n" + hits[0].group(1) + "}\n", hits[0].start())
    return text[hits[0].start():end] if end >= 0 else ""


def inspect(snapshot):
    """True when the guard's owners are present as reviewed; False on an earlier revision; anything else stops."""
    texts = {rel: _read(snapshot, rel) or "" for rel in (PREVIEW, VAL, FIELDS)}
    present = [re.sub(r"\s*\n\s*", "", _body(texts[rel], fn)).count(needle) for rel, fn, needle in STATEMENTS]
    if not any(present):
        return False
    expected = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
    if present != expected:
        raise SystemExit(f"SD owners: the guard's owning statements changed; re-review the owners they price: {present}")
    # The support group pushes its rows only into its own `rows` (two pushes: components and magnitudes).
    group = _body(texts[PREVIEW], "append_combination_results").split("let group = || -> Result<Vec<ResultItem>, &'static str> {", 1)
    if len(group) != 2 or group[1].split("Ok(rows)", 1)[0].count("rows.push(") != 2:
        raise SystemExit("SD owners: the support group's rows changed; re-review SD-WH")
    return True


def withheld_names(pa, g):
    """SD-WH (per combination, one at a time, in the combination callee)."""
    names = s("String") * (pushcap(pa + g) + pushcap(pa + g) // 2)
    listed = Xc("String", NAMED) + B(NAMED * max(8, 2 * ENTRY))
    rows = s("ResultItem") * (pushcap(NAMED) + pushcap(NAMED) // 2)
    return names + listed + rows


def seen_terms():
    """SD-HS (per mechanics combination validated, one at a time): <= 3 term operands. Model document 0.9.0 (S1-HS):
    each entry is a borrowed `(bool, &str)` operand, its own native atom."""
    return HashMov("(bool,&str)", 3)


def probe_error():
    """SD-PB: one probe's boxed 'field probe' error text at a time."""
    return B(max(8, 2 * len("field probe")))
