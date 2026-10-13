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
// a current 0.5.0 document (its schema_version is the only change).
const BASE_BLANK_SHA256 = "453cac5786dabfff1e2db0904f4d49b518d16fc3b4eedc6ddcd53295ee106d84";

describe("blank model 0.4.0 (T1 WP3 item 2)", () => {
  it("leaves the existing blank builder byte-identical", async () => {
    expect(await canonicalSha256Hex(buildBlankLocalModelDocument(AT))).toBe(BASE_BLANK_SHA256);
  });

  it("builds a 0.4.0 document with the current pressure contract (3.0.0/exact_pressure_v3) and no engineering content", async () => {
    const blank = buildBlankLocalModelDocument(AT);
    const model = buildBlankLoadStateModelDocument(AT);
    expect(model.schema_version).toBe(BLANK_LOAD_STATE_MODEL_SCHEMA_VERSION);
    expect(model.schema_version).toBe("0.4.0");
    expect(model.pressure_contract).toEqual({ version: "3.0.0", mode: "exact_pressure_v3" });
    for (const key of ["materials", "sections", "nodes", "pipe_segments", "supports", "components", "load_cases", "combinations"] as const)
      expect(model[key]).toEqual([]);
    for (const key of ["reference_configurations", "rule_packs", "library_refs"]) expect(Object.hasOwn(model, key)).toBe(false);
    // Only the version, the contract and the project name differ from the blank path.
    const { schema_version, pressure_contract, project, ...rest } = model;
    const { schema_version: blankVersion, project: blankProject, ...blankRest } = blank;
    expect([schema_version, pressure_contract, blankVersion]).toEqual(["0.4.0", { version: "3.0.0", mode: "exact_pressure_v3" }, MODEL_SCHEMA_VERSION]);
    expect(rest).toEqual(blankRest);
    expect({ ...project, name: blankProject.name }).toEqual(blankProject);
    expect(project.name).toBe("Blank Local Model 0.4.0");
    expect(Object.keys(model).slice(0, 2)).toEqual(["schema_version", "pressure_contract"]);
    // The existing builder is not changed by building the 0.4.0 document.
    expect(await canonicalSha256Hex(buildBlankLocalModelDocument(AT))).toBe(BASE_BLANK_SHA256);
  });

  it("persists and reopens as a current 0.4.0 browser document, byte-for-byte", async () => {
    const model = buildBlankLoadStateModelDocument(AT);
    const created = await createLocalProject(model);
    expect(JSON.stringify(created.model)).toBe(JSON.stringify(model));
    const opened = await openLocalProject();
    expect(JSON.stringify(opened?.model)).toBe(JSON.stringify(model));
    expect(opened?.model_document_version?.status).toBe("current");
  });

  it("accepts load/reference-state operations only on the 0.4.0 blank (engine refusal code for the old version)", async () => {
    const intent = (model: PreviewModel) => makeRichIntent({ object_type: "Model", ref: model.project.id }, "set_field", "reference_configurations", "not_present", [], "Reference configurations");
    const fresh = buildBlankLoadStateModelDocument(AT);
    const accepted = await validateModelOperation(fresh, intent(fresh), await computeModelHash(fresh));
    expect(accepted.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
    const old = buildBlankLocalModelDocument(AT);
    const refused = await validateModelOperation(old, intent(old), await computeModelHash(old));
    expect(refused.diagnostics.map(d => d.code)).toContain("OP-LOAD-STATE-SCHEMA-VERSION-INVALID");
  });

  it("the session creates each blank through its own handler; New Blank is unchanged", async () => {
    const hook = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
    await act(async () => { await hook.result.current.project.handleCreateBlankLoadStateProject(); });
    await waitFor(() => expect(hook.result.current.model.model?.schema_version).toBe("0.4.0"));
    expect(hook.result.current.model.model?.pressure_contract).toEqual({ version: "3.0.0", mode: "exact_pressure_v3" });
    expect(hook.result.current.project.projectMessage).toBe("Created blank local model 0.4.0 document without fixture entities, engineering values or external file copies.");
    expect(hook.result.current.project.projectOperation).toBe("create_blank");
    await act(async () => { await hook.result.current.project.handleCreateBlankProject(); });
    await waitFor(() => expect(hook.result.current.model.model?.schema_version).toBe(MODEL_SCHEMA_VERSION));
    const created = hook.result.current.model.model!;
    expect(Object.hasOwn(created, "pressure_contract")).toBe(false);
    expect(created.project.name).toBe("Blank Local Model");
    expect(hook.result.current.project.projectMessage).toBe("Created blank local model document without fixture entities or external file copies.");
    hook.unmount();
  });
});
