"""RT-L2 (pressure elongation per load case, model document 0.9.0's `model_settings.pressure_elongation`): the positive
owners the setting adds outside the chain's TEXT inventory, and the bound on its list that every one of them rests on.

RTL2-K (the list's bounds). Two bounds, by where the work runs relative to D1.4's elongation clause.
- At the census (parse time). The closed-field check (lib.rs `run_linear_static_preview_value_dispatch`,
  `model_fields::unknown_field_refusal`), `CapturedInvocation::parse` (source_receipt.rs, `serde_json::from_value` of the
  raw clone: the typed list is allocated here) and the censuses (`retained_memory::admit`: `borrowed_value_census`,
  `borrowed_request_census`, `raw_text_census`, `nested_typed_census`, `combination_census`) all run before
  `domain_clauses`, so the typed list they see or form is bounded only by the raw census: each element is one raw string
  value, so k <= D1.9's RawValues cap (`caps::RAW_VALUES`, 16,384), and the strings' total capacity is at most D1.9's
  RawStringCapacity cap (`caps::RAW_STRING_CAPACITY`, 131,072): each typed string is a parse or a move of one raw
  string value, never longer. RTL2-TS stays at this bound, in T_resident and in T25.1a (the latter is `requested()`'s
  re-derivation inside the permitted run, after the clause; it is kept at the census as an over-bound, because its
  string bytes have no per-string capacity cap below RawStringCapacity and only its slots would fall).
- After the clause (the permitted run). `family_clauses` (called once by `domain_clauses`, called once by `admit` before
  `law_order`, which applies `domain?` before it returns the permit's index) declines a document whose
  `operating_load_cases` is longer than its load cases, from the two lengths alone; D1.4's 1 <= c <= C check stands in
  the same function, and like it the clause is allocation-free, so their order within the domain is immaterial.
  `run_linear_static_preview_value_dispatch` enters `permitted_dispatch` only with a permit, and model validation
  (`validate_model_inputs` -> `validate_model_settings`) runs only inside the permitted run's ordinary run, never in
  retained_memory.rs, source_receipt.rs or model_fields.rs. So on every permitted run validation sees k <= c <= C
  (`caps::LOAD_CASES`, 3): RTL2-VS and RTL2-VL (o10_rules.py) are priced at k = C.
The guard (`_clause_guard`) stops the run if the clause is removed or changed, leaves `family_clauses`, or the call chain
that makes it precede validation (family_clauses <- domain_clauses <- admit before law_order; admit before
permitted_dispatch; no validation call on the admission side) changes. All caps are read from the snapshot's caps module.

RTL2-TS (typed residence, T_resident and T25.1a with S1-TS): the parsed `Vec<String>` holds pushcap(k) String slots
(serde's sequence visitor grows by doubling without a size hint, exactly k with one) and at most RawStringCapacity bytes.

RTL2-VS (validation's refusal working set, BuildScratch, one validate_model_settings call at a time; k <= C by the clause):
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
    """The RT-L2 facts at this snapshot, {"k_census": the parse-time list bound, "string_capacity": its bytes, "k": the
    list bound after D1.4's elongation clause}; None when the snapshot has no pressure-elongation setting. Anything partly
    present or changed stops the run."""
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
    for name in ("RAW_VALUES", "RAW_STRING_CAPACITY", "LOAD_CASES"):
        hit = re.findall(r"^    pub\(crate\) const " + name + r": usize = ([0-9_]+);$", rm, re.M)
        if len(hit) != 1:
            raise SystemExit(f"RTL2-K: caps::{name} not found once; re-review the list's bound")
        caps[name] = int(hit[0].replace("_", ""))
    # RTL2-K (census side): neither the census walk nor the nested census reads model_settings; the parse-time bound is
    # the raw census's.
    walk = rm.split("    fn walk(&mut self, request: &LinearStaticPreviewRequest)", 1)[1].split("\npub(super) fn nested_typed_census", 1)[0]
    if "model_settings" in walk:
        raise SystemExit("RTL2-K: the census now reads model_settings; re-derive the list's bound")
    _clause_guard(snapshot, rm)
    return {"k_census": caps["RAW_VALUES"], "string_capacity": caps["RAW_STRING_CAPACITY"], "k": caps["LOAD_CASES"]}


CLAUSE = """    if m.model_settings.as_ref().and_then(|settings| settings.pressure_elongation.as_ref())
        .is_some_and(|setting| setting.operating_load_cases.len() > m.load_cases.len())
    {
        return refuse(C::Invocation, F::PressureElongationCases);
    }
"""
CASES_CHECK = """    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES {
        return refuse(C::Invocation, F::LoadCases);
    }
"""


def _fn_body(text, head):
    """The production body of the one top-level function whose signature starts with `head`, up to its closing brace
    at column 0."""
    if text.count(head) != 1:
        raise SystemExit(f"RTL2-K: {head!r} not found once; re-review the clause's placement")
    return text.split(head, 1)[1].split("\n}\n", 1)[0]


def _code(text):
    """The text with its line comments removed (the guard's needles are code, never comment or message text). The whole
    file is read: retained_memory.rs and lib.rs hold `#[cfg(test)]` items before the functions the guard binds."""
    return re.sub(r"//[^\n]*", "", text or "")


def _before(text, first, second):
    """`first` and `second` each occur once in `text`, `first` earlier."""
    return text.count(first) == 1 and text.count(second) == 1 and text.index(first) < text.index(second)


def _clause_guard(snapshot, rm):
    """RTL2-K (clause side): D1.4's elongation clause stands in family_clauses after the load-case count check, the
    clause is part of the domain that gates the permit, and the permit gates the run in which validation runs."""
    rm_p = _code(rm)
    family = _fn_body(rm_p, "fn family_clauses(request: &LinearStaticPreviewRequest) -> Result<(), AdmissionRefusal> {")
    domain = _fn_body(rm_p, "pub(super) fn domain_clauses(")
    law = _fn_body(rm_p, "pub(super) fn law_order(")
    admit = _fn_body(rm_p, "pub(super) fn admit(")
    lib = _code(_read(snapshot, PP + "lib.rs"))
    dispatch = _fn_body(lib, "fn run_linear_static_preview_value_dispatch(")
    checks = {
        "clause once, in family_clauses": rm_p.count("F::PressureElongationCases") == 1 and family.count(CLAUSE) == 1,
        # k <= c, and the load-case count check gives c <= C; both are allocation-free domain clauses, so their order
        # within the domain does not matter, only that both gate the permit.
        "beside the load-case count check": family.count(CASES_CHECK) == 1,
        "domain_clauses calls family_clauses once": domain.count("family_clauses(request)?;") == 1 and rm_p.count("family_clauses(") == 2,
        "admit forms the domain once, before law_order": rm_p.count("domain_clauses(") == 2
            and _before(admit, "let domain = domain_clauses(", "let verdict = law_order(caller, build, domain,"),
        "law_order applies the domain before its permit": _before(law, "    domain?;\n", "Ok(index)"),
        "dispatch admits before the permitted run": dispatch.count("retained_memory::admit(&capture, &request, entry, limit)") == 1
            and dispatch.count("Some(Ok((permit, report))) => return permitted_dispatch(permit, report, request, capture, solver_mode),") == 1
            and _before(dispatch, "retained_memory::admit(", "permitted_dispatch("),
        "no validation on the admission side": all("validate_model_inputs" not in _code(_read(snapshot, PP + f))
                                                   and "validate_model_settings" not in _code(_read(snapshot, PP + f))
                                                   for f in ("retained_memory.rs", "source_receipt.rs", "model_fields.rs"))
            and "validate_model" not in dispatch,
    }
    failed = [k for k, ok in checks.items() if not ok]
    if failed:
        raise SystemExit(f"RTL2-K: D1.4's elongation clause or its precedence over validation changed ({failed}); "
                         "re-derive RTL2-VS and RTL2-VL at the census")


def typed_list(facts):
    """RTL2-TS: the parsed list's String slots and string bytes."""
    from g3lib import Xc, B, pushcap
    return Xc("String", pushcap(facts["k_census"])) + B(facts["string_capacity"])


def validation_working_set(facts, cases):
    """RTL2-VS: validate_model_settings' setting refusal working set, at its largest, at k = C (the clause)."""
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
