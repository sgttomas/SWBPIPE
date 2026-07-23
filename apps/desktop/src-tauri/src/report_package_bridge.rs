use open_pipe_stress_product_physics::{solver_component_name, solver_component_version};
use open_pipe_stress_report_package as package;
pub use package::wire::ReportPackageRequest;

pub fn assemble_request(
    request: ReportPackageRequest,
) -> Result<package::ReportPackageContainerOutcome, String> {
    let solver_name = solver_component_name();
    let solver_version = solver_component_version();
    let solver_build_ref = format!("{solver_name}@{solver_version}");
    package::wire::assemble_wire_request(
        request,
        &package::wire::LinkedSolverIdentity {
            solver_name,
            solver_version,
            solver_build_ref: &solver_build_ref,
        },
    )
}
