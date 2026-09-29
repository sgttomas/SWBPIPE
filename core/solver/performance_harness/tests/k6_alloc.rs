//! K6 tests F1 and F2: the counting, capped allocator's accounting and cap
//! boundary. It is a `harness = false` binary (no libtest threads), and it
//! registers the observation binary's own allocator source as its global
//! allocator, so an accounting regression can abort only this binary.
//! Allocations are made through direct `GlobalAlloc` calls, so the sequence
//! is known exactly.

#[path = "../src/bin/k6_observe/alloc.rs"]
mod alloc;

use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOCATOR: alloc::K6Alloc = alloc::K6Alloc;

fn layout(size: usize) -> Layout {
    Layout::from_size_align(size, 8).expect("layout")
}

/// F1: alloc, realloc up (both peak models), realloc down, alloc_zeroed and
/// dealloc, over a known sequence.
fn accounting() {
    alloc::reset_peaks();
    let base = alloc::current();
    unsafe {
        let p = ALLOCATOR.alloc(layout(1000));
        assert!(!p.is_null());
        assert_eq!(alloc::current(), base + 1000, "alloc adds its size");
        assert_eq!(alloc::peak(), base + 1000);
        assert_eq!(alloc::peak_move(), base + 1000);

        let q = ALLOCATOR.realloc(p, layout(1000), 3000);
        assert!(!q.is_null());
        assert_eq!(
            alloc::current(),
            base + 3000,
            "realloc up adds the difference"
        );
        assert_eq!(alloc::peak(), base + 3000, "in-place peak: the new size");
        assert_eq!(
            alloc::peak_move(),
            base + 4000,
            "move peak: the old and the new block together"
        );

        let r = ALLOCATOR.realloc(q, layout(3000), 500);
        assert!(!r.is_null());
        assert_eq!(
            alloc::current(),
            base + 500,
            "realloc down releases the difference"
        );
        assert_eq!(alloc::peak(), base + 3000, "a shrink leaves the peak");
        assert_eq!(alloc::peak_move(), base + 4000);

        let z = ALLOCATOR.alloc_zeroed(layout(4096));
        assert!(!z.is_null());
        assert!(
            std::slice::from_raw_parts(z, 4096).iter().all(|&b| b == 0),
            "zeroed memory"
        );
        assert_eq!(alloc::current(), base + 4596, "alloc_zeroed is counted");
        assert_eq!(alloc::peak(), base + 4596);
        assert_eq!(alloc::peak_move(), base + 4596);

        ALLOCATOR.dealloc(z, layout(4096));
        ALLOCATOR.dealloc(r, layout(500));
    }
    assert_eq!(alloc::current(), base, "dealloc releases every byte");
    alloc::stage_reset();
    assert_eq!(alloc::stage_peak(), base);
    assert_eq!(alloc::stage_peak_move(), base);
}

/// F2: exactly at the cap succeeds; one byte over fails (null, no abort),
/// for alloc, alloc_zeroed and a growing realloc.
fn cap_boundary() {
    let base = alloc::current();
    alloc::set_cap(base + 1000);
    unsafe {
        let p = ALLOCATOR.alloc(layout(1000));
        assert!(!p.is_null(), "an allocation exactly at the cap succeeds");
        assert_eq!(alloc::current(), base + 1000);
        let over = ALLOCATOR.alloc(Layout::from_size_align(1, 1).expect("layout"));
        assert!(over.is_null(), "one byte over the cap is refused");
        let over_zeroed = ALLOCATOR.alloc_zeroed(Layout::from_size_align(1, 1).expect("layout"));
        assert!(over_zeroed.is_null(), "alloc_zeroed is capped too");
        assert_eq!(alloc::current(), base + 1000, "a refusal reserves nothing");
        ALLOCATOR.dealloc(p, layout(1000));

        let q = ALLOCATOR.alloc(layout(400));
        assert!(!q.is_null());
        let grown = ALLOCATOR.realloc(q, layout(400), 1000);
        assert!(!grown.is_null(), "a realloc exactly to the cap succeeds");
        let refused = ALLOCATOR.realloc(grown, layout(1000), 1001);
        assert!(
            refused.is_null(),
            "a realloc one byte over the cap is refused"
        );
        assert_eq!(alloc::current(), base + 1000);
        ALLOCATOR.dealloc(grown, layout(1000));
    }
    alloc::set_cap(usize::MAX);
    assert_eq!(alloc::current(), base);
}

fn main() {
    accounting();
    cap_boundary();
    println!("k6_alloc: F1 accounting and F2 cap boundary passed");
}
