// Additional test-only inspection wrappers for the newly repaired validation boundaries.
#[cfg(test)]
impl AdapterWork {
 pub(super) fn rv60_closed_keys(&self,value:&serde_json::Value,expected:&[&str])->Result<(),CaptureError>{self.closed_keys(value,expected,"rv60 shape")}
}
#[cfg(test)]
pub(super) fn rv60_mode_metadata(row:&ResultItem,case:&str,work:&AdapterWork)->Result<(),CaptureError>{validate_final_metadata(row,k::ProductRecipe::NonQuantity,case,work)}
