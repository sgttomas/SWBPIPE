import { StrictMode } from "react";
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import {
  buildDraftRulePackDocument,
  type LocalRulePackIndexEntry,
  type RulePackDocument,
  type RulePackDocumentValidation,
  type RulePackValidationFinding
} from "../../services/rulePackService";
import { RulePackManagerPanel, faultsOf, partOfSubject } from "./RulePackManagerPanel";
import { useReturnCount } from "./useReturnCount";
import { DormantSection } from "../workspace/dormantSection";
import { COMPLETENESS_STATUSES, LIBRARY_KINDS, REQUIRED_FOR_TARGETS, SLOT_KINDS, SOURCE_KINDS, VALUE_STATUSES } from "./DeclarationsEditor";
import {
  REPORTED_STATUSES,
  RUNTIME_POLICY_FIELDS,
  referenceNames,
  ACCEPTABILITY_BASES,
  ACCEPTABILITY_RELATIONS,
  ANALYSIS_STATUSES,
  DIAGNOSTIC_CODES,
  DIAGNOSTIC_POLICY_FIELDS
} from "./CheckDefinitionsEditor";
import {
  AGGREGATE_FUNCTIONS,
  BINARY_OPERATORS,
  COMPARE_OPERATORS,
  DIMENSIONS,
  EDITABLE_NODE_TYPES,
  LOGICAL_OPERATORS,
  LOOKUP_MODES,
  UNARY_OPERATORS,
  bindableVariables,
  collectRulePackVariables
} from "./ExpressionComposer";
import {
  ACCEPTABILITY_BASIS_WORDS,
  ACCEPTABILITY_RELATION_WORDS,
  ANALYSIS_STATUS_WORDS,
  CHECK_GROUP_WORDS,
  COMPLETENESS_WORDS,
  DIAGNOSTIC_CODE_WORDS,
  DIAGNOSTIC_POLICY_FIELD_WORDS,
  DIMENSION_WORDS,
  LIBRARY_KIND_WORDS,
  NODE_KIND_WORDS,
  OPERATOR_WORDS,
  PAGE_WORDS,
  PART_WORDS,
  REQUIRED_FOR_WORDS,
  SLOT_KIND_WORDS,
  SOURCE_KIND_WORDS,
  VALUE_STATUS_WORDS,
  fieldWord,
  type Words
} from "./rulePackWords";

// The Rules page as a person meets it: the project's packs read when the page
// opens, one button to start a pack, an open pack's parts as closed lines with
// their summaries and any fault's words, and every label and option in the
// product's words, read from the one words module (rulePackWords.ts). The
// store and backend routes are stubbed with backend-shaped responses; every id
// and text below is invented.

const backend = vi.hoisted(() => ({
  // The store: the list returns the entries of the project it is asked for.
  entries: [] as LocalRulePackIndexEntry[],
  listCalls: 0,
  listProjects: [] as Array<string | null>,
  // The next list reads that fail, in order.
  listFailures: 0,
  // The next list reads held open until the test releases them, in order.
  heldLists: 0,
  releaseLists: [] as Array<() => void>,
  validation: null as unknown as RulePackDocumentValidation,
  checksumDocuments: [] as unknown[],
  // A save held open until the test releases it.
  releaseSave: null as null | (() => void),
  // When true, saveLocalRulePack adds/updates the entry in backend.entries on resolve.
  upsertOnSave: false,
  deleted: [] as Array<{ projectId: string; rulePackId: string }>,
  openEnvelope: null as unknown
}));

vi.mock("../../services/rulePackService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/rulePackService")>();
  return {
    ...actual,
    listLocalRulePacks: vi.fn(async (projectId: string | null) => {
      backend.listCalls += 1;
      backend.listProjects.push(projectId);
      if (backend.listFailures > 0) {
        backend.listFailures -= 1;
        throw new Error("invented store failure");
      }
      if (backend.heldLists > 0) {
        backend.heldLists -= 1;
        await new Promise<void>((resolve) => backend.releaseLists.push(resolve));
      }
      const entries = backend.entries.filter((entry) => projectId !== null && entry.project_id === projectId);
      return { route: "tauri_backend", entries };
    }),
    openLocalRulePack: vi.fn(async () => ({ route: "tauri_backend", envelope: backend.openEnvelope })),
    deleteLocalRulePack: vi.fn(async (projectId: string, rulePackId: string) => {
      backend.deleted.push({ projectId, rulePackId });
      backend.entries = backend.entries.filter(
        (entry) => !(entry.project_id === projectId && entry.rule_pack_id === rulePackId)
      );
      return {
        route: "tauri_backend",
        receipt: { project_id: projectId, rule_pack_id: rulePackId, deleted: true, message: `Deleted ${rulePackId}.` }
      };
    }),
    saveLocalRulePack: vi.fn(
      (projectId: string, document: Record<string, unknown>) =>
        new Promise((resolve) => {
          const metadata = (document.metadata ?? {}) as Record<string, unknown>;
          const rulePackId =
            typeof metadata.rule_pack_id === "string" ? metadata.rule_pack_id : "private_draft_rule_pack";
          const rulePackName = typeof metadata.name === "string" ? metadata.name : "Private Draft Rule Pack";
          const rulePackVersion = typeof metadata.rule_pack_version === "string" ? metadata.rule_pack_version : "0.1.0";
          const lifecycle = typeof metadata.lifecycle_status === "string" ? metadata.lifecycle_status : "draft";
          backend.releaseSave = () => {
            if (backend.upsertOnSave) {
              const nextEntry: LocalRulePackIndexEntry = {
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
                ...backend.entries.filter((entry) => !(entry.project_id === projectId && entry.rule_pack_id === rulePackId)),
                nextEntry
              ];
            }
            resolve({
              route: "tauri_backend",
              envelope: {
                project_id: projectId,
                rule_pack_id: rulePackId,
                storage_mode: "local_sqlite",
                created_at_unix: 1,
                updated_at_unix: 1,
                document,
                validation: backend.validation,
                message: "Saved."
              }
            });
          };
        })
    ),
    validateRulePack: vi.fn(async () => ({ route: "tauri_backend", validation: backend.validation })),
    computeRulePackChecksum: vi.fn(async (document: unknown) => {
      backend.checksumDocuments.push(document);
      return {
        route: "tauri_backend",
        computed: {
          algorithm: "sha256",
          canonicalization: "JCS",
          payload_scope: "rule_pack_payload",
          payload_ref: "private_draft_rule_pack",
          payload_excludes: ["checksums"],
          basis: "invented_basis",
          grammar_version: "1.0.0",
          grammar_version_bound: true,
          value: "0".repeat(64)
        }
      };
    })
  };
});

const modelStub = {
  project: { id: "project:invented-page", name: "Invented Page Project" },
  rule_pack_refs: []
} as unknown as PreviewModel;

const savedEntry: LocalRulePackIndexEntry = {
  project_id: "project:invented-page",
  rule_pack_id: "invented_saved_pack",
  rule_pack_name: "Invented saved pack",
  rule_pack_version: "0.2.0",
  lifecycle_status: "review_pending",
  privacy_class: "private_user_data",
  storage_mode: "local_sqlite",
  created_at_unix: 1,
  updated_at_unix: 1
};

function validationWith(findings: RulePackValidationFinding[]): RulePackDocumentValidation {
  return {
    document_kind: "openpipestress.rule_pack.document_validation",
    rule_pack_id: "private_draft_rule_pack",
    rule_pack_name: "Private Draft Rule Pack",
    rule_pack_version: "0.1.0",
    schema_version: "0.5.0",
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
    has_blocking_findings: findings.some((finding) => finding.blocking)
  };
}

const PART_KEYS = ["loadCases", "categories", "inputs", "formula", "checks"] as const;

function part(key: (typeof PART_KEYS)[number]): HTMLDetailsElement {
  return screen.getByTestId(`rule-pack-part-${key}`) as HTMLDetailsElement;
}

function openPart(key: (typeof PART_KEYS)[number]): HTMLDetailsElement {
  const element = part(key);
  element.open = true;
  return element;
}

function optionTexts(testId: string, scope: HTMLElement | Document = document): string[] {
  const select = within(scope as HTMLElement).getAllByTestId(testId)[0] as HTMLSelectElement;
  return Array.from(select.options).map((option) => option.textContent ?? "");
}

function words(tokens: readonly string[], table: Words): string[] {
  return tokens.map((token) => table[token]);
}

beforeEach(() => {
  backend.entries = [savedEntry];
  backend.listProjects = [];
  backend.listFailures = 0;
  backend.heldLists = 0;
  backend.releaseLists = [];
  backend.releaseSave = null;
  backend.upsertOnSave = false;
  backend.deleted = [];
  backend.openEnvelope = null;
  backend.listCalls = 0;
  backend.validation = validationWith([]);
  backend.checksumDocuments = [];
});

afterEach(cleanup);

describe("the Rules page with no pack open", () => {
  it("reads the project's packs without a press and shows the list, the start button and no editor field", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    const row = await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    expect(backend.listCalls).toBe(1);
    expect(screen.queryByTestId("rule-pack-refresh-list")).toBeNull();
    expect(row.textContent).toContain("Invented saved pack");
    expect(row.textContent).toContain("version 0.2.0");
    expect(row.textContent).toContain("awaiting review");
    expect(row.textContent).not.toContain("invented_saved_pack");
    expect(screen.getByTestId("rule-pack-open-invented_saved_pack").textContent).toBe(PAGE_WORDS.open);
    expect(screen.getByTestId("rule-pack-delete-invented_saved_pack").textContent).toBe(PAGE_WORDS.delete);
    expect(screen.getByTestId("rule-pack-new-draft").textContent).toBe(PAGE_WORDS.newPack);
    expect(screen.getByTestId("rule-pack-scope-status").textContent).toBe(PAGE_WORDS.scope);

    const panel = screen.getByTestId("rule-pack-manager-panel");
    expect(panel.querySelectorAll("input, select, textarea")).toHaveLength(0);
    expect(screen.queryByTestId("rule-pack-validate")).toBeNull();
    expect(screen.queryByTestId("rule-pack-parts")).toBeNull();
  });

  it("says in one line when the project has no saved pack", async () => {
    backend.entries = [];
    render(<RulePackManagerPanel model={modelStub} />);
    expect((await screen.findByTestId("rule-pack-list-status")).textContent).toBe(PAGE_WORDS.listEmpty);
  });
});

describe("an open pack", () => {
  it("shows its name and version, the save controls in order, and its parts closed with their summaries", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));

    const header = screen.getByTestId("rule-pack-header");
    expect(header.textContent).toContain("Private Draft Rule Pack");
    expect(header.textContent).toContain("version 0.1.0");
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe(PAGE_WORDS.notSaved);

    const controls = ["rule-pack-validate", "rule-pack-compute-checksum", "rule-pack-save", "rule-pack-discard-draft"].map(
      (testId) => screen.getByTestId(testId)
    );
    expect(controls.map((control) => control.textContent)).toEqual([
      PAGE_WORDS.validate,
      PAGE_WORDS.checksum,
      PAGE_WORDS.save,
      PAGE_WORDS.discard
    ]);
    // The controls come straight after the name, before the parts.
    expect(header.compareDocumentPosition(controls[0]) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(controls[3].compareDocumentPosition(part("loadCases")) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();

    for (const key of PART_KEYS) expect(part(key).open).toBe(false);
    const summary = (key: (typeof PART_KEYS)[number]) => screen.getByTestId(`rule-pack-part-summary-${key}`).textContent;
    expect(part("loadCases").querySelector("summary")?.textContent).toContain(PART_WORDS.loadCases);
    expect(summary("loadCases")).toBe("1");
    expect(summary("categories")).toBe("1");
    // One input and one limit, both stated incomplete by the template.
    expect(summary("inputs")).toBe(`2, 2 ${PART_WORDS.markedIncomplete}`);
    expect(summary("formula")).toBe("user_required_input_1");
    expect(summary("checks")).toBe("1");

    // The document and the codes are closed disclosures.
    expect((screen.getByTestId("rule-pack-document") as HTMLDetailsElement).open).toBe(false);
    expect((screen.getByTestId("rule-pack-details") as HTMLDetailsElement).open).toBe(false);
  });

  it("shows a fault's words on its part's closed line, and only while the pack reads as it was checked", async () => {
    backend.validation = validationWith([
      { code: "RULE_PACK_FIELD_REQUIRED", subject: "required_inputs[0].name", message: "the input needs a name", blocking: true },
      { code: "RULE_PACK_FIELD_REQUIRED", subject: "value_slots[0]", message: "the limit needs a kind", blocking: true },
      { code: "RULE_PACK_REFERENCE_UNRESOLVED", subject: "user_check_1", message: "the check names no formula", blocking: true },
      { code: "RULE_PACK_NOTE", subject: "required_load_cases[0]", message: "a warning only", blocking: false }
    ]);
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-validate"));

    const inputsFault = await screen.findByTestId("rule-pack-part-fault-inputs");
    expect(part("inputs").open).toBe(false);
    expect(inputsFault.textContent).toBe("the input needs a name (and 1 more)");
    expect(screen.getByTestId("rule-pack-part-fault-checks").textContent).toBe("the check names no formula");
    // A non-blocking finding is not a fault on the line.
    expect(screen.queryByTestId("rule-pack-part-fault-loadCases")).toBeNull();
    // Beside the save controls, in words.
    expect(screen.getByTestId("rule-pack-validation-words").textContent).toContain("3 things to fix");
    expect(screen.getByTestId("rule-pack-validation-words").textContent).toContain("the limit needs a kind");

    // An edit after the check: the closed lines no longer claim its faults.
    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    expect(screen.queryByTestId("rule-pack-part-fault-inputs")).toBeNull();
    expect(screen.getByTestId("rule-pack-validation-words").textContent).toContain(PAGE_WORDS.staleFindings);
  });

  it("reaches every existing control by its test id once its part is opened", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    const controlsByPart: Record<(typeof PART_KEYS)[number], string[]> = {
      loadCases: [
        "rule-pack-load-case-name", "rule-pack-load-case-description", "rule-pack-load-case-basis-type",
        "rule-pack-load-case-basis-id", "rule-pack-load-case-id", "rule-pack-load-case-remove", "rule-pack-load-case-add"
      ],
      categories: [
        "rule-pack-category-name", "rule-pack-category-description", "rule-pack-category-mapping-user_required_load_case_1",
        "rule-pack-category-id", "rule-pack-category-remove", "rule-pack-category-add"
      ],
      inputs: [
        "rule-pack-input-name", "rule-pack-input-source-kind", "rule-pack-input-dimension", "rule-pack-input-unit",
        "rule-pack-input-id", "rule-pack-input-required-for", "rule-pack-input-completeness", "rule-pack-input-remove",
        "rule-pack-input-add", "rule-pack-slot-id", "rule-pack-slot-kind", "rule-pack-slot-dimension", "rule-pack-slot-unit",
        "rule-pack-slot-value-status", "rule-pack-slot-required-for", "rule-pack-slot-completeness", "rule-pack-slot-remove",
        "rule-pack-slot-add", "rule-pack-declarations-unit-policy"
      ],
      formula: [
        "rule-pack-formula-select", "rule-pack-node-type", "rule-pack-variable-ref", "rule-pack-expression-text-preview",
        "rule-pack-variable-browser", "rule-pack-expression-unit-policy"
      ],
      checks: [
        "rule-pack-check-name", "rule-pack-check-description", "rule-pack-check-category-ref", "rule-pack-check-formula-ref",
        "rule-pack-check-input-ref", "rule-pack-check-input-ref-remove", "rule-pack-check-input-ref-add",
        "rule-pack-check-slot-ref", "rule-pack-check-slot-ref-remove", "rule-pack-check-slot-ref-add",
        "rule-pack-check-acceptability-relation", "rule-pack-check-id", "rule-pack-check-acceptability-basis",
        ...["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"].map(
          (status) => `rule-pack-check-status-${status}`
        ),
        ...DIAGNOSTIC_POLICY_FIELDS.map((field) => `rule-pack-check-policy-${field}`),
        "rule-pack-check-remove", "rule-pack-check-add"
      ]
    };
    for (const key of PART_KEYS) {
      const element = openPart(key);
      for (const testId of controlsByPart[key]) {
        expect(within(element).getAllByTestId(testId).length, `${testId} in ${key}`).toBeGreaterThan(0);
      }
    }
    // The document and the save controls keep theirs too.
    for (const testId of ["rule-pack-draft-json", "rule-pack-validate", "rule-pack-compute-checksum", "rule-pack-save", "rule-pack-discard-draft", "rule-pack-boundary-note"]) {
      expect(screen.getByTestId(testId)).toBeTruthy();
    }
  });

  it("places each finding in the part its subject names", () => {
    const document = buildDraftRulePackDocument();
    expect(partOfSubject("required_load_cases[1].basis_ref", document)).toBe("loadCases");
    expect(partOfSubject("load_case_mappings[0].category_ref", document)).toBe("categories");
    expect(partOfSubject("required_inputs[0] (actual)", document)).toBe("inputs");
    expect(partOfSubject("user_limit_slot_1", document)).toBe("inputs");
    expect(partOfSubject("formula_declarations[0].expression_ast", document)).toBe("formula");
    expect(partOfSubject("user_check_1", document)).toBe("checks");
    expect(partOfSubject("grammar_version", document)).toBeNull();
  });
});

describe("the words of the page, read from the one words module", () => {
  it("has a word for every token of every vocabulary the editors offer", () => {
    const vocabularies: Array<[readonly string[], Words]> = [
      [SOURCE_KINDS, SOURCE_KIND_WORDS],
      [REQUIRED_FOR_TARGETS, REQUIRED_FOR_WORDS],
      [COMPLETENESS_STATUSES, COMPLETENESS_WORDS],
      [SLOT_KINDS, SLOT_KIND_WORDS],
      [VALUE_STATUSES, VALUE_STATUS_WORDS],
      [LIBRARY_KINDS, LIBRARY_KIND_WORDS],
      [ACCEPTABILITY_BASES, ACCEPTABILITY_BASIS_WORDS],
      [ACCEPTABILITY_RELATIONS, ACCEPTABILITY_RELATION_WORDS],
      [ANALYSIS_STATUSES, ANALYSIS_STATUS_WORDS],
      [DIAGNOSTIC_CODES, DIAGNOSTIC_CODE_WORDS],
      [DIAGNOSTIC_POLICY_FIELDS, DIAGNOSTIC_POLICY_FIELD_WORDS],
      [DIMENSIONS, DIMENSION_WORDS],
      [EDITABLE_NODE_TYPES, NODE_KIND_WORDS],
      [[...UNARY_OPERATORS, ...BINARY_OPERATORS, ...COMPARE_OPERATORS, ...LOGICAL_OPERATORS, ...AGGREGATE_FUNCTIONS, ...LOOKUP_MODES], OPERATOR_WORDS]
    ];
    for (const [tokens, table] of vocabularies) {
      for (const token of tokens) {
        const word = table[token];
        expect(word, token).toBeTruthy();
        // A word is not the format's token: no underscores, no code in capitals.
        expect(word, token).not.toMatch(/_|^[A-Z0-9_]{4,}$/);
      }
    }
  });

  it("labels and offers each control in those words, keeping the format name and token as titles", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));

    const inputs = openPart("inputs");
    expect(optionTexts("rule-pack-input-source-kind", inputs)).toEqual(words(SOURCE_KINDS, SOURCE_KIND_WORDS));
    expect(optionTexts("rule-pack-input-required-for", inputs)).toEqual(words(REQUIRED_FOR_TARGETS, REQUIRED_FOR_WORDS));
    expect(optionTexts("rule-pack-input-completeness", inputs)).toEqual(words(COMPLETENESS_STATUSES, COMPLETENESS_WORDS));
    expect(optionTexts("rule-pack-input-dimension", inputs)).toEqual(words(DIMENSIONS, DIMENSION_WORDS));
    expect(optionTexts("rule-pack-slot-kind", inputs)).toEqual(words(SLOT_KINDS, SLOT_KIND_WORDS));
    expect(optionTexts("rule-pack-slot-value-status", inputs)).toEqual(words(VALUE_STATUSES, VALUE_STATUS_WORDS));
    const sourceKind = within(inputs).getByTestId("rule-pack-input-source-kind");
    const sourceLabel = sourceKind.closest("label") as HTMLLabelElement;
    expect(sourceLabel.querySelector("span")?.textContent).toBe(fieldWord("source_kind"));
    expect(sourceLabel.title).toBe("Format name: source_kind");
    expect(Array.from((sourceKind as HTMLSelectElement).options).map((option) => option.title)).toEqual([...SOURCE_KINDS]);
    // The name leads the row; the identifier is under "More".
    const row = within(inputs).getAllByTestId("rule-pack-required-input-row")[0];
    expect(row.querySelector("input")?.getAttribute("data-testid")).toBe("rule-pack-input-name");
    expect(within(row).getByTestId("rule-pack-input-id").closest("details")?.getAttribute("data-testid")).toBe("rule-pack-input-more");

    const checks = openPart("checks");
    expect(optionTexts("rule-pack-check-acceptability-relation", checks)).toEqual(
      words(ACCEPTABILITY_RELATIONS, ACCEPTABILITY_RELATION_WORDS)
    );
    expect(optionTexts("rule-pack-check-acceptability-basis", checks)).toEqual(words(ACCEPTABILITY_BASES, ACCEPTABILITY_BASIS_WORDS));
    expect(optionTexts("rule-pack-check-policy-missing_input", checks)).toEqual(words(DIAGNOSTIC_CODES, DIAGNOSTIC_CODE_WORDS));
    const status = within(checks).getByTestId("rule-pack-check-status-USER_RULE_CHECKED").closest("label") as HTMLLabelElement;
    expect(status.textContent).toBe(ANALYSIS_STATUS_WORDS.USER_RULE_CHECKED);
    // A reference to an input shows its name, not its identifier.
    expect(optionTexts("rule-pack-check-input-ref", checks)).toEqual(["User-defined required input (placeholder)"]);

    const formula = openPart("formula");
    expect(optionTexts("rule-pack-node-type", formula)).toEqual(words(EDITABLE_NODE_TYPES, NODE_KIND_WORDS));
    expect(within(formula).getAllByTestId("rule-pack-node-kind")[0].textContent).toBe(NODE_KIND_WORDS.variable_ref);

    // No field label on any opened part is a field name with an underscore.
    for (const key of PART_KEYS) {
      for (const label of Array.from(openPart(key).querySelectorAll('label[title^="Format name"] > span, .rule-pack-node-label'))) {
        expect(label.textContent ?? "", key).not.toMatch(/[a-z]_[a-z]/);
      }
    }
  });
});

describe("the document a set of entries produces", () => {
  it("is byte for byte the document the format's own members give, and is what the checksum is computed over", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));

    const loadCases = openPart("loadCases");
    fireEvent.change(within(loadCases).getByTestId("rule-pack-load-case-name"), { target: { value: "Invented sustained case" } });
    fireEvent.change(within(loadCases).getByTestId("rule-pack-load-case-basis-type"), { target: { value: "load_case" } });
    fireEvent.change(within(loadCases).getByTestId("rule-pack-load-case-basis-id"), { target: { value: "L-100" } });
    const categories = openPart("categories");
    fireEvent.change(within(categories).getByTestId("rule-pack-category-name"), { target: { value: "Invented category" } });
    const inputs = openPart("inputs");
    fireEvent.change(within(inputs).getByTestId("rule-pack-input-name"), { target: { value: "Invented stress" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-input-dimension"), { target: { value: "stress" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-input-unit"), { target: { value: "MPa" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-input-completeness"), { target: { value: "complete" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-slot-kind"), { target: { value: "stress_limit" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-slot-dimension"), { target: { value: "stress" } });
    fireEvent.change(within(inputs).getByTestId("rule-pack-slot-unit"), { target: { value: "MPa" } });
    const checks = openPart("checks");
    fireEvent.change(within(checks).getByTestId("rule-pack-check-acceptability-relation"), { target: { value: "less_than" } });
    fireEvent.click(within(checks).getByTestId("rule-pack-check-status-HUMAN_REVIEW_REQUIRED"));

    // The same entries made on the format's own members.
    const expected = buildDraftRulePackDocument() as Record<string, any>;
    expected.required_load_cases[0] = { ...expected.required_load_cases[0], name: "Invented sustained case" };
    expected.required_load_cases[0] = { ...expected.required_load_cases[0], basis_ref: { ref_type: "load_case", ref_id: "" } };
    expected.required_load_cases[0] = { ...expected.required_load_cases[0], basis_ref: { ref_type: "load_case", ref_id: "L-100" } };
    expected.stress_categories[0] = { ...expected.stress_categories[0], name: "Invented category" };
    const input = expected.required_inputs[0];
    expected.required_inputs[0] = { ...input, name: "Invented stress" };
    expected.required_inputs[0] = {
      ...expected.required_inputs[0],
      quantity_intent: { ...expected.required_inputs[0].quantity_intent, dimension: "stress" }
    };
    expected.required_inputs[0] = {
      ...expected.required_inputs[0],
      quantity_intent: { ...expected.required_inputs[0].quantity_intent, unit_ref: "MPa" }
    };
    expected.required_inputs[0] = { ...expected.required_inputs[0], completeness_status: "complete" };
    expected.value_slots[0] = { ...expected.value_slots[0], slot_kind: "stress_limit" };
    expected.value_slots[0] = { ...expected.value_slots[0], quantity_intent: { ...expected.value_slots[0].quantity_intent, dimension: "stress" } };
    expected.value_slots[0] = { ...expected.value_slots[0], quantity_intent: { ...expected.value_slots[0].quantity_intent, unit_ref: "MPa" } };
    expected.check_definitions[0] = { ...expected.check_definitions[0], acceptability_relation: "less_than" };
    expected.check_definitions[0] = {
      ...expected.check_definitions[0],
      result_statuses: expected.check_definitions[0].result_statuses.includes("HUMAN_REVIEW_REQUIRED")
        ? expected.check_definitions[0].result_statuses.filter((status: string) => status !== "HUMAN_REVIEW_REQUIRED")
        : [...expected.check_definitions[0].result_statuses, "HUMAN_REVIEW_REQUIRED"]
    };

    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(textarea.value).toBe(JSON.stringify(expected, null, 2));
    // No field was prefilled with an engineering value: the limit's number is not in the pack.
    expect(JSON.stringify(expected.value_slots)).not.toMatch(/"value":/);

    // The checksum is computed over exactly that document.
    fireEvent.click(screen.getByTestId("rule-pack-compute-checksum"));
    await waitFor(() => expect(backend.checksumDocuments).toHaveLength(1));
    expect(JSON.stringify(backend.checksumDocuments[0] as RulePackDocument, null, 2)).toBe(JSON.stringify(expected, null, 2));
  });
});

// Round 1 review repairs.

const otherEntry: LocalRulePackIndexEntry = {
  ...savedEntry,
  project_id: "project:invented-other",
  rule_pack_id: "invented_other_pack",
  rule_pack_name: "Invented other pack"
};

function modelFor(id: string): PreviewModel {
  return { project: { id, name: id }, rule_pack_refs: [] } as unknown as PreviewModel;
}

describe("the list shows one project's packs only", () => {
  it("reads nothing without a project, so no other project's pack can be opened or deleted", async () => {
    render(<RulePackManagerPanel model={null} />);
    await act(async () => {});
    expect(backend.listCalls).toBe(0);
    expect(screen.queryByTestId("rule-pack-entry-invented_saved_pack")).toBeNull();
    expect(screen.queryByTestId("rule-pack-open-invented_saved_pack")).toBeNull();
  });

  it("drops a list read that a save in project A finishes after the page moved to project B", async () => {
    backend.entries = [savedEntry, otherEntry];
    const { rerender } = render(<RulePackManagerPanel model={modelFor("project:invented-page")} />);
    await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());

    rerender(<RulePackManagerPanel model={modelFor("project:invented-other")} />);
    await screen.findByTestId("rule-pack-entry-invented_other_pack");
    await act(async () => {
      backend.releaseSave?.();
    });
    await act(async () => {});
    expect(screen.getByTestId("rule-pack-entry-invented_other_pack")).toBeTruthy();
    expect(screen.queryByTestId("rule-pack-entry-invented_saved_pack")).toBeNull();
  });

  it("shows project B's packs when a save in project A finishes while B's list is still being read", async () => {
    backend.entries = [savedEntry, otherEntry];
    const { rerender } = render(<RulePackManagerPanel model={modelFor("project:invented-page")} />);
    await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());

    backend.heldLists = 1;
    rerender(<RulePackManagerPanel model={modelFor("project:invented-other")} />);
    await waitFor(() => expect(backend.releaseLists).toHaveLength(1));
    // A's save finishes first: it starts no read, so B's read still stands.
    await act(async () => {
      backend.releaseSave?.();
    });
    await act(async () => {
      backend.releaseLists[0]();
    });
    await screen.findByTestId("rule-pack-entry-invented_other_pack");
    expect(screen.queryByTestId("rule-pack-entry-invented_saved_pack")).toBeNull();
    expect(backend.listProjects).toEqual(["project:invented-page", "project:invented-other"]);
  });

  it("clears saved state and save status immediately when switching projects while a post-save list read is pending", async () => {
    backend.entries = [otherEntry];
    backend.upsertOnSave = true;
    const { rerender } = render(<RulePackManagerPanel model={modelFor("project:invented-page")} />);
    await screen.findByTestId("rule-pack-list-status");

    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    backend.heldLists = 1;
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());
    await act(async () => {
      backend.releaseSave?.();
    });
    await waitFor(() => expect(backend.releaseLists).toHaveLength(1));
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("");
    expect(screen.getByTestId("rule-pack-action-status").textContent).toContain("Saved on this computer.");

    rerender(<RulePackManagerPanel model={modelFor("project:invented-other")} />);
    await screen.findByTestId("rule-pack-entry-invented_other_pack");
    expect(screen.getByTestId("rule-pack-open-pack")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe(PAGE_WORDS.notSaved);
    expect(screen.queryByTestId("rule-pack-action-status")).toBeNull();

    await act(async () => {
      backend.releaseLists[0]();
    });
    await act(async () => {});
    expect(screen.getByTestId("rule-pack-entry-invented_other_pack")).toBeTruthy();
    expect(screen.queryByTestId("rule-pack-entry-private_draft_rule_pack")).toBeNull();
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe(PAGE_WORDS.notSaved);
    expect(screen.queryByTestId("rule-pack-action-status")).toBeNull();
  });
});

/** The Rules page as the shell mounts it: retained, asleep while hidden, counted back in. */
function ShellRules({
  shown,
  model,
  onQueueIntent
}: {
  shown: boolean;
  model: PreviewModel;
  onQueueIntent?: (intent: unknown) => void;
}) {
  const visit = useReturnCount(shown);
  return (
    <DormantSection page="rule-packs" active={shown} sessionGeneration={1}>
      <RulePackManagerPanel model={model} visit={visit} onQueueIntent={onQueueIntent} />
    </DormantSection>
  );
}

describe("the list is read again on each return to the Rules page", () => {
  it("recovers a failed read and shows a pack saved meanwhile, through the sleeping section", async () => {
    backend.listFailures = 1;
    const { rerender } = render(<ShellRules shown model={modelStub} />);
    await waitFor(() => expect(screen.getByTestId("rule-pack-list-status").textContent).toContain("could not be read"));
    expect(backend.listCalls).toBe(1);

    // Away and back in the same project: the hidden page sleeps, and the return reads again.
    rerender(<ShellRules shown={false} model={modelStub} />);
    await act(async () => {});
    expect(backend.listCalls).toBe(1);
    rerender(<ShellRules shown model={modelStub} />);
    await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    expect(backend.listCalls).toBe(2);

    // A pack saved elsewhere while the page was away shows on the next return.
    rerender(<ShellRules shown={false} model={modelStub} />);
    backend.entries = [savedEntry, { ...savedEntry, rule_pack_id: "invented_second_pack", rule_pack_name: "Invented second pack" }];
    await act(async () => {});
    expect(screen.queryByTestId("rule-pack-entry-invented_second_pack")).toBeNull();
    rerender(<ShellRules shown model={modelStub} />);
    await screen.findByTestId("rule-pack-entry-invented_second_pack");
    expect(backend.listCalls).toBe(3);
    expect(backend.listProjects).toEqual(["project:invented-page", "project:invented-page", "project:invented-page"]);
  });

  it("reads once on the first showing, under StrictMode as the app runs, and again for another project", async () => {
    backend.entries = [savedEntry, otherEntry];
    const { rerender } = render(
      <StrictMode>
        <ShellRules shown model={modelStub} />
      </StrictMode>
    );
    await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    await act(async () => {});
    expect(backend.listCalls).toBe(1);

    rerender(
      <StrictMode>
        <ShellRules shown model={modelFor("project:invented-other")} />
      </StrictMode>
    );
    await screen.findByTestId("rule-pack-entry-invented_other_pack");
    expect(screen.queryByTestId("rule-pack-entry-invented_saved_pack")).toBeNull();
    expect(backend.listProjects).toEqual(["project:invented-page", "project:invented-other"]);
  });

  it("never says a model's pack is missing from the store when a return's read fails", async () => {
    const referencing = {
      project: { id: "project:invented-page", name: "Invented Page Project" },
      rule_pack_refs: [
        {
          rule_pack_id: "invented_saved_pack",
          name: "Invented saved pack",
          rule_pack_version: "0.2.0",
          checksum: { algorithm: "sha256", value: "0".repeat(64) }
        }
      ]
    } as unknown as PreviewModel;
    const queue = vi.fn();
    const { rerender } = render(<ShellRules shown model={referencing} onQueueIntent={queue} />);
    await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    expect(screen.getByTestId("rule-pack-model-ref-invented_saved_pack")).toBeTruthy();
    expect(screen.queryByTestId("rule-pack-ref-unresolved-invented_saved_pack")).toBeNull();

    rerender(<ShellRules shown={false} model={referencing} onQueueIntent={queue} />);
    backend.listFailures = 1;
    rerender(<ShellRules shown model={referencing} onQueueIntent={queue} />);
    await waitFor(() => expect(screen.getByTestId("rule-pack-list-status").textContent).toContain("could not be read"));
    expect(screen.getByTestId("rule-pack-model-ref-invented_saved_pack")).toBeTruthy();
    expect(screen.queryByTestId("rule-pack-ref-unresolved-invented_saved_pack")).toBeNull();
  });
});

describe("the count of faults agrees with the parts", () => {
  it("counts a blocking unit finding, so the page never says no faults while a check shows one", async () => {
    backend.validation = {
      ...validationWith([]),
      has_blocking_findings: false,
      unit_check: {
        status: "findings",
        checks: [
          {
            check_id: "user_check_1",
            formula_id: "user_formula_1",
            status: "findings",
            findings: [
              { code: "UnitMismatch", severity: "blocking", subject: "add_subtract", path: "formula_declarations[0]", operands: [], message: "operand units differ" },
              { code: "RULE_PACK_LIMIT_CANDIDATE_MISMATCH", severity: "conditional", subject: "s", path: "p", operands: [], message: "conditional only" }
            ]
          }
        ],
        declaration_findings: []
      }
    };
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    const words = await screen.findByTestId("rule-pack-validation-words");
    expect(words.textContent).toContain("1 thing to fix");
    expect(words.textContent).not.toContain(PAGE_WORDS.noFaults);
    // A unit finding of a check's formula goes on the formula's line, where it can be repaired.
    expect(screen.getByTestId("rule-pack-part-fault-formula").textContent).toBe("operand units differ");
    expect(screen.queryByTestId("rule-pack-part-fault-checks")).toBeNull();
    expect(faultsOf(backend.validation, null)).toHaveLength(1);
  });

  it("puts a check of unsaved edits on the open pack, not on the saved pack's row", async () => {
    const document = buildDraftRulePackDocument();
    backend.openEnvelope = {
      project_id: savedEntry.project_id,
      rule_pack_id: savedEntry.rule_pack_id,
      storage_mode: "local_sqlite",
      created_at_unix: 1,
      updated_at_unix: 1,
      document,
      validation: validationWith([]),
      message: "Opened."
    };
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_saved_pack"));
    const row = () => screen.getByTestId("rule-pack-entry-invented_saved_pack");
    await waitFor(() => expect(row().textContent).toContain(PAGE_WORDS.noFaults));

    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    backend.validation = validationWith([
      { code: "RULE_PACK_FIELD_REQUIRED", subject: "required_inputs[1]", message: "the new input needs a unit", blocking: true }
    ]);
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    await screen.findByTestId("rule-pack-part-fault-inputs");
    expect(row().textContent).not.toContain("to fix");
    expect(row().textContent).toContain("awaiting review");
  });
});

describe("a pack saved under another identifier", () => {
  it("is the open pack after saving, so the pack it was opened from shows no verdict of it", async () => {
    backend.openEnvelope = {
      project_id: savedEntry.project_id,
      rule_pack_id: savedEntry.rule_pack_id,
      storage_mode: "local_sqlite",
      created_at_unix: 1,
      updated_at_unix: 1,
      document: buildDraftRulePackDocument(),
      validation: validationWith([]),
      message: "Opened."
    };
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_saved_pack"));
    const row = (id: string) => screen.getByTestId(`rule-pack-entry-${id}`);
    await waitFor(() => expect(row("invented_saved_pack").textContent).toContain(PAGE_WORDS.noFaults));

    // The store answers the save under the identifier its document now names.
    backend.validation = validationWith([
      { code: "RULE_PACK_FIELD_REQUIRED", subject: "required_inputs[0]", message: "the input needs a unit", blocking: true }
    ]);
    const renamed = { ...savedEntry, rule_pack_id: "private_draft_rule_pack", rule_pack_name: "Renamed pack" };
    backend.entries = [savedEntry, renamed];
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());
    // The list read after the save is held: meanwhile the old row shows no verdict of the new text.
    backend.heldLists = 1;
    await act(async () => {
      backend.releaseSave?.();
    });
    await waitFor(() => expect(backend.releaseLists).toHaveLength(1));
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("");
    expect(row("invented_saved_pack").textContent).not.toContain("to fix");
    expect(row("invented_saved_pack").textContent).toContain("awaiting review");
    await act(async () => {
      backend.releaseLists[0]();
    });
    await waitFor(() => expect(row("private_draft_rule_pack").textContent).toContain("to fix"));
    expect(row("invented_saved_pack").textContent).not.toContain("to fix");
    expect(row("invented_saved_pack").textContent).toContain("awaiting review");
  });

  it("binds the saved identity on first save so its row shows the verdict and deleting its row closes the open pack", async () => {
    backend.entries = [];
    backend.upsertOnSave = true;
    render(<RulePackManagerPanel model={modelStub} />);
    await screen.findByTestId("rule-pack-list-status");

    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe(PAGE_WORDS.notSaved);
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());
    await act(async () => {
      backend.releaseSave?.();
    });

    const row = await screen.findByTestId("rule-pack-entry-private_draft_rule_pack");
    expect(row.textContent).toContain(PAGE_WORDS.noFaults);
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("");

    fireEvent.click(screen.getByTestId("rule-pack-delete-private_draft_rule_pack"));
    await waitFor(() => expect(backend.deleted).toEqual([{ projectId: "project:invented-page", rulePackId: "private_draft_rule_pack" }]));
    await waitFor(() => expect(screen.queryByTestId("rule-pack-entry-private_draft_rule_pack")).toBeNull());
    expect(screen.queryByTestId("rule-pack-open-pack")).toBeNull();
    expect(screen.queryByTestId("rule-pack-draft-json")).toBeNull();
    expect(screen.queryByTestId("rule-pack-save-state")).toBeNull();
  });

  it("keeps the saved identity from the save response even when the post-save list read fails", async () => {
    backend.entries = [];
    backend.upsertOnSave = true;
    const { rerender } = render(<ShellRules shown model={modelStub} />);
    await screen.findByTestId("rule-pack-list-status");

    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    backend.listFailures = 1;
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(backend.releaseSave).not.toBeNull());
    await act(async () => {
      backend.releaseSave?.();
    });
    await waitFor(() => expect(screen.getByTestId("rule-pack-list-status").textContent).toContain("could not be read"));
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("");

    // Returning to the page recovers the list, and the already-bound saved entry matches the listed row.
    rerender(<ShellRules shown={false} model={modelStub} />);
    rerender(<ShellRules shown model={modelStub} />);
    const row = await screen.findByTestId("rule-pack-entry-private_draft_rule_pack");
    expect(row.textContent).toContain(PAGE_WORDS.noFaults);

    fireEvent.click(screen.getByTestId("rule-pack-delete-private_draft_rule_pack"));
    await waitFor(() => expect(screen.queryByTestId("rule-pack-entry-private_draft_rule_pack")).toBeNull());
    expect(screen.queryByTestId("rule-pack-open-pack")).toBeNull();
  });

  it("unbinds an opened pack and its saved state on project switch while preserving the authored draft", async () => {
    const projectBSameIdEntry: LocalRulePackIndexEntry = {
      ...savedEntry,
      project_id: "project:invented-other",
      rule_pack_name: "Project B same-id pack"
    };
    backend.entries = [savedEntry, projectBSameIdEntry];
    backend.openEnvelope = {
      project_id: savedEntry.project_id,
      rule_pack_id: savedEntry.rule_pack_id,
      storage_mode: "local_sqlite",
      created_at_unix: 1,
      updated_at_unix: 1,
      document: buildDraftRulePackDocument(),
      validation: validationWith([]),
      message: "Opened."
    };
    const { rerender } = render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(await screen.findByTestId("rule-pack-open-invented_saved_pack"));
    await waitFor(() => expect(screen.getByTestId("rule-pack-entry-invented_saved_pack").textContent).toContain(PAGE_WORDS.noFaults));
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe("");

    rerender(<RulePackManagerPanel model={modelFor("project:invented-other")} />);
    const rowInB = await screen.findByTestId("rule-pack-entry-invented_saved_pack");
    expect(rowInB.textContent).toContain("Project B same-id pack");
    expect(rowInB.textContent).not.toContain(PAGE_WORDS.noFaults);
    expect(screen.getByTestId("rule-pack-open-pack")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-save-state").textContent).toBe(PAGE_WORDS.notSaved);

    // Deleting Project B's same-ID pack clears Project B's row only and leaves the carried draft open.
    fireEvent.click(screen.getByTestId("rule-pack-delete-invented_saved_pack"));
    await waitFor(() =>
      expect(backend.deleted).toEqual([{ projectId: "project:invented-other", rulePackId: savedEntry.rule_pack_id }])
    );
    await waitFor(() => expect(screen.queryByTestId("rule-pack-entry-invented_saved_pack")).toBeNull());
    expect(screen.getByTestId("rule-pack-open-pack")).toBeTruthy();
  });
});

describe("findings reach the part their subject names", () => {
  it("looks a dotted identifier up whole, and places a duplicate mapping identifier with the categories", () => {
    const document = {
      ...buildDraftRulePackDocument(),
      stress_categories: [{ category_id: "stress.hot", name: "Hot", description: "" }],
      required_inputs: [{ input_id: "check_definitions.like_id", name: "An input" }]
    } as RulePackDocument;
    expect(partOfSubject("stress.hot", document)).toBe("categories");
    expect(partOfSubject("check_definitions.like_id", document)).toBe("inputs");
    expect(partOfSubject("user_load_case_mapping_1", document)).toBe("categories");
  });
});

describe("references never make two declarations look the same", () => {
  it("shows the identifier beside a name two options share", () => {
    expect(referenceNames(["a", "b", "c"], { a: "Same", b: "Same", c: "Other" })).toEqual({
      a: "Same (a)",
      b: "Same (b)",
      c: "Other"
    });
    expect(referenceNames(["x"], {})).toEqual({ x: "x" });
  });

  it("names two inputs with the same placeholder name apart, and offers no limit as a formula variable", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    const checks = openPart("checks");
    expect(optionTexts("rule-pack-check-input-ref", checks)).toEqual([
      "User-defined required input (placeholder) (user_required_input_1)",
      "User-defined required input (placeholder) (user_required_input_2)"
    ]);
    const formula = openPart("formula");
    const variable = within(formula).getByTestId("rule-pack-variable-ref") as HTMLSelectElement;
    const values = Array.from(variable.options).map((option) => option.value);
    expect(values).toEqual(["user_required_input_1", "user_required_input_2"]);
    expect(values).not.toContain("user_limit_slot_1");
    expect(bindableVariables(collectRulePackVariables(buildDraftRulePackDocument())).map((v) => v.id)).toEqual([
      "user_required_input_1"
    ]);
  });
});

describe("an input whose name is empty", () => {
  it("is offered in the formula by its identifier", () => {
    const document = buildDraftRulePackDocument();
    (document.required_inputs as Array<Record<string, unknown>>)[0].name = "";
    expect(collectRulePackVariables(document).find((variable) => variable.kind === "required_input")?.name).toBe(
      "user_required_input_1"
    );
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    const inputs = openPart("inputs");
    fireEvent.change(within(inputs).getByTestId("rule-pack-input-name"), { target: { value: "" } });
    const formula = openPart("formula");
    expect(optionTexts("rule-pack-variable-ref", formula)).toEqual(["user_required_input_1"]);
    expect(NODE_KIND_WORDS.variable_ref).toBe("Input");
  });
});

describe("load cases and categories that share a name", () => {
  it("names them apart where a category maps load cases and where a check picks its category", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    const loadCases = openPart("loadCases");
    fireEvent.click(within(loadCases).getByTestId("rule-pack-load-case-add"));
    within(loadCases)
      .getAllByTestId("rule-pack-load-case-name")
      .forEach((field) => fireEvent.change(field, { target: { value: "Sustained" } }));
    const categories = openPart("categories");
    fireEvent.click(within(categories).getByTestId("rule-pack-category-add"));
    within(categories)
      .getAllByTestId("rule-pack-category-name")
      .forEach((field) => fireEvent.change(field, { target: { value: "Primary" } }));

    const mapping = within(categories).getAllByTestId("rule-pack-category-mapping")[0];
    const labels = Array.from(mapping.querySelectorAll("label span")).map((node) => node.textContent ?? "");
    expect(labels.length).toBeGreaterThan(1);
    expect(new Set(labels).size).toBe(labels.length);
    labels.forEach((label) => expect(label).toMatch(/^Sustained \(.+\)$/));

    const checks = openPart("checks");
    const picked = optionTexts("rule-pack-check-category-ref", checks).filter((text) => text.startsWith("Primary"));
    expect(picked.length).toBeGreaterThan(1);
    expect(new Set(picked).size).toBe(picked.length);
  });
});

describe("what a check reports and which messages it uses", () => {
  it("separates the outcomes a check reports and the messages it uses from those only kept with the pack", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    const checks = openPart("checks");
    const statuses = within(checks).getByTestId("rule-pack-check-result-statuses");
    const labels = Array.from(statuses.querySelectorAll(".rule-pack-node-label")).map((node) => node.textContent);
    expect(labels).toEqual([CHECK_GROUP_WORDS.reportedOutcomes, CHECK_GROUP_WORDS.keptOutcomes]);
    // The reported outcomes come first, under their own heading.
    const order = Array.from(statuses.querySelectorAll("input")).map((input) => input.getAttribute("data-testid"));
    expect(order.slice(0, 3)).toEqual(REPORTED_STATUSES.map((status) => `rule-pack-check-status-${status}`));
    expect(ANALYSIS_STATUS_WORDS.USER_RULE_CHECKED).toBe("Within limit");
    expect(ANALYSIS_STATUS_WORDS.USER_RULE_FAILED).toBe("Exceeds limit");
    expect(fieldWord("acceptability_relation")).toBe("Within limit when the result is");

    const policy = within(checks).getByTestId("rule-pack-check-diagnostic-policy");
    const groups = Array.from(policy.querySelectorAll(":scope > .rule-pack-check-subgroup"));
    expect(groups.map((group) => group.querySelector(".rule-pack-node-label")?.textContent)).toEqual([
      CHECK_GROUP_WORDS.usedMessages,
      CHECK_GROUP_WORDS.keptMessages
    ]);
    const used = Array.from(groups[0].querySelectorAll("select")).map((select) => select.getAttribute("data-testid"));
    expect(used).toEqual(RUNTIME_POLICY_FIELDS.map((field) => `rule-pack-check-policy-${field}`));
    expect(groups[1].querySelectorAll("select")).toHaveLength(DIAGNOSTIC_POLICY_FIELDS.length - RUNTIME_POLICY_FIELDS.length);
    // Every stored token is kept: the eight settings still read the template's codes.
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(JSON.parse(textarea.value).check_definitions[0].diagnostic_policy).toEqual(
      (buildDraftRulePackDocument().check_definitions as Array<Record<string, unknown>>)[0].diagnostic_policy
    );
  });

  it("says that an input's value from the model, the owner or nowhere chosen is entered when checking", () => {
    for (const token of ["model_input", "owner_design_basis", "TBD", "user_supplied_rule_value"]) {
      expect(SOURCE_KIND_WORDS[token]).toMatch(/[Ee]ntered when checking/);
    }
    expect(fieldWord("value_status")).toBe("Declared value status");
  });
});
