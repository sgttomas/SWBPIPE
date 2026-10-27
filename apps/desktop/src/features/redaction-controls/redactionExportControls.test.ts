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
    const run = redactExportPayload(itemsPayload, {
      exportContext: USER_RELEASED_REPORT_CONTEXT,
      explicitLocalPrivateIntent: true
    });
    expect(run.blocked).toBe(true);
    expect(run.decisions).toHaveLength(2);
    expect(run.decisions.every((decision) => decision.reason_code === "USER_RELEASE_INTENT_REQUIRED")).toBe(true);

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
      for (const explicitUserReleaseIntent of [undefined, false, true]) {
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
    const nested = admitted.decisions.find((decision) => decision.path.endsWith("nested.__route_key__value"));
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
    const inner = projected.decisions.find((decision) => decision.path.endsWith("__route_key__note"));
    expect(inner).toMatchObject({ review_status: "quarantined", reason_code: "PROTECTED_CONTENT_BLOCKED" });
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
