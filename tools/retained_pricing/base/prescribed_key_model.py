"""C7 borrowed prescribed-key owners; no visits, old owners or limits removed."""
import hashlib, os, re, runpy
HELPERS = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
FILE = "core/reporting/result_export/src/retained_precision.rs"
# T4-U6 (D-5): NumericCase gains `all_rows` (every published row, reduced twins included) and g5c gives each reduced
# twin its nominal row's class; the prescribed set, its borrowed pairs and their lifetimes are unchanged. The new
# field and its Vec are priced by U6-RS (`all_rows_field`, u6_owners_model.py and g4_caps.py).
# O4 S5 (retained-pricing-rules.md §8.9, O4-RD): g5c builds the case's distinct-basis member set (a BTreeSet of <= m
# borrowed ids, priced by O4-RD) and classes a distinct member's reduced rows as bound rows; the drain, the one
# `last` clone and the borrowed prescribed pairs are unchanged.
PINS = {"numeric_cases":"9342fa881a9a15724197456be1dab39570904d2e4120a5185b366a19e9163cd9",
        "numeric_owner":"60bf376d50b8b0b25dc9786a4b6cbf5fe1f50d93ff2dec6082eed1a68911e9b8",
        "g5c":"95b1c8577acf28e3d8b3b5fd997d68faadeb3c93db5d2c91e8bb8e1df6457355"}
OLD_PINS = {"numeric_cases":"9342fa881a9a15724197456be1dab39570904d2e4120a5185b366a19e9163cd9",
            "numeric_owner":"18c7b744c4a56801128f53852a85d66742ccde82fc062c13fc3eca53a3e352e1",
            "g5c":"e39869d4e6592eab543d717d3f87aa850a9f9a247750cf062a93c5f1f7ecd192"}
OLD_STRUCT_PIN = "05bb3df70804cb36a61c2bd1ccf080b12334f6ce608f64dba59b392207a8b830"
STRUCT_PIN = "036f0250d28707840ca7a9478d49401c105f602f32ed94a808a20b8941777c05"


def all_rows_field(snapshot):
    """U6-RS: whether NumericCase carries the D-5 `all_rows: Vec<&'a Value>` field (a tenth field)."""
    with open(os.path.join(snapshot, FILE), encoding="utf-8") as stream:
        source = stream.read()
    hits = list(re.finditer(r"^struct NumericCase<'a> \{", source, re.M))
    if len(hits) != 1:
        raise ValueError("C7: missing/duplicate NumericCase owner")
    struct = source[hits[0].start():source.find("\n}", hits[0].start()) + 2]
    return "    all_rows: Vec<&'a Value>," in struct.split("\n")


def borrowed(snapshot):
    with open(os.path.join(snapshot, FILE), encoding="utf-8") as stream:
        source = stream.read()
    hits = list(re.finditer(r"^struct NumericCase<'a> \{", source, re.M))
    if len(hits) != 1:
        raise ValueError("C7: missing/duplicate NumericCase owner")
    end = source.find("\n}", hits[0].start())
    struct = source[hits[0].start():end + 2]
    if "prescribed: BTreeSet<(String, String)>" in struct:
        # Known original owner/constructor/query bodies, not marker absence.
        if hashlib.sha256(struct.encode()).hexdigest() != OLD_STRUCT_PIN:
            raise ValueError("C7: changed legacy owner")
        for name, digest in OLD_PINS.items():
            body, _ = HELPERS["_indented_function"](source, name)
            if hashlib.sha256(body.encode()).hexdigest() != digest:
                raise ValueError("C7: changed legacy owner count/constructor/query: " + name)
        return False
    if hashlib.sha256(struct.encode()).hexdigest() != STRUCT_PIN:
        raise ValueError("C7: borrowed owner layout/source changed")
    for name, digest in PINS.items():
        body, _ = HELPERS["_indented_function"](source, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError("C7: borrowed owner count/constructor/query/lifetime changed: " + name)
    return True


def owned_formula(C, k, Q, tree_nodes, pushcap):
    # Full case Vec, all retained sets, actual set before rejection,
    # FromIterator's input/sort buffers, and source tuple carriers.
    return {"Node((&str,&str),())": C * tree_nodes(k) + tree_nodes(Q),
            "s(NumericCaseBorrowed)": pushcap(C),
            "s(BTreeSetBorrowed)": C + 1,
            "s(Vec)": 3,
            "s(PairBorrowed)": 2 * pushcap(Q) + 4,
            "s(NumericCaseResultBorrowed)": 2}
