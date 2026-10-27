// TP-E4-REDACTION-001: the app-side TypeScript mirror of the DEL-12-02 core
// redaction contract must match the python contract decision-for-decision.
// Parity is pinned by the shared invented corpus
// `fixtures/redaction_export_controls/cases.json`, which
// `tests/security/test_redaction_export_controls.py` asserts against
// `core/security/redaction/controls.py`. All fixture data is invented.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  EXPORT_CONTEXTS,
  GOVERNED_LOCAL_FIRST_ROUTE_IDS,
  REDACTED_VALUE,
  REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
  USER_RELEASED_REPORT_CONTEXT,
  classifyExportItem,
  controlRouteExport,
  controlUserReleasedReportPackageRequest,
  enforceLocalFirstRoute,
  redactExportPayload,
  type ControlExportContext,
  type RedactionAction,
  type RedactionExportContext,
  type RedactionFindingClass,
  type RedactionFindingSeverity,
  type RedactionReasonCode
} from "./redactionExportControls";

type ParityCase = {
  case_id: string;
  item: Record<string, unknown>;
  export_context: ControlExportContext;
  explicit_local_private_intent: boolean;
  explicit_user_release_intent?: boolean;
  expected: {
    action: RedactionAction;
    reason_code: RedactionReasonCode;
    source_metadata_present: boolean;
    finding: { class: RedactionFindingClass; severity: RedactionFindingSeverity } | null;
  };
};

const corpusPath = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../../fixtures/redaction_export_controls/cases.json"
);
const corpus = JSON.parse(readFileSync(corpusPath, "utf8")) as {
  deliverable_id: string;
  cases: ParityCase[];
};

/** The new context, passed past the walk's type the way a caller outside the type system could. */
const WALK_RELEASED_CONTEXT = USER_RELEASED_REPORT_CONTEXT as unknown as RedactionExportContext;
const WALK_REFUSAL = /^USER_RELEASE_ROUTE_REQUIRED: the redaction walk does not admit the user_released_report context/;

/** The two new-context corpus items that state no privacy at all. The item rule decides them by
 * step 13 (`USER_RELEASE_VALUE_UNBOUND`); on the user-release projector an object that states no
 * privacy passes its leaves the default label, `private_project_data` / `private_only` (§3.2), so
 * with the wrapper's intent they carry `USER_RELEASED_PROJECT_DATA`. */
const PROJECTOR_DEFAULT_LABEL_CASES = new Set(["user-released-missing-metadata-blocked", "user-released-storage-metadata-only-unbound"]);

/** A new-context corpus case on the user-release projector, as `{ item }` with an empty model:
 * its first finding is the case's (the object's own steps 1-9 decision, else each leaf's under
 * the item's label), every finding lies at or beneath the item, and the lossless route admits
 * the request only for an include or a warning with the wrapper's intent. */
function expectUserReleaseCorpusFindingOnProjector(parityCase: ParityCase): void {
  const intent = parityCase.explicit_user_release_intent === true;
  expect(() =>
    redactExportPayload(
      { item: parityCase.item },
      { exportContext: WALK_RELEASED_CONTEXT, explicitUserReleaseIntent: intent }
    )
  ).toThrowError(WALK_REFUSAL);
  const projected = controlUserReleasedReportPackageRequest({ item: parityCase.item }, {}, intent);
  const defaultLabel = PROJECTOR_DEFAULT_LABEL_CASES.has(parityCase.case_id);
  const expected: Pick<ParityCase["expected"], "action" | "reason_code" | "finding"> = defaultLabel
    ? {
        action: "warning_only",
        reason_code: "USER_RELEASED_PROJECT_DATA",
        finding: { class: "PRIVATE_DATA_WARNING", severity: "WARNING" }
      }
    : parityCase.expected;
  if (defaultLabel) {
    expect(intent, parityCase.case_id).toBe(true);
    expect(projected.findings.every((finding) => finding.code === "USER_RELEASED_PROJECT_DATA"), parityCase.case_id).toBe(true);
  }
  expect(projected.findings.every((finding) => finding.path === "$.request.item" || finding.path.startsWith("$.request.item.")), parityCase.case_id).toBe(true);
  if (expected.finding === null) {
    expect(projected.findings, parityCase.case_id).toHaveLength(0);
  } else {
    expect(projected.findings.length, parityCase.case_id).toBeGreaterThan(0);
    expect(projected.findings[0].code, parityCase.case_id).toBe(expected.reason_code);
    expect(projected.findings[0].class, parityCase.case_id).toBe(expected.finding.class);
    expect(projected.findings[0].severity, parityCase.case_id).toBe(expected.finding.severity);
  }
  const admitted = (expected.action === "include" || expected.action === "warning_only") && intent;
  expect(projected.blocked, parityCase.case_id).toBe(!admitted);
  expect(projected.payload, parityCase.case_id).toEqual(admitted ? { item: parityCase.item } : null);
}

describe("redactionExportControls parity with the core python contract", () => {
  it("loads the shared DEL-12-02 corpus", () => {
    expect(corpus.deliverable_id).toBe("DEL-12-02");
    expect(corpus.cases.length).toBeGreaterThan(0);
  });

  for (const parityCase of corpus.cases) {
    it(`matches the core decision for ${parityCase.case_id}`, () => {
      const originalItem = structuredClone(parityCase.item);
      const decision = classifyExportItem(parityCase.item, {
        exportContext: parityCase.export_context,
        explicitLocalPrivateIntent: parityCase.explicit_local_private_intent,
        explicitUserReleaseIntent: parityCase.explicit_user_release_intent
      });
      expect(parityCase.item).toEqual(originalItem);
      expect(decision.action).toBe(parityCase.expected.action);
      expect(decision.reason_code).toBe(parityCase.expected.reason_code);
      expect(decision.source_metadata_present).toBe(parityCase.expected.source_metadata_present);

      if (parityCase.export_context === USER_RELEASED_REPORT_CONTEXT) {
        // The redaction walk refuses the context (§3.2); the case's finding is checked on the
        // user-release projector, the only payload entry for it.
        expectUserReleaseCorpusFindingOnProjector(parityCase);
        return;
      }
      const run = redactExportPayload(
        { item: parityCase.item },
        {
          exportContext: parityCase.export_context,
          explicitLocalPrivateIntent: parityCase.explicit_local_private_intent,
          explicitUserReleaseIntent: parityCase.explicit_user_release_intent
        }
      );
      if (parityCase.expected.finding === null) {
        expect(run.findings).toHaveLength(0);
      } else {
        expect(run.findings).toHaveLength(1);
        expect(run.findings[0].code).toBe(parityCase.expected.reason_code);
        expect(run.findings[0].class).toBe(parityCase.expected.finding.class);
        expect(run.findings[0].severity).toBe(parityCase.expected.finding.severity);
      }
    });
  }
});

describe("redactExportPayload walk behavior", () => {
  const inventedPayload = () => ({
    report_id: "invented-redaction-report",
    project: {
      name: {
        field_id: "project.name",
        field_class: "project",
        privacy_classification: "private_project_data",
        redistribution_status: "private_only",
        review_status: "accepted",
        value: "Invented Local Project A"
      },
      status: {
        field_id: "project.status",
        field_class: "public_metadata",
        privacy_classification: "public_metadata",
        redistribution_status: "public_permissive",
        review_status: "accepted",
        value: "MECHANICS_SOLVED"
      }
    },
    materials: [
      {
        field_id: "material.unknown_source",
        field_class: "material",
        privacy_classification: "unknown",
        redistribution_status: "unknown",
        review_status: "pending",
        value: "Invented material placeholder"
      }
    ]
  });

  it("redacts private and unknown values for public contexts without mutating the source", () => {
    const source = inventedPayload();
    const original = structuredClone(source);
    const run = redactExportPayload(source, { exportContext: "public_report" });
    const payload = run.payload as ReturnType<typeof inventedPayload>;

    expect(source).toEqual(original);
    expect(run.blocked).toBe(false);
    expect(payload.project.name.value).toBe(REDACTED_VALUE);
    expect(payload.project.status.value).toBe("MECHANICS_SOLVED");
    expect(payload.materials[0].value).toBe(REDACTED_VALUE);
    expect(run.decisions.map((decision) => decision.reason_code)).toContain("PRIVATE_DATA_REDACTED");
    expect(run.summary.redacted_count).toBe(2);
    expect(run.summary.cloud_transmission_attempted).toBe(false);
  });

  it("blocks a local private export until explicit intent is recorded, then retains with a warning", () => {
    const blocked = redactExportPayload(inventedPayload(), {
      exportContext: "local_private",
      explicitLocalPrivateIntent: false
    });
    expect(blocked.blocked).toBe(true);
    expect(blocked.findings.map((finding) => finding.code)).toContain("LOCAL_PRIVATE_INTENT_REQUIRED");

    const allowed = redactExportPayload(inventedPayload(), {
      exportContext: "local_private",
      explicitLocalPrivateIntent: true
    });
    const payload = allowed.payload as ReturnType<typeof inventedPayload>;
    expect(allowed.blocked).toBe(false);
    expect(payload.project.name.value).toBe("Invented Local Project A");
    expect(allowed.findings.map((finding) => finding.code)).toContain("PRIVATE_LOCAL_ALLOWED_WITH_WARNING");
    expect(allowed.summary.warning_count).toBeGreaterThanOrEqual(1);
  });

  it("strips unsafe detail keys when redacting storage-boundary records", () => {
    const run = redactExportPayload(
      {
        concrete_path: {
          field_id: "storage.path",
          field_class: "path",
          privacy_classification: "path_data",
          redistribution_status: "private_only",
          review_status: "accepted",
          file_path: "SYNTHETIC_PATH_SHOULD_NOT_SURVIVE",
          value: "SYNTHETIC_PATH_VALUE_SHOULD_NOT_SURVIVE"
        }
      },
      { exportContext: "public_report" }
    );
    const serialized = JSON.stringify(run.payload);
    expect(serialized).not.toContain("SYNTHETIC_PATH_SHOULD_NOT_SURVIVE");
    expect(serialized).not.toContain("SYNTHETIC_PATH_VALUE_SHOULD_NOT_SURVIVE");
    expect(run.findings.map((finding) => finding.code)).toContain("CONCRETE_PATH_REDACTED");
  });

  it("rejects an unsupported export context loudly", () => {
    expect(() =>
      redactExportPayload({}, { exportContext: "cloud_sync" as RedactionExportContext })
    ).toThrowError(/unsupported export_context/);
  });
});

describe("route projection", () => {
  it("does not promote an opaque sibling from public envelope metadata", () => {
    const payload = {
      privacy: {
        privacy_classification: "public_metadata",
        redistribution_status: "public_permissive"
      },
      provenance: {
        privacy_classification: "invented_public_example",
        redistribution_status: "invented_non_engineering_example"
      },
      opaque_sibling: "Invented value without leaf metadata"
    };

    for (const exportContext of ["public_report", "shared_model", "downstream_tool"] as const) {
      const controlled = controlRouteExport(payload, {
        routeId: "DOTH-HANDOFF-002",
        exportContext
      });
      const decision = controlled.decisions.find((item) => item.path.endsWith("opaque_sibling"));

      expect(decision).toMatchObject({
        privacy_classification: "unknown",
        redistribution_status: "unknown",
        review_status: "pending",
        action: "redact_value",
        reason_code: "REDISTRIBUTION_STATUS_UNKNOWN"
      });
      expect((controlled.payload as { opaque_sibling: string }).opaque_sibling).toBe(REDACTED_VALUE);
    }

    const blockedLocal = controlRouteExport(payload, {
      routeId: "DOTH-HANDOFF-002",
      exportContext: "local_private"
    });
    expect(blockedLocal.blocked).toBe(true);
    expect(blockedLocal.payload).toBeNull();
    expect(blockedLocal.summary.local_first?.reason_code).toBe("LOCAL_PRIVATE_INTENT_REQUIRED");

    const local = controlRouteExport(payload, {
      routeId: "DOTH-HANDOFF-002",
      exportContext: "local_private",
      explicitLocalPrivateIntent: true
    });
    const localDecision = local.decisions.find((item) => item.path.endsWith("opaque_sibling"));
    expect(localDecision).toMatchObject({
      privacy_classification: "unknown",
      redistribution_status: "unknown",
      review_status: "pending",
      action: "warning_only",
      reason_code: "REDISTRIBUTION_STATUS_UNKNOWN"
    });
    expect((local.payload as { opaque_sibling: string }).opaque_sibling).toBe(payload.opaque_sibling);
  });

  it("keeps public basis exact-record-local across nested records and collections", () => {
    const payload = {
      public_record: {
        privacy_classification: "public_metadata",
        redistribution_status: "public_permissive",
        direct_leaf: "Direct record metadata",
        nested_record: { opaque_leaf: "Nested record without its own metadata" },
        nested_list: [{ opaque_leaf: "Nested collection record without metadata" }]
      }
    };

    for (const exportContext of ["public_report", "shared_model", "downstream_tool"] as const) {
      const controlled = controlRouteExport(payload, {
        routeId: "DOTH-HANDOFF-002",
        exportContext
      });
      const direct = controlled.decisions.find((item) => item.path.endsWith("direct_leaf"));
      const nested = controlled.decisions.filter((item) => item.path.endsWith("opaque_leaf"));

      expect(direct).toMatchObject({
        privacy_classification: "public_metadata",
        redistribution_status: "public_permissive",
        review_status: "accepted",
        action: "include"
      });
      expect(nested).toHaveLength(2);
      for (const decision of nested) {
        expect(decision).toMatchObject({
          privacy_classification: "unknown",
          redistribution_status: "unknown",
          review_status: "pending",
          action: "redact_value",
          reason_code: "REDISTRIBUTION_STATUS_UNKNOWN"
        });
      }
      expect(controlled.payload).toMatchObject({
        public_record: {
          nested_record: { opaque_leaf: REDACTED_VALUE },
          nested_list: [{ opaque_leaf: REDACTED_VALUE }]
        }
      });
    }

    const local = controlRouteExport(payload, {
      routeId: "DOTH-HANDOFF-002",
      exportContext: "local_private",
      explicitLocalPrivateIntent: true
    });
    const nestedLocal = local.decisions.filter((item) => item.path.endsWith("opaque_leaf"));
    expect(nestedLocal).toHaveLength(2);
    for (const decision of nestedLocal) {
      expect(decision).toMatchObject({
        privacy_classification: "unknown",
        redistribution_status: "unknown",
        review_status: "pending",
        action: "warning_only",
        reason_code: "REDISTRIBUTION_STATUS_UNKNOWN"
      });
    }
    expect(local.payload).toMatchObject({
      public_record: {
        nested_record: { opaque_leaf: payload.public_record.nested_record.opaque_leaf },
        nested_list: [{ opaque_leaf: payload.public_record.nested_list[0].opaque_leaf }]
      }
    });
  });

  it("emits metadata-only local-first evidence and fails closed for an unknown route", () => {
    const governed = controlRouteExport(
      { invented: "opaque" },
      { routeId: "DOTH-HANDOFF-002", exportContext: "downstream_tool" }
    );
    const unknown = controlRouteExport(
      { invented: "opaque" },
      { routeId: "INVENTED-UNKNOWN-ROUTE", exportContext: "downstream_tool" }
    );

    expect(governed.summary.local_first).toEqual({
      route_id: "DOTH-HANDOFF-002",
      export_context: "downstream_tool",
      storage_context: "downstream_tool",
      action: "include_metadata_only",
      reason_code: "SAFE_PUBLIC_METADATA",
      blocked: false,
      metadata_only: true,
      explicit_local_private_intent: false
    });
    expect(unknown.blocked).toBe(true);
    expect(unknown.payload).toBeNull();
    expect(unknown.summary.local_first).toMatchObject({
      reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN",
      blocked: true,
      metadata_only: true
    });
  });

  // DEL-15-01 W-6a: File > Save Model Document… (twin of the python
  // `test_model_document_save_route_is_governed_local_private_with_explicit_intent`).
  it("governs DOTH-MODEL-DOC-007 as a local-private route that needs explicit intent", () => {
    expect(enforceLocalFirstRoute({ routeId: "DOTH-MODEL-DOC-007", exportContext: "local_private", explicitLocalPrivateIntent: true })).toEqual({
      route_id: "DOTH-MODEL-DOC-007",
      export_context: "local_private",
      storage_context: "local_private",
      action: "include_metadata_only",
      reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED",
      blocked: false,
      metadata_only: true,
      explicit_local_private_intent: true
    });
    expect(enforceLocalFirstRoute({ routeId: "DOTH-MODEL-DOC-007", exportContext: "local_private" })).toMatchObject({
      blocked: true,
      reason_code: "LOCAL_PRIVATE_INTENT_REQUIRED"
    });
    expect(enforceLocalFirstRoute({ routeId: "DOTH-MODEL-DOC-008", exportContext: "local_private", explicitLocalPrivateIntent: true })).toMatchObject({
      blocked: true,
      reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN"
    });
    const controlled = controlRouteExport(
      {
        field_id: "invented.model.private",
        privacy_classification: "private_project_data",
        redistribution_status: "private_only",
        review_status: "accepted",
        value: "Invented private model value"
      },
      { routeId: "DOTH-MODEL-DOC-007", exportContext: "local_private", explicitLocalPrivateIntent: true, requireLosslessMaterialization: true }
    );
    expect(controlled.blocked).toBe(false);
    expect(controlled.payload).toMatchObject({ value: "Invented private model value" });
    expect(controlled.summary.local_first).toMatchObject({ route_id: "DOTH-MODEL-DOC-007", blocked: false });
  });

  // DEL-15-01 W-6b: File > Save Handoff… (twin of the python
  // `test_handoff_package_save_route_is_governed_local_private_with_explicit_intent`).
  it("governs DOTH-HANDOFF-PACKAGE-008 as a local-private route that needs explicit intent", () => {
    expect(enforceLocalFirstRoute({ routeId: "DOTH-HANDOFF-PACKAGE-008", exportContext: "local_private", explicitLocalPrivateIntent: true })).toEqual({
      route_id: "DOTH-HANDOFF-PACKAGE-008",
      export_context: "local_private",
      storage_context: "local_private",
      action: "include_metadata_only",
      reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED",
      blocked: false,
      metadata_only: true,
      explicit_local_private_intent: true
    });
    expect(enforceLocalFirstRoute({ routeId: "DOTH-HANDOFF-PACKAGE-008", exportContext: "local_private" })).toMatchObject({
      blocked: true,
      reason_code: "LOCAL_PRIVATE_INTENT_REQUIRED"
    });
    expect(enforceLocalFirstRoute({ routeId: "DOTH-HANDOFF-PACKAGE-009", exportContext: "local_private", explicitLocalPrivateIntent: true })).toMatchObject({
      blocked: true,
      reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN"
    });
    const controlled = controlRouteExport(
      {
        field_id: "invented.handoff.private",
        privacy_classification: "private_project_data",
        redistribution_status: "private_only",
        review_status: "accepted",
        value: "Invented private handoff value"
      },
      { routeId: "DOTH-HANDOFF-PACKAGE-008", exportContext: "local_private", explicitLocalPrivateIntent: true, requireLosslessMaterialization: true }
    );
    expect(controlled.blocked).toBe(false);
    expect(controlled.payload).toMatchObject({ value: "Invented private handoff value" });
    expect(controlled.summary.local_first).toMatchObject({ route_id: "DOTH-HANDOFF-PACKAGE-008", blocked: false });
  });

  it.each([
    {
      documentKind: "openpipestress.technical_preview.conservative_pcf_export_package",
      deliverableId: "DEL-17-07",
      profileId: "ops.pcf.conservative_subset"
    },
    {
      documentKind: "openpipestress.technical_preview.caepipe_mbf_export_package",
      deliverableId: "DEL-17-04",
      profileId: "ops.caepipe_mbf.smoke_tbd"
    }
  ])("uses exact $deliverableId structural paths without descendant promotion", ({
    documentKind,
    deliverableId,
    profileId
  }) => {
    const payload = {
      document_kind: documentKind,
      deliverable_id: deliverableId,
      scope_items: ["SOW-030"],
      export_profile: { profile_id: profileId },
      conversion_witnesses: [
        {
          witness_id: "conversion:witness:1",
          target_quantity: { target_field: "target.expected" },
          conversion_factor_to_target: 1000
        }
      ],
      opaque_descendants: {
        scope_items: ["opaque scope"],
        export_profile: { profile_id: "opaque profile" },
        conversion_witnesses: [
          {
            witness_id: "opaque witness",
            target_quantity: { target_field: "opaque target" },
            conversion_factor_to_target: 7
          }
        ]
      }
    };

    const controlled = controlRouteExport(payload, {
      routeId: "DOTH-FORMAT-003",
      exportContext: "downstream_tool"
    });
    const expectedStructural = controlled.decisions.filter(
      (item) =>
        !item.path.includes("opaque_descendants") &&
        (item.path.endsWith("scope_items[0]") ||
          item.path.endsWith("profile_id") ||
          item.path.endsWith("witness_id") ||
          item.path.endsWith("target_field") ||
          item.path.endsWith("conversion_factor_to_target"))
    );
    const opaque = controlled.decisions.filter((item) => item.path.includes("opaque_descendants"));

    expect(expectedStructural).toHaveLength(5);
    expect(expectedStructural.every((item) => item.action === "include")).toBe(true);
    expect(opaque).toHaveLength(5);
    expect(opaque.every((item) => item.privacy_classification === "unknown")).toBe(true);
    expect(opaque.every((item) => item.redistribution_status === "unknown")).toBe(true);
    expect(opaque.every((item) => item.action === "redact_value")).toBe(true);
    expect(controlled.payload).toMatchObject({
      opaque_descendants: {
        scope_items: [REDACTED_VALUE],
        export_profile: { profile_id: REDACTED_VALUE },
        conversion_witnesses: [
          {
            witness_id: REDACTED_VALUE,
            target_quantity: { target_field: REDACTED_VALUE },
            conversion_factor_to_target: REDACTED_VALUE
          }
        ]
      }
    });
  });

  it("keeps safe-token names, suffix lookalikes, and opaque descendants unknown", () => {
    const payload = {
      document_kind: "openpipestress.technical_preview.conservative_pcf_export_package",
      deliverable_id: "DEL-17-07",
      scope_items: ["SOW-030"],
      export_profile: {
        profile_id: "ops.pcf.conservative_subset",
        target_family: "pcf"
      },
      conversion_witnesses: [],
      schema: "opaque root schema",
      nested_deliverable_id: "DEL-99-99",
      opaque: {
        target_family: "opaque target",
        schema_version: "opaque version",
        target_family_status: "opaque status",
        nested: { deliverable_id: "opaque nested deliverable" }
      }
    };
    const controlled = controlRouteExport(payload, {
      routeId: "DOTH-FORMAT-003",
      exportContext: "downstream_tool"
    });
    const adversarial = controlled.decisions.filter(
      (decision) =>
        decision.path.endsWith("schema") ||
        decision.path.endsWith("nested_deliverable_id") ||
        decision.path.includes("opaque")
    );

    expect(adversarial).toHaveLength(6);
    expect(adversarial.every((decision) => decision.privacy_classification === "unknown")).toBe(true);
    expect(adversarial.every((decision) => decision.redistribution_status === "unknown")).toBe(true);
    expect(adversarial.every((decision) => decision.review_status === "pending")).toBe(true);
    expect(adversarial.every((decision) => decision.action === "redact_value")).toBe(true);
    expect(controlled.payload).toMatchObject({
      schema: REDACTED_VALUE,
      nested_deliverable_id: REDACTED_VALUE,
      opaque: {
        target_family: REDACTED_VALUE,
        schema_version: REDACTED_VALUE,
        target_family_status: REDACTED_VALUE,
        nested: { deliverable_id: REDACTED_VALUE }
      }
    });
  });

  it.each([
    { routeId: "DOTH-HANDOFF-002", deliverableId: "DEL-17-07" },
    { routeId: "DOTH-FORMAT-003", deliverableId: "DEL-99-99" }
  ])("does not apply PCF structural authority for $routeId / $deliverableId", ({ routeId, deliverableId }) => {
    const controlled = controlRouteExport(
      {
        document_kind: "openpipestress.technical_preview.conservative_pcf_export_package",
        deliverable_id: deliverableId,
        scope_items: ["SOW-030"],
        export_profile: { profile_id: "ops.pcf.conservative_subset", target_family: "pcf" },
        conversion_witnesses: []
      },
      { routeId, exportContext: "downstream_tool" }
    );
    const structuralLookalikes = controlled.decisions.filter(
      (decision) =>
        decision.path.endsWith("scope_items[0]") ||
        decision.path.endsWith("export_profile.__route_key__profile_id") ||
        decision.path.endsWith("export_profile.__route_key__target_family")
    );

    expect(structuralLookalikes).toHaveLength(3);
    expect(structuralLookalikes.every((decision) => decision.privacy_classification === "unknown")).toBe(true);
    expect(structuralLookalikes.every((decision) => decision.action === "redact_value")).toBe(true);
  });

  it("keeps an arbitrary unmetadataed leaf unknown despite false payload screening flags", () => {
    const controlled = controlRouteExport(
      {
        private_payload_included: false,
        protected_content_included: false,
        opaque_leaf: "Invented value without leaf metadata"
      },
      {
        routeId: "DOTH-HANDOFF-002",
        exportContext: "downstream_tool"
      }
    );
    const opaqueDecision = controlled.decisions.find((decision) => decision.path.endsWith("opaque_leaf"));

    expect(opaqueDecision).toMatchObject({
      privacy_classification: "unknown",
      redistribution_status: "unknown",
      review_status: "pending",
      action: "redact_value",
      reason_code: "REDISTRIBUTION_STATUS_UNKNOWN"
    });
    expect((controlled.payload as { opaque_leaf: string }).opaque_leaf).toBe(REDACTED_VALUE);
  });

  it("does not infer public unit metadata from unmetadataed key names", () => {
    const source = {
      unit_policy_evidence: {
        source_quantity: { value: 42, unit: "N", dimension: "force" }
      }
    };
    const original = structuredClone(source);
    const controlled = controlRouteExport(source, {
      routeId: "DOTH-HANDOFF-002",
      exportContext: "downstream_tool"
    });
    expect(source).toEqual(original);
    expect(controlled.payload).toEqual({
      unit_policy_evidence: {
        source_quantity: { value: REDACTED_VALUE, unit: REDACTED_VALUE, dimension: REDACTED_VALUE }
      }
    });
  });
});

// W-18b (r1), DEL-08-01 Design `report-user-release.md` §3 and §6: the user-released
// context. Twins of the python checks in `tests/security/test_redaction_export_controls.py`.

/** Keys sorted, no whitespace: the same text as python's
 * `json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)` for the
 * corpus's values. */
function canonicalText(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonicalText).join(",")}]`;
  if (typeof value === "object" && value !== null) {
    return `{${Object.keys(value)
      .sort()
      .map((key) => `${JSON.stringify(key)}:${canonicalText((value as Record<string, unknown>)[key])}`)
      .join(",")}}`;
  }
  return JSON.stringify(value);
}

const EXISTING_CASE_COUNT = 23;
const EXISTING_CASES_SHA256 = "1ad77bb8ba548ee5b9cc018a26493f73797df4751316cc1c335327b91afd142f";
const NON_PROJECT_PRIVATE_CLASSES = [
  "private_material_data",
  "private_component_data",
  "private_rule_pack_data",
  "owner_standard_data",
  "company_design_basis_data",
  "path_data",
  "secret_like_data"
];
const REDISTRIBUTION_STATUSES: unknown[] = [
  "public_permissive",
  "invented_non_engineering_example",
  "private_only",
  "unknown",
  "TBD",
  "unrecognized_status",
  null,
  undefined
];

function privateItem(extra: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    field_id: "invented.project.name",
    field_class: "project",
    privacy_classification: "private_project_data",
    redistribution_status: "private_only",
    review_status: "accepted",
    value: "Invented Released Project A",
    ...extra
  };
}

describe("user-released context: the existing corpus is unchanged", () => {
  it("pins the SHA-256 of the existing cases' canonical form", () => {
    const existing = corpus.cases.filter((parityCase) => !("explicit_user_release_intent" in parityCase));
    expect(existing).toHaveLength(EXISTING_CASE_COUNT);
    expect(existing.every((parityCase) => (EXPORT_CONTEXTS as string[]).includes(parityCase.export_context))).toBe(true);
    expect(createHash("sha256").update(canonicalText(existing), "utf8").digest("hex")).toBe(EXISTING_CASES_SHA256);
  });

  it("carries the release intent only on the new cases, and covers the new reason codes", () => {
    const added = corpus.cases.filter((parityCase) => "explicit_user_release_intent" in parityCase);
    expect(added.length).toBeGreaterThan(0);
    expect(added.every((parityCase) => typeof parityCase.explicit_user_release_intent === "boolean")).toBe(true);
    expect(
      added.every(
        (parityCase) =>
          parityCase.explicit_user_release_intent === false || parityCase.export_context === USER_RELEASED_REPORT_CONTEXT
      )
    ).toBe(true);
    const reasons = new Set(added.map((parityCase) => parityCase.expected.reason_code));
    for (const reason of ["USER_RELEASED_PROJECT_DATA", "USER_RELEASE_INTENT_REQUIRED", "USER_RELEASE_VALUE_UNBOUND"]) {
      expect(reasons.has(reason as RedactionReasonCode)).toBe(true);
    }
  });
});

describe("user-released context: owner property 1, the flag cannot leak private values", () => {
  it("throws for release intent with every corpus item in each of the five existing contexts", () => {
    for (const parityCase of corpus.cases) {
      for (const exportContext of EXPORT_CONTEXTS) {
        expect(() =>
          classifyExportItem(parityCase.item, { exportContext, explicitUserReleaseIntent: true })
        ).toThrowError(/admitted only in the user_released_report context/);
        expect(() =>
          redactExportPayload({ item: parityCase.item }, { exportContext, explicitUserReleaseIntent: true })
        ).toThrowError(/admitted only in the user_released_report context/);
        expect(() =>
          controlRouteExport(
            { item: parityCase.item },
            { routeId: "DREP-PACKAGE-SAVE-009", exportContext, explicitUserReleaseIntent: true }
          )
        ).toThrowError(/admitted only in the user_released_report context/);
        expect(() =>
          controlRouteExport(
            { item: parityCase.item },
            { routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID, exportContext, explicitUserReleaseIntent: true }
          )
        ).toThrowError(/admitted only in the user_released_report context/);
      }
    }
  });

  it("leaves an absent or false flag inert in the existing contexts", () => {
    for (const exportContext of EXPORT_CONTEXTS) {
      const withoutFlag = redactExportPayload({ item: privateItem() }, { exportContext, explicitLocalPrivateIntent: true });
      const falseFlag = redactExportPayload(
        { item: privateItem() },
        { exportContext, explicitLocalPrivateIntent: true, explicitUserReleaseIntent: false }
      );
      expect(falseFlag).toEqual(withoutFlag);
    }
  });

  it("grants nothing for payload-carried intent members, at the top level and nested", () => {
    const carried = {
      user_release_intent: true,
      explicit_user_release_intent: true,
      local_private_intent: true,
      explicit_local_private_intent: true,
      user_intent: true
    };
    // Nested in the items themselves: each item is decided by step 11 without intent.
    const itemsPayload = {
      item: privateItem({ ...carried, export_policy: { ...carried } }),
      nested: { deeper: [{ item: privateItem(carried) }] }
    };
    expect(() =>
      redactExportPayload(itemsPayload, { exportContext: WALK_RELEASED_CONTEXT, explicitLocalPrivateIntent: true })
    ).toThrowError(WALK_REFUSAL);
    const itemsProjected = controlUserReleasedReportPackageRequest(itemsPayload, {}, false);
    expect(itemsProjected.blocked).toBe(true);
    expect(itemsProjected.payload).toBeNull();
    expect(itemsProjected.decisions.length).toBeGreaterThan(0);
    expect(itemsProjected.decisions.every((decision) => decision.reason_code === "USER_RELEASE_INTENT_REQUIRED")).toBe(true);

    // At the top level and at every depth of a request on the release projector.
    const payload = {
      ...carried,
      item: privateItem({ ...carried, export_policy: { ...carried } }),
      nested: { ...carried, deeper: [{ ...carried, item: privateItem(carried) }] }
    };

    const projected = controlUserReleasedReportPackageRequest(payload, {}, false);
    expect(projected.blocked).toBe(true);
    expect(projected.payload).toBeNull();
    expect(projected.decisions.every((decision) => decision.reason_code === "USER_RELEASE_INTENT_REQUIRED")).toBe(true);
  });

  it("redacts each non-project private class with every redistribution status, and the lossless route withholds it", () => {
    for (const privacy of NON_PROJECT_PRIVATE_CLASSES) {
      for (const redistribution of REDISTRIBUTION_STATUSES) {
        const item: Record<string, unknown> = { field_id: "invented.private", field_class: "rule_pack", privacy_classification: privacy, value: 4.2 };
        if (redistribution !== undefined) item.redistribution_status = redistribution;
        const decision = classifyExportItem(item, { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true });
        expect(decision).toMatchObject({ action: "redact_value", reason_code: "PRIVATE_DATA_REDACTED" });

        const projected = controlUserReleasedReportPackageRequest(
          { section: { privacy_classification: privacy, ...(redistribution === undefined ? {} : { redistribution_status: redistribution }), note: "Invented note" } },
          {},
          true
        );
        expect(projected.blocked).toBe(true);
        expect(projected.payload).toBeNull();
        expect(projected.summary.materialization_withheld).toBe(true);
      }
    }
  });

  it("decides by steps 10-13 cases the existing order would decide in an unknown branch", () => {
    const decide = (item: Record<string, unknown>, explicitUserReleaseIntent: boolean) =>
      classifyExportItem(item, { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent });
    const existing = (item: Record<string, unknown>) =>
      classifyExportItem(item, { exportContext: "public_report" });

    const step10 = { field_id: "x", privacy_classification: "private_rule_pack_data", redistribution_status: "unknown", value: 1 };
    expect(existing(step10).reason_code).toBe("REDISTRIBUTION_STATUS_UNKNOWN");
    expect(decide(step10, true)).toMatchObject({ action: "redact_value", reason_code: "PRIVATE_DATA_REDACTED" });

    const step11 = { field_id: "x", privacy_classification: "private_project_data", redistribution_status: "TBD", value: 1 };
    expect(existing(step11).reason_code).toBe("REDISTRIBUTION_STATUS_UNKNOWN");
    expect(decide(step11, true)).toMatchObject({ action: "warning_only", reason_code: "USER_RELEASED_PROJECT_DATA" });
    expect(decide(step11, false)).toMatchObject({ action: "block_export", reason_code: "USER_RELEASE_INTENT_REQUIRED" });

    // Step 12 is reached with metadata the existing order also includes; the existing
    // unknown branch is never reached for it.
    const step12 = { field_id: "x", privacy_classification: "public_metadata", redistribution_status: "public_permissive", value: 1 };
    expect(decide(step12, false)).toMatchObject({ action: "include", reason_code: "SAFE_PUBLIC_METADATA" });

    const missing = { field_id: "x", value: 1 };
    expect(classifyExportItem(missing, { exportContext: "local_private" })).toMatchObject({
      action: "warning_only",
      reason_code: "MISSING_METADATA_REDACTED"
    });
    expect(decide(missing, true)).toMatchObject({ action: "block_export", reason_code: "USER_RELEASE_VALUE_UNBOUND" });

    const unknownPrivacy = { field_id: "x", privacy_classification: "unknown", redistribution_status: "private_only", value: 1 };
    expect(classifyExportItem(unknownPrivacy, { exportContext: "local_private" }).action).toBe("warning_only");
    expect(decide(unknownPrivacy, true)).toMatchObject({ action: "block_export", reason_code: "USER_RELEASE_VALUE_UNBOUND" });
  });

  it("blocks protected, quarantined and rejected content before the new steps", () => {
    for (const extra of [
      { privacy_classification: "protected_suspected" },
      { redistribution_status: "protected_suspected" },
      { review_status: "quarantined" },
      { review_status: "rejected" }
    ]) {
      expect(
        classifyExportItem(privateItem(extra), { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true })
      ).toMatchObject({ action: "block_export", reason_code: "PROTECTED_CONTENT_BLOCKED" });
    }
  });
});

describe("user-released context: local-first admission (§3.3)", () => {
  it("admits the context only on route 010 with the wrapper's release intent, under private_report", () => {
    expect(
      enforceLocalFirstRoute({
        routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
        exportContext: USER_RELEASED_REPORT_CONTEXT,
        explicitUserReleaseIntent: true
      })
    ).toEqual({
      route_id: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
      export_context: USER_RELEASED_REPORT_CONTEXT,
      storage_context: "private_report",
      action: "include_metadata_only",
      reason_code: "USER_RELEASE_METADATA_ALLOWED",
      blocked: false,
      metadata_only: true,
      explicit_local_private_intent: false,
      explicit_user_release_intent: true
    });
    expect(
      enforceLocalFirstRoute({ routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID, exportContext: USER_RELEASED_REPORT_CONTEXT })
    ).toMatchObject({ action: "block_storage", reason_code: "USER_RELEASE_INTENT_REQUIRED", blocked: true, explicit_user_release_intent: false });
    expect(
      enforceLocalFirstRoute({
        routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
        exportContext: USER_RELEASED_REPORT_CONTEXT,
        explicitLocalPrivateIntent: true
      })
    ).toMatchObject({ reason_code: "USER_RELEASE_INTENT_REQUIRED", blocked: true });
  });

  it("refuses the context on any other route and route 010 in any other context", () => {
    for (const routeId of ["DREP-PACKAGE-SAVE-009", "DREP-IPC-003", "DREP-HTML-SAVE-005", "DOTH-JSON-001"]) {
      expect(
        enforceLocalFirstRoute({ routeId, exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true })
      ).toMatchObject({ action: "block_storage", reason_code: "USER_RELEASE_ROUTE_REQUIRED", blocked: true, storage_context: "private_report" });
    }
    for (const exportContext of EXPORT_CONTEXTS) {
      expect(
        enforceLocalFirstRoute({
          routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
          exportContext,
          explicitLocalPrivateIntent: true,
          explicitUserReleaseIntent: true
        })
      ).toMatchObject({ action: "block_storage", reason_code: "USER_RELEASE_ROUTE_REQUIRED", blocked: true, storage_context: exportContext });
    }
    expect(
      enforceLocalFirstRoute({
        routeId: "INVENTED-UNKNOWN-ROUTE",
        exportContext: USER_RELEASED_REPORT_CONTEXT,
        explicitUserReleaseIntent: true
      })
    ).toMatchObject({ reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN", blocked: true });
  });

  it("keeps every existing route's evidence members exactly as before", () => {
    for (const exportContext of EXPORT_CONTEXTS) {
      for (const explicitUserReleaseIntent of [undefined, false]) {
        const evidence = enforceLocalFirstRoute({
          routeId: "DREP-PACKAGE-SAVE-009",
          exportContext,
          explicitLocalPrivateIntent: true,
          explicitUserReleaseIntent
        });
        expect(Object.keys(evidence)).toEqual([
          "route_id",
          "export_context",
          "storage_context",
          "action",
          "reason_code",
          "blocked",
          "metadata_only",
          "explicit_local_private_intent"
        ]);
        expect(evidence.storage_context).toBe(exportContext);
      }
    }
  });

  it("refuses user-release intent in every existing context on every governed route (Codex r1 finding 2)", () => {
    const routes = [...GOVERNED_LOCAL_FIRST_ROUTE_IDS].filter((routeId) => routeId !== REPORT_PACKAGE_USER_RELEASE_ROUTE_ID);
    expect(routes.length).toBeGreaterThan(10);
    for (const routeId of routes) {
      for (const exportContext of EXPORT_CONTEXTS) {
        for (const explicitLocalPrivateIntent of [false, true]) {
          // An absent or false flag leaves the evidence byte for byte as without the option.
          const without = enforceLocalFirstRoute({ routeId, exportContext, explicitLocalPrivateIntent });
          expect(without).not.toHaveProperty("explicit_user_release_intent");
          expect(without.reason_code).not.toMatch(/^USER_RELEASE_/);
          for (const absentOrFalse of [undefined, false, null]) {
            expect(
              JSON.stringify(
                enforceLocalFirstRoute({
                  routeId,
                  exportContext,
                  explicitLocalPrivateIntent,
                  explicitUserReleaseIntent: absentOrFalse as boolean | undefined
                })
              )
            ).toBe(JSON.stringify(without));
          }
          // Any stated intent is refused by name; it never admits the existing context.
          for (const stated of [true, "yes", 1]) {
            expect(
              enforceLocalFirstRoute({
                routeId,
                exportContext,
                explicitLocalPrivateIntent,
                explicitUserReleaseIntent: stated as boolean
              })
            ).toEqual({
              route_id: routeId,
              export_context: exportContext,
              storage_context: exportContext,
              action: "block_storage",
              reason_code: "USER_RELEASE_ROUTE_REQUIRED",
              blocked: true,
              metadata_only: true,
              explicit_local_private_intent: explicitLocalPrivateIntent,
              explicit_user_release_intent: stated === true
            });
          }
        }
      }
    }
    // The reproduction from the review: no longer admitted as safe public metadata.
    expect(
      enforceLocalFirstRoute({ routeId: "DREP-PACKAGE-SAVE-009", exportContext: "public_report", explicitUserReleaseIntent: true })
    ).toMatchObject({ action: "block_storage", reason_code: "USER_RELEASE_ROUTE_REQUIRED", blocked: true });
    // Unknown routes and invalid contexts stay refused by their own reasons, with the stated
    // intent recorded.
    expect(
      enforceLocalFirstRoute({ routeId: "INVENTED-UNKNOWN-ROUTE", exportContext: "public_report", explicitUserReleaseIntent: true })
    ).toMatchObject({ reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN", blocked: true, explicit_user_release_intent: true });
    expect(
      enforceLocalFirstRoute({ routeId: "DREP-PACKAGE-SAVE-009", exportContext: "invented_context", explicitUserReleaseIntent: true })
    ).toMatchObject({ reason_code: "LOCAL_FIRST_EXPORT_CONTEXT_INVALID", blocked: true, explicit_user_release_intent: true });
  });
});

describe("user-released context: the release projector (§3.2)", () => {
  const releasedRequest = () => ({
    package_id: "desktop-user-released-report-invented-run",
    report: {
      report_context: { export_context: "user_released_report", generated_at: null },
      provenance_notes: [
        {
          source_name: "Invented desktop session",
          privacy_classification: "private_project_data",
          redistribution_status: "private_only",
          review_status: "pending"
        }
      ],
      result_rows: [{ row_id: "invented-row-1", value: 12.5, unit: "MPa", metadata: { note: "Invented note" } }]
    }
  });
  const releasedModel = () => ({
    project: { id: "invented-project", name: "Invented Released Project A" },
    materials: [{ id: "m1", provenance_record: { redistribution_status: "public_permissive", review_status: "pending" } }]
  });

  it("admits an all-project request with the wrapper's intent, exactly, and blocks it without", () => {
    const request = releasedRequest();
    const model = releasedModel();
    const snapshot = structuredClone({ request, model });

    const admitted = controlUserReleasedReportPackageRequest(request, model, true);
    expect(admitted.blocked).toBe(false);
    expect(canonicalText(admitted.payload)).toBe(canonicalText(request));
    expect(admitted.summary).toMatchObject({
      route_id: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
      materialization_withheld: false,
      blocking_count: 0,
      local_first: { reason_code: "USER_RELEASE_METADATA_ALLOWED", storage_context: "private_report", blocked: false }
    });
    expect(admitted.decisions.every((decision) => decision.export_context === USER_RELEASED_REPORT_CONTEXT)).toBe(true);
    expect(admitted.decisions.every((decision) => decision.reason_code === "USER_RELEASED_PROJECT_DATA")).toBe(true);

    const refused = controlUserReleasedReportPackageRequest(request, model, false);
    expect(refused.blocked).toBe(true);
    expect(refused.payload).toBeNull();
    expect(refused.summary.local_first?.reason_code).toBe("USER_RELEASE_INTENT_REQUIRED");
    expect(refused.decisions.every((decision) => decision.reason_code === "USER_RELEASE_INTENT_REQUIRED")).toBe(true);
    expect({ request, model }).toEqual(snapshot);
  });

  it("labels by the nearest enclosing object, never by path tokens", () => {
    const request = {
      public_record: {
        privacy_classification: "public_metadata",
        redistribution_status: "public_permissive",
        stress_value: "Invented public label",
        nested: { privacy_classification: "private_project_data", value: 1 }
      }
    };
    const admitted = controlUserReleasedReportPackageRequest(request, {}, false);
    const stress = admitted.decisions.find((decision) => decision.path.endsWith("stress_value"));
    const nested = admitted.decisions.find((decision) => decision.path === "$.request.public_record.nested.value");
    expect(stress).toMatchObject({ privacy_classification: "public_metadata", action: "include" });
    expect(nested).toMatchObject({ privacy_classification: "private_project_data", reason_code: "USER_RELEASE_INTENT_REQUIRED" });
  });

  it("withholds a non-project private class, blocks protected and unknown own metadata, in the request or the model", () => {
    const cases: Array<[Record<string, unknown>, RedactionReasonCode]> = [
      [{ privacy_classification: "private_material_data" }, "PRIVATE_DATA_REDACTED"],
      [{ privacy_class: "private_rule_pack_data" }, "PRIVATE_DATA_REDACTED"],
      [{ privacy_classification: "protected_suspected" }, "PROTECTED_CONTENT_BLOCKED"],
      [{ redistribution_status: "protected_suspected" }, "PROTECTED_CONTENT_BLOCKED"],
      [{ review_status: "quarantined" }, "PROTECTED_CONTENT_BLOCKED"],
      [{ review_status: "rejected" }, "PROTECTED_CONTENT_BLOCKED"],
      [{ privacy_classification: "unknown" }, "USER_RELEASE_VALUE_UNBOUND"],
      [{ privacy_classification: null }, "USER_RELEASE_VALUE_UNBOUND"],
      [{ privacy_class: "private_user_supplied" }, "USER_RELEASE_VALUE_UNBOUND"]
    ];
    for (const [labels, reason] of cases) {
      const marked = { id: "invented", ...labels, deeper: { note: "Invented note" } };
      for (const [request, model] of [
        [{ ...releasedRequest(), marked }, releasedModel()],
        [releasedRequest(), { ...releasedModel(), marked }]
      ]) {
        const projected = controlUserReleasedReportPackageRequest(request, model, true);
        expect(projected.blocked).toBe(true);
        expect(projected.payload).toBeNull();
        expect(projected.decisions.some((decision) => decision.reason_code === reason)).toBe(true);
      }
    }
  });

  it("keeps a blocking disposition on every leaf beneath it", () => {
    const projected = controlUserReleasedReportPackageRequest(
      { held: { review_status: "quarantined", inner: { review_status: "accepted", privacy_classification: "public_metadata", note: "x" } } },
      {},
      true
    );
    const inner = projected.decisions.find((decision) => decision.path === "$.request.held.inner.note");
    expect(inner).toMatchObject({ review_status: "quarantined", reason_code: "PROTECTED_CONTENT_BLOCKED" });
  });

  // Codex r1 finding 1: steps 1-9 precede the nearest-metadata labelling, for every enclosing
  // object of the request and of the model, exactly as `classifyExportItem` decides it.
  const projectMetadata = { privacy_classification: "private_project_data", redistribution_status: "private_only" };
  const boundaryMarkers: Array<[string, Record<string, unknown>, RedactionAction, RedactionReasonCode]> = [
    ["1 professional claim", { professional_claim: true }, "block_export", "PROFESSIONAL_BOUNDARY_BLOCKED"],
    ["1 professional claim text", { professional_claim: "yes" }, "block_export", "PROFESSIONAL_BOUNDARY_BLOCKED"],
    ["2 network flag", { cloud_or_network_reference: true }, "block_export", "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"],
    ["2 network detail", { remote_url: "invented-remote" }, "block_export", "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"],
    ["3 SQL flag", { direct_sql_access: true }, "block_export", "DIRECT_SQL_ACCESS_BLOCKED"],
    ["3 SQL detail", { connection_string: "invented-connection" }, "block_export", "DIRECT_SQL_ACCESS_BLOCKED"],
    ["4 storage bypass", { storage_bypass: true }, "block_export", "STORAGE_BYPASS_BLOCKED"],
    ["4 storage bypass requested", { storage_bypass_requested: "yes" }, "block_export", "STORAGE_BYPASS_BLOCKED"],
    ["4 direct storage bypass", { direct_storage_bypass: true }, "block_export", "STORAGE_BYPASS_BLOCKED"],
    ["4 application service bypass", { application_service_bypass: 1 }, "block_export", "STORAGE_BYPASS_BLOCKED"],
    ["5 secret flag", { secret_material_present: true }, "block_export", "SECRET_MATERIAL_BLOCKED"],
    ["5 secret detail", { password: "INVENTED_SECRET_MARKER" }, "block_export", "SECRET_MATERIAL_BLOCKED"],
    ["6 payload flag", { contains_payload: true }, "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"],
    ["6 payload present", { payload_present: "present" }, "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"],
    ["6 cache payload", { cache_payload: 1 }, "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"],
    ["6 sqlite payload", { sqlite_payload: 1 }, "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"],
    ["7 path flag", { concrete_path_present: true }, "redact_value", "CONCRETE_PATH_REDACTED"],
    ["7 path detail", { file_path: "invented/relative/file" }, "redact_value", "CONCRETE_PATH_REDACTED"],
    ["8 protected privacy", { privacy_classification: "protected_suspected" }, "block_export", "PROTECTED_CONTENT_BLOCKED"],
    ["8 protected redistribution", { redistribution_status: "protected_suspected" }, "block_export", "PROTECTED_CONTENT_BLOCKED"],
    [
      "8 protected in provenance",
      { privacy_classification: null, provenance: { privacy_classification: "protected_suspected" } },
      "block_export",
      "PROTECTED_CONTENT_BLOCKED"
    ],
    ["9 quarantined", { review_status: "quarantined" }, "block_export", "PROTECTED_CONTENT_BLOCKED"],
    ["9 rejected", { review_status: "rejected" }, "block_export", "PROTECTED_CONTENT_BLOCKED"],
    [
      "9 quarantined in provenance",
      { review_status: "unknown", provenance: { review_status: "quarantined" } },
      "block_export",
      "PROTECTED_CONTENT_BLOCKED"
    ]
  ];

  it("reproduces the review's case: a secret-marked project item is blocked, not admitted", () => {
    const item = {
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      secret_material_present: true,
      value: "INVENTED_SECRET_MARKER"
    };
    expect(classifyExportItem(item, { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true })).toMatchObject({
      action: "block_export",
      reason_code: "SECRET_MATERIAL_BLOCKED"
    });
    for (const [request, model] of [
      [item, {}],
      [{}, item]
    ]) {
      const projected = controlUserReleasedReportPackageRequest(request, model, true);
      expect(projected.blocked).toBe(true);
      expect(projected.payload).toBeNull();
      expect(projected.decisions.some((decision) => decision.reason_code === "SECRET_MATERIAL_BLOCKED")).toBe(true);
      expect(JSON.stringify(projected.payload)).not.toContain("INVENTED_SECRET_MARKER");
    }
  });

  for (const [step, marker, action, reason] of boundaryMarkers) {
    it(`applies step ${step} before labelling, at depth, in the request and in the model`, () => {
      const item = { ...projectMetadata, ...marker, value: "Invented marked value" };
      const expected = classifyExportItem(item, { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true });
      expect(expected).toMatchObject({ action, reason_code: reason });
      // Without the marker the same item is admitted with the wrapper's intent.
      expect(
        controlUserReleasedReportPackageRequest({ item: { ...projectMetadata, value: "Invented marked value" } }, {}, true).blocked
      ).toBe(false);

      const nest = (inner: unknown) => ({ section: { ...projectMetadata, rows: [{ note: "Invented note" }, { detail: { inner } }] } });
      const markedPath = (root: string) => `${root}.section.rows[1].detail.inner`;
      for (const [request, model, root] of [
        [nest(item), releasedModel(), "$.request"],
        [releasedRequest(), { ...releasedModel(), ...nest(item) }, "$.model"],
        [item, releasedModel(), "$.request"]
      ] as Array<[unknown, unknown, string]>) {
        const projected = controlUserReleasedReportPackageRequest(request, model, true);
        expect(projected.blocked).toBe(true);
        expect(projected.payload).toBeNull();
        expect(projected.summary.materialization_withheld).toBe(true);
        const objectPath = request === item ? "$.request" : markedPath(root);
        const decision = projected.decisions.find((candidate) => candidate.path === objectPath);
        expect(decision).toBeDefined();
        const { decision_id: _decisionId, ...decided } = decision!;
        const { decision_id: _expectedId, ...wanted } = classifyExportItem(item, {
          exportContext: USER_RELEASED_REPORT_CONTEXT,
          explicitUserReleaseIntent: true,
          path: objectPath
        });
        expect(decided).toEqual(wanted);
        expect(projected.findings.some((finding) => finding.path === objectPath && finding.code === reason)).toBe(true);
      }
    });
  }

  it("decides every enclosing object, not only the nearest one that states metadata", () => {
    // A marker on an object without metadata, beneath a labelled project object, and on an
    // array element of the model.
    for (const [request, model] of [
      [{ section: { ...projectMetadata, block: { token: "INVENTED_SECRET_MARKER" } } }, {}],
      [{}, { components: [{ id: "c1" }, { id: "c2", sql: "invented" }] }]
    ]) {
      const projected = controlUserReleasedReportPackageRequest(request, model, true);
      expect(projected.blocked).toBe(true);
      expect(projected.payload).toBeNull();
      expect(
        projected.decisions.some((decision) =>
          ["SECRET_MATERIAL_BLOCKED", "DIRECT_SQL_ACCESS_BLOCKED"].includes(decision.reason_code)
        )
      ).toBe(true);
    }
  });

  it("labels from the provenance as classifyExportItem reads it", () => {
    const projected = controlUserReleasedReportPackageRequest(
      { record: { provenance: { privacy_classification: "private_rule_pack_data" }, note: "Invented note" } },
      {},
      true
    );
    expect(projected.blocked).toBe(true);
    expect(projected.decisions.find((decision) => decision.path === "$.request.record.note")).toMatchObject({
      privacy_classification: "private_rule_pack_data",
      reason_code: "PRIVATE_DATA_REDACTED"
    });
  });

  describe("the generic route exporter refuses the user-released context and route 010 outright (Codex r2)", () => {
    const refusal = /^USER_RELEASE_ROUTE_REQUIRED: the generic route exporter does not admit/;
    const flagRefusal = /admitted only in the user_released_report context/;
    const reviewPayload = {
      project_ref: {
        privacy_classification: "private_rule_pack_data",
        secret_material_present: true,
        data: "INVENTED_PRIVATE_RULE_VALUE"
      }
    };
    const variants: Array<[string, unknown, boolean]> = [
      ["the review's payload", reviewPayload, true],
      ...boundaryMarkers.map(
        ([step, marker]) =>
          [`step ${step}`, { project_ref: { ...projectMetadata, ...marker, data: "INVENTED_MARKED_VALUE" } }, true] as [string, unknown, boolean]
      ),
      ...NON_PROJECT_PRIVATE_CLASSES.map(
        (privacy) =>
          [
            `non-project class ${privacy}`,
            { project_ref: { privacy_classification: privacy, redistribution_status: "private_only", data: "INVENTED_PRIVATE_VALUE" } },
            true
          ] as [string, unknown, boolean]
      ),
      ["a clean project-data request", releasedRequest(), false],
      ["a raw payload without metadata", { project_ref: { data: "INVENTED_RAW_VALUE" } }, false]
    ];
    const routes = [...GOVERNED_LOCAL_FIRST_ROUTE_IDS, "INVENTED-UNKNOWN-ROUTE"];
    const optionSets = [
      {},
      { requireLosslessMaterialization: true },
      { explicitLocalPrivateIntent: true },
      { explicitLocalPrivateIntent: true, requireLosslessMaterialization: true }
    ];
    const releasedContext = USER_RELEASED_REPORT_CONTEXT as unknown as RedactionExportContext;

    it("refuses the user-released context on every route, with or without the wrapper's release intent", () => {
      for (const [label, payload] of variants) {
        const snapshot = canonicalText(payload);
        for (const routeId of routes) {
          for (const flag of [{ explicitUserReleaseIntent: true }, { explicitUserReleaseIntent: false }, {}]) {
            for (const options of optionSets) {
              expect(
                () => controlRouteExport(payload, { routeId, exportContext: releasedContext, ...flag, ...options }),
                `${label} on ${routeId}`
              ).toThrowError(refusal);
            }
          }
        }
        expect(canonicalText(payload)).toBe(snapshot);
      }
    });

    it("refuses route 010 in every existing context; release intent there is refused by the flag assertion first", () => {
      for (const [label, payload] of variants) {
        for (const exportContext of EXPORT_CONTEXTS) {
          for (const options of optionSets) {
            for (const flag of [{ explicitUserReleaseIntent: false }, {}]) {
              expect(
                () =>
                  controlRouteExport(payload, { routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID, exportContext, ...flag, ...options }),
                `${label} in ${exportContext}`
              ).toThrowError(refusal);
            }
            expect(() =>
              controlRouteExport(payload, {
                routeId: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
                exportContext,
                explicitUserReleaseIntent: true,
                ...options
              })
            ).toThrowError(flagRefusal);
          }
        }
      }
    });

    it("leaves the release projector deciding: the marked and non-project variants block, a clean request is admitted", () => {
      for (const [label, payload, refused] of variants) {
        const projected = controlUserReleasedReportPackageRequest(payload, {}, true);
        expect(projected.blocked, label).toBe(refused);
        if (refused) expect(projected.payload, label).toBeNull();
      }
      const request = releasedRequest();
      const admitted = controlUserReleasedReportPackageRequest(request, releasedModel(), true);
      expect(admitted.blocked).toBe(false);
      expect(canonicalText(admitted.payload)).toBe(canonicalText(request));
      expect(admitted.decisions.every((decision) => decision.reason_code === "USER_RELEASED_PROJECT_DATA")).toBe(true);
      expect(controlUserReleasedReportPackageRequest(request, releasedModel(), false).blocked).toBe(true);
    });
  });
});

describe("user-released context: payload-carried local-private intent is not read (Codex r1 finding 4)", () => {
  const withIntentGetters = (reads: string[]): Record<string, unknown> => {
    const item: Record<string, unknown> = {
      field_id: "invented.project.name",
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      value: "Invented value"
    };
    for (const key of ["local_private_intent", "explicit_local_private_intent", "user_intent", "export_policy"]) {
      Object.defineProperty(item, key, {
        enumerable: true,
        get() {
          reads.push(key);
          return key === "export_policy" ? { local_private_intent: true } : true;
        }
      });
    }
    return item;
  };

  it("never reads the intent members in user_released_report, and still reads them in local_private", () => {
    const releasedReads: string[] = [];
    expect(
      classifyExportItem(withIntentGetters(releasedReads), {
        exportContext: USER_RELEASED_REPORT_CONTEXT,
        explicitUserReleaseIntent: false
      })
    ).toMatchObject({ reason_code: "USER_RELEASE_INTENT_REQUIRED" });
    expect(releasedReads).toEqual([]);

    const localReads: string[] = [];
    expect(classifyExportItem(withIntentGetters(localReads), { exportContext: "local_private" })).toMatchObject({
      reason_code: "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
    });
    expect(localReads.length).toBeGreaterThan(0);
  });
});

describe("existing_contexts_preserve_raw_intent_named_members (§3.3, §6)", () => {
  const fixturePath = path.resolve(
    path.dirname(fileURLToPath(import.meta.url)),
    "../../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json"
  );
  const withRawMembers = () => {
    const request = structuredClone(
      (JSON.parse(readFileSync(fixturePath, "utf8")) as { request: Record<string, unknown> }).request
    ) as Record<string, any>;
    const rows = request.result_documents[0].result_envelope.result_sets[0].values as Array<Record<string, any>>;
    const row = rows.find((candidate) => typeof candidate.metadata === "object" && candidate.metadata !== null);
    expect(row).toBeDefined();
    row!.metadata.user_release_intent = true;
    row!.metadata.explicit_user_release_intent = true;
    const section = request.state_comparison_handoff_records[0].sections.state_run_sections[0];
    section.user_release_intent = "explicit";
    section.explicit_user_release_intent = true;
    return request;
  };

  it("preserves them byte for byte on DREP-PACKAGE-SAVE-009 and DREP-IPC-003 under local_private", () => {
    for (const routeId of ["DREP-PACKAGE-SAVE-009", "DREP-IPC-003"]) {
      const request = withRawMembers();
      const before = JSON.stringify(request);
      const controlled = controlRouteExport(request, {
        routeId,
        exportContext: "local_private",
        explicitLocalPrivateIntent: true,
        requireLosslessMaterialization: true
      });
      expect(controlled.blocked).toBe(false);
      expect(controlled.summary.materialization_withheld).toBe(false);
      expect(JSON.stringify(controlled.payload)).toBe(before);
      expect(JSON.stringify(request)).toBe(before);
      expect(controlled.summary.local_first).not.toHaveProperty("explicit_user_release_intent");
    }
  });

  it("preserves them byte for byte in a rendered document on DREP-HTML-SAVE-005", () => {
    const html = '<!doctype html><p data-user_release_intent="true">explicit_user_release_intent: true</p>';
    const controlled = controlRouteExport(html, {
      routeId: "DREP-HTML-SAVE-005",
      exportContext: "local_private",
      explicitLocalPrivateIntent: true,
      requireLosslessMaterialization: true
    });
    expect(controlled.blocked).toBe(false);
    expect(controlled.payload).toBe(html);
  });
});

// Codex r3 of the r1 slice: the walk decided a value-bearing parent as a whole, so a labelled
// parent hid restricted descendants (a nested secret marker, a nested non-project private class,
// a non-project private parent's unredacted `rows`). The walk now refuses the context; the
// user-release projector, which decides every node, is the only payload entry (§3.2).

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** One steps-1-9 marker per row: the members, and the reason the projector records for an object
 * that carries them. */
const STEP_1_TO_9_MARKERS: Array<[Record<string, unknown>, RedactionReasonCode]> = [
  [{ professional_claim: true }, "PROFESSIONAL_BOUNDARY_BLOCKED"],
  [{ cloud_or_network_reference: true }, "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"],
  [{ remote_url: "invented://remote" }, "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"],
  [{ direct_sql_access: true }, "DIRECT_SQL_ACCESS_BLOCKED"],
  [{ sql: "INVENTED_QUERY" }, "DIRECT_SQL_ACCESS_BLOCKED"],
  [{ storage_bypass: true }, "STORAGE_BYPASS_BLOCKED"],
  [{ application_service_bypass: "yes" }, "STORAGE_BYPASS_BLOCKED"],
  [{ secret_material_present: true }, "SECRET_MATERIAL_BLOCKED"],
  [{ password: "INVENTED_SECRET" }, "SECRET_MATERIAL_BLOCKED"],
  [{ contains_payload: true }, "PAYLOAD_METADATA_ONLY_REQUIRED"],
  [{ cache_payload: 1 }, "PAYLOAD_METADATA_ONLY_REQUIRED"],
  [{ payload: "INVENTED_PAYLOAD" }, "PAYLOAD_METADATA_ONLY_REQUIRED"],
  [{ concrete_path_present: true }, "CONCRETE_PATH_REDACTED"],
  [{ file_path: "SYMBOLIC_INVENTED_PATH" }, "CONCRETE_PATH_REDACTED"],
  [{ privacy_classification: "protected_suspected" }, "PROTECTED_CONTENT_BLOCKED"],
  [{ redistribution_status: "protected_suspected" }, "PROTECTED_CONTENT_BLOCKED"],
  [{ review_status: "quarantined" }, "PROTECTED_CONTENT_BLOCKED"],
  [{ review_status: "rejected" }, "PROTECTED_CONTENT_BLOCKED"],
  [{ provenance: { review_status: "quarantined" } }, "PROTECTED_CONTENT_BLOCKED"],
  [{ provenance: { privacy_classification: "protected_suspected" } }, "PROTECTED_CONTENT_BLOCKED"]
];

const CODEX_R3_NESTED_REPRO = {
  privacy_classification: "private_project_data",
  redistribution_status: "private_only",
  value: { privacy_classification: "private_rule_pack_data", secret_material_present: true, value: "INVENTED_NESTED" }
};
const CODEX_R3_SHALLOW_REPRO = {
  privacy_classification: "private_rule_pack_data",
  redistribution_status: "private_only",
  rows: ["INVENTED_NONPROJECT"]
};

/** `child` beneath a labelled project-data parent, `depth` levels down, alternating objects and
 * arrays (depth 0 is the parent's own `value`). */
function underLabelledParent(child: unknown, depth: number): { payload: unknown; childPath: string } {
  let node: unknown = child;
  let suffix = "";
  for (let level = depth; level > 0; level -= 1) {
    if (level % 2 === 1) {
      node = [{ note: "Invented sibling" }, node];
      suffix = `[1]${suffix}`;
    } else {
      node = { [`level_${level}`]: node };
      suffix = `.level_${level}${suffix}`;
    }
  }
  return { payload: { ...privateItem(), value: node }, childPath: `$.request.value${suffix}` };
}

/** The generated inputs: Codex's two repros, each steps-1-9 marker and each non-project private
 * class nested at depths 0-3 beneath a labelled project-data parent (with and without its own
 * privacy), and unlabelled values. */
function generatedReleaseInputs(): Array<{ label: string; payload: unknown; refusedAt?: string; reason?: RedactionReasonCode }> {
  const inputs: Array<{ label: string; payload: unknown; refusedAt?: string; reason?: RedactionReasonCode }> = [
    { label: "codex-r3-nested", payload: CODEX_R3_NESTED_REPRO, refusedAt: "$.request.value", reason: "SECRET_MATERIAL_BLOCKED" },
    { label: "codex-r3-shallow", payload: CODEX_R3_SHALLOW_REPRO, refusedAt: "$.request.rows[0]", reason: "PRIVATE_DATA_REDACTED" },
    { label: "unlabelled", payload: { project_ref: { data: "INVENTED_RAW_VALUE" } } },
    { label: "scalar", payload: "INVENTED_TOP" },
    { label: "null", payload: null }
  ];
  for (let depth = 0; depth <= 3; depth += 1) {
    STEP_1_TO_9_MARKERS.forEach(([marker, reason], index) => {
      // An object's own privacy supersedes its provenance's (as `classifyExportItem` reads it), so
      // the provenance-privacy marker is generated only without one.
      const statesProvenancePrivacy = isRecord(marker.provenance) && "privacy_classification" in marker.provenance;
      const ownPrivacies = statesProvenancePrivacy ? [undefined] : [undefined, "private_project_data", "private_rule_pack_data"];
      for (const ownPrivacy of ownPrivacies) {
        const child = { ...(ownPrivacy === undefined ? {} : { privacy_classification: ownPrivacy }), ...marker, value: "INVENTED_NESTED" };
        const { payload, childPath } = underLabelledParent(child, depth);
        // Re-pinned (input contract, HELP_HUMAN ruling 2026-10-27, (d)): a concrete path no longer
        // turns a block into a redaction. A path-marked object that states no label of its own is
        // decided at the object by step 13, `block_export` / `USER_RELEASE_VALUE_UNBOUND`; it was
        // `redact_value` / `CONCRETE_PATH_REDACTED`. Both refuse the release; the new answer is the
        // stricter action. With its own private label the object still redacts by the path.
        const pathMarker = reason === "CONCRETE_PATH_REDACTED";
        const decidedReason = pathMarker && ownPrivacy === undefined ? "USER_RELEASE_VALUE_UNBOUND" : reason;
        inputs.push({
          label: `marker-${index}-${ownPrivacy ?? "inherited"}-depth-${depth}`,
          payload,
          refusedAt: childPath,
          reason: decidedReason
        });
      }
    });
    for (const privacy of NON_PROJECT_PRIVATE_CLASSES) {
      const { payload, childPath } = underLabelledParent({ privacy_classification: privacy, rows: ["INVENTED_NONPROJECT", { cell: 1 }] }, depth);
      inputs.push({ label: `private-${privacy}-depth-${depth}`, payload, refusedAt: `${childPath}.rows[0]`, reason: "PRIVATE_DATA_REDACTED" });
    }
  }
  return inputs;
}

describe("user-released context: the redaction walk refuses the context; the projector decides every node (§3.2)", () => {
  it("refuses Codex's two repros, the generated inputs and every corpus item at the walk, whatever the intent", () => {
    const corpusItems = corpus.cases.map((parityCase) => ({ item: parityCase.item }));
    const payloads = [...generatedReleaseInputs().map((input) => input.payload), ...corpusItems];
    expect(payloads.length).toBeGreaterThan(300);
    for (const payload of payloads) {
      const before = structuredClone(payload);
      for (const explicitUserReleaseIntent of [true, false, undefined]) {
        for (const explicitLocalPrivateIntent of [true, false]) {
          expect(() =>
            redactExportPayload(payload, {
              exportContext: WALK_RELEASED_CONTEXT,
              explicitLocalPrivateIntent,
              ...(explicitUserReleaseIntent === undefined ? {} : { explicitUserReleaseIntent })
            })
          ).toThrowError(WALK_REFUSAL);
        }
      }
      expect(payload).toEqual(before);
    }
  });

  it("blocks Codex's nested repro and withholds the shallow one on the projector, with the wrapper's intent", () => {
    const nested = controlUserReleasedReportPackageRequest(CODEX_R3_NESTED_REPRO, {}, true);
    expect(nested.blocked).toBe(true);
    expect(nested.payload).toBeNull();
    expect(nested.decisions).toContainEqual(
      expect.objectContaining({ path: "$.request.value", action: "block_export", reason_code: "SECRET_MATERIAL_BLOCKED" })
    );
    expect(nested.decisions).toContainEqual(
      expect.objectContaining({ path: "$.request.value.value", action: "redact_value", reason_code: "PRIVATE_DATA_REDACTED" })
    );

    const shallow = controlUserReleasedReportPackageRequest(CODEX_R3_SHALLOW_REPRO, {}, true);
    expect(shallow.blocked).toBe(true);
    expect(shallow.payload).toBeNull();
    expect(shallow.summary.materialization_withheld).toBe(true);
    expect(shallow.decisions.map((decision) => [decision.path, decision.reason_code])).toEqual([
      ["$.request.privacy_classification", "PRIVATE_DATA_REDACTED"],
      ["$.request.redistribution_status", "PRIVATE_DATA_REDACTED"],
      ["$.request.rows[0]", "PRIVATE_DATA_REDACTED"]
    ]);
  });

  it("refuses each steps-1-9 marker and each non-project private class beneath a labelled parent, in arrays and at depth", () => {
    const refused = generatedReleaseInputs().filter((input) => input.refusedAt !== undefined);
    expect(refused).toHaveLength(2 + 4 * ((STEP_1_TO_9_MARKERS.length - 1) * 3 + 1 + NON_PROJECT_PRIVATE_CLASSES.length));
    for (const input of refused) {
      for (const model of [{}, { privacy_classification: "private_project_data", redistribution_status: "private_only" }]) {
        const projected = controlUserReleasedReportPackageRequest(input.payload, model, true);
        expect(projected.blocked, input.label).toBe(true);
        expect(projected.payload, input.label).toBeNull();
        expect(
          projected.decisions.some((decision) => decision.path === input.refusedAt && decision.reason_code === input.reason),
          input.label
        ).toBe(true);
        // The same value, alone and labelled as the user's project data, is admitted: the refusal
        // comes from the nested node, not from the parent.
        expect(controlUserReleasedReportPackageRequest(privateItem(), model, true).blocked).toBe(false);
      }
    }
  });

  it("keeps the user-release projector's admission of an unlabelled request with the wrapper's intent only", () => {
    const payload = { project_ref: { data: "INVENTED_RAW_VALUE" } };
    const admitted = controlUserReleasedReportPackageRequest(payload, {}, true);
    expect(admitted.blocked).toBe(false);
    expect(admitted.payload).toEqual(payload);
    expect(admitted.decisions.map((decision) => [decision.path, decision.reason_code])).toEqual([
      ["$.request.project_ref.data", "USER_RELEASED_PROJECT_DATA"]
    ]);
    const refused = controlUserReleasedReportPackageRequest(payload, {}, false);
    expect(refused.blocked).toBe(true);
    expect(refused.payload).toBeNull();
  });

  it("keeps an unlabelled value as today in each of the five existing contexts", () => {
    for (const exportContext of EXPORT_CONTEXTS) {
      const run = redactExportPayload({ project_ref: { data: "INVENTED_RAW_VALUE" } }, { exportContext });
      expect(run.decisions).toEqual([]);
      expect(run.payload).toEqual({ project_ref: { data: "INVENTED_RAW_VALUE" } });
    }
  });
});

// Codex r4: an explicitly stated but unrecognized own label is decided by step 13 at its object,
// so no descendant's label can replace it for the leaves beneath; only an absent label inherits.
const MALFORMED_LABEL_VALUES: unknown[] = [{}, [], { nested: "x" }, [1], 0, 7, 2.5, true, false, "", "bogus_label"];
/** Privacy values the classification decides by step 13 although they are of a recognized form. */
const UNKNOWN_PRIVACY_VALUES: unknown[] = [null, "unknown", "TBD"];
const RECOGNIZED_PRIVACY = ["public", "public_metadata", "invented_public_example", "private_project_data", ...NON_PROJECT_PRIVATE_CLASSES];
const RECOGNIZED_REDISTRIBUTION: unknown[] = ["public_permissive", "invented_non_engineering_example", "private_only", "unknown", "TBD", null];
const RELEASE_OPTIONS = { exportContext: USER_RELEASED_REPORT_CONTEXT, explicitUserReleaseIntent: true } as const;
const WELL_FORMED_CHILD = () => ({ privacy_classification: "private_project_data", redistribution_status: "private_only", value: "INVENTED_VALUE" });

type LabelForm = { label: string; members: Record<string, unknown>; privacy?: unknown; redistribution?: unknown };
const ABSENT_MEMBER = Symbol("absent");

/** One object label in each placement the projector reads: its own members, its provenance's,
 * and `privacy_class` for privacy. */
function labelForms(privacy: unknown, redistribution: unknown): LabelForm[] {
  const own: Record<string, unknown> = {};
  const provenance: Record<string, unknown> = {};
  if (privacy !== ABSENT_MEMBER) {
    own.privacy_classification = privacy;
    provenance.privacy_classification = privacy;
  }
  if (redistribution !== ABSENT_MEMBER) {
    own.redistribution_status = redistribution;
    provenance.redistribution_status = redistribution;
  }
  const forms: LabelForm[] = [{ label: "own", members: own, privacy, redistribution }];
  if (Object.keys(provenance).length > 0) forms.push({ label: "provenance", members: { provenance }, privacy, redistribution });
  if (privacy !== ABSENT_MEMBER) {
    const classed: Record<string, unknown> = { privacy_class: privacy };
    if (redistribution !== ABSENT_MEMBER) classed.redistribution_status = redistribution;
    forms.push({ label: "privacy_class", members: classed, privacy, redistribution });
  }
  return forms;
}

/** The labelled object at each position: the request's top, beneath a labelled project-data
 * parent at depths 0-3 (objects and arrays), and the same in the model. */
function placedObject(object: Record<string, unknown>): Array<{ position: string; request: unknown; model: unknown; path: string }> {
  const placed: Array<{ position: string; request: unknown; model: unknown; path: string }> = [
    { position: "request-top", request: object, model: {}, path: "$.request" },
    { position: "model-top", request: { value: 1 }, model: object, path: "$.model" }
  ];
  for (let depth = 0; depth <= 3; depth += 1) {
    const { payload, childPath } = underLabelledParent(object, depth);
    placed.push({ position: `request-depth-${depth}`, request: payload, model: {}, path: childPath });
    placed.push({
      position: `model-depth-${depth}`,
      request: { value: 1 },
      model: { project: payload },
      path: childPath.replace(/^\$\.request/, "$.model.project")
    });
  }
  return placed;
}

/** Whether the classification of the object alone decides it by step 13 because of the label it
 * states: `USER_RELEASE_VALUE_UNBOUND` for the object, and for every recognized completion of the
 * members it does not state as the classification reads them. */
function classificationUnboundByOwnLabel(object: Record<string, unknown>): boolean {
  if (classifyExportItem(object, RELEASE_OPTIONS).reason_code !== "USER_RELEASE_VALUE_UNBOUND") return false;
  const reads = (key: string) =>
    (object[key] !== undefined && object[key] !== null) ||
    (isRecord(object.provenance) && key in object.provenance) ||
    key in object;
  const privacies = reads("privacy_classification") ? [undefined] : RECOGNIZED_PRIVACY;
  const redistributions = reads("redistribution_status") ? [undefined] : RECOGNIZED_REDISTRIBUTION;
  if (!reads("privacy_classification") && !reads("redistribution_status")) return false;
  return privacies.every((privacy) =>
    redistributions.every((redistribution) => {
      const completed = {
        ...object,
        ...(privacy === undefined ? {} : { privacy_classification: privacy }),
        ...(redistribution === undefined ? {} : { redistribution_status: redistribution })
      };
      return classifyExportItem(completed, RELEASE_OPTIONS).reason_code === "USER_RELEASE_VALUE_UNBOUND";
    })
  );
}

describe("user-released context: a malformed own label blocks at its object (Codex r4, §3.2)", () => {
  it("reproduces Codex's case: an empty structured privacy over a project-data row blocks, in the request and the model", () => {
    for (const privacy of [{}, []]) {
      const malformed = { privacy_classification: privacy, rows: [WELL_FORMED_CHILD()] };
      expect(classifyExportItem(malformed, RELEASE_OPTIONS)).toMatchObject({
        action: "block_export",
        reason_code: "USER_RELEASE_VALUE_UNBOUND"
      });
      for (const [request, model, path] of [
        [malformed, {}, "$.request"],
        [{ value: 1 }, malformed, "$.model"]
      ] as Array<[unknown, unknown, string]>) {
        const projected = controlUserReleasedReportPackageRequest(request, model, true);
        expect(projected.blocked).toBe(true);
        expect(projected.payload).toBeNull();
        expect(projected.summary.materialization_withheld).toBe(true);
        expect(projected.decisions).toContainEqual(
          expect.objectContaining({ path, action: "block_export", reason_code: "USER_RELEASE_VALUE_UNBOUND" })
        );
        expect(projected.findings).toContainEqual(
          expect.objectContaining({ path, code: "USER_RELEASE_VALUE_UNBOUND", severity: "BLOCKING" })
        );
      }
    }
  });

  it("blocks each malformed privacy and redistribution form, own, in provenance and as privacy_class, at depth, in arrays, in the request and the model", () => {
    const malformedCases: Array<{ privacy: unknown; redistribution: unknown }> = [
      ...[...MALFORMED_LABEL_VALUES, ...UNKNOWN_PRIVACY_VALUES].map((privacy) => ({ privacy, redistribution: ABSENT_MEMBER })),
      ...[...MALFORMED_LABEL_VALUES, ...UNKNOWN_PRIVACY_VALUES].map((privacy) => ({ privacy, redistribution: "private_only" })),
      ...MALFORMED_LABEL_VALUES.map((redistribution) => ({ privacy: ABSENT_MEMBER, redistribution })),
      ...MALFORMED_LABEL_VALUES.map((redistribution) => ({ privacy: "private_project_data", redistribution }))
    ];
    let checked = 0;
    for (const { privacy, redistribution } of malformedCases) {
      for (const form of labelForms(privacy, redistribution)) {
        const object = { ...form.members, rows: [WELL_FORMED_CHILD(), { cell: WELL_FORMED_CHILD() }] };
        for (const placed of placedObject(object)) {
          const projected = controlUserReleasedReportPackageRequest(placed.request, placed.model, true);
          const where = `${form.label} ${JSON.stringify(form.members)} ${placed.position}`;
          expect(projected.blocked, where).toBe(true);
          expect(projected.payload, where).toBeNull();
          expect(
            projected.decisions.some(
              (decision) =>
                decision.path === placed.path &&
                decision.action === "block_export" &&
                decision.reason_code === "USER_RELEASE_VALUE_UNBOUND"
            ),
            where
          ).toBe(true);
          checked += 1;
        }
      }
    }
    expect(checked).toBeGreaterThan(800);
  });

  it("records the classification's own step-13 decision of the stated label, where the classification reads it", () => {
    for (const privacy of [...MALFORMED_LABEL_VALUES, ...UNKNOWN_PRIVACY_VALUES]) {
      const object = { privacy_classification: privacy, redistribution_status: "private_only", rows: [WELL_FORMED_CHILD()] };
      const projected = controlUserReleasedReportPackageRequest({ section: object }, {}, true);
      const decision = projected.decisions.find((candidate) => candidate.path === "$.request.section");
      expect(decision, JSON.stringify(privacy)).toBeDefined();
      const { decision_id: _id, ...decided } = decision!;
      const { decision_id: _wantedId, ...wanted } = classifyExportItem(
        { field_id: `${REPORT_PACKAGE_USER_RELEASE_ROUTE_ID}:$.request.section`, field_class: "section", privacy_classification: privacy, redistribution_status: "private_only" },
        { ...RELEASE_OPTIONS, path: "$.request.section" }
      );
      expect(wanted).toMatchObject({ action: "block_export", reason_code: "USER_RELEASE_VALUE_UNBOUND" });
      expect(decided).toEqual(wanted);
    }
  });

  it("still admits a well-formed nested request: absent labels inherit, recognized labels decide", () => {
    const request = {
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      sections: [
        { title: "Invented section", rows: [{ value: 1 }, { value: 2 }] },
        { redistribution_status: "unknown", rows: [WELL_FORMED_CHILD()] },
        { redistribution_status: null, note: "Invented note" },
        { privacy_classification: "private_project_data", note: "Invented note" },
        { provenance: { privacy_classification: "private_project_data", redistribution_status: "TBD" }, note: "Invented note" },
        { privacy_class: "private_project_data", redistribution_status: "private_only", note: "Invented note" },
        {
          privacy_classification: "public_metadata",
          redistribution_status: "public_permissive",
          status: "MECHANICS_SOLVED",
          nested: { privacy_classification: "invented_public_example", redistribution_status: "invented_non_engineering_example", label: "x" }
        }
      ]
    };
    for (const [payload, model] of [
      [request, {}],
      [{ value: 1 }, request],
      [{ wrapper: [{ inner: request }] }, { project: [request] }]
    ]) {
      const projected = controlUserReleasedReportPackageRequest(payload, model, true);
      expect(projected.blocked).toBe(false);
      expect(projected.payload).toEqual(payload);
      expect(projected.decisions.some((decision) => decision.reason_code === "USER_RELEASE_VALUE_UNBOUND")).toBe(false);
      expect(controlUserReleasedReportPackageRequest(payload, model, false).blocked).toBe(true);
    }
  });

  it("generated differential: every object the classification decides by step 13 for its own label blocks the projector", () => {
    const privacies: unknown[] = [ABSENT_MEMBER, ...RECOGNIZED_PRIVACY, "protected_suspected", ...UNKNOWN_PRIVACY_VALUES, ...MALFORMED_LABEL_VALUES];
    const redistributions: unknown[] = [ABSENT_MEMBER, ...RECOGNIZED_REDISTRIBUTION, "protected_suspected", ...MALFORMED_LABEL_VALUES];
    let unboundObjects = 0;
    let admittedWellFormed = 0;
    let projections = 0;
    for (const privacy of privacies) {
      for (const redistribution of redistributions) {
        for (const form of labelForms(privacy, redistribution)) {
          const object = { ...form.members, rows: [WELL_FORMED_CHILD()] };
          const unbound = form.label !== "privacy_class" && classificationUnboundByOwnLabel(object);
          // Well formed: a recognized project label, or an absent one, over the project-data row.
          const wellFormed =
            (privacy === ABSENT_MEMBER || privacy === "private_project_data") &&
            (redistribution === ABSENT_MEMBER || RECOGNIZED_REDISTRIBUTION.includes(redistribution)) &&
            !(privacy === ABSENT_MEMBER && redistribution === ABSENT_MEMBER && form.label !== "own");
          if (unbound) unboundObjects += 1;
          for (const placed of placedObject(object)) {
            const projected = controlUserReleasedReportPackageRequest(placed.request, placed.model, true);
            projections += 1;
            const where = `${form.label} ${JSON.stringify(form.members)} ${placed.position}`;
            if (unbound) {
              expect(projected.blocked, where).toBe(true);
              expect(
                projected.decisions.some(
                  (decision) => decision.path === placed.path && decision.reason_code === "USER_RELEASE_VALUE_UNBOUND"
                ),
                where
              ).toBe(true);
            }
            if (wellFormed) {
              expect(projected.blocked, where).toBe(false);
              admittedWellFormed += 1;
            }
          }
        }
      }
    }
    expect(unboundObjects).toBeGreaterThan(300);
    expect(admittedWellFormed).toBeGreaterThan(100);
    expect(projections).toBeGreaterThan(5000);
  });
});
