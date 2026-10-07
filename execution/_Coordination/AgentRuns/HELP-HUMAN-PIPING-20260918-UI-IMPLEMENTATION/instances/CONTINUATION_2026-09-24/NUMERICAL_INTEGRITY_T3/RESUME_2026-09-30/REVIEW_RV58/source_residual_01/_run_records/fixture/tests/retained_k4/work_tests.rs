use super::*;

#[test]
fn checked_work_max_is_exact_and_overflow_is_sticky() {
    let max = WorkTotal::exact_count(u64::MAX);
    assert_eq!(max.exact(), Ok(u64::MAX));
    let overflow = max.add(WorkTotal::exact_count(1));
    assert_eq!(overflow.exact(), Err(WorkFault::Overflow));
    assert_eq!(overflow.remainder(max).exact(), Err(WorkFault::Overflow));
    assert_eq!(overflow.mul(0).exact(), Err(WorkFault::Overflow));
    assert_eq!(overflow.legacy_saturated(), u64::MAX);
    assert_eq!(format!("{max:?}"), u64::MAX.to_string());
    assert!(format!("{overflow:?}").contains("UnavailableWork"));
}

#[test]
fn checked_work_foreign_snapshots_and_reverse_deltas_refuse() {
    let a = WorkStream::new();
    let b = WorkStream::new();
    let first = a.snapshot(WorkTotal::exact_count(7)).unwrap();
    let last = a.snapshot(WorkTotal::exact_count(11)).unwrap();
    assert_eq!(last.delta_since(first).exact(), Ok(4));
    assert_eq!(
        first.delta_since(last).exact(),
        Err(WorkFault::Inconsistent)
    );
    let foreign = b.snapshot(WorkTotal::exact_count(11)).unwrap();
    assert_eq!(
        last.delta_since(foreign).exact(),
        Err(WorkFault::Inconsistent)
    );
    let overflow = WorkTotal::exact_count(u64::MAX).mul(2);
    assert!(matches!(a.snapshot(overflow), Err(WorkFault::Overflow)));
}

#[test]
fn checked_work_status_union_and_room_preserve_unavailability() {
    let overflow = WorkTotal::exact_count(u64::MAX).mul(2);
    let inconsistent = WorkTotal::zero().remainder(WorkTotal::exact_count(1));
    assert_eq!(overflow.add(inconsistent).exact(), Err(WorkFault::Both));
    assert_eq!(overflow.room(0).exact(), Err(WorkFault::Overflow));
    assert_eq!(WorkTotal::exact_count(11).room(10).exact(), Ok(0));
    assert_eq!(std::mem::size_of::<WorkStatus>(), 1);
    assert_eq!(std::mem::size_of::<WorkFault>(), 1);
}
