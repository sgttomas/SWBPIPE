"""I75 T6S: identical, env-gated output-dump instrumentation for a scratch copy (base or candidate).

Usage: python3 instrument.py <copy>/projects/chirality-piping
Dumps one JSON line per builder output to $I75_DUMP: kind, route, sha256 of the inputs'
JSON text and sha256 of the output's JSON text (the bytes a download would carry, modulo
indentation). Never applied to the worktree.
"""
import sys, pathlib
root = pathlib.Path(sys.argv[1]) / "apps/desktop/src/features"
HELPER = '''import { appendFileSync as __i75append } from "node:fs";
import { createHash as __i75hash } from "node:crypto";
function __i75dump(kind: string, input: unknown, output: unknown): void {
  const path = (globalThis as any).process?.env?.I75_DUMP; if (!path) return;
  const h = (v: unknown) => __i75hash("sha256").update(JSON.stringify(v)).digest("hex");
  let route = "?"; try { const i = input as any; route = sourceContract(i.result ?? i.source); } catch { route = "!"; }
  __i75append(path, JSON.stringify({ kind, route, input: h(input), output: h(output) }) + "\\n");
}
'''
sn = root / "stress-neutral/StressNeutralExportPanel.tsx"
s = sn.read_text()
assert s.count("\n  return packet;\n") == 1
s = s.replace("\n  return packet;\n", "\n  __i75dump('sn', args, packet);\n  return packet;\n")
s = HELPER + s
sn.write_text(s)
re_ = root / "result-export/resultExportAdapter.ts"
s = re_.read_text()
old = "await validateResultDocument(doc,source);return doc;"
assert s.count(old) == 1
s = s.replace(old, "await validateResultDocument(doc,source);__i75dump('rd',{base,model,source,origin,request},doc);return doc;")
assert s.count("\n  return document;\n") == 1
s = s.replace("\n  return document;\n", "\n  __i75dump('rc',{model,result,analysisRun,inputManifest},document);\n  return document;\n")
s = HELPER + s
re_.write_text(s)
print("instrumented", sn, re_)
