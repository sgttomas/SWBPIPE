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
