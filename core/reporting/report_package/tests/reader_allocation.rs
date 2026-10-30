//! Allocation budget for the `.opsproj` reader on hostile input (review
//! round 1/2). A separate test binary, so its counting allocator measures
//! only this test. All data invented.

mod common;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use common::Fixture;
use open_pipe_stress_report_package::{
    assemble_report_package_container, read_report_package_container,
};

struct Counting;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATED.fetch_add(new_size, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn eocd(entries: u16, central_size: u32, central_offset: u32) -> Vec<u8> {
    let mut out = 0x0605_4b50u32.to_le_bytes().to_vec();
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&entries.to_le_bytes());
    out.extend_from_slice(&entries.to_le_bytes());
    out.extend_from_slice(&central_size.to_le_bytes());
    out.extend_from_slice(&central_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

#[test]
fn shared_local_header_entries_stay_within_an_allocation_budget() {
    // The writer's own manifest member and its central entry (version 2.0.0,
    // so reading passes the gate), then one local header with a 65535-byte
    // name and 65000 central entries pointing at it.
    let outcome = assemble_report_package_container(&Fixture::new().input());
    let manifest = &outcome.members[0];
    let bytes_all = &outcome.container_bytes;
    let local_len = 30 + manifest.file_name.len() + manifest.bytes.len();
    let end = bytes_all.len() - 22;
    let central_offset =
        u32::from_le_bytes(bytes_all[end + 16..end + 20].try_into().unwrap()) as usize;
    let manifest_central =
        bytes_all[central_offset..central_offset + 46 + manifest.file_name.len()].to_vec();

    let mut bytes = bytes_all[..local_len].to_vec();
    let shared_offset = bytes.len() as u32;
    bytes.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
    bytes.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0x21, 0]);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&[0u8; 8]);
    bytes.extend_from_slice(&u16::MAX.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend(std::iter::repeat_n(b'a', usize::from(u16::MAX)));
    let new_central_offset = bytes.len() as u32;
    bytes.extend_from_slice(&manifest_central);
    let shared: u16 = 65_000;
    for _ in 0..shared {
        bytes.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        bytes.extend_from_slice(&[20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0x21, 0]);
        bytes.extend_from_slice(&[0u8; 12]);
        bytes.extend_from_slice(&[0u8; 14]);
        bytes.extend_from_slice(&shared_offset.to_le_bytes());
    }
    let central_size = bytes.len() as u32 - new_central_offset;
    bytes.extend(eocd(shared + 1, central_size, new_central_offset));

    let before = ALLOCATED.load(Ordering::Relaxed);
    let refused = read_report_package_container(&bytes).expect_err("refused");
    let allocated = ALLOCATED.load(Ordering::Relaxed) - before;
    assert_eq!(refused.code(), "OPSPROJ_MEMBER_INTEGRITY", "{refused}");
    // Copying the shared local name per entry would allocate ~4 GiB; the
    // budget is a small multiple of the ~3 MB input.
    assert!(
        allocated < 64 * 1024 * 1024,
        "{allocated} bytes allocated for a {}-byte input",
        bytes.len()
    );
}
