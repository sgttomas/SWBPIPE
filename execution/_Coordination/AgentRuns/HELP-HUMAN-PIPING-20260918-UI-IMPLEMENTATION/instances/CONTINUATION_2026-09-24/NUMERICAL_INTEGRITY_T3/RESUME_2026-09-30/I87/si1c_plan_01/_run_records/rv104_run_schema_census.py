"""I87 read-only census (not repository content).

Validates the full-JSON point-run lines ('p') of RV104's candidate runner dump
against the committed schemas/rule_check_run_result.schema.json, and classifies
the failures by JSON path. Reads only the dump and the committed schema.
"""
import gzip
import json
import sys
from collections import Counter

import jsonschema

dump_path, schema_path = sys.argv[1], sys.argv[2]
schema = json.load(open(schema_path))
validator = jsonschema.Draft202012Validator(schema)
lines = invalid = 0
by_path = Counter()
by_status = Counter()
example = {}
with gzip.open(dump_path, "rt") as fh:
    for line in fh:
        parts = line.rstrip("\n").split("\t")
        if len(parts) < 4 or parts[1] != "p":
            continue
        lines += 1
        doc = json.loads(parts[3])
        errors = list(validator.iter_errors(doc))
        if errors:
            invalid += 1
            for err in errors:
                path = "/".join(str(p) if not isinstance(p, int) else "#" for p in err.absolute_path)
                key = (path, err.validator, repr(err.instance)[:20])
                by_path[key] += 1
                example.setdefault(key, parts[0])
            for check in doc.get("checks", []):
                if any(
                    (check.get("computed_value") or {}).get("value", 0) is None
                    or any(b.get("supplied") and "value" in b and b["value"] is None for b in check["bound_inputs"])
                    for _ in [0]
                ):
                    by_status[check["status"]] += 1
print("point-run lines", lines, "schema-invalid", invalid)
for key, count in by_path.most_common():
    print(count, key, "e.g.", example[key])
print("statuses of checks carrying a null value:", dict(by_status))
