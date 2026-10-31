#!/usr/bin/env node
// Interim adapter for the desktop perf runs (RT-U4, DEL-04-05): makes a rack document
// from `core/solver/performance_harness/rack/rack_model.py` openable by File > Open
// Model Document…, whose shape check (`src/features/model-file/modelDocumentShape.ts`)
// refuses the generator's output as it stands (MODEL-FILE-SHAPE: no `data_boundary`,
// no `label` on nodes, pipes, supports and components, no `kind`/`status` on load
// cases, no `terms` on subtraction combinations, no `diagnostics`).
//
//   node apps/desktop/native-smoke/perf-model-adapter.mjs IN.json OUT.json
//
// It adds only members that are absent, with the values the desktop itself writes or
// the bundled invented demo carries; nothing present is changed. It prints what it added.
// The injected result must then be written by `release_timing --write` from OUT.json,
// the document the app opens. To be retired once the generator emits the desktop shape.
import { readFileSync, writeFileSync } from "node:fs";

const DATA_BOUNDARY = {
  public_examples_policy: "invented_or_cleared_data_only",
  protected_source_policy: "no_bundled_protected_owner_or_standards_data",
  private_data_policy: "no_private_project_data",
  professional_boundary: "technical_preview_requires_human_engineering_review"
};

export function adaptRack(document) {
  const added = {};
  const note = (what) => { added[what] = (added[what] ?? 0) + 1; };
  if (document.data_boundary === undefined) { document.data_boundary = { ...DATA_BOUNDARY }; note("data_boundary"); }
  if (document.diagnostics === undefined) { document.diagnostics = []; note("diagnostics"); }
  for (const member of ["nodes", "pipe_segments", "supports", "components"]) {
    for (const entry of document[member] ?? []) if (entry.label === undefined) { entry.label = entry.id; note(`${member}[].label`); }
  }
  for (const entry of document.load_cases ?? []) {
    if (entry.kind === undefined) { entry.kind = "primitive_user_load"; note("load_cases[].kind"); }
    if (entry.status === undefined) { entry.status = "preview_only"; note("load_cases[].status"); }
  }
  // The desktop's own authoring writes a subtraction combination with empty terms.
  for (const entry of document.combinations ?? []) {
    if (entry.basis === "result_state_subtraction" && entry.terms === undefined) { entry.terms = []; note("combinations[].terms"); }
  }
  return added;
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].split("/").pop())) {
  const [input, output] = process.argv.slice(2);
  if (!input || !output) { console.error("usage: perf-model-adapter.mjs IN.json OUT.json"); process.exit(2); }
  const document = JSON.parse(readFileSync(input, "utf8"));
  const added = adaptRack(document);
  writeFileSync(output, JSON.stringify(document));
  console.error(`perf-model-adapter: ${output}: added ${JSON.stringify(added)}`);
}
