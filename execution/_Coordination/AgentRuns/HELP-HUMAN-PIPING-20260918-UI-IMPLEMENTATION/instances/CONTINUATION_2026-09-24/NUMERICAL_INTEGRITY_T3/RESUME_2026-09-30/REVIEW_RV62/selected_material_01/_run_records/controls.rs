
#[test]
fn rv62_independent_source_and_owned_text_controls() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, AdapterWork, CaptureError};
    for interpolated in [false, true] {
        let raw = i47_selected_specimen(interpolated, true);
        let (request, _) = source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
        let case = &request.model.load_cases[0];
        let (envelope, mut o) = observed(raw);
        let row = envelope.results.iter().find(|r| r.kind == "modulus_basis_record").unwrap();
        assert!(o.error.is_none());
        o.rv62_source_check(case).unwrap();
        o.rv62_record_check(row).unwrap();
        let mut capture = o.basis_record.take().unwrap();
        o.basis_record_calls = 0;
        assert!(o.rv62_source_check(case).is_err(), "source-side missing capture must refuse");
        assert!(o.basis_expected, "source expectation survives missing successful capture");
        o.basis_record_calls = 1;
        capture.case.push('x');
        o.basis_record = Some(capture);
        assert!(o.rv62_source_check(case).is_err(), "foreign captured owner");
        o.basis_record.as_mut().unwrap().case.pop();
        o.rv62_source_check(case).unwrap();
        let mut foreign = case.clone(); foreign.id.push('x');
        assert!(o.rv62_source_check(&foreign).is_err(), "foreign actual source owner");
        let mut base = case.clone(); base.modulus_basis_ref=None; base.modulus_basis_temperature=None;
        assert!(o.rv62_source_check(&base).is_err(), "base rejects captured selected history");
        o.rv62_source_check(case).unwrap();
        let mut altered = row.clone();
        altered.metadata.as_mut().unwrap().basis.push_str(" é");
        assert!(o.rv62_record_check(&altered).is_err());
        // Dynamic text is owned bytes, not a parser or a numerical reconstruction.
        o.basis_record.as_mut().unwrap().text = altered.metadata.as_ref().unwrap().basis.clone();
        o.rv62_record_check(&altered).unwrap();
        assert!(o.rv62_record_check(row).is_err());
        for event in [E::ValidationEntry,E::LibraryBoundary,E::KeyProbe,E::IdentityByteRead] {
            o.adapter=AdapterWork::default();
            let mut counts=[0;10];counts[event as usize]=u64::MAX;o.adapter.counts.set(counts);
            assert!(matches!(o.rv62_record_check(&altered), Err(CaptureError::Accounting(AdapterFault::Overflow(actual))) if actual==event));
            let prefix=o.adapter.counts.get();
            assert!(o.rv62_record_check(&altered).is_err());
            assert_eq!(o.adapter.counts.get(),prefix);
            println!("RV62_RECORD_PREFIX {interpolated} {event:?} {prefix:?}");
        }
    }
    println!("RV62_SOURCE_TEXT_CONTROLS passed");
}
