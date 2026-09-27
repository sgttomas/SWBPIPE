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
//! controls pass.
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
            // The receipt's ordinary attempt (captured route) is the linear
            // attempt; it is never demoted by the formation check.
            let v = serde_json::to_value(&envelope).unwrap();
            for case in v["source_block_recovery"]["body"]["cases"]
                .as_array()
                .into_iter()
                .flatten()
            {
                assert_ne!(case["ordinary_attempt"]["outcome"], "sensitive", "{ctx}");
            }
        }
    }
}
