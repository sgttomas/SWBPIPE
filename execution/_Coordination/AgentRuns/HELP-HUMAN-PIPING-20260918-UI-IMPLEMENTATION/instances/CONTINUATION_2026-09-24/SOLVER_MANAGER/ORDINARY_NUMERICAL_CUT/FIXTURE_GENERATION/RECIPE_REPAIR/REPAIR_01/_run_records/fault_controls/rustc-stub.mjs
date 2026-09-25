#!/usr/bin/env node
// Tool-version response only; no compiler invocation.
if (process.argv.length !== 3 || process.argv[2] !== "--version") throw new Error("unexpected rustc stub command");
process.stdout.write("rustc 0.0.0 (transaction-only-stub)\n");
