use std::{any::type_name, collections::BTreeSet, mem::{size_of, align_of}};
use open_pipe_stress_frame_kernel::Matrix12;
use piping_numerical_robustness::cases::Row;
fn report<T>(label: &str) {
    println!("{}\t{}\t{}\t{}", label, type_name::<T>(), size_of::<T>(), align_of::<T>());
}
fn main() {
    report::<Option<[f64; 3]>>("H.coordinates_item");
    report::<(u32, usize)>("VR.constraints_item");
    report::<BTreeSet<usize>>("VR.adjacency_header");
    report::<(usize, usize, Matrix12)>("VR.formed_item");
    report::<(&str, &Row)>("VR.borrowed_control_pair");
    report::<(String, (f64, f64))>("VR.floor_input_pair");
    report::<(usize, f64, f64)>("VR.prescribed_coupling");
    report::<&[u8]>("VR.decimal_chunk_view");
    report::<&String>("VR.failure_head_pointer");
    report::<Vec<f64>>("VR.skyline_row_header");
}
