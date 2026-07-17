import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { RulePackManagerPanel } from "./RulePackManagerPanel";
import {
  buildDraftRulePackDocument,
  stampChecksumIntoDocument,
  RULE_PACK_BACKEND_DIAGNOSTIC
} from "../../services/rulePackService";
import type { PreviewModel } from "../../types";

// Phase C2 slice 1 (TP-C2-EDITOR-001). jsdom has no Tauri runtime, so these
// tests pin the honest browser-preview seam: drafts stay in memory and
// every backend action reports the explicit desktop-only diagnostic instead
// of a synthesized fallback.

const modelStub = {
  project: { id: "project:invented-panel-test", name: "Invented Panel Test Project" }
} as unknown as PreviewModel;

afterEach(cleanup);

describe("RulePackManagerPanel", () => {
  it("scopes the manager to the loaded project and stays honest without one", () => {
    render(<RulePackManagerPanel model={null} />);
    expect(screen.getByTestId("rule-pack-scope-status").textContent).toContain(
      "create or open a local project first"
    );
    cleanup();

    render(<RulePackManagerPanel model={modelStub} />);
    expect(screen.getByTestId("rule-pack-scope-status").textContent).toContain(
      "project:invented-panel-test"
    );
    expect(screen.getByTestId("rule-pack-scope-status").textContent).toContain(
      "local SQLite only"
    );
  });

  it("creates a private-by-default draft and supports discard", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(textarea.disabled).toBe(true);
    expect(screen.getByTestId("rule-pack-validate")).toHaveProperty("disabled", true);

    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(textarea.disabled).toBe(false);
    const draft = JSON.parse(textarea.value) as Record<string, unknown>;
    expect(draft.rule_pack_kind).toBe("open_pipe_stress_rule_pack");
    expect(draft.grammar_version).toBe("1.0.0");
    const classification = draft.classification as Record<string, unknown>;
    expect(classification.privacy_class).toBe("private_user_data");
    expect(classification.redistribution_status).toBe("private_only");
    expect(screen.getByTestId("rule-pack-action-status").textContent).toContain(
      "private_user_data"
    );

    fireEvent.click(screen.getByTestId("rule-pack-discard-draft"));
    expect((screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement).value).toBe("");
    expect(screen.getByTestId("rule-pack-action-status").textContent).toContain(
      "Draft discarded"
    );
  });

  it("reports the explicit desktop-only diagnostic for backend actions in browser preview", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));

    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-pack-action-status").textContent).toContain(
        "RULE-PACK-BACKEND-DESKTOP-ONLY"
      )
    );

    fireEvent.click(screen.getByTestId("rule-pack-refresh-list"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-pack-list-status").textContent).toContain(
        "RULE-PACK-BACKEND-DESKTOP-ONLY"
      )
    );
    expect(screen.getByTestId("rule-pack-list-status").textContent).toBe(
      RULE_PACK_BACKEND_DIAGNOSTIC
    );
  });

  it("routes compute-checksum and save to the desktop-only seam from a clean draft state", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    const status = () => screen.getByTestId("rule-pack-action-status").textContent ?? "";

    // compute-checksum: transition is observable from the create message.
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(status()).toContain("private_user_data");
    fireEvent.click(screen.getByTestId("rule-pack-compute-checksum"));
    await waitFor(() => expect(status()).toContain("RULE-PACK-BACKEND-DESKTOP-ONLY"));

    // save: re-new the draft so the transition is again non-vacuous.
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(status()).toContain("private_user_data");
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() => expect(status()).toContain("RULE-PACK-BACKEND-DESKTOP-ONLY"));
  });

  it("blocks save with an honest reason when no project is loaded", async () => {
    render(<RulePackManagerPanel model={null} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-save"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-pack-action-status").textContent).toContain(
        "create or open a local project first"
      )
    );
  });

  it("reports invalid draft JSON honestly instead of acting on it", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    fireEvent.change(textarea, { target: { value: "{not json" } });
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    await waitFor(() =>
      expect(screen.getByTestId("rule-pack-action-status").textContent).toContain(
        "RULE-PACK-DRAFT-JSON-INVALID"
      )
    );
  });

  it("keeps the DEC-037 and professional boundaries visible", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    const note = screen.getByTestId("rule-pack-boundary-note").textContent ?? "";
    expect(note).toContain("DEC-037");
    expect(note).toContain("read-only");
    expect(note).toContain("no expression text parser");
    expect(note).toContain("never committed to the repository");
    expect(note).toContain(
      "acceptance and professional judgment remain with the responsible engineer",
    );
  });

  it("reveals the structured composer only with a draft and writes edits back into the document", () => {
    render(<RulePackManagerPanel model={modelStub} />);
    // No draft: a hint stands in for the composer; no composer surface yet.
    expect(screen.getByTestId("rule-pack-composer-no-draft")).toBeTruthy();
    expect(screen.queryByTestId("rule-pack-expression-composer")).toBeNull();

    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(screen.getByTestId("rule-pack-expression-composer")).toBeTruthy();
    expect(screen.getByTestId("rule-pack-variable-browser").textContent).toContain(
      "user_required_input_1"
    );

    // A structured edit (switch the formula's root node to a comparison)
    // rewrites the canonical document JSON the validate/save flow reads.
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(JSON.parse(textarea.value).formula_declarations[0].declaration_payload.expression_ast.node).toBe(
      "variable_ref"
    );
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], {
      target: { value: "compare" }
    });
    expect(JSON.parse(textarea.value).formula_declarations[0].declaration_payload.expression_ast.node).toBe(
      "compare"
    );
  });

  it("declares a required input through the editor and the composer's variable picker reflects it", () => {
    // Slice 4 (TP-C2-DECLEDITOR-001): the declarations editor and the
    // expression composer read the same draft document, so a variable added
    // through the editor is immediately bindable in the composer — no raw JSON.
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(screen.getByTestId("rule-pack-declarations-editor")).toBeTruthy();

    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(JSON.parse(textarea.value).required_inputs).toHaveLength(2);
    expect(screen.getByTestId("rule-pack-variable-browser").textContent).toContain(
      "user_required_input_2 (required_input)"
    );
  });

  it("mounts the check-definitions editor and writes an added check back to the document", () => {
    // Slice 5 (TP-C2-CHECKDEF-001): the check-definitions editor reads the same
    // draft document; adding a check grows the canonical document JSON.
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    expect(screen.getByTestId("rule-pack-check-definitions-editor")).toBeTruthy();
    expect((screen.getByTestId("rule-pack-check-formula-ref") as HTMLSelectElement).value).toBe(
      "user_formula_1"
    );

    fireEvent.click(screen.getByTestId("rule-pack-check-add"));
    const textarea = screen.getByTestId("rule-pack-draft-json") as HTMLTextAreaElement;
    expect(JSON.parse(textarea.value).check_definitions).toHaveLength(2);
  });

  it("binds a freshly declared required input in a check's reference picker", () => {
    // The declarations editor and the check editor read the same draft, so a
    // variable declared in one is immediately bindable as a check reference in
    // the other — no raw JSON.
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-input-add"));
    const inputRef = screen.getByTestId("rule-pack-check-input-ref") as HTMLSelectElement;
    const optionValues = Array.from(inputRef.options).map((option) => option.value);
    expect(optionValues).toContain("user_required_input_2");
  });
});

describe("rulePackService draft helpers", () => {
  it("builds a schema-shaped private draft with the frozen grammar declared", () => {
    const draft = buildDraftRulePackDocument();
    expect(draft.schema_version).toBe("0.4.0");
    expect(draft.grammar_version).toBe("1.0.0");
    const formulas = draft.formula_declarations as Array<Record<string, unknown>>;
    const payload = formulas[0].declaration_payload as Record<string, unknown>;
    expect(payload.grammar_status).toBe("frozen_open_pipe_stress_declared_expression");
    expect((payload.expression_ast as Record<string, unknown>).node).toBe("variable_ref");
    const boundary = draft.professional_boundary as Record<string, unknown>;
    expect(boundary.software_makes_compliance_claim).toBe(false);
    expect(boundary.human_review_required).toBe(true);
  });

  it("stamps a computed checksum without touching other members", () => {
    const draft = buildDraftRulePackDocument();
    const stamped = stampChecksumIntoDocument(draft, {
      algorithm: "sha256",
      canonicalization: "JCS",
      payload_scope: "rule_pack_payload",
      payload_ref: "private_draft_rule_pack",
      payload_excludes: ["checksums"],
      basis: "rfc8785_jcs_sha256_excluding_checksums",
      grammar_version: "1.0.0",
      grammar_version_bound: true,
      value: "0000000000000000000000000000000000000000000000000000000000000000"
    });
    const checksums = stamped.checksums as Record<string, unknown>;
    const rulePackChecksum = checksums.rule_pack_checksum as Record<string, unknown>;
    expect(rulePackChecksum.value).toBe(
      "0000000000000000000000000000000000000000000000000000000000000000"
    );
    expect(rulePackChecksum.verification_status).toBe("verified");
    expect(checksums.hash_basis).toBe("Canonical JSON/JCS-compatible");
    expect(stamped.metadata).toEqual(draft.metadata);
    expect(stamped.required_inputs).toEqual(draft.required_inputs);
  });
});
