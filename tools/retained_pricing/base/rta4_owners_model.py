"""RT-A4 D3 (DEL-05-02 Work absolute-sum-srss-combinations.md §10; retained-pricing-rules.md §8.46): the positive owners
RT-A4 adds outside the chain's TEXT inventory on D1, and the owners it changes.

D1.4 refuses on both routes every absolute_sum and srss basis and every term naming a combination, on route E every
combination, and every difference basis and subtrahend support state; D1.6 refuses nonlinear supports and constant
effort. On route L a subtraction's or envelope's id may name a combination (admitted since S1; evaluated since RT-A4),
so at z = 2 one combination can name the other (z - 1 <= 1 named combination; a cycle is refused before the solve).
Text (every String a TEXT site allocates) is the chain's; this model prices the containers and the struct clones the
TEXT inventory does not see. Every count is a D1 cap (c cases, z <= 2 combinations, <= 3 operands each, P = P_final rows
per case, g supports, PA = C_eq * P_final envelope rows, D diagnostics).

RT-A4-PL (combination_plan::Plan::new; route L, built by append_combination_modulus_basis_records at every z and by
  append_combination_results at z >= 1, each live for its caller's body; RT-A4-WZ zeros the withholding's): the
  resolver's case-id set (c, collected), index_of (z, inserted), order (z usize), named and done (z bools each), the
  per-combination targets (z Vec headers, <= 3 Target each) and dependencies (z headers, push-grown <= 3 usize), κ (z
  FixedLoadCount, each significand's capacity <= 130 limbs: an exact sum's magnitude is built with capacity
  long.len() + 1, long <= ceil((4,096 + 4,097) / 64) = 129 limbs after the bounded alignment), reached (z headers, <= c
  &str push-grown) and, one combination at a time, the κ computation's transient: <= 3 operand κ clones (<= 64 limbs
  each), the running sum and product (130 + 2 x 65 limbs), the two aligned operands of `plus` (shift_left: vec![0; whole]
  then pushes, capacity <= 256 limbs each), the magnitude and its normalization (2 x 130 limbs), and one `more` list (c),
  with one resolve()'s targets (<= 3 Target).
RT-A4-CB (append_combination_results, z >= 1, one combination at a time): the targets (<= 3), `out` (push-grown <= P + 8g
  ResultItem with its previous backing; moved into `results` whose growth is already priced), the withheld-combination
  map (<= z entries; key text is TEXT). At z >= 2, for the <= z - 1 named combination: `out_bases` (push-grown <= P + 8g
  Option<String>; base text is TEXT), published_by_base (<= P outer entries, each its base key's clone (<= 1,024 B, not a
  TEXT site) and a one-entry inner table holding a ResultItem clone with its row text) and published_rows (one entry, the
  combination's rows cloned whole with their text). The published maps live to the callee's return.
RT-A4-DV (preview_physics::combination_derivation, z >= 1, transient): the resolver set (c), the ordered list (z
  refs), the recorded set (<= z Strings; text is TEXT) and the records (z Values; their members are the derivation's
  Value, priced resident in the envelope at z >= 2 by derivation_value).
RT-A4-RD (the T17 V5 reader: preview_combination_gates::check, model-free; preview_combination_operands::check), every
  z: check's row index by id over the envelope's rows (PA), the operand check's case set (c), free_check's withheld set
  (<= z). At z >= 2 (a nested record): derivation()'s case set, positions and <= z records (operands and terms <= 3
  each), the records map, the κ graph (κ and reached per record) and, once (RT-A4-NR), the row index (tails and cases
  BTreeMaps over <= P base ids, each with a base String and a c-entry case table; the case rows (PA refs); by_tail; the
  published maps) and one re-derivation (`form`: <= P + 8g derived rows as Value trees of a row's size, the named
  withholdings, the support list and one operand's candidate list; derived_rows' expected, seen and stated maps).

Bound to the modules' complete bytes and the owning statements; a snapshot without combination_plan.rs adds nothing
(earlier registered trees keep their coefficients), a changed module stops the run."""
import hashlib
import os

from g3lib import B, X, Xc, s, pushcap, HashReq, HashMov

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
# Operating-case slice 1b D3 (§8.51): preview_combination_gates.rs's model-free check first calls
# preview_difference_basis::check, which on D1 returns before any allocation (PF1B-FZ: the producer's own successor
# makes no difference-basis claim); free_check's stated-reason match gains an allocation-free arm; modulus_key becomes
# pub(crate); model_check (model-bound: RT-A4-RM) reads the kept combinations and a declared gate from the new module.
# No RT-A4-RD owner changes.
PINS = {PP + "combination_plan.rs": "86e8d1c0686b92579aeee96087e0b6e390ee6729d10a56568228f099e64b6cbb",
        RE + "preview_combination_gates.rs": "b6c9ad33780eaaa2b2b59dd86d0cc53ff8c622525d5b612d8b252ea0084f8afe",
        RE + "combination_graph.rs": "f3a3ca6b332896afd19412ee96e18750c271ae323ca34c46876e382c710eaef8",
        "core/loads/load_case_algebra/src/fixed_load_count.rs": "c234ca4cb3fb6a1360edd8d3f55bd9d6e9d9ecf6a8310d68379096167abb38bf"}
# (file, statement): each exactly once in its file (the plan's owners, the callee's buffers, the readers' sets).
STATEMENTS = (
    (PP + "combination_plan.rs", "Resolver { cases: model.load_cases.iter().map(|case| case.id.as_str()).collect() }"),
    (PP + "combination_plan.rs", "let mut order = Vec::with_capacity(count);"),
    (PP + "combination_plan.rs", "let mut reached: Vec<Vec<&'m str>> = vec![Vec::new(); count];"),
    (PP + "preview_physics.rs", "let mut published_by_base: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();"),
    (PP + "preview_physics.rs", "let mut published_rows: HashMap<String, Vec<ResultItem>> = HashMap::new();"),
    (PP + "preview_physics.rs", "let mut out: Vec<ResultItem> = Vec::new();"),
    (PP + "preview_physics.rs", "let mut out_bases: Vec<Option<String>> = Vec::new();"),
    (PP + "preview_physics.rs", "published_by_base.entry(base.clone()).or_default().insert(combination.id.clone(), row.clone());"),
    (PP + "preview_physics.rs", "published_rows.insert(combination.id.clone(), out.clone());"),
    (PP + "preview_physics.rs", "results.extend(out);"),
    (PP + "lib.rs", "let plan = combination_plan::Plan::new(model);"),
    (RE + "preview_combination_gates.rs", "let by_id: HashMap<&str, &Value> = rows.iter().filter_map(|r| r[\"id\"].as_str().map(|id| (id, r))).collect();"),
    (RE + "preview_combination_gates.rs", "let index = index.get_or_insert_with(|| RowIndex::new(rows));"),
    (RE + "preview_combination_operands.rs", "let cases: HashSet<&str> = quality.iter().filter(|q| q[\"basis_ref\"][\"ref_type\"] == \"load_case\").filter_map(|q| q[\"basis_ref\"][\"ref_id\"].as_str()).collect();"),
)
TERMS = 3
SIG_LIMBS = 64            # <= 4,096 significand bits (FIXED_LOAD_COUNT_MAX_BITS)
STORED_LIMBS = 130        # a stored κ's significand capacity (module doc)
KAPPA_WORK_LIMBS = TERMS * SIG_LIMBS + STORED_LIMBS + 2 * 65 + 2 * 256 + 2 * STORED_LIMBS
RID = 1024


def _read(snapshot, rel):
    with open(os.path.join(snapshot, rel), encoding="utf-8") as stream:
        return stream.read()


def inspect(snapshot):
    """True on the reviewed RT-A4 source; False when combination_plan.rs is absent; anything else stops."""
    if not os.path.exists(os.path.join(snapshot, PP + "combination_plan.rs")):
        return False
    for rel, digest in PINS.items():
        with open(os.path.join(snapshot, rel), "rb") as stream:
            if hashlib.sha256(stream.read()).hexdigest() != digest:
                raise SystemExit(f"RT-A4 owners: {rel} changed; re-review §8.46's owners")
    for rel, text in STATEMENTS:
        count = _read(snapshot, rel).count(text)
        # lib.rs builds one plan, in append_combination_modulus_basis_records (the callee's is in preview_physics.rs).
        # The row index is built once per check, in free_check or (unreached: RT-A4-RM) model_check.
        if count != (2 if "RowIndex::new(rows)" in text else 1):
            raise SystemExit(f"RT-A4 owners: {text!r} occurs {count} times in {rel}; re-review §8.46's owners")
    # The reader's Operand and the producer's Target are the same two-variant enum of one &str (priced as s(Target)).
    target = "pub(crate) enum Target<'a> {\n    Case(&'a str),\n    Combination(&'a str),\n}"
    operand = "pub(crate) enum Operand<'a> {\n    Case(&'a str),\n    Combination(&'a str),\n}"
    if _read(snapshot, PP + "combination_plan.rs").count(target) != 1 or _read(snapshot, RE + "combination_graph.rs").count(operand) != 1:
        raise SystemExit("RT-A4 owners: Target or Operand changed shape; re-review s(Target)")
    # s(GateRecord) is an upper over the reader's private Record: two &str, two Vec headers and an Option<&str>.
    record = ("struct Record<'a> {\n    id: &'a str,\n    basis: &'a str,\n    operands: Vec<Operand<'a>>,\n    terms: Terms<'a>,\n"
              "    /// An envelope's mode.\n    mode: Option<&'a str>,\n}")
    gates = _read(snapshot, RE + "preview_combination_gates.rs")
    if gates.count(record) != 1 or gates.count("type Terms<'a> = Vec<(Operand<'a>, f64)>;") != 1:
        raise SystemExit("RT-A4 owners: the reader's Record changed; re-review s(GateRecord)")
    return True


def _kappa_stored(n):
    return s("FixedLoadCount") * n + B(8 * STORED_LIMBS * n)


def _kappa_work():
    return s("FixedLoadCount") * TERMS + B(8 * KAPPA_WORK_LIMBS)


def plan(cases, z):
    """RT-A4-PL: one Plan, resident for its caller's body, with its build's transient."""
    if not z:
        return HashReq("&str", cases)          # the resolver; every per-combination vector is empty (no allocation)
    resident = (HashReq("&str", cases) + HashMov("(&str,usize)", z) + B(8 * z) + B(z) + _kappa_stored(z)
                + s("Vec") * z + s("&str") * (pushcap(cases) * z))
    transient = (s("Vec") * z + s("Target") * (TERMS * z) + s("Vec") * z + B(8 * pushcap(TERMS) * z) + B(z)
                 + _kappa_work() + s("&str") * cases + s("Target") * TERMS + s("String") * TERMS)
    return resident + transient


def callee(cases, z, p, g):
    """RT-A4-CB: append_combination_results' owners beside its plan (z >= 1)."""
    if not z:
        return X()
    rows = p + 8 * g
    each = (s("Target") * TERMS + s("ResultItem") * (pushcap(rows) + pushcap(rows) // 2)
            + HashMov("(String,String)", z))
    named = max(0, z - 1)
    if named:
        each += s("String") * (pushcap(rows) + pushcap(rows) // 2)                 # out_bases
        published = (HashMov("PublishedBase", p) + B(p * RID)
                     + (HashReq("(String,ResultItem)", 1) + X({"Text(row)": 1})) * p
                     + HashReq("(String,Vec<f64>)", 1) + Xc("ResultItem", rows) + X({"Text(row)": rows}))
        each += published * named
    return plan(cases, z) + each


def derivation(cases, z):
    """RT-A4-DV: combination_derivation's transient (z >= 1)."""
    if not z:
        return X()
    return HashReq("&str", cases) + s("&Value") * z + HashMov("String", z) + s("Value") * z + s("String") * TERMS


def derivation_value(z):
    """The derivation member's Value (z >= 2: a nested record and its operand; none below): a three-key object, its
    z-entry array, and z records of three keys whose expression holds <= 3 terms of two keys (or a subtraction's two ids,
    or an envelope's <= 3 ids and its mode); ids IN128. As (arr, obj, ent, strb, keyb)."""
    if z < 2:
        return (0, 0, 0, 0, 0)
    arr = z + z * TERMS
    obj = 1 + z * (2 + TERMS)
    ent = 3 + z * (3 + 1 + 2 * TERMS)
    strb = z * (128 + 24 + TERMS * 128 + 8)
    keyb = len("nonlinear_supports") + len("consuming_constant_effort") + len("combinations") + z * (
        len("combination_id") + len("basis") + len("expression") + len("terms") + TERMS * (len("combination_id") + len("factor")))
    return (arr, obj, ent, strb, keyb)


def reader(cases, z, p, g, pa, d, row_tree):
    """RT-A4-RD: the T17 V5 reader's working sets. `row_tree` is one row's Value tree (g4_caps' per-row form)."""
    out = HashReq("(&str,&Value)", pa) + HashReq("&str", cases) + HashMov("&str", z)
    if z < 2:
        return out
    rows = p + 8 * g
    records = (HashReq("&str", cases) + HashReq("(&str,usize)", z) + s("GateRecord") * pushcap(z)
               + (s("Target") * TERMS + (s("Target") + B(8)) * TERMS + s("&str") * TERMS) * z
               + HashReq("(&str,&Value)", z) + HashMov("(&str,FixedLoadCount)", z) + B(8 * STORED_LIMBS * z)
               + HashMov("(&str,Vec<f64>)", z) + s("&str") * (pushcap(cases) * z) + _kappa_work() + s("FixedLoadCount") * TERMS)
    index = (X({"Node(String,(String,CaseRows))": 1 + p // 5}) + B(p * (2 * RID + 8)) + HashReq("(&str,&Value)", cases) * p
             + s("&Value") * (pushcap(pa) + pushcap(pa) // 2)
             + HashMov("(&str,PublishedTails)", z) + HashMov("(String,&Value)", rows) * z + B(rows * RID) * z
             + HashMov("(&str,Vec<f64>)", z) + s("&Value") * (pushcap(rows) + pushcap(rows) // 2) * z
             + X({"Node(String,CaseRows)": 1 + p // 5}) + HashMov("(String,String)", p) + B(p * (RID + 8))
             + HashMov("(String,&Value)", rows) * z + B(rows * RID) * z)
    formed = (s("Value") * (pushcap(rows) + pushcap(rows) // 2) + row_tree * rows
              + HashMov("(String,String)", rows) + s("&str") * pushcap(g) + s("&Value") * pushcap(pa) + s("String") * 8 * TERMS
              + HashMov("(&str,&Value)", rows) + HashMov("&str", rows) + HashMov("(&str,&str)", d) + s("&Value") * pushcap(d))
    return out + records + index + formed


# Illustrative strides for the chain's own check only (G5 evaluates every atom in-build).
ASSUMED = {"s(Target)": 24, "s(FixedLoadCount)": 40, "s(PublishedBase)": 72, "s((&str,FixedLoadCount))": 56,
           "s((String,&Value))": 32, "s((&str,&str))": 32, "s((&str,PublishedTails))": 64, "s(GateRecord)": 96,
           "Node(String,(String,CaseRows))": 1100, "Node(String,CaseRows)": 800}
