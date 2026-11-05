"""RT-A3 §4 D3 (DEL-05-02 Work combination-certificate.md; retained-pricing-rules.md §8.53): the positive owners the
combination certificate adds outside the chain's TEXT inventory on D1.

D1.9: route E refuses every combination and route L admits z <= 2, so at the profile's points L(3,0) and E(3) (z = 0)
the certificate holds only RTA3C-CI; every other owner here exists only at z >= 1 (L(2,1), L(1,2)), where the point
must stay at or below the merged tree. Text (every String a TEXT site allocates: ids, labels, row-id clones, messages)
is the chain's (o10_rules.py RTA3C-*); this model prices the containers, the Value trees serde_json builds from borrowed
text (json! of a &str copies it, which no TEXT site sees) and the struct strides. Counts are D1 caps: c cases, z <= 2
combinations of <= 3 load-case terms (D1.4), G = 28(m + g) K-D5 groups per case (KD5L1/KD5L2), a group's <= 2 row sets
of <= 3 rows, PA envelope rows, ZR = z(P + 8g + 1) combination rows (RT-A4-CB's <= P + 8g and the modulus record), RID
1,024 B a result id, IN 128 B an id, LABEL the KD5T2 label bound.

RTA3C-CI (every z; run_linear_static_preview_observed_inner): `case_ids`, the model's c case ids as &str, collected
  with exact capacity after the envelope is assembled and live to the function's return (publish and observer.finish
  run inside it). The run's operand map is HashMap::new() and allocates nothing until an insert (z >= 1).
RTA3C-OP (z >= 1; the case solves to aggregation): each formed operand's group vector (<= G OperandGroup, push-built)
  for <= min(c, 3z) cases, held in its LoadCaseSolve and then the run's map (its entries); operand's transient
  member_scales map (<= m + g bodies of ActionScales).
RTA3C-PB (z >= 1; publish's transient): the gates (z), the case-row index over PA rows (one Vec<usize> each), the
  combination rows by sources and by combination (ZR), the case order's serde copy of numerical_quality.cases and the
  array clone taken from it while it lives (two trees), and its ids, the used cases, the records and diagnostics
  vectors, and one combination at a time: its rows (P + 8g + 1), its terms and missing list, the derived vector
  (<= 2G) with each Derived's own `rows` backing (<= 3 usize), the formed map (<= 2G), the covered row set (a
  HashSet<usize> of <= min(6G, P + 8g + 1) rows), one group's per-operand coverage, spec pair and exact arithmetic.
  The Value copies (§8.53 S1): json! interpolates by serialization (`to_value(&expr)`), so each nesting copies what it
  wraps. The record push holds the derived Values, their collected clone and the record's copy of that (the record
  is the resident member's part); the operand-case build holds one case's collected group Values and one group's
  rows Values beside their copies; `json!({"operand_cases": .., "combinations": records})` copies the whole member
  while both source vectors live to the return. Charged: the derived groups twice and one member tree beyond the
  resident one, one case's groups and one group's rows, each with the row-id bytes the chain's TEXT row-id clones
  (RTA3C-RS) do not hold. `pressure` clones the evidence's pressure array: absent on route L (D1.5), it allocates
  nothing there.
RTA3C-MV (z >= 1; resident in the envelope and so in the successor): the member Value: <= min(c, 3z) operand cases of
  G group objects (8 entries; rows: <= 2 slots, each null or an array of <= 3 ids) and z records (8 entries, an
  expression of <= 3 terms) of <= 2G derived group objects (14 entries; rows: an array of <= 3 ids; equal_pressure
  null or an outer {elastic_group} object whose value is null or the inner 4-entry object {entity, location, group,
  row_set}; on route L always null, D1.5, priced at its largest form).
RTA3C-RD (z >= 1; the G7 base read's member_check, route L): the row index (by id, by case key, by sources, by
  combination), the case order and set, the operand map (<= min(c, 3z) vectors of <= G groups), one case's duplicate
  set (G) and one group's expected rows (a Vec<Value> of <= 2 slots, each null or an array of <= 3 ids) beside its
  json! copy, and one record at a time: its rows, terms and missing list, the derived vector (each `rows` backing
  <= 3 usize) and formed map (<= 2G), the covered row set, the two spec vectors of cloned (kind, location) Values, the
  derived Value trees (strings copied from &str by json!), their collected clone and the expected record's copy (three
  trees), the expected diagnostics (z), the used cases' id clones, the stated and wanted case lists and the published
  list.

Bound to the two modules' complete bytes (RTA3C-PIN's digests) and the owning statements; a snapshot without the
producer module adds nothing; a changed module stops the run."""
import hashlib
import os

from g3lib import B, X, Xc, s, pushcap, HashReq, HashMov

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
# The RE pin follows its unit tests reading the legacy W-CB3 successor (§8.53; one test line, in place).
PINS = {PP + "combination_certificate.rs": "5b85e6c57ae9c3b7cf9722e8be424532e12866f8b825597b20334c3e98ab5b1a",
        RE + "combination_certificate.rs": "6c08cf1703e9cb1cb6ee8a98a32d5347f68cd4502a86299885628b8990e4efd9"}
# (file, statement): each exactly once in its file.
STATEMENTS = (
    (PP + "lib.rs", "let case_ids: Vec<&str> = model.load_cases.iter().map(|case| case.id.as_str()).collect();"),
    (PP + "lib.rs", "let mut certificate_operands: HashMap<String, combination_certificate::Operand> = HashMap::new();"),
    (PP + "combination_certificate.rs", "let index = CaseIndex::new(results);"),
    (PP + "combination_certificate.rs", "let scales = formation_guard::member_scales(records, bodies);"),
    (PP + "combination_certificate.rs", "let pressure: Vec<Value> = evidence.get(\"pressure\").and_then(Value::as_array).cloned().unwrap_or_default();"),
    (RE + "combination_certificate.rs", "let rows = Rows::new(results);"),
)
TERMS = 3
SETS = 2
SET_ROWS = 3
RID = 1024
IN = 128
# serde_json Value-tree strides (illustrative here; G5 evaluates every atom in-build, as g4_caps' VF does).
VALUE_ENTRY_KEY = 24          # a map key String header (its bytes are keyb)


def _read(snapshot, rel):
    with open(os.path.join(snapshot, rel), encoding="utf-8") as stream:
        return stream.read()


def inspect(snapshot):
    """True on the reviewed source; False when the producer module is absent; anything else stops."""
    if not os.path.exists(os.path.join(snapshot, PP + "combination_certificate.rs")):
        return False
    for rel, digest in PINS.items():
        with open(os.path.join(snapshot, rel), "rb") as stream:
            if hashlib.sha256(stream.read()).hexdigest() != digest:
                raise SystemExit(f"RT-A3 §4 owners: {rel} changed; re-review §8.53's owners")
    for rel, text in STATEMENTS:
        if _read(snapshot, rel).count(text) != 1:
            raise SystemExit(f"RT-A3 §4 owners: {text!r} is not once in {rel}; re-review §8.53's owners")
    return True


def _groups(m, g):
    return 28 * (m + g)


def _used(c, z):
    return min(c, TERMS * z)


def case_ids(c):
    """RTA3C-CI: the case-id vector, every z."""
    return s("&str") * c


def operands(c, z, m, g):
    """RTA3C-OP: the formed operands' group vectors and the run's map entries (z >= 1)."""
    if not z:
        return X()
    G = _groups(m, g)
    used = _used(c, z)
    return (s("C3OperandGroup") * (pushcap(G) * used) + HashMov("C3OperandEntry", used)
            + HashReq("(usize,ActionScales)", m + g))


# The member's Value-tree shapes (the producer's json! literals; RTA3C-PIN binds them). A derived group object: 14
# entries; its rows an array of <= SET_ROWS ids; equal_pressure at most the outer one-entry {elastic_group} object
# holding the inner four-entry {entity, location, group, row_set} object.
DERIVED_KEYS = ("entity", "location", "group", "row_set", "rows", "factors_bound", "rounding_error", "bound", "q", "q_hat",
                "floor_scale", "verdict", "reason", "equal_pressure")
ELASTIC_KEYS = ("entity", "location", "group", "row_set")
OPERAND_GROUP_KEYS = ("entity", "location", "group", "moment", "rows", "q", "bound", "floor_scale")


def _derived_value(G, label):
    """<= 2G derived group objects of one combination (n = 2G), each: its object (14 entries), one slot in `groups`
    and <= 3 row-id slots in `rows` (the ids' bytes are the chain's row-id clones, RTA3C-RS); entity, location, group,
    verdict and reason strings copied; equal_pressure at most an outer one-entry object and the inner four-entry
    object (entity, location, group "axial", row_set)."""
    n = 2 * G
    return dict(arr=n * (1 + SET_ROWS), obj=3 * n, ent=(len(DERIVED_KEYS) + 1 + len(ELASTIC_KEYS)) * n,
                strb=n * (IN + 2 * label + len("not_established") + len("bound_exceeds_criterion") + IN + label + len("axial")),
                keyb=n * (sum(map(len, DERIVED_KEYS)) + len("elastic_group") + sum(map(len, ELASTIC_KEYS))),
                nums=8 * n)


def _row_id_bytes(used, z, G):
    """The member's row-id string bytes: <= 3 per derived group (2G per combination) and <= 2 x 3 per operand group."""
    return RID * SET_ROWS * (z * 2 * G + used * G * SETS)


def member_value(c, z, m, g, label):
    """RTA3C-MV: the resident member Value as (arr, obj, ent, strb, keyb, nums); zeros below z = 1."""
    if not z:
        return (0, 0, 0, 0, 0, 0)
    return member_counts(_used(c, z), z, _groups(m, g), label)


def member_counts(used, z, G, label):
    """The member Value's counts for `used` operand cases of <= G groups and z records (the strings exclude the row ids,
    _row_id_bytes)."""
    d = _derived_value(G, label)
    arr = 2 + used * (1 + G * (1 + SETS + SETS * SET_ROWS)) + z * (1 + TERMS) + z * d["arr"]
    obj = 1 + used * (1 + G) + z * (2 + TERMS) + z * d["obj"]
    ent = 2 + used * (2 + 8 * G) + z * (8 + 3 + 2 * TERMS) + z * d["ent"]
    strb = used * (IN + G * (IN + 2 * label)) + z * (IN + len("result_state_subtraction") + len("not_assessed") + 429 + 13
                                                      + TERMS * IN) + z * d["strb"]
    keyb = (len("operand_cases") + len("combinations") + used * (len("load_case_id") + len("groups")
            + G * sum(map(len, OPERAND_GROUP_KEYS)))
            + z * (len("combination_id") + len("basis") + len("expression") + len("subtrahend_support_state") + len("assessment")
                   + len("reason") + len("groups") + len("uncovered_row_count")
                   # the expression: {terms: [{load_case_id | combination_id, factor}]} or {minuend_id, subtrahend_id}
                   + max(len("terms") + TERMS * (len("combination_id") + len("factor")), len("minuend_id") + len("subtrahend_id")))
            + z * d["keyb"])
    nums = used * G * 4 + z * (1 + TERMS) + z * d["nums"]
    return (arr, obj, ent, strb, keyb, nums)


def publish(c, z, m, g, p, pa, value_tree):
    """RTA3C-PB: publish's transient beside the member it builds (z >= 1). `value_tree(arr, obj, ent, strb, keyb)` is the
    caller's Value-tree form."""
    if not z:
        return X()
    G = _groups(m, g)
    zr = z * (p + 8 * g + 1)
    rows = p + 8 * g + 1
    used = _used(c, z)
    # The case order's to_value tree and the array clone taken while it lives.
    quality = value_tree(2 * (c + z), 4 * (c + z), 24 * (c + z), (c + z) * (4 * RID + 512), (c + z) * 24 * 32) * 2
    one_combination = (B(8 * pushcap(rows)) + s("C3Derived") * pushcap(2 * G) + HashMov("C3FormedEntry", 2 * G)
                       + s("C3PerOperand") * pushcap(TERMS) + B(TERMS * (SETS * 24 + 4 * 8 * pushcap(SET_ROWS))) + B(4 * 576)
                       + _combination_owners(G, rows) + s("(&str,&str)") * (2 * SET_ROWS)
                       # The derived Values and their collected clone beside the record's copy (the clone's row ids).
                       + _value(_derived_value(G, 64), value_tree) * 2 + B(2 * G * SET_ROWS * RID))
    # The whole member copied by json! while records and operand_cases live (with its row ids), one case's collected
    # group Values and one group's rows Values (each beside its copy).
    member_copy = value_tree(*member_counts(used, z, G, 64)[:5]) + B(_row_id_bytes(used, z, G))
    case_groups = (value_tree(G * (1 + SETS + SETS * SET_ROWS), G, len(OPERAND_GROUP_KEYS) * G, G * (IN + 2 * 64),
                              G * sum(map(len, OPERAND_GROUP_KEYS))) + B(_row_id_bytes(1, 0, G)))
    group_rows = value_tree(SETS * (1 + SET_ROWS), 0, 0, SETS * SET_ROWS * RID, 0)
    return (s("(String,bool)") * z + HashMov("C3CaseKey", pa) + B(8 * pushcap(1) * pa)
            + HashMov("C3SourcesKey", zr) + B((TERMS * 16 + 8 * pushcap(1)) * zr)
            + HashMov("C3RowsOf", z) + B(8 * pushcap(rows) * z)
            + quality + s("String") * c + B(IN * c) + HashReq("String", used) + B(IN * used)
            + s("Value") * pushcap(z) + s("Diagnostic") * pushcap(z) + one_combination
            + member_copy + case_groups + group_rows)


def _value(d, value_tree):
    return value_tree(d["arr"], d["obj"], d["ent"], d["strb"], d["keyb"])


def _combination_owners(G, rows):
    """One combination's owners outside the Value trees (both modules): each Derived's `rows` backing (<= 3 usize, exact
    capacity), the covered row set (<= min(6G, rows) usize), the terms (<= 3 (String, f64)) and the missing list."""
    return (s("usize") * (2 * G * SET_ROWS) + HashReq("usize", min(2 * G * SET_ROWS, rows))
            + s("(String,f64)") * TERMS + s("&str") * pushcap(TERMS))


def reader(c, z, m, g, p, pa, value_tree):
    """RTA3C-RD: the G7 base read's member_check working set (z >= 1)."""
    if not z:
        return X()
    G = _groups(m, g)
    zr = z * (p + 8 * g + 1)
    rows = p + 8 * g + 1
    used = _used(c, z)
    label = 0
    index = (HashMov("(&str,usize)", pa) + HashMov("C3CaseKey", pa) + B(8 * pushcap(1) * pa)
             + HashMov("C3SourcesKey", zr) + B((TERMS * 16 + 8 * pushcap(1)) * zr) + HashMov("C3RowsOf", z) + B(8 * pushcap(rows) * z))
    cases = s("&str") * pushcap(c) + HashReq("&str", c)
    operand_map = HashMov("C3ROperandEntry", used) + s("C3ROperandGroup") * (pushcap(G) * used)
    # One group's expected rows (<= 2 slots, each null or an array of <= 3 ids copied from &str) and its json! copy.
    one_case = HashMov("C3Triple", G) + value_tree(SETS * (1 + SET_ROWS), 0, 0, SETS * SET_ROWS * RID, 0) * 2
    d = _derived_value(G, 128)
    derived_tree = value_tree(d["arr"], d["obj"], d["ent"], d["strb"] + 2 * G * SET_ROWS * RID, d["keyb"])
    # The two spec vectors of (kind, location) Value clones (<= 3 each; a kind or location string <= 64 B).
    specs = s("Value") * (2 * 2 * SET_ROWS) + B(2 * SET_ROWS * 2 * 64)
    one_record = (B(8 * pushcap(rows)) + s("C3RDerived") * pushcap(2 * G) + HashMov("C3FormedEntry", 2 * G)
                  + derived_tree * 3 + s("C3PerOperand") * pushcap(TERMS) + B(TERMS * (SETS * 24 + 4 * 8 * pushcap(SET_ROWS))) + B(4 * 576)
                  + _combination_owners(G, rows) + specs)
    tail = (s("Value") * pushcap(z) + value_tree(z, z, 6 * z, z * (64 + 32 + 16), z * 40) + s("&Value") * pushcap(z) + HashReq("String", used)
            + B(IN * used) + s("&str") * (2 * pushcap(used)))
    return index + cases + operand_map + one_case + one_record + tail + B(label)


# Illustrative strides for the chain's own check only (G5 evaluates every atom in-build).
ASSUMED = {"s(C3OperandGroup)": 112, "s(C3OperandEntry)": 48, "s((usize,ActionScales))": 24, "s(C3CaseKey)": 104,
           "s(C3SourcesKey)": 64, "s(C3RowsOf)": 40, "s(C3Derived)": 88, "s(C3FormedEntry)": 88, "s(C3PerOperand)": 40,
           "s((String,bool))": 32, "s(C3ROperandEntry)": 40, "s(C3ROperandGroup)": 80, "s(C3Triple)": 48, "s(C3RDerived)": 72,
           "s((String,f64))": 32}
