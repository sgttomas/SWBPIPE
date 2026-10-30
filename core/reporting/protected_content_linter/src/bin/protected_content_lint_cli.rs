//! DEL-08-05 protected-content lint CLI (DEC-058 check (b) surface).
//!
//! Reads caller-named text files, runs the report protected-content lint
//! engine over them, and prints a machine-readable JSON finding list to
//! stdout. Built for the DEC-058 release-candidate scan runner
//! (`tools/release/run_release_candidate_scan.py`); local-only, no network.
//!
//! This binary documents heuristic findings only. It does not clear, sign,
//! or gate anything: the scan act, per-finding dispositions, and the
//! legal/protected-data release-gate signature are the owner's human acts
//! (DEC-058/DEC-027). A clean run is not a legal clearance, release claim,
//! professional approval, certification, or code-compliance determination.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use open_pipe_stress_protected_content_linter::{
    lint_targets, FindingClass, FindingCode, FindingSeverity, LintConfiguration, LintTarget,
    PrivacyClassification, Provenance, RedistributionStatus, ReviewRoute, ReviewStatus,
    SurfaceKind,
};

const USAGE: &str =
    "usage: protected_content_lint_cli [--provenance-mode engine|external] <file> [<file> ...]\n\
Runs the DEL-08-05 protected-content lint engine over the named files and\n\
prints findings as JSON. --provenance-mode external disables the engine's\n\
own per-target provenance heuristic for callers that verify provenance\n\
separately (DEC-058 check (c)); the default mode 'engine' keeps it on, so\n\
every target fails toward UnknownProvenanceReviewRequired.";

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                escaped.push_str(&format!("\\u{:04x}", control as u32));
            }
            other => escaped.push(other),
        }
    }
    escaped
}

fn screaming_snake(debug_name: &str) -> String {
    let mut out = String::with_capacity(debug_name.len() + 8);
    for (index, character) in debug_name.chars().enumerate() {
        if character.is_ascii_uppercase() && index > 0 {
            out.push('_');
        }
        out.push(character.to_ascii_uppercase());
    }
    out
}

fn code_name(code: FindingCode) -> String {
    screaming_snake(&format!("{code:?}"))
}

fn class_name(class: FindingClass) -> String {
    screaming_snake(&format!("{class:?}"))
}

fn severity_name(severity: FindingSeverity) -> String {
    screaming_snake(&format!("{severity:?}"))
}

fn route_name(route: ReviewRoute) -> String {
    screaming_snake(&format!("{route:?}"))
}

/// Provenance placeholder for `--provenance-mode external`: the caller
/// (the DEC-058 scan runner) verifies real provenance separately in check
/// (c); this placeholder only keeps the engine's own per-target provenance
/// heuristic from double-reporting. It asserts nothing about the file.
fn external_check_provenance() -> Provenance {
    let note = "Provenance verified separately by the DEC-058 check (c) \
                provenance-manifest pass; engine provenance heuristic disabled \
                for this run."
        .to_string();
    Provenance {
        source_name: note.clone(),
        source_location: note.clone(),
        source_license: note.clone(),
        contributor: note.clone(),
        contributor_certification: note,
        redistribution_status: RedistributionStatus::Tbd,
        review_status: ReviewStatus::NotApplicable,
        privacy_classification: PrivacyClassification::PublicMetadata,
    }
}

fn engine_mode_provenance() -> Provenance {
    let note = "Unverified caller-supplied file; fails toward \
                UnknownProvenanceReviewRequired."
        .to_string();
    Provenance {
        source_name: note.clone(),
        source_location: note.clone(),
        source_license: "TBD".to_string(),
        contributor: note.clone(),
        contributor_certification: note,
        redistribution_status: RedistributionStatus::Unknown,
        review_status: ReviewStatus::Pending,
        privacy_classification: PrivacyClassification::PublicMetadata,
    }
}

/// Windows canonical paths often carry the extended `\\?\` prefix. Convert
/// them to stable logical identities before exact authorization. This does
/// not confer public-surface authority by itself.
fn normalize_windows_file_path(raw: &str) -> String {
    let slash_path = raw.replace('\\', "/");
    if let Some(unc) = slash_path.strip_prefix("//?/UNC/") {
        format!("unc://{unc}")
    } else if let Some(extended) = slash_path.strip_prefix("//?/") {
        extended.to_string()
    } else if let Some(unc) = slash_path.strip_prefix("//") {
        format!("unc://{unc}")
    } else {
        slash_path
    }
}

fn exact_file_target_path(path: &Path) -> Result<String, &'static str> {
    let raw = path.to_string_lossy();
    if cfg!(windows) {
        Ok(normalize_windows_file_path(&raw))
    } else if raw.contains('\\') {
        // Backslash is a literal Unix filename character. Rewriting it as a
        // separator could authorize another file; reject this CLI selection.
        Err("literal backslash in file path cannot be represented in lint scope")
    } else {
        Ok(raw.into_owned())
    }
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1).peekable();
    let mut provenance_mode = "engine".to_string();
    let mut paths: Vec<String> = Vec::new();

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--provenance-mode" => match args.next() {
                Some(mode) if mode == "engine" || mode == "external" => {
                    provenance_mode = mode;
                }
                _ => {
                    eprintln!("error: --provenance-mode requires 'engine' or 'external'");
                    eprintln!("{USAGE}");
                    return ExitCode::from(2);
                }
            },
            "--help" | "-h" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            other => paths.push(other.to_string()),
        }
    }

    if paths.is_empty() {
        eprintln!("error: no input files named");
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }

    let mut targets = Vec::with_capacity(paths.len());
    let mut exact_public_targets = Vec::with_capacity(paths.len());
    for path in &paths {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("error: cannot read '{path}': {error}");
                return ExitCode::from(2);
            }
        };
        let provenance = if provenance_mode == "external" {
            external_check_provenance()
        } else {
            engine_mode_provenance()
        };
        let exact_path = match fs::canonicalize(path) {
            Ok(path) => match exact_file_target_path(&path) {
                Ok(path) => path,
                Err(reason) => {
                    eprintln!("error: cannot scan '{}': {reason}", path.display());
                    return ExitCode::from(2);
                }
            },
            Err(error) => {
                eprintln!("error: cannot resolve '{path}': {error}");
                return ExitCode::from(2);
            }
        };
        exact_public_targets.push(exact_path.clone());
        targets.push(LintTarget {
            target_id: exact_path.clone(),
            path: exact_path,
            surface: SurfaceKind::PublicFixture,
            text,
            provenance,
        });
    }

    let mut configuration = LintConfiguration::public_surfaces_only("release-scan-cli-cfg");
    // Positional arguments select exact files for this invocation. They do not
    // authorize neighboring paths or certify the files for publication.
    configuration.public_surface_roots = exact_public_targets;
    let run = lint_targets("release-scan-lint-cli", configuration, targets);
    if run.summary.scanned_target_count != paths.len() {
        eprintln!(
            "error: {} explicitly selected file(s) were outside the authorized scan scope",
            paths.len() - run.summary.scanned_target_count
        );
        return ExitCode::from(2);
    }

    let mut findings_json = Vec::with_capacity(run.findings.len());
    for finding in &run.findings {
        findings_json.push(format!(
            "{{\"finding_id\":\"{}\",\"code\":\"{}\",\"class\":\"{}\",\"severity\":\"{}\",\
             \"path\":\"{}\",\"line\":{},\"column\":{},\"matched_policy\":\"{}\",\
             \"excerpt\":\"{}\",\"message\":\"{}\",\"remediation\":\"{}\",\
             \"review_route\":\"{}\",\"disposition\":\"PENDING\"}}",
            json_escape(&finding.finding_id),
            code_name(finding.code),
            class_name(finding.class),
            severity_name(finding.severity),
            json_escape(&finding.source_location.path),
            finding.source_location.line,
            finding.source_location.column,
            json_escape(&finding.matched_policy),
            json_escape(&finding.excerpt),
            json_escape(&finding.message),
            json_escape(&finding.remediation),
            route_name(finding.review_route),
        ));
    }

    println!(
        "{{\"engine\":\"open_pipe_stress_protected_content_linter\",\
         \"deliverable_id\":\"DEL-08-05\",\"decision_basis\":\"DEC-058\",\
         \"provenance_mode\":\"{}\",\
         \"summary\":{{\"target_count\":{},\"scanned_target_count\":{},\
         \"skipped_private_target_count\":{},\"skipped_incomplete_target_count\":{},\
         \"finding_count\":{},\"blocking_finding_count\":{},\
         \"clean_scan_is_clearance\":false}},\
         \"findings\":[{}]}}",
        json_escape(&provenance_mode),
        run.summary.target_count,
        run.summary.scanned_target_count,
        run.summary.skipped_private_target_count,
        run.summary.skipped_incomplete_target_count,
        run.summary.finding_count,
        run.summary.blocking_finding_count,
        findings_json.join(",")
    );

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_extended_drive_and_unc_paths_keep_exact_logical_identity() {
        let drive = normalize_windows_file_path(r"\\?\C:\public\selected.txt");
        let unc = normalize_windows_file_path(r"\\?\UNC\server\share\selected.txt");
        assert_eq!(drive, "C:/public/selected.txt");
        assert_eq!(unc, "unc://server/share/selected.txt");
        assert_eq!(
            normalize_windows_file_path(r"\\server\share\selected.txt"),
            "unc://server/share/selected.txt"
        );

        let mut config = LintConfiguration::public_surfaces_only("windows-exact");
        config.public_surface_roots = vec![drive.clone(), unc.clone()];
        let target = |path: String| LintTarget {
            target_id: path.clone(),
            path,
            surface: SurfaceKind::PublicFixture,
            text: String::new(),
            provenance: open_pipe_stress_protected_content_linter::invented_provenance(),
        };
        assert!(
            open_pipe_stress_protected_content_linter::should_scan_target(&target(drive), &config)
        );
        assert!(
            open_pipe_stress_protected_content_linter::should_scan_target(&target(unc), &config)
        );
        assert!(
            !open_pipe_stress_protected_content_linter::should_scan_target(
                &target(normalize_windows_file_path(
                    r"\\?\C:\public\selected.txt.neighbor"
                )),
                &config
            )
        );
        assert!(
            !open_pipe_stress_protected_content_linter::should_scan_target(
                &target(normalize_windows_file_path(
                    r"\\?\UNC\server\share-evil\selected.txt"
                )),
                &config
            )
        );
    }
}
