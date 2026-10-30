import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import {
  BLANK_LOAD_STATE_MODEL_SCHEMA_VERSION, buildBlankLoadStateModelDocument, buildBlankLocalModelDocument, createLocalProject,
  openLocalProject, MODEL_SCHEMA_VERSION
} from "./projectService";
import { canonicalSha256Hex, computeModelHash } from "./hashService";
import { validateModelOperation } from "./operationService";
import { makeRichIntent } from "../features/rich-authoring/formSupport";
import { useWorkspaceSession } from "../features/workspace/workspaceSession";
import type { PreviewModel } from "../types";

const AT = new Date("2026-06-12T00:00:00Z");
// sha256 (RFC 8785) of buildBlankLocalModelDocument(AT), computed on the base
// tree 1b95d0b66 before this change and re-pinned by B7a, which made the blank
// a current 0.5.0 document, and by models 0.6.0 and 0.7.0 (its schema_version is the only
// change each time). Re-pinned by W-17 slice (a), which adds project.units.angle
// = rad and changes nothing else (projectUnitSelection.test.ts checks that the
// blank without angle still hashes to the previous pin). Model document 0.8.0 re-pinned
// it by its version token only (the blank at 0.7.0 gives the former pin).
const BASE_BLANK_SHA256 = "76093a1c58dea3823db425b6e8529b407f8a2d55a736e8c4fcc4af0a7f50cd99";
// T4-U8 (D-6/O3): the blank's contract carries the authoring default shear model.
const CONTRACT = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "timoshenko_annulus_energy_v1" };

// The load/reference-state blank is the one current version, 0.8.0, with the exact
// contract the group requires and an empty reference-configuration list (0.4.0 is retired).
describe("blank load/reference-state model (T1 WP3 item 2; current model 0.8.0)", () => {
  it("leaves the existing blank builder byte-identical", async () => {
    expect(await canonicalSha256Hex(buildBlankLocalModelDocument(AT))).toBe(BASE_BLANK_SHA256);
  });

  it("builds a 0.8.0 document with the current pressure contract (3.0.0/exact_pressure_v3) and no engineering content", async () => {
    const blank = buildBlankLocalModelDocument(AT);
    const model = buildBlankLoadStateModelDocument(AT);
    expect(model.schema_version).toBe(BLANK_LOAD_STATE_MODEL_SCHEMA_VERSION);
    expect(model.schema_version).toBe(MODEL_SCHEMA_VERSION);
    expect(model.pressure_contract).toEqual(CONTRACT);
    for (const key of ["materials", "sections", "nodes", "pipe_segments", "supports", "components", "load_cases", "combinations", "reference_configurations"] as const)
      expect(model[key]).toEqual([]);
    for (const key of ["rule_packs", "library_refs"]) expect(Object.hasOwn(model, key)).toBe(false);
    // Only the contract, the empty reference configurations and the project name differ from the blank path.
    const { pressure_contract, reference_configurations, project, ...rest } = model;
    const { project: blankProject, ...blankRest } = blank;
    expect([pressure_contract, reference_configurations]).toEqual([CONTRACT, []]);
    expect(rest).toEqual(blankRest);
    expect({ ...project, name: blankProject.name }).toEqual(blankProject);
    expect(project.name).toBe("Blank Local Load/reference Model 0.8.0");
    expect(Object.keys(model).slice(0, 2)).toEqual(["schema_version", "pressure_contract"]);
    // The existing builder is not changed by building the load/reference-state document.
    expect(await canonicalSha256Hex(buildBlankLocalModelDocument(AT))).toBe(BASE_BLANK_SHA256);
  });

  it("persists and reopens as a current browser document, byte-for-byte", async () => {
    const model = buildBlankLoadStateModelDocument(AT);
    const created = await createLocalProject(model);
    expect(JSON.stringify(created.model)).toBe(JSON.stringify(model));
    const opened = await openLocalProject();
    expect(JSON.stringify(opened?.model)).toBe(JSON.stringify(model));
    expect(opened?.model_document_version?.status).toBe("current");
  });

  it("accepts load/reference-state operations only on the exact-contract blank (engine refusal code for the pressure-free blank)", async () => {
    const intent = (model: PreviewModel, before: string) => makeRichIntent({ object_type: "Model", ref: model.project.id }, "set_field", "reference_configurations", before, [], "Reference configurations");
    const fresh = buildBlankLoadStateModelDocument(AT);
    const accepted = await validateModelOperation(fresh, intent(fresh, "[]"), await computeModelHash(fresh));
    expect(accepted.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
    const old = buildBlankLocalModelDocument(AT);
    const refused = await validateModelOperation(old, intent(old, "not_present"), await computeModelHash(old));
    expect(refused.diagnostics.map(d => d.code)).toContain("OP-LOAD-STATE-SCHEMA-VERSION-INVALID");
  });

  it("the session creates each blank through its own handler; New Blank is unchanged", async () => {
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    await act(async () => { await hook.result.current.project.handleCreateBlankLoadStateProject(); });
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Load/reference Model 0.8.0"));
    expect(hook.result.current.model.model?.schema_version).toBe(MODEL_SCHEMA_VERSION);
    expect(hook.result.current.model.model?.pressure_contract).toEqual(CONTRACT);
    expect(hook.result.current.project.projectMessage).toBe("Created blank local load/reference-state model 0.8.0 document without fixture entities, engineering values or external file copies.");
    expect(hook.result.current.project.projectOperation).toBe("create_blank");
    await act(async () => { await hook.result.current.project.handleCreateBlankProject(); });
    await waitFor(() => expect(hook.result.current.model.model?.project.name).toBe("Blank Local Model"));
    const created = hook.result.current.model.model!;
    expect(created.schema_version).toBe(MODEL_SCHEMA_VERSION);
    expect(Object.hasOwn(created, "pressure_contract")).toBe(false);
    expect(hook.result.current.project.projectMessage).toBe("Created blank local model document without fixture entities or external file copies.");
    hook.unmount();
  });
});
