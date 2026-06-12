// Cross-engine operation contract corpus runner (TP-SEAM-CORPUS-001;
// TS engine lane retired at TP-SEAM-SWAP-001 per DEC-020 / ADR-0001).
//
// Reads the SAME case files as the Rust runner
// (core/model_operations/operation_applier/tests/contract_corpus.rs) from
// fixtures/model_operations/contract_corpus/ and executes each one through
// two browser-mode lanes — the thin routing adapter in operationService.ts
// (wasm-backed; asserts the public seam) and the wasm engine directly
// (asserts the engine itself) — checking the expected outcomes recorded
// from the native Rust contract reference under identical comparison rules:
// - exact equality on semantic fields; diagnostics compared as
//   code/severity/blocking/affected_refs records, order-insensitively;
// - engine-identity fields excluded only via the documented allowlist
//   (application_route, diagnostics[].id, diagnostics[].source,
//   diagnostics[].message, diagnostics[].remediation — see the corpus
//   README). Tightened at TP-H1-HASHUNIFY-001 (backend_canonicalization
//   label compared) and again at H5: the engine canonicalization is
//   RFC 8785 (JCS), so model_basis.backend_model_hash and
//   applied_model_backend_hash VALUES are full byte-equality comparisons
//   in every lane — JS transport (200.0 → 200) no longer shifts the
//   canonical bytes, which is what kept those values excluded between
//   TP-H1-HASHUNIFY-001 and H5;
// - applied model documents compared BOTH by parsed-JSON deep equality AND
//   by the engine's own canonical hash through the wasm exports (the ECMA
//   harness renderer twins that previously approximated transport
//   invariance are retired — the engine rendering IS transport-invariant,
//   pinned by fixtures/canonical_hash/).
//
// All corpus data is invented (PUBLIC_DOMAIN_OR_ORIGINAL); no protected
// standards content; no compliance/certification/approval claims.

import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, it } from "vitest";
import type { EditorOperationIntent, ModelHashEvidence, OperationOutcome, PreviewModel } from "../types";
import { applyModelOperation, validateModelOperation } from "./operationService";
import { loadWasmEngine, type WasmOperationEngine } from "./wasmEngine/loadWasmEngine";

// The harness canonical hash is the engine's own (H1 / F-5a): the setup file
// pre-warms the engine, so the singleton resolves before any case executes.
const harnessEngine = await loadWasmEngine();

const corpusDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../fixtures/model_operations/contract_corpus"
);

const REQUIRED_ACCEPTED_KINDS = [
  "set_field",
  "update_load",
  "update_support",
  "create_node",
  "connect_pipe_run",
  "create_section",
  "create_material",
  "create_support",
  "create_load_case",
  "delete_load_case",
  "create_primitive_load",
  "delete_primitive_load",
  "create_combination",
  "delete_combination",
  "create_combination_term",
  "delete_support",
  "delete_combination_term"
] as const;

const REQUIRED_BLOCK_CODES = [
  "OP-TARGET-ALREADY-EXISTS", // duplicate id
  "OP-TARGET-NOT-FOUND", // missing target
  "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE", // unit mismatch
  "OP-UNIT-DIMENSION-UNKNOWN", // invalid dimension
  "OP-STALE-BEFORE-VALUE", // stale before-value
  "OP-FIELD-EDIT-DEFERRED", // deferred field
  "OP-FIELD-PATH-UNSUPPORTED", // unsupported field path
  "OP-VALUE-NOT-NUMERIC", // non-finite magnitude
  "OP-VALUE-EMPTY", // empty required text
  "OP-UNIT-DIMENSION-MISMATCH", // dynamic terms.N.factor block
  "OP-SUPPORT-DELETE-REFERENCED", // support still referenced by primitive load
  "OP-LOAD-CASE-DELETE-REFERENCED" // load case still referenced by combination
] as const;

type SemanticDiagnostic = {
  code: string;
  severity: string;
  blocking: boolean;
  affected_refs: string[];
};

type CorpusCase = {
  case_id: string;
  description: string;
  mode: "apply" | "validate_only";
  base_model: unknown;
  intent: unknown;
  claimed_model_hash: ModelHashEvidence | null;
  expected: {
    outcome: Record<string, unknown> & {
      validation: Record<string, string>;
      diagnostics: SemanticDiagnostic[];
    };
    applied_model: unknown | null;
    applied_model_canonical_sha256: string | null;
  } | null;
};

function loadCases(): CorpusCase[] {
  const files = readdirSync(corpusDir)
    .filter((name) => name.endsWith(".json"))
    .filter((name) => statSync(path.join(corpusDir, name)).isFile())
    .sort();
  expect(files.length).toBeGreaterThan(0);
  return files.map((name) => JSON.parse(readFileSync(path.join(corpusDir, name), "utf8")) as CorpusCase);
}

function diagnosticSortKey(item: SemanticDiagnostic): string {
  return `${item.code}\u0000${item.affected_refs.join(",")}\u0000${item.severity}`;
}

function codepointCompare(left: string, right: string): number {
  return left < right ? -1 : left > right ? 1 : 0;
}

// Projects the engine outcome onto the cross-engine semantic comparison
// surface. Mirrored exactly by `semantic_outcome` in the Rust runner.
function semanticOutcome(outcome: OperationOutcome): Record<string, unknown> {
  const diagnostics: SemanticDiagnostic[] = outcome.diagnostics
    .map((item) => ({
      code: item.code,
      severity: item.severity,
      blocking: item.severity === "blocking",
      affected_refs: item.affected_refs
    }))
    .sort((left, right) => codepointCompare(diagnosticSortKey(left), diagnosticSortKey(right)));
  return {
    mode: outcome.mode,
    operation_id: outcome.operation_id,
    change_id: outcome.change_id,
    operation_kind: outcome.operation_kind,
    change_kind: outcome.change_kind,
    target_object_type: outcome.target_object_type,
    target_ref: outcome.target_ref,
    schema_version: outcome.schema_version,
    document_kind: outcome.document_kind,
    deliverable_refs: outcome.deliverable_refs,
    validation: outcome.validation,
    diff_preview: outcome.diff_preview,
    diagnostics,
    model_basis: {
      claimed_model_hash: outcome.model_basis.claimed_model_hash,
      claimed_hash_canonicalization: outcome.model_basis.claimed_hash_canonicalization,
      binding_status: outcome.model_basis.binding_status,
      // Label compared since TP-H1-HASHUNIFY-001; the VALUE joined the
      // compared surface at H5 — RFC 8785 rendering made input-text hashes
      // transport-invariant, retiring the measured exclusion.
      backend_canonicalization: outcome.model_basis.backend_canonicalization,
      backend_model_hash: outcome.model_basis.backend_model_hash
    },
    input_model_unchanged: outcome.input_model_unchanged,
    acceptance: outcome.acceptance,
    audit_boundary: outcome.audit_boundary,
    professional_boundary: outcome.professional_boundary,
    applied: outcome.applied_model !== null
  };
}

// The engine's own canonical hash through the wasm exports (RFC 8785 since
// H5). Transport-invariant: identical values hash identically whether they
// arrived as raw case-file text or through JSON.stringify, so this single
// formula replaced the retired TS ECMA harness renderer and serves the
// blessed-value comparison in every lane.
function corpusCanonicalHash(value: unknown): string {
  return `sha256:${harnessEngine.canonicalSha256Hex(JSON.stringify(value))}`;
}

function isDynamicPrimitiveMagnitudePath(fieldPath: string): boolean {
  return /^primitive_loads\.\d+\.magnitude\.value$/.test(fieldPath);
}

function isDynamicTermFactorPath(fieldPath: string): boolean {
  return /^terms\.\d+\.factor$/.test(fieldPath);
}

describe("operation contract corpus — browser-mode adapter lane (wasm engine via operationService)", () => {
  const cases = loadCases();

  it("covers the required operation kinds, block classes, and dynamic field paths", () => {
    const acceptedKinds = new Set<string>();
    const blockedCodes = new Set<string>();
    const acceptedDynamicPaths = new Set<string>();
    const blockedDynamicPaths = new Set<string>();
    for (const corpusCase of cases) {
      expect(corpusCase.expected, `case ${corpusCase.case_id}: expected outcome missing`).not.toBeNull();
      const expected = corpusCase.expected!;
      const intent = corpusCase.intent as { change?: { change_kind?: string; field_path?: string } };
      const changeKind = intent.change?.change_kind ?? "";
      const fieldPath = intent.change?.field_path ?? "";
      const accepted = expected.outcome.validation.application_status === "applied_to_session_model";
      if (accepted) {
        acceptedKinds.add(changeKind);
        if (isDynamicPrimitiveMagnitudePath(fieldPath)) acceptedDynamicPaths.add("primitive_loads.N.magnitude.value");
        if (isDynamicTermFactorPath(fieldPath)) acceptedDynamicPaths.add("terms.N.factor");
      } else {
        if (isDynamicPrimitiveMagnitudePath(fieldPath)) blockedDynamicPaths.add("primitive_loads.N.magnitude.value");
        if (isDynamicTermFactorPath(fieldPath)) blockedDynamicPaths.add("terms.N.factor");
      }
      for (const item of expected.outcome.diagnostics) {
        if (item.blocking) blockedCodes.add(item.code);
      }
    }
    for (const kind of REQUIRED_ACCEPTED_KINDS) {
      expect(acceptedKinds, `no accepted-apply corpus case for operation kind ${kind}`).toContain(kind);
    }
    for (const code of REQUIRED_BLOCK_CODES) {
      expect(blockedCodes, `no corpus case produces blocking diagnostic ${code}`).toContain(code);
    }
    for (const dynamicPath of ["primitive_loads.N.magnitude.value", "terms.N.factor"]) {
      expect(acceptedDynamicPaths, `no accepted-apply corpus case edits ${dynamicPath}`).toContain(dynamicPath);
      expect(blockedDynamicPaths, `no blocked corpus case targets ${dynamicPath}`).toContain(dynamicPath);
    }
  });

  for (const corpusCase of cases) {
    it(`case ${corpusCase.case_id} matches the Rust contract reference`, async () => {
      expect(corpusCase.expected, "expected outcome missing — regenerate with CORPUS_BLESS=1 cargo test").not.toBeNull();
      const expected = corpusCase.expected!;
      const model = JSON.parse(JSON.stringify(corpusCase.base_model)) as PreviewModel;
      const snapshot = JSON.parse(JSON.stringify(corpusCase.base_model));
      const intent = corpusCase.intent as unknown as EditorOperationIntent;
      const claimed = corpusCase.claimed_model_hash;

      const outcome =
        corpusCase.mode === "apply"
          ? await applyModelOperation(model, intent, claimed)
          : await validateModelOperation(model, intent, claimed);

      expect(model, "the input model document must never be mutated").toEqual(snapshot);
      expect(semanticOutcome(outcome)).toEqual(expected.outcome);

      if (expected.applied_model === null) {
        expect(outcome.applied_model).toBeNull();
        expect(outcome.applied_model_backend_hash).toBeNull();
        expect(expected.applied_model_canonical_sha256).toBeNull();
      } else {
        expect(outcome.applied_model, "accepted applies must return an applied model").not.toBeNull();
        expect(outcome.applied_model).toEqual(expected.applied_model);
        expect(corpusCanonicalHash(outcome.applied_model)).toBe(expected.applied_model_canonical_sha256);
        expect(
          outcome.applied_model_backend_hash,
          "the engine's applied_model_backend_hash must byte-equal the blessed canonical hash in every lane (H5 full byte-equality)"
        ).toBe(expected.applied_model_canonical_sha256);
      }
    });
  }
});

// Wasm engine lane (TP-SEAM-WASM-001, DEC-020 / ADR-0001): the same 44 case
// files run through the wasm32 build of the Rust crate, loaded in this test
// environment exactly as browser mode loads it. Together with the adapter
// lane above and the Rust-blessed expectations executed by `cargo test`,
// this proves native↔wasm parity (engine directly and through the public
// seam), including the corpus-harness canonical-hash comparison (hash
// stability across the wasm boundary). An absent wasm artifact fails loudly
// with the build command (WASM-ENGINE-ASSET-ABSENT → `npm run build:wasm`);
// the lane never skips silently and no fallback engine exists.
describe("operation contract corpus — wasm engine lane (native↔wasm parity)", () => {
  const cases = loadCases();
  let engine: WasmOperationEngine;

  beforeAll(async () => {
    engine = await loadWasmEngine();
  });

  it("reports the honest local_wasm_engine application route", () => {
    const corpusCase = cases[0];
    const outcome = JSON.parse(
      engine.validateOperationJson(JSON.stringify(corpusCase.base_model), JSON.stringify(corpusCase.intent), "null")
    ) as OperationOutcome;
    expect(outcome.application_route).toBe("local_wasm_engine");
  });

  it("returns the structured input-error envelope for malformed input instead of trapping", () => {
    const raw = engine.validateOperationJson("{not json", "{}", "null");
    const envelope = JSON.parse(raw) as { document_kind: string; error?: { code?: string; severity?: string } };
    expect(envelope.document_kind).toBe("openpipestress.desktop.wasm_engine_input_error");
    expect(envelope.error?.code).toBe("WASM-ENGINE-INPUT-JSON-INVALID");
    expect(envelope.error?.severity).toBe("blocking");
  });

  for (const corpusCase of cases) {
    it(`case ${corpusCase.case_id} wasm matches the Rust contract reference`, () => {
      expect(corpusCase.expected, "expected outcome missing — regenerate with CORPUS_BLESS=1 cargo test").not.toBeNull();
      const expected = corpusCase.expected!;
      const run = corpusCase.mode === "apply" ? engine.applyOperationJson : engine.validateOperationJson;
      const raw = run(
        JSON.stringify(corpusCase.base_model),
        JSON.stringify(corpusCase.intent),
        JSON.stringify(corpusCase.claimed_model_hash ?? null)
      );
      const outcome = JSON.parse(raw) as OperationOutcome;

      expect(semanticOutcome(outcome)).toEqual(expected.outcome);

      if (expected.applied_model === null) {
        expect(outcome.applied_model).toBeNull();
        expect(outcome.applied_model_backend_hash).toBeNull();
      } else {
        expect(outcome.applied_model, "accepted applies must return an applied model").not.toBeNull();
        expect(outcome.applied_model).toEqual(expected.applied_model);
        expect(corpusCanonicalHash(outcome.applied_model)).toBe(expected.applied_model_canonical_sha256);
        expect(
          outcome.applied_model_backend_hash,
          "the engine's applied_model_backend_hash must byte-equal the blessed canonical hash in every lane (H5 full byte-equality)"
        ).toBe(expected.applied_model_canonical_sha256);
      }
    });
  }
});
