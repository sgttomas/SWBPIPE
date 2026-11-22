// The words of the Rules page, written once.
//
// A person meets a rule pack through the product's words, not the format's
// field names and tokens. Every label, option and part heading the rule-pack
// editors show is read from this module, and the tests read the same tables,
// so a word is written in one place. The format's own name of each field and
// token stays findable as the control's title (its "format name"), for the
// person who writes a pack as a file.
//
// The words say what the format and the code that reads it say, and no more
// (schemas/rule_pack.schema.yaml; core/rules). A token whose effect the format
// does not state is given a plain reading of its own name, not a meaning.

export type Words = Readonly<Record<string, string>>;

/** The visible word for a format token, or the token itself when it has none. */
export function wordFor(words: Words, token: string): string {
  return words[token] ?? token;
}

/** Every unknown-value placeholder ("TBD") reads the same. */
export const NOT_CHOSEN = "Not chosen";

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

export const PAGE_WORDS = {
  title: "Rule packs",
  scope: "Kept on this computer and private to you.",
  noProject: "Rule packs are kept with a project: create or open a local project first.",
  newPack: "New rule pack",
  listEmpty: "No rule packs saved in this project yet.",
  listUnavailable: "Saved rule packs are read by the desktop app; this browser build keeps a pack in memory only.",
  open: "Open",
  delete: "Delete",
  usedByModel: "used by this model",
  notSaved: "not saved",
  unsavedChanges: "changes not saved",
  // Save controls, in the order a person presses them.
  validate: "Check",
  checksum: "Stamp checksum",
  save: "Save",
  discard: "Discard",
  document: "Document (JSON)",
  details: "Details",
  noFaults: "No faults found.",
  staleFindings: "Changed since the last check.",
  notYetChecked: "Not checked yet.",
  jsonInvalid: "The document below is not valid JSON, so the forms cannot read it.",
  checksumHeading: "Checksum"
} as const;

/** "N things to fix" for a count of blocking findings. */
export function thingsToFix(count: number): string {
  return count === 1 ? "1 thing to fix" : `${count} things to fix`;
}

// The lifecycle a stored pack declares (schema RulePackMetadata.lifecycle_status).
export const LIFECYCLE_WORDS: Words = {
  draft: "draft",
  review_pending: "awaiting review",
  accepted_private: "accepted (private)",
  accepted_public_example: "accepted (public example)",
  rejected: "rejected",
  quarantined: "quarantined",
  TBD: "state not stated"
};

// ---------------------------------------------------------------------------
// The parts of an open pack, each a closed line with a summary
// ---------------------------------------------------------------------------

export const PART_WORDS = {
  loadCases: "Load cases this pack needs",
  categories: "Stress categories",
  inputs: "Inputs and limits",
  formula: "Formula",
  checks: "Checks",
  inputsHeading: "Inputs",
  limitsHeading: "Limits",
  checksHeading: "Checks",
  markedIncomplete: "marked incomplete"
} as const;

/** "Heading: count" as a part's closed summary reads it. */
export function countLine(heading: string, count: number): string {
  return `${heading}: ${count}`;
}

// ---------------------------------------------------------------------------
// Field labels: format name -> words
// ---------------------------------------------------------------------------

export const FIELD_WORDS: Words = {
  // shared
  name: "Name",
  description: "Description",
  dimension: "Quantity",
  unit_ref: "Unit",
  required_for: "Needed for",
  completeness_status: "Stated completeness",
  // load cases and categories
  required_load_case_id: "Identifier",
  basis_ref: "Model load case or combination",
  "basis ref_type": "Kind",
  "basis ref_id": "Model's identifier",
  category_id: "Identifier",
  required_load_case_refs: "Checked on these load cases",
  // inputs
  input_id: "Identifier",
  source_kind: "Value comes from",
  library_kind: "Library",
  library_id: "Library identifier",
  record_id: "Record",
  "library slot_id": "Value in the record",
  result_kind: "Result kind",
  entity_ref: "Element",
  location: "Location",
  component: "Component",
  section_basis: "Section basis",
  tie_rod: "Tie rod",
  solver_offer: "Pick from the solved result",
  // limits
  slot_id: "Identifier",
  slot_kind: "Kind",
  value_status: "Declared value status",
  // checks
  check_id: "Identifier",
  category_ref: "Stress category",
  formula_ref: "Formula",
  required_input_refs: "Inputs used",
  value_slot_refs: "Limits used",
  acceptability_basis: "Basis",
  acceptability_relation: "Passes when the result is",
  result_statuses: "Outcomes it may report",
  diagnostic_policy: "Messages"
};

/** The visible label of a format field. */
export function fieldWord(formatName: string): string {
  return wordFor(FIELD_WORDS, formatName);
}

// ---------------------------------------------------------------------------
// Option words: format token -> words
// ---------------------------------------------------------------------------

export const BASIS_TYPE_WORDS: Words = { load_case: "Load case", combination: "Combination" };

// RequiredInput.source_kind. model_input, owner_design_basis and TBD are all
// taken from a value entered when the check runs (rule_check_runner); none is
// read from the model, so each says so.
export const SOURCE_KIND_WORDS: Words = {
  solver_result: "Solved result",
  model_input: "Model input, entered when checking",
  user_supplied_rule_value: "Entered when checking",
  private_library_value: "Private library",
  owner_design_basis: "Owner's design basis, entered when checking",
  TBD: "Not chosen, entered when checking"
};

// required_for: only rule_check inputs are tested for completeness.
export const REQUIRED_FOR_WORDS: Words = {
  rule_check: "The check",
  reporting: "Reporting",
  diagnostic: "Diagnostics",
  TBD: NOT_CHOSEN
};

export const COMPLETENESS_WORDS: Words = {
  complete: "Complete",
  missing_required_value: "Value missing",
  missing_unit: "Unit missing",
  missing_provenance: "Source missing",
  missing_redistribution_status: "Sharing terms missing",
  review_pending: "Awaiting review",
  TBD: NOT_CHOSEN
};

export const SLOT_KIND_WORDS: Words = {
  allowable: "Allowable",
  stress_limit: "Stress limit",
  ratio_limit: "Ratio limit",
  coefficient: "Coefficient",
  category_threshold: "Category threshold",
  source_reference: "Source reference",
  TBD: NOT_CHOSEN
};

export const VALUE_STATUS_WORDS: Words = {
  not_provided: "Not provided",
  private_user_supplied: "Private, yours",
  public_permissive_reviewed: "Public, reviewed",
  invented_non_engineering_example: "Invented example",
  protected_suspected: "Suspected protected",
  TBD: NOT_CHOSEN
};

export const LIBRARY_KIND_WORDS: Words = {
  material: "Material",
  section: "Section",
  component: "Component"
};

export const ACCEPTABILITY_BASIS_WORDS: Words = {
  user_supplied_rule_pack: "Your rule pack",
  owner_design_basis: "Owner's design basis",
  private_project_basis: "Private project basis",
  invented_non_engineering_example: "Invented example",
  TBD: NOT_CHOSEN
};

// CheckDefinition.acceptability_relation: passes when computed <relation> limit.
export const ACCEPTABILITY_RELATION_WORDS: Words = {
  less_than: "below the limit",
  less_than_or_equal: "at most the limit",
  greater_than: "above the limit",
  greater_than_or_equal: "at least the limit"
};

// AnalysisStatus. A check reports only the first three (rule_check_runner);
// "Fails" rather than "exceeds", since a minimum or a true/false check fails
// without exceeding anything.
export const ANALYSIS_STATUS_WORDS: Words = {
  USER_RULE_CHECKED: "Passes",
  USER_RULE_FAILED: "Fails",
  RULE_INPUTS_INCOMPLETE: "Inputs incomplete",
  HUMAN_REVIEW_REQUIRED: "Needs human review",
  MODEL_INCOMPLETE: "Model incomplete",
  MECHANICS_SOLVED: "Solved"
};

// DiagnosticPolicy: the condition each message is for.
/** The two halves of a check's outcomes and of its messages. */
export const CHECK_GROUP_WORDS = {
  reportedOutcomes: "Outcomes a check reports",
  keptOutcomes: "Kept with the pack; a check never reports these",
  usedMessages: "Messages a check shows",
  keptMessages: "Kept with the pack; not used when checking"
} as const;

export const DIAGNOSTIC_POLICY_FIELD_WORDS: Words = {
  rule_check_blocking: "Check blocked",
  missing_input: "Input missing",
  unit_mismatch: "Units differ",
  provenance_gap: "Source missing",
  redistribution_gap: "Sharing terms missing",
  checksum_mismatch: "Checksum differs",
  protected_content_suspected: "Suspected protected content",
  evaluator_error: "Formula cannot be evaluated"
};

// RulePackDiagnosticCode: the message shown.
export const DIAGNOSTIC_CODE_WORDS: Words = {
  RULE_CHECK_BLOCKING: "Check blocked",
  RULE_INPUT_MISSING: "Input missing",
  RULE_UNIT_MISMATCH: "Unit mismatch",
  RULE_PROVENANCE_WARNING: "Source warning",
  RULE_REDISTRIBUTION_WARNING: "Sharing warning",
  RULE_CHECKSUM_MISMATCH: "Checksum mismatch",
  RULE_PROTECTED_CONTENT_WARNING: "Protected content warning",
  RULE_EVALUATOR_ERROR: "Formula error",
  RULE_INCOMPLETE_DATA: "Incomplete data",
  RULE_PROFESSIONAL_BOUNDARY_NOTICE: "Professional boundary notice"
};

// DimensionId, as an engineer says each quantity.
export const DIMENSION_WORDS: Words = {
  dimensionless: "dimensionless",
  length: "length",
  mass: "mass",
  time: "time",
  temperature: "temperature",
  temperature_interval: "temperature interval",
  angle: "angle",
  rotation: "rotation",
  force: "force",
  moment: "moment",
  pressure: "pressure",
  stress: "stress",
  area: "area",
  volume: "volume",
  density: "density",
  linear_stiffness: "linear stiffness",
  rotational_stiffness: "rotational stiffness",
  displacement: "displacement",
  velocity: "velocity",
  acceleration: "acceleration",
  thermal_conductivity: "thermal conductivity",
  specific_heat: "specific heat",
  thermal_expansion_coefficient: "thermal expansion coefficient",
  second_moment_area: "second moment of area",
  section_modulus: "section modulus",
  mass_per_length: "mass per length",
  volume_per_length: "volume per length",
  slope: "slope",
  TBD: NOT_CHOSEN
};

// ---------------------------------------------------------------------------
// The formula (expression composer)
// ---------------------------------------------------------------------------

export const NODE_KIND_WORDS: Words = {
  literal: "Number",
  variable_ref: "Input or limit",
  unary: "Sign, absolute or not",
  binary: "Arithmetic",
  compare: "Comparison",
  logical: "And / or",
  select: "If, then, otherwise",
  aggregate: "Smallest or largest of",
  interpolate: "Interpolate in a table",
  lookup: "Look up in a table"
};

export const OPERATOR_WORDS: Words = {
  negate: "negative of",
  abs: "absolute value of",
  not: "not",
  add: "plus",
  subtract: "minus",
  multiply: "times",
  divide: "divided by",
  less_than: "less than",
  less_than_or_equal: "at most",
  greater_than: "greater than",
  greater_than_or_equal: "at least",
  equal: "equal to",
  not_equal: "not equal to",
  and: "and",
  or: "or",
  min: "smallest",
  max: "largest",
  exact: "exact row",
  step: "step"
};

// The place of a child expression inside its parent.
export const CHILD_WORDS: Words = {
  expression: "Formula",
  operand: "Of",
  left: "First",
  right: "Second",
  condition: "If",
  then: "Then",
  else: "Otherwise",
  argument: "Argument"
};

export const COMPOSER_FIELD_WORDS: Words = {
  node: "Kind",
  value: "Value",
  dimension: "Quantity",
  unit_ref: "Unit",
  variable: "Input or limit",
  operator: "Operation",
  function: "Which",
  mode: "Match",
  table_id: "Table name",
  argument_dimension: "Argument quantity",
  argument_unit_ref: "Argument unit",
  result_dimension: "Result quantity",
  result_unit_ref: "Result unit",
  argument: "Argument",
  result: "Result",
  formula: "Formula",
  readsAs: "Reads as"
};

export const FORMULA_KIND_WORDS: Words = {
  derived_quantity: "derived quantity"
};

/** The words for a child at `label`, keeping its number ("operand 2" -> "Of 2"). */
export function childWord(label: string): string {
  const match = /^(\w+)(?: (\d+))?$/.exec(label);
  if (!match) return label;
  // An aggregate's numbered operands are its values: "operand 2" -> "Value 2".
  if (match[1] === "operand" && match[2]) return `Value ${match[2]}`;
  const word = CHILD_WORDS[match[1]];
  if (!word) return label;
  return match[2] ? `${word} ${match[2]}` : word;
}
