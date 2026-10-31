#!/usr/bin/env node
// Drives the product's self-weight generator (core/loads/self_weight_wasm, which runs
// core/product_physics/src/self_weight.rs) for the reference-system fixture. Input on stdin:
// {"glue": <dir of the wasm-bindgen --target nodejs glue>, "model": <model>, "requests": [<request
// without source_model_hash>...]}. Output on stdout: one plan outcome per request, in order.
// The source model hash is the RFC 8785 canonical JSON's sha256, which for these documents (no
// integer beyond 2^53) is JSON.stringify of the value with object keys sorted by UTF-16 code units,
// the form core/serialization/canonical_json renders. Called by generate_reference_system.py
// --capture-self-weight; it computes no weight itself.
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import path from "node:path";

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value !== null && typeof value === "object") {
    const keys = Object.keys(value).sort();
    return `{${keys.map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

let input = "";
for await (const chunk of process.stdin) input += chunk;
const { glue, model, requests } = JSON.parse(input);
const engine = createRequire(import.meta.url)(path.join(glue, "open_pipe_stress_self_weight_wasm.js"));
const hash = `sha256:${createHash("sha256").update(canonical(model), "utf8").digest("hex")}`;
const modelJson = JSON.stringify(model);
const outcomes = requests.map((request) =>
  JSON.parse(engine.generate_self_weight_plan_json(modelJson, JSON.stringify({ ...request, source_model_hash: hash })))
);
process.stdout.write(JSON.stringify(outcomes));
