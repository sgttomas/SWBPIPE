"""I99 B3-W: probe-only instrumentation of the archive copy (cfg(test) prints only; no control-flow change).
I86's diff (R/I86/b1_w_probe_01/_run_records/instrumentation.diff) renamed I99, without the timing marks."""
import sys, pathlib
src = pathlib.Path(sys.argv[1])
def edit(name, old, new, count=1):
    p = src / name
    t = p.read_text()
    assert t.count(old) == count, (name, old[:80], t.count(old))
    p.write_text(t.replace(old, new))
edit("lib.rs",
 "    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));\n",
 "    let ordinary = run_linear_static_preview_observed(request, solver_mode, Some(capture), &mut budget, Some(&mut observer));\n"
 "    #[cfg(test)] retained_memory::zz_i99_probe::seeds(\"permitted_run\", &observer, &ordinary);\n")
edit("lib.rs",
 "    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {\n        return (ordinary, Err(W1Fallback::NoticeReservation));\n    };\n",
 "    let Some(notice) = ReservedNotice::reserve(&mut ordinary, case_id) else {\n        return (ordinary, Err(W1Fallback::NoticeReservation));\n    };\n"
 "    #[cfg(test)] println!(\"I99_W1_START case={case_id}\");\n")
edit("lib.rs",
 "        Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),\n",
 "        Err(failure) => { #[cfg(test)] println!(\"I99_PREPARATION_FAILURE error={:?} preparation_error={:?}\", failure.capture.error, failure.preparation_error); return notice.publish(failure.ordinary, W1Fallback::Preparation) },\n")
edit("lib.rs",
 "        Err(refusal) => return notice.publish(refusal.ordinary, W1Fallback::Candidate),\n",
 "        Err(refusal) => { #[cfg(test)] { let e = format!(\"{:?}\", refusal.error); let mut end = e.len().min(1500); while !e.is_char_boundary(end) { end -= 1; } println!(\"I99_CANDIDATE_REFUSAL len={} error={}\", e.len(), &e[..end]); } return notice.publish(refusal.ordinary, W1Fallback::Candidate) },\n")
edit("lib.rs",
 "    if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {\n",
 "    if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {\n"
 "        #[cfg(test)] { use sha2::Digest; println!(\"I99_PRECOMMIT_ERROR {:?}\", error);\n"
 "            if let Ok(d) = std::env::var(\"I99_OUT\") { let tag = format!(\"{:x}\", sha2::Sha256::digest(serde_json::to_vec(&invocation).unwrap()));\n"
 "                let path = std::path::Path::new(&d).join(format!(\"precommit_refused_{}_{}.json\", capture.mode().as_str(), &tag[..12]));\n"
 "                std::fs::write(&path, serde_json::to_vec_pretty(&serde_json::json!({\"source\": &successor, \"invocation\": &invocation})).unwrap()).unwrap();\n"
 "                println!(\"I99_PRECOMMIT_DUMP {}\", path.file_name().unwrap().to_string_lossy()); } }\n")
edit("retained_product.rs",
 "        let selected=matches!(case.outcome,k::ExecutionOutcome::Selected(_));\n",
 "        #[cfg(test)] { match &case.outcome {\n"
 "            k::ExecutionOutcome::Selected(_)=>println!(\"I99_NATIVE_OUTCOME Selected\"),\n"
 "            k::ExecutionOutcome::Unresolved{reason,attempts,..}=>println!(\"I99_NATIVE_OUTCOME Unresolved reason={:?} attempts={:?}\",reason,attempts.iter().map(|a|(a.precision,format!(\"{:?}\",a.role),format!(\"{:?}\",a.outcome))).collect::<Vec<_>>()),\n"
 "            k::ExecutionOutcome::Refused{refusal,attempts,..}=>println!(\"I99_NATIVE_OUTCOME Refused refusal={:?} attempts={:?}\",refusal,attempts.iter().map(|a|(a.precision,format!(\"{:?}\",a.role),format!(\"{:?}\",a.outcome))).collect::<Vec<_>>()),\n"
 "        } }\n"
 "        let selected=matches!(case.outcome,k::ExecutionOutcome::Selected(_));\n")
edit("retained_memory.rs",
 "#[path = \"retained_memory_witness_tests.rs\"]\nmod witness_tests;\n",
 "#[path = \"retained_memory_witness_tests.rs\"]\nmod witness_tests;\n#[cfg(test)]\n#[path = \"zz_i99_probe.rs\"]\npub(super) mod zz_i99_probe;\n")
print("ok")
