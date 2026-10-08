"""I98 B2-W, W-CB1: which reader check refuses the proxy's precommit (read-only; VENV).

Usage: python py_reader_d6b.py <archive PY reader path> <precommit dump>...

For each precommit dump the probe wrote (`{"source": successor, "invocation": invocation}`):
1. the archive's accepted Python reader, unchanged: its first failure (it should equal the
   Rust reader's precommit refusal);
2. the same reader text with exactly one check neutralised in memory, D6b's
   `fail(quality[i]["solve_quality"] in ("sensitive", "unresolved", "failed"))`
   (a selected *case* must not be ordinarily `checks_passed`), executed with the original
   `__file__` so its packaged statics resolve. Nothing is written; the archive file is unchanged.
If (2) passes, D6b is the only check the proxy fails, and every other gate (G0-G8) passes.
"""
import hashlib
import json
import sys
import types

D6B = 'fail(quality[i]["solve_quality"] in ("sensitive", "unresolved", "failed"))'


def load_module(path, patch):
    text = open(path, encoding="utf-8").read()
    if patch:
        assert text.count(D6B) == 1, "D6b's line is not unique"
        text = text.replace(D6B, "fail(True)  # I98 probe: D6b neutralised in memory")
    # Executed as a member of the reader's own package, so its relative imports resolve.
    module = types.ModuleType("core.analysis_runs.retained_precision_probe" + ("_d6b" if patch else ""))
    module.__file__ = path
    module.__package__ = "core.analysis_runs"
    sys.modules[module.__name__] = module
    exec(compile(text, path, "exec"), module.__dict__)
    # Locate a failure: the reader's line that called the failing check (stack frames in the
    # reader's own text; the reader's behaviour is unchanged).
    original = module.__dict__["_need"]
    def located(ok, gate, code, *args, **kwargs):
        if not ok:
            frames = [f for f in __import__("traceback").extract_stack()[:-1] if f.filename == path]
            print(f"  PY_READER_FIRST_FAILURE gate={gate} code={code} reader_lines={[f.lineno for f in frames][-3:]} "
                  f"text={[f.line for f in frames][-1:]}")
        return original(ok, gate, code, *args, **kwargs)
    module.__dict__["_need"] = located
    return module, hashlib.sha256(text.encode("utf-8")).hexdigest()


def main():
    reader, dumps = sys.argv[1], sys.argv[2:]
    print(f"reader_sha256={hashlib.sha256(open(reader, 'rb').read()).hexdigest()}")
    for patch in (False, True):
        module, text_sha = load_module(reader, patch)
        for dump in dumps:
            doc = json.load(open(dump, encoding="utf-8"))
            label = dump.rsplit("/", 1)[-1]
            try:
                verdict = module.validate_retained_precision(doc["source"], doc["invocation"])
                keys = {k: verdict[k] for k in verdict if k in ("invocation_bound", "numerical_eligible", "eligible")}
                classes = verdict.get("classifications")
                print(f"PY_READER {'D6b_neutralised' if patch else 'unchanged'} {label} text_sha={text_sha[:16]} PASS {keys} classifications={len(classes) if classes is not None else None}")
            except Exception as error:  # the reader raises its first failure
                print(f"PY_READER {'D6b_neutralised' if patch else 'unchanged'} {label} text_sha={text_sha[:16]} FAIL {type(error).__name__}: {str(error)[:300]}")


if __name__ == "__main__":
    main()
