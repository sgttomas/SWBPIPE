// Adoption preconditions of the desktop model document (DEL-15-01 W-6a review
// round 3; adapted from the independent reviewer's adopt-and-render harness).
//
// The shape check (`modelDocumentShape.ts`, ported natively for the save) states
// the members the desktop dereferences. This test keeps that statement honest:
// it mutates the bundled demo, opens each mutant through File > Open Model
// Document… in the real App, and requires one of two things of every case:
// open's admission refuses it (the shape check or the W-13 document validator,
// never the model-index last guard), or the session adopts it and renders the
// workspace without throwing. Cases: every string member of the demo's first
// entity of each collection (and of every top-level object) retyped as a number,
// an object, an array and an object whose `toString` is null, plus the
// reviewer's 133 member mutations.
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";

type Path = Array<string | number>;
type Case = { id: string; document: unknown };

const RETYPED: ReadonlyArray<unknown> = [17, {}, [], { toString: null }];

function mutated(path: Path, value: unknown): Case {
  const document = structuredClone(demoModel) as Record<string | number, unknown>;
  let target = document;
  for (const step of path.slice(0, -1)) target = target[step] as Record<string | number, unknown>;
  target[path[path.length - 1]] = structuredClone(value);
  return { id: `${path.join(".")}=${JSON.stringify(value)}`, document };
}

/** Every string leaf, following only the first entry of each array. */
function stringLeaves(value: unknown, path: Path, out: Path[]) {
  if (typeof value === "string") out.push(path);
  else if (Array.isArray(value)) { if (value.length > 0) stringLeaves(value[0], [...path, 0], out); }
  else if (value && typeof value === "object") for (const [key, child] of Object.entries(value)) stringLeaves(child, [...path, key], out);
}

/** The reviewer's member mutations (W-6a review round 3, `prepare-cases.py`). */
const REVIEWER_MEMBERS: ReadonlyArray<[Path, ReadonlyArray<unknown>]> = [
  ...["length", "force", "angle", "temperature", "stress"].map((key): [Path, unknown[]] => [["project", "units", key], [17, {}, [], null, { toString: null }]]),
  ...["completeness", "geometry", "modifiers", "mechanics_interface", "tags", "notes", "provenance_record"].map((key): [Path, unknown[]] => [["components", 0, key], [17, {}, [{}], null]]),
  ...["distributed_loads", "forces", "nodal_loads", "gravity", "temperature", "pressure", "line_loads", "displacements"].map((key): [Path, unknown[]] => [["load_cases", 0, key], [17, {}, [{}], null]]),
  ...["shear_modulus", "thermal_expansion_coefficient", "temperature_points"].map((key): [Path, unknown[]] => [["materials", 0, key], [17, {}, [{}], null]]),
  ...["id", "target_ref", "remediation", "path"].map((key): [Path, unknown[]] => [["diagnostics", 0, key], [17, {}, { toString: null }, [{}], null]]),
  ...["stiffness", "gap", "friction", "direction"].map((key): [Path, unknown[]] => [["supports", 0, key], [17, {}, [{}], null]])
];

export function adoptionCases(): Case[] {
  const leaves: Path[] = [];
  stringLeaves(demoModel, [], leaves);
  const cases = new Map<string, Case>();
  for (const path of leaves) for (const value of RETYPED) { const item = mutated(path, value); cases.set(item.id, item); }
  for (const [path, values] of REVIEWER_MEMBERS) for (const value of values) { const item = mutated(path, value); cases.set(item.id, item); }
  return [...cases.values()];
}

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO_ROW = `tree-row-project-${encodeURIComponent(demoModel.project.id)}`;

it("refuses at open's admission, or adopts and renders, every retyped member of the demo", async () => {
  const cases = adoptionCases();
  expect(cases.length).toBeGreaterThanOrEqual(133 + 4 * 60);
  let answer: unknown = null;
  const start = async () => {
    cleanup();
    invokeMock.mockReset();
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await screen.findByTestId(DEMO_ROW);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) => command === "open_model_document_file" ? Promise.resolve(answer) : Promise.resolve(null));
  };
  await start();
  const failures: Array<{ id: string; reason: string }> = [];
  const refusedBy = new Map<string, number>();
  let adopted = 0;
  for (const [index, item] of cases.entries()) {
    const name = `case-${index}.json`;
    answer = { outcome: "opened", file_name: name, byte_count: 1, schema_status: "current", document: item.document };
    const errors: unknown[] = [];
    const spy = vi.spyOn(console, "error").mockImplementation((...args: unknown[]) => { errors.push(args[0]); });
    try {
      act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: "file.open-model" })); });
      const shown = await waitFor(() => {
        const text = screen.getByTestId("local-project-message").textContent ?? "";
        if (!text.includes(`(${name})`) && !text.includes(` ${name} `)) throw new Error("pending");
        return text;
      });
      if (shown.startsWith("Open model document refused")) {
        if (shown.includes("could not index")) failures.push({ id: item.id, reason: `refused only by the model-index last guard: ${shown}` });
        const code = /refused \([^)]*\): ([A-Z0-9_-]+)/.exec(shown)?.[1] ?? "unknown";
        refusedBy.set(code, (refusedBy.get(code) ?? 0) + 1);
      } else {
        await waitFor(() => expect(screen.getByTestId(DEMO_ROW)).toBeInTheDocument());
        // Let the adoption's effects and deferred renders run before judging it.
        for (let flush = 0; flush < 3; flush += 1) await act(async () => { await new Promise((resolve) => setTimeout(resolve, 0)); });
        adopted += 1;
      }
      // jsdom's canvas notice is not a crash; a thrown render or React's report of one is.
      const crash = errors.find((entry) => !String(entry).includes("getContext")
        && (entry instanceof Error || /Uncaught|The above error occurred|error occurred in/.test(String(entry))));
      if (!screen.queryByTestId("desktop-preview-shell") || crash) {
        failures.push({ id: item.id, reason: crash ? String(crash).slice(0, 200) : "the workspace unmounted" });
        spy.mockRestore();
        await start();
        continue;
      }
    } catch (error) {
      failures.push({ id: item.id, reason: String(error).slice(0, 200) });
      spy.mockRestore();
      await start();
      continue;
    }
    spy.mockRestore();
  }
  if (failures.length > 0) console.info(failures.map((item) => `${item.id} :: ${item.reason.split("\n")[0]}`).join("\n"));
  process.stdout.write(`adoption cases ${cases.length}: adopted ${adopted}; refused ${JSON.stringify(Object.fromEntries(refusedBy))}\n`);
  expect(failures).toEqual([]);
  expect(adopted).toBeGreaterThan(0);
}, 900_000);
