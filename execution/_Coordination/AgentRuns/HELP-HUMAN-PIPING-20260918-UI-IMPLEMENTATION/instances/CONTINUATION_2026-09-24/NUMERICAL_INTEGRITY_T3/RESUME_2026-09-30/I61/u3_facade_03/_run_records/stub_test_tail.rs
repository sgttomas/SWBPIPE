
/// DISPOSABLE ARCHIVE ONLY: the permitted dispatch end to end behind the stub.
#[test]
fn u3_archive_stub_permitted_dispatch_end_to_end() {
    use super::retained_memory::{set_stub, StubMode};
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(PINNED) {
        let raw = raw();
        let plain = plain(mode, &raw);
        set_stub(StubMode::Permit);
        let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        set_stub(StubMode::Off);
        assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), plain, "{name}: ordinary base untouched");
        assert!(out.admission().is_some(), "RV85 S3: the permitted output keeps its report");
        let successor = out.successor().unwrap_or_else(|| panic!("{name}: {:?}", out.retained())).clone();
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":successor,
            "invocation":{"request":raw,"solver_mode":mode.as_str()}})).unwrap();
        println!("I61_U3_STUB {name} successor_file_sha256={} receipt={}", sha(text.as_bytes()), successor["retained_precision"]["receipt_sha256"]);
        assert_eq!((sha(text.as_bytes()).as_str(), successor["retained_precision"]["receipt_sha256"].as_str()), (file_sha, Some(receipt_sha)));
        match out.into_publication() {
            RetainedPublication::Successor(value) => assert_eq!(value, successor),
            RetainedPublication::Ordinary(_) => panic!("{name}: the transfer completed"),
        }
        if let Some(dir) = std::env::var("I61_U3_OUT").ok().map(std::path::PathBuf::from) {
            std::fs::write(dir.join(format!("u3_stub_dispatch_successor_{name}.json")), &text).unwrap();
        }
        // No W1 work ran: exact ordinary bytes.
        for (stub, expected) in [(StubMode::RefuseLate, "LateGate"), (StubMode::RefuseComplete, "CompleteGate"), (StubMode::NoStack, "StackReservation")] {
            set_stub(stub);
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            set_stub(StubMode::Off);
            assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), plain, "{name} {stub:?}: exact ordinary bytes");
            assert!(out.admission().is_some(), "RV85 S3: {stub:?} keeps its report");
            let cause = format!("{:?}", out.retained().unwrap().as_ref().err().unwrap());
            println!("I61_U3_STUB {name} {stub:?} fallback={cause}");
            assert!(cause.starts_with(expected), "{cause}");
            assert!(out.successor().is_none());
            assert!(matches!(out.into_publication(), RetainedPublication::Ordinary(e) if serde_json::to_vec(&e).unwrap() == plain));
        }
        // W1 work ran: a fault armed on the caller's thread fires on the reserved-stack
        // thread (carry_test_hooks), and the publication carries R-2's notice.
        for (arm, expected) in [(retained_tests_hooks::corrupt_next_precommit as fn(), "Precommit"), (retained_tests_hooks::withdraw_next_native_source, "Native"),
            (retained_tests_hooks::break_next_staging, "Staging")] {
            arm();
            set_stub(StubMode::Permit);
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            set_stub(StubMode::Off);
            assert_eq!(retained_tests_hooks::armed(), retained_tests_hooks::Armed::default(), "consumed on the reserved-stack thread");
            let cause = format!("{:?}", out.retained().unwrap().as_ref().err().unwrap());
            println!("I61_U3_STUB {name} carried {expected} fallback={cause}");
            assert!(cause.starts_with(expected), "{cause}");
            assert!(out.admission().is_some(), "RV85 S3");
            let noticed = with_notice(&plain, "case", None);
            assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), noticed, "{name}: the notice after the ordinary prefix");
            assert!(matches!(out.into_publication(), RetainedPublication::Ordinary(e) if serde_json::to_vec(&e).unwrap() == noticed));
        }
    }
}

/// DISPOSABLE ARCHIVE ONLY: under a stub permit, every request-shaped fixture's
/// publication is its plain route's bytes, plus R-2's notice exactly where W1 work ran.
#[test]
fn u3_archive_stub_every_fixture_keeps_ordinary_bytes() {
    use super::retained_memory::{set_stub, StubMode};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
        entries.sort();
        for p in entries { if p.is_dir() { walk(&p, out) } else if p.extension().map_or(false, |e| e == "json") { out.push(p) } }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut files = Vec::new();
    walk(&root, &mut files);
    let (mut checked, mut problems) = (0, Vec::new());
    let mut causes = std::collections::BTreeMap::<String, usize>::new();
    for path in files {
        let rel = path.strip_prefix(&root).unwrap().display().to_string();
        let Ok(v) = serde_json::from_slice::<Value>(&std::fs::read(&path).unwrap()) else { continue };
        let raw = if v.get("model").and_then(|m| m.get("load_cases")).is_some() { v }
            else if v.get("load_cases").is_some() && (v.get("nodes").is_some() || v.get("components").is_some()) { json!({ "model": v }) }
            else { continue };
        for mode in MODES {
            let plain = run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_vec(&e).unwrap());
            set_stub(StubMode::Permit);
            let got = catch_unwind(AssertUnwindSafe(|| run_linear_static_preview_value_with_retained_direct(raw.clone(), mode)));
            set_stub(StubMode::Off);
            checked += 1;
            match got {
                Err(_) => problems.push(format!("{rel} {}: PANIC", mode.as_str())),
                Ok(out) => {
                    let cause = match &out {
                        Err(s) => format!("Err({s})"),
                        Ok(o) => match o.retained() {
                            None => "no W1 result".into(),
                            Some(Ok(_)) => "successor".into(),
                            Some(Err(f)) => format!("{f:?}").split(|c: char| c == '(' || c == ' ').next().unwrap().to_owned(),
                        },
                    };
                    let w1_ran = matches!(cause.as_str(), "Preparation" | "Native" | "Candidate" | "Serializer" | "Precommit");
                    let expected = match (&plain, w1_ran) {
                        (Ok(bytes), true) => Ok(with_notice(bytes, raw["model"]["load_cases"][0]["id"].as_str().unwrap(), None)),
                        (other, _) => other.clone(),
                    };
                    let bytes = out.as_ref().map(|o| serde_json::to_vec(o.envelope()).unwrap()).map_err(|s| s.clone());
                    if bytes != expected { problems.push(format!("{rel} {}: bytes differ ({cause})", mode.as_str())); }
                    if let Ok(o) = out {
                        if o.admission().is_none() { problems.push(format!("{rel} {}: RV85 S3 report missing", mode.as_str())); }
                        let successor = o.successor().is_some();
                        if successor != (cause == "successor") { problems.push(format!("{rel}: successor() disagrees")); }
                        match o.into_publication() {
                            RetainedPublication::Successor(_) if cause == "successor" => {}
                            RetainedPublication::Ordinary(e) if cause != "successor" && Ok(serde_json::to_vec(&e).unwrap()) == expected => {}
                            _ => problems.push(format!("{rel} {}: publication disagrees ({cause})", mode.as_str())),
                        }
                    }
                    println!("I61_U3_STUB_SWEEP {rel} {} {cause}{}", mode.as_str(), if w1_ran { " +notice" } else { "" });
                    *causes.entry(cause).or_default() += 1;
                }
            }
        }
    }
    println!("I61_U3_STUB_SWEEP checked={checked} causes={causes:?} problems={problems:?}");
    assert!(problems.is_empty(), "{problems:?}");
}

/// DISPOSABLE ARCHIVE ONLY: a permitted invocation whose ordinary receipt cannot
/// finalize (T20's RV4 construction C1) stays fail-closed, exactly as the
/// ordinary route refuses it.
#[test]
fn u3_archive_stub_finalization_failure_stays_fail_closed() {
    use super::retained_memory::{set_stub, StubMode};
    let request = super::s11g_tests::c1_request(1.0, 1e-7);
    for mode in MODES {
        let plain = run_linear_static_preview_value_with_mode(request.clone(), mode).map(|e| serde_json::to_vec(&e).unwrap());
        assert_eq!(plain.as_ref().err().map(String::as_str), Some("SOURCE_BLOCKS_FINALIZATION_FAILED"), "precondition");
        set_stub(StubMode::Permit);
        let got = run_linear_static_preview_value_with_retained_direct(request.clone(), mode);
        set_stub(StubMode::Off);
        println!("I61_U3_STUB_C1 {} {:?}", mode.as_str(), got.as_ref().err());
        assert_eq!(got.err().as_deref(), Some("SOURCE_BLOCKS_FINALIZATION_FAILED"));
    }
}

/// DISPOSABLE ARCHIVE ONLY (RV85 N1): an exact source-block selection is recorded as
/// coexistence with exact bytes and G-C is never consulted (the stub panics if it is).
#[test]
fn u3_archive_stub_exact_selection_skips_g_c() {
    use super::retained_memory::{set_stub, StubMode};
    let raw: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json")).unwrap();
    for mode in MODES {
        let plain = plain(mode, &raw);
        set_stub(StubMode::NoCompleteExpected);
        let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode);
        set_stub(StubMode::Off);
        let out = out.unwrap();
        let cause = format!("{:?}", out.retained().unwrap().as_ref().err().unwrap());
        println!("I61_U3_STUB_N1 {} exact selection fallback={cause}", mode.as_str());
        assert_eq!(cause, "Coexistence");
        assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), plain);
        assert!(out.admission().is_some());
    }
}
