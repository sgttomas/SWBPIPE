#!/usr/bin/env python3
"""I66 U7 slice F: move P/fixtures/results/retained_precision_carrier_cases.json from v3
(as 07i's patch leaves it, sha256 f20a7db0...) to v4. Adds D-U7-4's declared-difference
entry, adopted from I67's slice-T draft with its two fields ('capture', 'current_model_edits';
RR "The memory branch merged into NUM; U7 slices T and P committed"), documents v4 in the
note, and corrects the note's one phrase that U7 makes false ("the held withheld count").
Everything else is unchanged (asserted)."""
import hashlib, json, sys
from pathlib import Path

P = Path(sys.argv[1])
DRAFT = Path(sys.argv[2])
PATH = P / "fixtures/results/retained_precision_carrier_cases.json"
raw = PATH.read_bytes()
assert hashlib.sha256(raw).hexdigest() == "f20a7db0d76192ffb3350adbb3a22899ca5b9d9a9b350d3cd246ba992c173280", "07i's case file"
v3 = json.loads(raw)
assert v3["format"] == "I66-U6-CARRIER-CASES-v3" and len(v3["declared_differences"]) == 5
entry = json.loads(DRAFT.read_text())["entry"]
assert entry["id"] == "D-U7-4:ts_requires_live_native_capture" and len(entry["forms"]) == 2

held = "interval_bindable 0, the held withheld count; or 'empty')"
assert v3["note"].count(held) == 1
note = v3["note"].replace(held, "interval_bindable 0, the not-Current withheld count (these forms carry no invocation); or 'empty')")
note += (" v4 (I66 U7 slice F): a form may carry 'capture' ('none': TS registers the form's invocation without an IPC "
         "capture, so no live native capture exists; Rust and Python read the invocation argument exactly as for "
         "'invocation': 'fixture') and 'current_model_edits' (edits, op 'set', applied only to TS's current model, which is "
         "otherwise the captured invocation's model; Rust and Python have no current-model input beyond 'requested', so "
         "these edits change nothing they read). Every consumer reads both fields explicitly; any other unknown field is a defect.")
v4 = {"format": "I66-U6-CARRIER-CASES-v4", "note": note}
for key in ("scope", "fixtures", "cases"):
    v4[key] = v3[key]
v4["declared_differences"] = v3["declared_differences"] + [entry]
PATH.write_text(json.dumps(v4, indent=2) + "\n")
print(hashlib.sha256(PATH.read_bytes()).hexdigest())
