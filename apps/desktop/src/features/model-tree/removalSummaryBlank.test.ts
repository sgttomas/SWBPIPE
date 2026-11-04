// The desktop's deletion texts follow the applier's (`node_delete_display`,
// `pipe_delete_display`): an absent or blank member gives the empty text, which
// the applier gives too, rather than a crash. Invented records.
import { expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { nodeDeletionSummary, pipeDeletionSummary, supportDeletionSummary } from "./removalIntents";

type Pipe = PreviewModel["pipe_segments"][number];
type Node = PreviewModel["nodes"][number];
type Support = PreviewModel["supports"][number];

it("gives the empty text for an unlabelled pipe, node or support, as the applier does", () => {
  const pipe = { id: "P1", from: "N1", to: "N2", material: "material:steel" } as unknown as Pipe;
  expect(pipeDeletionSummary(pipe)).toBe("");
  expect(pipeDeletionSummary({ ...pipe, label: "  " })).toBe("");
  expect(pipeDeletionSummary({ ...pipe, label: " Run " })).toBe("Run; N1->N2; material=material:steel");
  expect(nodeDeletionSummary({ id: "N1", position: { x: 0, y: 1, z: 2 } } as unknown as Node)).toBe("");
  expect(supportDeletionSummary({ id: "S1" } as unknown as Support)).toBe("");
});
