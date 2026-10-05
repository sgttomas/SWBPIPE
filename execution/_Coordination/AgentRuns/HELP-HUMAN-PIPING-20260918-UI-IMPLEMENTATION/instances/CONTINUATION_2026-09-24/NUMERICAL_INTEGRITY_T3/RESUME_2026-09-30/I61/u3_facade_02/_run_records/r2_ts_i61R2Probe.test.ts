// I61 U3 grant 1b, R-2 condition (disposable lane archive only, never committed):
// the desktop result admission over a base preview-physics-1 publication carrying
// one info RETAINED_PRECISION_UNAVAILABLE notice. The bytes come from PP's
// u3_r2_base_readers_accept_the_unavailable_notice (I61_R2_OUT).
import { expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { sourceContract, numericalResultStanding } from "./numericalResultQuality";
import { validatePreviewPhysicsEvidence } from "./previewPhysicsEvidence";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { MechanicsResult, PreviewModel } from "../../types";

const OUT = "WT/scratch/i61_u3_facade_02/out";
const load = (name: string) => JSON.parse(readFileSync(`${OUT}/${name}.json`, "utf8")) as MechanicsResult;
it("i61 R-2: desktop admission accepts the unavailable notice", async () => {
  const lines: string[] = [];
  for (const label of ["milestone", "exportable"]) {
  const model = (JSON.parse(readFileSync(`${OUT}/${label}_request.json`, "utf8")) as { model: PreviewModel }).model;
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const base = load(`${label}_base_${mode}`);
    expect(sourceContract(base)).toBe("preview_physics");
    validatePreviewPhysicsEvidence(base, model);
    const baseStanding = numericalResultStanding(base, model);
    for (const kind of ["plain", "receipt"]) {
      const noticed = load(`${label}_noticed_${kind}_${mode}`);
      const last = noticed.diagnostics[noticed.diagnostics.length - 1];
      expect(last.code).toBe("RETAINED_PRECISION_UNAVAILABLE");
      expect(noticed.diagnostics.length).toBe(base.diagnostics.length + 1);
      expect(sourceContract(noticed)).toBe("preview_physics");
      expect(() => validatePreviewPhysicsEvidence(noticed, model)).not.toThrow();
      const standing = numericalResultStanding(noticed, model);
      expect(standing).toEqual(baseStanding);
      const hash = await canonicalSha256HexCheckedV1(noticed);
      lines.push(`I61_R2_TS ${label} ${mode} ${kind} contract=${sourceContract(noticed)} reader=ok standing=${JSON.stringify(standing)} canonical_sha256=${hash}`);
    }
    // Negative control: a notice naming a missing result row is refused by the same reader.
    const dangling = load(`${label}_noticed_plain_${mode}`);
    dangling.diagnostics[dangling.diagnostics.length - 1].affected_refs = ["result:not-a-row"];
    expect(() => validatePreviewPhysicsEvidence(dangling, model)).toThrow(/dangling result reference/);
    lines.push(`I61_R2_TS ${label} ${mode} negative_control=refused`);
  }
  }
  writeFileSync(`${OUT}/ts_r2_results.txt`, lines.join("\n") + "\n");
}, 120000);
