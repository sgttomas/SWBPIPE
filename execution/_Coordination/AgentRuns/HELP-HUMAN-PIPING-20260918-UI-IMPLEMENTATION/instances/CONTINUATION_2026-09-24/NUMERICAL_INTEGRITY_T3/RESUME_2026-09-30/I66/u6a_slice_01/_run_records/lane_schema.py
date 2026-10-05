"""I66 U6a, D-U6-2 schema lane (scratch only): validate the Rust-produced successor
derivatives against results.v0.3 as committed (expected to refuse only the two new
reason codes) and against a lane copy carrying U6c's RowDisclosure delta."""
import json, sys
from copy import deepcopy
from pathlib import Path
LANE = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(LANE))
from tests.schema_validation import validate_instance
schema_path = LANE / "schemas/results.v0.3.schema.yaml"
committed = json.loads(schema_path.read_text())
delta = deepcopy(committed)
enum = delta["$defs"]["RowDisclosure"]["properties"]["reason_code"]["enum"]
enum += ["retained_precision_absolute_verified", "retained_precision_not_covered"]
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((OUT / f"derivative_{mode}.json").read_text())
    try:
        validate_instance(committed, doc, instance_label=f"{mode} derivative vs committed schema")
        print(f"{mode}: committed schema ACCEPTS (unexpected)")
    except AssertionError as error:
        text = str(error)
        print(f"{mode}: committed schema refuses: {text[:300]}")
    validate_instance(delta, doc, instance_label=f"{mode} derivative vs lane delta")
    print(f"{mode}: lane schema with the U6c RowDisclosure delta accepts the derivative")
    # Without the class codes (the base disposition) the document must still be the successor branch.
    stripped = deepcopy(doc); del stripped["result_envelope"]["retained_precision"]
    try:
        validate_instance(delta, stripped, instance_label="receipt dropped")
        print(f"{mode}: receipt-dropped derivative ACCEPTED (unexpected)")
    except AssertionError:
        print(f"{mode}: receipt-dropped derivative refused by the successor branch")
