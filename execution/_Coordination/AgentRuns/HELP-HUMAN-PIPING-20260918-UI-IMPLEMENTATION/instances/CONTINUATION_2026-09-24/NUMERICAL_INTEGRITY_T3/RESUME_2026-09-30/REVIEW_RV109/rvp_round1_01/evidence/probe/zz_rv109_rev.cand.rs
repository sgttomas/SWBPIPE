//! RV109 probe shim, candidate copy: reads ST's seam field.
pub(super) fn late_total(o: &crate::retained_product::ProductCapture) -> Option<usize> { Some(o.late_loads_total) }
