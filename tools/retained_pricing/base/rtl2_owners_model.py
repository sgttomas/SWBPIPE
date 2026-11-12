"""RT-L2 (pressure elongation per load case, model document 0.9.0's `model_settings.pressure_elongation`): the positive
owners the setting adds outside the chain's TEXT inventory, and the bound on its list that every one of them rests on.

RTL2-K (the list's bound). D1 has no clause or cap row that names `model_settings` (family_clauses and the census walk
read neither it nor its list), so an admitted request may carry `operating_load_cases` of any length the raw census
admits. Each element is one raw string value, so k <= D1.9's RawValues cap (retained_memory.rs `caps::RAW_VALUES`,
16,384), and the strings' total capacity is at most D1.9's RawStringCapacity cap (`caps::RAW_STRING_CAPACITY`, 131,072):
each typed string is a parse or a move of one raw string value, never longer. The setting is refused by validation
(without a pressure contract, empty, duplicated or naming no case) but it is admitted by D1 first, so every owner
below is live on the refusal path at that k. Both caps are read from the snapshot's caps module.

RTL2-TS (typed residence, T_resident and T25.1a with S1-TS): the parsed `Vec<String>` holds pushcap(k) String slots
(serde's sequence visitor grows by doubling without a size hint, exactly k with one) and at most RawStringCapacity bytes.

RTL2-VS (validation's refusal working set, BuildScratch, one validate_model_settings call at a time):
- `load_case_ids`, a HashSet<&str> collected over the c <= C load cases (one table, exact size hint);
- pressure_elongation_refusals' `ids` (Vec<&str>, pushcap(k)), `seen` (<= k entries), `duplicated` (<= k/2: an id is
  duplicated only at its second occurrence) and `unknown` (<= k), three push-grown hash sets live together, each with
  its previous table during its last resize;
- `refusals`, a push-grown Vec<ModelSettingError> ({&'static str, String}, the (&str,String) stride) of at most k + 1
  entries (the contract refusal, then at most one per list element: an element's first occurrence can be unknown, a
  later one a duplicate; the empty-list refusal only at k = 0), with its previous backing. Each message is a format!
  site the TEXT inventory prices at its loop multiplicity; it moves into the diagnostic.

RTL2-FS (the formulation statement, T_resident): ElongationStatement::of collects the named load cases as references,
at most C = 3, one push-grown backing of 4 pointer-width slots (32 B), transient while the formulation basis is formed;
charged as resident (an over-bound) because the basis is formed outside the solve phases.

RTL2-RD (the pressure reader on the projected successor, T17 V5, route E): assembly's key list, a vec! of 14 static keys
that may take two pushes (the region setting and the exception record; neither on D1), charged at 28 slots with the
previous 14; elongation_statement's collected statements (at most 3 of the 3 fixed statements, one backing of 4).

Bound to the exact allocating statements; a snapshot without the setting adds nothing, a partly present or changed
owner stops the run."""
import os
import re

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
VAL, MS, EA, PE, RM = PP + "validation.rs", PP + "model_settings.rs", PP + "exact_admission.rs", RE + "physics_evidence.rs", PP + "retained_memory.rs"

STATEMENTS = (
    (VAL, "let load_case_ids: HashSet<&str> = model.load_cases.iter().map(|case| case.id.as_str()).collect();", 1),
    (VAL, "let refusals = crate::model_settings::pressure_elongation_refusals(", 1),
    (VAL, "for (index, error) in refusals.into_iter().enumerate() {", 1),
    (MS, "    pub operating_load_cases: Vec<String>,", 1),
    (MS, "    pub pressure_elongation: Option<PressureElongationSetting>,", 1),
    (MS, "let mut refusals = Vec::new();", 1),
    (MS, "let ids: Vec<&str> = operating_load_cases.into_iter().collect();", 1),
    (MS, "let mut seen = std::collections::HashSet::new();", 1),
    (MS, "let mut duplicated = std::collections::HashSet::new();", 1),
    (MS, "let mut unknown = std::collections::HashSet::new();", 1),
    (MS, "refusals.push(ModelSettingError {", 4),
    (MS, "if !seen.insert(*id) && duplicated.insert(*id) {", 1),
    (MS, "if !load_case_ids.contains(id) && unknown.insert(*id) {", 1),
    (EA, "model.load_cases.iter().filter(|case| model.pressure_elongation_included(&case.id)).collect();", 1),
    (PE, "let mut shape = vec![", 1),
    (PE, "shape.push(", 2),
    (PE, "let stated: Vec<&str> = limitations.iter().filter_map(Value::as_str).filter(|text| statements.contains(text)).collect();", 1),
)
SHAPE_KEYS = 14


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    return open(path, encoding="utf-8").read() if os.path.exists(path) else None


def _production(text):
    return text.split("#[cfg(test)]", 1)[0] if text is not None else None


def inspect(snapshot):
    """The RT-L2 facts at this snapshot, {"k": list bound, "string_capacity": its bytes}; None when the snapshot has no
    pressure-elongation setting. Anything partly present or changed stops the run."""
    ms = _read(snapshot, MS) if snapshot else None
    if ms is None or "pub struct PressureElongationSetting {" not in ms:
        return None
    texts = {f: _production(_read(snapshot, f)) for f in (VAL, MS, EA, PE)}
    for f, statement, count in STATEMENTS:
        if texts[f] is None or texts[f].count(statement) != count:
            raise SystemExit(f"RTL2: {f}: {statement!r} changed (expected {count}); re-review the owner it prices")
    shape = texts[PE].split("let mut shape = vec![", 1)[1].split("];", 1)[0]
    if len(re.findall(r'"[a-z_]+",', shape)) != SHAPE_KEYS:
        raise SystemExit("RTL2-RD: assembly's key list changed; re-review the owner it prices")
    rm = _read(snapshot, RM)
    caps = {}
    for name in ("RAW_VALUES", "RAW_STRING_CAPACITY"):
        hit = re.findall(r"^    pub\(crate\) const " + name + r": usize = ([0-9_]+);$", rm, re.M)
        if len(hit) != 1:
            raise SystemExit(f"RTL2-K: caps::{name} not found once; re-review the list's bound")
        caps[name] = int(hit[0].replace("_", ""))
    # RTL2-K: neither the census walk nor the family clauses read model_settings (the bound is the raw census's).
    walk = rm.split("    fn walk(&mut self, request: &LinearStaticPreviewRequest)", 1)[1].split("\npub(super) fn nested_typed_census", 1)[0]
    family = rm.split("fn family_clauses(", 1)[1].split("\n}\n", 1)[0]
    if "model_settings" in walk or "pressure_elongation" in family or "model_settings" in family:
        raise SystemExit("RTL2-K: D1 now reads model_settings; re-derive the list's bound")
    return {"k": caps["RAW_VALUES"], "string_capacity": caps["RAW_STRING_CAPACITY"]}


def typed_list(facts):
    """RTL2-TS: the parsed list's String slots and string bytes."""
    from g3lib import Xc, B, pushcap
    return Xc("String", pushcap(facts["k"])) + B(facts["string_capacity"])


def validation_working_set(facts, cases):
    """RTL2-VS: validate_model_settings' setting refusal working set, at its largest."""
    from g3lib import s, HashReq, HashMov, pushcap
    k = facts["k"]
    refusals = pushcap(k + 1)
    return (HashReq("&str", cases) + s("&str") * pushcap(k) + HashMov("&str", k) + HashMov("&str", k // 2)
            + HashMov("&str", k) + s("(&str,String)") * (refusals + refusals // 2))


def formulation_statement():
    """RTL2-FS: ElongationStatement::of's named cases, one backing of 4 references."""
    from g3lib import B
    return B(4 * 8)


def reader_working_set():
    """RTL2-RD: assembly's key list (28 slots with the previous 14) and elongation_statement's statements (4)."""
    from g3lib import s
    return s("&str") * (2 * SHAPE_KEYS + SHAPE_KEYS + 4)
