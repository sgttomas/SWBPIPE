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
  combination rows by sources and by combination (ZR), the case order's serde copy of numerical_quality.cases and its
  ids, the used cases, the records and diagnostics vectors, and one combination at a time: its rows (P + 8g + 1), the
  derived vector (<= 2G) and formed map (<= 2G), and one group's per-operand coverage and exact arithmetic.
RTA3C-MV (z >= 1; resident in the envelope and so in the successor): the member Value: <= min(c, 3z) operand cases of
  G group objects (8 entries; rows: <= 2 arrays of <= 3 ids) and z records (8 entries, an expression of <= 3 terms) of
  <= 2G derived group objects (14 entries; rows <= 3 ids; equal_pressure null or a 4-entry object).
RTA3C-RD (z >= 1; the G7 base read's member_check, route L): the row index (by id, by case key, by sources, by
  combination), the case order and set, the operand map (<= min(c, 3z) vectors of <= G groups), one case's duplicate
  set (G) and expected rows Value, and one record at a time: its rows, the derived vector and formed map (<= 2G), the
  derived Value trees (strings copied from &str by json!) and the expected record's clone of them, the expected
  diagnostics (z) and the published list.

Bound to the two modules' complete bytes (RTA3C-PIN's digests) and the owning statements; a snapshot without the
producer module adds nothing; a changed module stops the run."""
import hashlib
import os

from g3lib import B, X, Xc, s, pushcap, HashReq, HashMov

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
PINS = {PP + "combination_certificate.rs": "5b85e6c57ae9c3b7cf9722e8be424532e12866f8b825597b20334c3e98ab5b1a",
        RE + "combination_certificate.rs": "0f688c6e80faad61888e8e677ad8a6c03256631858c0d4f797d9af77d3542f0f"}
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


def _derived_value(G, label):
    """<= 2G derived group objects of one combination: 14 entries, <= 3 row ids (their bytes are the chain's row-id
    clones), entity, location, group, verdict and reason strings copied; equal_pressure a 4-entry object at most."""
    n = 2 * G
    return dict(arr=n, obj=2 * n, ent=(14 + 4) * n, strb=n * (IN + 2 * label + len("not_established") + len("bound_exceeds_criterion") + IN + label),
                keyb=n * (len("entity") + len("location") + len("group") + len("row_set") + len("rows") + len("factors_bound")
                          + len("rounding_error") + len("bound") + len("q") + len("q_hat") + len("floor_scale") + len("verdict")
                          + len("reason") + len("equal_pressure") + len("elastic_group") + len("entity") + len("location") + len("group")),
                nums=8 * n)


def member_value(c, z, m, g, label):
    """RTA3C-MV: the resident member Value as (arr, obj, ent, strb, keyb, nums); zeros below z = 1."""
    if not z:
        return (0, 0, 0, 0, 0, 0)
    G = _groups(m, g)
    used = _used(c, z)
    d = _derived_value(G, label)
    arr = 2 + used * (1 + G * (1 + SETS)) + z * (1 + TERMS) + z * d["arr"]
    obj = 1 + used * (1 + G) + z * (2 + TERMS) + z * d["obj"]
    ent = 2 + used * (2 + 8 * G) + z * (8 + 3 + 2 * TERMS) + z * d["ent"]
    strb = used * (IN + G * (IN + 2 * label)) + z * (IN + len("result_state_subtraction") + len("not_assessed") + 429 + 13
                                                      + TERMS * IN) + z * d["strb"]
    keyb = (len("operand_cases") + len("combinations") + used * (len("load_case_id") + len("groups")
            + G * (len("entity") + len("location") + len("group") + len("moment") + len("rows") + len("q") + len("bound") + len("floor_scale")))
            + z * (len("combination_id") + len("basis") + len("expression") + len("subtrahend_support_state") + len("assessment")
                   + len("reason") + len("groups") + len("uncovered_row_count") + TERMS * (len("load_case_id") + len("factor")))
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
    quality = value_tree(2 * (c + z), 4 * (c + z), 24 * (c + z), (c + z) * (4 * RID + 512), (c + z) * 24 * 32)
    one_combination = (B(8 * pushcap(rows)) + s("C3Derived") * pushcap(2 * G) + HashMov("C3FormedEntry", 2 * G)
                       + s("C3PerOperand") * pushcap(TERMS) + B(TERMS * (SETS * 24 + 4 * 8 * pushcap(SET_ROWS))) + B(4 * 576))
    return (s("(String,bool)") * z + HashMov("C3CaseKey", pa) + B(8 * pushcap(1) * pa)
            + HashMov("C3SourcesKey", zr) + B((TERMS * 16 + 8 * pushcap(1)) * zr)
            + HashMov("C3RowsOf", z) + B(8 * pushcap(rows) * z)
            + quality + s("String") * c + B(IN * c) + HashReq("String", _used(c, z)) + B(IN * _used(c, z))
            + s("Value") * pushcap(z) + s("Diagnostic") * pushcap(z) + one_combination)


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
    one_case = HashMov("C3Triple", G) + value_tree(SETS + 1, 0, SETS * SET_ROWS, SETS * SET_ROWS * RID, 0)
    d = _derived_value(G, 128)
    derived_tree = value_tree(d["arr"], d["obj"], d["ent"], d["strb"] + 2 * G * SET_ROWS * RID, d["keyb"])
    one_record = (B(8 * pushcap(rows)) + s("C3RDerived") * pushcap(2 * G) + HashMov("C3FormedEntry", 2 * G)
                  + derived_tree * 2 + s("C3PerOperand") * pushcap(TERMS) + B(TERMS * (SETS * 24 + 4 * 8 * pushcap(SET_ROWS))) + B(4 * 576))
    tail = s("Value") * pushcap(z) + value_tree(z, z, 6 * z, z * (64 + 32 + 16), z * 40) + s("&Value") * pushcap(z) + HashReq("String", used)
    return index + cases + operand_map + one_case + one_record + tail + B(label)


# Illustrative strides for the chain's own check only (G5 evaluates every atom in-build).
ASSUMED = {"s(C3OperandGroup)": 112, "s(C3OperandEntry)": 48, "s((usize,ActionScales))": 24, "s(C3CaseKey)": 104,
           "s(C3SourcesKey)": 64, "s(C3RowsOf)": 40, "s(C3Derived)": 88, "s(C3FormedEntry)": 88, "s(C3PerOperand)": 40,
           "s((String,bool))": 32, "s(C3ROperandEntry)": 40, "s(C3ROperandGroup)": 80, "s(C3Triple)": 48, "s(C3RDerived)": 72}
