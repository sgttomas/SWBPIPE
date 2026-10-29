//! Small native observations, not stable numerical expectations or residency credit.
use std::{ffi::c_void, mem::size_of, ptr};

type Handle = *mut c_void;
#[repr(C)]
#[derive(Default, Debug)]
struct CountersEx2 {
    cb: u32,
    page_fault_count: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    quota_peak_paged_pool_usage: usize,
    quota_paged_pool_usage: usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
    private_usage: usize,
    private_working_set_size: usize,
    shared_commit_usage: u64,
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> Handle;
    fn K32GetProcessMemoryInfo(process: Handle, counters: *mut CountersEx2, cb: u32) -> i32;
    fn VirtualAlloc(address: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(address: *mut c_void, size: usize, kind: u32) -> i32;
}
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 0x04;
const BYTES: usize = 4 * 1024 * 1024;
fn sample(label: &str) {
    let mut c = CountersEx2 {
        cb: size_of::<CountersEx2>() as u32,
        ..Default::default()
    };
    // SAFETY: c has the documented EX2 layout and remains live during the call.
    let ok = unsafe {
        K32GetProcessMemoryInfo(GetCurrentProcess(), &mut c, size_of::<CountersEx2>() as u32)
    };
    assert_ne!(ok, 0, "GetProcessMemoryInfo EX2 failed");
    eprintln!(
        "{label}: private_commit={} private_working_set={} total_working_set={} shared_commit={}",
        c.private_usage, c.private_working_set_size, c.working_set_size, c.shared_commit_usage
    );
}
#[test]
fn mapping_commit_touch_and_release_are_distinct_observations() {
    sample("before");
    // SAFETY: null asks the OS to choose an address. The range is bounded to 4 MiB.
    let p = unsafe {
        VirtualAlloc(
            ptr::null_mut(),
            BYTES,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    } as *mut u8;
    assert!(!p.is_null(), "VirtualAlloc failed");
    sample("committed_untouched");
    for offset in (0..BYTES).step_by(4096) {
        // SAFETY: every offset is within the committed writable range.
        unsafe { p.add(offset).write_volatile(1) };
    }
    sample("touched");
    // SAFETY: release uses the base address and zero size as required by VirtualFree.
    assert_ne!(unsafe { VirtualFree(p.cast(), 0, MEM_RELEASE) }, 0);
    sample("released");
    let mut heap = vec![0u8; BYTES];
    for offset in (0..BYTES).step_by(4096) {
        heap[offset] = 1;
    }
    std::hint::black_box(&heap);
    sample("heap_touched");
    drop(heap);
    sample("heap_freed_allocator_may_retain");
}
