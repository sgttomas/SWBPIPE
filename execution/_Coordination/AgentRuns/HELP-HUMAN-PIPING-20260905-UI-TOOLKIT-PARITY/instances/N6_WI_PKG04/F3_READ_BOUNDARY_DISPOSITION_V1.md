# F3 support-family read-boundary disposition v1 — source HELD

PKG04 / DEL0403; evidence-only during root N7 freeze. No product/contract source changes or dispatch. This is a requested compatibility repair proposal, not implementation acceptance. Basis current source hashes in F3_READ_BOUNDARY_MANIFEST_V1.json and N2 F3_SUPPORT_FAMILY_DISPOSITION_V1.md; parent approved prior anchor subset adapter remains intact.

## Evidence and recommendation

Adopt explicit canonical-token validation at the physical preview read boundary; do not introduce PascalCase aliases. Product fixtures contain anchor (2), guide (2), nonlinear (2), variable_spring_hanger (1), constant_effort_support (1), and absent-family (92) shape-matching records; see F3_FIXTURE_INVENTORY_V1.json. No PascalCase family appears in this inventoried set. This is evidence of known fixtures, not proof no user saved such a document.

Consumers establish exact anchor/line_stop/vertical_support matching in rigid_linear_support_from_preview; exact spring matching before linear spring emission; support_hanger_type recognizes variable_spring_hanger/spring_hanger/constant_effort_support through hanger type first, then family. nonlinear configuration is currently handled before linear/hanger emission. Guide is currently only the catch-all fallback, so explicit guide with six DOFs becomes Anchor and must be corrected.

PascalCase LineStop/VerticalSupport/Spring/Anchor/Guide are currently not faithfully consumed aliases. UI spelling does not establish mechanical meaning. A finite alias map could be separately adopted as a compatibility repair, but current evidence does not justify silently remapping existing loaded records now. No broad lowercase, trim or punctuation normalization. Preferred behavior is a blocking diagnostic with raw family token and supported canonical values, leaving source untouched for explicit correction through ordinary operation authoring.

## Exact proposed read-boundary token rule

Allowed explicitly present strings: anchor, guide, line_stop, vertical_support, spring, variable_spring_hanger, spring_hanger, constant_effort_support, nonlinear. No other strings, including empty, whitespace-only, padded canonical strings, PascalCase or typo variants. `None` retains prior inference; existing serde Option permits omitted or null family, so this proposal does not change null into a new error. Do not turn explicit invalid string into None.

Add a private lib.rs helper `validate_support_family_tokens(model, diagnostics)` and call immediately after current validate_model_inputs in run_linear_static_preview_with_mode, before the existing first has_blocking return, shared-section resolution, unit normalization, model build or solve. Iterate every support including nonlinear/hanger objects; an unknown family cannot escape through those precedence branches. Proposed deterministic code SUPPORT_FAMILY_INVALID, severity blocking, identity diagnostic:support:{stable_suffix(id)}:family, related entities support ID and family field. Message names raw token and exact canonical alternatives; no guessed replacement applied.

Add explicit `Some("guide") => SupportFamily::Guide` in rigid_linear_support_from_preview next to explicit anchor. Preserve anchor exact subset, line_stop/vertical_support, spring/hanger classification and the existing fallback for None. A six-DOF explicit Guide will then hit the existing solver InvalidSupportDof diagnostic for rotations rather than silently acting as Anchor; do not relax Guide mechanics or clip invalid DOFs. Explicit recognized non-rigid records continue through their existing consumer branch before rigid mapping.

## Branch coverage and compatibility boundaries

- Rigid: anchor retains all explicit subsets; guide now truly Guide; line_stop and vertical_support keep their existing DOF limits. Absent family still gets six=>Anchor / otherwise Guide.
- Spring: exact spring remains elastic; unsupported Spring blocks before it can become a rigid support. No stiffness/DOF default changes. Existing dimensional validation and explicit stiffness requirement remain owning consumer behavior.
- Variable hanger: preserve variable_spring_hanger and genuine spring_hanger alias; existing hanger_type trim/alias semantics unchanged. Constant-effort family and its opt-in/consumption warnings remain unchanged.
- Nonlinear: recognized family nonlinear with nonlinear data remains current route; recognized nonlinear behavior aliases one-way/oneway/lift-off/liftoff and DOF parser aliases are untouched. An unknown family combined with nonlinear data still blocks before mechanics.
- A missing family with existing hanger/nonlinear data retains established dispatch. This avoids making the new family check force a rigid inference over richer input.

Compatibility impact is deliberate: existing explicit invalid family strings now block rather than producing potentially wrong mechanics, even when another payload branch previously dominated. Existing explicit guide with rotational DOFs now reports its actual invalidity. Users must correct source explicitly; no automatic migration or native saved-value rewrite is authorized.

## Canonical-token validity versus payload-class consistency

Token validation alone does not certify a recognized family agrees with richer payload. Current established precedence is nonlinear data first, then hanger.hanger_type over family. Consequently recognized combinations such as family=spring plus a valid variable hanger are already consumed as hanger, while family=anchor plus nonlinear data is also overridden. Changing all recognized mixed-class records is a distinct compatibility policy, not necessary to fix explicit unknown-family fallthrough. For this narrow F3 read repair retain current precedence and do not invent a universal exclusivity matrix. N2 may enforce stricter new-authoring conflicts only through a parent-accepted exact matrix; that must not be described as existing solver policy. Expose this residual explicitly: a canonical token is spelling validity, not proof of class-consistent metadata. If parent requires consistency blocking at read boundary too, it needs a separate frozen matrix and compatibility tests before source release.

This is a scope split, not silence about the issue. Explicit family=nonlinear without nonlinear data and family/hanger type mismatches remain existing semantic contradictions to resolve under that matrix. They are not implemented as new mechanics or claimed solved by this token fix.

## Exact future source fence and tests (NOT RELEASED)

Only core/product_physics/src/lib.rs: early helper/call, explicit guide match arm and original focused tests. No validation.rs/solver/schema/operation/UI changes. Before write reverify source hash against current parent released N7 predecessor (not an old N6/N3 hash), archive accepted bytes, hold on mismatch. Preserve all N3 Section/N5 selfweight code byte-for-byte. Parent must release exclusive source ownership after freeze; no production dispatch now.

Focused tests: each canonical token accepted by helper; absent and null retain inference; invalid PascalCase/empty/padded/unknown strings produce SUPPORT_FAMILY_INVALID via public preview with no mechanical results; exercise invalid family on rigid, spring payload, variable/constant hanger and nonlinear payload so bypass is impossible. Explicit six-DOF guide maps Guide and raises existing rotational invalidity; absent six-DOF remains Anchor; named line_stop/vertical support and anchor subset regressions persist. Canonical spring has stiffness contribution rather than rigid restraint; Spring cannot. Existing spring_hanger alias and hanger data behavior unchanged; nonlinear/DOF genuine aliases pass. A helper test can isolate token validity from deliberately incomplete named data, but public valid fixtures must retain existing solver results.

Run focused and full product_physics crate tests with offline locked scratch target after release; no tests were run for this read-only proposal. Fresh independent100%-frozen-diff review, hash-bound source return and parent integrated checks required. Review product delta against accepted before bytes, not aggregate HEAD. No source/classification closure now.
