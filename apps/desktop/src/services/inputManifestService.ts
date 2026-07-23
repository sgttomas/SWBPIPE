import type { PreviewModel } from "../types";
import {
  canonicalJsonString,
  canonicalSha256Hex,
} from "./hashService";

const SHA256_HEX = /^[0-9a-f]{64}$/;

export type InputManifestRulePack = {
  rule_pack_id: string;
  version: string;
  checksum_sha256: string;
  source_notice: string;
};

export type InputManifestExternalAsset = {
  asset_ref: string;
  media_type: string;
  checksum_sha256: string;
};

export type CurrentSessionInputManifestArgs = {
  model: PreviewModel;
  solver: {
    solver_name: string;
    solver_version: string;
    solver_build_ref: string;
    solver_mode: string;
    settings: Record<string, unknown>;
  };
  active_rule_packs: InputManifestRulePack[];
  external_assets: InputManifestExternalAsset[];
};

export type CurrentSessionInputManifest = {
  schema_version: "1.0.0";
  document_kind: "openpipestress.current_session_input_manifest";
  model_basis: {
    model_ref: string;
    model_payload: PreviewModel;
  };
  unit_basis: {
    project_units: Record<string, string>;
  };
  solver_basis: CurrentSessionInputManifestArgs["solver"];
  load_basis: {
    load_cases: PreviewModel["load_cases"];
    combinations: NonNullable<PreviewModel["combinations"]>;
  };
  active_rule_packs: InputManifestRulePack[];
  external_assets: InputManifestExternalAsset[];
  identity_policy: {
    canonicalization: "rfc8785_jcs";
    hash_algorithm: "sha256";
    canonical_bytes_scope: "entire_input_manifest_object";
  };
  replay_boundary: {
    included_as_package_member: false;
    portable_replay_claimed: false;
    current_session_ref_hash_integrity_only: true;
  };
};

export type CurrentSessionInputManifestEvidence = {
  manifest: CurrentSessionInputManifest;
  manifest_ref: {
    object_type: "InputManifest";
    ref: string;
  };
  manifest_sha256: string;
  canonical_bytes: string;
};

export async function buildCurrentSessionInputManifest(
  args: CurrentSessionInputManifestArgs,
): Promise<CurrentSessionInputManifestEvidence> {
  validateArgs(args);
  const manifest: CurrentSessionInputManifest = {
    schema_version: "1.0.0",
    document_kind: "openpipestress.current_session_input_manifest",
    model_basis: {
      model_ref: args.model.project.id,
      model_payload: structuredClone(args.model),
    },
    unit_basis: {
      project_units: sortedRecord(args.model.project.units),
    },
    solver_basis: {
      ...args.solver,
      settings: structuredClone(args.solver.settings),
    },
    load_basis: {
      load_cases: structuredClone(args.model.load_cases),
      combinations: structuredClone(args.model.combinations ?? []),
    },
    active_rule_packs: args.active_rule_packs
      .map((item) => ({ ...item }))
      .sort((left, right) => left.rule_pack_id.localeCompare(right.rule_pack_id)),
    external_assets: args.external_assets
      .map((item) => ({ ...item }))
      .sort((left, right) => left.asset_ref.localeCompare(right.asset_ref)),
    identity_policy: {
      canonicalization: "rfc8785_jcs",
      hash_algorithm: "sha256",
      canonical_bytes_scope: "entire_input_manifest_object",
    },
    replay_boundary: {
      included_as_package_member: false,
      portable_replay_claimed: false,
      current_session_ref_hash_integrity_only: true,
    },
  };
  const canonicalBytes = await canonicalJsonString(manifest);
  const manifestSha256 = await canonicalSha256Hex(manifest);
  if (!SHA256_HEX.test(manifestSha256)) {
    throw new Error(
      "INPUT-MANIFEST-HASH-INVALID: canonical SHA-256 must be bare lowercase 64-hex.",
    );
  }
  return {
    manifest,
    manifest_ref: {
      object_type: "InputManifest",
      ref: inputManifestRef(manifest.model_basis.model_ref, manifestSha256),
    },
    manifest_sha256: manifestSha256,
    canonical_bytes: canonicalBytes,
  };
}

export async function verifyCurrentSessionInputManifest(
  evidence: CurrentSessionInputManifestEvidence,
): Promise<void> {
  if (
    evidence.manifest_ref.object_type !== "InputManifest" ||
    !evidence.manifest_ref.ref.trim() ||
    !SHA256_HEX.test(evidence.manifest_sha256)
  ) {
    throw new Error(
      "INPUT-MANIFEST-EVIDENCE-INCOMPLETE: manifest ref and exact SHA-256 are required.",
    );
  }
  const canonicalBytes = await canonicalJsonString(evidence.manifest);
  const manifestSha256 = await canonicalSha256Hex(evidence.manifest);
  const manifestModelRef = evidence.manifest.model_basis.model_ref;
  const payloadModelRef =
    evidence.manifest.model_basis.model_payload.project.id;
  const expectedRef = inputManifestRef(manifestModelRef, manifestSha256);
  if (
    !manifestModelRef.trim() ||
    manifestModelRef !== payloadModelRef ||
    canonicalBytes !== evidence.canonical_bytes ||
    manifestSha256 !== evidence.manifest_sha256 ||
    evidence.manifest_ref.ref !== expectedRef
  ) {
    throw new Error(
      "INPUT-MANIFEST-HASH-MISMATCH: manifest transformation requires hash and ref recomputation.",
    );
  }
}

function validateArgs(args: CurrentSessionInputManifestArgs): void {
  if (
    !args.model ||
    !args.model.project?.id?.trim() ||
    !args.model.schema_version?.trim() ||
    !args.model.document_kind?.trim()
  ) {
    throw new Error(
      "INPUT-MANIFEST-MODEL-INCOMPLETE: exact current model input is required.",
    );
  }
  const units = Object.entries(args.model.project.units ?? {});
  if (
    units.length === 0 ||
    units.some(([dimension, unit]) => !dimension.trim() || !unit.trim())
  ) {
    throw new Error(
      "INPUT-MANIFEST-UNITS-INCOMPLETE: declared project units are required.",
    );
  }
  if (
    !args.solver?.solver_name?.trim() ||
    !args.solver.solver_version?.trim() ||
    !args.solver.solver_build_ref?.trim() ||
    !args.solver.solver_mode?.trim() ||
    !isCompleteJson(args.solver.settings)
  ) {
    throw new Error(
      "INPUT-MANIFEST-SOLVER-INCOMPLETE: solver identity, mode, and settings are required.",
    );
  }
  if (
    !Array.isArray(args.model.load_cases) ||
    args.model.load_cases.length === 0 ||
    args.model.load_cases.some(
      (item) =>
        !item.id?.trim() ||
        !item.kind?.trim() ||
        !item.status?.trim() ||
        !item.provenance?.trim(),
    )
  ) {
    throw new Error(
      "INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE: complete load-case bases are required.",
    );
  }
  if (!Array.isArray(args.active_rule_packs)) {
    throw new Error(
      "INPUT-MANIFEST-RULE-PACKS-INCOMPLETE: active rule-pack inventory is required.",
    );
  }
  const rulePackIds = new Set<string>();
  for (const item of args.active_rule_packs) {
    if (
      !item.rule_pack_id?.trim() ||
      !item.version?.trim() ||
      !item.source_notice?.trim() ||
      !SHA256_HEX.test(item.checksum_sha256) ||
      rulePackIds.has(item.rule_pack_id)
    ) {
      throw new Error(
        "INPUT-MANIFEST-RULE-PACKS-INCOMPLETE: active rule packs require unique IDs, versions, notices, and exact SHA-256.",
      );
    }
    rulePackIds.add(item.rule_pack_id);
  }
  if (!Array.isArray(args.external_assets)) {
    throw new Error(
      "INPUT-MANIFEST-ASSETS-INCOMPLETE: external asset inventory is required.",
    );
  }
  const assetRefs = new Set<string>();
  for (const item of args.external_assets) {
    if (
      !item.asset_ref?.trim() ||
      !item.media_type?.trim() ||
      !SHA256_HEX.test(item.checksum_sha256) ||
      assetRefs.has(item.asset_ref)
    ) {
      throw new Error(
        "INPUT-MANIFEST-ASSETS-INCOMPLETE: external assets require unique refs, media types, and exact SHA-256.",
      );
    }
    assetRefs.add(item.asset_ref);
  }
}

function isCompleteJson(value: unknown): boolean {
  if (value === undefined || typeof value === "function") return false;
  if (typeof value === "number") return Number.isFinite(value);
  if (Array.isArray(value)) return value.every(isCompleteJson);
  if (value && typeof value === "object") {
    return Object.entries(value).every(
      ([key, nested]) => key.trim().length > 0 && isCompleteJson(nested),
    );
  }
  return value !== null;
}

function sortedRecord(value: Record<string, string>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(value).sort(([left], [right]) => left.localeCompare(right)),
  );
}

function safeToken(value: string): string {
  return (
    value.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") ||
    "current-session"
  );
}

function inputManifestRef(modelRef: string, digest: string): string {
  return `input-manifest:${safeToken(modelRef)}:${digest}`;
}
