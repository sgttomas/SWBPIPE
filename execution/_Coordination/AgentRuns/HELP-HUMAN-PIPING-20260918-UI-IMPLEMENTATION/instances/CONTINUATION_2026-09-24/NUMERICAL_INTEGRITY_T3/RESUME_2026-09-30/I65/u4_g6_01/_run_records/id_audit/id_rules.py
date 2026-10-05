"""The id-audit provenance rules (I65 U4 G6). Each rule matches a candidate by its site file, its
copied expression and the BINDING of that expression's base (the fn parameter with its callers,
the for/let/closure pattern and what it iterates); the identifier's spelling alone never decides.
A rule gives the source class and its reason. Order matters: the first match wins."""
import re
P = "product_physics/src/"
IN_ROOT = (r"model\.(nodes|pipe_segments|supports|load_cases|materials|project)|&model\b|input\.model|"
           r"case\.primitive_loads|load_case\.primitive_loads|PreviewLoadCase|PreviewSupport|PreviewNode|"
           r"PreviewPipe|MaterialInput|materials\b|material_map\.get|pipe_map\.get|bracket\[|"
           r"temperature_points|effective_case|load_state")
RULES = [
    # ---- template-bound (handled with their template sizes in id_audit.py) ----
    ("TPLSITE", lambda c: c["short"] in ("lib.rs:13044", "lib.rs:13074", "lib.rs:11617", "lib.rs:11649") and c["base"] == "id",
     "a format!-built id passed by the result builders (own template bound)"),
    ("TPLSITE", lambda c: c["short"] in ("lib.rs:4624", "lib.rs:5276", "lib.rs:5281", "lib.rs:5314", "lib.rs:5329", "lib.rs:5359") and c["base"] == "result_id",
     "a format!-built result id local (own template bound)"),
    ("TPLSITE", lambda c: c["a"] == "projection.projection_id",
     "Projection.projection_id = format!(\"projection:{id}\") (rows.rs:294)"),
    # ---- diagnostic ids ----
    ("DIAG", lambda c: c["base"] == "record" and "record: &Diagnostic" in c["b"], "ordinary_report(record: &Diagnostic): a Diagnostic id"),
    ("DIAG", lambda c: c["short"] == "lib.rs:1916" and c["base"] == "d", "evidence = diagnostics.iter().find(..) (lib.rs:1847): a Diagnostic id"),
    ("DIAG", lambda c: c["base"] == "diagnostic_ref", "the D5 seed's diagnostic_ref: the id of a diagnostic actually pushed"),
    # ---- result ids ----
    ("RES", lambda c: re.fullmatch(r"(row|r|actual|result)\.id", c["a"]) and re.search(
        r"ResultItem|rows|results|\*actual|qualify\(|maximum_row|solve\.results|observed|row\.value|best|derived", c["b"]),
     "a ResultItem id (the binding iterates/receives ResultItems)"),
    ("RES", lambda c: c["a"] in ("matched[0].id",) or c["a"].startswith("primary[") or c["a"].endswith("]].id") or c["a"] == "d.row.id",
     "a ResultItem id (matched: Vec<&ResultItem>; primary: &BTreeMap<usize, ResultItem>; Derived.row: ResultItem)"),
    ("RES", lambda c: c["a"] in ("binding.result_id", "r.result_id"), "FunctionalRowBinding/RowTreatment.result_id: a copy of a ResultItem id (lib.rs:5592; source_receipt.rs:867)"),
    ("RES", lambda c: c["a"].endswith("result_ref"), "LocatedQuantity.result_ref: a ResultItem id"),
    ("RES", lambda c: c["a"] == "r.source_result_refs", "ResultItem.source_result_refs: ResultItem ids"),
    ("RES", lambda c: re.match(r"text\(&(r|row)\[", c["a"]), "a results-JSON row id read back by the reader"),
    # ---- composites ----
    ("COMP", lambda c: re.match(r"(identity|exact_source_identity)\(", c["a"]) or c["a"] in ("id_location", "station_id_location(loc)", "result_tail(base_id)", "idloc")
     or c["base"] in ("basis_key", "basis_record"),
     "a length-prefixed identity/location of at most 3 input ids or static parts, or its tail: the 600-B composite class"),
    # ---- Display impls of error/report enums (multiplicity 1): field names are &'static str literals or input ids ----
    ("FMTSTATIC", lambda c: c["fn"].startswith("fn fmt(") and not re.search(r"\.len\(\)$", c["a"]) and c["base"] not in ("dof",),
     "a Display field of an error enum: a &'static str literal or an input id (priced at the static class, 512 B)"),
    ("NUM", lambda c: c["fn"].startswith("fn fmt(") and c["base"] == "dof", "a DOF index (usize)"),
    # ---- numbers ----
    ("NUM", lambda c: c["a"].endswith(".len()") or (c["base"] == "dof" and re.search(r"usize|for &dof|global_dof|\*dof", c["b"] + c["a"])),
     "an integer (length or DOF index)"),
    # ---- entity refs and their suffixes ----
    ("IN128", lambda c: "entity_ref" in c["a"] or (c["base"] == "suffix" and "row.entity_ref" in c["b"]),
     "a ResultItem.entity_ref: every assignment copies an input element/node/support/case id"),
    ("IN128", lambda c: c["base"] == "entity" and ("entity: &str" in c["b"] or "fn provenance_diag" in c["b"]), "validation entity labels: callers pass literal entity kinds or input ids"),
    # ---- input-rooted ----
    ("IN128", lambda c: re.search(IN_ROOT, c["b"]), "an input model/request string field (the binding iterates or borrows the input model)"),
    ("IN128", lambda c: c["base"] in ("pipe",) and re.search(r"built\.pipes|input$|\(pipe, frame\)", c["b"]),
     "StraightPipeElement.element_id: built from the input pipe id (lib.rs build_preview_model)"),
    ("IN128", lambda c: c["base"] in ("load_case_id", "case_id", "pipe_id", "support_id", "node_id", "load_id", "material_id", "field_path", "point_id", "basis_ref")
     and re.search(r"load_case\.id|case\.id|\.load_case_id|: &str|\.as_deref|\.id\.clone|case_id\.into|\(case_id, load_id, support_id\)|point\.id|solve\.load_case_id|stable_suffix\(", c["b"]),
     "an input id passed down (the parameter's callers pass input case/pipe/support/node/load/material ids or their stable_suffix)"),
    ("IN128", lambda c: c["base"] in ("load", "l") and re.search(r"PrimitiveLoad|nodal_loads|filter_map|primitive_loads|model", c["b"]),
     "a primitive/nodal load's id or category: copied from the input load"),
    ("IN128", lambda c: c["base"] in ("support", "spring", "finding", "s", "m", "member", "slot", "bend", "record", "solve", "t", "term", "authored", "node", "point", "lower", "upper", "pipe_input", "material", "case", "input", "self")
     and not re.search(r"ResultItem|Diagnostic", c["b"]),
     "a solver/record field copied from an input id (support/spring/load ids, member pipe ids, load-case ids, node ids, material and point ids)"),
    ("IN128", lambda c: c["base"] == "id" and re.search(r"for id in ids|fn provenance_diag|support_ids|member_ids|\.map\(\|\(index, id\)\||\(id, dof\)|\(id, node\)|\.map\(\|id\| id\.to_string|functional_id", c["b"]),
     "an input id (support/member id lists; provenance diagnostics' entity ids; functional ids of an input case)"),
    # ---- static literal tables in the same fn ----
    ("STATIC", lambda c: c["base"] in ("kind", "component", "unit", "location", "id_tail", "name", "family", "field", "dimension", "part", "entry", "quantity", "suffix", "source", "subject_id", "element_id")
     and re.search(r"for \(|let \w+ = match|\[\"|: &str|: &'static|kind\.into|unit\.into|component\.to_string|kind\.to_string|unit\.to_string|location\.to_string|Dimension|\.map\(\|entry\||\.map\(\|part\||let suffix|fn error|g1\(|fn project|for \(position, source\)|load_ledger|subject_id\.into|element_id\.into|let mut append|\|component: &str|let component = match|let kind = |if let Some\(family\)|let Some\(family\)|fn name|fn code", c["b"] + " " + c["fn"]),
     "a string literal or an entry of a literal table/match in the same function (each entry < 128 B), or a &'static str parameter"),
]

# ---- manual resolutions (each read at the site; see ID_CLASS_AUDIT.md) ----
MANUAL = {
    ("retained_product.rs:2416", "idloc"): ("STATIC", "idloc: the End branch's literal location or station_id_location(metadata.location) (lib.rs:13099), a literal location label"),
    ("retained_product.rs:3267", "text"): ("IN128", "prepared_string(text): its one caller passes &x.source_id, an input load id (already priced at 1,024, kept)"),
    ("retained_product.rs:691", "text"): ("COMP", "observation_copy(text): callers pass &case.id (input) and the mode/parity rows' metadata.basis (a basis label, composite class; already priced at 1,024, kept)"),
    ("source_receipt/rows.rs:292", "id"): ("TPL_FID", "let id = source::functional_id(case_id, index) = format!(\"source-functional:{}:{case}:{index}\", case.len()) (source_receipt/source.rs:27; the run's bound 188 B)"),
    ("lib.rs:1763", "entry"): ("TPL_ENTRY", "range_scaling_evidence_line's entries are the format! results at lib.rs:1699-1753 (the run's largest, 8,241 B at :1708)"),
    ("lib.rs:1061", "dof"): ("STATIC", "a DOF name from the literal array [\"UX\",...,\"RZ\"]"),
    ("lib.rs:1660", "pipe.element_id"): ("IN128", "built.pipes element ids: copies of input pipe ids"),
    ("lib.rs:1671", "pipe.element_id"): ("IN128", "built.pipes element ids: copies of input pipe ids"),
    ("lib.rs:1812", "family"): ("STATIC", "ForceScalingFailure::NotAdmitted.family: &'static str (lib.rs:1400)"),
    ("lib.rs:1817", "quantity"): ("TPL145", "ForceScalingFailure::Publication.quantity: \"reaction\", \"reaction@\"/\"spring_action@\" + integrity_dof_label (node id + ':' + DOF name, <= 131) or \"end_actions@\" + element id (lib.rs:1630-1660): <= 14 + 131 = 145 B"),
    ("lib.rs:3949", "stable_suffix(&pipe_id)"): ("IN128", "load_state_eigen_loads' Err(pipe_id): an input pipe id"),
    ("lib.rs:5591", "entity"): ("IN128", "source_row_bindings(entity): an input element/node/support id"),
    ("lib.rs:5591", "kind"): ("STATIC", "source_row_bindings(kind): a literal result kind"),
    ("lib.rs:5591", "location"): ("STATIC", "source_row_bindings(location): a literal metadata location"),
    ("lib.rs:6178", "direct_system.dimension"): ("NUM", "a matrix dimension (usize)"),
    ("lib.rs:9591", "basis_ref"): ("IN128", "the input load case's modulus_basis_ref"),
    ("pressure_runtime.rs:92", "stable_suffix(&refs.join(\":\"))"): ("TPL148", "problem(refs): callers pass [\"pressure_contract\"], [&component.id, \"objective_connector\"] or [&component.id] (pressure_runtime.rs:115-172): joined <= 128 + 1 + 19 = 148 B"),
    ("pressure_runtime.rs:94", "refs"): ("IN128", "the same <= 2 refs, each an input component id or a literal (refs_vec class unchanged)"),
    ("retained_product.rs:1940", "r.kind"): ("STATIC", "ResultItem.kind: always a literal kind"),
    ("retained_product.rs:2332", "*"): ("STATIC", "an entry of a literal component array"),
    ("retained_product.rs:2351", "*"): ("STATIC", "an entry of a literal component array"),
    ("source_receipt.rs:221", "pipe"): ("IN128", "the captured eigen member's pipe id (an input pipe id)"),
    ("validation.rs:1478", "quantity.unit"): ("IN128", "an input quantity's unit text"),
    ("loads/primitive_loads/src/lib.rs:1242", "load_id"): ("IN128", "LoadFinding::new(load_id): callers pass input load ids"),
    ("loads/primitive_loads/src/lib.rs:907", "load_id"): ("IN128", "PrimitiveLoad::nodal_force(load_id): the input load id"),
    ("loads/primitive_loads/src/lib.rs:924", "load_id"): ("IN128", "PrimitiveLoad::uniform_element_load(load_id): the input load id"),
    ("lib.rs:14032", "id"): ("IN128", "stable_suffix(id) = id.replace(':', \"-\"): every caller passes an input id, a case id or a row entity_ref (id_callers.txt); same length"),
    ("lib.rs:2759", "row.id"): ("RES", "solve.results rows: ResultItem ids"),
    ("lib.rs:2761", "row.id"): ("RES", "solve.results rows: ResultItem ids"),
    ("lib.rs:2816", "maximum.location_ref"): ("IN128", "LocatedQuantity.location_ref = a row entity_ref (an input id)"),
    ("lib.rs:7979", "model.project.id"): ("IN128", "the input project id"),
    ("lib.rs:9864", "lower_e.unit"): ("IN128", "an input material quantity unit"),
    ("lib.rs:9868", "lower_g.unit"): ("IN128", "an input material quantity unit"),
    ("lib.rs:9872", "lower_alpha.unit"): ("IN128", "an input material quantity unit"),
    ("lib.rs:984", "solver_component_name()"): ("STATIC", "env!(\"CARGO_PKG_NAME\")"),
    ("lib.rs:985", "solver_component_version()"): ("STATIC", "env!(\"CARGO_PKG_VERSION\")"),
    ("preview_physics.rs:1028", "case_id"): ("IN128", "the evidence JSON's case id: the input load case id"),
    ("preview_physics.rs:563", "support_id"): ("IN128", "support_dispositions(model, ..) keys: input support ids"),
    ("preview_physics.rs:568", "support_id"): ("IN128", "support_dispositions(model, ..) keys: input support ids"),
    ("preview_physics.rs:690", "row.unit"): ("STATIC", "ResultItem.unit: always a literal unit"),
    ("retained_wire.rs:1545", "source"): ("NOTID", "a serde_json Value clone (the case source tree), priced by the publication forms, not text"),
    ("source_receipt.rs:739", "r.id"): ("RES", "ordinary(): rows of the envelope: ResultItem ids"),
    ("source_receipt.rs:781", "r.id"): ("RES", "failed(): rows of the envelope: ResultItem ids"),
    ("source_receipt/rows.rs:24", "input.load_case.id"): ("IN128", "the input load case id"),
    ("source_recovery.rs:1400", "row.key"): ("NOTID", "FunctionalDescriptor.key: its own 320-B class (G4), unchanged"),
    ("validation.rs:1031", "stiffness.dof"): ("IN128", "an input stiffness DOF text"),
    ("validation.rs:1911", "load.id"): ("IN128", "an input load id (filtered input loads)"),
    ("validation.rs:1924", "load.id"): ("IN128", "an input load id (filtered input loads)"),
    ("reporting/result_export/src/retained_precision.rs:285", "source"): ("NOTID", "a serde_json Value clone in the reader's normalize (priced by T17), not text"),
    ("reporting/result_export/src/retained_precision.rs:2613", "*"): ("IN128", "the successor's id_maps node id / constraint component: an input node id or a literal"),
    ("reporting/result_export/src/retained_precision.rs:3233", "*"): ("IN128", "input_derived_dofs node_id: an input node id"),
    ("reporting/result_export/src/retained_precision.rs:3234", "*"): ("STATIC", "input_derived_dofs component: a literal DOF name"),
    ("solver/frame_kernel/src/load_ledger.rs:133", "source"): ("IN128", "ForceTerm.source: a load id (its site_total rule, 5*l*2*128, unchanged)"),
    ("solver/frame_kernel/src/structural/retained/adaptive.rs:4854", "*"): ("NOTID", "StopDecision summaries: their composite class, unchanged (not identifiers)"),
    ("solver/frame_kernel/src/structural/retained/adaptive.rs:4898", "*"): ("NOTID", "StopDecision summaries (not identifiers)"),
    ("solver/frame_kernel/src/structural/retained/adaptive.rs:4899", "*"): ("NOTID", "StopDecision summaries (not identifiers)"),
    ("solver/linear_supports/src/lib.rs:190", "support_id"): ("IN128", "LinearSupport::spring(support_id): the input support id"),
    ("solver/linear_supports/src/lib.rs:252", "support_id"): ("IN128", "SupportFinding::new(support_id): an input support id"),
}
