use open_pipe_stress_solver_performance_harness::{
    run_sparse_default_promotion_observation_suite, HarnessSettings,
    DEC_053_SPARSE_DEFAULT_PROMOTION_OBSERVATION_ID, DEC_053_SPARSE_DEFAULT_PROMOTION_POLICY_REF,
    DEC_053_SPARSE_DEFAULT_PROMOTION_STATUS,
};
use std::process::Command;

fn main() {
    let settings = HarnessSettings {
        solver_version: "dec053-sparse-default-promotion-local-observation".to_string(),
        repeat_count: 2,
        ..HarnessSettings::default()
    };
    let observations = run_sparse_default_promotion_observation_suite(&settings)
        .expect("DEC-053 sparse default promotion observations should run");

    println!("{{");
    println!("  \"record_kind\": \"sparse_default_promotion_observation\",");
    println!(
        "  \"record_id\": \"{}\",",
        DEC_053_SPARSE_DEFAULT_PROMOTION_OBSERVATION_ID
    );
    println!("  \"decision_ref\": \"DEC-053\",");
    println!("  \"tranche_id\": \"TP-R4-D7-SPARSEDEFAULTPROMOTE-001\",");
    println!("  \"status\": \"observed_sparse_interactive_default_promotion_evidence\",");
    println!(
        "  \"default_sparse_promotion_status\": \"{}\",",
        DEC_053_SPARSE_DEFAULT_PROMOTION_STATUS
    );
    println!(
        "  \"policy_ref\": \"{}\",",
        DEC_053_SPARSE_DEFAULT_PROMOTION_POLICY_REF
    );
    println!("  \"observation_count\": {},", observations.len());
    println!("  \"timing_policy\": \"observed_not_threshold_gated\",");
    println!("  \"allocator_rss_policy\": \"observed_not_threshold_gated\",");
    println!("  \"hardware_normalization_policy\": \"hardware_metadata_recorded_no_cross_machine_threshold\",");
    println!(
        "  \"ci_gate_status\": \"covered_by_local_dec025_cargo_pytest_surfaces_no_hosted_ci\","
    );
    println!("  \"hardware_metadata\": {{");
    println!("    \"os\": \"{}\",", escape(std::env::consts::OS));
    println!("    \"arch\": \"{}\",", escape(std::env::consts::ARCH));
    println!(
        "    \"rss_kib_at_packet_emit\": {}",
        current_rss_kib().unwrap_or(0)
    );
    println!("  }},");
    println!("  \"observations\": [");
    for (index, observation) in observations.iter().enumerate() {
        println!("    {{");
        println!(
            "      \"observation_id\": \"{}\",",
            escape(&observation.observation_id)
        );
        println!(
            "      \"fixture_id\": \"{}\",",
            escape(&observation.fixture_id)
        );
        println!(
            "      \"practical_size_band\": \"{}\",",
            escape(&observation.practical_size_band)
        );
        println!(
            "      \"fixture_family\": \"{}\",",
            escape(&observation.fixture_family)
        );
        println!("      \"node_count\": {},", observation.node_count);
        println!("      \"element_count\": {},", observation.element_count);
        println!("      \"total_dofs\": {},", observation.total_dofs);
        println!("      \"reduced_dofs\": {},", observation.reduced_dofs);
        println!(
            "      \"dense_first_solve_elapsed_nanos\": {},",
            observation.dense_first_solve_elapsed_nanos
        );
        println!(
            "      \"sparse_first_solve_elapsed_nanos\": {},",
            observation.sparse_first_solve_elapsed_nanos
        );
        println!(
            "      \"dense_reduced_matrix_value_storage_bytes\": {},",
            observation.dense_reduced_matrix_value_storage_bytes
        );
        println!(
            "      \"sparse_ordered_profile_value_storage_bytes\": {},",
            observation.sparse_ordered_profile_value_storage_bytes
        );
        println!(
            "      \"allocator_rss_observation_status\": \"{}\",",
            escape(&observation.allocator_rss_observation_status)
        );
        println!(
            "      \"hardware_normalization_status\": \"{}\",",
            escape(&observation.hardware_normalization_status)
        );
        println!(
            "      \"true_condition_number_2norm\": {},",
            observation.true_condition_number_2norm
        );
        println!(
            "      \"true_condition_number_method\": \"{}\",",
            escape(&observation.true_condition_number_method)
        );
        println!(
            "      \"sparse_pivot_condition_ratio_estimate\": {},",
            optional_f64(observation.sparse_pivot_condition_ratio_estimate)
        );
        println!(
            "      \"max_abs_sparse_dense_solution_delta\": {},",
            observation.max_abs_sparse_dense_solution_delta
        );
        println!(
            "      \"sparse_dense_relative_delta\": {},",
            observation.sparse_dense_relative_delta
        );
        println!(
            "      \"max_abs_sparse_residual\": {},",
            observation.max_abs_sparse_residual
        );
        println!(
            "      \"max_abs_sparse_repeat_solution_delta\": {},",
            observation.max_abs_sparse_repeat_solution_delta
        );
        println!(
            "      \"nonpositive_pivot_count\": {},",
            observation.nonpositive_pivot_count
        );
        println!(
            "      \"default_sparse_promotion_status\": \"{}\",",
            escape(&observation.default_sparse_promotion_status)
        );
        println!(
            "      \"dense_path_role\": \"{}\",",
            escape(&observation.dense_path_role)
        );
        println!(
            "      \"sparse_path_role\": \"{}\",",
            escape(&observation.sparse_path_role)
        );
        println!(
            "      \"ci_gate_status\": \"{}\"",
            escape(&observation.ci_gate_status)
        );
        let suffix = if index + 1 == observations.len() {
            ""
        } else {
            ","
        };
        println!("    }}{suffix}");
    }
    println!("  ],");
    println!("  \"boundary\": [");
    println!("    \"sparse interactive is promoted only for preview/render iteration\",");
    println!("    \"dense scrutiny remains explicitly selectable for detailed parity review\",");
    println!(
        "    \"timing and allocator/RSS memory are observation fields, not release thresholds\","
    );
    println!("    \"true condition number is recorded for this bounded observation set beyond the pivot-ratio proxy\",");
    println!("    \"no hosted CI, release-readiness, professional, certification, sealing, authentication, or code-compliance claim is made\"");
    println!("  ]");
    println!("}}");
}

fn current_rss_kib() -> Option<u64> {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()?.trim().parse().ok()
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
