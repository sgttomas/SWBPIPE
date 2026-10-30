import { describe, expect, it } from "vitest";
import { controlReportPackageRequest, REPORT_PACKAGE_SAVE_ROUTE_ID } from "./reportRedactionProjector";

function privateRequest() {
  return {
    package_id: "invented-local-package",
    local_private_intent: true,
    explicit_local_private_intent: true,
    model: {
      field_id: "model-name",
      field_class: "model_name",
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      review_status: "pending",
      value: "Invented private model"
    }
  };
}

describe("report-package redaction binding", () => {
  it("requires UI-owned explicit intent and strips payload-carried intent", () => {
    const source = privateRequest();
    const snapshot = structuredClone(source);

    const blocked = controlReportPackageRequest(source, false);
    const allowed = controlReportPackageRequest(source, true);

    expect(blocked.summary.route_id).toBe(REPORT_PACKAGE_SAVE_ROUTE_ID);
    expect(blocked.blocked).toBe(true);
    expect(blocked.payload).toBeNull();
    expect(allowed.blocked).toBe(false);
    expect(allowed.payload).toMatchObject({
      package_id: "invented-local-package",
      model: expect.objectContaining({ value: "Invented private model" })
    });
    expect(allowed.payload).not.toHaveProperty("local_private_intent");
    expect(allowed.payload).not.toHaveProperty("explicit_local_private_intent");
    expect(source).toEqual(snapshot);
  });
});
