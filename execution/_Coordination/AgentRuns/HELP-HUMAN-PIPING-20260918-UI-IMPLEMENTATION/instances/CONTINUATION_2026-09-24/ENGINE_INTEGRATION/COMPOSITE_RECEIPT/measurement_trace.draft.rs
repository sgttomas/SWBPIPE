#[cfg(test)]
thread_local! {
    static WORK_TRACE: std::cell::RefCell<Option<Vec<Value>>> = const {std::cell::RefCell::new(None)};
}
#[cfg(test)]
pub(super) fn start_trace() {WORK_TRACE.with(|trace|*trace.borrow_mut()=Some(vec![]));}
#[cfg(test)]
pub(super) fn take_trace()->Vec<Value> {WORK_TRACE.with(|trace|trace.borrow_mut().take().unwrap_or_default())}
#[cfg(test)]
fn trace(stage:&str,selected:&SelectedSourceRecovery) {WORK_TRACE.with(|trace|if let Some(values)=&mut *trace.borrow_mut(){values.push(json!({"stage":stage,"charged":selected.summary().work.charged,"rejected":selected.summary().work.rejected}));});}
