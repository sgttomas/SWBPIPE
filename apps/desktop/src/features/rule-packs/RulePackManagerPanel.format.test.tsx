import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import type {
  LocalRulePackEnvelope,
  LocalRulePackIndexEntry,
  RulePackDocumentValidation,
  RulePackValidationFinding
} from "../../services/rulePackService";
import { RulePackManagerPanel } from "./RulePackManagerPanel";

// W-17 rule-pack format 0.5.0 (DEL-06-01 Design `rule-pack-0.5.0.md` §2.1,
// §2.9, §3): validation shows the format findings by code, and a stored pack in
// another format opens with the refusal shown and offers delete only. jsdom has
// no Tauri runtime, so the store and validate routes are stubbed with
// backend-shaped responses; every id and text below is invented.

const backend = vi.hoisted(() => ({
  validation: null as unknown as RulePackDocumentValidation,
  entries: [] as LocalRulePackIndexEntry[],
  envelope: null as unknown as LocalRulePackEnvelope,
  deleted: [] as string[],
  deletedTargets: [] as Array<{ projectId: string; rulePackId: string }>
}));

vi.mock("../../services/rulePackService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/rulePackService")>();
  return {
    ...actual,
    validateRulePack: vi.fn(async () => ({ route: "tauri_backend", validation: backend.validation })),
    listLocalRulePacks: vi.fn(async (projectId: string | null) => ({
      route: "tauri_backend",
      entries: backend.entries.filter((entry) => projectId !== null && entry.project_id === projectId)
    })),
    openLocalRulePack: vi.fn(async () => ({ route: "tauri_backend", envelope: backend.envelope })),
    saveLocalRulePack: vi.fn(async (projectId: string, document: Record<string, unknown>) => {
      const metadata = (document.metadata ?? {}) as Record<string, unknown>;
      const rulePackId =
        typeof metadata.rule_pack_id === "string" ? metadata.rule_pack_id : "private_draft_rule_pack";
      const rulePackName = typeof metadata.name === "string" ? metadata.name : "Private Draft Rule Pack";
      const rulePackVersion = typeof metadata.rule_pack_version === "string" ? metadata.rule_pack_version : "0.1.0";
      const lifecycle = typeof metadata.lifecycle_status === "string" ? metadata.lifecycle_status : "draft";
      const entry: LocalRulePackIndexEntry = {
        project_id: projectId,
        rule_pack_id: rulePackId,
        rule_pack_name: rulePackName,
        rule_pack_version: rulePackVersion,
        lifecycle_status: lifecycle,
        privacy_class: "private_user_data",
        storage_mode: "local_sqlite",
        created_at_unix: 1,
        updated_at_unix: 2
      };
      backend.entries = [
        ...backend.entries.filter((item) => !(item.project_id === projectId && item.rule_pack_id === rulePackId)),
        entry
      ];
      return {
        route: "tauri_backend",
        envelope: {
          project_id: projectId,
          rule_pack_id: rulePackId,
          storage_mode: "local_sqlite",
          created_at_unix: 1,
          updated_at_unix: 2,
          document,
          validation: backend.validation,
          message: "Saved."
        }
      };
    }),
    deleteLocalRulePack: vi.fn(async (projectId: string, rulePackId: string) => {
      backend.deleted.push(rulePackId);
      backend.deletedTargets.push({ projectId, rulePackId });
      backend.entries = backend.entries.filter(
        (entry) => !(entry.project_id === projectId && entry.rule_pack_id === rulePackId)
      );
      return {
        route: "tauri_backend",
        receipt: { project_id: projectId, rule_pack_id: rulePackId, deleted: true, message: `Deleted ${rulePackId}.` }
      };
    })
  };
});

const finding = (code: string, subject: string, message: string): RulePackValidationFinding => ({
  code,
  subject,
  message,
  blocking: true
});

function validationWith(findings: RulePackValidationFinding[], schemaVersion = "0.5.0"): RulePackDocumentValidation {
  return {
    document_kind: "openpipestress.rule_pack.document_validation",
    rule_pack_id: "invented_pack",
    rule_pack_name: "Invented",
    rule_pack_version: "0.1.0",
    schema_version: schemaVersion,
    grammar_version: "1.0.0",
    grammar_version_supported: true,
    supported_grammar_versions: ["1.0.0"],
    evaluator_grammar_version: "1.0.0",
    document_findings: findings,
    lifecycle_findings: [],
    lifecycle_blocked: false,
    formulas: [],
    unit_check: { status: "consistent", checks: [], declaration_findings: [] },
    checksum: { declared_value: null, computed: null, match_status: "not_computable" },
    publicly_exportable: false,
    professional_boundary_notice: "notice",
    has_blocking_findings: findings.length > 0
  };
}

const modelStub = {
  project: { id: "project:invented-format", name: "Invented Format Project" }
} as unknown as PreviewModel;

const legacyEntry: LocalRulePackIndexEntry = {
  project_id: "project:invented-format",
  rule_pack_id: "invented_legacy_pack",
  rule_pack_name: "Invented legacy pack",
  rule_pack_version: "0.1.0",
  lifecycle_status: "draft",
  privacy_class: "private_user_data",
  storage_mode: "local_sqlite",
  created_at_unix: 1,
  updated_at_unix: 1
};

beforeEach(() => {
  backend.deleted = [];
  backend.deletedTargets = [];
});

afterEach(cleanup);

describe("RulePackManagerPanel format 0.5.0 validation", () => {
  it("shows each format finding by its code", async () => {
    backend.validation = validationWith([
      finding("RULE_PACK_REFERENCE_UNRESOLVED", "load_case_mappings[0].category_ref", "names no declared stress category"),
      finding("RULE_PACK_DUPLICATE_ID", "required_load_cases[1].basis_ref", "two required load cases name the same load_case 'load:L-100'"),
      finding("RULE_PACK_MAPPING_INCOMPLETE", "user_required_load_case_2", "required load case is reached by no check"),
      finding("RULE_SOLVER_RESULT_REF_MISSING", "required_inputs[0] (actual)", "the solver_result input carries no solver_result_ref selector")
    ]);
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    await waitFor(() => expect(screen.getByTestId("rule-pack-validation")).toBeTruthy());
    const codes = Array.from(screen.getByTestId("rule-pack-validation-findings").querySelectorAll("strong")).map((node) => node.textContent);
    expect(codes).toEqual([
      "RULE_PACK_REFERENCE_UNRESOLVED",
      "RULE_PACK_DUPLICATE_ID",
      "RULE_PACK_MAPPING_INCOMPLETE",
      "RULE_SOLVER_RESULT_REF_MISSING"
    ]);
    expect(screen.getByTestId("rule-pack-validation-summary").textContent).toContain("blocking_findings=true");
    // A draft (not a stored pack) keeps its editors; no delete-only refusal.
    expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull();
    expect(screen.getByTestId("rule-pack-load-case-categories-editor")).toBeTruthy();
  });

  it("opens a stored 0.4.0 pack with the refusal shown and offers delete only", async () => {
    backend.entries = [legacyEntry];
    backend.validation = validationWith(
      [finding("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED", "schema_version", "rule-pack format '0.4.0' (metadata.schema_version '0.4.0') is not supported; the supported format is '0.5.0'.")],
      "0.4.0"
    );
    backend.envelope = {
      project_id: legacyEntry.project_id,
      rule_pack_id: legacyEntry.rule_pack_id,
      storage_mode: "local_sqlite",
      created_at_unix: 1,
      updated_at_unix: 1,
      document: { schema_version: "0.4.0", metadata: { rule_pack_id: legacyEntry.rule_pack_id, schema_version: "0.4.0" } },
      validation: backend.validation,
      message: "Opened invented_legacy_pack."
    };
    render(<RulePackManagerPanel model={modelStub} />);
    // The list is read when the page opens. A stored 0.4.0 pack stays listed.
    await screen.findByTestId("rule-pack-entry-invented_legacy_pack");
    fireEvent.click(screen.getByTestId("rule-pack-open-invented_legacy_pack"));

    const refusal = await screen.findByTestId("rule-pack-version-refusal");
    expect(refusal.querySelector("[title]")?.getAttribute("title")).toBe("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED");
    expect(refusal.textContent).toContain("format '0.4.0'");
    expect(refusal.textContent).toContain("reads rule-pack format 0.5.0 only");
    expect(refusal.textContent).toContain("can only be deleted");
    expect(screen.getByTestId("rule-pack-validation-findings").textContent).toContain("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED");
    // Delete only: no editors, and the document actions are disabled.
    expect(screen.queryByTestId("rule-pack-load-case-categories-editor")).toBeNull();
    expect(screen.queryByTestId("rule-pack-declarations-editor")).toBeNull();
    for (const testId of ["rule-pack-validate", "rule-pack-compute-checksum", "rule-pack-save", "rule-pack-draft-json"]) {
      expect(screen.getByTestId(testId)).toHaveProperty("disabled", true);
    }

    fireEvent.click(screen.getByTestId("rule-pack-refused-delete"));
    await waitFor(() => expect(backend.deleted).toEqual(["invented_legacy_pack"]));
    await waitFor(() => expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull());
    expect(screen.queryByTestId("rule-pack-draft-json")).toBeNull();
    expect(screen.queryByTestId("rule-pack-entry-invented_legacy_pack")).toBeNull();
    expect(screen.getByTestId("rule-pack-action-status").textContent).toContain("Deleted Invented legacy pack.");
  });

  it("returns to authoring when a new draft replaces the refused stored pack", async () => {
    backend.entries = [legacyEntry];
    backend.validation = validationWith([finding("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED", "schema_version", "format '0.4.0' is not supported")], "0.4.0");
    backend.envelope = { ...backend.envelope, validation: backend.validation };
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_legacy_pack"));
    await screen.findByTestId("rule-pack-version-refusal");
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull();
    expect(screen.getByTestId("rule-pack-load-case-categories-editor")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-validate")).toHaveProperty("disabled", false);
  });

  it("clears a refused stored pack when switching to another project or closing the project so it cannot be deleted or edited across projects", async () => {
    const projectB = {
      project: { id: "project:invented-other-format", name: "Other Format Project" }
    } as unknown as PreviewModel;
    const projectBEntry: LocalRulePackIndexEntry = {
      ...legacyEntry,
      project_id: "project:invented-other-format",
      rule_pack_id: "invented_project_b_pack",
      rule_pack_name: "Project B pack"
    };
    backend.entries = [legacyEntry, projectBEntry];
    backend.validation = validationWith([finding("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED", "schema_version", "format '0.4.0' is not supported")], "0.4.0");
    backend.envelope = {
      project_id: legacyEntry.project_id,
      rule_pack_id: legacyEntry.rule_pack_id,
      storage_mode: "local_sqlite",
      created_at_unix: 1,
      updated_at_unix: 1,
      document: { schema_version: "0.4.0", metadata: { rule_pack_id: legacyEntry.rule_pack_id, schema_version: "0.4.0" } },
      validation: backend.validation,
      message: "Opened invented_legacy_pack."
    };

    const { rerender } = render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_legacy_pack"));
    await screen.findByTestId("rule-pack-version-refusal");
    expect(screen.getByTestId("rule-pack-refused-delete")).toBeTruthy();

    rerender(<RulePackManagerPanel model={projectB} />);
    await screen.findByTestId("rule-pack-entry-invented_project_b_pack");
    expect(screen.queryByTestId("rule-pack-entry-invented_legacy_pack")).toBeNull();
    expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull();
    expect(screen.queryByTestId("rule-pack-refused-delete")).toBeNull();
    expect(screen.queryByTestId("rule-pack-open-pack")).toBeNull();
    expect(screen.queryByTestId("rule-pack-draft-json")).toBeNull();
    expect(screen.queryByTestId("rule-pack-load-case-categories-editor")).toBeNull();
    expect(backend.deletedTargets).toEqual([]);
    expect(backend.entries.some((entry) => entry.project_id === legacyEntry.project_id && entry.rule_pack_id === legacyEntry.rule_pack_id)).toBe(true);

    // Switching back to Project A and then to no project clears a newly opened refused pack too.
    rerender(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_legacy_pack"));
    await screen.findByTestId("rule-pack-version-refusal");

    rerender(<RulePackManagerPanel model={null} />);
    await waitFor(() => expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull());
    expect(screen.queryByTestId("rule-pack-refused-delete")).toBeNull();
    expect(screen.queryByTestId("rule-pack-open-pack")).toBeNull();
    expect(backend.deletedTargets).toEqual([]);
  });

  it("keeps an edited draft repairable when Check finds an unsupported schema version on a saved 0.5.0 pack", async () => {
    const projectB = {
      project: { id: "project:invented-other-format", name: "Other Format Project" }
    } as unknown as PreviewModel;
    backend.entries = [];
    backend.validation = validationWith([], "0.5.0");

    const { rerender } = render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    const savedRow = await screen.findByTestId("rule-pack-entry-private_draft_rule_pack");
    expect(savedRow.textContent).toContain("No faults found.");

    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    const edited = JSON.parse(textarea.value) as Record<string, unknown>;
    edited.schema_version = "0.4.0";
    (edited.metadata as Record<string, unknown>).schema_version = "0.4.0";
    fireEvent.change(textarea, { target: { value: JSON.stringify(edited, null, 2) } });

    backend.validation = validationWith(
      [finding("RULE_PACK_SCHEMA_VERSION_UNSUPPORTED", "schema_version", "format '0.4.0' is not supported")],
      "0.4.0"
    );
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    const words = await screen.findByTestId("rule-pack-validation-words");
    expect(words.textContent).toContain("format '0.4.0' is not supported");

    // The refusal only applies to a stored pack; the unsaved edit keeps all editors enabled and leaves the stored 0.5.0 copy untouched.
    expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull();
    expect(screen.queryByTestId("rule-pack-refused-delete")).toBeNull();
    expect(screen.getByTestId("rule-pack-load-case-categories-editor")).toBeTruthy();
    for (const testId of ["rule-pack-validate", "rule-pack-compute-checksum", "rule-pack-save", "rule-pack-draft-json"]) {
      expect(screen.getByTestId(testId)).toHaveProperty("disabled", false);
    }
    expect(backend.deletedTargets).toEqual([]);
    expect(backend.entries.some((entry) => entry.project_id === "project:invented-format" && entry.rule_pack_id === "private_draft_rule_pack")).toBe(true);

    // Switching projects preserves the unsaved edited draft instead of treating it as a refused stored pack.
    rerender(<RulePackManagerPanel model={projectB} />);
    await waitFor(() => expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("not saved"));
    expect(screen.getByTestId("rule-pack-open-pack")).toBeTruthy();
    expect((screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement).value).toContain('"schema_version": "0.4.0"');
    expect(screen.queryByTestId("rule-pack-version-refusal")).toBeNull();
    expect(backend.deletedTargets).toEqual([]);
  });
});
