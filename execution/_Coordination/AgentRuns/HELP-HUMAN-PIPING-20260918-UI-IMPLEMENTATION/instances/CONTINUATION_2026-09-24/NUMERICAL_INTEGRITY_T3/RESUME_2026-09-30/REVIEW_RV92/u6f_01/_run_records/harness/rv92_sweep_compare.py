"""RV92: base-vs-candidate comparison of an existing-behaviour sweep.
argv: base.jsonl cand.jsonl label. Plain committed inputs must be identical;
injected forms (ids containing '!') are listed by field when they change."""
import collections, json, sys

B = {d["id"]: d for d in map(json.loads, open(sys.argv[1]))}
C = {d["id"]: d for d in map(json.loads, open(sys.argv[2]))}
label = sys.argv[3]
plain_changed, injected_changed = [], collections.Counter()
examples = collections.defaultdict(list)
for k in B:
    if k not in C:
        plain_changed.append((k, "missing in cand"))
        continue
    diff = sorted(f for f in set(B[k]) | set(C[k]) if B[k].get(f) != C[k].get(f))
    if not diff:
        continue
    if "!" in k:
        form = k.split("!")[-1]
        for f in diff:
            injected_changed[(form, f)] += 1
            if len(examples[(form, f)]) < 2:
                examples[(form, f)].append((k[:90], str(B[k].get(f))[:120], str(C[k].get(f))[:120]))
    else:
        plain_changed.append((k, diff))
print(f"[{label}] inputs base {len(B)} cand {len(C)}; plain (committed) inputs changed: {len(plain_changed)}")
for k, d in plain_changed[:20]:
    print("   PLAIN", k[:100], d)
print(f"[{label}] injected forms: changed fields by form")
for (form, f), n in sorted(injected_changed.items()):
    print(f"   {form:14s} {f:16s} {n}")
    for e in examples[(form, f)]:
        print("        e.g.", e)
unchanged_injected = sum(1 for k in B if "!" in k and k in C and B[k] == C[k])
print(f"[{label}] injected inputs fully unchanged: {unchanged_injected} of {sum(1 for k in B if '!' in k)}")
