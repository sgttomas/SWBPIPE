//! Independent product-observation regressions. See README for frozen oracle provenance.
#[cfg(test)]
mod tests {
    use open_pipe_stress_product_physics::{
        run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
        PreviewSolverMode, ResultItem,
    };
    use serde_json::{json, Value};

    // Existing product publication precision, not an engineering tolerance.
    const ROUND6_ALLOWANCE: f64 = 0.5e-6 + 1e-10;
    const MODES: [PreviewSolverMode; 2] = [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ];
    fn close(actual: f64, expected: f64) {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= ROUND6_ALLOWANCE,
            "actual {actual:.15e}, independent expected {expected:.15e}"
        );
    }
    fn section_i() -> f64 {
        std::f64::consts::PI * (0.168_f64.powi(4) - 0.154_f64.powi(4)) / 64.0
    }
    fn point_tip_mm(force: f64) -> f64 {
        1000.0 * force * 2.0_f64.powi(3) / (3.0 * 200e9 * section_i())
    }
    fn nodal_case(id: &str, direction: &str, force: f64, dimension: &str, unit: &str) -> Value {
        json!({"id":id,"primitive_loads":[{"id":format!("load:{id}"),"category":"occasional",
            "target":{"type":"node","node":"node:tip"},"direction":direction,
            "magnitude":{"value":force,"unit":unit},"dimension":dimension,"provenance":"original analytical fixture"}]})
    }
    fn model(case: Value) -> Value {
        json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model",
            "project":{"id":"project:physics-audit","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"MPa"}},
            "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
            "nodes":[{"id":"node:root","position":{"x":0,"y":0,"z":0}},{"id":"node:tip","position":{"x":2,"y":0,"z":0}}],
            "pipe_segments":[{"id":"pipe:beam","from":"node:root","to":"node:tip","material":"material:original",
                "y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,"unit":"m"}}}],
            "materials":[{"id":"material:original","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":77e9,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":"invented; not a material standard"}],
            "supports":[{"id":"support:root","node":"node:root","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"]}],
            "load_cases":[case],"combinations":[]})
    }
    fn solve(mut model: Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
        // These records are authored here from invented analytical inputs.
        // The product requires explicit public provenance on each consumed record.
        for group in [
            "nodes",
            "pipe_segments",
            "supports",
            "materials",
            "load_cases",
            "combinations",
        ] {
            for record in model[group].as_array_mut().unwrap() {
                record["provenance"] = json!("invented_original_analytical_fixture");
                if let Some(loads) = record
                    .get_mut("primitive_loads")
                    .and_then(Value::as_array_mut)
                {
                    for load in loads {
                        load["provenance"] = json!("invented_original_analytical_fixture");
                    }
                }
                if let Some(generation) = record.get_mut("equivalent_static") {
                    generation["provenance"] = json!("invented_original_analytical_fixture");
                }
            }
        }
        let request = LinearStaticPreviewRequest {
            model: serde_json::from_value(model).unwrap(),
            materials: vec![],
        };
        let out = run_linear_static_preview_with_mode(request, mode);
        assert_eq!(
            out.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            out.diagnostics
        );
        assert!(out.results.iter().all(|r| r.value.is_finite()));
        out
    }
    fn rows<'a>(
        out: &'a MechanicsEnvelope,
        entity: &str,
        kind: &str,
        case: &str,
    ) -> Vec<&'a ResultItem> {
        out.results
            .iter()
            .filter(|r| {
                r.entity_ref == entity
                    && r.kind == kind
                    && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case)
            })
            .collect()
    }
    fn scalar(out: &MechanicsEnvelope, entity: &str, kind: &str, case: &str) -> f64 {
        let found = rows(out, entity, kind, case);
        assert_eq!(found.len(), 1, "missing/ambiguous {entity} {kind} {case}");
        found[0].value
    }
    fn station(out: &MechanicsEnvelope, kind: &str, location: &str, case: &str) -> f64 {
        let found: Vec<_> = rows(out, "pipe:beam", kind, case)
            .into_iter()
            .filter(|r| r.metadata.as_ref().is_some_and(|m| m.location == location))
            .collect();
        assert_eq!(
            found.len(),
            1,
            "missing/ambiguous station {kind} {location} {case}"
        );
        found[0].value
    }
    fn gap(dof: &str, size: f64) -> Value {
        json!({"id":"support:gap","node":"node:tip","family":"nonlinear","restraints":[],
            "nonlinear":{"behavior":"gap","dof":dof,"initial_state":"inactive","closes_when":"positive","gap":{"value":size,"unit":"m"}}})
    }

    #[test]
    fn actual_cantilever_recovery_and_force_norm() {
        for mode in MODES {
            let out = solve(
                model(nodal_case("P", "global_y", 350.0, "force", "N")),
                mode,
            );
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "P"),
                point_tip_mm(350.0),
            );
            close(
                scalar(&out, "support:root", "reaction_resultant", "P"),
                350.0,
            );
            close(
                station(&out, "element_local_bending_moment_z", "end_i", "P").abs(),
                700.0,
            );
            // Actual recovered field, not the old literal-only 6*10 witness.
            for (location, x) in [("quarter_1", 0.5), ("midspan", 1.0), ("quarter_3", 1.5)] {
                close(
                    station(&out, "element_local_bending_moment_z", location, "P").abs(),
                    350.0 * (2.0 - x),
                );
                close(
                    station(&out, "element_local_shear_force_y", location, "P").abs(),
                    350.0,
                );
            }
        }
    }
    #[test]
    fn pure_moment_does_not_become_force() {
        for mode in MODES {
            let out = solve(
                model(nodal_case("M", "global_z", 100.0, "moment", "N*m")),
                mode,
            );
            close(scalar(&out, "support:root", "reaction_resultant", "M"), 0.0);
            close(
                station(&out, "element_local_bending_moment_z", "midspan", "M").abs(),
                100.0,
            );
        }
    }
    #[test]
    fn inactive_gap_preserves_parallel_spring_and_its_action() {
        for mode in MODES {
            let mut input = model(nodal_case("S", "global_y", 350.0, "force", "N"));
            input["supports"].as_array_mut().unwrap().extend([
                json!({"id":"support:spring","node":"node:tip","family":"spring","restraints":[],"stiffness":{"dof":"UY","value":{"value":1e6,"unit":"N/m"}}}),gap("UY",1.0)]);
            let out = solve(input, mode);
            let u = 350.0 / (3.0 * 200e9 * section_i() / 2.0_f64.powi(3) + 1e6);
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "S"),
                1000.0 * u,
            );
            close(
                scalar(&out, "support:spring", "reaction_resultant", "S"),
                1e6 * u,
            );
            close(
                scalar(&out, "support:root", "reaction_resultant", "S"),
                350.0 - 1e6 * u,
            );
            close(
                station(&out, "element_local_bending_moment_z", "midspan", "S").abs(),
                350.0 - 1e6 * u,
            );
            close(
                scalar(
                    &out,
                    "support:gap",
                    "nonlinear_support_active_set_state_code",
                    "S",
                ),
                0.0,
            );
            close(
                scalar(
                    &out,
                    "support:gap",
                    "nonlinear_support_final_displacement",
                    "S",
                ),
                1000.0 * u,
            );
            close(
                scalar(&out, "support:gap", "nonlinear_support_final_reaction", "S"),
                0.0,
            );
            let end_j = station(&out, "element_local_shear_force_y", "end_j", "S");
            close(end_j, 350.0 - 1e6 * u);
            // Joint equilibrium: known applied force, actual signed beam action,
            // independent spring action. Avoid subtracting rounded u*ks.
            close(350.0 - end_j - 1e6 * u, 0.0);
        }
    }
    #[test]
    fn closed_axial_stop_selects_same_state_for_nodes_and_elements() {
        for mode in MODES {
            let mut input = model(nodal_case("G", "global_x", 350.0, "force", "N"));
            input["supports"]
                .as_array_mut()
                .unwrap()
                .push(gap("UX", 0.0));
            let out = solve(input, mode);
            close(
                scalar(
                    &out,
                    "support:gap",
                    "nonlinear_support_active_set_state_code",
                    "G",
                ),
                1.0,
            );
            close(
                scalar(
                    &out,
                    "support:gap",
                    "nonlinear_support_final_displacement",
                    "G",
                ),
                0.0,
            );
            let contact = scalar(&out, "support:gap", "nonlinear_support_final_reaction", "G");
            close(contact, -350.0);
            close(
                350.0 + contact - station(&out, "element_local_axial_force", "end_j", "G"),
                0.0,
            );
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_x", "G"),
                0.0,
            );
            close(scalar(&out, "node:tip", "displacement_magnitude", "G"), 0.0);
            for location in ["quarter_1", "midspan", "quarter_3"] {
                close(
                    station(&out, "element_local_axial_force", location, "G"),
                    0.0,
                );
            }
        }
    }
    #[test]
    fn axial_and_torsion_fields_are_constant() {
        for mode in MODES {
            for (direction, dimension, unit, kind) in [
                ("global_x", "force", "N", "element_local_axial_force"),
                (
                    "global_x",
                    "moment",
                    "N*m",
                    "element_local_torsional_moment",
                ),
            ] {
                let out = solve(
                    model(nodal_case("C", direction, 350.0, dimension, unit)),
                    mode,
                );
                for location in ["quarter_1", "midspan", "quarter_3"] {
                    close(station(&out, kind, location, "C").abs(), 350.0);
                    let (stress_kind, expected) = if dimension == "force" {
                        (
                            "element_local_axial_normal_stress",
                            350.0
                                / (std::f64::consts::PI * (0.168_f64.powi(2) - 0.154_f64.powi(2))
                                    / 4.0)
                                / 1e6,
                        )
                    } else {
                        (
                            "element_local_torsional_shear_stress",
                            350.0 * 0.084 / (2.0 * section_i()) / 1e6,
                        )
                    };
                    close(station(&out, stress_kind, location, "C").abs(), expected);
                }
            }
        }
    }
    #[test]
    fn opposite_signed_sum_recomputes_magnitudes() {
        for mode in MODES {
            let mut input = model(nodal_case("P", "global_y", 350.0, "force", "N"));
            input["load_cases"]
                .as_array_mut()
                .unwrap()
                .push(nodal_case("N", "global_y", -350.0, "force", "N"));
            input["combinations"] = json!([{"id":"SUM","basis":"mechanics","terms":[{"load_case":"P","factor":1},{"load_case":"N","factor":1}]}]);
            let out = solve(input, mode);
            for kind in ["global_nodal_displacement_y", "displacement_magnitude"] {
                close(scalar(&out, "node:tip", kind, "SUM"), 0.0);
            }
            close(
                scalar(&out, "support:root", "reaction_resultant", "SUM"),
                0.0,
            );
        }
    }
    #[test]
    fn tiny_primitive_retains_precision_until_combination_publication() {
        for mode in MODES {
            let mut input = model(nodal_case("TINY", "global_y", 0.00035, "force", "N"));
            input["load_cases"]
                .as_array_mut()
                .unwrap()
                .push(nodal_case("DIRECT", "global_y", 350.0, "force", "N"));
            input["combinations"] = json!([{"id":"SCALED","basis":"mechanics","terms":[{"load_case":"TINY","factor":1e6}]}]);
            let out = solve(input, mode);
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "SCALED"),
                point_tip_mm(350.0),
            );
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "DIRECT"),
                point_tip_mm(350.0),
            );
        }
    }
    #[test]
    fn derived_links_resolve_to_the_owning_primitive_case() {
        for mode in MODES {
            let mut input = model(nodal_case("FIRST", "global_y", 350.0, "force", "N"));
            input["load_cases"]
                .as_array_mut()
                .unwrap()
                .push(nodal_case("SECOND", "global_y", 700.0, "force", "N"));
            let out = solve(input, mode);
            let mut checked = 0;
            for result in out
                .results
                .iter()
                .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_id == "SECOND"))
            {
                for reference in &result.source_result_refs {
                    if reference.starts_with("result:") {
                        let target = out
                            .results
                            .iter()
                            .find(|r| &r.id == reference)
                            .expect("dangling result reference");
                        assert_eq!(
                            target.basis_ref, result.basis_ref,
                            "cross-case derived source {}",
                            result.id
                        );
                        checked += 1;
                    }
                }
            }
            assert!(checked > 0, "no actual derived source edges exercised");
        }
    }
    fn uniform_case() -> Value {
        json!({"id":"U","primitive_loads":[{"id":"load:udl","category":"occasional","target":{"type":"element","pipe":"pipe:beam"},"direction":"global_y","magnitude":{"value":100,"unit":"N/m"},"dimension":"force_per_length"}]})
    }
    #[test]
    fn full_distributed_load_assembly_and_cut_recovery() {
        for mode in MODES {
            let out = solve(model(uniform_case()), mode);
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "U"),
                1000.0 * 100.0 * 2.0_f64.powi(4) / (8.0 * 200e9 * section_i()),
            );
            close(
                scalar(&out, "support:root", "reaction_resultant", "U"),
                200.0,
            );
            for (location, x) in [("quarter_1", 0.5_f64), ("midspan", 1.0), ("quarter_3", 1.5)] {
                close(
                    station(&out, "element_local_shear_force_y", location, "U").abs(),
                    100.0 * (2.0 - x),
                );
                close(
                    station(&out, "element_local_bending_moment_z", location, "U").abs(),
                    50.0 * (2.0 - x).powi(2),
                );
            }
        }
    }
    #[test]
    fn partial_wind_assembly_and_cut_recovery() {
        for mode in MODES {
            // q = pressure * shape * projected OD = (100/.168)*1*.168 N/m.
            let case = json!({"id":"W","primitive_loads":[],"equivalent_static":{"wind":{
                "pressure":{"value":100.0/0.168,"unit":"Pa"},"shape_factor":{"value":1,"unit":"1"},"direction":"global_y","exposed_pipe_refs":[],
                "exposed_spans":[{"pipe_ref":"pipe:beam","start_fraction":{"value":0.25,"unit":"1"},"end_fraction":{"value":0.75,"unit":"1"}}]}}});
            let out = solve(model(case), mode);
            let (a, b, l) = (0.5_f64, 1.5_f64, 2.0_f64);
            let tip = 1000.0 * 100.0 / (6.0 * 200e9 * section_i())
                * (l * (b.powi(3) - a.powi(3)) - (b.powi(4) - a.powi(4)) / 4.0);
            close(
                scalar(&out, "node:tip", "global_nodal_displacement_y", "W"),
                tip,
            );
            close(
                scalar(&out, "support:root", "reaction_resultant", "W"),
                100.0,
            );
            for (location, x) in [("quarter_1", a), ("midspan", 1.0), ("quarter_3", b)] {
                close(
                    station(&out, "element_local_shear_force_y", location, "W").abs(),
                    100.0 * (b - x),
                );
                close(
                    station(&out, "element_local_bending_moment_z", location, "W").abs(),
                    50.0 * (b - x).powi(2),
                );
            }
        }
    }
}
