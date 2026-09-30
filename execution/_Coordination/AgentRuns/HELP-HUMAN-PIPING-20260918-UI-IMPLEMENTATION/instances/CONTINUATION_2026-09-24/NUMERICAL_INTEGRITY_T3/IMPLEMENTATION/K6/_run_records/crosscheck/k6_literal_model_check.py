"""K6 B2 addendum: the literal `references.py --model <id>` output equals the
imported model_json(defn, full=True) that k6_crosscheck.py compares, for the
six 10-member RF-LARGE cases and RF-LARGE-CONT-n10000-ROT.

Usage: python3 k6_literal_model_check.py <references.py>. Standard library only.
"""
import importlib.util
import json
import subprocess
import sys

IDS = ['RF-LARGE-%s-n00010-%s' % (f, o) for f in ('CHAIN', 'TREE', 'CONT') for o in ('AX', 'ROT')]
IDS.append('RF-LARGE-CONT-n10000-ROT')


def main():
    path = sys.argv[1]
    spec = importlib.util.spec_from_file_location('references', path)
    refs = importlib.util.module_from_spec(spec)
    sys.modules['references'] = refs
    spec.loader.exec_module(refs)
    refs.build_large()
    failures = 0
    for case_id in IDS:
        case = next(c for c in refs.CASES if c['id'] == case_id)
        imported = json.dumps(refs.model_json(case['defn'], full=True), indent=1)
        literal = subprocess.run([sys.executable, path, '--model', case_id], capture_output=True,
                                 text=True, check=True).stdout.rstrip('\n')
        same = literal == imported
        failures += not same
        print('%s %s' % ('EQUAL' if same else 'DIFFERS', case_id))
    print('summary: %d of %d equal' % (len(IDS) - failures, len(IDS)))
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
