// The bounded Grid's layout rules reach the blocks they size (run sheet S2). The stylesheet
// sizes the bounded grid's children with child-combinator rules
// (`.entity-grid.bounded > .direct-coordinate-workarea` and the rest). S2 wrapped the entity
// families in `.entity-grid-families` beside the Run sheet tab; a rule that no longer reaches
// its block leaves the review table with no height in a browser. jsdom does no layout, so this
// checks the selectors against the rendered tree instead.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import fixtureJson from "../../../../../fixtures/model_operations/run_sheet_authoring.json";
import type { PreviewModel } from "../../types";
import { ModelTree } from "./ModelTree";
// vitest serves CSS imports empty, so the stylesheet is read from disk (tests run from apps/desktop).
const stylesheet = readFileSync(resolve(process.cwd(), "src/styles.css"), "utf8");

/** Every rule that sizes a direct child of the bounded grid: each top-level selector of a rule
 * that names `.entity-grid.bounded` and ends `> .⟨class⟩`, with that class. */
function boundedChildRules(): Array<{ selector: string; target: string }> {
  const rules: Array<{ selector: string; target: string }> = [];
  const css = stylesheet.replace(/\/\*[\s\S]*?\*\//g, "");
  for (const match of css.matchAll(/([^{}]+)\{[^{}]*\}/g)) {
    for (const part of match[1].split(/,(?![^(]*\))/).map((item) => item.trim())) {
      const target = /\)?\s*>\s*\.([\w-]+)$/.exec(part)?.[1];
      if (target && part.includes(".entity-grid.bounded")) rules.push({ selector: part, target });
    }
  }
  return rules;
}

describe("the bounded Grid's layout rules reach their blocks", () => {
  it("each `.entity-grid.bounded > …` rule matches its block in the rendered Grid, with the families wrapper in place", () => {
    const model = structuredClone((fixtureJson as unknown as { basis_model: PreviewModel }).basis_model);
    const host = { store: null, queuedBatches: [], editorIntents: [], registerBatch: vi.fn() } as never;
    render(<ModelTree boundedGrid model={model} selection={{ type: "node", id: model.nodes[0].id }} onSelect={vi.fn()} runSheet={host} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const rules = boundedChildRules();
    const grid = screen.getByTestId("entity-grid");
    expect(grid.querySelector(":scope > .entity-grid-families")).not.toBeNull();
    // Each sized block is reached by a rule naming it.
    for (const target of ["direct-coordinate-workarea", "entity-grid-review", "entity-grid-review-toggle", "entity-grid-tabs"]) {
      const blocks = [...grid.querySelectorAll(`.${target}`)];
      expect(blocks.length, target).toBeGreaterThan(0);
      const reaching = rules.filter((rule) => rule.target === target && blocks.some((block) => block.matches(rule.selector)));
      expect(reaching.map((rule) => rule.selector), `a bounded rule reaches .${target}`).not.toEqual([]);
    }
    expect(rules.map((rule) => rule.target)).toContain("run-sheet");
  });
});
