// A1 (DEL-07-05 Design `results-navigator.md` §2 to §4): the navigation model. Invented rows only.
import { describe, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import {
  ALL_CASES,
  UNSTATED_CASE,
  firstRowOf,
  isNavigatorExcludedTarget,
  navigatorCases,
  navigatorEntities,
  navigatorKeyStep,
  rowCaseKey,
  stepIndex
} from "./resultsNavigator";

type Row = MechanicsResult["results"][number];
const row = (id: string, entity: string, basis?: [string, string], envelope = false): Row => ({
  id, kind: "invented", value: 1, unit: "N", entity_ref: entity,
  ...(basis ? { basis_ref: { ref_type: basis[0], ref_id: basis[1] } } : {}),
  ...(envelope ? { metadata: { component: "x", coordinate_system: "global", location: "node", basis: "explicit_user_range_envelope", sign_convention: "s" } } : {})
});

describe("stepIndex", () => {
  it("steps one place and stops at either end without wrapping", () => {
    expect(stepIndex(3, 0, 1)).toBe(1);
    expect(stepIndex(3, 1, -1)).toBe(0);
    expect(stepIndex(3, 2, 1)).toBeNull();
    expect(stepIndex(3, 0, -1)).toBeNull();
  });
  it("from no position goes to the first (Next) or the last (Previous)", () => {
    expect(stepIndex(3, -1, 1)).toBe(0);
    expect(stepIndex(3, -1, -1)).toBe(2);
  });
  it("cannot step an empty list", () => {
    expect(stepIndex(0, -1, 1)).toBeNull();
    expect(stepIndex(0, -1, -1)).toBeNull();
  });
});

describe("navigatorCases", () => {
  it("lists every case once: load cases, combinations, other bases, then rows that state none", () => {
    const rows = [
      row("a", "S1", ["combination", "C1"]),
      row("b", "S1"),
      row("c", "S1", ["load_case", "L2"]),
      row("d", "S1", ["other_basis", "X"]),
      row("e", "S2", ["load_case", "L1"]),
      row("f", "S2", ["load_case", "L2"]),
      row("g", "S2", ["combination", "ENV"], true)
    ];
    const cases = navigatorCases(rows);
    expect(cases.map((item) => item.key)).toEqual(["load_case:L2", "load_case:L1", "combination:C1", "combination:ENV", "other_basis:X", UNSTATED_CASE]);
    expect(cases.map((item) => item.label)).toEqual([
      "Load case L2", "Load case L1", "Combination C1", "Combination ENV (range envelope)", "other_basis X", "No case stated"
    ]);
    expect(cases.find((item) => item.key === "combination:ENV")?.envelope).toBe(true);
    expect(rowCaseKey(rows[1])).toBe(UNSTATED_CASE);
    expect(ALL_CASES).not.toBe(UNSTATED_CASE);
  });
  it("lists no case for no rows", () => {
    expect(navigatorCases([])).toEqual([]);
  });
});

describe("navigatorEntities and firstRowOf", () => {
  it("lists each entity once in first-appearance order and finds its first row", () => {
    const rows = [row("a", "P1", ["load_case", "L1"]), row("b", "S1", ["load_case", "L1"]), row("c", "P1", ["load_case", "L2"])];
    expect(navigatorEntities(rows)).toEqual(["P1", "S1"]);
    expect(firstRowOf(rows, "P1")?.id).toBe("a");
    expect(firstRowOf(rows, "N9")).toBeNull();
    expect(firstRowOf(rows, null)).toBeNull();
  });
});

describe("navigatorKeyStep", () => {
  const chord = (key: string, mods: Partial<{ altKey: boolean; shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }> = {}) =>
    navigatorKeyStep({ key, altKey: false, shiftKey: false, ctrlKey: false, metaKey: false, ...mods });
  it("maps the Alt arrow chords to the three axes", () => {
    expect(chord("ArrowRight", { altKey: true })).toEqual({ axis: "entity", delta: 1 });
    expect(chord("ArrowLeft", { altKey: true })).toEqual({ axis: "entity", delta: -1 });
    expect(chord("ArrowDown", { altKey: true })).toEqual({ axis: "case", delta: 1 });
    expect(chord("ArrowUp", { altKey: true })).toEqual({ axis: "case", delta: -1 });
    expect(chord("ArrowRight", { altKey: true, shiftKey: true })).toEqual({ axis: "kind", delta: 1 });
    expect(chord("ArrowLeft", { altKey: true, shiftKey: true })).toEqual({ axis: "kind", delta: -1 });
  });
  it("takes no other chord: plain arrows, Ctrl or Meta, Shift with Up or Down", () => {
    expect(chord("ArrowRight")).toBeNull();
    expect(chord("ArrowDown")).toBeNull();
    expect(chord("ArrowRight", { altKey: true, ctrlKey: true })).toBeNull();
    expect(chord("ArrowRight", { altKey: true, metaKey: true })).toBeNull();
    expect(chord("ArrowDown", { altKey: true, shiftKey: true })).toBeNull();
    expect(chord("z", { altKey: true })).toBeNull();
  });
});

describe("isNavigatorExcludedTarget", () => {
  it("leaves keys in editing and choosing controls to them", () => {
    const host = document.createElement("div");
    host.innerHTML = `<input id="i"/><textarea id="t"></textarea><select id="s"></select><div contenteditable="true"><span id="c">x</span></div><div role="combobox" id="r"></div><table><tbody><tr id="row" tabindex="0"><td>v</td></tr></tbody></table><button id="b">b</button>`;
    document.body.append(host);
    for (const id of ["i", "t", "s", "c", "r"]) expect(isNavigatorExcludedTarget(host.querySelector(`#${id}`))).toBe(true);
    for (const id of ["row", "b"]) expect(isNavigatorExcludedTarget(host.querySelector(`#${id}`))).toBe(false);
    expect(isNavigatorExcludedTarget(null)).toBe(false);
    host.remove();
  });
});
