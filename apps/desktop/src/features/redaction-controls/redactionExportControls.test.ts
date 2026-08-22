// TP-E4-REDACTION-001: the app-side TypeScript mirror of the DEL-12-02 core
// redaction contract must match the python contract decision-for-decision.
// Parity is pinned by the shared invented corpus
// `fixtures/redaction_export_controls/cases.json`, which
// `tests/security/test_redaction_export_controls.py` asserts against
// `core/security/redaction/controls.py`. All fixture data is invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  REDACTED_VALUE,
  classifyExportItem,
  controlRouteExport,
  redactExportPayload,
  type RedactionAction,
  type RedactionExportContext,
  type RedactionFindingClass,
  type RedactionFindingSeverity,
  type RedactionReasonCode
} from "./redactionExportControls";

type ParityCase = {
  case_id: string;
  item: Record<string, unknown>;
  export_context: RedactionExportContext;
  explicit_local_private_intent: boolean;
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
        explicitLocalPrivateIntent: parityCase.explicit_local_private_intent
      });
      expect(parityCase.item).toEqual(originalItem);
      expect(decision.action).toBe(parityCase.expected.action);
      expect(decision.reason_code).toBe(parityCase.expected.reason_code);
      expect(decision.source_metadata_present).toBe(parityCase.expected.source_metadata_present);

      const run = redactExportPayload(
        { item: parityCase.item },
        {
          exportContext: parityCase.export_context,
          explicitLocalPrivateIntent: parityCase.explicit_local_private_intent
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
