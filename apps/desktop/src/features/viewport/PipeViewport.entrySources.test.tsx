// The user's text never crosses projects (PRD §5.5; DEL-07-03 AC-001): the viewport's
// New component form keeps the user's own sources within a project session and starts
// empty in the next one.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { modelIndexFor } from "../workspace/modelIndex";
import { PipeViewport } from "./PipeViewport";

afterEach(cleanup);

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = (): PreviewModel =>
  JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });

describe("viewport New component sources", () => {
  it("start empty, stay within a project session and start empty in the next", () => {
    const first = demo();
    const tree = (model: PreviewModel, sessionGeneration: number, revision: number) => (
      <DisplayUnitsProvider initialPreference="SI">
        <PipeViewport model={model} modelIndex={modelIndexFor(model, sessionGeneration, revision)} selection={{ type: "node", id: "node:N-110" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} />
      </DisplayUnitsProvider>
    );
    const view = render(tree(first, 1, 0));
    expect(byId("viewport-create-component-source")).toHaveValue("");
    expect(byId("viewport-create-component-provenance")).toHaveValue("");
    type("viewport-create-component-source", "geometry source");
    type("viewport-create-component-provenance", "component source");

    // An edit within the same project session keeps the user's text.
    const edited = structuredClone(first);
    view.rerender(tree(edited, 1, 1));
    expect(byId("viewport-create-component-provenance")).toHaveValue("component source");

    // Another project: nothing of the user's text carries.
    const next = demo();
    view.rerender(tree(next, 2, 0));
    expect(byId("viewport-create-component-source")).toHaveValue("");
    expect(byId("viewport-create-component-provenance")).toHaveValue("");
  });
});
