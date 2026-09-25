#!/usr/bin/env node
// Transaction harness only: never compiles or executes product physics.
import { appendFileSync, readFileSync } from "node:fs";
import path from "node:path";
const root = process.env.FAULT_SANDBOX_ROOT;
if (!root || process.cwd() !== root) throw new Error("cargo stub outside its sandbox");
const args = process.argv.slice(2);
appendFileSync(path.join(root, "stub-commands.ndjson"), `${JSON.stringify({ program: "cargo-stub", args })}\n`);
if (args[0] === "metadata") {
  process.stdout.write(JSON.stringify({ packages: [{ name: "transaction_only_local_stub", version: "0.0.0", source: null,
    manifest_path: path.join(root, "core/product_physics/Cargo.toml") }] }));
} else if (args[0] === "run" && args.includes("preview_result") && args.at(-2) === "--") {
  const suffix = args.at(-1) === "sparse_interactive" ? "sparse" : args.at(-1) === "dense_scrutiny" ? "dense" : null;
  if (!suffix) throw new Error("unexpected stub mode");
  process.stdout.write(readFileSync(path.join(root, "stub-payloads", `${suffix}.stdout.json`)));
} else if (args.length === 1 && args[0] === "--version") {
  process.stdout.write("cargo 0.0.0 (transaction-only-stub)\n");
} else throw new Error(`unexpected cargo-stub command: ${args.join(" ")}`);
