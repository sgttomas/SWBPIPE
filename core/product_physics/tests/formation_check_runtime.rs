//! K-D5 product tests (T3 D1 revision 5a.2 §4.3.1, §6 K-D5 row) through both
//! public entries: the captured `run_linear_static_preview_value_with_mode`
//! and the historical typed `run_linear_static_preview_with_mode`, in both
//! solver modes.
//!
//! The requests are P1's detection inputs (T3 `DETECTION/scripts/gen.py.txt`
//! run on the frozen R1 references `c0f14201c`), byte for byte: invented
//! inputs with no library data. On main (P1, `DETECTION/RETURN.md`) 122 is
//! published Passed and eligible with a breach of 2.43 (dense) and 1.21
//! (sparse) of the criterion; 345 and the RF-CHAIN r1e-04 continuity
//! controls pass. The PP-route large-coordinate elbows (RV5-B1, ROOT's
//! ruling) are invented K-D5 inputs built in `pp_route_elbow_request`, with
//! exact references from the `kd5_models.py` run record.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, NumericalQualityStatus, PreviewSolverMode,
};
use serde_json::{json, Value};

const RF_SKEW_T_CANT_OFF_122_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-SKEW-T-CANT-OFF-122-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":1.0,"y":2.0,"z":2.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":1,"y":0,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UX","UY","UZ"],"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":144.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:1","node":"N0","family":"spring","restraints":["RY"],"stiffness":{"dof":"RY","value":{"value":1000000.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:2","node":"N0","family":"spring","restraints":["RZ"],"stiffness":{"dof":"RZ","value":{"value":1000000.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-SKEW-T-CANT-OFF-122-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_moment","target":{"type":"node","node":"N1"},"direction":"RX","magnitude":{"value":0.0048,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"load:1","category":"concentrated_moment","target":{"type":"node","node":"N1"},"direction":"RY","magnitude":{"value":0.0096,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"load:2","category":"concentrated_moment","target":{"type":"node","node":"N1"},"direction":"RZ","magnitude":{"value":0.0096,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_SKEW_T_CANT_OFF_345_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-SKEW-T-CANT-OFF-345-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":3.0,"y":4.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":0,"z":1},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UX","UY","UZ"],"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":86.4,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:1","node":"N0","family":"spring","restraints":["RY"],"stiffness":{"dof":"RY","value":{"value":1000000.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"spring:N0:2","node":"N0","family":"spring","restraints":["RZ"],"stiffness":{"dof":"RZ","value":{"value":1000000.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-SKEW-T-CANT-OFF-345-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_moment","target":{"type":"node","node":"N1"},"direction":"RX","magnitude":{"value":0.005184,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"load:1","category":"concentrated_moment","target":{"type":"node","node":"N1"},"direction":"RY","magnitude":{"value":0.006912,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_T_N03_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-T-n03-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UX","UY","UZ","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":216.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-T-n03-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_moment","target":{"type":"node","node":"N3"},"direction":"RX","magnitude":{"value":0.0216,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_T_N05_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-T-n05-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N4","position":{"x":7.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N5","position":{"x":10.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M4","from":"N3","to":"N4","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M5","from":"N4","to":"N5","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UX","UY","UZ","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":216.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-T-n05-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_moment","target":{"type":"node","node":"N5"},"direction":"RX","magnitude":{"value":0.0216,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_T_N10_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-T-n10-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N4","position":{"x":7.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N5","position":{"x":10.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N6","position":{"x":10.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N7","position":{"x":14.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N8","position":{"x":15.75,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N9","position":{"x":19.25,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N10","position":{"x":20.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M4","from":"N3","to":"N4","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M5","from":"N4","to":"N5","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M6","from":"N5","to":"N6","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M7","from":"N6","to":"N7","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M8","from":"N7","to":"N8","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M9","from":"N8","to":"N9","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M10","from":"N9","to":"N10","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UX","UY","UZ","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["RX"],"stiffness":{"dof":"RX","value":{"value":216.0,"unit":"N*m/rad"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-T-n10-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_moment","target":{"type":"node","node":"N10"},"direction":"RX","magnitude":{"value":0.0216,"unit":"N*m"},"dimension":"moment","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_A_N03_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-A-n03-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UY","UZ","RX","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["UX"],"stiffness":{"dof":"UX","value":{"value":59700.0,"unit":"N/m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-A-n03-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_force","target":{"type":"node","node":"N3"},"direction":"global_x","magnitude":{"value":5.97,"unit":"N"},"dimension":"force","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_A_N05_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-A-n05-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N4","position":{"x":7.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N5","position":{"x":10.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M4","from":"N3","to":"N4","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M5","from":"N4","to":"N5","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UY","UZ","RX","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["UX"],"stiffness":{"dof":"UX","value":{"value":59700.0,"unit":"N/m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-A-n05-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_force","target":{"type":"node","node":"N5"},"direction":"global_x","magnitude":{"value":5.97,"unit":"N"},"dimension":"force","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;
const RF_CHAIN_A_N10_R1E_04: &str = r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},"project":{"id":"invented:t3-p1:RF-CHAIN-A-n10-r1e-04","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N1","position":{"x":2.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N2","position":{"x":3.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N3","position":{"x":6.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N4","position":{"x":7.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N5","position":{"x":10.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N6","position":{"x":10.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N7","position":{"x":14.5,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N8","position":{"x":15.75,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N9","position":{"x":19.25,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"N10","position":{"x":20.0,"y":0.0,"z":0.0},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"pipe_segments":[{"id":"M1","from":"N0","to":"N1","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M2","from":"N1","to":"N2","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M3","from":"N2","to":"N3","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M4","from":"N3","to":"N4","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M5","from":"N4","to":"N5","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M6","from":"N5","to":"N6","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M7","from":"N6","to":"N7","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M8","from":"N7","to":"N8","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M9","from":"N8","to":"N9","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"},{"id":"M10","from":"N9","to":"N10","material":"mat:N","y_reference":{"x":0,"y":1,"z":0},"section":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"materials":[{"id":"mat:N","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"supports":[{"id":"rigid:N0","node":"N0","restraints":["UY","UZ","RX","RY","RZ"],"provenance":"invented_t3_p1_detection_input_no_library_data","family":"anchor"},{"id":"spring:N0:0","node":"N0","family":"spring","restraints":["UX"],"stiffness":{"dof":"UX","value":{"value":59700.0,"unit":"N/m"}},"provenance":"invented_t3_p1_detection_input_no_library_data"}],"load_cases":[{"id":"case","label":"RF-CHAIN-A-n10-r1e-04","kind":"primitive_user_load","primitive_loads":[{"id":"load:0","category":"concentrated_force","target":{"type":"node","node":"N10"},"direction":"global_x","magnitude":{"value":5.97,"unit":"N"},"dimension":"force","provenance":"invented_t3_p1_detection_input_no_library_data"}],"provenance":"invented_t3_p1_detection_input_no_library_data"}],"combinations":[]},"materials":[]}"#;

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn request(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// Both entries, one mode: (entry label, envelope).
fn both_entries(value: &Value, mode: PreviewSolverMode) -> [(&'static str, MechanicsEnvelope); 2] {
    let captured = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
    let typed_request: LinearStaticPreviewRequest = serde_json::from_value(value.clone()).unwrap();
    let typed = run_linear_static_preview_with_mode(typed_request, mode);
    [("captured", captured), ("typed", typed)]
}

fn integrity_codes(envelope: &MechanicsEnvelope) -> Vec<String> {
    envelope
        .diagnostics
        .iter()
        .filter(|d| d.code.starts_with("NUMERICAL_INTEGRITY"))
        .map(|d| d.code.clone())
        .collect()
}

fn case_qualities(envelope: &MechanicsEnvelope) -> Vec<NumericalQualityStatus> {
    envelope
        .numerical_quality
        .cases
        .iter()
        .map(|c| c.solve_quality)
        .collect()
}

#[test]
fn kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries() {
    let value = request(RF_SKEW_T_CANT_OFF_122_R1E_04);
    for mode in MODES {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{entry} {}", mode.as_str());
            assert_eq!(
                integrity_codes(&envelope),
                vec!["NUMERICAL_INTEGRITY_SENSITIVE"],
                "{ctx}"
            );
            assert_eq!(
                case_qualities(&envelope),
                vec![NumericalQualityStatus::Sensitive],
                "{ctx}"
            );
            // The demoted report differs from the ordinary one only in quality.
            let message = &envelope
                .diagnostics
                .iter()
                .find(|d| d.code == "NUMERICAL_INTEGRITY_SENSITIVE")
                .unwrap()
                .message;
            assert!(message.contains("quality: Sensitive"), "{ctx}");
            // Since F1a the record is rendered as one `formation_check:`
            // evidence line (tests in `src/f1a_tests.rs`); this still guards
            // against a `FormationCheck` Debug rendering in the message.
            assert!(
                !message.contains("FormationCheck"),
                "{ctx}: no in-band marker"
            );
        }
    }
}

#[test]
fn kd5_skew_345_and_chain_continuity_controls_do_not_demote() {
    for text in [
        RF_SKEW_T_CANT_OFF_345_R1E_04,
        RF_CHAIN_T_N03_R1E_04,
        RF_CHAIN_T_N05_R1E_04,
        RF_CHAIN_T_N10_R1E_04,
        RF_CHAIN_A_N03_R1E_04,
        RF_CHAIN_A_N05_R1E_04,
        RF_CHAIN_A_N10_R1E_04,
    ] {
        let value = request(text);
        let id = value["model"]["project"]["id"]
            .as_str()
            .unwrap()
            .to_string();
        for mode in MODES {
            for (entry, envelope) in both_entries(&value, mode) {
                let ctx = format!("{id} {entry} {}", mode.as_str());
                assert_eq!(
                    integrity_codes(&envelope),
                    vec!["NUMERICAL_INTEGRITY_CHECKS_PASSED"],
                    "{ctx}"
                );
                assert_eq!(
                    case_qualities(&envelope),
                    vec![NumericalQualityStatus::ChecksPassed],
                    "{ctx}"
                );
            }
        }
    }
}

/// 122 with an open gap support at the tip (never closes under this load).
fn with_nonlinear_support(mut value: Value) -> Value {
    value["model"]["supports"].as_array_mut().unwrap().push(json!({
        "id": "support:kd5-open-gap", "node": "N1", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive",
                      "closes_when": "positive_displacement", "gap": {"value": 1000.0, "unit": "mm"}},
        "provenance": "invented K-D5 control: an open gap support that never closes"
    }));
    value
}

#[test]
fn kd5_nonlinear_support_invocation_is_never_selected() {
    // ROOT: a case with a nonlinear support is not selected; its ordinary
    // result and standing are published exactly as today. Precondition: the
    // same linear system without the support demotes (the paths differ).
    let plain = request(RF_SKEW_T_CANT_OFF_122_R1E_04);
    let nonlinear = with_nonlinear_support(plain.clone());
    for mode in MODES {
        for ((entry, demoted), (_, envelope)) in both_entries(&plain, mode)
            .into_iter()
            .zip(both_entries(&nonlinear, mode))
        {
            let ctx = format!("{entry} {}", mode.as_str());
            assert_eq!(
                case_qualities(&demoted),
                vec![NumericalQualityStatus::Sensitive],
                "{ctx}"
            );
            assert!(
                envelope
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "NONLINEAR_SUPPORT_LOOP_CONVERGED"),
                "{ctx}: the nonlinear loop ran"
            );
            assert!(
                !integrity_codes(&envelope)
                    .iter()
                    .any(|c| c == "NUMERICAL_INTEGRITY_SENSITIVE"),
                "{ctx}"
            );
            // A nonlinear-support invocation is not source-eligible (PP
            // `source_eligible`), so it carries no source-block receipt, and
            // no ordinary-attempt entry exists that the check could demote.
            // (RV5 NOTE: the former loop over the receipt's cases iterated
            // nothing; this asserts the state it relied on.)
            let v = serde_json::to_value(&envelope).unwrap();
            assert!(v["source_block_recovery"].is_null(), "{ctx}");
        }
    }
}

/// A PP-route elbow at large coordinates (RV5-B1): N0 at `x0`, N1 at `x1`
/// (z = 0), one realized bend of radius `radius` m on M1 with y reference +y
/// (no designed mismatch); OD 0.2 m, wall
/// 0.01 m, E 2e11 Pa, G 8e10 Pa; N0 translations rigid, rotational springs
/// 1e6 N·m/rad; a tip moment (1, 1, 1) N·m. Invented inputs.
fn pp_route_elbow_request(id: &str, x0: [f64; 2], x1: [f64; 2], radius: f64) -> Value {
    let p = "invented_k_d5_rv5_b1_pp_route_large_coordinate_elbow_no_library_data";
    let spring = |dof: &str| {
        json!({"id": format!("spring:N0:{dof}"), "node": "N0", "family": "spring", "restraints": [dof],
               "stiffness": {"dof": dof, "value": {"value": 1.0e6, "unit": "N*m/rad"}}, "provenance": p})
    };
    let moment = |dof: &str| {
        json!({"id": format!("load:{dof}"), "category": "concentrated_moment", "target": {"type": "node", "node": "N1"},
               "direction": dof, "magnitude": {"value": 1.0, "unit": "N*m"}, "dimension": "moment", "provenance": p})
    };
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:k-d5:{id}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": [
            {"id": "N0", "position": {"x": x0[0], "y": x0[1], "z": 0.0}, "provenance": p},
            {"id": "N1", "position": {"x": x1[0], "y": x1[1], "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": "mat:N",
                           "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                           "section": {"outside_diameter": {"value": 0.2, "unit": "m"},
                                       "wall_thickness": {"value": 0.01, "unit": "m"}},
                           "provenance": p}],
        "materials": [{"id": "mat:N", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"},
                       "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": p}],
        "components": [{"id": "component:bend", "label": "K-D5 PP-route elbow", "kind": "bend", "node": "N1",
                        "geometry": {"bend_pipe_ref": "M1", "bend_radius": {"value": radius, "unit": "m"},
                                     "bend_plane_orientation": "global_xy_preview",
                                     "bend_geometry_source_reference": "invented"},
                        "modifiers": {"flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
                                      "source_reference": "invented"},
                        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
                                                "rule_check_consumption": "user_rule_pack_inputs_only"},
                        "provenance": p}],
        "supports": [
            {"id": "rigid:N0", "node": "N0", "restraints": ["UX", "UY", "UZ"], "provenance": p},
            spring("RX"), spring("RY"), spring("RZ")],
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load",
                        "primitive_loads": [moment("RX"), moment("RY"), moment("RZ")], "provenance": p}],
        "combinations": []
    }, "materials": []})
}

/// One PP-route elbow and the exact intended free displacements of its request
/// (global DOF, m or rad), from exact binary64 inputs: PP's centre and derived
/// section (id = od − 2t), D1's objective curved re-formation at 60 digits,
/// rounded once (`kd5_models.py` run record, `product_test_models`).
struct PpRouteElbow {
    id: &'static str,
    x0: [f64; 2],
    x1: [f64; 2],
    radius: f64,
    u_int: [(usize, f64); 9],
}

/// X ≈ 5e5 m, φ = 2°: PP's binary64 centre is equidistant to 1.5e-12 R, and the
/// product's element is objective here (model PP_UTM_2_PRODUCT_SECTION).
const PP_UTM_5E5: PpRouteElbow = PpRouteElbow {
    id: "PP-UTM-5E5-PHI2",
    x0: [500000.0, 350000.0],
    x1: [500000.010469849, 350000.0001827519],
    radius: 0.3,
    u_int: [
        (3, 1e-06),
        (4, 1e-06),
        (5, 1e-06),
        (6, -1.8287001134315171e-10),
        (7, 1.0479998171669618e-08),
        (8, -1.0297158446337196e-08),
        (9, 1.0024314433163164e-06),
        (10, 1.0019471998915866e-06),
        (11, 1.0019385480234911e-06),
    ],
};

/// X = 5e6 m, Y = 3.5e6 m, φ = 5°: PP's binary64 centre (rounded at ulp(5e6))
/// is admissible but not equidistant, so the product's chord R(cos φ − 1),
/// R sin φ is not the actual chord and the published error exceeds the
/// criterion (model PP_UTM_5E6_PHI5_PRODUCT_SECTION; RV5-B1, ROOT's ruling).
const PP_UTM_5E6: PpRouteElbow = PpRouteElbow {
    id: "PP-UTM-5E6-PHI5",
    x0: [5000000.0, 3500000.0],
    x1: [5000000.026146723, 3500000.0011415905],
    radius: 0.3,
    u_int: [
        (3, 1e-06),
        (4, 1e-06),
        (5, 1e-06),
        (6, -1.1434351901457734e-09),
        (7, 2.6210121544675173e-08),
        (8, -2.5067176622443854e-08),
        (9, 1.0061076233272944e-06),
        (10, 1.004902172644534e-06),
        (11, 1.0048463700931492e-06),
    ],
};

/// The same elbow as PP_UTM_5E6 with T4-U1's objective element: u_int is
/// T4-I6's exact reference for the element formed from (x_i, x_j, R, y)
/// (round 00 `t3_models[16]`, `u_int_new`, rounded once).
const PP_UTM_5E6_OBJECTIVE: PpRouteElbow = PpRouteElbow {
    id: "PP-UTM-5E6-PHI5",
    x0: [5000000.0, 3500000.0],
    x1: [5000000.026146723, 3500000.0011415905],
    radius: 0.3,
    u_int: [
        (3, 1e-06),
        (4, 1e-06),
        (5, 1e-06),
        (6, -1.1434351902152318e-09),
        (7, 2.6210121544673118e-08),
        (8, -2.506717662237247e-08),
        (9, 1.0061076233284874e-06),
        (10, 1.0049021726459545e-06),
        (11, 1.0048463700931473e-06),
    ],
};

/// K1-IP at X = 5e6 m, Y = 3.5e6 m (T4-I6 round 01 item 4; RV2 N-5): the
/// 0.3 m chord bent by φ = 1e-8 rad (R ≈ 3e7 m). x1 − x0 equals K1's d
/// exactly in binary64, so u_int is K1's (round 00 `m31b_kill_and_mutant`
/// 'K1-IP-1E-8-X5e6', equal to X = 0's).
const K1_IP_UTM: PpRouteElbow = PpRouteElbow {
    id: "K1-IP-1E-8-X5E6",
    x0: [5000000.0, 3500000.0],
    x1: [5000000.259807621, 3500000.1500000004],
    radius: 30000000.018065747,
    u_int: [
        (3, 1e-06),
        (4, 1e-06),
        (5, 1e-06),
        (6, -1.5416514863688512e-07),
        (7, 2.670218695534503e-07),
        (8, -1.1285672092604801e-07),
        (9, 1.0719600545549025e-06),
        (10, 1.0650181408042967e-06),
        (11, 1.0555353102138075e-06),
    ],
};

impl PpRouteElbow {
    fn request(&self) -> Value {
        pp_route_elbow_request(self.id, self.x0, self.x1, self.radius)
    }

    /// max over the free rows of |u − u_int| / (1e-9·max(|u_int|, S*_kind)),
    /// S* per D1 §4.1.6.1 items 4–6 (one body; L_b the chord's extent).
    fn actual_ratio(&self, envelope: &MechanicsEnvelope) -> f64 {
        let names = ["ux", "uy", "uz", "rx", "ry", "rz"];
        let published = |dof: usize| {
            let id = format!("result:disp:N{}:{}", dof / 6, names[dof % 6]);
            let row = envelope
                .results
                .iter()
                .find(|r| r.id == id)
                .unwrap_or_else(|| panic!("{id}"));
            match row.unit.as_str() {
                "mm" => row.value / 1000.0,
                "rad" => row.value,
                other => panic!("{id}: unit {other}"),
            }
        };
        let (mut st, mut sr) = (0.0_f64, 0.0_f64);
        for &(dof, v) in &self.u_int {
            if dof % 6 < 3 {
                st = st.max(v.abs());
            } else {
                sr = sr.max(v.abs());
            }
        }
        let (dx, dy) = (self.x1[0] - self.x0[0], self.x1[1] - self.x0[1]);
        let l_b = (dx * dx + dy * dy).sqrt();
        let (tr, ro) = (st.max(l_b * sr), sr.max(st / l_b));
        self.u_int
            .iter()
            .map(|&(dof, v)| {
                let scale = v.abs().max(if dof % 6 < 3 { tr } else { ro });
                (published(dof) - v).abs() / (1e-9 * scale)
            })
            .fold(0.0, f64::max)
    }
}

/// Both entries, both modes: (context, envelope, actual ratio), after the
/// precondition that M1 is realized as a curved element.
fn run_pp_route_elbow(elbow: &PpRouteElbow) -> Vec<(String, MechanicsEnvelope, f64)> {
    let value = elbow.request();
    let mut out = Vec::new();
    for mode in MODES {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{} {entry} {}", elbow.id, mode.as_str());
            let v = serde_json::to_value(&envelope).unwrap();
            assert!(
                v["results"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|r| r["entity_ref"] == "M1"
                        && r["metadata"]["coordinate_system"] == "arc_chord_frame"),
                "{ctx}: M1 realized as an arc"
            );
            let actual = elbow.actual_ratio(&envelope);
            eprintln!("kd5 {ctx} actual={actual}");
            out.push((ctx, envelope, actual));
        }
    }
    out
}

#[test]
fn kd5_large_coordinate_pp_route_elbow_is_published_accurately_and_not_demoted() {
    // ROOT's control (RV5-B1): an ordinary large-coordinate elbow realized by
    // PP itself at X ≈ 5e5 m is published within half the criterion of its
    // exact intended solution, and K-D5 leaves it Passed on both entries in
    // both modes (no false demotion).
    for (ctx, envelope, actual) in run_pp_route_elbow(&PP_UTM_5E5) {
        assert!(actual < 0.5, "{ctx}: actual {actual}");
        assert_eq!(
            integrity_codes(&envelope),
            vec!["NUMERICAL_INTEGRITY_CHECKS_PASSED"],
            "{ctx}"
        );
        assert_eq!(
            case_qualities(&envelope),
            vec![NumericalQualityStatus::ChecksPassed],
            "{ctx}"
        );
    }
}

fn assert_published_accurately_and_not_demoted(elbow: &PpRouteElbow) {
    for (ctx, envelope, actual) in run_pp_route_elbow(elbow) {
        assert!(actual < 0.5, "{ctx}: actual {actual}");
        assert_eq!(
            integrity_codes(&envelope),
            vec!["NUMERICAL_INTEGRITY_CHECKS_PASSED"],
            "{ctx}"
        );
        assert_eq!(
            case_qualities(&envelope),
            vec![NumericalQualityStatus::ChecksPassed],
            "{ctx}"
        );
    }
}

#[test]
fn kd5_very_large_coordinate_pp_route_elbow_is_published_accurately_and_not_demoted() {
    // C2's UTM control at X = 5e6 m (T4-U1; the elbow of the ignored test
    // below): with T4-U1's objective element no binary64 centre enters, the
    // published u is within half the criterion of the exact intended
    // solution, and K-D5 leaves it Passed on both entries in both modes.
    assert_published_accurately_and_not_demoted(&PP_UTM_5E6_OBJECTIVE);
}

#[test]
fn kd5_k1_stable_form_on_the_pp_route_at_utm_coordinates_is_not_demoted() {
    // K1 (T4-I6 round 01 item 4) through PP: the stable small-angle form at
    // φ = 1e-8 rad realized by PP at X = 5e6 m publishes u within half the
    // criterion and is not demoted, on both entries in both modes. K-D5 with
    // the binary64 formula chord (M31b) demotes it.
    assert_published_accurately_and_not_demoted(&K1_IP_UTM);
}

#[test]
#[ignore = "M31b0 equivalence pending ROOT ruling (DEL-04-01 Design); see K1/K2 for M31b"]
fn kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries() {
    // ROOT's product-level demotion test (RV5-B1): at X = 5e6 m PP's own
    // binary64 centre makes the product's chord differ from the actual chord,
    // and the published error exceeds the criterion. The check's H uses the
    // actual chord, so the case publishes SENSITIVE on both entries in both
    // modes; mutation 31b (H from the product's chord) publishes
    // CHECKS_PASSED and fails the integrity assertion below.
    for (ctx, envelope, actual) in run_pp_route_elbow(&PP_UTM_5E6) {
        // Precondition: the published error is above the criterion.
        assert!(actual > 1.0, "{ctx}: actual {actual}");
        assert_eq!(
            integrity_codes(&envelope),
            vec!["NUMERICAL_INTEGRITY_SENSITIVE"],
            "{ctx}"
        );
        assert_eq!(
            case_qualities(&envelope),
            vec![NumericalQualityStatus::Sensitive],
            "{ctx}"
        );
    }
}
