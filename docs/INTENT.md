# INTENT

## Purpose of this document

This file records the intent behind the proposed free and open source piping stress analysis software discussed in the conversation leading to the Product Requirements Document (`open_piping_stress_prd.md`). It is intended to preserve the governing philosophy of the project so future design, implementation, governance, and contribution decisions remain aligned with the original objective.

## Core intent

Build a **free and open source piping stress analysis platform** that implements the best-known open analytical methods for global piping flexibility and stress analysis while **respecting ASME copyrights and other standards-body intellectual property**.

The software should provide the engineering workflow, modeling environment, solver, reporting tools, and user interface needed for piping stress analysis, but it should not redistribute proprietary code text, code tables, code-derived formulas, copyrighted examples, material allowable tables, stress-intensification-factor tables, flexibility-factor tables, or protected dimensional standards.

The responsible engineer or organization must provide the code-specific and project-specific information needed to complete a code check.

## Foundational technical premise

The project is grounded in the classical piping flexibility and stress-analysis tradition represented by **M. W. Kellogg, _Design of Piping Systems_ (1956)**, especially the treatment of:

- material behavior and design limits;
- pressure, weight, thermal expansion, displacement, sustained, occasional, and repeated-load behavior;
- local component effects in elbows, bends, branches, reducers, flanges, valves, nozzles, and expansion joints;
- simplified flexibility methods for hand and preliminary analysis;
- the general analytical method for multi-plane piping systems, branches, restraints, terminal reactions, forces, moments, and stresses.

The intent is **not** to reproduce Kellogg mechanically or limit the software to 1956 methods. The intent is to preserve the key insight that modern piping stress analysis is fundamentally a **centerline / beam / flexibility problem** for the vast majority of piping systems, refined by decades of later theory, better numerical methods, improved component data, better material data, and modern computing.

The modern implementation should therefore use a robust **3D frame finite-element solver** as the computational successor to the classical flexibility-equation approach. The global piping system is modeled as connected line elements with six degrees of freedom per node, not as a full 3D solid mesh. Solid or shell FEA belongs only in specialized local analyses where the normal beam-based pipe stress model is insufficient.

## Product identity

The product should be understood as:

> An open, auditable, code-neutral piping flexibility and stress-analysis platform that performs the mechanics and lets the user supply the governing code data, material properties, component data, and rule checks.

It should not be represented as:

> An open-source copy of ASME B31.1, ASME B31.3, ASME B31J, ASME B16, ASME B36, or any other copyrighted standard.

It should not claim to be ASME-approved, ASME-certified, officially code-compliant, or endorsed by any standards body.

## Copyright-respecting design principle

The public project should ship **open mechanics and empty or non-code examples**, not protected standards content.

The project may ship:

- a 3D centerline piping modeler;
- an analytical solver;
- section-property calculations from user-entered dimensions;
- unit handling;
- load-case management;
- stress-recovery calculations based on general mechanics;
- a graphical user interface;
- a rule-pack schema;
- a sandboxed, unit-aware rule evaluator;
- private-library import mechanisms;
- reporting and audit tools;
- validation examples using original, public-domain, permissively licensed, or invented data.

The public project should not ship:

- ASME code text;
- ASME tables;
- ASME figures;
- ASME examples;
- ASME material allowable tables;
- B31J stress-intensification-factor or flexibility-factor tables or equations copied from the standard;
- B16 valve, flange, fitting, pressure-temperature, or face-to-face dimensional tables copied from standards;
- B36 pipe dimensional tables copied from standards;
- proprietary commercial software examples or report templates;
- vendor catalog data unless redistribution is permitted and documented.

## User-supplied code and data layer

The system should require users to provide the code-specific and project-specific information needed to perform a code check. This includes, where applicable:

- material properties;
- allowable stresses;
- temperature-dependent modulus values;
- thermal expansion data;
- yield and ultimate strength values;
- joint factors;
- weld factors;
- quality factors;
- corrosion allowance;
- mill tolerance;
- pressure design assumptions;
- stress-intensification factors;
- flexibility factors;
- stress indices;
- load combinations;
- stress equations;
- allowable limits;
- cycle factors;
- occasional load factors;
- hydrotest limits;
- component dimensions;
- valve face-to-face dimensions;
- valve weights and centers of gravity;
- flange and fitting weights;
- expansion-joint spring rates and effective pressure areas;
- nozzle/equipment allowable loads;
- owner or company-specific requirements.

The software should make missing code data explicit. It may solve the global mechanics when enough physical data is present, but it should block or clearly qualify any code check when required code-specific values are missing.

## Rule-pack intent

The project should provide a **rule-pack system**, not bundled ASME rules.

A rule pack should be a user-supplied or organization-supplied data artifact that defines:

- the governing code or internal design basis;
- required inputs;
- stress categories;
- load combinations;
- formula expressions;
- allowable limits;
- pass/fail criteria;
- source notes;
- versioning;
- checksums;
- redistribution status.

Public example rule packs should use invented, non-code values and should be clearly marked as examples only.

Private rule packs may represent a licensed code edition, owner standard, or company practice, but those files are the responsibility of the user and should not be redistributed with the open source project unless the contributor has documented redistribution rights.

## Analytical engine intent

The solver should implement the best practical version of global piping stress theory using a modern 3D frame finite-element approach.

The engine should support:

- nodes with six degrees of freedom;
- straight pipe elements;
- bend and elbow elements or bend macro-elements;
- rigid elements;
- valves;
- reducers;
- flanges;
- branch connections;
- expansion joints;
- anchors;
- guides;
- line stops;
- vertical supports;
- spring hangers;
- one-way supports;
- gaps;
- friction;
- imposed anchor and nozzle displacements;
- weight loads;
- pressure effects;
- thermal expansion;
- hydrotest cases;
- wind, seismic, and occasional loads;
- multiple operating cases;
- load-case algebra;
- restraint reactions;
- equipment terminal loads;
- stress resultants;
- code-check inputs and outputs through user-defined rules.

The solver should calculate fundamental mechanics quantities such as:

- displacements;
- rotations;
- axial forces;
- shear forces;
- torsional moments;
- in-plane bending moments;
- out-of-plane bending moments;
- resultant bending moments;
- pressure thrust where modeled;
- support and restraint loads;
- equipment/nozzle loads;
- section properties;
- axial stress;
- bending stress;
- torsional shear stress;
- pressure membrane stresses;
- optional general-purpose equivalent stresses for engineering review.

The solver should not need to know whether the user is applying ASME B31.1, ASME B31.3, EN 13480, CSA Z662, an owner standard, or another design basis. The solver calculates the mechanical state; the user-supplied rule pack evaluates acceptability.

## Graphical user interface intent

The GUI should make piping stress modeling repeatable, transparent, and auditable. It should guide the engineer through:

1. creating a project;
2. selecting units;
3. defining design basis metadata;
4. importing or creating private rule packs;
5. defining materials;
6. defining pipe sections;
7. building 3D piping geometry;
8. inserting bends, valves, reducers, flanges, branches, expansion joints, and equipment connections;
9. assigning supports and restraints;
10. assigning pressures, temperatures, insulation, contents, and component weights;
11. defining primitive load cases;
12. defining combination cases;
13. solving the analytical model;
14. reviewing displacements, rotations, reactions, forces, and moments;
15. applying user-supplied rule checks;
16. generating an auditable calculation report.

The GUI should use warnings and blocking checks to distinguish between:

- data required to solve the model;
- data required to perform a code check;
- assumptions made by the user;
- missing source/provenance information;
- nonlinear convergence issues;
- questionable model conditions;
- values outside expected engineering ranges.

## Component library intent

The product may support component libraries, but public libraries must be handled conservatively.

The project should provide schemas and import tools for component data. The user or organization may maintain private libraries for valves, flanges, fittings, reducers, expansion joints, supports, and equipment connections.

Publicly distributed component libraries should include only data that is:

- original;
- public-domain;
- permissively licensed;
- vendor-authorized for redistribution;
- or otherwise lawfully redistributable.

Every component-library entry should include provenance:

- source;
- date;
- contributor;
- license or permission basis;
- verification status;
- whether the value is vendor-specific, measured, calculated, or standard-derived.

The software should not silently insert protected standard dimensions or weights. Where required data is missing, the GUI should require user input or documented import.

## Reporting intent

Reports should be transparent enough for engineering review and audit.

A report should include:

- software version;
- solver version;
- commit hash or build identifier;
- units;
- project coordinate system;
- analysis assumptions;
- model geometry;
- node table;
- element table;
- material table;
- component table;
- support table;
- load cases;
- load combinations;
- rule-pack name and checksum;
- source notes for user-supplied data;
- statement that code-specific values were user supplied;
- missing-data warnings;
- displacement results;
- rotation results;
- forces and moments;
- restraint reactions;
- equipment terminal loads;
- stress summaries;
- pass/fail ratios;
- governing locations;
- plots and deformed shapes;
- signoff or review block.

The report should not reproduce copyrighted code text unless the user independently inserts such text into a private report template and has the right to do so.

## Validation intent

The project should earn trust through transparent verification and validation rather than by claiming equivalence to a commercial tool or a copyrighted standard.

Validation should include:

- hand-calculated beam and frame benchmarks;
- original piping flexibility examples;
- public-domain or permissively licensed examples;
- unit tests for stiffness matrices, transformations, load vectors, stress recovery, and rule evaluation;
- nonlinear restraint test cases;
- regression tests for every release;
- documented tolerances;
- clear distinction between solver verification and code compliance.

Commercial software comparisons may be included only when the inputs and outputs can be lawfully published.

## Governance intent

The repository should maintain a strict intellectual-property policy.

Contributors should not submit copied content from ASME, API, MSS, ISO, EN, CSA, ASTM, commercial software manuals, vendor catalogs, or any other protected source unless redistribution rights are documented.

Contributions containing protected standards tables, figures, examples, or copied rule text should be rejected.

A contributor certification should be required for data and rule contributions, stating that the contributor has the right to submit the material under the project license.

## Product philosophy

The guiding philosophy is:

> Open source the mechanics. Let the responsible engineer supply the code basis.

The software should make the mechanics visible, repeatable, testable, and auditable. It should not hide proprietary standards content inside the executable or pretend that code compliance can be divorced from the licensed code edition, project basis, user-entered material data, and engineering judgment.

## Non-goals

The project is not intended to:

- replace ownership of the governing piping code;
- provide legal or professional engineering advice;
- certify code compliance automatically;
- distribute ASME content;
- duplicate commercial software databases;
- eliminate engineering judgment;
- replace local FEA where local shell or solid analysis is required;
- perform fabrication, inspection, examination, or testing compliance outside the defined analytical scope unless future modules are explicitly created for those purposes.

## Success definition

The project succeeds if it becomes a transparent, auditable, legally respectful, technically credible open platform for piping flexibility and stress analysis that:

- implements robust modern global piping stress mechanics;
- preserves the classical analytical foundation of piping stress analysis;
- supports practical engineering workflows through a GUI;
- allows private code and material data to be entered or imported by the user;
- keeps protected standards content out of the public repository;
- produces clear calculation reports;
- is validated against open benchmarks;
- gives engineers and organizations a trustworthy open alternative for piping stress analysis without infringing standards-body copyrights.
