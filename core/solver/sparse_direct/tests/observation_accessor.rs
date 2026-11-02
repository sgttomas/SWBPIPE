//! The read-only observation accessor of `ProfileFactorization`
//! (`lower_factor_entries`, `pivots`), added for F1's Sturm counts
//! (DEL-04-09 `Work/modal-analysis.md` §3.3, the observation seam).
//!
//! Read-only is asserted twice: by the API shape (both methods take `&self`
//! and return borrowing iterators), and by comparing the factorization and
//! its solve results bit for bit before and after iterating.
//!
//! No allocation is asserted with a counting global allocator. The counter
//! is thread-local, so allocations by the test harness or by tests on other
//! threads are not counted; only the iteration under test runs between the
//! two readings.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use open_pipe_stress_sparse_direct::{
    factorize_ldlt, ProfileFactorization, SymmetricMatrixEntry, SymmetricProfileMatrix,
};

struct CountingAllocator;

thread_local! {
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.with(|count| count.set(count.get() + 1));
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.with(|count| count.set(count.get() + 1));
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.with(|count| count.set(count.get() + 1));
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

fn allocations() -> usize {
    ALLOCATIONS.with(Cell::get)
}

/// An indefinite 5×5 matrix with a ragged profile (row 3 starts at column 1,
/// row 4 at column 0), so that the iteration crosses rows of different
/// widths and stored envelope zeros.
fn ragged_factorization() -> ProfileFactorization {
    let entries = [
        (0, 0, 4.0),
        (1, 0, 1.0),
        (1, 1, 3.0),
        (2, 2, -2.0),
        (3, 1, 0.5),
        (3, 3, 5.0),
        (4, 0, 0.25),
        (4, 2, 1.0),
        (4, 4, 6.0),
    ]
    .map(|(row, col, value)| SymmetricMatrixEntry { row, col, value });
    let profile = SymmetricProfileMatrix::from_entries(5, &entries).unwrap();
    factorize_ldlt(&profile).unwrap()
}

#[test]
fn accessor_yields_the_factor_in_ordered_position() {
    // 2×2 by hand: A = [[4, 2], [2, 5]] = L D Lᵀ with l₁₀ = 1/2, d = (4, 4).
    let profile = SymmetricProfileMatrix::from_dense(&[vec![4.0, 2.0], vec![2.0, 5.0]]).unwrap();
    let factorization = factorize_ldlt(&profile).unwrap();
    assert_eq!(
        factorization.lower_factor_entries().collect::<Vec<_>>(),
        vec![(1, 0, 0.5)]
    );
    assert_eq!(factorization.pivots().collect::<Vec<_>>(), vec![4.0, 4.0]);

    let factorization = ragged_factorization();
    let positions: Vec<(usize, usize)> = factorization
        .lower_factor_entries()
        .map(|(row, col, _)| (row, col))
        .collect();
    assert_eq!(
        positions,
        vec![(1, 0), (3, 1), (3, 2), (4, 0), (4, 1), (4, 2), (4, 3)]
    );
    let pivots: Vec<f64> = factorization.pivots().collect();
    assert_eq!(pivots.len(), 5);
    let negative = pivots.iter().filter(|&&d| d < 0.0).count();
    assert_eq!(negative, factorization.report().nonpositive_pivot_count);
    assert_eq!(negative, 1);

    // L D Lᵀ rebuilt from the observed entries reproduces A to rounding.
    let mut l = [[0.0f64; 5]; 5];
    for (row, col, value) in factorization.lower_factor_entries() {
        l[row][col] = value;
    }
    for (i, row) in l.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    let a = [
        [4.0, 1.0, 0.0, 0.0, 0.25],
        [1.0, 3.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, -2.0, 0.0, 1.0],
        [0.0, 0.5, 0.0, 5.0, 0.0],
        [0.25, 0.0, 1.0, 0.0, 6.0],
    ];
    for i in 0..5 {
        for j in 0..5 {
            let rebuilt: f64 = (0..5).map(|k| l[i][k] * pivots[k] * l[j][k]).sum();
            assert!((rebuilt - a[i][j]).abs() <= 1e-15, "({i}, {j})");
        }
    }
}

#[test]
fn accessor_leaves_the_factorization_unchanged_and_allocates_nothing() {
    let factorization = ragged_factorization();
    let snapshot = factorization.clone();
    let rhs = [1.0, -2.0, 0.5, 3.0, -1.25];
    let before = factorization.solve_ordered(&rhs).unwrap();

    let start = allocations();
    let mut entry_sum = 0.0;
    let mut entry_count = 0usize;
    for (row, col, value) in factorization.lower_factor_entries() {
        entry_sum += value.abs() + (row + col) as f64;
        entry_count += 1;
    }
    let mut pivot_sum = 0.0;
    for d in factorization.pivots() {
        pivot_sum += d.abs();
    }
    let allocated = allocations() - start;
    assert_eq!(allocated, 0, "the accessor allocated {allocated} times");
    assert_eq!(entry_count, 7);
    assert!(entry_sum.is_finite() && pivot_sum > 0.0);

    // The counter does count this thread's allocations.
    let probe = allocations();
    let boxed = Box::new(entry_sum);
    assert!(allocations() > probe);
    drop(boxed);

    assert_eq!(factorization, snapshot);
    let after = factorization.solve_ordered(&rhs).unwrap();
    assert_eq!(before.len(), after.len());
    for (x, y) in before.iter().zip(&after) {
        assert_eq!(x.to_bits(), y.to_bits());
    }
}
