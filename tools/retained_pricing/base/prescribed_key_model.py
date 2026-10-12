"""C7 borrowed prescribed-key owners; no visits, old owners or limits removed."""
import hashlib, os, re, runpy
HELPERS = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
FILE = "core/reporting/result_export/src/retained_precision.rs"
PINS = {"numeric_cases":"9342fa881a9a15724197456be1dab39570904d2e4120a5185b366a19e9163cd9",
        "numeric_owner":"d4a532e72a5029ae4dafd47e6d71d4332d5842d3c8ee8951d08e3d619a8f4d29",
        "g5c":"bb8d6cac00d054a60d0a2aa20353ec57b75ce3901449e23209f2273c404fef37"}
OLD_PINS = {"numeric_cases":"9342fa881a9a15724197456be1dab39570904d2e4120a5185b366a19e9163cd9",
            "numeric_owner":"18c7b744c4a56801128f53852a85d66742ccde82fc062c13fc3eca53a3e352e1",
            "g5c":"e39869d4e6592eab543d717d3f87aa850a9f9a247750cf062a93c5f1f7ecd192"}
OLD_STRUCT_PIN = "05bb3df70804cb36a61c2bd1ccf080b12334f6ce608f64dba59b392207a8b830"
STRUCT_PIN = "240d5cede006c54d302a0e39dba35251b0ceab8545ce18837ac5e563eb025df1"


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
