//! The desktop's bridge to the shared report-package producer: the solver
//! identity this application links, and the two entries native admission
//! calls. Route 009's save reaches the producer through the general entry,
//! [`assemble_request`], which refuses every user-release marker; route 010's
//! branch reaches it only through [`assemble_user_released_request`], the one
//! caller of `report_package::user_release::assemble_derived` (W-18b,
//! DEL-08-01 Design `report-user-release.md` §2.5 step 9, §2.8;
//! `user_release_reach_scan`). Standard claim fence applies (claims taxonomy
//! per DEC-081).

use open_pipe_stress_product_physics::{solver_component_name, solver_component_version};
use open_pipe_stress_report_package as package;
pub use package::wire::ReportPackageRequest;
use serde_json::Value;

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

/// Route 010's producer entry (§2.5 step 9): the admitting entry's preflight,
/// the whole-request derivation from the bound basis (`inputs`, with `S` the
/// request's own attested source) and the input manifest, then the shared
/// producer under the user-release entry.
pub fn assemble_user_released_request(
    request: Value,
    inputs: &package::user_release::UserReleaseInputs<'_>,
    input_manifest: &Value,
) -> Result<package::ReportPackageContainerOutcome, String> {
    let solver_name = solver_component_name();
    let solver_version = solver_component_version();
    let solver_build_ref = format!("{solver_name}@{solver_version}");
    package::user_release::assemble_derived(
        request,
        &package::wire::LinkedSolverIdentity {
            solver_name,
            solver_version,
            solver_build_ref: &solver_build_ref,
        },
        inputs,
        input_manifest,
    )
}
