//! Frozen independent U4a controls: both entries/modes and the actual
//! binary64 pressure publication, including unchanged cut/stress families.
use super::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
const REQUESTS: &str = include_str!("../../../validation/references/t4_u4a/requests.json");
const ORACLE: &str = include_str!("../../../validation/references/t4_u4a/oracle.json");
const TANGENT_PREVIEW: bool = true;
const ENDS: [(&str, &str); 6] = [
    ("F_x", "element_local_axial_force"), ("F_y", "element_local_shear_force_y"),
    ("F_z", "element_local_shear_force_z"), ("M_x", "element_local_torsional_moment"),
    ("M_y", "element_local_bending_moment_y"), ("M_z", "element_local_bending_moment_z"),
];
fn n(v: &Value) -> f64 { v.as_str().map(|v| v.parse().unwrap()).or_else(|| v.as_f64()).unwrap() }
fn reference_inputs() -> (Value, Value) {
    let requests: Value = serde_json::from_str(REQUESTS).unwrap();
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(REQUESTS.as_bytes())), oracle["requests_sha256"].as_str().unwrap());
    (requests, oracle)
}
/// Re-author only the copied invocation to the current 0.7.0 document (B7a's
/// pressure-free version; B7b's exact contract with zero allowance and
/// Euler-Bernoulli). The SHA-bound frozen request and every physical input stay fixed.
fn current_request(requests: &Value, id: &str) -> Value {
    crate::b7b_admission_tests::reauthor_current_request(requests["cases"][id].clone())
}
fn row<'a>(out: &'a MechanicsEnvelope, kind: &str, entity: &str, location: &str, component: &str) -> &'a ResultItem {
    row_on(out, kind, entity, location, component, "")
}
/// The one row on section basis `basis` (D-5: `; section_basis=…` on its
/// token; "" for a row published once, as every preview row is).
fn row_on<'a>(out: &'a MechanicsEnvelope, kind: &str, entity: &str, location: &str, component: &str, basis: &str) -> &'a ResultItem {
    let found = out.results.iter().filter(|r| r.kind == kind && r.entity_ref == entity &&
        r.metadata.as_ref().is_some_and(|m| (location.is_empty() || m.location == location) && (component.is_empty() || m.component == component)
            && m.basis.rsplit_once("; section_basis=").map_or("", |(_, b)| b) == basis)).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{entity} {kind} {location} {component} [{basis}]: {:?}", out.diagnostics);
    found[0]
}
fn close(label: &str, got: f64, expected: f64, scale: f64) {
    let tolerance = 1e-9 * expected.abs().max(scale);
    assert!((got - expected).abs() <= tolerance, "{label}: {got:.16e} != {expected:.16e}, distance {} tolerances", (got-expected).abs()/tolerance);
}
fn check(out: &MechanicsEnvelope, case: &Value, label: &str) {
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: {:?}", out.diagnostics);
    let pressure = case["route"] == "pressure-1";
    for review in out.results.iter().filter(|r| r.kind == "curved_bend_macro_element_review") {
        let expected = if pressure || TANGENT_PREVIEW {
            "recovery=node_on_element_end_actions_from_assembled_stiffness_in_end_tangent_frame"
        } else {
            "recovery=end_forces_from_assembled_stiffness_in_chord_frame"
        };
        assert!(review.metadata.as_ref().unwrap().basis.split(';').any(|part| part == expected),
            "{label}: macro review describes the published end frame");
    }
    for (pipe, member) in case["members"].as_object().unwrap() {
        let arc = member["kind"] == "arc";
        let tangent = !arc || pressure || TANGENT_PREVIEW;
        let force_scale = n(&member["zero_scale"]["force"]);
        let moment_scale = n(&member["zero_scale"]["moment"]);
        for end in ["end_i", "end_j"] {
            let expected = &member["end_rows_node_on_element_elastic"][end][if tangent {"tangent_frame"} else {"chord_frame"}];
            for (key, kind) in ENDS {
                if pressure && key == "F_x" {
                    assert!(!out.results.iter().any(|r| r.kind == kind && r.entity_ref == *pipe));
                    continue;
                }
                let r = row(out, kind, pipe, end, "");
                close(&format!("{label} {pipe} {end} {key}"), r.value, n(&expected[key]), if key.starts_with('F') {force_scale} else {moment_scale});
                if arc && tangent {
                    let m = r.metadata.as_ref().unwrap();
                    assert_eq!(m.coordinate_system, "element_local");
                    assert_eq!(m.basis, SECTION_RESULTANT_BASIS);
                    assert_eq!(m.sign_convention, if end == "end_i" {ARC_END_I_SIGN} else {ARC_END_J_SIGN});
                }
            }
        }
        for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
            let cut = &member["cuts_tangent_frame_j_side"][location];
            for (key, kind) in [("N", ENDS[0].1), ("V_y", ENDS[1].1), ("V_z", ENDS[2].1), ("T", ENDS[3].1), ("M_y", ENDS[4].1), ("M_z", ENDS[5].1)] {
                if pressure && key == "N" { continue; }
                let endpoint = location.starts_with("end");
                if endpoint && arc && !tangent { continue; }
                let sign = if endpoint && location == "end_i" {-1.0} else {1.0};
                close(&format!("{label} cut {pipe} {location} {key}"), sign*row(out, kind, pipe, location, "").value, n(&cut[key]), if key == "N" || key.starts_with('V') {force_scale} else {moment_scale});
            }
            // D-5 (T4-U6): a pressure-1 case's stresses are frozen per section
            // basis with that basis's section (`stresses_SI_by_section_basis`,
            // `derived.section_bases`); a preview case keeps its one section.
            let bases: Vec<(&str, &Value, &Value)> = if pressure {
                ["nominal_section", "reduced_section"].into_iter()
                    .map(|b| (b, &member["stresses_SI_by_section_basis"][b][location], &case["derived"]["section_bases"][b])).collect()
            } else {
                vec![("", &member["nominal_stresses_SI"][location], &case["derived"])]
            };
            for &(basis, stress, section) in &bases {
                for (key, kind) in [("sigma_b_y", "element_local_bending_normal_stress_y"), ("sigma_b_z", "element_local_bending_normal_stress_z"), ("tau_t", "element_local_torsional_shear_stress")] {
                    let max_stress = if key == "tau_t" {moment_scale*n(&case["derived"]["r_o"])/n(&section["J"])} else {moment_scale/n(&section["Z"])};
                    close(&format!("{label} stress {pipe} {location} {key} [{basis}]"), row_on(out,kind,pipe,location,"",basis).value*1e6, n(&stress[key]),max_stress);
                }
            }
            if pressure {
                for (key, kind) in [("N_w", "pipe_wall_axial_force_v2"), ("S", "pipe_effective_axial_force_v2")] {
                    close(&format!("{label} pressure {pipe} {location} {key}"), row(out,kind,pipe,location,"").value,n(&cut[key]),force_scale);
                }
                for &(basis, stress, section) in &bases {
                    close(&format!("{label} membrane {pipe} {location} [{basis}]"),row_on(out,"pipe_axial_membrane_stress_v2",pipe,location,"",basis).value,
                        n(&stress["sigma_m_wall_membrane"]),force_scale/n(&section["A_s"]));
                }
            } else {
                let stress = &member["nominal_stresses_SI"][location]["sigma_axial"];
                close(label,row(out,"element_local_axial_normal_stress",pipe,location,"").value*1e6,n(stress),force_scale/n(&case["derived"]["A_s"]));
            }
        }
        if pressure {
            for end in ["end_i","end_j"] {
                close(label,row(out,"pipe_wall_endpoint_action_v2",pipe,end,"").value,n(&member["wall_end_action_along_end_tangent"][end]),force_scale);
            }
        }
    }
    for (node, values) in case["node_displacements"].as_object().unwrap() {
        for (key, kind) in [("ux","global_nodal_displacement_x"),("uy","global_nodal_displacement_y"),("uz","global_nodal_displacement_z"),("rx","global_nodal_rotation_x"),("ry","global_nodal_rotation_y"),("rz","global_nodal_rotation_z")] {
            let scale = case["node_displacements"].as_object().unwrap().values().flat_map(|v| if key.starts_with('u') {vec![n(&v["ux"]).abs(),n(&v["uy"]).abs(),n(&v["uz"]).abs()]} else {vec![n(&v["rx"]).abs(),n(&v["ry"]).abs(),n(&v["rz"]).abs()]}).fold(0.0,f64::max);
            // Node rows intentionally have no metadata on the legacy surface.
            let r = out.results.iter().find(|r| r.kind == kind && r.entity_ref == *node).unwrap();
            close(label,r.value/if key.starts_with('u') {1000.0} else {1.0},n(&values[key]),scale);
        }
    }
    for (field, support, moment) in [("support_A_on_pipe","support:A","moment_about_A"),("redundant_support_D_on_pipe","support:D","moment_about_D")] {
        if case[field].is_null() {continue;}
        for (family, keys) in [("force",["Fx","Fy","Fz"]),(moment,["Mx","My","Mz"])] {
            let values=case[field][family].as_array().unwrap();
            let scale=values.iter().map(|v| n(v).abs()).fold(0.0,f64::max);
            for (key,value) in keys.into_iter().zip(values) {close(label,row(out,"support_reaction_component_v2",support,"",key).value,n(value),scale);}
        }
    }
    if pressure || TANGENT_PREVIEW {
        assert!(!out.results.iter().any(|r| r.metadata.as_ref().is_some_and(|m| m.coordinate_system == "arc_chord_frame")));
        for (mutant, control) in case["discriminators"].as_object().unwrap() {
            // M7 cannot be value-discriminated: FK checks the coefficient balls.
            if control["listed_rows"].is_null() {continue;}
            for wrong in control["listed_rows"].as_array().unwrap() {
                let pipe=wrong["pipe"].as_str().unwrap(); let end=wrong["location"].as_str().unwrap();let key=wrong["row"].as_str().unwrap();
                let kind=ENDS.iter().find(|(k,_)| *k==key).unwrap().1;
                let expected=n(&case["members"][pipe]["end_rows_node_on_element_elastic"][end]["tangent_frame"][key]);
                let scale=n(&case["members"][pipe]["zero_scale"][if key.starts_with('F') {"force"} else {"moment"}]);
                assert!((row(out,kind,pipe,end,"").value-n(&wrong["mutant_value"])).abs()>=1e3*1e-9*expected.abs().max(scale),"{label} {mutant}");
            }
        }
    }
}
#[test]
fn u4a_frozen_references_both_entries_and_modes() {
    let (requests, oracle) = reference_inputs();
    let baseline=std::env::var("U4A_PREVIEW_BASELINE").ok().map(|path|serde_json::from_str::<Value>(&std::fs::read_to_string(path).unwrap()).unwrap());
    for (id,case) in oracle["cases"].as_object().unwrap() {
        for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
            for captured in [true,false] {
                let document=current_request(&requests,id);
                tests::KD5_RECOVERY_PROBE.with(|p|*p.borrow_mut()=Some(Vec::new()));
                let out=if captured {run_linear_static_preview_value_with_mode(document,mode).unwrap()} else {run_linear_static_preview_with_mode(serde_json::from_value(document).unwrap(),mode)};
                let records=tests::KD5_RECOVERY_PROBE.with(|p|p.borrow_mut().take().unwrap());
                if !id.ends_with("GAP") {assert!(out.diagnostics.iter().any(|d| d.code=="NUMERICAL_INTEGRITY_CHECKS_PASSED"),"{id} {mode:?}: {:?}",out.diagnostics);}
                check(&out,case,&format!("{id} {mode:?} captured={captured}"));
                for (pipe, member) in case["members"].as_object().unwrap().iter().filter(|(_,m)|m["kind"]=="arc") {
                    if id.ends_with("GAP") {assert!(!records.iter().any(|r|r.member==*pipe));continue;}
                    let record=records.iter().find(|r|r.member==*pipe).expect("arc certificate record");
                    for end in ["end_i","end_j"] {
                        for (group, keys) in [("shear",vec!["F_y","F_z"]),("torsion",vec!["M_x"]),("bending",vec!["M_y","M_z"])] {
                            let values=keys.iter().map(|key|row(&out,ENDS.iter().find(|(k,_)|k==key).unwrap().1,pipe,end,"").value).collect::<Vec<_>>();
                            let q=if values.len()==1 {values[0].abs()} else {norm2(values[0],values[1])};
                            let recorded=record.groups.iter().find(|g|g.location==end && g.kind==group).unwrap();
                            assert_eq!(recorded.q.to_bits(),q.to_bits(),"{id} {pipe} {end} {group}: groups use published end rows");
                        }
                        let axial=record.groups.iter().find(|g|g.location==end && g.kind=="axial").unwrap();
                        let frame=if case["route"]=="pressure-1" || TANGENT_PREVIEW {"tangent_frame"} else {"chord_frame"};
                        let expected=n(&member["end_rows_node_on_element_elastic"][end][frame]["F_x"]).abs();
                        close(id,axial.q,expected,n(&member["zero_scale"]["force"]));
                    }
                }
                if !TANGENT_PREVIEW && case["route"] == "preview-physics-1" {
                    if let Some(baseline) = &baseline {
                        let key=format!("{id}/{}/{}",if mode==PreviewSolverMode::SparseInteractive {"sparse_interactive"} else {"dense_scrutiny"},if captured {"raw"} else {"typed"});
                        let bytes=serde_json::to_string(&out).unwrap();
                        assert_eq!(format!("{:x}",Sha256::digest(bytes.as_bytes())),baseline["captures"][&key]["sha256"].as_str().unwrap(),"stage1 baseline envelope bytes {key}");
                        assert_eq!(bytes,baseline["captures"][key]["bytes"].as_str().unwrap());
                    }
                }
            }
        }
    }
}
#[test]
fn u4a_pressure_binary64_demotion_checks_both_recovery_and_unchanged_cuts() {
    let (requests, oracle) = reference_inputs();
    for (id,case) in oracle["cases"].as_object().unwrap().iter().filter(|(_,c)| c["route"]=="pressure-1") {
        for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
            for captured in [true,false] {
                tests::KD5_RECOVERY_PROBE.with(|p|*p.borrow_mut()=Some(Vec::new()));
                U4A_FORCE_BINARY64.with(|flag|flag.set(true));
                let document=current_request(&requests,id);
                let out=if captured {run_linear_static_preview_value_with_mode(document,mode).unwrap()} else {run_linear_static_preview_with_mode(serde_json::from_value(document).unwrap(),mode)};
                U4A_FORCE_BINARY64.with(|flag|flag.set(false));
                let records=tests::KD5_RECOVERY_PROBE.with(|p|p.borrow_mut().take().unwrap());
                assert!(records.is_empty(),"forced ordinary recovery must publish no certified records");
                assert!(out.diagnostics.iter().any(|d| d.code=="NUMERICAL_INTEGRITY_SENSITIVE" && d.message.contains("member rows are binary64 recoveries")),"{id}: {:?}",out.diagnostics);
                check(&out,case,&format!("binary64 {id} {mode:?} captured={captured}"));
            }
        }
    }
}
