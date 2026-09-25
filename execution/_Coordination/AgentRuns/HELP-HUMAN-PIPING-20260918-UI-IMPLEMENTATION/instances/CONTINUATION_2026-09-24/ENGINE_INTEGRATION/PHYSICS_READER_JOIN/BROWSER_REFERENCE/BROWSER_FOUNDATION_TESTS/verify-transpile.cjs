// Run from the repository root; no app, browser, npm or build execution.
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const ts = require(path.resolve("projects/chirality-piping/node_modules/typescript"));
const files = ["projects/chirality-piping/apps/desktop/e2e/ui-foundation.spec.ts", "projects/chirality-piping/apps/desktop/e2e/ui-foundation-dist.spec.ts"];
const outcomes = files.map(file => {
  const source = fs.readFileSync(file, "utf8");
  const output = ts.transpileModule(source, { fileName: file, reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, isolatedModules: true } });
  return { file, sha256: crypto.createHash("sha256").update(source).digest("hex"), diagnostics: (output.diagnostics || []).map(d => ({ code: d.code, category: ts.DiagnosticCategory[d.category], message: ts.flattenDiagnosticMessageText(d.messageText, "\n") })) };
});
const report = { validation: "source-only TypeScript transpile; not runtime or semantic type check", command: process.argv.slice(1), node: process.version, typescript: ts.version, files: outcomes };
fs.writeFileSync(path.join(__dirname, "TRANSPILE.json"), JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
if (outcomes.some(r => r.diagnostics.some(d => d.category === "Error"))) process.exitCode = 1;
