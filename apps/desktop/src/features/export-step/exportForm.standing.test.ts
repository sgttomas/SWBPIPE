import { describe, expect, it } from "vitest";
import demoModelFixture from "../../../../../fixtures/product_preview/invented_demo_model.json";
import type { PreviewModel } from "../../types";
import { buildPcfExportPacket, pcfPackageRecord } from "../pcf-export/PcfExportPanel";
import { buildReviewGeometryPacket, reviewGeometryPackageRecord } from "../review-geometry/ReviewGeometryPanel";
import { FIXED_ENTRIES } from "./exportForm";
import { commonTerms, fixedTerms } from "./openExportStep";

// DEL-07-12: each format's fixed entries are exactly the entries that packages built from two different models
// hold in common, so that `FIXED_ENTRIES` fails loudly when a generator changes. The models are invented: the
// demonstration model, and a second one with other units and no supports or components. (The stress-neutral and
// local FEA formats, which need a solved result, are checked in their panels' tests; the result export's table is
// empty.)
function demo(): PreviewModel {
  return structuredClone(demoModelFixture) as PreviewModel;
}

function other(): PreviewModel {
  const model = demo();
  model.project.id = "project:invented-second-model";
  model.project.units = { ...model.project.units, length: "mm" };
  for (const node of model.nodes) node.position = { x: node.position.x * 1000, y: node.position.y * 1000, z: node.position.z * 1000 };
  model.supports = [];
  model.components = [];
  return model;
}

describe("the fixed entries every package of a format carries, from two different models", () => {
  it("review geometry", () => {
    const [first, second] = [demo(), other()].map((model) => reviewGeometryPackageRecord(buildReviewGeometryPacket({ model, result: null, analysisRun: null })));
    expect(commonTerms(first, second)).toEqual(fixedTerms("review-geometry"));
  });

  it("PCF", () => {
    const [first, second] = [demo(), other()].map((model) => pcfPackageRecord(buildPcfExportPacket({ model, result: null, analysisRun: null })));
    expect(commonTerms(first, second)).toEqual(fixedTerms("pcf"));
  });

  it("the result export has none", () => {
    expect(FIXED_ENTRIES["result-export"]).toEqual([]);
  });
});
